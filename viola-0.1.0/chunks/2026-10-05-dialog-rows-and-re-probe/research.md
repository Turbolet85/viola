# Codebase Research — 2026-10-05-dialog-rows-and-re-probe

## Scope
- **Depth:** deep · **Reads:** 19 · **Globs/Greps:** 21 (plus 9 read-only static reads of the installed 2.1.288
  binary through a scratch scanner, and 3 structure-only census passes over the prototype logs)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, 8 Session Additions applied
  (the `--local-live` row literal, no piped `boot`, the short `TMPDIR`); `.claude/rules/testing.md` — read in full, 28
  Session Additions applied (no `vhome`-less `--record` home, bounded mutant-reachable waits, the guard controls, the
  fake agent's control-file replay).
- **Platform issues consulted:** none — no runner-only bullet (CI 75198e5 green) and no CI-reading entry outside the
  operator leg. The installed CLI's own behaviour was read statically from its binary (M6), never from a search.
- **External inputs:** `inputs#I1` — the operator's take-up directive (the M7 founder card, measure first, one builder
  window, name the live sessions, Epoch 3 unsplit); `inputs#I2` — the structure-only census of the viola-lab
  prototype's interactive 2.1.287 hook logs (decision effect, revise, concurrency not exercised).

## Measured facts (no live `claude` session was started at this phase)
- **M1 — M7 re-verified at HEAD.** `src/cmd/hook.rs:89-91`: with `--capture` the hook files the payload and returns
  exit 0 before any other work; `capture` (`:344-353`) writes only the file. Nothing reaches stdout, so the capture arm
  answers no dialog. The premise of the founder card stands.
- **M2 — the capture arm's naming is not concurrency-safe.** `free_k` (`src/cmd/hook.rs:357-365`) picks the first
  free `k` from a directory listing, and `replace_private` then renames the temp file over `<Event>.<k>.json`. Two
  dialog hooks capturing at once can pick the same `k`; the later rename replaces the earlier payload. The
  dialog-concurrency row cannot be measured through the arm until its naming is exclusive (a create-new claim, then
  retry). Derivation: `src/cmd/hook.rs@75198e5`.
- **M3 — the probe plugin registers only the four spine events.** `CAPTURE_EVENTS`
  (`crates/viola-agent-claude/src/ledger.rs:84-89`) and `capture_plugin_files` (`:120-145`): SessionStart,
  UserPromptSubmit, Stop, SessionEnd. No PreToolUse (with the `AskUserQuestion|ExitPlanMode` matcher),
  PermissionRequest or PostToolUse entry exists, so no current verify run can observe a dialog.
- **M4 — the verdict is all-rows, and one boolean gates two things.** `ledger::verified` (`ledger.rs:357-366`)
  is true only when every `LedgerRow::ALL` row (`:32`, ten) reads `pass`. Its production callers are
  `src/run/version_gate.rs:16` and `:170` (code-graph, M-trace). `run` passes that one `cli_verified` both to the
  dialog gate (`DialogSlot::new`, `src/cmd/run.rs:221-229`; `src/run/dialog.rs:191`, `:338`) and to the readiness
  gate's signatures (`src/cmd/run.rs:492`, `cli_verified.then_some(&SIGNATURES)`). Adding the dialog rows to `ALL`
  unchanged therefore makes every ten-row stamp unverified (until re-stamped), and makes a dialog-row failure also
  switch the screen readiness gate off for that version.
- **M5 — viola's decision bodies vs the prototype's.** `dialog::decision_body` (`crates/viola-agent-claude/src/
  dialog.rs:339-395`; one production caller, `src/cmd/hook.rs:293`): question → PreToolUse `allow` + `updatedInput`
  (input + `answers` + `annotations`); plan approve → PreToolUse `allow` with NO `updatedInput`; plan revise →
  PermissionRequest `deny` + `message`; permission → PermissionRequest `allow` / `deny` + `message`; a question on
  PermissionRequest → no body. The prototype's bodies (`inputs#I2`) added `permissionDecisionReason` everywhere and
  sent its plan approve WITH `updatedInput {plan, planFilePath}`. viola's bare approve has never run against a live
  CLI.
