# Codebase Research — 2026-10-04-dialog-answers-by-dialog-id

_Revised 2026-10-04 (phase revision, run `.andromeda/runs/2026-10-04T12-15-37-phase/`) on the founder's live rulings of ~11:55Z
and ~14:08Z (relayed by the overseer): dialog fixtures come from the viola-lab prototype's live 2.1.287 captures, no new ledger
row lands here (S3 / S7 / S8 / concurrency go to `:82` with its own re-probe), and non-null decisions flow on the 6-row spine
stamp as a dated gap closed by `:82`. The first research pass (run `2026-10-04T10-52-40-phase`) stands where not revised below._

## Scope
- **Depth:** deep · **Reads:** 24 (first pass) + 14 (revision) · **Globs/Greps:** 31 (first pass) + 12 (revision)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` (read in full at the revision; 8 Session Additions applied: the `--in-diff` witness rule, never piping `boot`, judging mutation by verdict, `test(=tests::name)` / `binary(<stem>)` filters, unbuilt-selector plan defect, reverse-order pairs for a local red, short `TMPDIR`) · `.claude/rules/testing.md` (read in full at the revision; 26 Session Additions applied, chiefly: unobservable bodies are unkillable, timing reds are measured not re-bounded, a security property needs an in-crate test, waits below the 20 s cargo-mutants floor, a forced window shown red then green)
- **Platform issues consulted:** red B's signature (`Auto-set build timeout to 1s` / `TIMEOUT … in 1s build`) → cargo-mutants 27.1.0's own source, read from the local registry copy: `timeouts.rs:80-95` `build_timeout = baseline × multiplier` with no lower bound, `main.rs:233-238` `--build-timeout-multiplier` `conflicts_with = "build_timeout"`. Red A is in-repo test code; no hosted-runner tracker search applies.

## Files inspected
- First pass (unchanged): `src/cmd/run.rs`, `src/cmd/hook.rs`, `crates/viola-agent-claude/src/{hook,ledger}.rs`, `crates/viola-core/src/{lib,obs}.rs`, `crates/viola-state/src/{snapshot,stamps,fs}.rs`, `src/run/{version_gate,wait,send}.rs`, `crates/viola-channel/src/client.rs`, `src/cmd/{mod,client,wait,send}.rs`, `src/main.rs`, `src/human.rs`, `plugin/hooks/hooks.json`, `src/bin/viola-fake-agent.rs`, `fixtures/`, `crates/viola-e2e/src/harness/run/{perf,mutants}.rs`, `tests/{cli_send,cli_controls_not_disableable}.rs`, `.andromeda/input.md:125-132` / `:215-226`, `viola-0.1.0/intent.md:187-194`, the two failing CI job logs of ci#37196414168.
- `src/cmd/verify.rs` (full) — `measure` runs ONE print-mode probe (`-p PROBE_PROMPT --model haiku --plugin-dir … --no-session-persistence`); `record` writes the first capture of each `CAPTURE_EVENTS` spine event as `<dir>/<version>/<Event>.default.json` after `ledger::scrub` + `ledger::is_clean`, through `payload.to_string()` (serde_json's sorted-key map) plus `\n`. With no new row this file does not change: the spine recording at 2.1.287 is this verb as it stands.
- `src/bin/viola-fake-agent.rs` (140-330) — `fire` reads `<fixtures>/<cli_version()>/<event>.<variant>.json` and runs every `type: "command"` hook of `event` in turn, each awaited (`wait_with_output`) with the fixture on stdin; `hook_commands` reads no `matcher`; `DEFAULT_CLI_VERSION` = `2.1.283` (`:18`), its literal pinned at `:651`.
- `schemas/fake-script.v1.json` — the step `event` enum already admits `PreToolUse` / `PermissionRequest`; `variant` is a free string.
- `fixtures/fake-scripts/gated-turn.json` and `tests/cli_fake_agent.rs:460-487` — a `PreToolUse` `ask` step against a synthetic fixture with no `tool_name` and a matcher-less echo plugin; an absent matcher matching all keeps it unchanged.
- `tests/contract_fixture_hygiene.rs` (1-80) + `tests/support/hygiene.rs` — walks every `fixtures/claude/*/*.json` (non-`.json` files skipped, `claude_fixtures_walks_every_version_dir_and_only_json_files`), checks no absolute path (`^[A-Za-z]:[\/]`, `/home/`, `/Users/`, `\Users\`), no host username as a whole word, and `schemas/claude-fixture.v1.json` (only `hook_event_name` pinned, enum already holds both dialog events). The schema's `description` says "a hook payload the CLI handed viola verify's capture plugin".
- `tests/contract_fake_agent_drift.rs` — per recorded version dir, a fake-agent print turn's spine stdin bytes equal each `<Event>.default.json`; reads only the four spine fixtures.
- `tests/support/{fake,home,verify}.rs` — `RECORDED_CLI_VERSION` (`fake.rs:16`) feeds `Wrapper::boot`'s `--cli-version` (`home.rs:333`) and `stamped_home`'s verify (`verify.rs:140`).
- `crates/viola-e2e/src/harness/boot.rs:22` `DEFAULT_CLI_VERSION`, read by `viola-harness`'s `--cli-version` default, `perf.rs:164`, `tests/cli.rs`, `tests/harness_lifecycle.rs`.
- The prototype's live captures, `~/.viola/sessions/*/events.ndjson` (read-only, through a scratch reader; no content copied here): each line `{"data":<the hook payload, keys sorted>,"event":…,"name":…,"tool":…,"ts_ms":…}`. Dialog payload keys: `cwd · effort · hook_event_name · permission_mode · prompt_id · scratchpad_dir · session_id · tool_input · tool_name · transcript_path`, plus `tool_use_id` on PreToolUse only (one PreToolUse also carries `agent_id`).
- `Cargo.toml` — no `insta` in `[workspace.dependencies]` or any `[dev-dependencies]` (`grep -n insta Cargo.toml` → 0 hits; `grep -rn 'insta::' tests src crates` → 0 hits), while testing.md §Framework names `insta 1.48.0 (check mode only)` for the decision bodies.

## Graph impact (rust plane; trace `.andromeda/runs/2026-10-04T10-52-40-phase/tree-query-2026-10-04-dialog-answers-by-dialog-id.json`, first pass — the revision adds no symbol it did not query)
- **append_hook_event** — called from `Methods::dispatch` @ `src/cmd/run.rs:111` and 7 unit tests in `src/run/send.rs`. The dialog event appends through the same `feed.appending` so it wakes `wait`.
- **appending** — one product caller, `append_hook_event` @ `src/run/send.rs:145`; three tests in `src/run/wait.rs`. A dialog append is a second product caller.
- **role_of** — no change; `answer` reads as `Role::Cli` by construction (`src/main.rs:96-101`).
- **read_stamps** — product callers `version_gate` (`src/run/version_gate.rs:136-137`) and `verify`; the strict-modes read changes only the gate's call.
- **HookEvent::ALL** — read by `parse_capture_file_name` (`ledger.rs:137`): with the two dialog events in `ALL`, `PreToolUse.1.json` parses, so the `unknown_event` case (`ledger.rs:613`) is re-pinned; `event_name` (`ledger.rs:71-81`) gains the two arms. `CAPTURE_EVENTS` and `LedgerRow::ALL` are unchanged.
- **CLI-version constants** (re-derived: `grep -rn 'RECORDED_CLI_VERSION\|DEFAULT_CLI_VERSION' src crates tests --include=*.rs`): three constants feed every fixture-chain boot; `grep -rn '2\.1\.283' src crates tests --include=*.rs` → 16 lines in 7 files, of which only the three constants and the three version-answer assertions (`src/bin/viola-fake-agent.rs:651`, `tests/run_cli.rs:544`, `tests/tui_passthrough.rs:270`) follow the default; the rest are stamped-line parser inputs (`boot.rs:546-557`, `:736-755`, `run.rs:904`) and a `parse_version` case (`ledger.rs:467`) that stay.

## Patterns detected
- **Append under the feed lock** (`src/run/wait.rs:68-81`): the append closure runs inside the lock, then the generation bumps and parked waits wake.
- **Rebuild once before serving** (`src/cmd/run.rs:200-209`, `src/run/wait.rs:84-95`): the `dialog_id` counter restore takes the same pre-`serve` pass.
- **In-flight slot with a Condvar and an injected clock** (`src/run/send.rs` `SendSlot`): the pending dialog's await is the same shape.
- **Shared CLI client verb** (`src/cmd/client.rs`; `send` / `wait` / `last`): `client::start` → `live_endpoint` (exit 21) → one request → `reply_of` → `--json` or human line, `exit_of` for refusals.
- **Exec-form hook registration** (`plugin/hooks/hooks.json`).
- **Recorded fixtures are re-serialized** (`src/cmd/verify.rs:319` `payload.to_string()`): the spine fixtures are serde_json's sorted-key form, not the CLI's byte order — the same form the prototype stored, so a relayed fixture written as `data.to_string()` + `\n` is the recorded fixtures' own form.

## Conventions to follow
- **Closed types on the wire**: `dialog_id` a `u64`, `behavior` a closed enum per kind, the dialog kind a closed enum (security-plan §Input Validation; api.md).
- **Per-method `from` typing** (`parse_from`, `src/run/send.rs`; `src/run/wait.rs:107`).
- **Free text through `validate_paste_text`**, client and wrapper both.
- **Logs only via `obs_event!`** with `corr` the `dialog_id` number (observability.md).
- **viola-e2e tests take labelled case tables, not rstest**; root tests use the rstest chain `home → fake_agent_path → stamped_home → booted_wrapper` and `StampedHome::unstamped`.
- **Workspace-pinned dependencies** (`Cargo.toml:196` `rstest = { version = "=0.27.0", default-features = false }`): a new test-only crate is a `[workspace.dependencies]` exact pin plus a root `[dev-dependencies]` `.workspace = true` line.

## Mechanism equalities (verified at HEAD, against a run, or measured)
- Red B: `baseline_build ≈ 0 s` × `--build-timeout-multiplier=5` yields `Auto-set build timeout to 1s` on the CI runner and `0s` on this host (measured at implement, `evidence/red-b.md`); an absolute `--build-timeout` bypasses the multiplier (`timeouts.rs:80-82`, `options.build_timeout` read first). The control is `MUTANTS_PROGRESS` (`mutants.rs:21`).
- Red A: when the child has exited before `spawn_send` writes, `write_all` returns `BrokenPipe` — measured red at implement on a forced window (`evidence/red-a.md`).
- **Print mode raises no dialog hook** (measured at implement, `evidence/print-mode-dialog-probe.md`): on 2.1.287, `claude -p` — default mode, `--permission-mode plan`, and `--tools "AskUserQuestion,ExitPlanMode,Bash"` — had neither dialog tool in its tool list and fired no PreToolUse(matcher) / PermissionRequest hook. So no `viola verify` probe can record a dialog fixture or measure S3 / S7 / S8 / concurrency; the relayed captures are the only 2.1.287 dialog payloads on this host.
- **A PermissionRequest repeats its PreToolUse** (measured over the prototype captures by `.andromeda/runs/2026-10-04T12-15-37-phase/capture-pairs.py`, 2026-10-04 ~14:10Z): every PermissionRequest for `ExitPlanMode` (3) and `AskUserQuestion` (5) — 8 of 8 — follows a PreToolUse of the same tool in the same session whose `tool_input` is EQUAL, and differs from it only by lacking `tool_use_id`. So a PermissionRequest carries no id that names its PreToolUse; the plan-style continuation must match by kind and order (the next PermissionRequest of the armed dialog's tool), and the fixtures are taken as coherent PreToolUse → PermissionRequest PAIRS.
- **AskUserQuestion raises PermissionRequest too** (`capture-reader.py`): 5 PermissionRequest `AskUserQuestion` captures exist, so plan step 4's "any other PermissionRequest → `permission`" would log a second dialog for one question; the classifier maps PermissionRequest `AskUserQuestion` to the question continuation (overseer resolution 2).
- **No ordinary-tool PermissionRequest exists** in the captures (`capture-reader.py` beside it: PermissionRequest tool names are only `ExitPlanMode` and `AskUserQuestion`), so the `permission` kind has no relayed fixture: its classification and bodies are unit/insta-pinned, its e2e owed to `:82` (overseer resolution 3).
- The pending-dialog return: at HEAD `wait` without `after` sets `from = end_offset` (`src/run/wait.rs:108-111`).

## New files to create
- `src/cmd/answer.rs` — the `viola answer <target> <dialog_id> [--file] [--json]` verb on the shared client pattern
- `src/run/dialog.rs` — the wrapper's dialog slot: `dialog_id` counter restored at start, one pending dialog, the await with its deadline, `answer` matching, the snapshot's `pending_dialog`, the plan-style continuation
- `crates/viola-agent-claude/src/dialog.rs` — PreToolUse / PermissionRequest payload → dialog kind + `data` (or the continuation), and the S3 / S7 / S8 decision bodies (pure, no I/O)
- `crates/viola-state/src/strict.rs` — the strict-modes check, first used by `run`'s stamps read
- `tests/cli_answer.rs` — test-plan §6 Path 4 (cli + wrapper channel half), the unverified branch, the `v1-30` dialog-kind wait witness
- `fixtures/fake-scripts/path4.json` — the gated Path 4 turn script
- `fixtures/claude/2.1.287/RELAYED.md` — the relayed fixtures' provenance: source session, line, sha256, the founder's ruling, the overseer's review
- `viola-0.1.0/chunks/2026-10-04-dialog-answers-by-dialog-id/evidence/` — red A / red B / print-mode probe records (written), the relay script, the relay record, the spine recording's output

## Files to modify
- `Cargo.toml` — `insta` `=1.48.0` pinned in `[workspace.dependencies]` and taken as a root dev-dependency
- `Cargo.lock` — the insta entries
- `src/cmd/mod.rs` — the `Answer` subcommand and its dispatch arm
- `src/cmd/run.rs` — `Methods` serves `hook.dialog` and `answer`; the dialog slot built and its counter restored before `server.serve`; the unknown-method unit test re-pinned
- `src/cmd/hook.rs` — the dialog tier: `pre-tool-use` / `permission-request` send `hook.dialog` under the dialog deadline and print the decision body
- `src/run/mod.rs` — `mod dialog`
- `src/run/wait.rs` — the without-`after` pending-dialog return
- `src/human.rs` — the `answered  <name>  dialog <id>` line and the `answer` refusal hints
- `crates/viola-core/src/lib.rs` — `DIALOG_DEADLINE`
- `crates/viola-agent-claude/src/lib.rs` — `mod dialog`
- `crates/viola-agent-claude/src/hook.rs` — `HookEvent` gains `PreToolUse` and `PermissionRequest`; the non-event test re-pinned
- `crates/viola-agent-claude/src/ledger.rs` — `event_name` for the two new events; the `unknown_event` capture-name case re-pinned; no new row
- `crates/viola-channel/src/client.rs` — a request bounded by a read deadline, for the hook's dialog wait
- `crates/viola-state/src/lib.rs` — `mod strict`
- `crates/viola-state/src/snapshot.rs` — the additive `pending_dialog` field
- `crates/viola-state/src/stamps.rs` — the strict-modes-checked stamps read
- `src/run/version_gate.rs` — `run`'s stamps read through the strict-modes check
- `plugin/hooks/hooks.json` — PreToolUse (matcher `AskUserQuestion|ExitPlanMode`) and PermissionRequest entries, each with a `timeout` above the dialog deadline
- `src/bin/viola-fake-agent.rs` — hook `matcher` evaluation; `DEFAULT_CLI_VERSION` and its pinned literal at 2.1.287
- `schemas/claude-fixture.v1.json` — the `description` names relayed captures beside `viola verify` recordings
- `crates/viola-e2e/src/harness/boot.rs` — `DEFAULT_CLI_VERSION` at 2.1.287
- `crates/viola-e2e/src/harness/run/perf.rs` — the fifth row `pre-tool-use` and its unstamped session
- `crates/viola-e2e/src/harness/run/mutants.rs` — red B's absolute build timeout (landed)
- `tests/cli_send.rs` — red A's `BrokenPipe`-only tolerance and the forced-window case (landed)
- `tests/cli_controls_not_disableable.rs` — the `answer` unstamped exit-12 row
- `tests/hook_events.rs` — the registered hook event set it pins
- `tests/support/fake.rs` — `RECORDED_CLI_VERSION` at 2.1.287
- `tests/run_cli.rs` — the fake agent's version-answer literal
- `tests/tui_passthrough.rs` — the fake agent's version-answer literal
- derived `fixtures/claude/2.1.287/*.default.json` by `viola verify --record` — the four spine fixtures, recorded once from the installed 2.1.287
- derived `fixtures/claude/2.1.287/PreToolUse.*.json` by `python3 viola-0.1.0/chunks/2026-10-04-dialog-answers-by-dialog-id/evidence/relay_fixtures.py` — the relayed PreToolUse captures
- derived `fixtures/claude/2.1.287/PermissionRequest.*.json` by `python3 viola-0.1.0/chunks/2026-10-04-dialog-answers-by-dialog-id/evidence/relay_fixtures.py` — the relayed PermissionRequest captures
- derived `tests/snapshots/*.snap` by `cargo insta test` — the pinned decision bodies, created once per new body and then held in check mode

## Open questions
- none — the first pass's three plan-decision questions are answered (the recording: founder ~11:55Z, relayed captures plus a spine-only `viola verify --record`; the frames: the sixth dated gap, founder ~11:15Z; the stamps read: strict-modes here), and the revision's one fork (non-null decisions on the 6-row stamp) is the founder's live ruling of ~14:08Z.
