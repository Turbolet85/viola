# Codebase Research — 2026-10-04-wait-and-last

## Scope
- **Depth:** deep · **Reads:** 19 · **Globs/Greps:** 24
- **Harness rules consulted:** `.claude/rules/verification-harness.md` (read in full): 9 additions; the ones applied are the `test(/…/)` vs `binary(<stem>)` filter rule (2026-09-27/10-04), never piping `boot` (2026-09-25), and the short `TMPDIR` for socket paths (2026-10-04). `.claude/rules/testing.md` (loaded with the test files read): 25 additions; the ones applied are waiting on the exact asserted line (2026-09-24), `try_wait`-aware bounded waits below the kill line (2026-09-24/27), a remove-the-guard run per new guard (2026-09-25), and process-global `OnceLock` tests one per global (2026-09-24).
- **Platform issues consulted:** none. No runner-only bullet: the take-up CI read was green (`bcff692`, ci#37190826318, 15/15).

## Files inspected
- `src/main.rs` (1-300, 309-365): `role_of` (`:89-105`) files only `verify` as `Role::Cli` (`:97-98`). `prints_internal_error` (`:131-133`) prints for `Role::Cli` + `Failed | Panicked` only. The catch site prints at `:72-74`. The rstest tables are at `:309-323` (role), `:327-345` (exit code) and `:347-361` (prints).
- `src/cmd/mod.rs` (full, 104 lines): `Command` has `Run · Send · Verify · Hook`. The `Send` arm calls `crate::human::internal_error()` itself on `Err` (`:73`). `Verify` builds its sink from `VIOLA_NAME`.
- `src/cmd/send.rs` (full, 441 lines): the client template. `read_text` caps at `MAX_FRAME + 1`. `Reply` / `reply_of` / `fault_detail` / `fault_document` (`:87-130`); `Out` (`:153-221`, `--json` document vs human lines); `live_endpoint` (`:292-299`, liveness only); `own_name` (`:302-305`); `request` under `#[instrument(name = "send.client")]` (`:307-310`); `unreachable` (`:322-334`) logs `process-exit{exit_code:21, detail:"instance-dead", during}` at **ERROR**. `ChannelError::Connect` → `during:"connect"`, and `Closed | Io` → `during:"call"` (`:264-269`).
- `src/cmd/run.rs` (1-330; the unit test at `:569-590`): `Methods {name, instance_dir, send}` (`:63-67`). `dispatch` answers only `hook.event`, and every other method `-32601` (`:103-113`). `dispatch_call` routes `send` (`:115-120`). The server is served at `:196-201`, before `start_state` writes the start events (`:202`). The unit test at `:579-582` pins `["wait", "hook.dialog", "anything"]` → `MethodNotFound`. The file is 737 lines.
- `src/run/send.rs` (1-340): `SendSlot` waits on a `Condvar` with `wait_timeout(…, STEP)` (`STEP` = 20 ms, `:25`). The waiting loop re-reads the injected `Clock` each step (`:101-124`). `append_hook_event` (`:130-147`) is the wrapper's one append path for hook events, and its relabel settles the waiting send after the append. The send records go through `append_event_at` (`:248-284`).
- `src/cmd/hook.rs` (170-236): `deliver` sends `hook.event` over the channel (`:209-223`). `append_session_end` (`:227-236`) appends a SessionEnd directly with `try_append_event`, and only when the channel did not take it.
- `crates/viola-channel/src/client.rs` (full): `Client::request` blocks on `read_frame` with no client-side deadline (`:60-89`). The client `ResponseLine` drops with its default class `Error(Internal)` on an early `?` return (`:81`, `:84-86`), so a closed peer logs a client `channel-response{result_class:"error", error_code:-32603}` at ERROR.
- `crates/viola-channel/src/frame.rs` (17-41): `read_frame` returns `ChannelError::Closed` on EOF without `\n` (`:31-32`) and `Io` on a read error (`:26`).
- `crates/viola-channel/src/server.rs` (20-79, 240-470): one worker thread per connection (`serve`, `:67-74`). `dispatched` wraps `dispatch_call` in `catch_unwind` → `-32603` (`:246-267`). `METHODS` already lists `wait` and `last` (`:354-365`). `log_request` logs `corr · conn · srv_conn · method · sender · v` only (`:374-385`). `ResponseLine`'s `Drop` logs `result_class · error_code · duration_ms` with no `outcome` (`:406-470`).
- `crates/viola-state/src/events.rs` (full, 321 lines): writers only. `append_event`, `end_offset` (`:72-76`), `append_event_at` (`:80-90`), `try_append_event`. There is no reader, no torn-line healing and no tailing in `viola-state` (re-derived: `grep -n -E "torn|heal|fn read_" crates/viola-state/src/*.rs` → `read_snapshot` and `read_stamps` only).
- `src/human.rs` (1-112): the refusal pair, `write_internal_error` (`:22-24`), `write_result`, the send mirror writers, and `send_hint` (`:87-102`), whose exit-21 cause `not-running` → `"{name} is not running; viola list shows the live instances"`. There is no escaper.
- `src/obs.rs` (200-275): `internal_error_exit_line` (`:218-232`) writes `process-exit{exit_code:1, detail:"internal-error"}` only when the process ctx is `Run | Cli`. `report_internal_error` (`:242-248`) writes that line plus the chain into the detail file.
- `src/run/mod.rs` (20-90): `log_self_start`, and `log_self_exit` (ERROR with a detail, INFO without one).
- `schemas/diag-line.v1.json` (probed): `channel-request` admits `after` and `timeout_ms` (`allOf[19]`), `channel-response` admits `outcome` as a string (`allOf[20]`), and `process-exit.during` is the enum `connect | call` (`allOf[26]`).
- `crates/viola-agent-claude/src/hook.rs` (84, 143): Stop → `{"last_assistant_message": <string|null>}` passes the string through unchanged, control characters included.
- `fixtures/claude/2.1.283/Stop.default.json`: `last_assistant_message:"ok"`, a fixed value.
- `tests/hook_events.rs` (1-135): the precedent for per-test synthetic fixtures. `fake::write_fixture` writes into `<scratch>/fixtures/2.1.283/`, a `Stop` body carries the canary, and `Wrapper::boot(…, extra = ["--fixtures", <scratch>])`.
- `tests/cli_send.rs` (1-150, 320-360): the Path 2 shape. `boot()` waits for the `session-start` record. `spawn_send` / `finish` capture exit + stdout + stderr. `send_unreachable_name_exits_21_not_running` pins the human and `--json` exit-21 forms and the two `process-exit` lines.
- `tests/support/home.rs` (300-380): `Wrapper::boot` takes `script` + `extra`, `wait_ready`, and `stop` waits until the endpoint is unconnectable.
- `tests/channel_endpoint.rs` (100-125, 205-255): the companion pin; see Files to modify.

## Graph impact (from the code-graph query; rust plane, `tree-query-2026-10-04-wait-and-last.json`, 42 rows; lines below are editor lines = SCIP + 1)
- **role_of** — 2 callers: `main` @ `src/main.rs:53` and the rstest @ `:323`. The change is internal to `main.rs`.
- **prints_internal_error** — `main` @ `src/main.rs:72` and the rstest @ `:361`.
- **internal_error** (human) — `cmd::dispatch` @ `src/cmd/mod.rs:74` (the `Send` arm) and `main` @ `src/main.rs:73`. These two sites are the double print the fix removes.
- **append_hook_event** — `Methods::dispatch` @ `src/cmd/run.rs:109` plus 6 unit tests in `src/run/send.rs` (`:538`, `:576`, `:591`, `:608`, `:635`, `:655`). A signature change threads through all 7.
- **live_endpoint · own_name · reply_of · send_hint · write_send_unable** — callers only in `src/cmd/send.rs` and `src/human.rs`'s own tests. Extracting the shared client pieces touches only these two files plus the new verbs.
- **end_offset** — `src/run/send.rs:320`, viola-state's own tests, and `tests/cli_send.rs` (test-side reader, unaffected).

## Patterns detected
- **Condvar wake with injected-clock deadline** (`src/run/send.rs:101-124`): a `Mutex` state plus `Condvar::wait_timeout(STEP)` that re-reads `Clock::now()` against a deadline each step. Tests drive it with a `JumpClock` (`src/run/send.rs:363-371`). This is the precedent for `wait`'s parked handler and its §4 clock-driven unit case.
- **The wrapper's in-process append path** (`src/run/send.rs:130-147`, `src/run/send.rs:248-284`): every line the wrapper appends (hook events, send records) is written by the `run` process itself. The one out-of-process writer is the hook's direct SessionEnd (`src/cmd/hook.rs:227-236`), and it runs only when the channel did not take the event.
- **Client verb shape** (`src/cmd/send.rs:223-310`): obs init as `ObsProcess::Cli` with the instance, a liveness-only endpoint, one `Client::connect(…, "cli")?.request(method, params)` under a `<verb>.client` span, and the reply mapped to a document or human lines plus a typed exit and `log_self_exit`.
- **Per-test synthetic fixtures** (`tests/hook_events.rs:31-37`): a `Stop` body written per test into a scratch fixtures root, passed to `Wrapper::boot` via `--fixtures`.

## Conventions to follow
- **One `write_all` per human message** (`src/human.rs:10-12`, `:67-78`): `src/human.rs` stays print-macro-free and `#[allow]`-free.
- **Fixed-message thiserror / codes only** (`crates/viola-channel/src/lib.rs:30-46`): a new client error is a variant of the crate's one enum.
- **`#[instrument(skip_all, name = "<area>.<op>")]` only** (`src/cmd/send.rs:307`): the obs spans `wait.block`, `last.client` and `run.wait_dispatch` follow.
- **Unit tables with literal oracles** (`src/main.rs:309-361`): `#[case::label]` rows with integer exits.

## Measured facts (re-derived at HEAD)
- **E1 — mid-call close maps to exit 21.** The equality: a wrapper process that exits while a client is blocked in `Client::request` yields `ChannelError::Closed` (EOF, `frame.rs:31-32`) or `ChannelError::Io` (reset, `frame.rs:26`), and `send` already maps both to exit 21 `during:"call"` (`src/cmd/send.rs:267`). The client-side `channel-response` for that case reads `error`/`-32603` at ERROR (`client.rs:81`), not `outcome:"instance-unreachable"`.
- **E2 — the wake has one in-process source.** Every append that can make a parked `wait` return while its wrapper is alive is made in the `run` process: `hook.event` through `append_hook_event` (`src/cmd/run.rs:108-110` → `src/run/send.rs:142`). The hook's direct SessionEnd (`src/cmd/hook.rs:227-236`) happens only after `deliver` failed, so that wrapper is unreachable and its parked clients see E1. A `wait` called afterwards still finds the line by scanning from `after`.
- **E3 — no dialog event can be produced at HEAD.** `Methods::dispatch` answers `hook.dialog` `-32601` (`src/cmd/run.rs:104-105`), and `HOOK_KINDS` holds no dialog kind (`src/cmd/run.rs:70-76`). The pending-dialog state (`pending_dialog` in the snapshot) is `Dialog answers by dialog_id`'s (`working-route.md:78`).
- **E4 — the Err-path stderr line is witnessable on the real binary without a seam.** `target/harness/debug/viola` (built 2026-10-04 09:00Z, from the `bcff692` tree): `echo hi | viola --home <a regular file> send builder` → exit 1, stderr exactly `error: internal error\n`, stdout empty. The same holds for `verify`. Today `send`'s line comes from its dispatch arm, because `role_of` files it `Other`. So after the fix the same probe gives one line only if the arm no longer prints and `role_of` files `send`, `wait` and `last` as `Cli`. A double print reads as two lines and an unfiled verb as zero, so the witness cannot pass vacuously. Scratch: `<scratchpad>/home-is-a-file`.
- **E5 — the log schema already admits the obs fields.** `after` and `timeout_ms` on `channel-request`, and `outcome` (string) on `channel-response` (`schemas/diag-line.v1.json` `allOf[19]`, `allOf[20]`). No schema change is needed for them. `woken_kind` is absent (0 hits).
- **E6 — exit 21's level.** obs-plan §6 Log levels mapping (`.andromeda/obs-plan.md:844`) puts `process-exit` exit 21 at `warn`. `send`'s `unreachable` logs it at ERROR (`src/cmd/send.rs:325`). This is a pre-existing deviation in the code `wait`/`last` will share.
- **E7 — a Stop payload's control characters reach `turn-ended` intact** (`crates/viola-agent-claude/src/hook.rs:143`). So a scratch `Stop` fixture carrying ESC / CR / DEL / U+009B plus the canary drives `last`'s human-mode escaping end to end, on the `hook_events.rs` precedent, with no recorded fixture invented (the fixture is per-test synthetic, outside `fixtures/claude/`).
- **E9 — `EventKind` lacks the dialog kinds.** `viola-core`'s `EventKind` (`crates/viola-core/src/lib.rs:44-55`) holds 10 variants with no `question` / `permission` / `plan`, and its `as_str` table test is at `:276-286`. events.md says "Normalised kinds live only in `viola-core`", so the wake set's three dialog kinds are added there and named from there.
- **E10 — killing a wrapper from outside, per OS.** Windows: `OpenProcess` + `TerminateProcess` on the snapshot `pid` (`tests/run_cli.rs:606-623`; the child ends with its wrapper there). Unix: `Command::new("kill")` with a signal on the snapshot pid (`tests/cli_instance_state.rs:208-214`). Either way the wrapper dies without a SessionEnd, so a parked `wait` meets E1 rather than a `session-end` wake.
- **E8 — torn-line healing belongs to `working-route.md:85`** (Epoch 4 `Self-healing state`). This chunk's reader skips a last line with no `\n` without healing it or logging `state-recovered`.

## New files to create
- `src/run/wait.rs` — the wrapper's `wait` and `last`: the append feed (generation counter + Condvar, signalled after every wrapper append), the scan from `after` for the first wake kind, the injected-clock timeout, the in-memory newest turn and its start-up rebuild, plus unit tests
- `src/cmd/wait.rs` — `viola wait <name> [--after <cursor>] [--timeout-ms <n>] [--json]`
- `src/cmd/last.rs` — `viola last <name> [--json]`
- `src/cmd/client.rs` — the client plumbing `send`, `wait` and `last` share, moved out of `send.rs`: the reply shapes, the liveness-only endpoint, `own_name`, the exit-21 and fault outputs
- `tests/cli_wait_last.rs` — Path 3's cli + channel half (`path3_…`), the `--json` documents, the human lines and escaping, exit 21 before connect, the Err-path internal-error line
- `tests/chaos_wait_vanish.rs` — the wrapper killed while a `viola wait` is parked → exit 21 `during:"call"`
- `fixtures/fake-scripts/path3.json` — Path 3's gated turn: a PostToolUse (`activity`), then a Stop (`turn-ended`), each released by a control line

## Files to modify
- `src/main.rs` — `role_of` files `send`, `wait` and `last` as `Role::Cli`, plus the table rows
- `src/cmd/mod.rs` — the `Wait` and `Last` subcommands and arms; the `Send` arm stops printing its own `error: internal error`
- `src/cmd/send.rs` — uses `src/cmd/client.rs`; its exit-21 line at `warn`
- `src/cmd/run.rs` — `Methods` gains the feed and the newest turn, routes `wait` and `last`, rebuilds the newest turn before serving; the `:579` unit test drops `wait`
- `src/run/mod.rs` — declares `wait`
- `src/run/send.rs` — `append_hook_event` signals the feed and records a `turn-ended`; its 6 unit-test callers follow
- `src/human.rs` — the message-mode escaper and the `wait` / `last` / exit-21 writers
- `crates/viola-core/src/lib.rs` — `EventKind` gains `Question`, `Permission`, `Plan` (the wake set's dialog kinds), with their `as_str` rows
- `crates/viola-state/src/events.rs` — a capped reader of complete lines from a byte offset, each with its start and end offset
- `crates/viola-channel/src/server.rs` — `after` / `timeout_ms` on `channel-request` and `outcome` on `channel-response` for `wait`
- `crates/viola-channel/src/client.rs` — `after` / `timeout_ms` on the client's `channel-request`
- `tests/channel_endpoint.rs` — `channel_wrapper_logs_each_call_with_corr_and_conn` stops using `last` / `wait` as unserved methods (a bare `wait` would park the test)

## Open questions
- RESOLVED at P4 (F1, the founder's ruling, live, 2026-10-04, relayed by the Viola overseer): Does a CLI `wait` / `last` frame after `send`'s liveness-only pre-check need a fifth founder-live interim gap, or do these verbs wait for Epoch 6 server verification? → the fifth dated gap is ratified, in the same shape as send's, owned by `:109` / `:111`
- RESOLVED at P4 (a decisive lean, obs-plan §4 Scenario `wait` / `last` names `channel-response{outcome}`): Where does `wait`'s `outcome` reach `channel-response`? → derived in `viola-channel` from the `wait` result on both sides, so the `server.rs` / `client.rs` rows stand
