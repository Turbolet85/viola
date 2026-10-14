# Codebase Research — 2026-10-13-owed-ledger-rows

## Scope
- **Depth:** deep · **Reads:** 16 · **Globs/Greps:** 14
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, 16 Session Additions applied
  (2026-10-10: a live rig that needs the stamped version puts that version's install dir first on its `PATH` and reads
  `claude --version` by path first; 2026-10-07: the tmpfs backing, a stalled-start red reported, never re-run; the
  fake agent's statusline options run an absolute command split on white space, never through a shell; `agents --json`
  lands with its consumer).
- **Platform issues consulted:** none — no runner-only bullet: the one CI run Setup read is green.
- **External inputs:** none — every fact this chunk turns on lives in this repository (the six CARRY blocks cite this
  repository's earlier chunk folders).

## Measured facts
1. **The row set** (`crates/viola-agent-claude/src/ledger.rs@14e6232`): `LedgerRow::ALL: [Self; 18]` at `:42`, last
   `PasteNewline` (`:60`); `check(row, &Probes)` one arm per row (`:760`–`:789`). `LOCAL_COMMANDS: [(&str,
   Option<PostCondition>); 2]` at `:157` (`/clear` → `Some(NewSession)`, `/remote-control` → `None`); `PostCondition`
   holds `NewSession` alone (`:150`). `FRAMING_TURNS: [(&str, &str); 4]` at `:165` (`paste-1`, `paste-2`, `paste-3`,
   `clear-1`). Re-derived: `command grep -n` over the file at HEAD.
2. **`/compact` today, code half** (`src/run/send/classify.rs:27`): `classify` finds no `/compact` in `LOCAL_COMMANDS`
   and returns `Confirms::Prompt`, so `send` waits for a matching `prompt-submitted`. The claim of a local command's
   post-condition is `SendSlot::session_started` (`src/run/send/slot.rs:98`–`:115`): it claims only a `session-start`
   of cause `clear` with an id **different** from the one remembered, for `Confirms::Post(PostCondition::NewSession)`.
   A `/compact` post-condition (cause `compact`, the **same** id, measured at chunk 2026-10-10-viola-revive,
   `evidence/live-revive.md` "Start 2, continued") cannot be served by that arm: it needs a new `PostCondition` member
   and a second claim arm. The CLI half (`/compact` fires no UserPromptSubmit) was read at that chunk from the refused
   send and the `compact` session-start 18.8 s later — a reading, one start, 2.1.287.
3. **Run B and the verify child set** (`src/cmd/verify/typed.rs@14e6232`): `measure` (`:72`) runs four runs in order
   (A untrusted, B trusted, C dialogs, D plan) under the `--model haiku` args; Run B (`:110`) passes no `--settings`
   (only `--plugin-dir`), runs `turn` then `framing_turns` (`:307`), ends by Ctrl-C. Runs C and D pass `SETTINGS_FLAG`
   (`:156`, `:197`). The print probe passes `--no-session-persistence` (`src/cmd/verify.rs:195`); Run B does not, so
   Run B's session is persisted and resumable (inferred from the flag's absence; not read live).
4. **`--resume` homes** (`crates/viola-agent-claude/src/lib.rs`): `is_session_id` `:127` (36 chars, four hyphens),
   `resume_args(id, fork)` `:140`; product callers only `src/cmd/revive.rs` (`preflight` `:165`, graph: this run dir's
   `tree-query-2026-10-13-owed-ledger-rows.json`, query 2). A verify resume child would take Run B's id from Run B's own
   SessionStart capture — a new id source onto a child's argv (security history: the ratified crossing covers a logged
   id or a typed `--id` only).
5. **The statusline shapes** (`crates/viola-agent-claude/src/statusline.rs@14e6232`): `override_document` `:91` (the
   pinned path + ` hook statusline`, plain characters only, Unix only), `shell_argv` `:109` (`["/bin/sh", "-c",
   command]`, Unix only), `reading` `:25` (the `rate_limits` parse). `viola hook statusline --capture <DIR>` captures
   nothing today: `hook` (`src/cmd/hook.rs:96`) sends the `statusline` word to the pass-through only without
   `--capture`, and with it falls to `HookEvent::from_arg`, which takes no `statusline` word → exit 0, nothing filed.
   So any verify capture of a statusline payload is a new capture arm (a widening of the founder-ratified capture
   exception, security history 2026-09-28).
6. **Reading the shell** (chunk 2026-10-10-statusline-pass-through, `evidence/live-statusline.md`): the shell was read
   by a command string carrying `"$0" "${BASH_VERSION:+bash}" "${ZSH_VERSION:+zsh}"`, expanded by whatever shell ran
   it. viola's own override string carries no such word, so a `/bin/sh` probe needs a probe command of its own; the
   flag the CLI hands the shell was not read and a recorder sees no shell argument (that file, "Start 1").
7. **`rate_limits` timing** (same evidence): absent on a session's first statusline payload in both live readings;
   present 4.2 s later (start 1, third payload) and 0.35 s later (start 2, second payload). A probe that reads it after
   Run B's turn reads a session that has had an API response; the CARRY's hypothesis on its cause stays open (a CLI
   behaviour, no live start at research).
