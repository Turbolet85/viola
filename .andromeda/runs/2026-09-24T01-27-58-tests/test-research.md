## Test Runner / Framework

### cargo test (libtest, Rust 1.95.0 toolchain)

- **Version:** Rust 1.95.0 (host toolchain; MSRV `rust-version = "1.89"`, edition 2024)
- **Last release:** 2026-04-16 (Rust 1.95.0, the host toolchain)
- **Status:** actively maintained
- **Agent-runnable:** yes — exit code 0 means everything passed and 101 means a failure, plus one `test <path> ... ok|FAILED` line per test and a `test result:` summary line on stdout. Configuration: `cargo test --workspace --doc` covers the doctests nextest cannot run. Stable libtest has no JSON output (`--format json` still needs `-Z unstable-options` on nightly), so use nextest JUnit for structured results.
- **Fits because:** it is the library-only surface (Sec 2: "`cargo test` per crate / workspace", "libtest per-test result lines") and harness `run` step 1 (Sec 3). Integration tests under `tests/` get `CARGO_BIN_EXE_viola`, which the cli, hook and tui drivers need.
- **Key detail:** libtest JSON output is a 2026 Rust Project Goal and is still unstable. Do not build the harness on `--format json`.
- **Source:** https://github.com/rust-lang/rust/releases/tag/1.95.0 ; https://rust-lang.github.io/rust-project-goals/2026/libtest-json.html

### cargo-nextest

- **Version:** 0.9.146
- **Last release:** 2026-09-21
- **Status:** actively maintained
- **Agent-runnable:** yes — exit code is non-zero on any failure. It writes JUnit XML (stable, Jenkins format) and lists tests as JSON with `cargo nextest list --message-format json`. Libtest-JSON output is experimental and "not currently full-fidelity". Configuration: `.config/nextest.toml` needs:
  - a `[profile.ci]` with `junit.path = "junit.xml"`, `retries = 0` (the creator says "do not flake", so retries must not hide flakes), `slow-timeout = { period = "30s", terminate-after = 4 }` and `fail-fast = false`
  - a `[profile.mutants]` with `fail-fast = true` for cargo-mutants
- **Fits because:** each test runs in its own process, so per-test `--home` isolation holds (upstream §1 Per-test isolation). Test groups (`max-threads = 1`) serialise the few tests that share an OS resource, such as the default port 47319. Harness `run` builds its `{suite, passed, failed, survived}` JSON summary from the JUnit file (Sec 3). nextest is also the runner under `cargo llvm-cov nextest` and `cargo mutants --test-tool=nextest`.
- **Key detail:** nextest does not run doctests. A separate `cargo test --doc` step is required, and a mutant caught only by a doctest shows as "missed" under nextest. Releases are frequent; in 2026 it fixed an occasional hang with libtest JSON on Linux and added support for build-dir layout v2.
- **Source:** https://nexte.st/docs/machine-readable/ ; https://github.com/nextest-rs/nextest/releases

## Coverage Tool

### cargo-llvm-cov

- **Version:** 0.9.1
- **Last release:** 2026-09-06
- **Status:** actively maintained
- **Agent-runnable:** yes — emits LCOV, Cobertura XML or JSON (`--json` adds context at the root). `--fail-under-lines/-functions/-regions/-file-lines <MIN>` turns thresholds into a non-zero exit code. Configuration: `cargo llvm-cov nextest --workspace --lcov --output-path target/lcov.info --fail-under-lines <N>`, and `cargo llvm-cov report --json --summary-only` for a JSON summary.
- **Fits because:** it covers all workspace crates (library-only surface) on the 3-OS matrix (multi-os-compat trigger). `x86_64-pc-windows-msvc` is confirmed working, and prebuilt binaries exist for Linux, macOS and Windows. `show-env` ("Get coverage of external tests") instruments binaries run outside cargo test. That includes the `viola` wrappers the harness boots for the fake-agent E2E and Playwright suites.
- **Key detail:** doctest coverage needs `--doctests` on both `show-env` and `report`, and nextest coverage must be merged with a separate doctest run. 0.8.6 (2026-05-09) added `--fail-under-file-lines`.
- **Source:** https://github.com/taiki-e/cargo-llvm-cov ; https://nexte.st/docs/integrations/test-coverage/

## Fixture & Factory Library

### rstest

- **Version:** 0.27.0
- **Last release:** 2026-09-06
- **Status:** actively maintained
- **Agent-runnable:** yes — it is a proc-macro, so results come through the libtest/nextest exit code and JUnit. Configuration: `[dev-dependencies] rstest = "0.27"`, with `#[fixture] fn home() -> TempHome` and `#[rstest] #[case(...)]` tables.
- **Fits because:** table cases fit several matrices:
  - the refusal-order matrices: `send` is human-typing → budget-paused → turn-running → input-not-ready → no-prompt-submitted, and `answer` is human-typing → unverified-cli → unknown-dialog
  - the exit-code table 0/1/2/10–14/20/21
  - the per-`<cli-version>` fixture replay (multi-version-compat trigger)

  `#[files("fixtures/claude/*/*.json")]` generates one test per recorded hook payload in the project's `fixtures/claude/<cli-version>/` tree. `viola verify` records that tree locally and CI replays it (Founder Direction 2: "the fake agent + `viola verify` fixtures are the contract").
- **Key detail:** fixtures are injected as test arguments, so temp home → fake-agent path → booted wrapper can be one composable fixture chain. That is the self-bootstrapping the harness `boot` needs (Sec 3). The same `#[files]` walk should fail on absolute paths or usernames in fixtures (security excerpt: "Fixtures from real prompts, or unchecked for paths and usernames"). Stamps must come from running `viola verify` against the fake agent, never from a hand-written `stamps.json` (the open conflict in Sec 3). The repo was last pushed 2026-09-06 and is not archived.
- **Source:** https://github.com/la10736/rstest ; https://docs.rs/rstest/latest/rstest/

### tempfile

- **Version:** 3.27.0
- **Last release:** 2026-03-11
- **Status:** actively maintained
- **Agent-runnable:** yes — it is a library, and a failing test gives a non-zero test exit code. Configuration: `let home = tempfile::TempDir::new()?;` then pass `--home <home.path()>` to every verb. Use `TempDir::keep()` only when a failing test must leave its home behind for `logs`.
- **Fits because:** it implements "each test uses its own viola home (global `--home`)" (upstream §1). Because drop removes the dir, harness `cleanup` idempotence is testable.
- **Key detail:** on Unix, tempfile creates dirs with mode 0700, which already passes viola's strict-modes (`mode & 0o077 == 0`). Tests for "group/world-writable home → refused" must `chmod` explicitly. On Windows `%TEMP%` sits under `%USERPROFILE%`, so the "`--home` outside `%USERPROFILE%`" DACL path needs a dedicated test directory.
- **Source:** https://github.com/Stebalien/tempfile

