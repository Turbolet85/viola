# Step 0 — live screen probe, `claude` 2.1.288 (session 1 of the cap)

**Outcome: STOP at step 0, no code written.** Plan STOP condition 2 fired ("the trust dialog needs more than one
key"). Two further findings bear on the plan's design and are surfaced with it.

## How it ran
- A throwaway Rust probe outside the repository (session scratchpad, `probe0`), built on `viola-pty`
  (`spawn` + `pump_with_paste`) and `vt100 =0.16.2`, the same model `viola-agent-claude::screen` feeds.
- `~/.local/share/mise/installs/claude/2.1.288/claude --model haiku --plugin-dir <scratch>/plugin`, interactive (no
  `-p`), in a fresh empty dir under the session scratchpad (no trusted ancestor in `~/.claude.json`), at 80×24, no
  resize. The 10 inherited `CLAUDE*` names were removed (names only). The capture plugin ran
  `<copied viola> hook <event> --capture <scratch>/captures` for the four spine events.
- Started 2026-10-05T06:28:57Z; the child exited 1 about 2.6 s after spawn (`Exited(Exit { code: Some(1), source:
  HandleWait })`); 1 391 bytes of output in total.

## What the screen showed (rows as vt100 renders them; the cwd described, not copied)
`start`, settled at 300 ms of quiet 847 ms after spawn, and unchanged at 2 000 ms of quiet (2 545 ms):

```
────────────────────────────────────────────────────────────────────────────────
 Accessing workspace:
 <the cwd, absolute, wrapped over two rows>
 Quick safety check: Is this a project you created or one you trust? (Like your
 own code, a well-known open source project, or work from your team). If not,
 take a moment to review what's in this folder first.
 Claude Code'll be able to read, edit, and execute files here.
 Security guide
 ❯ No, exit
   Yes, I trust this folder
 Enter to confirm · Esc to cancel
```

- **The one accept key (Enter, `\r`) chose "No, exit".** The focus starts on the cancel option. The child exited 1
  within 500 ms of the key, and the screen kept the dialog. No trust entry was written: `~/.claude.json` holds no
  project for the probe dir, and `~/.claude/projects/` gained no dir.
- **No hook fired.** `captures/` stayed empty, including SessionStart, for the whole run, the 2.5 s the dialog was up
  included.
- **Text classes on the screen:** the cwd (absolute, wrapped across two rows), and fixed product text. No account
  email or organisation appeared, since the run never got past the dialog.

## The CLI's own code (read from the 2.1.288 binary, no API use)
- The trust dialog is a confirm component rendered with `cancelFirst:!0`, `focus:"cancel"`, `hideIndexes:!0`,
  `cancelLabel:"No, exit"`, `confirmLabel:"Yes, I trust this folder"`, and `refuseInput` / `openedAt` (input refused
  for a window after the dialog opens). So accepting takes at least a focus move and Enter, and an early key may be
  refused. Which key moves the focus, and how long the refusal window is, were not measured.
- **Trust is inherited from parent directories.** The trust check walks up from the cwd and returns the first
  ancestor whose `projects[<dir>].hasTrustDialogAccepted` is true (`zE`, called from `k0` / `jE`). On this host
  `/home/turbolet/dev/projects/viola` is a trusted project, so any probe dir under `target/e2e-home/` (the plan's
  record and `local-live` homes) starts with no trust dialog at all.

## Plan STOP conditions
| # | Condition | Reading |
|---|---|---|
| 1 | a startup modal other than the trust dialog | none seen |
| 2 | the trust dialog needs more than one key | **FIRED**: Enter alone selects "No, exit" |
| 3 | the post-turn screen does not settle within `GATE_MAX_WAIT` | not reached |
| 4 | the prompt does not reach UserPromptSubmit within `CONFIRM_WINDOW_FALLBACK` | not reached |
| 5 | a screen class the scrub cannot clean | not reached past the dialog; the cwd wraps across rows (see below) |

## Findings beyond the STOP (for the decision)
- **F1, the trust dialog's default is cancel:** the founder's "one accept key" ruling cannot be met on 2.1.288. One
  key is "No, exit".
- **F2, hooks wait for trust:** SessionStart does not fire while the dialog is up. So plan step 3 phase 1 ("wait for the
  SessionStart capture and quiet, then record `start`") cannot complete on an untrusted dir.
- **F3, inherited trust:** under a trusted ancestor no dialog renders. There the planned `modal-signature` check
  ("`start` rows hold a `SIGNATURES.modals` literal") fails by construction, which reads `9 pass 1 fail` for the
  record entries and `local-live`.
- **F4, wrapped paths:** a long cwd wraps across rows, so a per-row scrub that matches the home prefix can leave a
  username split across a row boundary. This applies to the record step's screen scrub.

## Numbers
- start settle: 847 ms (300 ms quiet), 2 545 ms (2 000 ms quiet); `GATE_MAX_WAIT` = 5 s.
- prompt latency, turn settle, max turn gap: not measured (the run did not reach the turn).

---

# Step 0 of plan revision 2 — sessions 2 and 3 (2026-10-05)

**Outcome: STOP 3 at step 0, no code written.** Run A read as planned. Run B's start showed a modal, a second
CLI-native dialog the plan did not foresee, so the input-box literal and the timing numbers were not reached.

## How it ran
- A throwaway Rust probe outside the repository (session scratchpad, `probe1`, revision 1's `probe0` adapted). It is
  built on `viola-pty` (`spawn` + `pump_with_paste`) and `vt100 =0.16.2`. The PTY is shared through an
  `Arc<Mutex<PortablePty>>` wrapper that implements `Pty`, so the drive kills the child while the pump runs, which is
  the plan's step-3 design. It used `viola-pty` unchanged.
- Both runs used `~/.local/share/mise/installs/claude/2.1.288/claude --model haiku`, interactive (no `-p`), at 80×24
  with no resize. The 10 inherited `CLAUDE*` names were removed (names only).
- A screen was snapshotted at every 300 ms quiet that followed new output, and the run held to 2 000 ms of quiet
  before acting.
- **Session 2, Run A** (start 08:37:44Z):
  - cwd: a fresh 0700 dir `/tmp/viola-verify-s2-<epoch>`, which has no trusted ancestor;
  - no `--plugin-dir`, and input `std::io::empty()`, so no byte was written;
  - ended by `Pty::kill`.
- **Session 3, Run B** (start 08:38:15Z):
  - cwd: `<repo root>/.viola-verify-<shell pid>/`, 0700;
  - `--plugin-dir` held the capture plugin (`<HEAD viola copy> hook <event> --capture <scratch>`, the four spine
    events);
  - input was an in-process channel. The probe's rule was "a start that shows a modal is killed with no key", and it
    fired, so nothing was pasted and no Ctrl-C was sent.

## Session 2, Run A — as planned
- The first settled screen is the workspace trust dialog, settled at **847 ms** (300 ms quiet) and unchanged at 2 547
  ms (2 000 ms quiet). Its rows hold `Yes, I trust this folder` on one row and "❯ No, exit" above it.
- **Text classes:**
  - the cwd, absolute, on one row (the short `/tmp` path did not wrap);
  - fixed product text.
  - There was no account, email or organisation text.
- No startup modal other than the trust dialog appeared (STOP 1 clear).
- The kill ended the child about 0.2 s later (exit 1, `HandleWait`), 1 195 bytes in total.
- **Residual (STOP 2), read after the kill, names only:**
  - `~/.claude.json` `projects` held 8 keys, as before, with none for the dir and no `viola-verify` key;
  - `~/.claude/projects/` held 13 dirs, as before, with none for the dir.
  - The dir was then removed.

## Session 3, Run B — STOP 3
- The first settled screen is a **second CLI-native dialog**, settled at **563 ms** (300 ms quiet) and unchanged at
  2 263 ms (2 000 ms quiet). Its rows, described:
  - a title row, `Allow external CLAUDE.md file imports?`;
  - two rows of fixed explanation ("This project's CLAUDE.md or .claude/rules imports files outside the current working
    directory…");
  - an `External imports:` row, then one row holding the import's absolute path under the user's home (the repo
    CLAUDE.md's `@.claude/session-handoff.md`, which lies outside the probe subdir);
  - a two-row security note with a docs URL;
  - the options "❯ No, disable external imports" (focused) and "Yes, allow external imports";
  - `Enter to confirm · Esc to cancel`.
- No trust dialog appeared, which confirms M13: the repo root is a trusted ancestor.
- No hook fired (`captures/` empty, SessionStart included), so this dialog also comes before the session's hooks.
- The kill ended the child about 0.2 s later (exit 1), 1 317 bytes in total.
- **Residual, names only:**
  - `~/.claude.json` `projects` still held 8 keys, with none for the dir;
  - `~/.claude/projects/` still held 13 dirs, with none for the dir;
  - the root project's `hasClaudeMdExternalIncludesApproved` / `hasClaudeMdExternalIncludesWarningShown` read
    `false` / `false`, unchanged.
  - The dir was then removed, and no `claude` process was left with a probe-dir cwd.

## Why the dialog shows (read from the 2.1.288 binary, no API use)
- The memory-file walk loads the ancestor project's CLAUDE.md for a subdir cwd. An `@` import whose target lies outside
  the cwd is an "external include". The repo root's own sessions never see the dialog, because there the import is
  inside the cwd.
- The dialog shows while the project config's `hasClaudeMdExternalIncludesApproved` is unset, and it carries the same
  `refusedWithin` / `noteRefused` input-refusal window as the trust dialog.
- **The project config is keyed by the cwd's git root** (`IOe`: `SI(cwd)` → the git root, else the cwd). So for any
  subdir of this repo the key is the repo root's `projects` entry, which reads `false` / `false`.
- Either answer writes `hasClaudeMdExternalIncludesApproved: <choice>, hasClaudeMdExternalIncludesWarningShown: true`
  into that entry (`P9t`). The approval is per project, not per subdir.
- **The show condition** (`pPo`): the dialog shows only when neither flag is set and the walk finds an external
  include. So a single recorded answer, either one, ends it for every subdir of the repo. "No" (`Approved:false,
  WarningShown:true`) also leaves the import unloaded in subdir sessions. Repo-root sessions are unaffected, because
  there the import is not external.
- Both installed binaries carry the dialog: 2.1.287 and 2.1.288 each hold `Allow external CLAUDE.md file imports?`,
  `Yes, allow external imports` and `Yes, I trust this folder` (strings read, no session).

## Plan STOP conditions (revision 2)
| # | Condition | Reading |
|---|---|---|
| 1 | Run A's first quiet screen holds no `Yes, I trust this folder`, or shows another startup modal | clear: the trust dialog only, with the literal |
| 2 | a `projects` key or projects dir for Run A's dir after the no-key kill | clear: none |
| 3 | Run B's start shows any modal | **FIRED**: "Allow external CLAUDE.md file imports?" |
| 4 | Run B's `ready` / `turn` settle later than `GATE_MAX_WAIT` | not reached |
| 5 | the prompt does not reach UserPromptSubmit within `CONFIRM_WINDOW_FALLBACK` | not reached |
| 6 | no input-box literal, or a candidate row (or junction) holds a path, the username or an email | not reached; the modal's own rows hold an absolute home path |

## Numbers (revision 2)
- Run A modal settle: 847 ms (300 ms quiet), 2 547 ms (2 000 ms quiet).
- Run B start settle (the external-imports dialog): 563 ms (300 ms quiet), 2 263 ms (2 000 ms quiet).
- Input-box literal, `ready` / `turn` settle, prompt latency, max turn gap: not measured.
- Live sessions: 3 of 10 used (`live-sessions.md`).

---

# Step 0 of plan revision 3 — session 4 (the plan's "session 5"), Run B only (2026-10-05)

**Outcome: clear, no STOP.** The precondition held, Run B reached the input box with no modal, the turn ran, and the
input-box literal is chosen. Run A was not repeated (session 2's reading stands, M19).

## Precondition (STOP 0), read before the session, booleans and counts only
- Read 2026-10-05T09:18:58Z from `~/.claude.json`, the repo root's `projects` entry:
  `hasTrustDialogAccepted: true`, `hasClaudeMdExternalIncludesApproved: false`,
  `hasClaudeMdExternalIncludesWarningShown: true` — "No, disable external imports", as the overseer set it on the
  founder's amended live ruling (~09:05Z) and verified at 09:10Z and 09:11:52Z (`relay-7.md`).
- **Residual baseline:** 8 project keys, none `viola-verify`; `~/.claude/projects/` holds 13 dirs, none
  `viola-verify`.
- STOP 0: clear.

## How it ran
- Revision 2's throwaway probe `probe1`, copied to this session's scratchpad as `probe2` (outside the repository). One
  change: a start counts as a modal when any row holds the trust literal, either external-imports text, or a dialog
  footer (`Enter to confirm` / `Esc to cancel`). `probe1` read "trust" only. `viola-pty` and `vt100 =0.16.2` are
  unchanged, and the PTY is shared through the same `Arc<Mutex<PortablePty>>` wrapper.
- `~/.local/share/mise/installs/claude/2.1.288/claude --model haiku --plugin-dir <scratch plugin>`, interactive, at
  80×24 with no resize. The 10 inherited `CLAUDE*` names were removed (names only). The capture plugin ran a copy of
  HEAD's `viola` (`hook <event> --capture <scratch>`) for the four spine events.
- cwd: `<repo root>/.viola-verify-<shell pid>/`, 0700. Started 2026-10-05T09:20:15Z and ended 09:20:23Z.
- The drive held for 2 000 ms of quiet, then pasted `PROBE_PROMPT` once (one bracketed paste + `\r`, through
  `PasteHandle::paste`), waited for the Stop capture, then for quiet, then sent Ctrl-C, and Ctrl-C again after 500 ms.

## What the screens showed (described, never copied raw)
- **`ready`** (first 300 ms quiet, 1 084 ms after spawn; unchanged at the 2 000 ms quiet, 2 784 ms):
  - the CLI's banner block: the product name with its version, the model name with the account's plan tier, and the
    cwd as a `~`-abbreviated path (row 3);
  - the input box: a rule, a prompt row holding a rotating placeholder suggestion, and a rule;
  - the user's own status line (from the user's global settings): a context figure and the model name;
  - the footer row: a permission-mode indicator (`manual mode on`, built at runtime, not a binary literal) and the
    agents hint `← for agents`.
- **`turn`** (first 300 ms quiet after the Stop capture): the same banner, the typed prompt echoed, a one-word reply,
  a "worked for" line holding a local time of day, the empty input box, and the same footer row. A later redraw
  (4 996 ms) only filled in the status line's usage figures.
- After the first Ctrl-C the footer row read the CLI's "press Ctrl-C again" hint. After exit the screen held the
  CLI's resume line, which holds the session id.
- **Text classes seen:** the cwd (`~`-abbreviated, no username), the model name, the account's plan tier, the user's
  status line (context and usage figures), a time of day, the session id (exit only), and fixed product text. No
  email, organisation name or absolute home path appeared on any row.
- No modal appeared at any point (STOP 3 clear). All four spine hooks fired: SessionStart at 565 ms, UserPromptSubmit
  at 2 820 ms, Stop at 4 379 ms, SessionEnd at 7 488 ms.

## The input-box literal: `for agents`
- **On both screens:** the footer row of `ready` and of `turn` (row 23) holds it.
- **On neither dialog:** no row of session 2's trust dialog or session 3's external-imports dialog holds it (both
  re-read from their scratch reports).
- **Holds no path, username, model, account or organisation text.** Its row's only other text is the mode indicator.
  The seam with row 22 (the status line) joins a context figure and a model name, with no path, username or email.
- **A string in both binaries:** `for agents` occurs 4 times in 2.1.287 and 4 times in 2.1.288. In 2.1.288 it is the
  agents footer's hint, rendered `<arrow> for agents` when the agents view is closed, otherwise `to go back`.
  `Yes, I trust this folder` (2) and `Yes, allow external imports` (2) are strings in 2.1.287 too.
- **Rejected in-rule candidates:** `manual mode on` is not a binary literal and changes with the permission mode
  (`accept edits on`, `plan mode on` and `auto mode on` are the literal siblings). `⏸` is that indicator's glyph.
- **Rejected by the rule:**
  - the banner's product name: the trust dialog's text holds "Claude Code";
  - the rules and the `❯` prompt glyph: both dialogs hold them;
  - the banner's model, plan and cwd rows and the status line: model, account and path text.

## Plan STOP conditions (revision 3)
| # | Condition | Reading |
|---|---|---|
| 0 | the precondition does not hold | clear (09:18:58Z) |
| 1 | Run A: no trust literal, or another modal | not re-run; session 2 reads clear (M19) |
| 2 | Run A residual | not re-run; session 2 reads clear (M19) |
| 3 | Run B's start shows any modal | clear: no trust, external-imports or dialog-footer text |
| 4 | `ready` or `turn` settles later than `GATE_MAX_WAIT` (5 s) | clear: 1 084 ms after spawn; 314 ms after Stop |
| 5 | the prompt does not reach UserPromptSubmit within `CONFIRM_WINDOW_FALLBACK` (10 s) | clear: 31 ms |
| 6 | no input-box literal, or a candidate row (or junction) holds a path, the username or an email | clear: `for agents` |

## Numbers (session 4)
- `ready_settle_ms` 1 084 · `prompt_latency_ms` 31 · `turn_settle_ms` 314 · `max_turn_gap_ms` 224.
- 4 973 bytes of output in total. The pump returned `Exited` code 0, 1 382 ms after the first Ctrl-C.

## Residual, read after the session (2026-10-05T09:20:49Z, booleans and counts only)
- The repo root's three flags are unchanged: trust `true`, `Approved: false`, `WarningShown: true`.
- `~/.claude.json` still holds 8 project keys, none for the probe dir.
- `~/.claude/projects/` holds 14 dirs (13 before). The new one is the session transcript for the dead
  `.viola-verify-<pid>` path, the residual plan revision 3 names.
- The probe dir was left empty (the scratch probe has no drop guard) and was removed by hand; no `claude` process has a
  probe-dir cwd.
- **Also observed:** the user's own global status line ran inside the probe session. The plan names the user's global
  hooks as part of this residual; the status line is the same class.

---

# Step 10 — the 2.1.288 record run (gate entry 7, sessions 5-7): STOPPED

**Outcome: the measurement stamped clean, the recording was refused, the round STOPPED.** No retry (the cap).

## What ran
- The live round `--live-legs --entry 7`, fired 2026-10-05T09:42:11Z (record home `viola-record-20261005T094211Z`):
  entry 6 (`claude --version`) green, then entry 7 red (`round-094211Z.txt`).
- `viola verify --record` against `~/.local/share/mise/installs/claude/2.1.288/claude`, from the repo root: the print
  probe, Run A in a fresh `viola-verify-*` dir under `/tmp` (TMPDIR unset), Run B in `<repo root>/.viola-verify-<pid>/`.

## What it read (the record home's `ledger/stamps.json`; numbers and row ids only)
- All ten rows `pass`; stdout ended `stamped 2.1.288  10 pass  0 fail`.
- `measured.typed_probe`: `ready_settle_ms` 1084, `turn_settle_ms` 601, `prompt_latency_ms` 50, `max_turn_gap_ms` 310.
- `measured.largest_hook_payload`: SessionStart 423, UserPromptSubmit 548, Stop 571, SessionEnd 470 bytes.

## Why the gate is red
- After the stamp, `record` printed `unable: a recorded payload still holds a path or a username` /
  `hint: record with a viola home under your user home` and verify exited 1. The entry's `cp` never ran.
  `fixtures/claude/2.1.288/` was not created.
- **Which file tripped it is not measurable.** The refusal checks the four scrubbed payloads, then the three screens.
  It refuses whole, writes nothing, and its one fixed message is the same for a payload and a screen. The probe dir
  that held the captures is removed before `record` runs, and no diagnostics line is written without `VIOLA_NAME`.

## Residual, read after the run (2026-10-05T09:43:16Z, booleans and counts only)
- **The repo root's flags are unchanged:** trust `true`, `Approved: false`, `WarningShown: true`.
- **STOP 2 clear.** `~/.claude.json` holds 8 project keys, none for a `/tmp` Run A dir. `~/.claude/projects/` holds
  no `-tmp-viola-verify*` dir.
- **STOP 1 is read through the row only.** `modal-signature` passed, so Run A's settled screen held a compiled modal
  literal. The screen itself was not kept, because the recording was refused.
- `~/.claude/projects/` holds 15 dirs (14 before). The new one is Run B's transcript for the dead
  `.viola-verify-<pid>` path, the planned residual.
- No `claude` process has a probe-dir cwd, and no `.viola-verify-*` dir is left in the repo root.

---

# Step 10 — the entry-7 re-run with a named refusal (sessions 8-10): STOPPED, cause named

## The named refusal, proven offline first (no session)
- Approved by the overseer (founder-delegated) on 2026-10-05: `record` now names the refused file and its check code
  (`home-path` · `absolute-path` · `username` · `email`; a screen adds `row <n>` and `seam`), never the content.
  Recorded `widening` in `scope-record.md`.
- **Planted proofs.** `cli_verify` plants:
  - a payload holding a planted path under another user's home dir → `unable: a recorded fixture is not clean: Stop.default.json
    absolute-path`;
  - four dirty screens → `Screen.ready.json row 23 home-path` / `row 23 username` / `row 23 email` /
    `row 23 seam username`.
- No refusal carries the user's name. Ledger units 128/128, `cli_verify` 22/22.

## The re-run (fired 2026-10-05T10:00:46Z, record home `viola-record-20261005T100046Z`)
- All ten rows `pass`: `stamped 2.1.288  10 pass  0 fail`. `typed_probe`: ready 1057 ms, turn settle 597 ms,
  latency 45 ms, max gap 310 ms.
- **The refusal named the cause:** `SessionStart.default.json absolute-path`. Nothing was written or copied, and the
  round STOPPED at 7 (`round-100046Z.txt`).

## The cause, read with no session
- The print-mode probe's `cwd` is `<record home>/ledger/probes/<pid>`. The plan's record entries pass
  `--home "$h/home"`, so the recorded `cwd` reads `<user home>/dev/projects/viola/target/e2e-home/viola-record-<ts>/`, then a dir named
  `home`, then `ledger/probes/<pid>`.
- The scrub rewrites the user home to `~`, but the record home's own `/home/` component stays. The fixture hygiene
  check (`has_absolute_path`: any `/home/`) refuses it, and the same `cwd` sits in all four spine payloads.
- The committed 2.1.287 set was recorded with a home at `~/.viola-record-<ts>`, which has no such component, so its
  scrubbed `cwd` passes.
- The product's check works as designed. The record entries' `--home` path is the defect.

## Residual, read after the run (2026-10-05T10:01:22Z)
- Flags unchanged. STOP 2 clear: no `/tmp` Run A key or dir.
- `~/.claude/projects/` holds 16 dirs. The new one is Run B's transcript, the planned residual.
- No probe process and no probe dir is left.

