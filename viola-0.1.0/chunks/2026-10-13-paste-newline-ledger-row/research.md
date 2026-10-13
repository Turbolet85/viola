# Codebase Research — 2026-10-13-paste-newline-ledger-row

## Scope
- **Depth:** deep · **Reads:** 14 · **Globs/Greps:** 13
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, 16 Session Additions applied
  (the 2026-10-10 entry: a live rig that needs the stamped version puts that version's install dir first on its
  `PATH`; the 2026-10-07 entries: the tmpfs backing, a stalled-start red reported, never re-run; the `--local-live`
  refusal under `CI`). `.claude/rules/testing.md` — the two entries matched by `re-stamp|live start|--record`
  (2026-10-05: never give `--record` a home with a path component named `home`; 2026-10-10: the fake agent first on
  `PATH` for a verb that looks its program up).
- **Platform issues consulted:** none — no runner-only bullet: the one CI run Setup read is green at P3.
- **External inputs:** `inputs#I1` — the operator's directive at take-up: the fifth Run B paste and the live-start cap
  are the founder's own P4 forks; no live start beyond his cap; no mutation run, witness run or workflow dispatch;
  record times from `date -u`.

## Measured facts
1. **CI at HEAD** (`ci.py conclusion --sha 19483111dbe9…`, this run dir's trail, re-read once at P3): `19483111dbe9`
   **green**, checks 15/15, wall 576 s, `ci#38463266088` push completed/success. Setup's `in progress` is closed;
   nothing to fold.
2. **The row set** (`crates/viola-agent-claude/src/ledger.rs@19483111`): `LedgerRow::ALL: [Self; 17]` at `:41`;
   `check(row, &Probes)` at `:745`, one arm per row; the framing rows read Run B's captures through `pasted(trusted,
   text)` (`:677`: the index of the first UserPromptSubmit whose `prompt_text` equals `text`). Run B's added pastes
   are one compiled table, `FRAMING_TURNS: [(&str, &str); 3]` at `:159` (`PROBE_LONG_PASTE` → `paste-1`,
   `PROBE_TAG_PASTE` → `paste-2`, `PROBE_LOCAL_COMMAND` → `clear-1`); `framing_stem` (`:166`) and
   `framing_variants` (`:714`) both read it. `LOCAL_COMMANDS` at `:151` holds `/clear` (post-condition
   `NewSession`) and `/remote-control` (none); `/compact` is not on it.
3. **The step counter derives from `ALL`** (`src/cmd/verify.rs:389`, `check_rows`: `total = LedgerRow::ALL.len()`,
   `step_line` formats `[{n:02}/{total:02}]`): a row added to `ALL` moves `MM` and the order with no printer change.
   The literals that pin seventeen are test-side (§Sweeps).
4. **Run B's driver** (`src/cmd/verify/typed.rs@19483111`): `trusted_run` (`:109`) runs `turn` (the probe prompt)
   then `framing_turns` (`:306`), which pastes the long and the tag-like texts from a literal list `[PROBE_LONG_PASTE,
   PROBE_TAG_PASTE]` (`:309`), each only when `input_box_up(rows)` holds, waits `added_turn_ended` (one more
   UserPromptSubmit and one more Stop), then `box_wait`; then the local command, waited by `local_command_answered`.
   Every paste is `Run::paste_text` (`:691`): nothing once the stop is set, else one bracketed write through
   `PasteHandle`. A fifth paste is one more element in the `:309` list and one more `FRAMING_TURNS` row, under the
   same guards and the same stop rule (graph: `framing_turns` has the one caller `trusted_run` at `typed.rs:138`).
5. **The fake agent replays a compiled paste by its stem** (`src/bin/viola-fake-agent.rs`, `Agent::submit` `:555`):
   under `--framing` a submitted text equal to a `FRAMING_TURNS` text fires that stem's recorded
   `UserPromptSubmit.<stem>.json` unchanged; without `--framing` it fires `default` with `prompt` = the typed text.
   Its `Input` (`:681`) keeps every byte inside a bracketed paste as prompt content, an LF included, and only a CR
   outside a paste submits (case at `:1096`, `a\rb` inside a paste stays one prompt). So: with a recorded
   `paste-3` the CI row reads the CLI's recorded bytes; without one, `--framing` fires nothing for the text and the
   row fails, as a set without a variant must (verification-harness.md, the framing replay).
6. **Recording** (`src/cmd/verify.rs:423` `record`, `:448` `named_payloads`): the framing variants are named by
   `framing_variants`, scrubbed, checked `unclean`, written only at 0 failing rows. The precedent round (chunk
   2026-10-06-local-command-and-paste-framing-rows, its plan's record entry): `viola --home "$h/vhome" verify
   --record "$h/out" -- <2.1.287 install>/claude`, then only the new variant files copied into
   `fixtures/claude/2.1.287/` — the rest of the committed set is left byte-identical.
7. **The stamped set and the host CLI**: `fixtures/claude/2.1.287/` (28 files) is the one `STAMPED` set
   (`tests/contract_ledger_probes.rs:39`), `2.1.283` / `2.1.288` drift-only. On the host, `claude` on `PATH` resolves to
   `~/.local/share/mise/installs/claude/2.1.296/claude`; `~/.local/share/mise/installs/claude/2.1.287/` is installed
   (listing of `~/.local/share/mise/installs/claude/`: `2.1.287 2.1.288 2.1.289 2.1.296`). A live round on 2.1.287 names
   that program by path (the harness rule of 2026-10-10). No `claude` was started at research.
8. **The live-start unit**: one `viola verify` round = five `claude` starts — the print probe, Runs A, B, C, D
   (re-derived: chunk plans 2026-10-07-send-waits-out-the-paste-hint `:323` and 2026-10-08-first-live-test-and-self-drive
   `:517`, "5 live starts (print, Run A, Run B, Run C, Run D)"). A fifth Run B paste adds no start and no spawn pair
   (the six pairs are per child, obs history 2026-10-05-dialog-rows-and-re-probe). A `run --local-live` firing is
   one more round (five). A `--record` round's `--home` must not hold a component named `home`, so a `--local-live`
   home (`target/e2e-home/viola-live-<pid>/home`) cannot be the record round's home: recording and re-stamping that
   home are two rounds when both are wanted.
9. **What each other shape needs** (sizing; read from the code and the CARRY text, no live start):
   - `--resume <id>` (scope §2): a sixth child that resumes Run B's session (one more start per round, one more
     spawn pair, a new probe dir), its id through `viola_agent_claude::resume_args`; a child-set widening.
   - `/compact` (scope §3): a further Run B paste, a live compaction turn (API time beyond a probe reply), a new
     `LOCAL_COMMANDS` entry with a post-condition kind `session-start` cause `compact` with the SAME id (the
     `PostCondition` enum holds `NewSession` alone, `ledger.rs:144`), and the send-side claim in
     `src/run/send/classify.rs` / `src/run/send/slot.rs` (arch history 2026-10-13-epoch-4-cleanup).
   - the settings override, `/bin/sh` and `rate_limits` (scope §4, §5): a verify child started with a `statusLine`
     in a `--settings` document whose command is a capture the probe owns; reading which shell ran needs a recorder
     (that chunk's `evidence/live-recorder.sh`); Runs C and D already pass one `--settings`, so "which of two wins"
     is a measurement of its own. Every one is a new child shape under the founder-ratified set.
   - `claude agents --json` (scope §6): one more CLI start beside a live run, with no product reader yet ("Unwrapped
     sessions on the board" owns the reader, api.md).
10. **v1-34 cannot be claimed by this chunk** (`matrix.py show --id v1-34`): its requirement needs every relied-on
    behaviour rowed, and the harness-prefix and R8 identity-floor rows stay owed with no entry (the founder,
    2026-10-07T09:43Z). No other unclaimed cap names a ledger row (P4 reads each candidate).
11. **CARRY 2's sites at HEAD** (`git diff --stat 632f6a7 HEAD` over the two files: both changed since, 414 / 69
    lines): `Agent::replay_dialogs` `src/bin/viola-fake-agent.rs:446` with both `!` still on `:450`
    (`if !has("PreToolUse") && !has("PermissionRequest")`); the `"--screens"` arm of `Opts::parse` at `:107`;
    `ProbeDir::typed` at `src/cmd/verify.rs:318` (`self.dir.join("typed")`). Each mutant's effect is "measured at
    chunk 2026-10-09-epoch-3-cleanup-ii" (its `evidence/survivors.md`), not re-run (no mutation run, `inputs#I1`).
    The fake agent's tests are inline in `src/bin/viola-fake-agent.rs`; `ProbeDir` has none naming `typed`.