## Per-Surface Driver: cli (verbs + `--json` / exit-code contract)

### assert_cmd + predicates

- **Version:** assert_cmd 2.2.2 (predicates 3.1.4)
- **Last release:** 2026-05-11 (predicates 3.1.4: 2026-02-11)
- **Status:** actively maintained
- **Agent-runnable:** yes — it asserts the exit code, stdout and stderr of a `std::process::Command`. Configuration: `Command::cargo_bin("viola")?.args(["--home", h, "send", "b", "--json"]).write_stdin(txt).assert().code(13).stdout(predicate::function(|s: &str| serde_json::from_str::<Value>(s)…))`.
- **Fits because:** it is the Sec 2 cli driver the scope names. It checks:
  - exit codes 0/1/2/10–14/20/21
  - the single `{"v":1,…}` JSON document
  - stderr `unable  <name>  <reason>  <detail>` plus a last-line `hint:`
  - no SGR under non-TTY / `NO_COLOR` / `TERM=dumb` (grep stdout for `\x1b[`)
- **Key detail:** `Command::cargo_bin` is not deprecated. It only works in integration tests and reuses existing features and build modes, so keep CLI tests in the root bin's `tests/`.
- **Source:** https://github.com/assert-rs/assert_cmd ; https://docs.rs/assert_cmd/latest/assert_cmd/cargo/index.html

### trycmd (snapbox)

- **Version:** trycmd 1.2.1 (snapbox 1.2.2)
- **Last release:** 2026-07-21 (snapbox 1.2.2: 2026-05-26)
- **Status:** actively maintained
- **Agent-runnable:** yes — cases are `.toml`/`.md` files holding the expected stdout, stderr and exit code, and a mismatch fails the test with a diff. Configuration: `trycmd::TestCases::new().case("tests/cmd/*.toml")`, run in the default verify mode only.
- **Fits because:** it pins the human-mode layouts from the layout excerpt as data files: `viola-list` piped/NO_COLOR, the `viola-send` readback mirror `[RB]`/`[  ]`/`[/ ]`, `viola-help`, and the `viola-verify` step counter. It also covers cross-surface-coordination: the CLI caption row must equal NAME · LIVE · STATUS · WHEEL · DIALOG · CLI.
- **Key detail:** snapbox redactions (`[..]`, `[EXE]`, custom substitutions) mask timestamps, cursors and temp paths so the human-output checks are deterministic. Expected files are regenerated only by a local refresh that is then committed, never as a gate step.
- **Source:** https://github.com/assert-rs/snapbox

## Per-Surface Driver: cli (`viola hook <event>` / `viola hook statusline`)

### assert_cmd (hook mode, with a std::time::Instant deadline)

- **Version:** 2.2.2
- **Last release:** 2026-05-11
- **Status:** actively maintained
- **Agent-runnable:** yes — it asserts `.code(0)`, `.stderr("")`, and stdout equal to the exact decision JSON or empty. Configuration: `.env("VIOLA_NAME", n).env("VIOLA_DIR", d).write_stdin(fs::read("fixtures/claude/<ver>/pre-tool-use.ask.json")?)`, plus `.timeout(Duration)` and a measured `Instant` bound for the spine and SessionEnd (~1 s) deadlines.
- **Fits because:** it covers three things:
  - the hook-stdin security trigger: oversize stdin, malformed JSON, a clap error, a channel failure, a forced panic and a missing `VIOLA_NAME` must each give exit 0, empty stderr and no body
  - the decision bodies of critical path 4 (dialog → `answer`)
  - the statusline passthrough of critical path 6 (budget governor)
- **Key detail:** use `.env_remove`/`.env_clear()` selectively. A test that clears the env must re-add `LLVM_PROFILE_FILE` (set by cargo-llvm-cov), or that hook run drops out of coverage.
- **Source:** https://docs.rs/assert_cmd/latest/assert_cmd/

## Per-Surface Driver: tui (`viola run` terminal passthrough, the human's side of the PTY)

### portable-pty (reused as the outer-PTY test driver)

- **Version:** 0.9.0 upstream latest; the arch pins `=0.8.1`
- **Last release:** 2025-02-11
- **Status:** actively maintained
- **Agent-runnable:** yes — the test owns the master side. It `spawn`s `viola run <name> -- <fake agent>`, writes keystrokes and `\x1b[200~…\x1b[201~` bracketed pastes, `resize`s, and waits on the child handle. Verdicts are the handle's exit code, `events.ndjson` JSON lines, and the fake agent's JSON receipt report. Screen content is never read. Configuration: a `tests/support/outer_pty.rs` helper over the same `viola-pty` seam (spawn · read · write · resize · wait · kill), draining the master reader on its own thread.
- **Fits because:** Sec 2 tui asks for a "PEXPECT-class outer-PTY driver… concrete crate is Phase 2's choice". It works the same on ConPTY (windows-2025) and openpty (ubuntu, macOS) for the multi-os-compat trigger. It drives:
  - critical path 5: a human key produces `wheel{holder:"human",cause:"human-input"}`, `send` exits 10, and `release` returns the wheel
  - R3/S1: one paste arrives as one prompt, newlines intact, no ESC
- **Key detail:** exit must be detected on the process handle (`child.wait()`), never on master EOF. ConPTY does not close the output stream when the child exits (creator excerpt, Windows).
  - This adds no new dependency. The wezterm monorepo that hosts portable-pty was last pushed 2026-09-17. The pinned 0.8.1 dates from 2023-03-13, but it is an arch decision.
  - Other expect-style crates were rejected. expectrl 0.9.0 depends on `conpty ^0.5`, whose latest release (0.7.0) is from 2024, and on `nix ^0.26`. rexpect 0.7.1 is Unix-only. rust-expect 0.6.1 has only about 726 downloads.
- **Source:** https://crates.io/crates/portable-pty ; https://github.com/wezterm/wezterm

## Per-Surface Driver: ipc-internal (wrapper channel, JSON-RPC 2.0 over ndjson)

### interprocess (raw local-socket client, alongside the viola-channel sync client)

