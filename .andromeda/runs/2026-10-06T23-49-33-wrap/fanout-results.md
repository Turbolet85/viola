# Fan-out results — 2026-10-06-local-command-send-outcomes

Seven doc-agents, one batch, the prompt of `amendment-flow.md` verbatim. Detector counts per prompt: architecture 2,
security-plan 3, design-system 1, layout-templates 1, test-plan 3, obs-plan 3, a11y-plan 2 = 15, the `doc:` names
over `drift-base.md`. Entity probe on every return: 0 entities (`<phase>`, `<n>`, `<name>` arrived literal).

Each list below is the return's proposal set: `section`, `change`, `basis` and `dependent-of` as returned; a long
`change` is kept whole; `rationale` is given by its cited report facts. The disposition and the check that
decided it follow each row.

## Verdicts
- architecture — 8 proposals.
- security-plan — `proposals: []`. Stripped: a comment block, three "no drift" verdicts with their bases, and one
  note (`security-plan.md:74` and `:223` state the prompt-text comparison, which still holds for an unlisted send;
  not proposed). Raw twin: `.raw-fanout-security-plan.md`.
- design-system — `proposals: []`. Stripped: a comment block, the "no drift" verdict, and site notes for the plan's
  ninth expected amendment (`design-system.md:11`, `:23`, `:850` name the matching `prompt-submitted` as the only
  thing that fills the box; `:149`, `:185`, `:572` already state the post-condition; Component Patterns 2 states no
  trigger). Raw twin: `.raw-fanout-design-system.md`.
- layout-templates — 1 proposal.
- test-plan — 7 proposals, plus three "no proposal" verdicts and a swept-and-left list (stripped, kept below).
- obs-plan — 8 proposals, plus two "no drift" verdicts (stripped, kept below).
- a11y-plan — `proposals: []`. Stripped: a comment block, two "no drift" verdicts with their bases. Raw twin:
  `.raw-fanout-a11y-plan.md`.

