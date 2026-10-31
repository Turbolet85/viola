# tests · Test Strategy — category read

## Carried
- (.andromeda/test-plan-detail/test-strategy.md:10) The exit-code table must stay this typed set: causes are told apart inside exits 21 and 1, and no case may add an exit code to it.
  > rstest 0.27.0 `#[case]` tables cover the refusal-order matrices, the exit-code table (0/1/2/10–14/20/21) and the per-`<cli-version>` fixture replay.
- (.andromeda/test-plan-detail/test-strategy.md:10) Any captured-span case asserting a server-verification code must be compiled out on macOS with the others, since the check reads no peer pid there.
  > Runs on all 3 OSes, except the two real-cargo-mutants harness tests (§10 Mutation gate) and the server check's captured-span cases, compiled out on macOS, where the server check reads no peer pid:
- (.andromeda/test-plan-detail/test-strategy.md:10) A new or moved server-check case must gate only itself on macOS unless every case in its module is gated.
  > (a module is gated whole only when its every case is gated: `src/cmd/link.rs` gates its server-check case alone, and its reply-shape and line cases run on macOS)
- (.andromeda/test-plan-detail/test-strategy.md:14) The CLI and MCP contract moved by this chunk must keep its suites on real endpoints, real tempfile homes and the real binary, never on stand-ins.
  > Every arch Standard Contract (Wrapper channel, On-disk, Hooks, CLI, MCP) has a crate-level `tests/` suite, using a real `interprocess` endpoint, a real tempfile home, and assert_cmd against `CARGO_BIN_EXE_viola`.
- (.andromeda/test-plan-detail/test-strategy.md:29) A witness that neutralises a refusal guard and so lets a product process start (a run, revive or verify cause) must spawn through `Running`, bounded by `WITHIN`.
  > Beside it stands the per-chunk remove-the-guard rule (`.claude/rules/testing.md`): a control whose neutralised guard lets a product process start needs a bounded witness; the G4 / G10 witnesses spawn through `tests/support/cli.rs` `Running`, bounded by `WITHIN`, since nextest's default profile has no kill line
- (.andromeda/test-plan-detail/test-strategy.md:42) Each exit-cause case must read its exit code, stdout document and last stderr `hint:` line through this one runner and its two captures, with a planted bin dir only through `spawn_on` / `viola_on`.
  > `cli.rs` holds the one started-child guard and runner (`Running`, `Ran`, `spawn`, `viola`), with stdout and stderr as two captures beside the exit code, and `spawn_on` / `viola_on`, which put a planted bin dir on the child's `PATH`; a plain `spawn` of `list` runs with `PATH` set to an empty bin dir, so no test reads the host's `claude` sessions.
- (.andromeda/test-plan-detail/test-strategy.md:42) The new table must use the shared error-body check on any document it pins, reporting place and class and never bytes.
  > the one error-body check, `error_body_hit` and `assert_error_body_clean` (classes `absolute-path` · `caused-by` · `serde-path` · `planted`, reporting place and class, never bytes)
- (.andromeda/test-plan-detail/test-strategy.md:42) A root table must define no local copy of the shared reader, wait, guard or result struct.
  > No root test file outside `tests/support/` defines its own copy of the reader, the wait, the guard or the result struct.
- (.andromeda/test-plan-detail/test-strategy.md:43) The `cross_` prefix must land in a home this naming rule admits and the E2E layer's regex selects, which the scope's root `tests/cross_exit_causes.rs` has to be reconciled with.
  > **Tokio-client E2E suites and the harness binary `viola-harness`:** `crates/viola-e2e/` (publish = false). This covers the rmcp client, cross-surface parity and critical-path scenarios, as `tests/{path<N>,mcp,cross}_<topic>.rs`
- (.andromeda/test-plan-detail/test-strategy.md:43) The table's binary must report in the E2E layer under this regex, whichever home it takes.
  > both homes report in the E2E layer, whose regex is `^(path[0-9]*|tui|mcp|cross|chaos|contract)_` (§3 5-command `run` step 2).
- (.andromeda/test-plan-detail/test-strategy.md:58) The server-verification cause's new code must be asserted in this file, off macOS, and this row's wording must hold after the move.
  > root `security_negatives_server_identity.rs`, off macOS: a CLI frame verb against a server that is not the recorded wrapper
- (.andromeda/test-plan-detail/test-strategy.md:63) This row names the document the chunk may move, so it must be amended with the contract choice, keeping exit 21, no frame and no line.
  > each of the six frame verbs `send` · `wait` · `last` · `answer` · `pause` · `release` refusing a widened instance before any log, exit 21 with the instance pair or the `instance-unreachable` document, no frame and no line
- (.andromeda/test-plan-detail/test-strategy.md:73) Every new code and writer must stay inside these three surface witnesses and the census of writers, and each must pass green with the new codes.
  > (the three surface witnesses, root `cli_error_bodies.rs` `cli_error_documents_hold_no_path_or_upstream_text`, root `channel_error_bodies.rs` `channel_error_bodies_hold_no_path_or_upstream_text` and `crates/viola-e2e/tests/mcp_tools.rs` `mcp_tools_error_results_hold_no_path_or_upstream_text`; root `contract_error_bodies.rs`, the enumeration and the census of writers)
