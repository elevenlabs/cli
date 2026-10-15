//! Wire tests for `flows … create-and-wait` against a local mock API. They run
//! the real binary with `ELEVENLABS_FLOWS_WAIT_FAKE_CLOCK` set, so no test
//! actually sleeps: each wait is logged to stderr as `flows-wait: sleep <ms>ms`
//! and advances a virtual clock instead, which is also what `--wait-timeout`
//! is measured against.

use std::process::{Command, Output};
use std::time::{Duration, SystemTime};

use serde_json::{json, Value};
use wiremock::matchers::{body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

const KEY: &str = "sk_test_0123456789abcdef";
const IMAGE_BODY: &str = r#"{"model_id":"bytedance-seedream-5-lite","prompt":"a corgi"}"#;

fn cli(server: &MockServer, args: &[&str]) -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_elevenlabs"));
    cmd.args(["--base-url", &server.uri()])
        .args(args)
        .env("ELEVENLABS_API_KEY", KEY)
        .env("ELEVENLABS_FLOWS_WAIT_FAKE_CLOCK", "1")
        .env("NO_COLOR", "1")
        .env("FERN_CLI_CREDENTIAL_STORE", "file")
        .env("HOME", env!("CARGO_TARGET_TMPDIR"));
    for var in ["ELEVENLABS_BASE_URL", "ELEVENLABS_VIA", "ELEVENLABS_OUTPUT"] {
        cmd.env_remove(var);
    }
    cmd
}

/// Run the CLI off the async runtime, so the mock server keeps serving.
async fn run(cmd: Command) -> Output {
    let mut cmd = cmd;
    tokio::task::spawn_blocking(move || cmd.output().unwrap())
        .await
        .unwrap()
}

fn stderr(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

fn stdout_json(out: &Output) -> Value {
    serde_json::from_slice(&out.stdout).unwrap_or_else(|e| {
        panic!(
            "stdout is not one JSON document ({e}):\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&out.stdout),
            stderr(out)
        )
    })
}

/// The waits the fake clock recorded, in order.
fn sleeps(out: &Output) -> Vec<Duration> {
    stderr(out)
        .lines()
        .filter_map(|l| l.strip_prefix("flows-wait: sleep "))
        .map(|ms| Duration::from_millis(ms.trim_end_matches("ms").parse().unwrap()))
        .collect()
}

/// `base` plus up to 20% jitter.
fn assert_jittered(actual: Duration, base: Duration) {
    assert!(
        actual >= base && actual <= base.mul_f64(1.2) + Duration::from_millis(1),
        "{actual:?} is not within [{base:?}, {:?}]",
        base.mul_f64(1.2)
    );
}

fn generation(status: &str) -> Value {
    json!({"id": "gen_1", "status": status})
}

fn reply(status: &str, retry_after: Option<&str>) -> ResponseTemplate {
    let r = ResponseTemplate::new(200).set_body_json(generation(status));
    match retry_after {
        Some(v) => r.insert_header("Retry-After", v),
        None => r,
    }
}

async fn mount_create(server: &MockServer, response: ResponseTemplate) {
    Mock::given(method("POST"))
        .and(path("/v1/flows/image"))
        .respond_with(response)
        .expect(1)
        .mount(server)
        .await;
}

/// Serve `replies` to successive GETs of `gen_1`, then fail the test on any more.
async fn mount_gets(server: &MockServer, replies: Vec<ResponseTemplate>) {
    let n = replies.len() as u64;
    for r in replies {
        Mock::given(method("GET"))
            .and(path("/v1/flows/image/gen_1"))
            .respond_with(r)
            .up_to_n_times(1)
            .mount(server)
            .await;
    }
    // Lowest priority: only reached once every canned reply is used up.
    Mock::given(method("GET"))
        .and(path("/v1/flows/image/gen_1"))
        .respond_with(
            ResponseTemplate::new(500).set_body_json(json!({"detail": "polled too often"})),
        )
        .expect(0)
        .with_priority(10)
        .named(format!("no more than {n} GETs"))
        .mount(server)
        .await;
}

fn image_wait(server: &MockServer, extra: &[&str]) -> Command {
    let mut args = vec![
        "flows",
        "image",
        "create-and-wait",
        "--json",
        IMAGE_BODY,
        "--format",
        "json",
    ];
    args.extend_from_slice(extra);
    cli(server, &args)
}

#[tokio::test]
async fn retry_after_seconds_paces_each_poll() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/flows/image"))
        .and(body_json(
            serde_json::from_str::<Value>(IMAGE_BODY).unwrap(),
        ))
        .respond_with(reply("pending", Some("7")))
        .expect(1)
        .mount(&server)
        .await;
    mount_gets(
        &server,
        vec![
            reply("generating", Some("11")),
            reply("completed", Some("3")),
        ],
    )
    .await;

    let out = run(image_wait(&server, &[])).await;
    assert!(out.status.success(), "{}", stderr(&out));
    assert_eq!(stdout_json(&out), generation("completed"));
    let waits = sleeps(&out);
    assert_eq!(waits.len(), 2, "{waits:?}");
    // The create response's Retry-After sets the first wait.
    assert_jittered(waits[0], Duration::from_secs(7));
    assert_jittered(waits[1], Duration::from_secs(11));
}