12. **CARRY 3's site at HEAD**: `replace_private_retrying` `crates/viola-state/src/fs.rs:281`, `attempts += 1` at
    `:295`; the cases `replace_private_retrying_tries_exactly_the_attempts_while_retry_holds` (`:442`) and
    `replace_private_retrying_returns_at_once_when_retry_refuses` (`:466`) stand. The Windows grading is a runner-only
    reading (run 38036448183, job `mutants (viola-state)`, 10 s each); its per-mutant log was not kept, so it is not
    closable on this host, and a re-reading needs a dispatch the operator's direction excludes.

## Files inspected
- `crates/viola-agent-claude/src/ledger.rs` (`:1`–`:173`, `:677`–`:799`) — the row enum, ids, words, compiled texts, `LOCAL_COMMANDS`, `FRAMING_TURNS`, `pasted`, `clear_start`, `framing_variants`, `check`
- `src/cmd/verify/typed.rs` (`:60`–`:359`, `:620`–`:780`) — `measure`, Runs A–D, `framing_turns`, `Run::paste_text`, the stop
- `src/cmd/verify.rs` (`:316`–`:322`, `:370`–`:479`, `:256`–`:266`) — `ProbeDir::typed`, `check_rows`, `step_line`, `record`, `named_payloads`, the `stamped` line
- `src/bin/viola-fake-agent.rs` (`:446`–`:456`, `:540`–`:620`, `:676`–`:725`, `:928`–`:1027`) — `replay_dialogs`, `submit`, `end_turn`, `Input`, the `Opts::parse` cases
- `crates/viola-pty/src/pump.rs` (`:60`–`:104`) — `PasteHandle::paste` writes the text's bytes unchanged between `ESC[200~` and `ESC[201~`, then CR
- `src/run/version_gate.rs` (`:255`–`:315`) — `stamps_verdict_needs_the_dialog_rows`
- `tests/support/verify.rs` (`:36`–`:105`) — `framing_set`, the synthetic variants
- `tests/contract_ledger_probes.rs` (`:1`–`:60`) — `ROW_IDS`, `STAMPED`, `DRIFT_ONLY`
- `crates/viola-e2e/src/harness/run.rs` (`:66`–`:76`) — `Suite`'s field order
- `crates/viola-state/src/fs.rs` (`:281`–`:298`) — `replace_private_retrying`
- `viola-0.1.0/chunks/2026-10-06-local-command-and-paste-framing-rows/plan.md` (`:300`–`:434`) and its `report.md` (`:325`–`:345`) — the precedent record round and its process census (no host harm recorded)