- **Version:** 2.4.4
- **Last release:** 2026-09-03
- **Status:** actively maintained
- **Agent-runnable:** yes — response frames are parsed as JSON and asserted as `{"result":{"ok"|"refusal"}}` or `{"error":{code,message,data}}`. Configuration: protocol tests use the `viola-channel` sync client. Malformed-frame tests open a raw `interprocess::local_socket::Stream` to the endpoint read from `snapshot.json` and write bytes directly:
  - bad JSON → -32700
  - a frame over `MAX_FRAME` (16 MiB) → -32600 and the connection closes
  - a higher `params.v` → -32602 with `data:{supported,wrapper}`
- **Fits because:** it covers the IPC security-vector trigger: MAX_FRAME, higher `v`, `release`+`from` → `release-from-driver`, pid/start-time mismatch → exit 21, and a squatted endpoint → `run` exit 1. It also checks the `cursor` byte offset from critical path 2 against the `events.ndjson` length.
- **Key detail:** tests take the endpoint only from the snapshot. The "squatted endpoint" test creates the listener first with the interprocess server API and must never use `try_overwrite`, which the security excerpt rejects.
- **Source:** https://github.com/kotauskas/interprocess

### jsonschema (payload schema assertion)

- **Version:** 0.57.0
- **Last release:** 2026-09-21
- **Status:** actively maintained
- **Agent-runnable:** yes — `validator.iter_errors(&instance)` returns every violation with its `instance_path()`, so a failing test lists them all. Configuration: `let v = jsonschema::validator_for(&schema)?; assert!(v.iter_errors(&frame).next().is_none())`, using schemas emitted from the product's `schemars` 1.2.2 derivations.
- **Fits because:** Sec 2 ipc-internal asks for "JSON schema assertion". The same validator checks MCP `structuredContent`, CLI `--json` documents, `/api/sessions` and snapshot envelopes (multi-version-compat).
- **Key detail:** the security excerpt says schemars schemas are advisory because rmcp does not enforce them. Use jsonschema only on the test side, to check outputs and to craft negative inputs that violate the advertised schema.
- **Source:** https://github.com/Stranger6667/jsonschema

### windows-sys (security-descriptor read-back, already in the Stack)

- **Version:** 0.61.2
- **Last release:** 2025-10-06
- **Status:** actively maintained
- **Agent-runnable:** yes — a `#[cfg(windows)]` test reads the pipe or home DACL through `GetSecurityInfo` + `ConvertSecurityDescriptorToStringSecurityDescriptorW` and asserts the SDDL string equals `D:P(A;;GA;;;<user-SID>)(A;;GA;;;SY)`. The verdict is the libtest exit code.
- **Fits because:** Sec 1 marks viola-channel and viola-state as partially testable. A second OS account is not available on hosted runners, but the DACL the product sets can be read and asserted as the same user (filesystem trigger: "Windows DACL owner / ACE cases where the runner allows, stubbed otherwise").
- **Key detail:** it is already a Stack dependency (the `TerminateProcess` fallback). Tests only add the `Win32_Security_Authorization` features in dev-deps, and edition 2024 resolver 3 keeps dev-dep features out of release builds.
- **Source:** https://github.com/microsoft/windows-rs

## Per-Surface Driver: ipc-internal (MCP stdio server `viola mcp`)

### rmcp (client role, test-only)

- **Version:** 3.4.1
- **Last release:** 2026-09-23
- **Status:** actively maintained
- **Agent-runnable:** yes — `TokioChildProcess` spawns `viola mcp --home <tmp>`. `list_tools()` and `call_tool(CallToolRequestParam{…})` return a typed `CallToolResult{is_error, structured_content, content}`. Configuration: dev-dependency `rmcp = { version = "=3.4.1", features = ["client", "transport-child-process"] }`.
- **Fits because:** it is the Sec 2 MCP driver. Positive checks:
  - `isError:false` with `structuredContent` equal to the `ok` payload
  - refusals as `"refused: <reason>"`
  - `instance-unreachable` / `wrapper-fault` errors
  - no `release`/`pause`/`link`/`unlink` in `tools/list`

  It also covers the MCP security trigger (non-`ViolaName` target, non-u64 `dialog_id`, open-string `behavior`, control chars in `text`) and the check that "MCP and CLI return identical payloads".
- **Key detail:** the security plan limits product rmcp features to `server` + `transport-io`, because the 2026 advisories sit in `transport-streamable-http-server`/`auth`. Keep `client` and `transport-child-process` in dev-deps only, and assert with `cargo tree -e features -p viola --edges normal` that neither reaches the release graph.
- **Source:** https://github.com/modelcontextprotocol/rust-sdk ; https://modelcontextprotocol.io/docs/2026-07-28/develop/build-client

## Per-Surface Driver: api-service (GUI HTTP GET + SSE on 127.0.0.1)

### reqwest (real-socket client, with curl for raw probes)

- **Version:** 0.13.5
- **Last release:** 2026-09-08
- **Status:** actively maintained
- **Agent-runnable:** yes — it returns the HTTP status, headers and a JSON body (`.json::<Value>()`). SSE comes from `.bytes_stream()` fed into a small line parser for `event:`/`data:`/`id:`. `curl -s -o /dev/null -w '%{http_code}'` prints the status as parseable stdout. Configuration:
  - build the client with `reqwest::Client::builder().no_proxy().cookie_store(false)`
  - read the token from `<home>/ui/<port>.url`
  - `GET /?t=<token>` with `redirect(Policy::none())`, and assert 303 plus `Set-Cookie: viola_<port>=…; HttpOnly; SameSite=Strict`
  - send the cookie by hand on later requests
- **Fits because:** it covers the loopback-GUI trigger:
  - 403 `host-not-allowed` for wrong-case, trailing-dot or wrong-port Host (set with `.header(HOST, …)`), and for a missing Host via `curl -H 'Host:'`
  - 405 on non-GET, and 401 without the cookie
  - CSP, `nosniff`, `no-referrer`, CORP and `no-store` headers, with no `Access-Control-*`
  - a 127.0.0.1-only bind
- **Key detail:** leave reqwest's `gzip`/`brotli`/`zstd` features off in the test client. That keeps `Content-Encoding` visible, which proves "SSE never compressed / only `/assets/*` compressed". Resume tests send `Last-Event-ID` by hand, including one malformed pair, which must drop the whole header.
- **Source:** https://github.com/seanmonstar/reqwest

### axum-test (in-process router tests, with tower `ServiceExt::oneshot`)