8. **Fixture classes** (`schemas/claude-fixture.v1.json:10`): the enum is the nine `REGISTERED_EVENTS`
   (`src/bin/viola-fake-agent.rs:41`). A recorded SessionStart of source `resume` or `compact` is a SessionStart variant
   inside the enum; a statusline payload is not a hook event, so recording it is a new fixture class (the schema, the
   hygiene walk and the fake agent's replay all move). `fixtures/claude/2.1.287/` holds 29 files (`ls | wc -l`), the
   framing variants `paste-1..3`, `clear-1` (×2 events).
9. **The live-start unit** (re-derived from the code, not copied): one `viola verify` round starts `claude` five times
   — `run_logged("version-probe")` is `--version` only (no session), so the five are the print probe
   (`verify.rs:196`) and Runs A–D (`typed.rs:79`–`:84`). A Run B paste adds no start (fact 3). A resume child adds one
   start per round and one spawn pair (`verify-pty-probe`, `typed.rs:41`). A record round and a `run --local-live`
   re-stamp of the tmpfs home are two rounds when both are wanted (a `--record` home may hold no path component named
   `home`; `testing.md` 2026-10-05).
10. **Host CLI** (`ls ~/.local/share/mise/installs/claude/`): `2.1.287 2.1.288 2.1.289 2.1.296` installed; `claude` on
    `PATH` resolves to 2.1.296 (`readlink -f "$(command -v claude)"`). A live round on 2.1.287 names it by path (fact
    9's harness rule). No `claude` was started at research.
11. **`claude agents --json` has no reader** (`command grep -rln 'agents --json\|claude agents' src crates
    --include=*.rs`: no file); `list` spawns none (security and arch histories, 2026-10-11-the-board-viola-list). Its
    consumer is the next markerless entry, "Unwrapped sessions on the board" (`working-route.md:138`). The test plan's
    consumer-first rule (tests history 2026-09-24-fake-agent-and-test-data-fixtures) keeps a fake-agent `agents --json`
    mode out until that consumer lands.
12. **The older-stamp oracle** (`src/run/version_gate.rs:299`, `:305`): two cases, "the fourteen rows of an older
    stamp" and "all but the newline row" (seventeen). Each row this chunk lands adds one more older id set.

## Files inspected
- `crates/viola-agent-claude/src/ledger.rs` (`:30`–`:175`, `:430`–`:445`, `:755`–`:790`) — the row enum, `LOCAL_COMMANDS`, `PostCondition`, `FRAMING_TURNS`, the capture plugin events, `check`
- `crates/viola-agent-claude/src/statusline.rs` (`:1`–`:135`) — `SETTINGS_FLAG`, `reading`, `user_command`, `override_document`, `shell_argv`
- `crates/viola-agent-claude/src/lib.rs` (`:120`–`:190`) — `is_session_id`, `resume_args`
- `src/cmd/verify.rs` (`:123`–`:367`) — `measure`, the print probe, `ProbeDir`, `write_plugin`, `read_captures`, the summary
- `src/cmd/verify/typed.rs` (`:41`–`:220`) — the four runs, Run B's argv, Runs C/D's `--settings`
- `src/cmd/hook.rs` (`:90`–`:130`) — the `statusline` word, the capture arm
- `src/run/send/classify.rs` (`:15`–`:35`), `src/run/send/slot.rs` (`:95`–`:130`), `src/run/send.rs` (`:15`–`:30`) — `/compact`'s classification and the post-condition claim
- `src/run/version_gate.rs` (`:295`–`:310`) — the older-stamp oracle
- `viola-0.1.0/chunks/2026-10-10-viola-revive/evidence/live-revive.md` (full) — the resume and `/compact` readings
- `viola-0.1.0/chunks/2026-10-10-statusline-pass-through/evidence/live-statusline.md` (full), `evidence/live-recorder.sh` (full) — the override, shell and `rate_limits` readings, the recorder
- `viola-0.1.0/chunks/2026-10-13-paste-newline-ledger-row/research.md` (full) — the predecessor's sizing, re-derived here where cited
- `schemas/claude-fixture.v1.json` (`:10`) — the fixture event enum

## Graph impact (rust plane, this run dir's `tree-query-2026-10-13-owed-ledger-rows.json`, two queries)
- **LOCAL_COMMANDS** — refs `ledger.rs:162` and two ledger tests (`:2685`, `:2692`), `src/run/send.rs:22`,
  `src/run/send/classify.rs:28`: a `/compact` entry reaches `send` through `classify` with no other reader.
- **PostCondition / NewSession** — `ledger.rs:157`, `:158`, test `:2687`; `src/run/send/classify.rs:20`;
  `src/run/send/slot.rs:109` (the one claim arm): a new member is matched in `slot.rs` and nowhere else.
- **FRAMING_TURNS** — `ledger.rs:174`, `:727` (the `framing_variants` destructuring, an array pattern of length 4 that
  changes shape with a fifth framing row).
- **PROBE_LOCAL_COMMAND** — `typed.rs:28`, `:332`; the fake agent `:18`, `:558`; ledger tests.
- **resume_args** — `src/cmd/revive.rs:165` and its lib unit test (`lib.rs:675`, `:679`).
- **shell_argv** — `src/cmd/hook/statusline.rs:145` and two tests; **override_document** — `src/cmd/run.rs:516`;
  **reading** — `src/cmd/hook/statusline.rs:135` and the budget tests.
- **framing_variants** — `src/cmd/verify.rs:459` (`named_payloads`) + one ledger test.
- **check_rows** — `src/cmd/verify.rs:246` (`measure`).
- (Line numbers are the trace's 0-indexed values + 1.)

## Patterns detected
- **A framing row is one compiled text + one stem + one `check` arm** (`ledger.rs:165`, `:785`–`:789`), replayed by
  the fake agent through `framing_stem` with no fake-agent change (graph: `framing_stem` callers in `Agent::submit` /
  `Agent::end_turn`).
- **A local-command row is a `LOCAL_COMMANDS` entry + a `PostCondition` + a `slot.rs` claim arm + its framing row**
  (`/clear`: `ledger.rs:158`, `slot.rs:98`–`:115`, `clear-1`).
- **A count move pins the new count as literals in six files** (§Sweeps) plus one older-set oracle (fact 12).
- **A whole verify run is a founder-ratified widening** (Runs C and D, chunk 2026-10-05-dialog-rows-and-re-probe; the
  arch and security histories).

## Conventions to follow
- **Compiled probe texts live in `viola-agent-claude`'s pure `ledger`** (`ledger.rs:110`–`:145`); no runtime text.
- **A recorded variant is copied alone into the stamped set**, never a whole re-recorded set (chunk
  2026-10-06-local-command-and-paste-framing-rows's record step).
- **Live readings are the chunk's own evidence, never a test layer** (tests history 2026-10-08-first-live-test-and-self-drive).

## New files to create
- `schemas/claude-statusline-fixture.v1.json` — the statusline payload fixture schema
- `fixtures/claude/2.1.287/StatusLine.default-1.json` — recorded live, copied alone from the record round
- `fixtures/claude/2.1.287/StatusLine.rate-limits-1.json` — recorded live, copied alone from the record round
- `viola-0.1.0/chunks/2026-10-13-owed-ledger-rows/evidence/` — the live rounds' record

## Files to modify
- `crates/viola-agent-claude/src/ledger.rs` — the three statusline rows, the relied set, their arms, file names, variants and unit cases
- `crates/viola-agent-claude/src/statusline.rs` — the probe document, its constants, the wrapped-here reading made public
- `src/cmd/hook.rs` — the hidden `--shell` option and the statusline capture arm
- `src/cmd/verify/typed.rs` — Run B's one probe `--settings`
- `src/cmd/verify.rs` — the statusline probe dir and settings file, the capture read, `check_rows` over the relied set, the recorded variants, step-line cases
- `src/run/version_gate.rs` — the verdict over the relied set; the older-stamp oracle gains the eighteen-id set
- `src/bin/viola-fake-agent.rs` — the `--framing` statusline replay
- `tests/hook_fail_open.rs` — the statusline capture arm's cases
- `tests/cli_verify.rs` — `STEPS_PASS`, `ROW_IDS`, the summaries per OS
- `tests/contract_ledger_probes.rs` — `ROW_IDS` and the summary count per OS
- `tests/contract_fixture_hygiene.rs` — the `StatusLine` files validated against their own schema
- `tests/contract_fake_agent_drift.rs` — the two variants replayed byte for byte
- `tests/support/home.rs` — the dialog-less summary per OS
- `tests/support/verify.rs` — synthetic `StatusLine` variants in `framing_set`
- `crates/viola-e2e/src/harness/run.rs` — `LEDGER_ROWS` and `LIVE_PASS` per OS

## Sweeps
- `18\]|; 18\]|18 pass|\[&str; 18\]|eighteen|/18\b` over `*.rs` (`command grep -rlE` then `command grep -cE` per
  file, `target/` excluded): 80 hits in 6 files — `tests/cli_verify.rs` 42 · `crates/viola-e2e/src/harness/run.rs` 24
  · `src/cmd/verify.rs` 8 · `crates/viola-agent-claude/src/ledger.rs` 3 · `tests/contract_ledger_probes.rs` 2 ·
  `src/run/version_gate.rs` 1. All six are on the modify list.
- `14 pass|4 fail` over `tests src crates` `*.rs`: 3 hits — `tests/support/home.rs:376`, `:386`, `tests/cli_verify.rs:1040`
  (the dialog-less summary, a shape-pinned reader; both files on the modify list).
- `six (spawn )?pairs` over `src crates tests` `*.rs`: 1 hit — `tests/cli_verify.rs:399` (moves only if a run is added).

## Open questions
- none — the three plan-decision questions research raised were answered at P4 by the founder's own word
  (`inputs#I1`): the statusline trio is built here; the `agents --json` row moves to its consumer; Run B gains the
  probe `--settings`; the cap is ten starts. The lists above are the decided write set.