## Graph impact (rust plane, this run dir's `tree-query-2026-10-13-paste-newline-ledger-row.json`, eight queries)
- **framing_stem** — 9 call sites: `ledger.rs` tests `framing_texts_are_fixed_ascii_each_with_its_stem` (`:2573`–`:2577`);
  the fake agent's import (`viola-fake-agent.rs:18`), `Agent::submit` (`:557`, `:558`) and `Agent::end_turn` (`:598`,
  `:603`) — a new `FRAMING_TURNS` row reaches the fake agent through this function with no fake-agent code change.
- **framing_variants** — `cmd::verify::named_payloads` (`src/cmd/verify.rs:459`) + one ledger test (`:2512`–`:2546`):
  the recording names a new stem's capture with no `verify.rs` change once `framing_variants` returns it.
- **FRAMING_TURNS** — 2 refs, both in `ledger.rs` (`:167`, `:715`); the destructuring at `:715`
  (`let [(long, …), (tag, …), (_, clear_stem)] = FRAMING_TURNS;`) is an array pattern of length 3 and changes shape
  with a fourth row.
- **framing_turns** (typed.rs) — 1 caller, `trusted_run` (`typed.rs:138`).
- **LedgerRow::ALL** — refs at `ledger.rs:943`, `:1212`, `:1235`, `:1503`, `:1637`, `:1966`; `src/cmd/verify.rs:390`, `:391`;
  `src/run/version_gate.rs:205`.
- **ledger::check** — the ledger's own unit tests + `cmd::verify::check_step` (`src/cmd/verify.rs:400`).
- **LOCAL_COMMANDS** — `ledger.rs:156`, two ledger tests, `src/run/send.rs:22`, `src/run/send/classify.rs:28` (the
  `/compact` touchpoints, if that shape lands).
- **crate edges** — `viola` → `viola-agent-claude` → `viola-core` only: the ledger change has no cross-crate reader but
  the root bin.

## Patterns detected
- **A framing row is one compiled text + one stem + one `check` arm** (`ledger.rs:159`, `:773`–`:775`): the long and
  tag rows pass when `pasted(trusted, TEXT).is_some()`.
