# Codebase Research — 2026-10-07-a-send-ending-in-a-newline-is-confirmed

_Rewritten at the plan's revision, 2026-10-07T14:43Z, after `/andromeda-implement` stopped on the take-up plan
(inputs#I7). What the stopped run measured is under "Measured at implement"; the two lists are the same write
set, most of it already in the tree._

## Scope
- **Depth:** deep · **Reads:** 24 at the take-up, 6 more at the revision · **Globs/Greps:** 31, 4 more
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read whole, its 9 Session Additions
  included; 2 applied (a root test file is selected with `binary(<stem>)`; the fake agent receipts the submitting
  Enter as its own `key` line). No live leg in this chunk.
- **Platform issues consulted:** query `cargo-llvm-cov "invalid instrumentation profile data (file header is
  corrupt)" profraw process killed truncated` → fetched `rust-lang/rust` `src/doc/rustc/src/instrument-coverage.md`:
  "the counter values are written to a `profraw` file at program termination", and its `%c` mode exists so that
  "if the instrumented program crashes, or is killed by a signal, perfect coverage information can still be
  recovered". The default mode gives no such guarantee: a process killed while it writes leaves what it had
  written. No runner-image or cargo-llvm-cov defect is involved.
- **External inputs:** `inputs#I1` — the operator's directive at the take-up (zero live starts; the red folded with
  its own acceptance and witness; a founder remedy card answered provisionally and listed; a widening held).
  `inputs#I2` — the P4 card's three answers (strip before the paste, every trailing LF, provisional for the
  founder; the red closed in this chunk with its limit stated, no new member in a CI upload).
  `inputs#I3` — the operator's review at P5 (the two filter entries assert their passed counts; the census runs
  on a host four other builders share, so its wall time is stated and each run is announced).
  `inputs#I4` — the operator's word with the implement invocation (the operator pass is the implementer's;
  `run_attempt` 1; a notice before each census run; a no-assertion red is read against the backing and the host
  record first; the remedy stays worded PROVISIONAL).
  `inputs#I5` — the step 7 card's answer (an uninstrumented host child; the deviation recorded; a Windows red
  fixed by cause in its own commit; first-attempt green binds the final sha).
  `inputs#I6` — the census card's answer (the host child stays; stop with nothing pushed; no push with a gate
  entry red by its own letter).
  `inputs#I7` — the revision directive (step 7 as built; one whole profile a run; steps 6 to 8 and entry 16 to
  what was measured; the red side stands as recorded, no rebuild; nothing else moves; PROVISIONAL stays).

## Files inspected
- `src/run/send.rs` (1-132, 203-245, 276-425, 820-934) — the claim, the hook tap, the `send` method in its refusal
  order, the unit harness (`confirmed_with`, `occupy`, `prompt`, `FixedClock`, `Pastes`).
- `src/bin/viola-fake-agent.rs` (214-246, 440-531) — `submit` and the scripted steps.
- `crates/viola-agent-claude/src/hook.rs` (index of its surface; 599-629 by grep) — `normalise`, `prompt_text`,
  the two newline cases.
