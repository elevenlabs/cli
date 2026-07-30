#!/usr/bin/env bash
# Agents-as-code manual verification for CLI v1.
#
# Run from inside the cli repo so dotenvy finds the repo-root .env by walking up.
# Creates REAL agents/tools/tests in the workspace the API key belongs to, and
# deletes them again in phase 7. Run against a scratch account if you have one.
#
#   bash scripts/verify-agents-as-code.sh
#
set -uo pipefail

# No command here may wait on input: a prompt with stdout redirected would
# hang invisibly. Anything that asks gets --yes, and stdin is closed so a
# missed one declines and fails loudly instead of blocking.
exec < /dev/null

CLI="$(cd "$(dirname "$0")/.." && pwd)/target/release/elevenlabs"
WORK="$(cd "$(dirname "$0")/.." && pwd)/.verify-project"

pass() { printf '  \033[32m✓\033[0m %s\n' "$1"; }
fail() { printf '  \033[31m✗ %s\033[0m\n' "$1"; FAILED=$((FAILED + 1)); }
phase() { printf '\n\033[1m══ %s\033[0m\n' "$1"; }
FAILED=0

phase "0. Setup"
rm -rf "$WORK"; mkdir -p "$WORK"; cd "$WORK"
"$CLI" --version && pass "binary runs"

phase "1. Read-only surface (no resources created)"
"$CLI" agents templates list >/dev/null && pass "agents templates list"
"$CLI" agents templates show customer-service >/dev/null && pass "templates show"
"$CLI" tests templates list >/dev/null && pass "tests templates list"

phase "2. init scaffolds the project"
"$CLI" agents init . >/dev/null
for f in agents.json tools.json tests.json .env.example .gitignore; do
  [ -f "$f" ] && pass "$f created" || fail "$f missing"
done
for d in agent_configs tool_configs test_configs; do
  [ -d "$d" ] && pass "$d/ created" || fail "$d/ missing"
done
# Regression: the config dirs must stay tracked. Ignoring them breaks silently,
# because agents.json is tracked while the configs it references are not.
grep -qE '^(agent|tool|test)_configs/' .gitignore \
  && fail ".gitignore ignores config dirs (would untrack committed work)" \
  || pass ".gitignore leaves config dirs tracked"
grep -qx '.env' .gitignore && pass ".gitignore covers .env" || fail ".env not ignored"

phase "3. Create an agent (first real resource)"
"$CLI" agents add e2e-agent --template customer-service
AGENT_ID=$(jq -r '.agents[0].id' agents.json)
CONFIG=$(jq -r '.agents[0].config' agents.json)
[ -n "$AGENT_ID" ] && [ "$AGENT_ID" != null ] \
  && pass "agent created: $AGENT_ID" || { fail "no agent id in agents.json"; exit 1; }
[ -f "$CONFIG" ] && pass "config written: $CONFIG" || fail "config file missing"

phase "4. Round-trip fidelity (the important one)"
# pull -> push -> pull. The two pulled configs must be byte-identical: any
# difference means the push/store path is dropping or reshaping fields.
"$CLI" agents pull --agent "$AGENT_ID" --update --yes >/dev/null
cp "$CONFIG" /tmp/e2e-roundtrip-a.json
"$CLI" agents push --agent "$AGENT_ID" >/dev/null
"$CLI" agents pull --agent "$AGENT_ID" --update --yes >/dev/null
cp "$CONFIG" /tmp/e2e-roundtrip-b.json
if diff -q /tmp/e2e-roundtrip-a.json /tmp/e2e-roundtrip-b.json >/dev/null; then
  pass "config survived pull -> push -> pull unchanged"
else
  fail "ROUND-TRIP LOSSY — diff:"
  diff /tmp/e2e-roundtrip-a.json /tmp/e2e-roundtrip-b.json | head -20
fi

phase "5. status / widget / dry-run"
"$CLI" agents status && pass "agents status"
"$CLI" agents widget embed "$AGENT_ID" | grep -q 'elevenlabs-convai' \
  && pass "widget embed emits the element" || fail "widget embed output unexpected"
"$CLI" agents push --dry-run >/dev/null && pass "push --dry-run sends nothing"

