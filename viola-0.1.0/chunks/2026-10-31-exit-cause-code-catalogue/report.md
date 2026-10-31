# Report — 2026-10-31-exit-cause-code-catalogue

**Chunk:** every exit-21 and exit-1 cause a distinct code across --json, MCP and process log, matching its human hint
**Date:** 2026-10-31T18:15Z
**Commits:** `8a4c1a2c fix(2026-10-31-exit-cause-code-catalogue): the handlers text helper compiled on macOS too` · `115cdbb9 chore(2026-10-31-exit-cause-code-catalogue): operator pre-CI commit, for the run this chunk's verdict reads` · ahead of them, the spine door's two, no source of this chunk: `404f7496 chore(records-door): commit B, the spine moved into the store`, `df4ca577 chore(records-door): the bookkeeping left after the last wrap, ahead of the spine door` · `16ef6b48` is the last wrap's own

## Changes (structured — detectors read this)
- **Files:** 41, `git diff --stat 404f7496 8a4c1a2c` reads `1494 insertions(+), 342 deletions(-)`; `gate.py scope` reads `scope: clean — changed 41 · listed 41 · recorded 0`. New: `tests/cross_exit_causes.rs` (567 lines). Modified: `.config/nextest.toml`, `Cargo.toml`, `schemas/diag-line.v1.json`, `crates/viola-core/src/lib.rs`, `crates/viola-state/src/liveness.rs`, `crates/viola-mcp/src/result.rs`, `crates/viola-mcp/src/tools.rs`, `src/main.rs`, `src/human/hints.rs`, `src/cmd/{mod,client,send,wait,last,answer,pause,link,list,plugin,run,revive,verify}.rs`, and the tests `crates/viola-mcp/tests/handlers.rs`, `crates/viola-e2e/tests/mcp_tools.rs`, `tests/support/start.rs`, `tests/contract_error_bodies.rs`, `tests/chaos_wait_vanish.rs`, `tests/cli_{send,list,machine_contract,wait_last,controls_not_disableable,plugin_install,links,error_bodies,instance_state,revive,verify}.rs`, `tests/security_negatives_{server_identity,strict_modes}.rs`.
- **Symbols / APIs:**
  - **The exit-21 document moved.** Every cause used to print one document, `{"v":1,"error":"instance-unreachable","detail":null}`. It now reads `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, `v` still 1, MCP's `isError` result the same less `v`, MCP's text still `refused: instance-unreachable` (`inputs#I2` Q1, the founder's own word). The five closed causes: `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`. No exit moved, no `RefusalReason` and no exit code was added, and `error` stays `instance-unreachable`.
  - `viola_core::UnreachableCause` (`crates/viola-core/src/lib.rs:181-190`, `as_str` `:193-201`): `StrictModes` · `InstanceDead` · `UnwrappedName` · `EndpointLost` · `ServerVerify`. No serde path; every word is written by the two clients.
  - `viola_state::liveness::without_endpoint(instance_dir: &Path) -> UnreachableCause` (`crates/viola-state/src/liveness.rs:107-115`): `symlink_metadata` reads a directory `InstanceDead`, anything else (absent, a regular file, a symlink) `UnwrappedName`; opens nothing, writes no line. The strict-modes check stays first at every caller. Callers: `src/cmd/client.rs` `Unreachable::without_endpoint` and `crates/viola-mcp/src/tools.rs` `frame`.
  - `src/cmd/client.rs`: `Unreachable` is now `Dead` · `Unwrapped` · `Lost` · `ServerUnverified`, `Dead(&'static str)` gone; `cause()` (`:114-122`), `detail()` (`:124-127`), `during()` (`call` for `Lost` only), `hint_cause()` (`not-running` · `unwrapped-name` · `endpoint-lost` · `server-verify`); `answer_of`: `Connect` is `Dead`, `Closed | Io` is `Lost`. `pub(crate) fn unreachable_document(cause)` (`:222-226`) is the one writer of the exit-21 document; `refuse_untrusted` writes `StrictModes` through it, `src/cmd/plugin.rs` too, and `refused_document` stays exit 1's. `src/cmd/list.rs`'s `refused_document` (a `Value` for `board_document`) builds `strict-modes-failed`, the board's only refusal.
  - The six no-endpoint arms read `Unreachable::without_endpoint(&instance_dir)`: `src/cmd/send.rs:222`, `src/cmd/wait.rs:55-57`, `src/cmd/last.rs:42-44`, `src/cmd/answer.rs:123`, `src/cmd/pause.rs:68`, `src/cmd/link.rs:103-105`; `release` and `unlink` reach them through `wheel_call` and `link_call`.
  - **A process-exit `during`:** `instance-dead` no longer carries `during:"call"`. A wrapper lost with the call open is `endpoint-lost` · `during:"call"`; `instance-dead`, `unwrapped-name` and `server-verify-failed` are `during:"connect"`.
  - **A stale holder's code:** `collision_check` (`src/cmd/run.rs`) prints and logs `stale-heartbeat` for a `stale` holder, and `already-live` for a live holder and a fresh replayed beat. The `run.collision_check` span's `outcome` stays `already-live` for a live or a stale holder: its closed set says a holder stands and did not move. A frame verb against a stale instance still reads `instance-dead` (`live_endpoint` gives none for a stale wrapper): stale heartbeat is exit 1's cause alone.
  - **`run`'s start under `--json`:** `Launch` gains `json: bool` (`run` passes false, `revive` passes `args.json`); `refused_with(json, detail, human)` (`src/cmd/run.rs:422-431`) prints `refused_document` in place of the pair for `batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, and for a program not found `internal-error` (no pair today, none added). All return before `spawn_child`. The human pairs are byte for byte as they stood.
  - **`viola revive --json`** (`src/cmd/revive.rs:119-134`, `started`): prints one exit-1 document for every refusal of the start it shares with `run`. An anyhow error from `trusted`, `checked_exe`, `open_wrapper_log`, `preflight` or `start` under `--json` prints `refused_document("internal-error")` and returns the same error, so the catch site still logs its chain and its line. A `pump_child` error prints no document: the child has held the terminal, and viola writes nothing past it. Its codes are now `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`. The sentence "a refusal `run`'s start makes after a passed preflight prints no document" is no longer true.
  - **The `cli` catch site under `--json`:** `src/main.rs` `json_asked(args)` (`:154-161`), `--json` among the arguments before a `--`, read from argv as `role_of` is; `src/cmd/mod.rs` `internal_error_document()` (`:99-102`). Where `prints_internal_error` is true, `json_asked` prints `{"v":1,"error":"internal-error","detail":null}` on stdout in place of `error: internal error`, stderr then empty. Without `--json` the line is byte for byte as it stood; `run`, `revive`'s start, `mcp` and `hook` stay silent there.
  - **`verify`'s codes:** `src/cmd/verify.rs` `Ended::Refused(&'static str)`, its code on the `process-exit` `verify` writes under a parsed `VIOLA_NAME`: `batch-script-child`, `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed`, `unclean-fixture`, beside the standing `interrupted`. `record` and `refuse_recording` return `Ended`. A refused home tree stays lineless; a failing row stays no refusal and no code; the fixture's `<file>` enters no code; `verify` takes no `--json`.
  - **MCP:** `crates/viola-mcp/src/result.rs` `unreachable(cause: UnreachableCause)`; `of_reply`: `Connect` is `InstanceDead`, `Closed | Io` is `EndpointLost`; `of_board`: the board's refusal is `StrictModes`. `tools.rs` `frame`: a refused `check_instance` is `StrictModes`, no `live_endpoint` is `without_endpoint`'s reading, a failed `server_verified` is `ServerVerify`. No frame and no line on any of them, `isError` the record. JSON-RPC errors unmoved; `-32603` keeps its fixed body.
  - Remaining callers kept: `refused_document` keeps its exit-1 callers; `human::refuse(` stays 18 sites (`command grep -rcE 'human::refuse\(' src`, the plan's rule).
- **Crates / modules:** no crate added or removed. `viola-core` stays pure; `viola-state` gains a `viola_core::UnreachableCause` use it already could reach. Nothing in `viola-channel`, `fixtures`, `plugin`, `scripts` or `.github`.
- **Dependencies:** none added or bumped.
- **Schema / config:** `schemas/diag-line.v1.json:78`, the `process-exit` `detail` enum gains six: `stale-heartbeat`, `unwrapped-name`, `endpoint-lost`, `program-not-found`, `version-unreadable`, `unclean-fixture`. `channel-response`'s three words and `hook-decision`'s seven did not move. No config key.
- **Spec-master edits:** none; every master's text is this wrap's.
- **Retired:** the exit-21 document's `detail: null`; `already-live` as a stale holder's refusal; `instance-dead` · `during` `call`; the `revive --json` post-preflight gap (CARRY of chunk 2026-10-17-cli-machine-contract); the "not built" lines of the exit-cause matrix and of the unwrapped-name hint; `NOT_ERROR_BODIES`' "post-preflight start refusals with no document" row (`tests/contract_error_bodies.rs`).
- **Counts / qualifiers moved:**
  - exit-21's document, one form → one per cause · `doc-null`: 44 standing · 42 changed (each its cause: a strict-modes site `strict-modes-failed`, a not-running site its cause), 2 re-read with their `isError` body; `detail-null`: 22 standing · 20 changed · 2 no-change (`e2e-test-strategy.md:132`, `test-scope-summary.md:155`, another refusal's `detail null`); `no-own-code`: 1 standing · changed (`error-handling.md:12`, "adds no error word").
  - the five causes, two owed words landed · `causes-new`: 3 standing · 3 changed (`surface.md:83`, `test-plan.md:553`, `e2e-test-strategy.md:284`, each saying no code or not built).
  - the exit-cause table built · `cross-table`: 11 standing · 4 changed (`test-plan.md:610`, `e2e-test-strategy.md:274`, `:275`, `test-scope-summary.md:12`), 7 re-derived (rows of the form "not a `cross_exit_causes.rs` row" stay true where the cause is named outside the table, its comment at the file's top: server verification, stale heartbeat, the `.cmd` human pair, `verify`'s version, recording, pinned-copy, interrupted and stamps refusals, `viola mcp`'s two silent exits).
  - a stale holder's code · `stale-live`: 6 standing · 3 changed (`conventions.md:35`, `surface.md:83`, `log-coverage.md:26`), 3 no-change (`span-trace-coverage.md:45`, the span's `outcome`, unmoved; `test-plan.md:241`, `test-scope-summary.md:183`, the distinction now built).
  - `revive --json`'s post-preflight gap closed · `revive-gap`: 12 standing · 11 changed, 1 no-change (`surface.md:82`, another clause); `start-codes-rev`: 4 standing · 4 changed with it; `title`: 14 standing · 14 changed (each "owed to" this entry).
  - the catch site's `--json` form · `catch-site`: 24 standing · re-derived: the human line stands byte for byte; a site stating the output under `--json` too takes the document (`layout-templates.md:288`, `:329`, `obs-plan.md:523`, `error-capture-reporting.md:16`, `conventions.md:38`).
  - `verify`'s three new log codes · `verify-codes`: no master site; the refusal line is the basis, `control: owner-generic, verify-codes, unreach-cause never fired over the seven masters' pre-pass text`. New text wherever §6's catalog stands.
  - `instance-dead` · `during` · `dead-call`: 4 standing · 3 changed (`occupied-resources.md:122`, `span-trace-coverage.md:95`, `:112`), 1 re-derived (`unit-test-strategy.md:87`).
  - `process-exit` `detail` enum 13 → 19 members, by its rule the enum's members (`schemas/diag-line.v1.json:78`, counted from the diff above) · `enum-count`: 3 standing · 1 changed (`log-coverage.md:28`, "13 values"), 2 no-change (`obs-plan.md:451`, `log-coverage.md:37`, name the catalog without the count; the catalog's own list grows).
  - the `ci` profile's kill line 24 → 25 `binary(` operands (`.config/nextest.toml:24`, counted from the diff above) · `kill-count`: 16 standing · 0 changed: no master states the count; 7 are the ledger's or the screen's 24 rows, a true claim sharing the token, and 9 name the kill line without a number.
    - Correction (`inputs#I7`, the overseer, 2026-10-31): the row above is false. `.andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10` states the count spelled out, "the twenty-four verify-driven binaries", which this row's pattern could not read; the wrap moved it to twenty-five with the name list (`fanout-test-plan.md` 4).
  - the unwrapped hint built, the endpoint-lost hint new · `hints`: 4 standing · 0 changed in wording (`surface.md:64`, `:65`, `layout-templates.md:143`, `:331`); the new hint is new text, and `surface.md:65`'s pending status moves.
  - `tests/support/start.rs` takes the start setups · `frozen-guard`: 1 standing · no-change (`e2e-test-strategy.md:38`, a true mention).
  - Symbols new to the masters (`UnreachableCause`, `without_endpoint`, `unreachable_document`, `internal_error_document`, `json_asked`, `refused_with`, `Ended::Refused`) · `unreach-cause`: no master site, the same refusal line.
  - Lines over 2 000 chars, a disposition per (line, fact): `authentication-authorization.md:27` doc-null @c561 changed · detail-null @c1115, @c1596, @c1688 changed; `authentication-authorization.md:36` doc-null @c89, @c1480 changed · detail-null @c2116 changed; `layout-templates.md:329` doc-null @c3262 changed · catch-site @c3537, @c3579 re-derived; `a11y-scope-summary.md:8` revive-gap @c973, @c998 changed · title @c1037 changed · doc-null @c1856, @c2264 changed; `surface.md:144` catch-site @c7 re-derived · title @c1986 changed; `occupied-resources.md:148` doc-null @c1741 changed; `:149` doc-null @c1362 changed; `:122` dead-call @c926 changed; `crate-dependency-direction.md:6` kill-count @c1614 no-change; `bootstrap-phases-derive-for-route-setup-project.md:10` kill-count @c3971, @c4356, @c4759 no-change.
  - Sweep reach: 15 patterns · 13 moved facts, 13 with a pattern · owner pattern 14 standing · by line 5 lines · 8 offsets. `{P}` from the tool's `total` line, the moved facts the report's own count. A listing reaches its patterns' wording and no further.
  - Basis: `cascade.py sweep --pre-pass`, trail `cascade-2026-10-31-exit-cause-code-catalogue-pre-pass.json`, patterns `report-patterns.toml` in this run dir. `per class · new 0/0 · standing 169/38 · leaf 35/12 · curation 2/1 · base 0/0`. The site dispositions above are this report's reading of the printed rows; the detectors read the lines.
