# Scope — 2026-10-05-real-cli-verify-probes

**Working entry** (`working-route.md:84`, Epoch 3 — Windows slice II: driving verbs and live proof):
Real-CLI verify probes — typed-input PTY probe against the installed claude, screen signatures and timing,
local-command, dialog-body and paste-framing ledger rows, measured and stamped.

## Intent
`viola verify` gains its first PTY child: a typed-input probe that runs the live installed `claude` interactively on
the Linux dev host. It records what the print-mode probe cannot see — the screen, the timing, local commands, dialog
hooks and paste framing. The ledger rows those measurements unlock are added, each with its `viola verify` probe and a
post-condition. The installed CLI version is stamped. The provisional constants and the dated gaps owed to this entry
are closed. viola stays mechanism: the probe measures and stamps, it decides nothing for a driver.

## Authority
- The founder's ruling **R-S2**, live, 2026-10-05 ~00:00Z, relayed by the overseer (inputs#I1). It ratifies the held
  widening: `viola verify` may gain a PTY-driven typed-input probe running the live installed `claude` on the Linux
  dev host. That covers its first PTY child, the capture arm's caller, the screen recording and the signature /
  quiet-period / max-wait ledger rows. It runs locally on the dev host only, with no Claude credential on a CI runner,
  and it stamps the version it runs.
- **R-S1** minted this entry by splitting it ahead of "First live test and self-drive" (inputs#I1). The partition is
  the overseer's answers at the 2026-10-05 0-pending wrap.
- The operator's directive at this take-up (inputs#I2):
  - size the chunk against one builder window;
  - if the nine CARRYs do not fit, bring a split card at P4 instead of planning past the window;
  - record how many live `claude` runs the plan needs, since each counts against the founder's subscription;
  - Epoch 3 stays unsplit.
- **The founder's live ruling on the P4 split card** (2026-10-05, answered 05:58Z via the overseer's AskUserQuestion,
  relayed by the operator; verbatim in the phase run dir `relay-2.md`):
  - the split is three ways, probe first;
  - **this chunk is W1 + W5 only**: the PTY typed-input probe, the screen and timing rows, `run`'s gate wiring, the
    fake-agent screen replay, the screen fixture class and the 2.1.288 stamp;
  - live `claude` is capped at **10 sessions** for this chunk;
  - two new Epoch 3 entries go before `:86`: the dialog rows (W3 + W6) first, then local-command + paste-framing
    (W2 + W4).

## Scope after the split (founder, live, 2026-10-05 05:58Z)
- **Built here:** W1 and W5 below.
- **Moved to new entries, not built here:** W2, W3, W4 and W6 below stay recorded as folded freight. This chunk's
  wrap re-homes them through route-resolve:
  - "Dialog rows and re-probe": W3 + W6, CARRYs 6, 7 and 9;
  - then "Local-command and paste-framing rows": W2 + W4, CARRYs 2, 3 and 5.

  Both go ahead of "First live test and self-drive". CARRY 1 (the split record) stays with this chunk.

## Revision 2 — the trust modal, re-ruled (founder, live, 2026-10-05)
Implement's step 0 stopped on revision 1's plan (`evidence/screen-probe-2.1.288.md`):
- the 2.1.288 trust dialog focuses "No, exit", so one key exits;
- no hook fires before trust;
- trust is inherited from a trusted parent, and the repo is one.

Two founder live rulings, relayed by the operator, now govern W1 and supersede the one-key ruling quoted below (now
unused):
- **~07:00Z** (`.andromeda/runs/2026-10-05T07-05-09-phase/relay-3.md`): two runs, never accept.
  - The modal sample comes from an untrusted dir. It is recorded, then the session is closed with NO key: viola never
    types into a CLI-native dialog, and nothing is written to `~/.claude`.
  - The input-box and timing rows come from a probe dir under an already-trusted parent, with no keys.
  - The plan names which trusted parent a user verify needs, records the start without waiting for SessionStart, and
    fixes the wrapped-path scrub gap.
  - The live cap stays 10 (9 left).