- `crates/viola-agent-claude/src/ledger.rs` (surface index) — `LOCAL_COMMANDS`, `PROBE_PROMPT`, the probe pastes.
- `tests/cli_send.rs` (23-66, 176-192, 278-306, test index) — the boot helpers and the `send_window_` cases.
- `tests/support/` (surface index) — `Wrapper::boot`, `release`, `receipt`, `fake::wait_for`.
- `fixtures/fake-scripts/*.json`, `schemas/fake-script.v1.json`, `fixtures/claude/2.1.287/` (listing;
  `UserPromptSubmit.default.json`'s keys and prompt).
- `src/cmd/run.rs` (900-997) — `start_opens_the_scenario_one_spans_under_run_start`.
- `src/run/version_gate.rs` (50-70, 285-345), `crates/viola-pty/src/lib.rs` (822-842, 890-912, 1130-1210),
  `crates/viola-state/src/liveness.rs` (104-124), `src/panic_frames.rs` (108-124) — every other test that runs the
  test binary as a child.
- `.github/workflows/ci.yml` (step index 53-193, uploads 157-189), `.config/nextest.toml` (override index).
- `.andromeda/architecture.md` (`:49`, `:70`, `:81`, `:292`), `.andromeda/test-plan.md` (`:1063`, `:1077`,
  `:1078`, `:1207`).
- `viola-0.1.0/chunks/2026-10-04-running-turn-refusal/evidence/watch-profraw.md` (whole).
- ci#37627485806 attempt 1: the failed log, the per-step conclusions, and the three artifacts
  `harness-ubuntu-latest` (id 11485600497), `diag-ubuntu-latest` (11484568161), `junit-ubuntu-latest`
  (11484488518), fetched to the session scratchpad and read there.
- At the revision: `src/run/mod.rs` (90-131) — `child_launch`, the plugin flag ahead of the user's arguments;
  `src/run/version_gate.rs` (40-117) — the bounded probe and `program --version`; `src/cmd/run.rs` (920-945) — the
  test as the stopped run left it; `scripts/profraw-census.sh` (whole, 84 lines); the chunk's `evidence/` (three
  files and `guards/read-attempt1.py`) and `scope-record.md`; the `portable-pty` 0.8.1 source, `ChildKiller for
  std::process::Child` (the Unix kill sends SIGHUP by pid, then waits).

## Graph impact (plane `rust`; trace `tree-query-2026-10-07-a-send-ending-in-a-newline-is-confirmed.json`, in the take-up's run dir)
- **`SendSlot::claim`** — 1 caller: `append_hook_event` @ `src/run/send.rs:219`. The claim's rule changes in one
  place; its signature need not.
- **`append_hook_event`** — 1 production caller, `dispatch` @ `src/cmd/run.rs:117`, and 17 call sites in
  `send.rs`'s own unit cases. No signature change is needed for either remedy shape.
- **`SendSlot::settle`** — 1 caller, `append_hook_event` @ `src/run/send.rs:242`.
- **Crate edges** — `viola → viola-agent-claude` exists (`send.rs:16-17` already reads `ledger::LOCAL_COMMANDS`
  and `screen::CONFIRM_WINDOW_FALLBACK`). A rule defined in the agent crate and read by `send.rs` adds no edge.
- No new graph query at the revision: the revised steps change no symbol's signature and add no caller.

## Patterns detected
- **The claim is one exact comparison** (`src/run/send.rs:120-132` at the take-up): `f.text == text`, only for a
  send that waits for a prompt (`Confirms::Prompt`) and is still `Match::Waiting`.
- **The tap** (`src/run/send.rs:210-245` at the take-up): a claim relabels the line `origin:"driver"`; the turn is
  marked for a prompt of any origin; a prompt still filed `human` moves the wheel before its line is appended; a
  claimed line settles the send with its `ts`.
- **`send`'s order** (`src/run/send.rs:303` on, as built): `validate_paste_text` first, then the typed text taken
  once (`:321`), the wheel, the local-command classification, one in flight, the gate, the second wheel and turn
  reads, `send-issued`, the paste, then the window.
- **Claude knowledge compiled in the agent crate and read by `send`** (`src/run/send.rs:16-17`):
  `LOCAL_COMMANDS` is matched by exact equality; the precedent for where a newline rule lives.
- **The fake agent echoes a paste** (`src/bin/viola-fake-agent.rs:452-467`): with no `--framing` variant for the
  text, `submit` fires `UserPromptSubmit` variant `default` with `prompt` set to the typed text, last newline
  included. Under the fake agent a text ending in a newline was confirmed before the build too.
- **The recorded default prompt** is `viola verify probe: reply with the single word ok`
  (`fixtures/claude/2.1.287/UserPromptSubmit.default.json`, key `prompt`). It ends in no newline.
- **The wrapper's plugin flag comes first in a child's arguments** (`src/run/mod.rs:124-128`): `child_launch`
  builds `--plugin-dir <dir>` and then the user's words. A child that is a libtest binary refuses that flag.
- **The version probe runs the same program** (`src/run/version_gate.rs:106-117`): `program --version`, no user
  arguments, stdin null, bounded; any first line that is not `X.Y.Z (Claude Code)` reads as no version
  (`crates/viola-agent-claude/src/ledger.rs:386-395`).