- **M6 — the installed 2.1.288, read statically from its binary** (`~/.local/share/mise/installs/claude/2.1.288/
  claude`, 245 734 584 B):
  - the hook output schemas: PreToolUse `{permissionDecision?, permissionDecisionReason?, updatedInput?: record,
    additionalContext?}`; PermissionRequest `decision: {behavior:"allow", updatedInput?, updatedPermissions?} |
    {behavior:"deny", message?, interrupt?}` — viola's bodies fit both;
  - a PreToolUse `allow` on a tool that requires user interaction is applied ("Hook satisfied user interaction for
    <tool> via updatedInput"), unless the context sets `requireCanUseTool`; an `ask` rule or safety check overrides a
    hook `allow` ("requires full permission pipeline"), and a `deny` rule overrides it too;
  - AskUserQuestion's permission result maps `updatedInput.answers` (keyed by question text, values mapped to option
    labels) and `updatedInput.annotations`;
  - AskUserQuestion is `isConcurrencySafe() → true` (two parallel calls can run, so two dialog hooks can be open at
    once — the concurrency row's premise is reachable); its `isEnabled` is `KJ()`, true whenever the session is
    interactive.
  These are static readings of shipped code paths, not measurements of behaviour; a live probe confirms them.
- **M7 — the prototype measured the hook-answer path live on 2.1.287** (`inputs#I2`, structure only):
  - a PreToolUse `allow` + `updatedInput` answered 21 of 21 AskUserQuestion and 4 of 4 ExitPlanMode dialogs, with
    no PermissionRequest after; the 3 + 1 left unanswered each went on to a PermissionRequest;
  - a PermissionRequest `deny` + `message` on ExitPlanMode (plan mode) was followed by a new ExitPlanMode PreToolUse
    (a re-plan: revise took effect);
  - not exercised: dialog concurrency (0 of 29), an ordinary-tool PermissionRequest, a PostToolUse (never
    registered), anything on 2.1.288.
- **M8 — print mode and dialogs.** On 2.1.287 print mode exposes no dialog tool and fires no dialog hook (measured,
  `2026-10-04-dialog-answers-by-dialog-id/evidence/print-mode-dialog-probe.md`). On 2.1.288 it is not measured live;
  `KJ()` (M6) disables AskUserQuestion in a non-interactive session only when channels are configured or no
  permission-prompt target exists. The re-probe is interactive either way.
- **M9 — Run B inherits the host user's settings.** The dev host's `~/.claude/settings.json` allows `Bash(*)` and
  `Edit(*)`, and registers user hooks on PreToolUse (`Edit|MultiEdit|Write|Read`, `Bash`), PostToolUse
  (`Edit|MultiEdit|Write`, `.*`), PreCompact and SessionEnd. So in Run B an ordinary Bash or Edit call raises NO
  PermissionRequest, and the probe cannot rely on the host's allow list (another user's host differs). 2.1.288 has
  `--settings <file-or-json>` (an added session-scoped source, e.g. an `ask` rule, which M6 says outranks an
  `allow`) and `--setting-sources <user,project,local>` (load only the named sources). No user hook matches the two
  dialog tools.
- **M10 — the fake agent cannot replay a dialog decision.** `run_hook` (`src/bin/viola-fake-agent.rs:325-362`) records
  the hook's `stdout_hex` but never acts on it; `fire` (`:295-323`) plays one fixture per event and variant; the
  trusted interactive mode is `--screens --turn-stop --trusted-root` (`:83-85`) with no dialog mode. CI can stamp
  the new rows only after a new argv mode (no env seam: two exist, a third needs a ruling) that plays the recorded
  dialog fixtures and branches on the hook's decision (body → the post-decision fixture; none → the PermissionRequest).
- **M11 — the row-count literals.** `grep -c -E '/10\]|10 pass|"/10"|LEDGER_ROWS'` over `src crates tests fixtures
  schemas scripts`: 37 hits in 4 files — `src/cmd/verify.rs` 2, `tests/contract_ledger_probes.rs` 1,
  `crates/viola-e2e/src/harness/run.rs` 14 (`LEDGER_ROWS: [&str; 10]` at `:333`), `tests/cli_verify.rs` 20.
- **M12 — W6's site.** `tests/cli_answer.rs:9` reads "The `permission` kind's end-to-end case is owed to the live test
  (working-route `:84`)."
