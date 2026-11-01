# Task — obs-plan · 2026-11-01-unix-endpoint-and-home-hardening

## The session
Built the verified per-user socket-directory check on `viola run`, `revive`'s start, the eight CLI frame verbs, `viola hook` and MCP's `frame`, reading a new closed word `socket-dir-refused`; the check never changes a mode, so a refused directory stays as found.
Added the peer euid check on both sides: the wrapper drops a refused client in `accept_loop` before any frame, and each client seam reads a refused server as `server-verify-failed`, now on macOS as well.
Moved `viola list`'s home from two rules to the read rule on every OS, a delegate's PROVISIONAL answer.
Retired the dated gaps owed to the route entry Unix endpoint and home hardening; macOS's peer pid gap stays named, owned by no route entry.
Measured CI `ci#41329861618` green on all three OSes, hook perf under `SPINE_DEADLINE` with both checks in the timed path, a bare `tempfile::tempdir()` at 0755 under the umask, and `cleanup`'s old `endpoint_gone` reading false after a force-killed wrapper, now a connect refused or a path absent.
Recorded the weakest proof: each client seam's euid wiring was read only through this run's forced refusals, and no committed test repeats them.

## Reached files
- crates/viola-core/src/lib.rs
- crates/viola-state/src/strict.rs
- crates/viola-state/src/board.rs
- crates/viola-channel/src/lib.rs
- crates/viola-channel/src/client.rs
- crates/viola-channel/src/tokio_client.rs
- crates/viola-channel/src/server.rs
- crates/viola-mcp/src/tools.rs
- src/cmd/run.rs
- src/cmd/client.rs
- src/cmd/hook.rs
- src/human/hints.rs
- schemas/diag-line.v1.json
- crates/viola-e2e/src/harness/cleanup.rs
- tests/security_negatives_socket_dir.rs
- crates/viola-e2e/tests/harness_lifecycle.rs
- crates/viola-mcp/tests/handlers.rs
- tests/support/home.rs
- tests/cli_list.rs
- tests/cross_exit_causes.rs
- tests/contract_error_bodies.rs
- tests/cli_error_bodies.rs
- tests/cli_controls_not_disableable.rs
- tests/hook_fail_open.rs
- tests/security_negatives_server_identity.rs
- Cargo.toml
- .config/nextest.toml