- **Version:** axum-test 21.1.0 (tower 0.5.3)
- **Last release:** 2026-08-20 (tower 0.5.3: 2026-01-12)
- **Status:** actively maintained
- **Agent-runnable:** yes — typed status, header and JSON assertions whose verdict is the libtest exit code. Configuration: `TestServer::new(viola_ui::router(state))` for a mock transport, or `.http_transport()` for a random real port. Alternatively `router.oneshot(Request::get("/health").header(HOST,"127.0.0.1:47319")…)`.
- **Fits because:** it gives fast unit-level coverage without booting `viola ui`:
  - the Problem Details URNs (`urn:viola:problem:*`, `v` member)
  - the `/ready` `checks` matrix: a failing `claude_agents` alone keeps 200
  - the `/api/sessions` envelope shape
- **Key detail:** axum-test 21.x requires axum ≥ 0.8.8, which fits the arch's axum 0.8.9. The Host-allowlist layer is outermost, so every request must set Host explicitly.
- **Source:** https://docs.rs/crate/axum-test/latest ; https://github.com/tower-rs/tower

### eventsource-client (optional SSE convenience client)

- **Version:** 0.18.0
- **Last release:** 2026-08-10
- **Status:** actively maintained
- **Agent-runnable:** yes — a typed `SSE::Event{event_type, data, id}` stream the test asserts on. Configuration: `ClientBuilder::for_url(u)?.header("Cookie", c)?.header("Host", h)?.last_event_id(id).reconnect(ReconnectOptions::reconnect(false).build()).build()`.
- **Fits because:** it checks the SSE `id:` list of `<ViolaName>:<offset>` pairs and resume behaviour (E2E expansion "SSE `Last-Event-ID` resume"; loopback-GUI trigger).
- **Key detail:** disable auto-reconnect. Otherwise the client resends `Last-Event-ID` itself and hides the "no header → start at each file's end" behaviour. Keep the raw reqwest parser for the keep-alive (15 s) and compression assertions. The older reqwest-eventsource (2024) and eventsource-stream (2022) are outside the recency window. LaunchDarkly repo last pushed 2026-08-24.
- **Source:** https://docs.rs/eventsource-client/latest/eventsource_client/struct.ClientBuilder.html

## Per-Surface Driver: web-spa (`viola ui` strip bay)

### Playwright Test (@playwright/test), headless Chromium only

- **Version:** 1.63.0
- **Last release:** 2026-09-04
- **Status:** actively maintained
- **Agent-runnable:** yes — exit code is non-zero on failure. `reporter: [['json',{outputFile:'pw.json'}],['junit',{outputFile:'pw-junit.xml'}]]` writes structured results. Configuration:
  - install with `npx playwright install --with-deps chromium`, on ubuntu only
  - set `use: { headless: true }` and leave out `webServer`
  - harness `boot` starts `viola ui --home <tmp> --port <free>`, and the test reads `ui/<port>.url` and navigates to `/?t=<token>`
- **Fits because:** it is the Sec 2 web-spa driver. Assertions:
  - role queries: `getByRole('table')`, `'row'`, `'columnheader'` (NAME · LIVE · STATUS · WHEEL · DIALOG · CLI), `'log'` and `'status'`
  - `toHaveAttribute('data-dialog','pending')`
  - `viola-readback[data-rb=open→read|refused|unconfirmable]`
  - `toHaveTitle('DIALOG <name> · viola')`
  - `scrollWidth <= clientWidth` at widths 1024 and 760–1023

  These serve critical paths 2, 4 and 6 and cross-surface-coordination (tape and transfer-marker readback flip together).
- **Key detail:** use only the headless test-runner mode; codegen, UI mode and `toHaveScreenshot` are excluded (Sec 2: "DOM / attribute / text only"). CSP and Trusted Types violations are collected three ways, and the test fails if any fire:
  - `page.on('console')` (type `error`)
  - `page.on('pageerror')`
  - a `securitypolicyviolation` listener added with `page.addInitScript`

  Never set `bypassCSP`. Node is used on the test side only, because the page is Lit 3.3.3 with no build step.
- **Source:** https://playwright.dev/docs/release-notes ; https://github.com/microsoft/playwright/releases

### @axe-core/playwright (ARIA check)

- **Version:** 4.13.0
- **Last release:** 2026-08-11
- **Status:** actively maintained
- **Agent-runnable:** yes — `new AxeBuilder({page}).analyze()` returns a JSON `violations[]` array, and the test asserts it is empty.
- **Fits because:** the design excerpt makes ARIA roles the E2E query contract (`role="table"` rack, `role="log"` tape, the skip link as first tab stop). axe catches role, name and structure regressions that would break role-based queries.
- **Key detail:** run it after each state strip (empty, 503, cocked) renders, not only at load.
- **Source:** https://www.npmjs.com/package/@axe-core/playwright

## Per-Surface Driver: library-only (workspace crates)

### cargo-nextest (with `cargo test --doc`)

- **Version:** 0.9.146
- **Last release:** 2026-09-21
- **Status:** actively maintained
- **Agent-runnable:** yes — JUnit XML plus the exit code. Configuration: `cargo nextest run --workspace --profile ci && cargo test --workspace --doc`.
- **Fits because:** Sec 2 library-only: "language-native test runner only… on all three CI OSes".
- **Key detail:** a separate `cargo check -p viola-core -p viola-pty -p viola-state -p viola-channel -p viola-agent-claude` (the sync crates, with no `tokio` feature) is a compile-level test of the tokio ban. It belongs in the CI gate list.
- **Source:** https://nexte.st/

## CI Integration Pattern

### GitHub Actions hosted runner images (native `[windows-2025, macos-latest, ubuntu-latest]` matrix)

- **Version:** win25/20260922.270; ubuntu24/20260920.314; macos-26-arm64/20260907.0351
- **Last release:** 2026-09-23 (win25/20260922.270; ubuntu24 2026-09-21; macos-26-arm64 2026-09-08)
- **Status:** actively maintained
- **Agent-runnable:** yes — job conclusions and annotations are readable as JSON with `gh run view --json jobs`. JUnit, LCOV and `mutants.out` are uploaded as artifacts (actions/upload-artifact v7.0.1) and fetched with `gh run download`. Configuration: `strategy: { fail-fast: false, matrix: { os: [...] } }`, `permissions: {}` at the top with `contents: read` per job, and every action pinned by SHA (security excerpt, Vector 9).
- **Fits because:** multi-os-compat trigger and Founder Direction 4. Named pipe vs Unix socket, ConPTY vs openpty, and DACL vs modes each run natively. The Playwright job runs on ubuntu only (design excerpt: DejaVu fonts).
- **Key detail:** `macos-latest` has pointed to macOS 26 on Apple Silicon (arm64) since the migration began on 2026-06-15. `windows-2025` moved to Visual Studio 2026 by default in June 2026. Pin `macos-26` if a label change must not switch the target triple mid-version.
- **Source:** https://github.com/actions/runner-images/releases ; https://github.blog/changelog/2026-05-14-github-actions-upcoming-image-migrations/

