# Codebase Research — 2026-10-04-confirmed-send-with-cl-1-records

## Scope
- **Depth:** deep · **Reads:** 15 · **Globs/Greps:** 22
- **Harness rules consulted:** `.claude/rules/verification-harness.md`, read in full (60 lines), 8 Session Additions. Two of them apply here:
  - the full-path nextest `test(=tests::name)` filter (2026-09-27);
  - "a plan gate that calls a harness selector not yet built is a plan defect" (2026-09-29).
- **Platform issues consulted:** none. No runner-only bullet is folded: Setup 5a's only sha read green, and no CI-reading entry falls outside the operator leg.
- **Graph tooling:** the host Python has no `duckdb` (health check 11's WARN). The queries ran through a session-scratchpad venv built from `scripts/requirements.txt` (duckdb 1.5.6), outside the repo. The trace is `.andromeda/runs/2026-10-04T05-41-59-phase/tree-query-2026-10-04-confirmed-send-with-cl-1-records.json`, plane `rust`, 6 queries.

## CI verdict (Setup 5a re-read)
- `51b533886b7c` (ci#37180564195) read **`verdict: green` · checks 15/15 · wall 282 s**, completed/success. Re-read at P3, 2026-10-04 ~05:52Z. Nothing to disposition.

## Files inspected
- `src/cmd/run.rs` (full):
  - `Methods` serves only `hook.event`; everything else is `MethodNotFound` (:100-111).
  - The unit test `methods_answer_method_not_found_for_every_other_method` pins `"send"` → `-32601` (:557-566). It must change.
  - `hook_event_line` admits `prompt-submitted` `origin` ∈ `harness|human` only (:85-86).
  - `Methods` is built in `start()` (:185-188), BEFORE the child is spawned (:198).
  - `gate::start` is called in `pump_child` (:420), and the pump gets `io::stdin()` + `Tee` (:429-435).
- `src/run/gate.rs` (full):
  - `Tee::write` writes the human's bytes first, then does `let _ = self.feed.send(…)` (:34-37).
  - `start` makes an unbounded `mpsc::channel()` (:47).
  - The `Screen` lives only inside the feed thread (:48-60), so no verdict can be read from outside it.
  - `guarded` poisons the model and writes `parse-rejected{vt100-feed, panicked, 1}` (:64-75).
- `crates/viola-agent-claude/src/screen.rs` (full):
  - the constants: `QUIET_PERIOD` 300 ms, `GATE_MAX_WAIT` 5 s, `CONFIRM_WINDOW_FALLBACK` 10 s (:9-16);
  - `Signatures{input_box, modals}` (:20-24) and `Readiness` / `GateStep` (:26-45);
  - `verdict` returns `InputNotReady` when no row holds an `input_box` literal (:110-114).
- `crates/viola-pty/src/pump.rs` (full):
  - `pump` takes the PTY writer once (`pty.writer()?`, :68) and gives it to ONE thread that copies `input` into it (:76-80);
  - no other code can reach the child's input writer.
- `crates/viola-channel/src/server.rs` (:1-450):
  - `Dispatch::dispatch(&self, method, params)` (:26-29);
  - `answer()` strips `conn` before dispatch (:198-200) and knows the request `id` and `Peer` (:195, :209-213), but passes neither to `dispatch` (:242-244);
  - `METHODS` already lists `"send"` (:333-344);
  - a connection worker thread runs per connection (:146-154), so a blocking `send` dispatch holds only its own connection;
  - `ResponseLine` writes `channel-response` from `Drop` (:385-450).
- `crates/viola-channel/src/client.rs` (:1-240):
  - `Client::connect` + `request` are sync and stamp `v` / `sender` / `conn` (:33-89);
  - on Windows the pipe opens with SQOS Identification (:143-226);
  - no server verification exists (no pid / start-time / peer check before the first frame).
- `crates/viola-core/src/lib.rs` (full):
  - `MAX_FRAME`, `SPINE_DEADLINE`, `Clock` / `SystemClock`, `ViolaName`;
  - `EventKind` has 7 kinds and no send kinds (:43-66);
  - no `RefusalReason`, no `validate_paste_text`, no error enum;
  - dependencies: `nutype` only (`crates/viola-core/Cargo.toml`), so serde for a kebab `RefusalReason` is a new dependency edge.
- `crates/viola-state/src/events.rs` (:20-100): `append_event` returns `()` (:63-68), takes the `events.lock` sibling and makes one `write_all`. No API yields the end offset that `cursor` needs.
- `crates/viola-core/src/obs.rs`: `ObsEvent::{SendIssued, SendConfirmed, SendRefused}` already exist (:19-21, :65-67).
- `schemas/diag-line.v1.json`:
  - the `send-*` field allow-list already carries `corr`, `from`, `from_trust`, `text_bytes`, `confirmed`, `refusal`, `detail`, `side`, `wheel`, `duration_ms`, `conn`, `srv_conn` and `rpc_id` (:59-65);
  - `parse-rejected.parser` has `paste-text` and `.detail` has `control-character` (:101-103);
  - `corr` is required on wrapper-side `send-*` (:116).
  - No schema change is owed for those lines.
- `src/human.rs` (full): `write_refusal` (`unable: …\nhint: …`), `write_internal_error`, `write_result`, each one write (:10-39). It has no TTY split, no `IsTerminal` and no readback-mirror writer.
- `src/cmd/mod.rs` (full): the subcommands are `Run`, `Verify` and hidden `Hook` (:29-38). `Verify` resolves the instance from `VIOLA_NAME` for its `cli` detail sink (:62-72).
- `src/bin/viola-fake-agent.rs`: `--suppress-prompt-submit` / `--local-command-mode` are built (:73-74).
  - The receipt `prompt` carries `bare_esc` and `submit` ∈ `suppressed|local-command|…` (:270-286).
  - Bracketed-paste parsing is in `Input` (:363-420).
  - In interactive mode it writes no screen bytes (its only stdout prints are print-mode / `--version`, :501, :535).
  - `--vt100-panic-bytes` is absent.
- `crates/viola-agent-claude/src/ledger.rs`: `LedgerRow::ALL` has 6 rows (:25). No local-command list or row exists (`grep -n -i 'local|clear|post.?condition'` hit only the module doc line 3 and the generic post-condition fn :180).
- `crates/viola-agent-claude/src/hook.rs`: `HARNESS_PREFIXES` (:90), `normalise` (:98), `prompt_origin` (raw-prefix classification, :159-160) and the `<pasted_content>` unwrap (:174-203).
- `scripts/g2-zero-panics.sh`: it counts `event:"panic"` lines in role files (`*/diagnostics/*.ndjson`, not `detail-*`) under `target/e2e-home` only (:19-35). The one exemption is `src/cmd/hook/seam.rs` (:10).
- `tests/support/fake.rs`: `receipt`, `of_kind` and `wait_for` are the receipt oracle (:40-77).
- `tests/support/home.rs`: `TestHome` / `StampedHome` (:106-226).
- `tests/cli_controls_not_disableable.rs`: the table's `#[case::hook_panic_seam_*]` rows (:109-112).
- `fuzz/Cargo.toml` is the only code-side pin of the fuzz target list (`grep -rln vt100_feed` outside `.andromeda` / chunk dirs: `fuzz/Cargo.toml` alone). `fuzz/corpus/` holds `channel_frame`, `hook_stdin`, `viola_name` and `vt100_feed`.

## Graph impact
- **`dispatch`** (trait method). One production caller: `server/dispatched()` @ `crates/viola-channel/src/server.rs:243`. Five unit-test callers in `src/cmd/run.rs` (:561, :575, :599, :618, :629).
  - `Dispatch` references (`refs`, 17): the implementors are `src/cmd/run.rs:100`, the server tests `Echo` (`server.rs:466`) and `Recording` (`:646`), the client tests `Ids` (`client.rs:247`) and `Recorder` (`:333`), `crates/viola-channel/tests/channel_frames.rs:18` and `tests/channel_endpoint.rs:32`.
  - A changed `dispatch` signature touches all seven. A defaulted second trait method touches only `src/cmd/run.rs` and `server.rs`.
- **`append_event`** — production callers `Methods::dispatch` @ `src/cmd/run.rs:107` and `start_state()` @ `src/cmd/run.rs:372`; 4 in-crate tests. An added offset-returning function leaves both untouched.
- **`start`** (gate.rs) — 1 production caller `pump_child()` @ `src/cmd/run.rs:420`; 1 test (`run_feed`, `gate.rs:181`).
- **`pump`** (viola-pty) — 1 production caller `pump_child()` @ `src/cmd/run.rs:429`; 11 in-crate tests (`pump.rs:187…571`). A changed `pump` signature touches those 11 test call sites.
- **`request`** (client) — no production caller yet; 4 client tests + 3 `tests/channel_endpoint.rs` calls (:113, :217, :303). `viola send` becomes its first production caller.

## Patterns detected
- **Tee-first passthrough** (`src/run/gate.rs:34-37`): the human's bytes are written before the copy is offered, and a failed send to the feed is ignored. A bounded queue keeps that property only if the offer can never block: `try_send` + an overflow mark, never `send`.
- **Fixed-shape refusal pair in one write** (`src/human.rs:10-12`): the send mirror's `[/ ] unable` + `hint:` follows the same one-write rule.
- **Wrapper re-validation** (`src/cmd/run.rs:79-97`): closed kind, object `data`, closed `origin`. `send` params re-run `validate_paste_text` the same way.
- **Drop-guard response line** (`server.rs:385-450`): `channel-response` is logged on every path, a panic included.
- **Literal-pinned test tables** (`tests/cli_controls_not_disableable.rs:109-112`; test-plan §4 Conventions): expected orders and exit codes are literals.

## Conventions to follow
- **One write per human line / per event line**: `src/human.rs:10`, `crates/viola-state/src/events.rs:88-93`.
- **obs_event! with typed `corr`**: `src/run/gate.rs:67-73`, `crates/viola-channel/src/client.rs:72-80`.
- **Spans `#[instrument(skip_all, name = "...")]`**: `src/cmd/run.rs:148`, `crates/viola-channel/src/server.rs:231`.
- **Test-only behaviour behind `fake-agent` / bin flags, never env**: `src/cmd/run.rs:50-58`; fake-agent flags at `src/bin/viola-fake-agent.rs:64-76`.
- **Proptest at 512 cases with `SourceParallel("proptest-regressions")`**: `crates/viola-core/src/lib.rs:101-109`.

## Mechanism equalities (verified at HEAD)
- **E1 — "the gate as landed refuses every fake-agent send."**
  - `Screen::verdict` returns `Done(InputNotReady)` when no row holds an `input_box` literal (`screen.rs:103-114`).
  - No production `Signatures` exists (`grep -rn 'Signatures {'` outside `screen.rs`: only `fuzz/fuzz_targets/vt100_feed.rs:14`).
  - The fake agent paints no screen in interactive mode.
  - So for THESE inputs (fake agent, any version) the landed gate reads `input-not-ready`. Some reading of "falls back to delivery confirmation only" is load-bearing (fork F1).
- **E2 — "a forced feed panic writes a G2-counted line."** `viola_panic_hook` writes `event:"panic"` for every panic, caught ones included (`src/main.rs:138`, `:222-223`). G2 counts role-file panic lines under `target/e2e-home` (`g2-zero-panics.sh:35`). So a run-level forced feed panic in a home under `target/e2e-home` reads G2 red. The same panic in a home outside that tree is outside G2's scope by construction.
- **E3 — "the wrapper can type a paste."** False at HEAD: the PTY writer is owned by `pump`'s input thread (`pump.rs:68`, `:76-80`). A paste path needs a new seam in viola-pty, a writer shared under a lock or an injected-input channel. The human-wins property then holds as "held only behind the current atomic paste".
- **E4 — "`cursor` = the pre-paste end offset."** No API yields it (`events.rs:63-68`). It needs an offset-returning read or append under the same `events.lock`.
- **E5 — "`origin:"driver"` appears on `prompt-submitted`."** The hook path cannot produce it (`run.rs:85-86`; `hook.rs:159-160` files `human|harness`). The wrapper relabels on a match against its own in-flight send. A driver text starting with a `HARNESS_PREFIXES` literal arrives filed `harness`, so the match must key on the text, not on the hook's origin.

## New files to create
- `src/cmd/send.rs` — the `viola send <name>` verb: stdin / `--file` under `take(MAX_FRAME + 1)`, client-side `validate_paste_text`, one `send` request, the readback mirror and exit code
- `src/run/send.rs` — the wrapper's `send` handler: re-validation, the one-in-flight guard, the gate wait, `send-issued`, one paste, the confirmation matcher, outcome records and `send-*` lines
- `fuzz/fuzz_targets/paste_text.rs` — the fifth fuzz target over `validate_paste_text`
- `fuzz/corpus/paste_text/` — its synthetic seeded corpus
- `tests/cli_send.rs` — critical Path 2 over the CLI and receipt: confirmed, `mute`, `local` (per fork F2), multi-line, the `[RB]` / `[/ ]` mirror lines, `--json`
- `tests/channel_paste_validation.rs` — the raw-client `control-character` cases at the wrapper (test-plan §5 Wrapper channel)
- `tests/chaos_feed_panic.rs` — the forced vt100 feed panic → exit 13 `input-not-ready`, passthrough intact (per fork F4)

## Files to modify
- `crates/viola-core/src/lib.rs` — `RefusalReason` + the send `not-delivered` details, `validate_paste_text`, the send event kinds, the confirmation window's use
- `crates/viola-core/Cargo.toml` — `serde` for the kebab-case `RefusalReason` round-trip
- `crates/viola-core/proptest-regressions/` — the committed regressions for the `validate_paste_text` property
- `crates/viola-state/src/events.rs` — the end offset under `events.lock` (`cursor`, and D-28's pre-issue `corr`)
- `crates/viola-channel/src/server.rs` — the request `id` and `Peer` reach the dispatch (`send-*` lines' `rpc_id`, `conn` / `srv_conn`, D-30)
- `crates/viola-channel/src/lib.rs` — re-export of any new dispatch-context type
- `crates/viola-channel/src/client.rs` — `Dispatch` test implementors, only if the trait signature changes
- `crates/viola-channel/tests/channel_frames.rs` — `Dispatch` implementor, only if the trait signature changes
- `tests/channel_endpoint.rs` — `Dispatch` implementor, only if the trait signature changes
- `crates/viola-pty/src/pump.rs` — the paste seam: one atomic write into the child's input, human bytes held behind it
- `crates/viola-pty/src/lib.rs` — export of the paste seam type
- `crates/viola-agent-claude/src/screen.rs` — the signature-less verdict reading (per fork F1)
- `fuzz/fuzz_targets/vt100_feed.rs` — its `verdict` call takes the new `Option<&Signatures>` parameter
- `crates/viola-agent-claude/src/ledger.rs` — the local-command list, only under fork F2's land branch
- `src/run/gate.rs` — bounded tee → feed queue (`sync_channel` + `try_send`, overflow → `input-not-ready`), a verdict query, the `run.readiness_gate` span
- `src/run/mod.rs` — `mod send`
- `src/cmd/run.rs` — `Methods` serves `send`, `driver` relabel on match, late-bound paste and gate handles; the `-32601` test updated
- `src/cmd/mod.rs` — the `Send` subcommand and its `cli` detail sink
- `src/human.rs` — the readback mirror writers (`[  ] open` stderr TTY-only via `IsTerminal`, `[RB]` / `unconfirmable` stdout, `[/ ] unable` + per-detail `hint:` stderr)
- `src/bin/viola-fake-agent.rs` — `--vt100-panic-bytes`
- `fuzz/Cargo.toml` — the `paste_text` `[[bin]]`
- `tests/cli_controls_not_disableable.rs` — the `send` ESC → exit 13 `control-character` row
- `.config/nextest.toml` — a `profile.mutants` override for the `send_window_` tests, which wait out the product's own 10 s confirmation window; the profile kills a root test at 5 s × 2 (`slow-timeout = { period = "5s", terminate-after = 2 }`), and the `package(viola-e2e)` override is the precedent
- `tests/support/home.rs` — a sized `Wrapper` boot (`OuterPty::spawn_sized`, `tests/support/outer_pty.rs:32`) and the out-of-scan chaos home (fork F4)
- `crates/viola-agent-claude/src/hook.rs` — none expected; listed only if the matcher needs a shared normaliser export

## Open questions
- **F1, the gate on a build with no compiled signatures (every build today).** Blocks: plan-decision. E1 shows the landed gate refuses every fake-agent send. Two readings of architecture [Screen Model]'s "falls back to delivery confirmation only":
  - (a) with no signature rows the gate checks poisoned + quiet-within-max-wait, skips the signature read, and confirmation decides;
  - (b) the gate is skipped entirely, which breaks test-plan §6 Chaos's "forced panic → `input-not-ready`".
- **F2, CARRY 1's local-command rows.** Blocks: plan-decision. No list exists. A `LedgerRow` in `ALL` makes every stamp unverified until a probe passes, and a typed `/clear` probe needs the held PTY drive. Without a list, `local` sends end `not-delivered` / `no-prompt-submitted`, never `unconfirmable`.
- **F3, server verification before `send`'s first frame.** Blocks: plan-decision.
  - Missing pieces: no client verifier, no strict-modes checker and no peer-identity check exist (`grep -rln 'strict|peer|euid'` over `crates/*/src src`: none outside the snapshot liveness helpers). All are owned by Epoch 6 `:109` / `:111`.
  - The `hook.event` exception may not be borrowed (security-plan-amendments-archive 2026-09-28).
  - So `viola send` without them is a fourth dated gap: a boundary widening, shown at P4 and held under the autonomous directive.