## The folded red, read against ci#37627485806 attempt 1
- **Still as recorded.** Job `test (ubuntu-latest)`: step 9 `Coverage and doctest (sh shim)` failure, step 27
  `Gate verdict` failure; steps 10-18 skipped, `Browser suite (sh shim)` among them, which is why the gate also
  lists `suite-missing` and `artifact-missing` for `playwright` (re-derived at the take-up: `gh run view
  37627485806 --attempt 1 --json jobs`; re-derived at implement: `evidence/ci-attempt-1.md`).
- **The profile is not on the run.** `harness-ubuntu-latest` holds 10 members; 0 `.profraw` (re-derived twice:
  `unzip -l` of the artifact, `grep -c profraw` = 0). The upload takes `target/agent-run/` (`ci.yml:163-172`); the
  profile sat in `target/llvm-cov-target/`. The session-learnings method reads the refused file's own counters,
  so it cannot run on this attempt.
- **What the run does hold.** The profile's name carries its writer's pid, 10799.
  - `diag-ubuntu-latest`: 222 lines carry a `pid`, all `process-start` of `run` or `cli` (re-derived:
    `evidence/guards/read-attempt1.py` over its 217 files). The nearest below is 10401 at 13:21:39.511Z, the
    nearest above 11040 at 13:21:41.038Z; no line of any of the 127 test homes falls between 13:21:39.514Z and
    13:21:41.038Z.
  - `junit-ubuntu-latest`: 279 cases overlap that window, 277 of suite `viola::bin/viola` and 2 of
    `viola::tui_wheel`. `cmd::run::tests::start_opens_the_scenario_one_spans_under_run_start` ran
    13:21:39.913Z to 13:21:40.706Z. Its home is a tempdir outside `target/e2e-home`, so it leaves no line in the
    diagnostics artifact.
- **Reproduced at HEAD, at the take-up.** The instrumented unit-test binary of the last pre-push ran that one
  test with `LLVM_PROFILE_FILE` pointed at the scratchpad: 40 runs in sequence each left 2 profiles of 128 264 B;
  of 1 200 runs at 48 workers, 1 198 left 2 and 2 left 3, one third profile being 69 632 B, which `llvm-profdata
  show` refuses with attempt 1's line and `merge` with `error: no profile can be merged`.
- **The cause, as far as it is established.** A test kills an instrumented child that exits by itself, and the
  kill sometimes lands inside the child's exit-time profile write. Corrected at implement in one respect: the
  child never ran its `--list`. It was started as `<test binary> --plugin-dir <dir> --list`, libtest refused the
  flag and it exited 101 at once (below). The mechanism is the one stated.
- **What is not established.** That pid 10799 was that child. The file is gone; the runner's binary signature
  (`3177174534174374433`) is not the local one for the same binary (`978539276648341693`). The second `tui_wheel`
  case was also alive in the window.
- **The class.** Every other test that runs the test binary as a child either waits for its exit before any kill
  (`version_gate.rs:296-315`, `liveness.rs:110-116`, `viola-pty` `lib.rs:1072-1126`) or kills a child that blocks
  and never exits by itself (`viola-pty` `lib.rs:1131-1153`, `:1165-1207`, `:1213`). Those `viola-pty` children
  are spawned through `viola_pty::spawn` directly, with arguments the test gives whole; no wrapper flag precedes
  them. `"--list"` as a child argument: 3 hits at the take-up · 1 changed (`src/cmd/run.rs`, now 0 there:
  `grep -c '"--list"' src/cmd/run.rs`) · 2 no-change (they wait). Sweep: `grep -rn '"--list"' src crates tests
  --include=*.rs`.
- **Two earlier runs with the same message** on the same leg, ci#36529038462 and ci#36481260151, stand in
  `test-plan.md:1207` as "recorded, not established". Whether they had this cause is not measurable now.

## Measured at implement (2026-10-07T14:20Z to 14:36Z; records in the chunk's `evidence/`)
- **The send half is built and read.** Before step 2, six of the seven send tests failed on the pasted value and
  both cross-process cases failed at the receipt's bytes; after it the unit filter read 16 passed and the
  integration filter 2 passed; with the rule neutralised inside `typed_text` the filter read 10 failed, and 16
  passed restored (`evidence/newline-red-green.md`).