- **Dev-tool versions:** none. `cc`, the toolchain and CI's pinned `tool:` line read by pre-push's `tools` stage as they stood.
- **Harness / gate surface:** `.config/nextest.toml`'s `ci` kill line gains `binary(cross_exit_causes)`. `Cargo.toml` gains its `[[test]]`, `required-features = ["fake-agent"]`. No harness verb, status shape or CI step moved. The new table is a root `cross_` binary, which the E2E regex takes as it took `path1_`.
- **Cross-project / external claims:**
  - CI, final HEAD: `8a4c1a2ce7b9 verdict: green · checks 15/15 · wall 770 s · runs ci#41226917636` (`evidence/operator-pass.md`). Earlier: `115cdbb9` `ci#41225642322` red, `lint (macos-latest)` and `test (macos-latest)`, one compile error, `crates/viola-mcp/tests/handlers.rs:539` (fixed by `8a4c1a2c`). The verdict stands on HEAD `8a4c1a2c`; this wrap's own commit adds to it.
  - `I1 · ../additional/viola-overseer/phase-exit-cause-directive.md · copy no-repo · unchanged`, cited `inputs#I1`.
  - `I2 · message: the P4 dialog through the overseer, 2026-10-31 16:52Z · copy message · n/a`, cited `inputs#I2`.
  - `I3 · message: the overseer, 2026-10-31 17:09Z · copy message · n/a`, cited `inputs#I3`. Its fact names the P5 review card `.andromeda/runs/2026-10-31T15-42-40-phase/p5-card-1.md`; its `## Beyond the plan` rows, verbatim, with the word's authority "delegate the overseer, 2026-10-31, PROVISIONAL":
    - | Proposal | Owner |
    - | The socket-directory refusal's code, `v1-18`'s leg, is left out of this chunk | the existing entry "Unix endpoint and home hardening" |
  - `I4 · message: the operator, the /andromeda-implement invocation, 2026-10-31T17:12Z · copy message · n/a`, cited `inputs#I4`.
  - `I5 · ../additional/viola-overseer/wrap-exit-cause-directive.md · copy no-repo · unchanged`, snapped at this wrap, cited `inputs#I5`.
  - `inputs: 5 entries — unchanged 2 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 3 · uncited 3 · unparsed 0`, the three uncited (I3, I4, I5) cited here. No DRIFT.