### taiki-e/install-action

- **Version:** v2.87.19
- **Last release:** 2026-09-23
- **Status:** actively maintained
- **Agent-runnable:** yes — the step exit code is non-zero if an install fails. Configuration: `uses: taiki-e/install-action@<sha> with: { tool: "cargo-nextest,cargo-llvm-cov,cargo-mutants,cargo-deny" }`.
- **Fits because:** prebuilt binaries on all three OSes keep CI fast, and they pin the same tool versions harness `run` uses locally.
- **Key detail:** pin to the SHA of a tagged release, not `@v2`, which zizmor flags as unpinned-uses.
- **Source:** https://github.com/taiki-e/install-action

### Swatinem/rust-cache (with dtolnay/rust-toolchain)

- **Version:** v2.9.2
- **Last release:** 2026-08-06 (dtolnay/rust-toolchain uses branch refs; last commit 2026-09-19)
- **Status:** actively maintained
- **Agent-runnable:** yes — step exit codes. Configuration: `dtolnay/rust-toolchain@<sha>` with `toolchain: stable` and `components: rustfmt, clippy`, plus a second job at `1.89` for the MSRV. `Swatinem/rust-cache@<sha>` goes in `ci.yml` only.
- **Fits because:** these back the CI/CD pipeline steps (fmt, clippy `-D warnings`, check, tests, release build) on the 3-OS matrix.
- **Key detail:** the security excerpt rejects "rust-cache in the release workflow" because of cache poisoning, so it lives in `ci.yml` only. The floating `v2` tag once resolved to a commit still calling the deprecated actions/cache API, which is another reason to SHA-pin.
- **Source:** https://github.com/swatinem/rust-cache/releases ; https://github.com/dtolnay/rust-toolchain

### cargo-deny (with EmbarkStudios/cargo-deny-action)

- **Version:** 0.20.2
- **Last release:** 2026-07-09 (cargo-deny-action v2.1.1: 2026-07-13)
- **Status:** actively maintained
- **Agent-runnable:** yes — `cargo deny --format json check` prints JSON diagnostics, and the exit code is non-zero on bans, advisories or licences. Configuration in `deny.toml`:
  - a `[bans]` tokio wrapper allow-list limited to `viola-mcp`, `viola-ui` and `viola-channel[tokio]`
  - bans on C crates
  - `[advisories] ignore = ["RUSTSEC-2017-0008"]` as the only ignore
- **Fits because:** it covers the supply-chain trigger (ubuntu job plus the weekly advisories run) and the tokio-ban half of "std threads + Tokio only in mcp/ui".
- **Key detail:** cargo-deny checks dev-dependencies too. Test-only tokio users (the rmcp client, axum-test) must sit in crates already on the tokio allow-list, or in a dedicated test crate added to the wrappers list.
- **Source:** https://github.com/EmbarkStudios/cargo-deny

### zizmor (with zizmorcore/zizmor-action)

- **Version:** 1.30.1
- **Last release:** 2026-09-09 (zizmor-action v0.6.4: 2026-09-09)
- **Status:** actively maintained
- **Agent-runnable:** yes — `--format=json` or `--format=sarif` writes structured findings, and the exit code is non-zero when there are findings. Configuration: `zizmor --format=json .github/workflows/`.
- **Fits because:** supply-chain trigger ("zizmor on workflows, SHA-pinned Actions assertion"). Its `unpinned-uses` audit is the SHA-pin assertion.
- **Key detail:** zizmor handles the June 2026 GitHub Actions `parallel:` step blocks the same way as serial steps.
- **Source:** https://github.com/zizmorcore/zizmor ; https://docs.zizmor.sh/usage/

### cargo-modules (boundary review)

- **Version:** 0.27.0
- **Last release:** 2026-08-03
- **Status:** actively maintained
- **Agent-runnable:** yes — exit codes: `cargo modules dependencies --acyclic` exits non-zero on any module cycle, and `cargo modules orphans --deny` exits non-zero when any orphaned source file exists. Configuration: `cargo modules dependencies --package viola-core --acyclic` and `cargo modules orphans --package <crate> --deny`.
- **Fits because:** the Stack names cargo-modules for "boundary review" (the dependency and boundary policy entity, Sec 1).
- **Key detail:** the `structure` and plain `dependencies` output is text/DOT and is not a gate. Crate-level boundaries (viola-core has no viola deps; only viola-agent-claude knows Claude shapes) are enforced better by cargo-deny bans or a `cargo tree -e normal -p viola-core` assertion.
- **Source:** https://github.com/regexident/cargo-modules

## Structured Log Parsing

### tracing-subscriber (product-side JSON emission)

- **Version:** 0.3.23
- **Last release:** 2026-03-13
- **Status:** actively maintained
- **Agent-runnable:** yes — `fmt().json().flatten_event(true)` writes one JSON object per line. Configuration: `tracing_subscriber::fmt().json().flatten_event(true).with_current_span(false).with_writer(<per-process file under diagnostics/>)`.
- **Fits because:** Founder Direction 3 ("Structured JSON logs from every process (run/hook/mcp/ui)… hook trace in diagnostics/") and harness `logs` (Sec 3).
- **Key detail:** 0.3.23 is newer than the rejected `<0.3.20` (RUSTSEC-2025-0055). For `viola hook`, the writer must never be stderr, and a failed log write must be swallowed so the hook still exits 0.
- **Source:** https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/format/struct.Json.html

### jq (test-side log assertion; jaq as a Rust-native equivalent)