- **Three named unit cases per framing row** (`ledger.rs` `check_framing_rows_pass_on_the_recorded_shape`,
  `check_paste_rows_fail_without_a_matching_capture`, `check_local_command_clear_fails_on_each_broken_post_condition`).
- **A count move pins the new count as literals in four test sites** (`tests/cli_verify.rs` `STEPS_PASS` / `ROW_IDS`,
  `tests/contract_ledger_probes.rs` `ROW_IDS`, `crates/viola-e2e/src/harness/run.rs` `LEDGER_ROWS` + `LIVE_PASS`,
  `src/cmd/verify.rs` tests) and one older-set oracle (`src/run/version_gate.rs`, a stamp of the older ids reads
  unverified).
- **Survivor disposal by hand-applied mutant** (chunks 2026-10-13-epoch-4-survivors / -ii): a nextest case in the
  owning package, shown red under the hand-applied mutant and green restored; no mutation run.

## Conventions to follow
- **Paste texts are synthetic ASCII, compiled in `viola-agent-claude`** (`ledger.rs:110`–`:140`); the test
  `framing_texts_are_fixed_ascii_each_with_its_stem` (`:2570`) pins them.
- **A recorded variant is copied alone into the stamped set** (fact 6), never a whole re-recorded set.
- **Live readings are the chunk's own evidence, never a test layer** (tests history 2026-10-08-first-live-test-and-self-drive).

## New files to create
- `fixtures/claude/2.1.287/UserPromptSubmit.paste-3.json` — the recorded LF paste prompt, copied alone from the record round (the fifth paste, the founder's answer, inputs#I2)
- `viola-0.1.0/chunks/2026-10-13-paste-newline-ledger-row/evidence/` — the live round's records, the hand-applied mutant readings

## Files to modify
- `crates/viola-agent-claude/src/ledger.rs` — the eighteenth row, its words, its check arm, the compiled LF text, `FRAMING_TURNS`, their unit cases
- `src/cmd/verify/typed.rs` — Run B's added-paste list gains the LF text
- `src/cmd/verify.rs` — its step-line test literals move to eighteen; a case killing `ProbeDir::typed` replaced by a default
- `src/bin/viola-fake-agent.rs` — cases killing the two `replay_dialogs` negations and the `--screens` arm deletion
- `src/run/version_gate.rs` — the older-set oracle gains the seventeen-id stamp reading unverified
- `tests/cli_verify.rs` — `STEPS_PASS`, `ROW_IDS`, the summaries and the stop table's receipt counts at eighteen
- `tests/contract_ledger_probes.rs` — `ROW_IDS` and the summary count at eighteen
- `tests/support/verify.rs` — the no-framing helper's prose and receipt count, if the extra paste moves them
- `crates/viola-e2e/src/harness/run.rs` — `LEDGER_ROWS` and `LIVE_PASS` at eighteen

## Sweeps
- `/17\]|; 17\]|17 pass|\[&str; 17\]|seventeen` over `*.rs` (`command grep -cE` per file, the six files the
  repo-wide `command grep -rnE … --include=*.rs` listed): 75 hits — `tests/cli_verify.rs` 40 · `crates/viola-e2e/src/harness/run.rs`
  22 · `src/cmd/verify.rs` 7 · `crates/viola-agent-claude/src/ledger.rs` 3 · `tests/contract_ledger_probes.rs` 2 ·
  `src/run/version_gate.rs` 1; all six files are on the modify list.
- `three (framing|added) pastes|four pastes|Run B's three` over `src crates tests` `*.rs`: 5 hits — `typed.rs:108`, `:301`,
  `ledger.rs:158`, `tests/support/verify.rs:420`, `tests/cli_verify.rs:1012`; each file is on the modify list (prose
  moves with the paste count on the fifth-paste branch; no change on the others).

## Open questions
- Does the founder rule a fifth Run B paste (R-L29), the LF folded into an existing paste, or no paste? → blocks:
  plan-decision (the row's existence and the fixture file on the two lists above).
- What live-start cap does the founder give this chunk? → blocks: plan-decision (whether the record round runs; with
  a cap below five no row can be stamped, since the fake agent replays only a recorded variant).
- Do the six other shapes ride this chunk or move to an entry of their own (the rate_limits row was placed here by
  the founder's own answer, the `agents --json` row by a card he approved)? → blocks: plan-decision (the cut).