- **M13 — live-session accounting.** `2026-10-05-real-cli-verify-probes/evidence/live-sessions.md`: one full
  `viola verify` spends 3 sessions (print probe, Run A, Run B); that chunk used 16 of a cap raised 10 → 12 → 15 → 18,
  across three plan revisions, with no retry ever allowed. Each interactive run this chunk adds costs one session
  per verify run. Both 2.1.287 and 2.1.288 are installed (`~/.local/share/mise/installs/claude/`). The dev host's
  `~/.claude.json` holds 8 project entries (the accepted Run B residual grows with each new trusted run).
- **M14 — CI since the last wrap.** `75198e5`: green, ci#37301006304, 15/15, wall 326 s.
- **M15 — the matrix entries this entry names.** `v1-15` (planned, unclaimed): its acceptance ends "the one S7 ledger
  row (`plan-approve-revise`), whose post-condition passes in the 2.1.287 stamp". `v1-30` (verified): its open
  witness is the `permission` kind's end-to-end wake. `v1-31` (planned): "the dialog never renders" stays `:90`'s.

## Files inspected
- `src/cmd/hook.rs` (1-110, 195-370) — the capture arm, `handle_dialog`, `body_of`, `free_k`
- `src/cmd/verify.rs` (1-290) — the orchestration: version probe → print probe → `typed::measure` → `check_rows` →
  `update_stamps` → `stamped` line → `--record` only at 0 fail; `ProbeDir` builds both plugin dirs
- `src/cmd/verify/typed.rs` (1-60, index) — Run A / Run B, `turn`, `wait_capture`, `settled`, `end_by_ctrl_c`
- `crates/viola-agent-claude/src/ledger.rs` (18-150, 196-395) — rows, `check`, `merge_stamp`, `verified`, `scrub`
  (recursive over strings and keys, `:371-392`)
- `crates/viola-agent-claude/src/dialog.rs` (index, 330-395) — `classify`, `Response`, `decision_body`
- `src/cmd/run.rs` (150-230; 492) — the version gate's one boolean into the dialog slot and the signatures
- `src/bin/viola-fake-agent.rs` (index, 275-435) — `fire`, `run_hook`, `submit`, scripted steps
- `fixtures/fake-scripts/path4.json`, `fixtures/claude/2.1.287/RELAYED.md`, `tests/cli_answer.rs` (1-14)
- `viola-0.1.0/chunks/2026-10-05-real-cli-verify-probes/{scope,research}.md`, `evidence/live-sessions.md`;
  `viola-0.1.0/chunks/2026-10-04-dialog-answers-by-dialog-id/evidence/print-mode-dialog-probe.md`

## Graph impact (rust plane, `db_state: fresh`, 106 rows; trace `tree-query-2026-10-05-dialog-rows-and-re-probe.json`)
- **`verified`** — production callers `version_gate` and `stamps_verdict` at `src/run/version_gate.rs:16` / `:170`;
  the rest are ledger unit tests. A change of its meaning (all rows → a split verdict) lands there and in `run`'s two
  consumers (M4).
- **`decision_body`** — one production caller, `body_of` at `src/cmd/hook.rs:293`; insta tests in `dialog.rs`.
- **`capture_plugin_files`** — one caller, `ProbeDir::create` at `src/cmd/verify.rs:243`.
- **`merge_stamp`** — `measure` at `src/cmd/verify.rs:202`, and `stamp` (a test helper) at `src/run/version_gate.rs:184`.
- **`measure`** (typed) — `src/cmd/verify.rs:184`; **`capture`** — `hook` at `src/cmd/hook.rs:90`.
- `check` collides by name with the hygiene walker's `check` (`tests/contract_fixture_hygiene.rs`); the ledger's
  own production caller is `check_rows` in `src/cmd/verify.rs`.

## Patterns detected
- **Probe as plugin + capture dir** (`ledger.rs:120-145`, `verify.rs:233-249`): each run gets its own `plugin/` and
  `captures/`, removed by the `ProbeDir` drop guard; rows are judged afterwards from the captures alone.
- **Interactive run shape** (`typed.rs:1-6`, `:46-60`): direct spawn under the R8 strip at 80×24, settle on quiet,
  one bracketed paste, captures awaited, Ctrl-C ×2 then kill; a modal start is killed with no key.
