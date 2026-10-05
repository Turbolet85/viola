# Scope — 2026-10-05-dialog-rows-and-re-probe

**Working entry** (`working-route.md:86`, Epoch 3 — Windows slice II: driving verbs and live proof):
Dialog rows and re-probe — S3/S7/S8 and dialog-concurrency ledger rows with their own interactive verify re-probe,
the decision taking effect, the permission end-to-end case.

## Intent
The capability ledger gains its dialog tier: the S3 / S7 / S8 decision-body rows and the dialog-concurrency row, each
with a post-condition measured by a `viola verify` re-probe that drives the interactive PTY run the previous chunk
landed (Run B), on the live installed `claude` on the Linux dev host. The re-probe records real dialog payloads, which
supersede the relayed 2.1.287 dialog fixtures, and records that a decision takes effect. The founder's dated gap R2
closes: a non-`null` dialog decision then needs the dialog rows to pass in the child version's stamp, not the ten-row
stamp alone. The `permission` kind gets its end-to-end case from a recorded ordinary-tool PermissionRequest. viola stays
mechanism: the probe measures and stamps, it decides nothing for a driver.

## Authority
- The founder's three-way split ruling, live, 2026-10-05 05:58Z, relayed by the overseer (verbatim in
  `.andromeda/runs/2026-10-05T00-16-17-phase/relay-2.md`, cited by `2026-10-05-real-cli-verify-probes/scope.md`): this
  entry is W3 + W6 of that chunk's scope (its scope CARRYs 6, 7 and 9), ahead of "Local-command and paste-framing rows"
  and "First live test and self-drive". Epoch 3 stays one epoch.