#[tokio::test]
async fn retry_after_http_date_is_honoured() {
    let server = MockServer::start().await;
    let date = httpdate::fmt_http_date(SystemTime::now() + Duration::from_secs(30));
    mount_create(&server, reply("pending", Some(&date))).await;
    mount_gets(&server, vec![reply("completed", None)]).await;

    let out = run(image_wait(&server, &[])).await;
    assert!(out.status.success(), "{}", stderr(&out));
    let waits = sleeps(&out);
    assert_eq!(waits.len(), 1, "{waits:?}");
    // One-second date resolution, plus the time to start the process.
    assert!(
        waits[0] >= Duration::from_secs(25) && waits[0] <= Duration::from_secs(36),
        "{waits:?}"
    );
}

#[tokio::test]
async fn missing_zero_or_garbage_retry_after_falls_back_to_the_minimum() {
    let server = MockServer::start().await;
    mount_create(&server, reply("pending", None)).await;
    mount_gets(
        &server,
        vec![
            reply("generating", Some("0")),
            reply("generating", Some("soon")),
            reply("generating", Some("1")),
            reply("completed", None),
        ],
    )
    .await;

    let out = run(image_wait(&server, &[])).await;
    assert!(out.status.success(), "{}", stderr(&out));
    let waits = sleeps(&out);
    assert_eq!(waits.len(), 4, "{waits:?}");
    for w in waits {
        assert_jittered(w, Duration::from_secs(3));
    }
}

#[tokio::test]
async fn min_poll_interval_raises_the_floor() {
    let server = MockServer::start().await;
    mount_create(&server, reply("pending", Some("2"))).await;
    mount_gets(&server, vec![reply("completed", None)]).await;

    let out = run(image_wait(&server, &["--min-poll-interval", "5"])).await;
    assert!(out.status.success(), "{}", stderr(&out));
    let waits = sleeps(&out);
    assert_eq!(waits.len(), 1);
    assert_jittered(waits[0], Duration::from_secs(5));
}

#[tokio::test]
async fn terminal_create_response_is_not_polled() {
    let server = MockServer::start().await;
    let run_body = json!({
        "id": "run_1",
        "template_id": "tpl_1",
        "status": "completed",
        "outputs": {"image": {"status": "completed"}},
    });
    Mock::given(method("POST"))
        .and(path("/v1/flows/templates/tpl_1/runs"))
        .and(body_json(
            json!({"inputs": {"prompt": "a corgi"}, "version_id": "latest"}),
        ))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(run_body.clone())
                .insert_header("Retry-After", "5"),
        )
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(500))
        .expect(0)
        .mount(&server)
        .await;

    let out = run(cli(
        &server,
        &[
            "flows",
            "templates",
            "runs",
            "create-and-wait",
            "--template-id",
            "tpl_1",
            "--inputs",
            r#"{"prompt":"a corgi"}"#,
            "--version-id",
            "latest",
            "--format",
            "json",
        ],
    ))
    .await;
    assert!(out.status.success(), "{}", stderr(&out));
    assert_eq!(stdout_json(&out), run_body);
    assert!(sleeps(&out).is_empty());
}