- **A child started through `start` cannot be the test binary at a child-entry test.**
  `<unit-test binary> --plugin-dir /nonexistent --list` → stderr `error: Unrecognized option: 'plugin-dir'`,
  stdout 0 bytes, exit 101; the same with `--exact <a test name> --nocapture`. libtest parses its arguments
  before any test body runs. The take-up's sentence that a child's arguments are the test's to give missed the
  flag `child_launch` puts ahead of them.
- **The census on the untouched test, under the two-a-run rule:** `bash scripts/profraw-census.sh 4800 48` on the
  instrumented binary of `9f2bebe` → exit 1, wall 136.6 s, `runs 4800 · passed 4800 · profiles 9603 · third 3 ·
  short 3`. 4 797 runs left two profiles of 128 264 B; 3 left a third, of 0 B, 73 728 B and 124 288 B (re-derived:
  the run's per-worker tallies, `sort | uniq -c`). This reading stands as the census's red side; the untouched
  test is not rebuilt (inputs#I7).
- **The fix as built** (inputs#I5): the test's program is `whoami` (`src/cmd/run.rs:935`). `whoami --plugin-dir
  /x` → `unrecognized option`, exit 1 (measured on this host, GNU coreutils). It is not instrumented.
- **The fixed test leaves one whole profile a run, not two.** After `pre-push` rebuilt the instrumented binary
  (coverage 1712/1712), `bash scripts/profraw-census.sh 48 1` → exit 1, `runs 48 · passed 48 · profiles 48 ·
  third 0 · short 0`; every run left one profile of 129 512 B. The equality the revised census needs: with an
  uninstrumented program, the test process is the only instrumented process a run starts, because the wrapper's
  child and the version probe's child are both that program (`src/cmd/run.rs:935`,
  `src/run/version_gate.rs:117`). Verified at 48 runs one at a time; not yet measured under load.
- **The script as it stands** holds the two-a-run rule in two places (`scripts/profraw-census.sh:73`, a run has a
  third when it left more than two; `:80`, the profiles are twice the runs), so it exits 1 on the fixed test.
- **Gate entries 1 to 15 of the take-up plan read green** on the tree the stopped run left (the gate tool's
  listing, 14:33Z to 14:35Z). Entry 16 and the operator entries were not run.

## Conventions to follow
- **Unit cases with literal oracles and an injected clock** (`src/run/send.rs`, `confirmed_as` and the cases
  beside it): a real `send` on a scoped thread, the hook's line fed through `append_hook_event`.
- **Root end-to-end cases** boot through `boot(script, extra)` (`tests/cli_send.rs:29-44`), read records after a
  pre-send offset and the fake agent's receipt, and end a turn with a scripted `Stop`.
- **A committed turn script** is a file under `fixtures/fake-scripts/`, checked by
  `tests/contract_fixture_hygiene.rs:432` through a `*.json` glob.
- **A case that waits out the 10 s window** carries `send_window_` in its name (`.config/nextest.toml`).
- **A send text carries the canary** (`tests/cli_send.rs:23`), except where the text must equal a compiled or
  recorded one.
- **A repository script** is run as `bash scripts/<name>.sh`, opens with a comment naming the plan section it
  serves, sets `-euo pipefail`, and prints no host path (`scripts/g2-zero-panics.sh`).

## New files to create
- `scripts/profraw-census.sh` — the counted witness: one test of the instrumented root-bin test binary, run many times under load, its raw profiles counted and sized
- `viola-0.1.0/chunks/2026-10-07-a-send-ending-in-a-newline-is-confirmed/evidence/` — the red-first readings, the census readings and the record of attempt 1

## Files to modify
- `src/run/send.rs` — the typed text taken once after validation, and its unit cases
- `crates/viola-agent-claude/src/hook.rs` — the rule for a sent text's trailing newlines, defined once
- `tests/cli_send.rs` — the cross-process case for a text ending in newlines
- `src/cmd/run.rs` — the test whose child must write no coverage profile

## Open questions
- none. The step 7 fork and the census count were answered on two cards at implement (`inputs#I5`,
  `inputs#I6`) and the revision's direction is `inputs#I7`.