- (.andromeda/test-plan.md:62) The CLI entity holds `revive`'s documents on both arms, so `revive --json` must print a document for the refusals after a passed preflight (the `v1-39` gap).
  > (the verbs, global `--home`, exits 0/1/2/10–14/20/21, `revive`'s `--json` documents on both arms)
- (.andromeda/test-plan.md:94) A cause with nothing on stdout, such as `viola mcp`'s refused home check, must be named outside this one-document signal or joined to the table with a reason.
  > its signal is the exit code plus one JSON document on stdout (the exit-2 unusable paths of `answer` and `send` excepted: a fixed stderr line and no document)
- (.andromeda/test-plan.md:135) These four start refusals must each stay exit 1 and each take its own code and case.
  > a second `run` of the same name, a squatted endpoint, a tampered pinned exe or a refused home tree exits 1.
- (.andromeda/test-plan.md:184) This trigger names MCP's `instance-unreachable` for the server-verification cause, so it must be amended with the contract choice, its exit 21 and silent hook unchanged.
  > server pid / start-time mismatch vs `snapshot.json` → CLI exit 21 / MCP `instance-unreachable` / hook exit 0 with no body
- (.andromeda/test-plan.md:196) MCP's distinct codes must stay codes only.
  > `isError` payloads carry codes only.
- (.andromeda/test-plan.md:241) The endpoint-lost-mid-call cause must keep this chaos case, read through whichever code the contract choice gives it.
  > an endpoint vanishing during `wait` read `instance-unreachable`
- (.andromeda/test-plan.md:269) Every new code on each surface must satisfy these sanitisation assertions.
  > assertions that CLI `--json`, MCP `isError` and channel `error.data` contain no absolute paths, serde paths or values, anyhow chains or tool `input`.
- (.andromeda/test-plan.md:327) Named tests in the table and its sibling files must follow this function naming.
  > General pattern: snake_case `<subject>_<condition>_<expected>`
- (.andromeda/test-plan.md:361) The table's form depends on its home: labelled rstest `#[case]`s in the root package, a labelled case table in `viola-e2e`, with no rstest edge added for it.
  > Table-driven cases use rstest `#[case]`, and each case name must be a readable label (for example `#[case::esc_refused]`). The one exception is `viola-e2e`, which carries no `[dev-dependencies]`: its table-driven cases are a labelled case table (an array of `(label, …)` rows in one `#[test]`, each assertion naming its label), so no manifest edge is added for rstest alone.
- (.andromeda/test-plan.md:393) The root bin's unit cases must hold each closed unreachable cause logged inside its span, with any new cause joining them.
  > the server check refused inside each frame verb's and the hook's span with each closed unreachable cause logged
- (.andromeda/test-plan.md:393) Every new code must pass this reply screen as a code word, and a value that is none must still read `unknown` or null.
  > the reply screen (a `refusal` that is no code word reads `unknown`, a string `detail` that is no code word reads null, a peer fault reads through `sanitised_fault`)
- (.andromeda/test-plan.md:444) These exits and the no-line rule must hold through the catalogue, and the rule's `instance-unreachable` wording must move with the contract choice.
  > strict-modes refusals give exit 1 for `run` / `mcp` / `viola revive` / `viola verify`, hook 0 with no output, `instance-unreachable` from the MCP frame tools, and CLI 21 for every other verb (the six frame verbs printing the instance pair); a refused check at `run`, `viola revive`, `viola verify`, the frame verbs, the hook and the MCP frame tools writes no line.
- (.andromeda/test-plan.md:455) Every test-started `viola` in the table must drop these variables, and a path oracle must use the resolved home.
  > A test-started `viola` drops `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN`, and a path oracle compares `resolved(home)`, never `TestHome::path()`.
- (.andromeda/test-plan.md:553) The unwrapped-name hint is owed to this chunk and must be built and witnessed beside the exit 21, and this as-landed note amended when it lands.
  > A `send` to the unwrapped row's name exits 21: a row `name` is never a target. As landed: `tests/cli_list.rs` `e1_send_naming_an_unwrapped_row_is_unreachable` (exit 21; the unwrapped-name hint not built, owed to "Exit-cause code catalogue").
- (.andromeda/test-plan.md:608) Any new `process-exit` `detail` value must validate against the diag-line schema, so the schema's closed list must carry it.
  > every home-level `diagnostics/*.ndjson` line under `target/e2e-home/**` is validated with jsonschema 0.57.0 against obs-plan's `schemas/diag-line.v1.json`
- (.andromeda/test-plan.md:610) This is the matrix the chunk must build: one negative per cause, its three assertions, a shared hint failing the table, each named cause in its named binary, and no `release` in a driver-facing hint.
  > - **Exit-cause matrix** (`cross_exit_causes.rs`, not built): one negative test per cause of exit 21 and of exit 1, each asserting the exit code, the `--json` document and a last `hint:` line that names its cause — a hint shared across causes fails the table; the server-verification cause is a case of `tests/security_negatives_server_identity.rs` on Linux and Windows (its macOS case owed to the route entry "Unix endpoint and home hardening"), the strict-modes cause of exit 21 for the six frame verbs is a case of `tests/security_negatives_strict_modes.rs` on three OSes, the exit-1 home-tree refusals of `run` and `verify` are cases of `tests/run_cli.rs` and `tests/cli_verify.rs`, and `revive`'s refusals are cases of `tests/cli_revive.rs`; driver-facing hints never contain `release`.
- (.andromeda/test-plan.md:612) `viola list`'s row owed by the carry must join this security-control suite, and the socket-directory refusal must keep its negative here.
  > - **Security control negatives** (`security_negatives_*.rs`): one negative test per security-plan control that had none — the Unix socket directory refusal
- (.andromeda/test-plan.md:617) MCP and CLI must still agree for a refusal, and the chunk must state how the new exit-cause codes read across the two surfaces.
  > for the same refusal, the MCP `structured_content` `{refusal, detail}` equals the CLI `--json` `{refusal, detail}`
- (.andromeda/test-plan.md:638) A case planting a home must make it this way, or the refusal it reads is the plain home's and not its own cause's.
  > a test that makes a home itself makes it through `viola_state::fs::create_private_dir`, never `create_dir_all`
- (.andromeda/test-plan.md:646) The `revive` and `list` cases, including a `claude` not on `PATH` as `internal-error`, must run under a test-scoped `PATH`.
  > every test-started `list` / `revive` runs under a test-scoped `PATH`.
- (.andromeda/test-plan.md:820) New cause and hint code must clear these thresholds on each OS job.
  > Gates are enforced per OS job with `cargo llvm-cov nextest … --fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80`.
- (.andromeda/test-plan.md:858) A recurrence of the WATCH's `verify_stopped_during_the_resume_run_removes_its_dir_and_exits_1` margin must be handled under this rule, with no retry and no `#[ignore]`.
  > If a test flakes once, it gets quarantined immediately and fixed at the root cause, never with a retry-once budget. For viola, quarantine means the chunk stays red until the root cause is fixed within that chunk. `#[ignore]` is not allowed as a parking place.
- (.andromeda/test-plan.md:875) Code-word, reply-screen and hint-selection cases that a lower layer can hold must sit there, with the table asserting only cross-process behaviour.
  > NEVER assert at E2E a case that a lower layer can hold.
- (.andromeda/test-plan.md:880) The Windows and macOS cause rows must be read on their own CI runners, never concluded from a Linux run.
  > NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
- (.andromeda/test-plan.md:888) The table's expected exit codes must be test literals, and its codes held the same way rather than imported from the product.
  > The expected values (the 11 identity-floor names, the refusal order, exit codes) are written as literals in the test.
- (.andromeda/test-plan.md:905) The already-live, stale and squatted cases must hold their long-lived first wrapper through the `Drop` guard, never `.assert()`.
  > NEVER use `assert_cmd` `.assert()` on the long-lived `viola run` wrapper. It blocks until exit; use std::process + a `Drop` guard.

## Bindings
- **Exit-cause table's home ↔ §2 naming and §4 grouping**: the scope's root rstest `tests/cross_exit_causes.rs` meets a `cross_` prefix placed in `viola-e2e`, which carries no rstest; the home and the table's form must be chosen together, (.andromeda/test-plan-detail/test-strategy.md:43) and (.andromeda/test-plan.md:361)
- **Exit-cause matrix ↔ security-plan and `.claude/rules/security.md`**: a moved `instance-unreachable` document amends the strict-modes wording on both sides, with no line and no frame kept, (.andromeda/test-plan-detail/test-strategy.md:63) and (.andromeda/test-plan.md:444)
- **Exit-cause matrix ↔ route entry "Unix endpoint and home hardening"**: the macOS server-verification case stays that entry's, so this chunk's case runs on Linux and Windows only, (.andromeda/test-plan.md:610) and (.andromeda/test-plan-detail/test-strategy.md:10)
- **Process-log codes ↔ obs-plan `schemas/diag-line.v1.json`**: tests own the check and obs owns the schema, so a new `process-exit` detail must land in the schema for the check to pass, (.andromeda/test-plan.md:608)
- **Error-body witnesses ↔ `viola_core::is_code_word` and `viola_channel::sanitised_fault`**: a new code must pass the screen and stand in the census, or the reply reads `unknown` or null, (.andromeda/test-plan.md:393) and (.andromeda/test-plan-detail/test-strategy.md:73)
- **Unwrapped-name hint ↔ design-system cli pattern 2**: the hint's text is the design-system's and its witness is this plan's, (.andromeda/test-plan.md:553)
- **Remove-the-guard witnesses ↔ `.claude/rules/testing.md`**: the rule's home is that file and its bound is this plan's `Running` and `WITHIN`, (.andromeda/test-plan-detail/test-strategy.md:29)