#[tokio::test]
async fn failed_result_is_printed_and_exits_1() {
    let server = MockServer::start().await;
    mount_create(&server, reply("pending", None)).await;
    // The shape prod returns for a moderated `gpt-image-1` prompt.
    let failed = json!({
        "id": "gen_1",
        "status": "failed",
        "failure_reason": "moderated",
        "error_message": "This generation may violate our Terms of Service and has been blocked.",
    });
    mount_gets(
        &server,
        vec![ResponseTemplate::new(200).set_body_json(failed.clone())],
    )
    .await;

    let out = run(image_wait(&server, &[])).await;
    assert_eq!(out.status.code(), Some(1), "{}", stderr(&out));
    // The result goes to stdout like `get` would print it; the note to stderr.
    assert_eq!(stdout_json(&out), failed);
    assert!(
        stderr(&out).contains("generation gen_1 failed"),
        "{}",
        stderr(&out)
    );
}

#[tokio::test]
async fn timeout_names_the_run_template_and_resume_command() {
    let server = MockServer::start().await;
    let running = |rid: &str| {
        ResponseTemplate::new(200)
            .set_body_json(json!({"id": "run_1", "template_id": "tpl_1", "status": "running"}))
            .insert_header("Retry-After", rid)
    };
    Mock::given(method("POST"))
        .and(path("/v1/flows/templates/tpl_1/runs"))
        .respond_with(running("6"))
        .expect(1)
        .mount(&server)
        .await;
    // At 6s × up to 1.2 jitter, a 10s timeout leaves time for exactly one
    // more full wait, which the deadline then cuts short.
    Mock::given(method("GET"))
        .and(path("/v1/flows/templates/tpl_1/runs/run_1"))
        .respond_with(running("6"))
        .expect(2)
        .mount(&server)
        .await;

    let out = run(cli(
        &server,
        &[
            "flows",
            "templates",
            "runs",
            "create-and-wait",
            "--template-id",
            "tpl_1",
            "--inputs",
            "{}",
            "--wait-timeout",
            "10",
            "--format",
            "json",
        ],
    ))
    .await;
    assert_eq!(out.status.code(), Some(1), "{}", stderr(&out));
    let err = &stdout_json(&out)["error"];
    assert_eq!(err["reason"], "waitTimeout", "{err}");
    assert_eq!(err["code"], 200);
    let message = err["message"].as_str().unwrap();
    assert!(
        message.contains("run_1") && message.contains("tpl_1"),
        "{message}"
    );
    assert!(message.contains("running after 10s"), "{message}");
    assert_eq!(
        err["help"],
        "To keep waiting, run: elevenlabs flows templates runs get --template-id tpl_1 --run-id run_1"
    );
    assert_eq!(err["details"]["id"], "run_1");

    let waits = sleeps(&out);
    assert_eq!(waits.len(), 2, "{waits:?}");
    assert_jittered(waits[0], Duration::from_secs(6));
    // The last wait is cut short at the deadline (the log truncates to ms).
    let total = waits[0] + waits[1];
    assert!(
        total <= Duration::from_secs(10) && total >= Duration::from_millis(9_998),
        "{waits:?}"
    );
}