## Touches
- Exit 21's closed causes move from five to six, the sixth `socket-dir-refused`: `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`, `socket-dir-refused`. A row counting five changes, a row listing the five extends, a row naming only the type does not change.
- `process-exit`'s `detail` enum moves from 19 to 20 members and `hook-decision`'s from 7 to 8, at `schemas/diag-line.v1.json:78` and `:53`; `.andromeda/obs-plan-detail/log-coverage.md:29` states the 19.
- `server-verify-failed`'s reach moves from Linux and Windows to Linux, Windows and macOS, where on macOS it means an euid mismatch alone; `.andromeda/obs-plan.md:361` and `:537` state the old reach. A refused euid takes this word at all three client seams, with no new word.
- Rows on macOS's server check: a row stating macOS compares no peer pid stays true, its recorded gap wording kept or read as the named carve-out; a row implying macOS has no server check at all, or reading a built check as owed, changes.
- Rows naming the dated owner Unix endpoint and home hardening: each owed clause is discharged and changes, except macOS's pid gap, which stays named with no route entry owning it.
- Rows stating the socket-directory check or the client euid check as owed or absent change, because both are built.
- The `human::refuse(` census at `.andromeda/obs-plan.md:311` moves from 18 to 19 sites, `run` from 7 to 8; `verify` 6, `list` 2, `revive` 1, `plugin` 1 and `client` 1 stay, and the chunk that census cites as its re-derivation moves.
- `.andromeda/obs-plan.md:311`'s list of start refusals, and its count of `run`'s refusals that `viola revive` adds to, meet a further refusal through `refuse_socket_dir`, which `revive`'s start shares.
- `revive --json`'s exit-1 codes move from ten to eleven: a row holding all ten or counting them extends; a row naming `batch-script-child` as a refusal's own word does not change.
- `viola list`'s home moves from two rules to the read rule on every OS, `.andromeda/obs-plan-detail/log-coverage.md:40` among the rows to read; `ScanRefused::Home`, the check order and no-open are unchanged, `instances/` stays on the write rule, and `read_budget_stamped`, `mcp`'s start and `plugin install` keep the write rule on the home.
- The Vector 7 trigger at `.andromeda/obs-plan.md:215` limits where a strict-modes failure writes a line; the socket-directory refusal shares the read verdict yet writes lines under its own word, so a row reading every such refusal as lineless needs that distinction, and `strict-modes-failed`'s own line sites do not move.
- Founder Direction 4's signal at `.andromeda/obs-plan.md:253`, a distinct `detail` code per exit 21 and exit 1 cause with no path or pid, gains a cause through the new word; its exception for a check refused before any log opens does not cover it.
- The hook surface and its fail-open rows, `.andromeda/obs-plan.md:119` and `:200`: a refused socket directory is not one of the silent no-ops, because it writes one `hook-decision`, while exit 0, empty streams and no frame hold.
- The must-trace scenarios that reach a frame, `wait` / `last` at `.andromeda/obs-plan.md:361` first, then `send`, `answer`, `pause`, `release`, and the `link` / `unlink` verbs: the order before any frame is now the directory check, the connect, the euid decision, then `verify_server`.
- The `viola run` start scenario gains a step before any snapshot, child or `Server::bind`; a `Server::bind` failure stays what it was.
- MCP's `instance-unreachable` no-line list at `.andromeda/obs-plan.md:537` gains the socket-directory refusal and an euid refusal, and its Linux and Windows wording on the server mismatch moves.
- The thiserror type list at `.andromeda/obs-plan.md:496` stays unmoved: derives stay 9, and `Refused` gains a variant, not a derive.
- The log-level rows for `process-exit` exit 21 and `hook-decision` with a fail-open `detail` take the new word at WARN; no row of the mapping moves, and the exit set is unchanged.
- The correlation and module-boundary invariants at `.andromeda/obs-plan.md:643` and `:644`: a dropped peer sent no frame, so it is not a channel call and logs neither side; a row claiming every accepted connection logs a line needs reading against that.
- The rule at `.andromeda/obs-plan.md:123` and `:190` that `from` is logged as unauthenticated stands: the euid decision gates an account before any frame and authenticates no self-reported name.
- Perf rows `.andromeda/obs-plan-detail/obs-scope-summary.md:191` and `.andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:29`: this run's five hook means are 2.0, 2.0, 2.0, 2.0 and 2.3 ms, the highest max 2.8 ms, so the band stays a true record and no value moves; the open `max` / `mean` label on these rows is a prior wrap's and is not moved here.
- The surviving-mutant condition at `.andromeda/obs-plan.md:670`: an always-admit mutant of a seam's `peer_admitted` read survives every committed test by construction; its owners are test-plan §1's untestable zone and a dated note on `matrix#v1-44`, and a row claiming full mutation cover of those seams would be stale.
- §9's rule at `.andromeda/obs-plan.md:600` on where test homes live reads against the new `SocketRoot` helper in `tests/support/home.rs`: test-owned socket roots sit under `/tmp`, on `TMPDIR` on macOS and `XDG_RUNTIME_DIR` elsewhere, and the report moves no rule on homes.
- The `ci` profile's kill line moves from 25 to 26 `binary(` terms through `tests/security_negatives_socket_dir.rs`; no row states this count, so a true claim about the line's length does not change.
- `scripts/agent-run.sh cleanup` now reads `endpoint_gone` through `unconnectable`, true after a force-killed wrapper while its socket file stays until a next bind of that name; the document's shape is unchanged, and a row stating the old file-absence reading changes.
- Citations of the form path and line into a reached file may have shifted, above all `src/cmd/run.rs`, `src/cmd/client.rs`, `src/cmd/hook.rs`, `crates/viola-state/src/strict.rs`, `crates/viola-channel/src/server.rs` and `schemas/diag-line.v1.json`.
- The rules that `viola-channel` stays sync and Tokio-free, and that obs adds no dependency, stay true: `TokioClient::server_euid` sits behind the existing feature, no dependency was added, and `channel-tokio-free.sh` and `cargo deny check` are green.
- The rule at `.andromeda/obs-plan.md:557` that home-level lines stay path-free and `endpoint` is never logged holds over every new line; `human/hints.rs`'s new `socket-dir` hint stays in the owned human output, with no SGR and `hint:` last.