- **Version:** jq 1.8.2 (jaq 3.1.1)
- **Last release:** 2026-06-20 (jaq 3.1.1: 2026-08-05)
- **Status:** actively maintained
- **Agent-runnable:** yes — `jq -e '<predicate>'` exits 1 when the result is false or null, so a filter works as an exit-code assertion. Configuration: `viola-harness logs --instance b | jq -e 'select(.kind=="send-issued") | .data.cursor'`. Inside Rust tests, read `events.ndjson` line by line with `serde_json::from_str::<Value>`.
- **Fits because:** harness `logs` filters by instance and kind (Sec 3). It also serves the secret-logging trigger: a grep across all logs and `diagnostics/` must find no token, `Cookie`, `?t=` or stripped `CLAUDE*` values.
- **Key detail:** E2E waits key on logged events and byte offsets, never sleeps (discipline trigger). jaq takes the same filter language and behaves the same on all three OSes.
- **Source:** https://github.com/jqlang/jq/releases ; https://github.com/01mf02/jaq

## Mocking & Stubbing

### mockall (trait-seam doubles)

- **Version:** 0.15.0
- **Last release:** 2026-06-28
- **Status:** actively maintained
- **Agent-runnable:** yes — an unmet expectation panics, so the libtest/nextest exit code is non-zero and the JUnit case fails. Configuration: `#[cfg_attr(test, mockall::automock)] trait Pty { fn spawn(…); fn read(…); fn write(…); fn resize(…); fn wait(…); fn kill(…); }` on the `viola-pty` seam, plus liveness and clock traits.
- **Fits because:** it injects faults into the six-method pty seam (child exits without EOF, write failure) and the liveness probe (pid alive + stale heartbeat → `stale`) without real processes, which covers the chaos-test trigger at unit level. viola makes no outbound HTTP calls, so no HTTP mocking library applies. External process boundaries are stubbed with native test binaries instead (see the tempfile + std::process block under Integration Test Patterns).
- **Key detail:** this is dependency injection, not runtime monkey-patching (research-targets REJECT rule). Doubles exist only for traits the product already defines at its seams.
- **Source:** https://github.com/asomers/mockall

### tokio `test-util` (paused clock; async crates only)

- **Version:** 1.53.1 (feature `test-util`)
- **Last release:** 2026-07-20
- **Status:** actively maintained
- **Agent-runnable:** yes — `#[tokio::test(start_paused = true)]` auto-advances virtual time through sleeps, so timing tests are deterministic and finish quickly. The verdict is the libtest exit code.
- **Fits because:** performance-budget trigger. It tests the SSE keep-alive every 15 s (viola-ui, `advance(15s)`) and `wait` `timeout_ms` handling in viola-mcp without real 15-second waits. Pair it with one real-time smoke test per OS that sees a keep-alive comment within 16 s.
- **Key detail:** paused time only affects `tokio::time::Instant`, not `std::time::Instant`, and it needs the current_thread runtime. The sync crates (heartbeat 1 s / 5 s, the confirmation window) need an injected clock or mock_instant.
- **Source:** https://tokio.rs/tokio/topics/testing ; https://docs.rs/tokio/latest/tokio/time/fn.pause.html

### mock_instant (clock for the sync crates)

- **Version:** 0.6.1
- **Last release:** 2026-06-08
- **Status:** actively maintained
- **Agent-runnable:** yes — it is a library, and results come through the test exit code. Configuration: product code uses a `viola_core::Clock` alias that resolves to `mock_instant` types under `cfg(test)` or a `test-clock` feature, and tests call `MockClock::advance(Duration::from_secs(6))`.
- **Fits because:** it covers heartbeat staleness flipping at >5 s, the `send` confirmation window, and `budget_override_until` expiry in the tokio-free crates (performance-budget and chaos triggers).
- **Key detail:** an explicit `Clock` trait injected through constructors is the cleaner option. If mock_instant is used, pick its global mode rather than thread-local, so the wrapper's heartbeat thread sees the advance.
- **Source:** https://crates.io/crates/mock_instant

## Integration Test Patterns

### tempfile + std::process (harness `boot` / `cleanup` pattern, stubbing processes rather than HTTP)

- **Version:** 3.27.0
- **Last release:** 2026-03-11
- **Status:** actively maintained
- **Agent-runnable:** yes — `boot` returns a JSON readiness report built from state it can query:
  - `snapshot.json` has `endpoint`/`pid`/`started_at`/`child_pid`
  - the `heartbeat` mtime is under 5 s
  - the log shows `wheel{cause:"start"}` → `budget-gate` → `session-start`
  - `GET /health` and `/ready` return 200

  `cleanup` kills on the process handle and asserts that a verb against the name exits 21. Configuration: start `viola run` with `std::process::Command`, or the outer PTY for tui tests, inside the TempDir home, with a `Drop` guard that kills the wrapper.
- **Fits because:** it implements the Sec 3 five-command harness (boot/run/status/cleanup/logs) and critical path 1 (`run` start ordering). The child in `viola run <name> -- <fake agent>` is a workspace test binary resolved through `CARGO_BIN_EXE_<name>`. It:
  - answers `--version`
  - writes JSON receipt lines (prompts, env, raw bytes) into the temp home
  - replays `fixtures/claude/<ver>/` hook payloads by calling the pinned `viola hook`

  A stub `claude` first on a test-scoped `PATH` emits recorded, oversize or malformed `claude agents --json`. Together these cover the child-spawn trigger, the R8 env strip, critical path 7 (unlisted `--version`) and critical path 2 (`no-prompt-submitted` when the fake suppresses UserPromptSubmit).
- **Key detail:** `assert_cmd`'s `.assert()` blocks until exit, so do not use it for the long-lived wrapper. Build the stubs as native `.exe` binaries, except for the one negative test where a `.cmd`/`.bat` PTY child must make `run` exit 1 (security excerpt, Vector 8).
- **Source:** https://docs.rs/tempfile ; https://doc.rust-lang.org/std/process/struct.Command.html

## Contract Testing (fake agent vs recorded `viola verify` fixtures)

[trigger-driven; pulled in by contract-test-against-sandbox from test-scope Sec 5; not a standard research-targets category but required for trigger coverage]

### insta (normalised snapshots of decision bodies and fake-agent transcripts)

- **Version:** 1.48.0
- **Last release:** 2026-06-11
- **Status:** actively maintained
- **Agent-runnable:** yes — in CI (`CI` env set), or with `INSTA_UPDATE=no cargo insta test --check`, a mismatch fails the test with a non-zero exit and no `.snap.new` files are written. Configuration: `insta::assert_json_snapshot!(decision, { ".dialog_id" => "[id]", ".ts" => "[ts]" })`.
- **Fits because:** per `<cli-version>`, it pins the exact hook decision JSON: PreToolUse `allow` + `updatedInput.answers` including `annotations`, and PermissionRequest `allow|deny` + `message`. It also pins the fake agent's replayed hook sequence against the recorded fixtures, which guards against an S8-style "instrument defect".
- **Key detail:** only check mode belongs in the agent loop; the interactive review/accept flow is excluded. Snapshots are regenerated only through the local `viola verify` fixture refresh, then committed.
- **Source:** https://insta.rs/docs/cli/ ; https://github.com/mitsuhiko/insta