- **08:25Z** (`relay-4.md`, answering P4's two boundary-widening questions):
  - Run B (trusted) runs in verify's working dir, `<cwd>/.viola-verify-<pid>/`, 0700, removed on every exit path, with
    a `.gitignore` line. The transient dir in the user folder was shown.
  - Run A (untrusted) runs in a fresh 0700 dir under the OS temp dir, removed on every exit path.

W1's bullets below that name the one accept key, "record the screen after SessionStart", and a single typed probe
in the probe dir read as revision 1's and are superseded by this section. The cap arithmetic moves `local-live`'s
live firing to "First live test and self-drive" (`:86`). The W5 stamp is the 2.1.288 record run's stamped line.

## Revision 3 — the external-imports dialog (founder, live, 2026-10-05 ~08:50Z)
Implement's step 0 on revision 2 stopped at STOP 3 (`evidence/screen-probe-2.1.288.md` §"Step 0 of plan revision 2";
research M19). [premise-corrected at P3, revision 3: "a probe dir under an already-trusted parent starts with no
modal" holds for the trust dialog only.] A subdir of this repo shows "Allow external CLAUDE.md file imports?",
because the repo CLAUDE.md `@`-imports a file outside the subdir, and the approval is keyed by the git root.

Rulings, verbatim in `.andromeda/runs/2026-10-05T09-09-05-phase/relay-6.md`:
- **(1) the founder, live:**
  - the founder answers the external-imports dialog by hand, once, choosing "No, disable external imports", in a repo
    subfolder;
  - viola still never types into it;
  - the overseer verifies the stored flags before implement reruns step 0.
  - **Amended live at ~09:05Z** (the founder, the overseer relaying at the P5 word; `relay-7.md`): the founder could
    not answer by hand, so the overseer set the flags by an atomic edit of `~/.claude.json`, the same as answering
    "No", with a backup kept. The overseer verified them at 09:10Z and 09:11:52Z. No hand session was spent.
- **(2) the founder, live:** the live cap is raised to **12**. The plan counts 10 of 12, with 2 spare.
- **The overseer, founder-delegated:**
  - `Yes, allow external imports` joins the compiled modal literals, so `run`'s full gate refuses while it is up;
  - the `verify` help names an unapproved external import as a blocker.
- Kept: no retry, and every STOP condition.

Run A's step-0 reading (session 2) stands, because nothing in revision 3 changes its inputs. Step 0 repeats Run B only.

## What it builds (the nine CARRYs folded, in route order)

### W1 — the typed-input PTY probe and the screen rows (CARRY 4: the held widening, ratified R-S2)
- The probe's shape is the readiness-gate chunk's `plan.md` §Held widening (`2026-10-04-readiness-gate-and-timing-
  constants`, lines 383-408):
  - `verify` spawns the resolved `claude` under a `viola-pty` PTY. It runs interactive (no `-p`) with `--model haiku
    --plugin-dir <ledger/probes/<pid>>/typed/plugin`, its cwd the probe dir. [premise-corrected at P4, val-1:
    `--no-session-persistence` "only works with --print" per the 2.1.288 `--help`, so the held widening's flag is
    dropped. The interactive session is persisted under `~/.claude`, which the founder's trust ruling below accepts as
    a residual.]
  - The workspace trust dialog shows interactively. The founder ruled live (2026-10-05, at this P4, via the overseer's
    AskUserQuestion, the residual shown): record the trust screen first, then type the one accept key for the probe's
    own empty per-run dir. Any other startup modal STOPs implement. The residual is a trust entry and a
    synthetic-prompt transcript for the dead probe path in `~/.claude`.
  - The older recorded sets and the timing shape were ruled by the overseer, founder-delegated, at this P4:
    - record 2.1.287's screens with the installed binary;
    - 2.1.283 leaves the stamp walk and stays for the byte-drift contract;
    - the timing rows validate the compiled values, so `run` reads no number from `stamps.json`.
  - The terminal is a fixed 80×24 with no resize (the H2 discipline). It runs under a bounded deadline with a
    `MAX_FRAME`-capped output reader.
  - The probe waits for the SessionStart capture, then for the screen to go quiet, and records the screen.
  - It types `PROBE_PROMPT` as one bracketed paste plus Enter, then waits for the UserPromptSubmit and Stop captures
    and quiet again, and records again.
  - It ends the child.
- The rows it adds are the input-box signature, modal-absent, and the measured quiet-period / maximum-wait keys. So
  `LedgerRow::ALL` grows, and every stamped home re-stamps.
- The gate's provisional 300 ms / 5 s / 10 s built-ins and its two-literal-list signature format leave PROVISIONAL.
  Until this lands, every CLI build reads screen-unverified.
- [premise-corrected: verify already strips R8 for both its children, `src/cmd/verify.rs:137` → `:141` / `:163`
  (research M2)] The held widening named four crossings. R-S2's words cover (1) the PTY child, (2) the capture arm's
  caller and (4) the screen recording.
  - (1)'s R8 sub-question is not open: a PTY child built from the same `StripPlan` keeps the strip verify already
    applies.
  - Still open: (3) a new obs `subject`, or `verify-probe` redefined. That is a `diag-line.v1.json` change plus its
    sidecar record (obs history `2026-09-29-t15-07-57-wrap`: the Decisions Log now lives in the amendment registry).
- `run`'s readiness gate passes no signatures at HEAD (`src/run/gate.rs:165`, `verdict(None, …)`; research M3). So
  landing the signature and timing rows includes wiring `run`'s gate and `send`'s confirmation window to the stamped
  version's passing rows, with compiled literals only.
- `--record` writes screen row text as fixtures, the first fixture class that is screen content. The text is scrubbed
  for paths and the username and refused whole if either survives.
  - The fake agent then replays the recorded screen, so CI can stamp the new rows without a credential (held widening,
    "The live leg it needs").
  - Verified (research M4, M9): the fake agent renders no screen today. The fixture schema and the hygiene walk are
    JSON-hook-only, so the class needs a new replay mode, a schema and a planted-red case.
- [premise-corrected: the refusal lives in the harness, `crates/viola-e2e/src/harness/run.rs:130`, and `viola` reads
  no `CI` (arch extract; arch history `2026-09-29-verify-stamped-test-homes-and-harness`)] The live leg runs only
  through `run --local-live`, which refuses `live-in-ci` before any spawn. `viola verify` itself gains no `CI` read.

### W2 — local-command rows and `unconfirmable` (CARRY 5) — MOVED to "Local-command and paste-framing rows"
- Each known local command maps to its measured post-condition, or to "none". `/clear` maps to SessionStart `clear`
  plus a new `session_id`. Each row has a typed `viola verify` probe.
- `send` gains its `ok {confirmed:false, detail:"unconfirmable", cursor}` outcome and the `[  ] unconfirmable` mirror
  writer.
- At HEAD a local command ends `not-delivered` / `no-prompt-submitted`; the test
  `send_window_local_command_is_not_presumed_delivered` pins that, and it changes here.
- Verified against the cap's notes: with W1's input-box signature and W2, `verification-matrix.json#v1-29`'s three
  owed clauses are all owed to this entry:
  - "only when the input box is ready";
  - `unconfirmable` for a ledger-listed local command;
  - `/clear` confirmed by its new-session post-condition.

  That makes v1-29 claimable here (its notes, the P5 entries of 2026-10-04-confirmed-send-with-cl-1-records and
  2026-10-04-running-turn-refusal).

### W3 — dialog rows, the re-probe and the permission end-to-end (CARRYs 6 and 7) — MOVED to "Dialog rows and re-probe"
- The S3 / S7 / S8 / dialog-concurrency capability-ledger rows, with their own `viola verify` re-probe. They supersede
  the relayed 2.1.287 dialog fixtures (`fixtures/claude/2.1.287/RELAYED.md`).
- The decision effect's first half: the decision takes effect, recorded by that re-probe.
- This closes the founder's ruling R2's dated gap: at HEAD a non-`null` decision flows on the six-row spine stamp
  (`LedgerRow::ALL` = 6, counter `/06`). CLAUDE.md's "one dated exception: the S3/S7/S8 dialog bodies ride the six-row
  stamp until route `:84`" closes with it.
- `verification-matrix.json#v1-15` is claimed here (the founder's ruling R2, ~14:08Z, 2026-10-04). Its acceptance
  stands unweakened: the S7 `plan-approve-revise` row's post-condition must pass in a real stamp.
- From a recorded ordinary-tool PermissionRequest:
  - the `permission` kind's end-to-end Path 4 case;
  - its `verification-matrix.json#v1-30` wake witness. v1-30 is verified; this witness was owed to the dialog chunk by
    its claim's condition, and was pinned here.
- The PermissionRequest body for a `question` first raised by PermissionRequest. It is unmeasured on 2.1.287 and left
  `null` to the human at HEAD.
- Verified (research M8): a print-mode turn raises no dialog hook on 2.1.287
  (`2026-10-04-dialog-answers-by-dialog-id/evidence/print-mode-dialog-probe.md`). So the dialog re-probe must drive the
  interactive PTY probe of W1, which W3 therefore depends on.
- New finding (research M7): the capture arm answers nothing, so "the decision takes effect" needs either a probe
  hook that also answers or the probe keying the rendered dialog. Both are crossings beyond R-S2's words, and the
  founder's to rule.

### W4 — paste-framing and identity rows (CARRYs 2 and 3) — MOVED to "Local-command and paste-framing rows"
- The long-paste wrapper, tag-escaping and harness-prefix ledger rows, with live typed probes, and the R8
  identity-floor row (CARRY 2, from 2026-09-28-capability-ledger-and-viola-verify).
- The harness prefixes gained `<\cross-session-message` and `<cross-session-message` on a RELAYED measurement (CARRY 3:
  founder-ratified live 2026-09-29 15:21:44). That measurement is overseer1's F115 on andromeda-worker plus the
  viola-lab prototype's `harness_injected`; there is no fixture or ledger row here. Measure a real cross-session
  UserPromptSubmit `prompt`, escaped or plain, and record the harness-prefix ledger row.
- HYPOTHESIS (CARRY 3, kept verbatim; still unmeasured at the hook layer. It was observed again at the model layer
  in this 2.1.288 session's 14 P2 hand-backs, research M10): "observed in this orchestrating Claude Code 2.1.283 session and never
  measured at the hook layer: a subagent's hand-back reaches the model framed `Another Claude session sent a message:`
  ahead of `<agent-message from="…">` (every doc-agent return of that chunk's wrap arrived so, and one had its quoted
  tags neutralised `<` → `<\`); if the UserPromptSubmit `prompt` carries that preface,
  `starts_with("<agent-message from=")` misses it". The probe measures it; it is not a premise.
- Verified (research M10): producing a real cross-session UserPromptSubmit needs a second session sending to the
  probed one. Nothing in this repository causes one, and the messaging socket and token are R8-stripped from the
  child.

### W5 — stamp the installed 2.1.288 (CARRY 8)
- Stamp the installed `claude` 2.1.288 through `viola verify`. mise installed it on the Linux dev host 2026-10-04
  14:19Z, and the stamp is owed since; the running sessions read 2.1.287. The probe stamps the version it runs.

### W6 — reword a stale name (CARRY 9) — MOVED to "Dialog rows and re-probe"
- `tests/cli_answer.rs:9` names this entry "the live test (working-route `:84`)" for the `permission` end-to-end case.
  The coordinate holds after the split, but the name does not. Reword it when this chunk touches that file (W3 does).

### CARRY 1 — the split's own record
- This entry was minted at the 2026-10-05 0-pending wrap (R-S1) ahead of "First live test and self-drive", whose line
  carried 11 CARRY blocks. The measured basis: `:78`, a lighter load, ran its builder to 86.7 % of the window. Epoch 3
  stays unsplit at 11 entries (founder 2026-09-29, re-affirmed 2026-10-04). This bullet is the sizing anchor for the
  operator's directive.

## Boundaries (not this chunk)
- Owned by `:86` First live test and self-drive:
  - "the dialog never renders" (`verification-matrix.json#v1-31`);
  - where that entry's live proof runs (the founder's call when it phases);
  - the Windows-only live items: H2's key loss after a resize, the DA1 no-terminal stall, and F-W3's real-terminal
    mouse report;
  - the Epoch 3 boundary audit's 25 Windows-dispatch survivors.
- No Claude credential on any CI runner. The live probe is local to the Linux dev host (R-S2).
- R-S3: the U02 setup upgrade and the `host-win32.md` regenerate run at the Epoch 3 boundary ritual, not here.
- The founder's dated security exceptions for `hook.event`, `send`, `wait`, `last`, `answer`, `pause` and `release`
  stay as they are; this chunk widens none of them. Their owner is Epoch 6.

## Surfaces and contracts it touches
- `src/cmd/verify.rs`: the print-mode probe today, with `run_bounded` its only spawn. It gains the PTY drive.
- `crates/viola-agent-claude/src/ledger.rs`: `LedgerRow::ALL`, `check`, `merge_stamp`, `verified`, the capture plugin.
- `crates/viola-agent-claude/src/screen.rs`: the readiness signature format and the timing constants.
- `crates/viola-agent-claude/src/dialog.rs`: the S3 / S7 / S8 bodies.
- `viola-pty`: spawn, read, write and the bracketed paste, reused by verify.
- `send`'s outcome set: `unconfirmable`, the RB mirror. Plus `answer` / `hook` for the permission end to end.
- The fake agent and `fixtures/claude/<version>/`: replay of the new rows and screen fixtures.
- The stamp file `ledger/stamps.json`: verify stays its only writer.
- Specs: architecture [CLI Version Compatibility], [Screen Model], [Delivery Confirmation], [Hook Transport]; obs-plan
  §6 (subjects, the detail catalog); test-plan §6 / §10; security-plan (subprocess boundary, NEVER-log floor for
  recorded screen text).

## CI read at take-up (Setup 5a: every commit from the last wrap's flip `ed2edde` through HEAD)
- `caae9ec` (chore(route) 0-pending wrap): verdict not yet available at take-up (ci#37246734079 in progress). Re-read
  at P3: green, 15/15, wall 250 s (research M11).
- `ed2edde` (the running-turn-refusal feat): green, ci#37241676960, 15/15 checks, wall 339 s.
- Nothing red to fold.