phase "6. Tools and tests"
"$CLI" tools add e2e-tool --type webhook >/dev/null 2>&1 \
  && pass "tools add" || fail "tools add"
TOOL_ID=$(jq -r '.tools[0].id // empty' tools.json)
"$CLI" tools push >/dev/null 2>&1 && pass "tools push" || fail "tools push"

"$CLI" tests add e2e-test --template basic-llm >/dev/null 2>&1 \
  && pass "tests add" || fail "tests add"
TEST_ID=$(jq -r '.tests[0].id // empty' tests.json)
"$CLI" tests push >/dev/null 2>&1 && pass "tests push" || fail "tests push"

# Auto-discovery: an untracked config in test_configs/ should be picked up.
cat > test_configs/stray.json <<'JSON'
{"name":"e2e-stray","chat_history":[{"role":"user","time_in_call_secs":0,"message":"hi"}],
 "success_condition":"The agent responded.","success_examples":[{"response":"Hello","type":"success"}],
 "failure_examples":[{"response":"","type":"failure"}]}
JSON
# Capture rather than pipe to grep: on failure we want to see what it printed.
discover_out=$("$CLI" tests push 2>&1)
if printf '%s' "$discover_out" | grep -qi 'discovered'; then
  pass "push auto-discovers untracked test config"
else
  fail "stray test config not discovered — push said:"
  printf '%s\n' "$discover_out" | sed 's/^/      /' | head -12
fi

phase "7. Path-containment regressions (should all be REFUSED)"
printf '{"agents":[{"config":"../../../../tmp/e2e-escape.json","id":"%s"}]}' "$AGENT_ID" > agents.json
rm -f /tmp/e2e-escape.json
"$CLI" agents pull --agent "$AGENT_ID" --update --yes >/dev/null 2>&1
[ -f /tmp/e2e-escape.json ] \
  && fail "TRAVERSAL: wrote outside the project" \
  || pass "traversal path refused"

mkdir -p /tmp/e2e-outside && echo '{"canary":true}' > /tmp/e2e-outside/target.json
ln -sf /tmp/e2e-outside/target.json agent_configs/linked.json
printf '{"agents":[{"config":"agent_configs/linked.json","id":"%s"}]}' "$AGENT_ID" > agents.json
"$CLI" agents pull --agent "$AGENT_ID" --update --yes >/dev/null 2>&1
grep -q canary /tmp/e2e-outside/target.json \
  && pass "symlinked config not written through" \
  || fail "SYMLINK: wrote through to the target"

# Restore a valid index for cleanup.
printf '{"agents":[{"config":"%s","id":"%s"}]}' "$CONFIG" "$AGENT_ID" > agents.json

# --override must not delete without confirmation.
echo keep > agent_configs/precious.json
"$CLI" agents init . --override < /dev/null >/dev/null 2>&1
[ -f agent_configs/precious.json ] \
  && pass "--override declines when non-interactive" \
  || fail "--override deleted configs unprompted"

phase "8. Cleanup (deletes the remote resources)"
# Delete every id in the indexes, not just the first: push auto-discovery
# registers extra tests, and deleting only .tests[0] leaks the rest remotely.
n=0
for id in $(jq -r '.tests[].id // empty' tests.json 2>/dev/null); do
  "$CLI" tests delete "$id" >/dev/null 2>&1 && n=$((n + 1))
done
pass "$n test(s) deleted"
n=0
for id in $(jq -r '.tools[].id // empty' tools.json 2>/dev/null); do
  "$CLI" tools delete "$id" >/dev/null 2>&1 && n=$((n + 1))
done
pass "$n tool(s) deleted"
"$CLI" agents delete --agent-id "$AGENT_ID" >/dev/null 2>&1 && pass "agent deleted"
rm -rf /tmp/e2e-outside /tmp/e2e-escape.json /tmp/e2e-roundtrip-*.json

printf '\n'
if [ "$FAILED" -eq 0 ]; then
  printf '\033[32mAll checks passed.\033[0m Scratch project left at %s\n' "$WORK"
else
  printf '\033[31m%d check(s) failed.\033[0m Scratch project left at %s\n' "$FAILED" "$WORK"
  printf 'Check for leftovers: agent %s (and any e2e-* tests/tools)\n' "$AGENT_ID"
fi
exit "$FAILED"