### jsonschema (fixture schema conformance)

- **Version:** 0.57.0
- **Last release:** 2026-09-21
- **Status:** actively maintained
- **Agent-runnable:** yes — `iter_errors` returns every violation with its instance path, and the verdict is the libtest exit code. Configuration: one test iterates `fixtures/claude/*/*.json` and validates each against the parser's tolerant schema. Unknown fields are allowed; the dialog-kind and decision enums are closed and required.
- **Fits because:** multi-version-compat trigger ("unknown event kinds / fields counted in `skipped` rather than failing") and the contract suite.
- **Key detail:** run it in the same walk as the path/username scrub check over the fixture tree (security excerpt, Anti-Patterns).
- **Source:** https://github.com/Stranger6667/jsonschema

## Test Data Strategy

### proptest strategies as generators

- **Version:** 1.11.0 (proptest-derive 0.8.0)
- **Last release:** 2026-03-24 (proptest-derive 0.8.0: 2026-02-05)
- **Status:** actively maintained
- **Agent-runnable:** yes — a failure prints the minimal shrunk case and saves its seed to `proptest-regressions/`, which is committed and replayed in CI. The verdict is the libtest exit code.
- **Fits because:** it gives self-bootstrapping data for the property-test trigger. Generators cover:
  - `ViolaName` values (`[a-z0-9-]`) plus invalid neighbours such as `../x`, uppercase and `/`
  - `Percent` 0–100 plus out-of-range values
  - event lines
  - `Last-Event-ID` pair lists
  - statusline `rate_limits` JSON with a malformed `resets_at`
- **Key detail:** no developer-seeded state (Sec 3). `budget.json` comes only from generated statusline stdin piped into `viola hook statusline`, never from a hand-written file.
- **Source:** https://github.com/proptest-rs/proptest

## Test Isolation Patterns

### cargo-nextest test groups (serial_test as a fallback)

- **Version:** 0.9.146 (serial_test 4.0.1)
- **Last release:** 2026-09-21 (serial_test 4.0.1: 2026-07-25)
- **Status:** actively maintained
- **Agent-runnable:** yes — JUnit XML plus the exit code. Configuration: `[test-groups] fixed-port = { max-threads = 1 }` with `[[profile.default.overrides]] filter = 'test(/default_port/)' test-group = 'fixed-port'` in `.config/nextest.toml`. `serial_test::serial` covers the same tests under plain `cargo test`.
- **Fits because:** the per-test `--home` already isolates the endpoint, log and `budget.json`, because the FNV-1a hash includes the absolute home path (upstream §1). Only process-global resources need serialising: the default port 47319, env mutation, and a `PATH` set in-process.
- **Key detail:** set env per child (`Command::env`) rather than `std::env::set_var`. In edition 2024, `set_var` is `unsafe` and races in multi-threaded test binaries.
- **Source:** https://nexte.st/ ; https://github.com/palfrey/serial_test

## Property-Based Testing

### proptest

- **Version:** 1.11.0
- **Last release:** 2026-03-24
- **Status:** actively maintained
- **Agent-runnable:** yes — the libtest exit code gives the verdict, a failure prints the minimal input, and `proptest-regressions/*.txt` seed files reproduce it. Configuration: `proptest! { #![proptest_config(ProptestConfig{ cases: 512, ..Default::default() })] #[test] fn paste(s in any::<String>()) { … } }`. test-strategy 0.4.5 (2026-02-11) offers an optional attribute syntax.
- **Fits because:** it covers every surface the property-test trigger names:
  - `validate_paste_text`: the check runs on decoded `char`s, not bytes, so C1 U+0080–U+009F is refused while UTF-8 continuation bytes pass
  - the `Last-Event-ID` parser: one bad pair drops the whole header
  - ndjson framing at `MAX_FRAME`
  - the hook stdin parser, including statusline
  - the `agents --json` parser
  - `prompt-submitted` normalisation (`<pasted_content` / `<\pasted_content`)
  - the vt100 feed under `catch_unwind`
- **Key detail:** commit `proptest-regressions/` so CI and cargo-mutants replay known failing seeds. Seeds, not values, are what gets persisted. The crate's MSRV is 1.85, which is below the arch's 1.89.
- **Source:** https://proptest-rs.github.io/proptest/proptest/failure-persistence.html ; https://crates.io/crates/proptest

### cargo-fuzz (with arbitrary; coverage-guided, ubuntu nightly job)

- **Version:** 0.13.2 (arbitrary 1.4.2)
- **Last release:** 2026-06-09 (arbitrary 1.4.2: 2025-08-14)
- **Status:** actively maintained
- **Agent-runnable:** yes — `cargo +nightly fuzz run <target> -- -max_total_time=120` exits non-zero on a crash and writes the reproducer to `fuzz/artifacts/<target>/`. `-runs=0` replays the corpus as a regression gate. Configuration: a `fuzz/` directory with one target per parser, fed by `arbitrary::Arbitrary` inputs shared with the proptest strategies.
- **Fits because:** the security excerpt lists these parsers for "fuzz/property coverage ('tests owns the cases')". Coverage guidance reaches panics in the vt100 feed and the hook stdin parser that random proptest input rarely hits.
- **Key detail:** it needs a nightly toolchain and LLVM sanitizers. The cargo-fuzz README says it is Unix-only, while the Fuzz Book mentions Windows via MSVC ASan. Run it as a separate time-boxed ubuntu job so it touches neither the MSRV/stable matrix nor Windows.
- **Source:** https://github.com/rust-fuzz/cargo-fuzz ; https://rust-fuzz.github.io/book/cargo-fuzz/setup.html

## Chaos & Fault Injection

### sysinfo + std (process- and file-level fault injection, already in the Stack)