## New facts
- F1 · log-coverage · `socket-dir-refused` is a closed `detail` code on `process-exit` and on `hook-decision`, meaning the socket directory of a start or a recorded endpoint is refused: a symlink, not a directory, another owner, or a foreign mode bit.
- F2 · span-trace-coverage · `viola run` and `revive`'s start check the endpoint's parent, create it only when absent, then check again, so a failed create, a lost race and a refused check read one code; a refusal is one `process-exit` with `subject` self, `exit_code` 1 and `detail` `socket-dir-refused`, a step with no span, before any snapshot, child or bind.
- F3 · span-trace-coverage · A CLI frame verb (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`) meeting a refused socket directory writes one `process-exit` with `exit_code` 21, `detail` `socket-dir-refused` and `during` `connect` at WARN inside its dispatch span, no `channel-request` and no frame.
- F4 · span-trace-coverage · `viola hook` meeting a refused socket directory writes one `hook-decision` with `detail` `socket-dir-refused` at WARN inside `hook.handle`, on the dialog path after `hook-invoked` with a null `corr` and with `decision_emitted` false; exit 0, empty streams, no frame.
- F5 · error-capture-reporting · MCP's `frame` checks the socket directory after `live_endpoint` and returns `instance-unreachable` through `UnreachableCause::SocketDir`, with no frame and no line, the `isError` result its only record; `UnreachableCause::SocketDir` has no serde path and no `RefusalReason`.
- F6 · span-trace-coverage · The wrapper's `accept_loop` filters each accepted stream through `peer_admitted`, dropping a refused peer on the accept thread before its worker thread and before any frame: the peer reads end-of-file, no line is written because no frame exists to key one on, nothing reaches the outer terminal and the wheel does not move.
- F7 · span-trace-coverage · Each client seam reads the server's euid after the connect and before `verify_server`, with a peer it cannot read refused where euids are reported (`PEER_EUID_REPORTED`, Unix); on Windows no euid is read, and the DACL and the pid comparison stay the controls.
- F8 · error-capture-reporting · `Refused::NotADirectory` is a new fixed-message variant of `Refused`, naming no path.
- F9 · span-trace-coverage · The socket-directory refusal's human pair, printed on start and frame-verb refusals in plain ASCII with `hint:` last and no SGR, names only the parsed `ViolaName`; its wording is a planning form with the founder's own wording owed at Epoch 7's boundary batch.
- F10 · pii-scrubbing-compliance · The new refusal lines and documents hold no path, pid or uid, witnessed by `assert_names_nothing` and `digit_runs` in `tests/security_negatives_socket_dir.rs` on `ubuntu-latest` and `macos-latest`; that binary is `cfg(unix)` and has no Windows case.

## Untouched
- The obs tier and its exporter carve-outs, no OTel SDK and no external error reporter: no crate was added or removed and no dependency moved.
- The `veil` redaction layer's `toggle` ban and the cargo-deny bans: `deny.toml` was not edited and `cargo deny check` is green.
- `schemas/diag-detail.v1.json`, the detail-file schema and its content fields: the session added only home-level words to the sibling schema.
- `.github/workflows/ci.yml`, `nightly.yml` and `windows-mutants.yml`, with the step order, the secret-scan gating, the upload gates and the artifact retention: the report says no CI step changed.
- The G1 and G2 gate commands, `scripts/g2-zero-panics.sh` and `scripts/lint-probes.sh`: run green, not edited.
- The panic hook, its one-line record, its catch sites and its bounded exemptions: the session names none of them.
- The `viola-pty` seam, the child's `process-exit` from the handle wait and the vt100 feed's `parse-rejected`: that crate was not edited.
- The `viola-agent-claude` entity, with the R8 strip and its never-log floor, the drift reports and the `claude agents --json` parse: that crate was not edited.
- `events.ndjson` byte offsets, its no-rotation retention and the file modes of `diagnostics/`: the session changed no store format and no log file's location.
- Derived metric naming and the ban on in-process instruments: no instrument and no exporter was added.
