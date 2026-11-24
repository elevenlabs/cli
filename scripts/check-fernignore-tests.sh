#!/usr/bin/env bash
# Fails when a hand-written file under tests/ is missing from .fernignore, since a regeneration deletes it.
set -euo pipefail

cd "$(dirname "$0")/.."

GENERATED=(tests/wire_test.rs)

missing=()
while IFS= read -r file; do
  for generated in "${GENERATED[@]}"; do
    [[ "$file" == "$generated" ]] && continue 2
  done
  grep -qxF "$file" .fernignore || missing+=("$file")
done < <(git ls-files tests)

if (( ${#missing[@]} )); then
  for file in "${missing[@]}"; do
    echo "::error file=${file}::${file} is hand-written but not listed in .fernignore; add it there or the next regeneration deletes it."
  done
  exit 1
fi
echo "Every hand-written test is listed in .fernignore."