- **Version:** 0.39.6
- **Last release:** 2026-07-09
- **Status:** actively maintained
- **Agent-runnable:** yes — every fault is followed by an assertion on queryable state:
  - `skipped.torn_lines` in `/api/sessions` or `list --json`
  - snapshot fields after replay
  - exit 21 or MCP `instance-unreachable`
  - `liveness` `stale` vs gone

  Configuration:
  - kill the wrapper mid-write with `Child::kill()` or `sysinfo::Process::kill_with(Signal::Kill)`
  - cut the last ndjson line with `File::set_len(len - k)`
  - corrupt `snapshot.json` or give it an unsupported `v`
  - stop the heartbeat while the pid lives (SIGSTOP on Unix)
  - use a fake-agent mode that exits without closing its PTY output
- **Fits because:** chaos-test trigger. It covers:
  - torn-line healing
  - replay recovering only `links`/`agent_session_id`/`wheel`/`budget_paused`/`budget_override_until`, with `dialog_pending:false`
  - handle-based exit detection (ConPTY)
  - the endpoint vanishing during `wait`
  - stale heartbeat vs live pid
- **Key detail:** after a replay, liveness must be `live` or gone, never `stale`, and one test should assert both halves. No maintained failpoint crate compatible with the tokio ban was found for 2025–2026:
  - fail-parallel 0.6.0 is maintained, but it has a mandatory, non-optional `tokio ^1.40` dependency, which violates the `cargo deny` tokio ban on the sync hot path
  - fail and failpoints were last released in 2022

  Fault seams therefore go through injected traits (mockall) plus the process and file faults above.
- **Source:** https://github.com/GuillaumeGomez/sysinfo ; https://crates.io/crates/fail-parallel

## Performance & Load Testing

### hyperfine (process-level latency budget)

- **Version:** 1.20.0
- **Last release:** 2025-11-18
- **Status:** actively maintained
- **Agent-runnable:** yes — `--export-json out.json` holds per-run times, mean, max and exit codes. A `jq -e '.results[0].max < 1.0' out.json` gate turns a budget breach into a non-zero exit. Configuration: `hyperfine -N --runs 30 --export-json hook-end.json 'viola hook session-end --home <tmp>' --input fixtures/claude/<ver>/session-end.json`.
- **Fits because:** performance-budget trigger. Spine hooks (SessionStart, UserPromptSubmit, Stop) must finish within the spine deadline and SessionEnd within ~1 s. hyperfine measures this on each OS runner, including process spawn cost.
- **Key detail:** gate on `max`, not `mean`, because the Claude Code deadline is a hard cutoff. `-N` (no shell) removes shell-startup noise on Windows. In-test `Instant::now()` bounds inside the assert_cmd hook tests give the same check at unit speed.
- **Source:** https://github.com/sharkdp/hyperfine

### criterion (micro-benchmarks)

- **Version:** 0.8.2
- **Last release:** 2026-02-04
- **Status:** actively maintained
- **Agent-runnable:** yes — it writes machine-readable `target/criterion/<bench>/new/estimates.json` and supports `--save-baseline`/`--baseline` comparisons. Configuration: `cargo bench -p viola-core --bench paste -- --save-baseline main`.
- **Fits because:** it tracks regressions in the hot parsers on the sync path (`validate_paste_text`, ndjson line parsing, the vt100 feed).
- **Key detail:** criterion does not exit non-zero on a regression. A gate needs a `jq` comparison over `estimates.json`, so hyperfine stays the budget gate. The project now lives in the criterion-rs org.
- **Source:** https://github.com/criterion-rs/criterion.rs

## Multi-Platform Compat Matrix

### GitHub Actions hosted runner images (native matrix, no cross-compilation)

- **Version:** win25/20260922.270; ubuntu24/20260920.314; macos-26-arm64/20260907.0351
- **Last release:** 2026-09-23 (win25/20260922.270; ubuntu24 2026-09-21; macos-26-arm64 2026-09-08)
- **Status:** actively maintained
- **Agent-runnable:** yes — per-OS job conclusions via `gh run view --json jobs`, and per-OS JUnit artifacts named `junit-${{ matrix.os }}.xml`.
- **Fits because:** multi-os-compat trigger: "full workspace and fake-agent E2E on all three runners". The OS branches run natively: named pipe vs Unix socket, ConPTY vs openpty, DACL vs modes, and macOS euid + dir-only verification. Cross-compiled binaries cannot exercise ConPTY or DACLs, so the matrix stays native and `cross` is not used.
- **Key detail:** `#[cfg(windows)]` / `#[cfg(unix)]` / `#[cfg(target_os = "macos")]` tests only compile on their own runner ("Every OS-specific branch compiles and is tested on its CI runner"), so a Linux-only local run is never proof for the other two. `macos-latest` now means macOS 26 on arm64 (aarch64-apple-darwin).
- **Source:** https://github.com/actions/runner-images ; https://github.blog/changelog/2026-05-14-github-actions-upcoming-image-migrations/

## Mutation Testing

[founder-mandated; pulled in by the mutation-testing discipline trigger from test-scope Sec 5 (Founder Direction 1); required, not optional]

### cargo-mutants

- **Version:** 27.1.0
- **Last release:** 2026-06-02
- **Status:** actively maintained
- **Agent-runnable:** yes — exit codes and machine-readable output files:
  - 0: all caught
  - 1: usage error
  - 2: missed mutants
  - 3: timeouts
  - 4: baseline failing
  - 5: the diff does not match the tree
  - 6: invalid diff
  - 70: internal error

  Results go to `mutants.out/outcomes.json` plus `missed.txt`/`caught.txt`/`timeout.txt`/`unviable.txt`. Configuration: `git diff <chunk-base>...HEAD > chunk.diff && cargo mutants --workspace --in-diff chunk.diff --test-tool=nextest --cargo-arg=--profile=mutants`.
- **Fits because:** Founder Direction 1: "cargo-mutants scoped to each chunk diff, surviving mutants are red". It is harness `run` step 4 (Sec 3), which exits non-zero on any survivor and reports `survived` in its JSON summary.
- **Key detail:** exit 3 (timeout) takes precedence over exit 2, so a run with one timeout and any number of missed mutants exits 3. The harness must count `missed` in `outcomes.json` or require an empty `missed.txt`, not trust the exit code alone. Two more limits:
  - `--in-diff` matches only code under test, so a test-only chunk runs zero mutants
  - nextest skips doctests, so a mutant caught only by a doctest reads as missed

  27.1.0 added `start_time`/`end_time` to `outcomes.json` and fixed `--in-diff` erroring on non-UTF-8 or binary-file diff content.
- **Source:** https://mutants.rs/exit-codes.html ; https://mutants.rs/in-diff.html ; https://mutants.rs/nextest.html ; https://github.com/sourcefrog/cargo-mutants/releases