## architecture
A1. D-arch-decisions · warning · §Established Decisions → [Delivery Confirmation] · basis `architecture.md:49`
  - change: retire "but `send` does not consume them yet (owed to the "Local-command send outcomes" route entry),
    so today every local command, `/clear` included, still ends `not-delivered` / `no-prompt-submitted`, never
    `ok`"; state the landed mechanism: `send` consumes `LOCAL_COMMANDS`; exact-text classification (no trim, no
    case folding, never a leading slash) when the slot is reserved, every refusal rung applying to a listed
    command; no post-condition or an unverified version → `send-issued`, the paste, `unconfirmable` at once, no
    window (`SendSlot::new(clock, cli_verified, wheel)`, no stamps read); `/clear` on a verified version waits the
    window for its post-condition only, never claimed by a `prompt-submitted`; the tap claims on a `session-start`
    with `cause` `clear` and a string `agent_session_id` differing from the remembered one (the last
    `session-start`'s id, in memory under the in-flight lock; no string id clears it; nothing remembered → any
    string id is new; never persisted), appends unchanged, settles with its `ts` as `submitted_at`; a closed window
    is `not-delivered` / `no-prompt-submitted` with the cursor; the opening sentence and the claim sentence scoped
    to ordinary sends; keep the limit (proven on the recorded 2.1.287 `clear-1` variants; live proof owed to "First
    live test and self-drive").
  - rationale: report Symbols / APIs bullets 1-5, expected amendment 1, Deviations 3 and 4.
  - **apply** — check 1, playbook "Accurate this-chunk addition"; check 5, expected amendment 1.
A2. D-arch-decisions · warning · [CLI Version Compatibility] · basis `architecture.md:91` · dependent-of D-arch-decisions
  - change: "`send`'s use of the local-command list (`/clear`'s confirmation, `unconfirmable`) is owed to the
    "Local-command send outcomes" route entry, and the harness prefixes and the R8 identity floor to "First live
    test and self-drive"" → landed with chunk 2026-10-06-local-command-send-outcomes ([Delivery Confirmation]); the
    harness prefixes and the R8 identity floor are owed to "First live test and self-drive".
  - **apply** — check 1 (same rule), with its primary A1.
A3. D-arch-decisions · warning · §Standard Contracts → Channel methods (`hook.event`) · basis `architecture.md:286` · dependent-of D-arch-decisions
  - change: the relabel sentence gains its exception: a send that is a listed local command waiting for its
    post-condition is claimed by no `prompt-submitted`; a `session-start` with `cause` `clear` and a differing
    string `agent_session_id` settles it, appended unchanged first.
  - **apply** — check 1, with A1.
A4. D-arch-decisions · warning · [CLI Version Compatibility] · basis `architecture.md:91`
  - change: after "The readiness gate of `viola run` ([Screen Model]) is unchanged by this." add the reading as
    measured under the fake agent only (`evidence/paste-hint-send.md`): a `send` on a verified CLI while the hint
    stands ends `not-delivered` / `input-not-ready` with nothing typed, the same sequence without the hold is
    delivered; no `send` was run in the real CLI's hint window; what `send` should do while the hint stands is the
    founder's open decision.
  - **apply** — check 1; check 5, expected amendment 3; the open decision is the overseer's disposition 1, relayed
    by the operator at this wrap. Nothing is decided by the text.
A5. D-arch-resources · warning · §Standard Contracts → Event `data` per kind · basis `architecture.md:303`
  - change: `send-confirmed`: `{cursor, confirmed?}` — `{cursor}` for a confirmed send, `{cursor, confirmed:false}`
    for an unconfirmable one, the one record of that outcome; additive, no `v` bump; no product reader.
  - **apply** — check 1; check 5, expected amendment 2.
A6. D-arch-resources · warning · §Occupied Resources → Binary, subcommands and exit codes · basis `architecture.md:355`
  - change: "seven argv options" → "eight"; the list gains `--tag-turn-screen <phase>`.
  - **apply** — check 1; check 5, expected amendment 4.
A7. D-arch-resources · warning · §Infrastructure Patterns → Project directory structure · basis the key file `project-directory-structure.md:21`
  - change: the `human.rs` line's mirror list gains `[  ] unconfirmable`.
  - **apply** (a key-file edit) — check 1; check 5, expected amendment 5.
A8. D-arch-resources · warning · §Infrastructure Patterns → Project directory structure · basis the key file `:28` · dependent-of D-arch-resources
  - change: the `send.rs` line names the local-command decision, the remembered session id and the
    post-condition claim.
  - **apply** — check 1 (an accurate this-chunk addition inside an existing line).

## layout-templates
L1. D-layout-surface · warning · §Surface: cli → Signature placement, item 2 · basis `layout-templates.md:351`
  - change: `[RB] read back …` on stdout with exit 0 when the send is confirmed, by the matching
    `prompt-submitted` or, for `/clear` on a verified CLI version, by its new-session post-condition; the `unable`
    and `unconfirmable` clauses stay. Sweep by the detector: `:50` and `:221` already carry the `session-start`
    trigger; the wireframe's `clear.txt` example ending `input-not-ready` stays valid.
  - **apply** — check 1; check 5, expected amendment 8.

## test-plan
T1. D-tests-coverage · warning · §6 Path 2 → Verification signal, the `local` bullet · basis `test-plan.md:750`
  - change: the verdicts as landed (exit 0 `unconfirmable` with its records; `/clear` on a verified CLI confirmed
    by its post-condition, with the stated limit; the off-list slash text exit 13; the forced-window pair).
  - **apply** — check 1; check 5, expected amendment 6.
T2. D-tests-coverage · warning · §1 Critical paths → Path: confirmed `send` · basis `test-plan.md:233` · dependent-of D-tests-coverage
  - change: the "does not consume them yet … until then `not-delivered`" sentence replaced by the landed rule.
  - **apply** — with T1.
T3. D-tests-coverage · warning · §6 Path 2, the Playwright bullet · basis `test-plan.md:751` · dependent-of D-tests-coverage
  - change: "once "Local-command send outcomes" makes `send` return it" retired; the web half stays owed to `:147`.
  - **apply** — with T1.
T4. D-tests-coverage · warning · §6 Path 2, the Human mode bullet · basis `test-plan.md:752` · dependent-of D-tests-coverage
  - change: add the unconfirmable line (stdout, exit 0, nothing on stderr) and that a post-condition-confirmed
    `/clear` prints `[RB] read back`.
  - **apply** — with T1.
T5. D-tests-coverage · warning · §4 root bin, the send-confirmation matcher bullet · basis `test-plan.md:581` · dependent-of D-tests-coverage
  - change: add the local-command decision's unit cases (`send_local_command_`, five functions, 13 cases) and the
    fourth mirror writer.
  - **apply** — with T1.
T6. D-tests-coverage · warning · §7 Fake agent → the Modes bullet · basis `test-plan.md:1077`
  - change: "seven argv options" → "eight"; the list gains `--tag-turn-screen <phase>`. The detector's note: the
    report's Counts bullet said this line carried no count, and it does.
  - **apply** — check 1; check 5, expected amendment 6. The report's bullet was corrected in place at P2.
T7. D-tests-coverage · warning · §5 CLI → the `tests/cli_verify.rs` bullet · basis `test-plan.md:654` · dependent-of D-tests-coverage
  - change: add the tag-turn modal case beside the turn-screen modal case.
  - **apply** — check 1 (an accurate addition at the bullet's own grain: it already names the first modal case).
- No proposal: D-tests-framework (nextest through the harness, rstest tables, no dependency); D-tests-obs-harness
  (no harness, status or log-format change; the unconfirmable send's lines fit `§3 → Log format` as written).
- Swept and left by the detector: `:114`, `:152`, `:707`, `:240`, `:749`, `:286`, `:588`, `:653`, `:740`, `:1063`,
  `:1337`.

## obs-plan
O1. D-obs-instrumentation · warning · §4 Scenario Confirmed `send` (CL-1) → Cleanup · basis `obs-plan.md:648`
  - change: `run.confirm_window` closes on the confirmation its send waits for, or on expiry: the matching
    `prompt-submitted{origin:"driver"}` for an ordinary send, the `session-start` with `cause` `clear` and a new
    string id for `/clear` on a verified CLI; expiry is `send-refused{detail:"no-prompt-submitted"}` for either; an
    unconfirmable send opens no `run.confirm_window`.
  - **apply** — check 1; check 5, expected amendment 7.
O2. … → Must-trace spans · basis `obs-plan.md:632` · dependent-of D-obs-instrumentation — the chain ends at
  `pty.paste_write` for an unconfirmable send; the hook side is the session-start hook for a post-condition `/clear`.
  - **apply** — with O1.
O3. … → Required log fields, the hook bullet · basis `obs-plan.md:642` · dependent-of D-obs-instrumentation —
  `hook-invoked{hook_event:"session-start"}` for a post-condition `/clear`; none awaited for an unconfirmable send.
  The detector's caveat: the report did not measure that line on this path.
  - **apply**, the hook line stated on its own basis: `src/cmd/hook.rs` logs `hook-invoked` with the invoked
    event's own `hook_event` for every hook event (read at this wrap, lines 152-163); the body says "as landed",
    not "measured on this path".
O4. … → Surfaces involved · basis `obs-plan.md:631` · dependent-of D-obs-instrumentation — **apply** — with O1.
O5. §1 Obs Scope Summary → Path: Confirmed `send` → Must-trace spans · basis `obs-plan.md:313` · dependent-of D-obs-instrumentation
  - **apply** — playbook "Verbatim upstream copy kept current" (routine: judged like any body amendment).
O6. §1 … → Surfaces involved · basis `obs-plan.md:312` · dependent-of D-obs-instrumentation — **apply** — as O5.
O7. §3 → Snapshot / paste-to-AI integration · basis the key file `snapshot-paste-to-ai-integration.md:13` · dependent-of D-obs-instrumentation
  - change: the hook-side join covers the session-start post-condition; an unconfirmable send has no window and
    no hook side.
  - **apply** (a key-file edit) — with O1.
O8. D-obs-instrumentation · warning · §5 Per-surface / per-path metrics, the `viola.send.confirm_ms` row · basis `obs-plan.md:763`
  - change: the derivation takes the `send-confirmed` lines whose `confirmed` is not `false`; an unconfirmable
    line carries `duration_ms` and had no window, so it is no send → readback sample.
  - **apply** — check 1, "Accurate this-chunk addition": the row defines the metric as the `open` → `read back`
    latency, and this chunk is what first writes a `send-confirmed` line that is not a read back. Not on the
    plan's list; named at the wrap card.
- No drift: D-obs-stack (no dependency); D-obs-pii (codes only; the command's text in no line).

## Raised by the orchestrator (check 5)
E9. design-system — the plan's ninth expected amendment ("§Surface: cli → Component Patterns 2 — the same trigger;
  no new word") drew no proposal. The detector's notes place the claim: Component Patterns 2 states no trigger, and
  three sites say the matching `prompt-submitted` alone fills the box (`design-system.md:11`, `:23`, `:850`).
  - **apply**, routine: the report substantiates it (a post-condition-confirmed `/clear` prints `[RB] read back`),
    and `:149`, `:185`, `:572` of the same master already state that trigger. The three sites gain it; no new word.

## Checks with nothing to decide
- Check 2, cross-contradiction: none. A2 and A4 edit different sentences of one paragraph, in one direction.
- Check 3, intent: the report's seven deviations are each justified; the scope record is empty and `gate.py scope`
  is clean; no boundary is widened (the same text is typed, the tap reads two more fields of an event it already
  accepts, the new option is in the test-only binary).
- Check 4, absence: the detectors' "single site" and "none" claims are re-checked by the cascade sweep
  (`cascade-dispositions.md`).
- Check 6, disproved claims: the report lists none.
- Escalations: 0.