#[tokio::test]
async fn zero_timeout_gives_up_after_the_create() {
    let server = MockServer::start().await;
    mount_create(&server, reply("pending", None)).await;
    mount_gets(&server, vec![]).await;

    let out = run(image_wait(&server, &["--wait-timeout", "0"])).await;
    assert_eq!(out.status.code(), Some(1), "{}", stderr(&out));
    let err = &stdout_json(&out)["error"];
    assert_eq!(err["reason"], "waitTimeout");
    assert_eq!(
        err["help"],
        "To keep waiting, run: elevenlabs flows image get --generation-id gen_1"
    );
    assert!(sleeps(&out).is_empty());
}

#[tokio::test]
async fn bad_options_are_rejected_before_any_request() {
    let server = MockServer::start().await;
    for bad in [
        ["--min-poll-interval", "0.4"],
        ["--min-poll-interval", "NaN"],
        ["--min-poll-interval", "inf"],
        ["--wait-timeout", "-1"],
        ["--wait-timeout", "NaN"],
    ] {
        let out = run(image_wait(&server, &bad)).await;
        assert_eq!(out.status.code(), Some(3), "{bad:?}: {}", stderr(&out));
        let err = &stdout_json(&out)["error"];
        assert_eq!(err["reason"], "validationError", "{bad:?}: {err}");
    }
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn dry_run_sends_nothing() {
    let server = MockServer::start().await;
    let out = run(image_wait(&server, &["--dry-run", "--wait-timeout", "5"])).await;
    assert!(out.status.success(), "{}", stderr(&out));
    let plan = stdout_json(&out);
    assert_eq!(plan["dry_run"], true);
    assert_eq!(plan["method"], "POST");
    assert_eq!(plan["url"], format!("{}/v1/flows/image", server.uri()));
    assert_eq!(
        plan["body"],
        serde_json::from_str::<Value>(IMAGE_BODY).unwrap()
    );
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn server_errors_on_a_poll_are_retried() {
    let server = MockServer::start().await;
    mount_create(&server, reply("pending", None)).await;
    mount_gets(
        &server,
        vec![
            ResponseTemplate::new(503)
                .set_body_json(json!({"detail": "busy"}))
                .insert_header("Retry-After", "0"),
            reply("completed", None),
        ],
    )
    .await;

    let out = run(image_wait(&server, &[])).await;
    assert!(out.status.success(), "{}", stderr(&out));
    assert_eq!(stdout_json(&out), generation("completed"));
    // The retry is the executor's, not a poll: only one wait was scheduled.
    assert_eq!(sleeps(&out).len(), 1);
}

#[tokio::test]
async fn progress_goes_to_stderr_and_stdout_stays_json() {
    let server = MockServer::start().await;
    mount_create(&server, reply("pending", Some("4"))).await;
    mount_gets(&server, vec![reply("completed", None)]).await;

    let out = run(image_wait(&server, &["--debug", "--query", "status"])).await;
    assert!(out.status.success(), "{}", stderr(&out));
    assert_eq!(stdout_json(&out), json!("completed"));
    assert!(
        stderr(&out).contains("flows: generation gen_1 is pending; checking again in"),
        "{}",
        stderr(&out)
    );

    // Without --debug, and with stderr not a terminal, there is no progress.
    let server = MockServer::start().await;
    mount_create(&server, reply("pending", None)).await;
    mount_gets(&server, vec![reply("completed", None)]).await;
    let out = run(image_wait(&server, &[])).await;
    assert!(!stderr(&out).contains("flows: "), "{}", stderr(&out));
}

#[tokio::test]
async fn raw_and_http_formats_are_rejected_before_any_request() {
    let server = MockServer::start().await;
    for format in ["raw", "http"] {
        let out = run(cli(
            &server,
            &[
                "flows",
                "image",
                "create-and-wait",
                "--json",
                IMAGE_BODY,
                "--format",
                format,
            ],
        ))
        .await;
        assert_eq!(out.status.code(), Some(3), "{format}: {}", stderr(&out));
        let all = format!("{}{}", String::from_utf8_lossy(&out.stdout), stderr(&out));
        assert!(
            all.contains("does not support --format raw or http"),
            "{format}: {all}"
        );
    }
    assert!(server.received_requests().await.unwrap().is_empty());
}