- **Reverted / negative API facts:** a `Started`-time document from `main` for `revive`'s start was rejected (a `pump_child` error reaches the same catch site, after the child has held the terminal). `refused_document` keeps its signature. The span's `outcome` was not moved. `not-running` stays the hint cause's key, not renamed to the log's `instance-dead`.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - The route CARRY's sentence "`session-live` is not built" (chunk 2026-10-10-viola-revive) is false: `src/cmd/revive.rs` and `schemas/diag-line.v1.json` hold it (scope.md's premise correction). No master site: a route carry this chunk folds. `no master site — a working-route CARRY, spent at this wrap's flip`.
  - `v1-18`'s requirement lists a socket-directory refusal the code does not hold: `bind_endpoint`'s `create_private_dir` failure is an anyhow error at the catch site (scope.md's premise correction). `no master site — a matrix entry, narrowed by a dated note at phase P5`; owner: the route entry "Unix endpoint and home hardening", the Beyond-the-plan row (`inputs#I3`).
  - A master's mention of `viola list --json` over a regular file at `diagnostics` reaching the catch site: not a master claim; the plan's step 14 said it and P5 measured exit 0 with no `VIOLA_NAME` (`.andromeda/runs/2026-10-31T15-42-40-phase/p5-controls.md`). `no master site — corrected in the plan before implement`.
- **Expected amendments (from plan):**
  - architecture §Standard Contracts CLI `--json` output · §Conventions CLI exit codes and MCP · §Occupied Resources · §Established Decisions [MCP], [Session Liveness]: carried, **Symbols / APIs** and **Retired**. Search: `doc-null`, `detail-null`, `revive-gap`, `title`, `stale-live`, `dead-call` · hits per the rows above (`architecture.md:226`, `:228`; `conventions.md:35`, `:36`, `:46`, `:64`, `:70`, `:76`, `:89`; `established-decisions.md:124`, `:305`; `occupied-resources.md:42`, `:122`, `:148`, `:149`, `:168`; `standard-contracts.md:111`, `:117`). The founder's own answer `inputs#I2` Q1; the wrap halts on it.
  - security-plan §Error Handling · §Authentication & Authorization · §Input Validation · §Threat Model Summary: carried, **Symbols / APIs**. Search: `doc-null`, `detail-null`, `no-own-code`, `revive-gap`, `title` (`security-plan.md:413`; `authentication-authorization.md:27`, `:36`, `:73`; `error-handling.md:12`, `:13`, `:22`; `input-validation.md:37`, `:38`; `threat-model-summary.md:99`). The unwrapped reading: `inputs#I2` Q2, `delegate the overseer`, PROVISIONAL.
  - design-system §Surface: cli patterns, exit-code rows 1 and 21, the two pending lines, body and `surface.md`: carried, **Symbols / APIs** and **Counts**. Search: `doc-null`, `causes-new`, `stale-live`, `revive-gap`, `hints`, `catch-site`, `title` (`design-system.md:334`, `:338`; `surface.md:64`, `:65`, `:82`, `:83`, `:84`, `:135`, `:144`, `:152`, `:168`). The `endpoint-lost` hint: `inputs#I2` Q3, `delegate the overseer`, PROVISIONAL.
  - layout-templates §Component — Primary content block 2 · §Output structure — `viola revive`, `viola list --json`, `viola wait` / `viola last`, `viola plugin install` · §Primary screens: carried. Search: `doc-null`, `revive-gap`, `catch-site`, `hints`, `title` (`layout-templates.md:102`, `:143`, `:146`, `:262`, `:288`, `:329`, `:331`).
  - test-plan §1 · §2 (the naming line, a root `cross_` binary) · §5 On-disk, MCP · §6 Exit-cause matrix, Cross-surface parity, Security control negatives · §9: carried. Search: `cross-table`, `doc-null`, `detail-null`, `causes-new`, `revive-gap`, `dead-call`, `kill-count`, `title`. The `ci` kill line's count: no master states it (`kill-count`, 0 changed), so it is carried and changes nothing.
  - obs-plan §1 · §4 · §5 · §6 catalog · §7 · §10: carried. Search: `enum-count`, `stale-live`, `dead-call`, `catch-site`, `doc-null`, `detail-null` (`obs-plan.md:361`, `:451`, `:519`, `:523`, `:527`; `log-coverage.md:26`, `:28`, `:39`).
  - a11y-plan §1 · §8 Error recovery and the literal-pinned list: carried. Search: `doc-null`, `revive-gap`, `title`, `catch-site` (`a11y-plan.md:31`; `a11y-scope-summary.md:8`, `:25`, `:80`; `cognitive-accessibility.md:19`, `:25`, `:31`, `:49`, `:92`).
  - `matrix.py` note on `v1-18` at P5: already written at phase P5 (`verification-matrix.json#v1-18` notes, the 2026-10-31 paragraph); nothing owed at this wrap. A `matrix` ref extension on `v1-39` is deviation 6.
- **Coverage of new surfaces:**
  - exit-21 `detail` causes on `--json` → validation closed enum✓ · instrumentation process-exit✓ · PII no path, pid or upstream text✓ · tests integ/e2e (`tests/cross_exit_causes.rs`, `tests/contract_error_bodies.rs`) · a11y one document, no hint, no ESC✓ · tokens n/a
  - MCP `detail` causes → validation closed enum✓ · instrumentation n/a (no line by `inputs#I2` Q4, `isError` the record) · PII✓ · tests integ/e2e (`handlers`, `mcp_tools`) · a11y n/a · tokens n/a
  - `unwrapped-name` and `endpoint-lost` hints → validation `<name>` the parsed `ViolaName`✓ · instrumentation process-exit✓ · PII✓ · tests unit + e2e (`send_hint_is_the_design_string`, the table) · a11y plain ASCII, last on stderr✓ · tokens n/a
  - `revive --json` start documents and the catch site's document → validation closed codes✓ · instrumentation the same lines as before✓ · PII✓ · tests e2e (the table, `cli_wait_last`) · a11y one document, stderr empty✓ · tokens n/a
  - `verify`'s `process-exit` codes → validation closed✓ · instrumentation process-exit✓ · PII the fixture's `<file>` enters no code✓ · tests integ (`tests/cli_verify.rs`, the table) · a11y n/a · tokens n/a

## Deviations from intent
1. **Step 13 moved more than it named.** `remove_dir_once_free` (the table's `cwd-missing` row), `FIXTURE_SESSION` (its `session-live` row) and `chaos_wait_vanish`'s `kill`, now `support::start::kill_wrapper` (its lost row), also went to `tests/support/start.rs`: tests S3 forbids a root file its own copy of what two share. All three files are listed.
2. **`Frozen` now exists on Unix too** (`tests/cli_instance_state.rs:149-181`), a `SIGSTOP` guard, and `run_refuses_a_stale_name` stands on it. Step 15 asked for the stale document "through its `Frozen` guard", which was Windows-only, so the case would have read nothing on Linux or macOS.
3. **`tool_inputs_that_pass_reach_the_liveness_check` was vacuous before this chunk.** Its bare temp dir reads 0755 under the umask, so the read rule refused it and it never reached the check it names; `detail: null` hid that. Its `empty_home` is now viola-made and it reads `unwrapped-name`. No plan step named it.
4. **`handlers`' unserved row binds nothing.** A Windows `Serving` has no `Drop` (its pipe lives in the accept thread until the process exits), so the row records an endpoint no one ever bound, through `record_live` (`crates/viola-mcp/tests/handlers.rs:64-86`).
5. **`mcp_tools`' row after `cleanup` reads its cause from the directory.** A kept home (`AGENT_RUN_KEEP_HOMES`, as on CI) holds the stopped instance and reads `instance-dead`; a removed one reads `unwrapped-name` (`crates/viola-e2e/tests/mcp_tools.rs:916-922`). It still compares the body with the CLI's.
6. **`v1-39`'s ref was extended**, where the P5 preview said it stands: the existing text kept, the table's four start rows and the stale document added, because no existing ref named a witness of the gap the cap was un-claimed on. Status and acceptance do not move.
7. **Added beyond the table's list:** `every_code_is_a_code_word` (`tests/cross_exit_causes.rs:131-137`); `verify_a_tampered_pinned_copy_refuses_with_its_code` and `verify_window_a_widened_stamps_file_refuses_with_its_code` are new in `tests/cli_verify.rs`, the second under the `verify_window_` prefix because it runs two whole verifies.
8. **The macOS check-only clippy recipe (test-plan §9) was not run before the first push.** The plan names it a recipe, not an entry, and says the operator reads `lint (macos-latest)` from CI. It would have read the compile error of `115cdbb9` locally; the first CI read went red on it. After the fix it reads exit 0, 2026-10-31T17:42:00Z. No recorded gate was skipped: the block does not list the recipe.

Scope record: `scope record: none — gate.py scope clean, 0 recorded`. `scope: clean — changed 41 · listed 41 · recorded 0 (companion 0 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 672`.

## Decisions & corrections
- The directive to this wrap, `inputs#I5`, the overseer, founder-delegated: forced sample; classifiers high, two draws; one re-spawn; check 7 escalates as written; the halt on Q1 and Q4; the socket-directory row; the WATCH; the delegate's words carried PROVISIONAL; no consolidation at Epoch 6's end, so a consolidation the letter would run stops the wrap before it.
- The founder's own words: `inputs#I2` Q1 (the cause in `detail`, CLI and MCP) and Q4 (claim `v1-18` with named carve-outs), live, 2026-10-31 before 16:52Z, through the overseer's dialog, given after the widening was shown to him.
- The delegate's words, PROVISIONAL: `inputs#I2` Q2 (unwrapped reads by the instance directory), Q3 (the hint "went away during the call"), and the P5 approval `inputs#I3` with its Beyond-the-plan row.
- The operator's directive at implement, `inputs#I4`: no live session, a kill deadline on every spawn, each long run journalled, the operator pass with its CI read, its first push carrying the two spine-door commits.
- Sweep hazard: a new test helper's own `cfg` gate is one more thing a case can trip on an OS the dev host cannot run. `handlers.rs`' `text` helper was `cfg(not(target_os = "macos"))` and a new every-OS case used it. The check-only recipes for both other OSes read it locally: Windows ran as an entry, macOS was a recipe and did not run, and CI's first read went red. Evidence: `evidence/operator-pass.md`, `outcome-…md` Decisions.
- Sweep hazard: an oracle moved by a shared body can hide a test that never reached its subject. `tool_inputs_that_pass_reach_the_liveness_check` read `instance-unreachable` from its strict-modes refusal, the same body as a dead instance until `detail` told the causes apart (deviation 3). A code per cause exposed it.
- Correction: implement's own added `strict-modes-failed` line in `only_the_wrapper_writes_the_snapshot_and_only_verify_the_stamps` was wrong, its wrapper being stopped, and read red on the first pass (`outcome-…md`).
- Recurrence of a standing learning (CLAUDE.md, timestamps from `date -u`): implement wrote two journal stamps by estimate, one 2 minutes ahead; the stamp-ahead hook refused it and both were corrected from the clock. Standing deferred item, `recurrence-despite-learning`.
- Guard refusal: the Bash guard refused a `cat` heredoc with a file target for a scratch script at implement; re-issued once through the Write tool. At phase research a pipe of `rec.py` into `cut`, re-issued as two calls (`p5-card-1.md`).
- Hygiene: entry 21's first firing refused two phase files holding the temp path of the `target/e2e-home` link (`p5-controls.md:42`, `p5-resume.md:22`); each respelled `$TMPDIR/…`, the verb re-fired (`evidence/operator-pass.md`).
- A P5 slip the card names: a stray `rm -f /dev/null` on a read-only command, denied by the permission classifier, nothing removed.
- A difference left untraced at phase: the classify batch's line read yes 497 and one-draw 46, the census 496 and 45 (`p5-card-1.md`, "I did not trace that difference of one").
- The operator pass fixed its red by one commit in a listed file, with entries 2, 3, 4, 20 and 21 re-fired before the push (`evidence/operator-pass.md`).

## Outcome
Acceptance criteria, each re-asserted against the diff:
- `verification-matrix.json#v1-18`: met as concretized, with the named carve-outs (`inputs#I2` Q4). What they leave unmet, by name: the socket-directory cause and macOS's server check, with "Unix endpoint and home hardening". The diff holds every listed word in `schemas/diag-line.v1.json:78`, `UnreachableCause` and `Ended::Refused`.
- `verification-matrix.json#v1-39`: met; `revive --json` prints one document for `batch-script-child` (`a_revive_whose_claude_is_a_script_is_the_batch_script_child_document`, Windows CI PASS 0.347 s), `pinned-hash-mismatch`, `squatted-name`, and a `claude` not on `PATH` as `internal-error`.
- Exit 21's five causes on the eight frame verbs, `list` and `plugin install`: met; no exit moved in the diff.
- MCP's body less `v`, the text unchanged, each cause its word: met (`handlers_frame_tools_name_the_cause_of_an_unreachable_instance`, `mcp_tools_a_wrapper_killed_under_a_parked_wait_reads_endpoint_lost`), `server-verify-failed` off macOS.
- No refused check gains a line or moves: met; every strict-modes writer in the diff stays before any log, and the named witnesses stand green.
- Every new word passes `is_code_word`, is a row of `tests/contract_error_bodies.rs`: met (`every_code_is_a_code_word`; `product_error_writers_are_each_named` PASS on all three OSes).
- `hook` unmoved: met; the diff holds no `src/cmd/hook.rs`, and `hook_fail_open` is in the block.
- The two hints, last on stderr, plain ASCII: met (`src/human/hints.rs:50-55`).
- Each logged cause one `process-exit` with its own `detail`, `schemas/diag-line.v1.json` admitting them, G4 no failure: met (entry `bash scripts/agent-run.sh schema-check`).
- The table green, with `no_two_codes_share_a_hint`, its control, no hint naming `release`: met.
- Unit and pre-push gates green, coverage floors: met (`{"cmd":"gate","ok":true,"breaches":[]}`).
- a11y: one document, stderr empty, no ESC; each human form ending with its `hint:`, a fault with none; a passed `revive` writes nothing while its child runs: met.

Gates, the block by `run`, the final state of implement and the operator pass (this wrap's light gate re-runs the block at P7):
- `df -h /tmp`: recorded, report-only, exit 0; read at implement and 22G free at this session's start.
- `cargo fmt --all --check`: green, re-read on the fixed tree (exit 0).
- `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings`: green, re-read on the fixed tree.
- `CARGO_TARGET_DIR=target/wincheck cargo clippy … --target x86_64-pc-windows-msvc …`: green, re-read on the fixed tree. Its first firing was red on `drop(serving)` and two `cfg(unix)`-only constants; fixed.
- `rg … '#\[(tracing::)?instrument\b(?!\(\s*skip_all\b)' .`: green, exit 1, no output.
- `bash scripts/lint-probes.sh`: green.
- `bash scripts/agent-run.sh run --unit`: green.
- the 22-binary integration `cargo nextest run`: green.
- `agent-run.sh run --e2e --filter 'binary(mcp_tools) | …'`: green.
- `agent-run.sh run --e2e --filter 'binary(cross_exit_causes)'`: green, `39 tests run: 39 passed` on Linux.
- `cargo deny check`: green.
- `bash scripts/channel-tokio-free.sh`: green.
- `gate.py guard`: green, `guard: clean`, `allowed 41`.
- `agent-run.sh cleanup` · `boot --unstamped --instance builder` · `status` · `g2-zero-panics.sh` · `schema-check` · `cleanup`: green.
- `bash scripts/agent-run.sh pre-push`: green, `3403 tests run: 3403 passed (5 slow), 0 skipped`, `{"cmd":"pre-push","ok":true,"stage":"linux-tests"}`, host `QUIET for viola`. The first firing was red on `tool_inputs_that_pass_reach_the_liveness_check` (deviation 3), fixed.
- `gate.py hygiene` (leg operator): green, `hygiene: clean`, its first firing refused 2 files, respelled.
- `git diff --quiet && git diff --cached --quiet && git push origin HEAD` (leg operator): green, twice, `16ef6b48→115cdbb9`, `115cdbb9→8a4c1a2c`.
- `ci.py conclusion --sha HEAD --wait 2400` (leg operator): green on the final HEAD, `8a4c1a2ce7b9 verdict: green · checks 15/15 · wall 770 s · runs ci#41226917636`. An earlier read on `115cdbb9` was red, the macOS compile error above.
- Smoke: boot path driven by hand, `boot --session p-causes-hand --unstamped --instance builder`, exit 0; `status` ready; `viola wait stranger --json` exit 21 `{"v":1,"error":"instance-unreachable","detail":"unwrapped-name"}`, stderr empty; `cleanup` `processes_gone:true`, `endpoint_gone:true`, `killed:[]`.

Watches:
- `verify_stopped_during_the_resume_run_removes_its_dir_and_exits_1` · readings on this chunk's CI runs: `ci#41226917636` (final HEAD) Ubuntu `4.655 s` / `4.596 s`, macOS `5.276 s` / `5.500 s`, Windows `cfg(unix)` not run; `ci#41225642322` (`115cdbb9`, the run red for another cause) Ubuntu `6.297 s` / `6.273 s`, macOS never built its tests. No red. Slowest reading 6.297 s against the 7 s `WITHIN`, on a CI runner, so 0.703 s of margin. Pre-push on the dev host, four firings: `4.436 s` / `4.432 s`, `4.437 s` / `4.468 s`, `4.491 s` / `4.484 s`, `4.478 s` / `4.465 s`. scope.md names `ci#41198134788` (`16ef6b48`, green) as the tally's first reading. No step edited the test or its bound. The tally and the retirement are P5's.

Outcome basis: the operator pass ran. Commits from the parent `404f7496`: `115cdbb9` (pre-CI), `8a4c1a2c` (fix). The verdicts above rest on that final state and on `chunks/2026-10-31-exit-cause-code-catalogue/evidence/operator-pass.md`. This window holds no implement run: `.andromeda/runs/2026-10-31T17-12-41-implement/outcome-2026-10-31-exit-cause-code-catalogue.md`, the only `*-implement` dir of this chunk, is read as its P4 report; nothing supersedes it. Implement's other evidence: `evidence/remove-the-guard.md`, `evidence/profile-census.md` (`runs 40 · passed 40 · profiles 160 · refused 0`). No operator directive between implement and this report changed a fact above; `inputs#I5` directs this wrap.

Process hygiene, implement's census, from its outcome: `p-causes-smoke` and `p-causes-hand` sessions `terminated` (each `cleanup` `processes_gone:true`, `killed:[]`); test wrappers and fake agents of each block, control and census `terminated`; six `viola run` / `viola wait` under `additional/viola-lab/prototype`, `~/.viola/bin/0.1.0-231c0225dac912d9/viola mcp` and `~/.local/viola-product/bin/viola wait viola-builder …` `left running: another tree's build — whoever started it stops it`. Re-measured here: `gate.py census --args projects/viola/target`, `census: none running`, 2026-10-31T18:14:46Z.
## New text, by line
Generated by `cites.py added` (cites v1.8); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 404f7496 (the parent of the oldest pre-CI commit 115cdbb9) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .config/nextest.toml — added 1 line(s) in 1 range(s)
added: 24
### Cargo.toml — added 5 line(s) in 1 range(s)
added: 41-45
### crates/viola-core/src/lib.rs — added 37 line(s) in 2 range(s)
added: 181-203 · 695-708
- 181-190 @184 «pub enum UnreachableCause {»
- 192-202 «impl UnreachableCause {»
  - 193-201 «pub const fn as_str(self) -> &'static str {»
  - 695-707 @696 «fn unreachable_cause_is_each_its_own_code_word() {»
### crates/viola-e2e/tests/mcp_tools.rs — added 64 line(s) in 4 range(s)
added: 716 · 916-922 · 925 · 931-985
  - 916-922 @918 «let cause = if home.join("instances").join("builder").is_dir() {»
- 931-939 @932 «fn kill_by_pid(pid: u64) {»
- 941-984 @945 «fn mcp_tools_a_wrapper_killed_under_a_parked_wait_reads_endpoint_lost() {»
  - 948-955 «let s = harness_session(»
  - 957-959 «let snapshot: Value = serde_json::from_slice(»
  - 964-974 «let killer = std::thread::spawn(move || {»
  - 978-981 «assert_eq!(»
### crates/viola-mcp/src/result.rs — added 44 line(s) in 9 range(s)
added: 10 · 29-31 · 53-55 · 92 · 207-219 · 224 · 260-276 · 293-296 · 299
### crates/viola-mcp/src/tools.rs — added 13 line(s) in 8 range(s)
added: 28-31 · 340 · 343 · 350 · 463-465 · 467 · 471 · 619
- 29-31 «use viola_state::liveness::{»
### crates/viola-mcp/tests/handlers.rs — added 70 line(s) in 6 range(s)
added: 64-87 · 107 · 428 · 469-470 · 498 · 512-552
- 64-86 @66 «fn record_live(instance: &Path, endpoint: String, pid: u32) {»
  - 67-83 «let snapshot = InstanceSnapshot {»
- 513-552 @518 «fn handlers_frame_tools_name_the_cause_of_an_unreachable_instance() {»
  - 523-528 «let target_args = |target: &str, name: &str| match name {»
  - 529-540 «let assert_cause = |tools: &Tools, target: &str, cause: &str| {»
  - 545-549 «record_live(»
### crates/viola-state/src/liveness.rs — added 22 line(s) in 3 range(s)
added: 10 · 107-116 · 242-252
- 107-115 @110 «pub fn without_endpoint(instance_dir: &Path) -> UnreachableCause {»
  - 111-114 «match std::fs::symlink_metadata(instance_dir) {»
### schemas/diag-line.v1.json — added 1 line(s) in 1 range(s)
added: 78
### src/cmd/answer.rs — added 1 line(s) in 1 range(s)
added: 123
### src/cmd/client.rs — added 65 line(s) in 19 range(s)
added: 14-15 · 35 · 94-96 · 99-101 · 106-110 · 114-130 · 133-134 · 141-143 · 189 · 192 · 222-227 · 240-241 · 244 · 446
       449 · 454 · 482-483 · 485-487 · 535-544
  - 114-122 @115 «fn cause(self) -> UnreachableCause {»
  - 124-127 @125 «fn detail(self) -> &'static str {»
- 222-226 @224 «pub(crate) fn unreachable_document(cause: UnreachableCause) {»
  - 535-543 @536 «fn without_endpoint_maps_the_directory_reading() {»
### src/cmd/last.rs — added 3 line(s) in 1 range(s)
added: 42-44
### src/cmd/link.rs — added 3 line(s) in 1 range(s)
added: 103-105
### src/cmd/list.rs — added 3 line(s) in 2 range(s)
added: 17 · 93-94
### src/cmd/mod.rs — added 5 line(s) in 1 range(s)
added: 99-103
- 99-102 @100 «pub(crate) fn internal_error_document() {»
### src/cmd/pause.rs — added 1 line(s) in 1 range(s)
added: 68
### src/cmd/plugin.rs — added 2 line(s) in 2 range(s)
added: 14 · 55
### src/cmd/revive.rs — added 23 line(s) in 6 range(s)
added: 119-134 · 139 · 145 · 149 · 153-155 · 237
  - 120-126 @122 «let started = started(home, args).inspect_err(|_| {»
  - 127-130 «match started {»
### src/cmd/run.rs — added 35 line(s) in 11 range(s)
added: 163-164 · 184 · 270-272 · 274 · 282 · 286-290 · 303-305 · 422-432 · 435-436 · 459-463 · 1673
- 422-431 @424 «fn refused_with(json: bool, detail: &'static str, human: impl FnOnce()) -> Started {»
  - 425-429 «if json {»
### src/cmd/send.rs — added 1 line(s) in 1 range(s)
added: 222
### src/cmd/verify.rs — added 14 line(s) in 13 range(s)
added: 81 · 102-103 · 106 · 164 · 171 · 178 · 206 · 296 · 309 · 518 · 535 · 632 · 637
### src/cmd/wait.rs — added 3 line(s) in 1 range(s)
added: 55-57
### src/human/hints.rs — added 18 line(s) in 4 range(s)
added: 24-25 · 50-55 · 90-97 · 130-131
### src/main.rs — added 24 line(s) in 3 range(s)
added: 73-77 · 154-162 · 441-450
- 154-161 @156 «fn json_asked(args: impl IntoIterator<Item = OsString>) -> bool {»
  - 157-160 «args.into_iter()»
  - 441-449 @447 «fn json_asked_reads_the_flag_before_a_separator(#[case] args: &[&str], #[case] asked: bool) {»
### tests/chaos_wait_vanish.rs — added 6 line(s) in 5 range(s)
added: 3-4 · 14 · 59 · 75 · 84
### tests/cli_controls_not_disableable.rs — added 1 line(s) in 1 range(s)
added: 568
### tests/cli_error_bodies.rs — added 19 line(s) in 11 range(s)
added: 24-27 · 129 · 135 · 141 · 148 · 154 · 160 · 166 · 172 · 190-195 · 200
- 24-25 «const UNWRAPPED_NAME: &str =»
- 26-27 «const STRICT_MODES: &str =»
### tests/cli_instance_state.rs — added 89 line(s) in 7 range(s)
added: 21 · 145-223 · 228 · 231-235 · 244 · 255 · 914
- 149-181 @150 «impl Frozen {»
  - 151-172 «fn attach(pid: u32) -> Self {»
  - 174-180 «fn signal(&self, sig: &str) {»
- 183-188 @184 «impl Drop for Frozen {»
  - 185-187 «fn drop(&mut self) {»
- 190-222 @193 «fn revive_json_of_a_stale_holder_is_the_stale_heartbeat_document(booted_wrapper: Wrapper) {»
  - 197-200 «let pid = snapshot_data(&dir).expect("snapshot")["pid"]»
  - 202-207 «fs::File::options()»
  - 209-211 «let child = revive_command(home, home.scratch(), &["builder", "--json"])»
  - 215-218 «assert_eq!(»
  - 231-234 «let pid = snapshot_data(&dir).expect("snapshot")["pid"]»
### tests/cli_links.rs — added 6 line(s) in 3 range(s)
added: 286-289 · 451 · 558
- 287-289 @288 «const REFUSED: &str =»
### tests/cli_list.rs — added 13 line(s) in 2 range(s)
added: 30-31 · 383-393
### tests/cli_machine_contract.rs — added 11 line(s) in 8 range(s)
added: 30-33 · 279 · 287 · 295 · 303 · 311 · 319 · 395
- 31-33 @32 «const REFUSED: &str =»
### tests/cli_plugin_install.rs — added 2 line(s) in 1 range(s)
added: 22-23
### tests/cli_revive.rs — added 10 line(s) in 5 range(s)
added: 6-8 · 15 · 24 · 27-30 · 32
- 27-30 «use support::start::{»
### tests/cli_send.rs — added 4 line(s) in 4 range(s)
added: 1060 · 1068 · 1075 · 1085
### tests/cli_verify.rs — added 85 line(s) in 6 range(s)
added: 316-317 · 326-402 · 440-441 · 448 · 902-903 · 905
- 329-346 @331 «fn assert_refused_line(home: &Path, detail: &str) {»
  - 335-337 «for line in &lines {»
  - 338-341 «let exits: Vec<&Value> = lines»
- 348-377 @351 «fn verify_a_tampered_pinned_copy_refuses_with_its_code(#[from(home)] home: TestHome) {»
  - 355-359 «let key = fs::read_dir(home.path().join("bin"))»
  - 360-362 «let pinned = key»
  - 371-375 «assert_eq!(»
  - 392-399 «assert_eq!(»
### tests/cli_wait_last.rs — added 25 line(s) in 5 range(s)
added: 409 · 418 · 426 · 436 · 463-483
- 463-482 @465 «fn cli_verbs_print_the_internal_error_document_under_json() {»
  - 469-481 «for (args, stdin) in [»
### tests/contract_error_bodies.rs — added 73 line(s) in 10 range(s)
added: 39 · 42-50 · 55 · 58 · 63 · 66 · 98 · 101-156 · 245 · 248
  - 101-108 «Body {»
  - 109-116 «Body {»
  - 117-124 «Body {»
  - 125-132 «Body {»
  - 133-140 «Body {»
  - 141-148 «Body {»
  - 149-156 «Body {»
### tests/cross_exit_causes.rs — new file · 567 line(s)
- 29-32 «use support::home::{»
- 34-37 «use support::start::{»
- 40-41 «const INSTANCE_STATE: &str =»
- 42-43 «const HOME_STATE: &str =»
- 45-46 «const UNWRAPPED_NAME: &str =»
- 47-48 «const ENDPOINT_LOST: &str =»
- 49-50 «const SERVER_VERIFY: &str =»
- 59-60 «const CWD_MISSING: &str = "resume it by hand from a directory you choose: viola run builder -- \»
- 69-92 «const CAUSES: &[Cause] = &[»
- 94-109 @95 «fn shared_hints(table: &[Cause]) -> Vec<&'static str> {»
  - 97-101 «for (code, _, hint) in table {»
  - 102-108 «table»
- 111-114 @112 «fn no_two_codes_share_a_hint() {»
- 116-122 @117 «fn shared_hints_reads_a_planted_duplicate() {»
- 124-129 @125 «fn no_hint_names_release() {»
  - 126-128 «for (code, _, hint) in CAUSES {»
- 131-137 @132 «fn every_code_is_a_code_word() {»
  - 133-136 «for (code, exit, _) in CAUSES {»
- 139-142 @140 «fn unreachable_document(cause: &str) -> String {»
- 144-147 @145 «fn refused_document(code: &str) -> String {»
- 149-154 @150 «fn assert_document(ran: &Ran, exit: i32, document: &str) {»
- 156-167 @157 «fn assert_hint(ran: &Ran, exit: i32, hint: &str) {»
  - 160-165 «assert_eq!(»
- 169-176 @170 «fn assert_unreachable(ran: &Ran, json: bool, cause: &str, hint: &str) {»
  - 171-175 «if json {»
- 178-185 @179 «fn assert_refused(ran: &Ran, json: bool, code: &str, hint: &str) {»
  - 180-184 «if json {»
- 187-205 @189 «fn assert_exit_line(home: &Path, file: &str, exit: u8, detail: &str, level: &str) -> Value {»
  - 193-195 «for line in &lines {»
  - 196-199 «let exits: Vec<&Value> = lines»
- 207-214 @208 «fn in_mode<'a>(words: &[&'a str], json: bool) -> Vec<&'a str> {»
  - 210-212 «if json {»
- 216-221 @217 «fn made_instance(home: &TestHome) -> PathBuf {»
- 223-225 «fn unstamped() -> StampedHome {»
- 227-230 @228 «fn run_start(home: &Path) -> Ran {»
- 232-239 @233 «fn start_in_mode(home: &TestHome, json: bool) -> Ran {»
  - 234-238 «if json {»
- 241-258 @244 «fn strict_modes_on_a_frame_verb_is_exit_21_and_opens_no_log(#[case] json: bool) {»
  - 248-253 «let ran = viola(»
- 260-272 @264 «fn strict_modes_on_the_board_is_exit_21_and_opens_no_log(#[case] json: bool) {»
- 274-292 @278 «fn strict_modes_on_plugin_install_is_exit_21_and_creates_nothing(#[case] json: bool) {»
  - 282-287 «let ran = viola(»
- 294-304 @297 «fn a_stopped_instance_is_instance_dead(#[case] json: bool) {»
- 306-326 @309 «fn a_name_never_started_is_unwrapped_name(#[case] json: bool) {»
  - 311-316 «let ran = viola(»
  - 318-324 «let exit = assert_exit_line(»
- 328-361 @332 «fn a_wrapper_killed_under_a_parked_wait_is_endpoint_lost(#[case] json: bool) {»
  - 337-340 «let pid = snapshot_data(&wrapper.instance_dir()).expect("snapshot")["pid"]»
  - 345-353 «while !support::start::role_lines(&home)»
- 363-376 @366 «fn a_live_holder_is_already_live(#[case] json: bool) {»
- 378-391 @381 «fn a_held_endpoint_is_squatted_name(#[case] json: bool) {»
- 393-415 @397 «fn a_tampered_pinned_copy_is_pinned_hash_mismatch(#[case] json: bool) {»
  - 401-405 «let pinned = PathBuf::from(»
- 417-437 @423 «fn strict_modes_on_a_start_is_exit_1_and_opens_no_log(»
- 439-454 @445 «fn a_revive_with_no_such_session_is_no_session(»
- 456-468 @459 «fn a_session_open_elsewhere_is_session_live(#[case] json: bool) {»
- 470-480 @473 «fn a_recorded_directory_gone_is_cwd_missing(#[case] json: bool) {»
- 482-505 @487 «fn a_revive_with_no_claude_on_its_path_is_internal_error(#[case] json: bool) {»
  - 491-494 «let child = revive_command(home, home.scratch(), &in_mode(&["builder"], json))»
  - 498-502 «if json {»
- 507-529 @512 «fn a_frame_verb_that_cannot_open_its_log_is_internal_error(#[case] json: bool) {»
  - 516-521 «let ran = viola(»
  - 523-528 «if json {»
- 531-546 @534 «fn verify_with_no_claude_on_its_path_is_program_not_found() {»
  - 539-545 «assert_exit_line(»
- 548-567 @552 «fn a_revive_whose_claude_is_a_script_is_the_batch_script_child_document() {»
  - 559-562 «let child = revive_command(home, home.scratch(), &["builder", "--json"])»
### tests/security_negatives_server_identity.rs — added 2 line(s) in 1 range(s)
added: 25-26
- 25-26 «const UNREACHABLE: &str =»
### tests/security_negatives_strict_modes.rs — added 6 line(s) in 2 range(s)
added: 29-30 · 302-305
- 29-30 «const UNREACHABLE: &str =»
### tests/support/start.rs — added 117 line(s) in 7 range(s)
added: 3-4 · 6 · 9 · 14-15 · 17-19 · 21-24 · 71-174
- 17-19 «use super::home::{»
- 71-82 @73 «pub fn first_life(stamped: StampedHome, dir: &Path) -> Wrapper {»
  - 77-79 «wait_events(&wrapper.instance_dir(), "the session-start record", |l| {»
- 84-93 @85 «pub fn wait_session_start_hooks(receipt: &Path, count: usize) {»
  - 86-92 «fake::wait_for(receipt, "the SessionStart hook's receipt", |lines| {»
- 95-100 @96 «pub fn made_dir(home: &TestHome, name: &str) -> PathBuf {»
- 102-110 @104 «pub fn stopped_life(stamped: StampedHome) -> (StampedHome, PathBuf, usize) {»
- 112-122 @114 «pub fn remove_dir_once_free(dir: &Path) {»
  - 117-121 «while fs::remove_dir_all(dir).is_err() {»
- 124-131 @125 «pub fn plant_agents(home: &TestHome, mode: Option<&str>, rows: &str) {»
  - 127-129 «if let Some(mode) = mode {»
- 133-141 @134 «pub fn listed(id: &str) -> String {»
  - 135-139 «serde_json::json!([{»
- 143-147 @144 «pub fn revive(home: &TestHome, dir: &Path, args: &[&str]) -> Ran {»
- 149-173 @150 «pub fn kill_wrapper(pid: u32) {»
  - 151-164 @152 «{»
  - 165-172 @166 «{»