- **Decision bodies are pure and insta-pinned** (`dialog.rs:339-395`; six `.snap` files under
  `crates/viola-agent-claude/src/snapshots/`).
- **Fake-agent modes are argv flags** (`viola-fake-agent.rs:63-85`), never env.

## Conventions to follow
- **Row ids and counts as test literals**, never `LedgerRow::ALL` (test-plan §11; `tests/cli_verify.rs`,
  `tests/contract_ledger_probes.rs`, `crates/viola-e2e/src/harness/run.rs:333`).
- **A failing row still stamps, exits 1, and blocks `--record`** (`src/cmd/verify.rs:201-222`).
- **Compiled probe prompts** (`ledger.rs:80-81`, ASCII, no tag characters), never runtime or upstream text.
- **One `process-start` / `process-exit` pair per spawn at the call site** (`verify.rs:71-98`; `typed.rs:28`
  `SUBJECT = "verify-pty-probe"`).

## New files to create
- derived `fixtures/claude/2.1.288/*.json` by `viola verify --record` — the recorded dialog payloads (and a re-recorded spine)
- derived `fixtures/claude/2.1.287/*.json` by `viola verify --record` — the same at 2.1.287, superseding the relayed four
- `viola-0.1.0/chunks/2026-10-05-dialog-rows-and-re-probe/evidence/` — step 0 and the live-session ledger
  (provisional — the shape of every list below waits on the founder card and the sizing card)

## Files to modify
- `crates/viola-agent-claude/src/ledger.rs` — the dialog rows, their `check`, the probe plugin's dialog entries, the verdict
- `crates/viola-agent-claude/src/dialog.rs` — any body change the live re-probe forces (plan approve, the question's PermissionRequest)
- `crates/viola-agent-claude/src/snapshots/` — insta bodies if a body changes
- `src/cmd/hook.rs` — exclusive capture naming; the answering form the founder ruled (path A, inputs#I3)
- `crates/viola-state/src/fs.rs` — an exclusive create-new helper at the private file mode (none exists: `create_private_dir`, `open_private_append`, `open_private_lock`, `replace_private*` at `fs.rs:35-278`)
- `src/cmd/verify.rs` — the dialog runs in the orchestration, the record step
- `src/cmd/verify/typed.rs` — the dialog run drivers
- `src/run/version_gate.rs` — the R2 closure
- `src/cmd/run.rs` — the readiness gate's verdict if the verdict splits
- `src/run/dialog.rs` — the dialog gate's verdict if the verdict splits
- `src/bin/viola-fake-agent.rs` — the dialog replay mode
- `crates/viola-e2e/src/harness/run.rs` — `LEDGER_ROWS`
- `crates/viola-e2e/src/harness/boot.rs` — step 4's verify argv (`--trusted-root` is set there and in `supervise.rs`; only verify's needs the dialog replay)
- `tests/cli_verify.rs` — step-counter literals, the dialog-run cases
- `tests/contract_ledger_probes.rs` — the stamped sets and the row literals
- `tests/cli_answer.rs` — the R2 negative: an `answer` on a home whose stamp fails the dialog rows (the `permission` case and W6's reword moved to "Permission end to end", the founder's ruling at P4, inputs#I3)
- `tests/contract_fixture_hygiene.rs` — the dialog fixture classes
- `tests/contract_fake_agent_drift.rs` — the recorded dialog tier beside the relayed one
- `tests/support/` — the stamped-home seam and the fake-agent argv
- `fixtures/claude/2.1.287/RELAYED.md` — a dated superseded section naming each recorded counterpart
- `schemas/fake-script.v1.json` — only if the script grammar grows

## Open questions
- The M7 crossing and the live cap: which path makes "the decision takes effect" measurable, and how many live
  `claude` sessions the founder grants → blocks: plan-decision (the P4 founder card).
- Sizing: whether W3a–W3d + W6 fit one builder window or split inside Epoch 3 → blocks: plan-decision (the same
  card, a second question).
- The verdict's shape after R2 closes: all rows in one `cli_verified` (a dialog-row failure also switches the screen
  gate off, M4) or a split verdict (spine + screen gates typing, the dialog rows gate decisions; a new snapshot / obs
  field under default-deny) → blocks: plan-decision.