- The founder's rulings R1 (~11:55Z) and R2 (~14:08Z), live, 2026-10-04, relayed by the overseer (chunk
  2026-10-04-dialog-answers-by-dialog-id): the dialog rows and their own re-probe land here, superseding the relayed
  fixtures; non-`null` decisions flow on the spine stamp until then (R2's dated gap); `verification-matrix.json#v1-15`
  is claimed here.
- The founder's 2026-10-05 rulings on the previous chunk's probe (`relay-3.md`, `relay-4.md`, `relay-6.md`,
  `relay-7.md` in that chunk's phase run dirs), standing: two runs, never accept; viola never types a byte into a
  CLI-native dialog (trust, external imports); a modal start is killed with no key; Run A's OS-temp dir and Run B's
  `<cwd>/.viola-verify-<pid>/` are 0700 and removed on every exit path; viola writes nothing under `~/.claude` (the CLI's
  own Run B transcript is the accepted residual).
- The operator's directive at this take-up (inputs#I1):
  - bring the M7 widening to P4 as a founder card with every option priced, including a hook-answers path that never
    types into a CLI dialog;
  - measure what can be measured before the card;
  - size the chunk against one builder window;
  - name the live `claude` sessions it needs — a new cap is the founder's decision (the previous chunk's cap of 18,
    16 used, was that chunk's own);
  - Epoch 3 stays unsplit.

## The founder's live rulings at P4 (2026-10-05, via the overseer's AskUserQuestion, relayed by the operator; inputs#I3)
The card was shown with every option priced, the crossing and the residual shown:
- **M7 path → A, hook answers.** The capture arm prints a product-built decision body, never a key into any dialog.
  The probe's `allow` runs one synthetic `touch` in Run C's own 0700 dir. Each verify run adds +2 CLI transcripts
  under `~/.claude` (the accepted Run B residual class). This is a boundary widening, ratified live (playbook
  "Boundary widening — what ratifies it").
- **Live cap → 16**, both installed versions stamped (2.1.288 and 2.1.287).
- **Sizing → split.** W3d and W6 leave this chunk for a new Epoch 3 entry right after it, "Permission end to end",
  minted at this chunk's wrap through route-resolve. Epoch 3 stays one epoch, now at 14 entries.

## Scope after the rulings
- **Built here:** W3a, W3b and W3c, through path A. That covers the four rows, the interactive dialog re-probe, the
  decision taking effect, the R2 closure, the 2.1.288 / 2.1.287 dialog recordings (superseding the relayed set as
  measured truth), and the fake-agent replay CI needs to stamp the grown ledger.
- **Moved to "Permission end to end"** (the new entry, recorded as freight below and re-homed by this chunk's wrap):
  - W3d: the `permission` Path 4 case and its `v1-30` wake witness, over the ordinary-tool PermissionRequest fixture
    this chunk records, and the question's PermissionRequest body;
  - W6: the `tests/cli_answer.rs:9` reword.

## What it builds (the five CARRYs folded, in route order)

### CARRY 1 — the split's own record
- This entry is W3 + W6 of `2026-10-05-real-cli-verify-probes`' scope, minted at its wrap on the founder's three-way
  split (05:58Z). The sibling "Local-command and paste-framing rows" (`:88`, W2 + W4) follows it; "First live test and
  self-drive" (`:90`) follows both. Epoch 3 stays one epoch (now 13 entries).
- The sizing anchor (that chunk's CARRY 1): `:78`, a lighter load, ran its builder to 86.7 % of the window; the
  previous chunk, W1 + W5 alone, needed three plan revisions and 16 live sessions (`evidence/live-sessions.md`).

### W3a — the dialog-tier ledger rows (CARRY 2)
- Four new capability-ledger rows, each with a `viola verify` probe and a post-condition:
  - **S3** — AskUserQuestion is answered through PreToolUse `updatedInput` (architecture [CLI Version Compatibility]);
  - **S7** — ExitPlanMode approves only through PreToolUse; revise goes through `deny` + `message`. Its
    `plan-approve-revise` post-condition must pass in a real stamp (`verification-matrix.json#v1-15`, claimed here,
    acceptance unweakened — R2);
  - **S8** — a question's free text and `annotations`;
  - **dialog concurrency** — whether two dialog hooks (PreToolUse `AskUserQuestion|ExitPlanMode`, PermissionRequest)
    can be open at once in one session, for example on parallel tool calls (architecture's measured-rows list).
- `LedgerRow::ALL` grows from 10 to 14 and the step counter from `/10` to `/14`; every stamped home re-stamps, and
  the two installed versions (2.1.288, 2.1.287) need new stamps (verified, research M4 / M11 / M13: `verified` reads
  every `ALL` row, 37 row-count literals in 4 files, both versions installed).
- R2's dated gap closes: a non-`null` dialog decision then requires the dialog rows to pass in the child version's
  stamp. CLAUDE.md's "one dated exception: the S3/S7/S8 dialog bodies ride the ten-row stamp until the route entry
  'Dialog rows and re-probe'", architecture's matching dated exception and security-plan's ten-row residual close with
  it (wrap amendments).
- [premise-corrected: research M4 — `run` hands one `cli_verified` both to the dialog gate and to the readiness gate's
  signatures (`src/cmd/run.rs:221-229`, `:492`)] The verdict's shape is a P4 design question with two sides, not only
  the keying: all rows in the one `cli_verified` (a dialog-row failure then also switches the screen readiness gate
  off for that version), or a split verdict (spine + screen rows gate typing, the dialog rows gate decisions — a new
  snapshot / obs field under default-deny). Per-kind keying is a refinement of the split.

### W3b — the interactive re-probe (CARRYs 2 and 4)
- The dialog rows get their OWN `viola verify` re-probe (R1). It drives the interactive PTY run the previous chunk
  landed (Run B, trusted, `<cwd>/.viola-verify-<pid>/`), because a print-mode turn raises no dialog hook — measured on
  2.1.287 (`2026-10-04-dialog-answers-by-dialog-id/evidence/print-mode-dialog-probe.md`) and re-stated on 2.1.288
  (research M8 of the previous chunk). [premise-corrected: research M8 — the previous chunk's M8 was measured on
  2.1.287 only; "2.1.288" in the CARRY belongs to its M7] On 2.1.288 print mode is not measured live; statically
  AskUserQuestion is enabled whenever the session is interactive (`KJ()`, research M6). The re-probe is interactive
  either way, so nothing turns on it.
- The re-probe must make the model raise AskUserQuestion, ExitPlanMode (plan mode) and an ordinary-tool
  PermissionRequest on demand, and a probe that fails to raise its dialog reads as a row failure, never a pass
  (verified as a requirement: test-plan §1 / §10, the arch extract). [premise-corrected: research M9 — Run B inherits
  the host user's settings, and the dev host allows `Bash(*)` / `Edit(*)`, so an ordinary tool raises no
  PermissionRequest there] The ordinary-tool PermissionRequest needs a session-scoped `ask` rule (`--settings`) or
  only the named setting sources (`--setting-sources`), never the host's allow list; the model's nondeterminism under
  `--model haiku` is measured at implement's step 0, not here (no live session at this phase).
- The re-probe's recorded payloads supersede the relayed 2.1.287 dialog fixtures (`fixtures/claude/2.1.287/RELAYED.md`,
  four files). The relayed set is retired (or kept and marked superseded — P4's choice) and the `--record`
  scrub/refusal covers the dialog payloads, whose `tool_input` can carry plan text and paths (verified: `ledger::scrub`
  walks every string and key recursively, `ledger.rs:371-392`).

### W3c — the decision taking effect (CARRYs 2 and 4: research M7, the crossing for the founder)
- The decision effect's first half — the decision takes effect — is recorded by the re-probe. Its second half, the
  dialog never rendering (`verification-matrix.json#v1-31`), stays with "First live test and self-drive".
- Measured at the previous chunk (research M7, kept verbatim as a claim to re-verify): "the capture arm answers
  nothing, so 'the decision takes effect' needs either a probe hook that also answers or the probe keying the rendered
  dialog — both crossings beyond R-S2's words and against the founder's 2026-10-05 'never type into a CLI dialog'
  ruling as worded". Re-verified at HEAD (research M1): `hook --capture` writes nothing to stdout and exits 0
  (`src/cmd/hook.rs:89-91`, `:344-353`). New at P3 (research M2): the arm's capture naming is not concurrency-safe
  (`free_k` then a rename over the target), so two dialog hooks capturing at once can lose a payload — the
  concurrency row needs exclusive naming whatever the ruling.
- Measured before the card (operator directive; research M6 / M7, `inputs#I2`): the prototype answered 25 of 25
  dialogs through a PreToolUse `allow` + `updatedInput` on interactive 2.1.287 with no PermissionRequest after, and a
  PermissionRequest `deny` + `message` on ExitPlanMode was followed by a re-plan; 2.1.288's binary applies a hook
  `allow` on a user-interaction tool and maps `updatedInput.answers` / `annotations`. Unmeasured: 2.1.288 live,
  viola's own bodies (its plan approve carries no `updatedInput`, unlike the prototype's), concurrency, an
  ordinary-tool PermissionRequest.
- **gate: the founder's ruling on the M7 widening** — the operator's directive brings it to P4 as a founder card, every
  option priced (code, live sessions, boundary crossing, residual), including a hook-answers path that never types
  into a CLI dialog. Until it is ruled, nothing that makes a decision take effect in a live session is planned past the
  card. The card's options are measured before it is shown (operator directive).

### W3d — the `permission` end to end and the question's PermissionRequest (CARRY 3) — MOVED to "Permission end to end"
- This chunk only records the input W3d needs: the ordinary-tool PermissionRequest fixture, from Run C's synthetic
  `touch`.
- The `permission` kind's end-to-end Path 4 case and its `verification-matrix.json#v1-30` wake witness, from a recorded
  ordinary-tool PermissionRequest. At HEAD the kind is unit / insta only (`dialog::tests`; test-plan's Path 4 note).
- The PermissionRequest body for a `question` first raised by PermissionRequest: unmeasured on 2.1.287 and left `null`
  to the human at HEAD. "Measure it" is this entry's work; whether a body is then built from it, or the `null` stays
  with a recorded reason, follows the measurement (verified open: the prototype's 3 AskUserQuestion PermissionRequests
  each followed an unanswered PreToolUse, so none was first raised by PermissionRequest — `inputs#I2`).

### W6 — reword a stale name (CARRY 5) — MOVED to "Permission end to end"
- `tests/cli_answer.rs:9` names the `permission` end-to-end case's owner "the live test (working-route `:84`)". Reword it
  to this entry's name and coordinate when this entry touches that file (W3d does).

## Sizing and live sessions (operator directive)
- One builder window. If W3a–W3d and W6 do not fit, P4 brings a split card rather than planning past the window.
  Epoch 3 stays one epoch: a split mints entries inside Epoch 3 (the founder's 05:58Z precedent), never a new epoch
  boundary.
- Every live `claude` session the plan needs is named and counted in the plan; the cap is the founder's decision.
  Counting convention (verified, research M13): a full `viola verify` run spends three sessions (the print-mode
  probe, Run A, Run B), so a stamp of each installed version costs at least three, plus one per interactive dialog
  run this chunk adds.

## Boundaries (not this chunk)
- "First live test and self-drive" (`:90`) owns: "the dialog never renders" (`verification-matrix.json#v1-31`); where
  its live proof runs; the Windows-only live items (H2, the DA1 stall, F-W3's real-terminal mouse report); the Epoch 3
  boundary audit's Windows-dispatch survivors; `local-live`'s live firing.
- "Local-command and paste-framing rows" (`:88`) owns: the local-command rows and `unconfirmable`, the long-paste
  wrapper, tag escaping, the harness prefixes, the R8 identity floor.
- No Claude credential on any CI runner; the live probe is local to the Linux dev host (R-S2). CI stamps the new rows
  against the fake agent's replay.
- R-S3: the U02 setup upgrade and the `host-win32.md` regenerate run at the Epoch 3 boundary ritual, not here.
- The founder's dated security exceptions for `hook.event`, `send`, `wait`, `last`, `answer`, `pause` and `release`
  stay as they are; this chunk widens none of them (owner: Epoch 6).

## Surfaces and contracts it touches
- `crates/viola-agent-claude/src/ledger.rs` — `LedgerRow::ALL`, `check`, `Probes`, the capture plugin, `scrub` /
  `is_clean`.
- `crates/viola-agent-claude/src/dialog.rs` — the S3 / S7 / S8 bodies and the PermissionRequest mapping.
- `src/cmd/verify.rs`, `src/cmd/verify/typed.rs` — the interactive runs, the re-probe.
- `src/cmd/hook/` — the hidden `--capture` arm, if the ruling widens it.
- The wrapper's dialog gate (`cli_verified`, `unverified-cli`) — the R2 gap's closure.
- The fake agent and `fixtures/claude/<version>/` — dialog fixture replay; `RELAYED.md`.
- `tests/cli_answer.rs` — the `permission` end-to-end case (W6).
- `ledger/stamps.json` — verify stays its only writer.
- Specs: architecture [CLI Version Compatibility], [Hook Contract], [Human Takeover / Wheel] (the dialog hand-back);
  security-plan (the dialog-decision rule and its residual, the subprocess boundary, NEVER-log floor for recorded
  payloads); test-plan §6 Path 4 and §10; obs-plan §6.

## CI read at take-up (Setup 5a: every commit from the last wrap's flip `75198e5` through HEAD)
- `75198e5` (the 2026-10-05-real-cli-verify-probes wrap): green, ci#37301006304, 15/15 checks, wall 326 s.
- Nothing red to fold.
