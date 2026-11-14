# Answer card — wrap of 2026-11-14-hyprland-seam (run 2026-11-14T09-29-36-wrap)

The founder's own answers to the P4 forks, on record before this Validate (`inputs#I2`, relayed verbatim by the overseer):

- **Q1 (Transport; playbook escalate pattern «Boundary widening — a subprocess/IPC boundary gains a new crossing»):**
  «hyprctl -j, direct spawn (Recommended)» — the founder's own live word, 07:41Z 2026-11-14: «hyprctl -j, прямой запуск».
- **Q2 (Recorder; same pattern, precedent viola-mcp-tap):** «Yes, titles replaced (Recommended)» — the founder's own
  live word, 07:41Z 2026-11-14: «Да, заголовки окон заменены». The overseer's P5 review (`inputs#I3` item 1) narrowed it
  to a closed keep-list: it narrows what is committed and widens nothing.

The operations main reads as lying wholly inside those answers:

- `ops-architecture.md · decision/hyprctl-direct-spawn` — area standard-contracts — `fit: fits type 1` — hyprctl is
  spawned directly from a caller-resolved path, no shell, no PATH lookup, stdin null, a closed argv; a dispatch is the
  measured Lua form `hl.dsp.focus`, its only inserted value a closed window address (`0x` + 1–16 lower-case hex) or an
  `i32` workspace id, never reply text (inside Q1; the Lua form measured at L2 on 0.56.2, `93b2ca1d`,
  `fixtures/hyprland/0.56.2/FocusWindow.recorded-1.json`).
  Grouped with it (check 2, one home): `ops-security-plan.md · decision/hyprctl-caller-resolved-program` and
  `· decision/hyprctl-dispatch-closed-values` — rejected, their fact's home this row.
- `ops-main-security-plan.md · decision/hyprland-recorder-string-allowlist` — area data-protection — `fit: fits type 1`
  — viola-hypr-tap keeps a reply string only under a closed per-request key allowlist in the seam's parsed form, every
  other string the placeholder, a non-JSON reply only as `ok`, then the shared scrub, an unclean set refused whole
  (inside Q2 as narrowed by `inputs#I3` item 1).
  Grouped with it (check 2, one home): `ops-architecture.md · decision/hyprland-fixture-keep-list`,
  `ops-security-plan.md · decision/hyprland-fixtures-closed-keep-list`, `ops-obs-plan.md ·
  decision/hyprland-fixtures-keep-list`, `ops-test-plan.md · decision/hyprland-replies-keep-list` — rejected, their
  fact's home this row.

Count of the card's lines: 2.

Named beside the card, never among what it asks: no escalated operation read `not_master`.

## The answer
Answer (the founder's own word, relayed by overseer1, verbatim from `../additional/viola-overseer/relay-wrap-hyprland-q1.md`):

> # Wrap of 2026-11-14-hyprland-seam — the answer to founder-forks.md Question 1 (the answer card)
>
> **The founder's own word, given in overseer1's question dialog at 2026-11-14T10:58:23Z, relayed by overseer1 and read
> by the Viola overseer in that session's transcript (`5480a236-a7e0-4f66-9641-91fd2a987e1f`, the dialog's result line).**
>
> The question put to him named both rows as the card does — `architecture · decision/hyprctl-direct-spawn`, with the Lua
> dispatch form `hl.dsp.focus({ window|workspace = … })` measured at L2 on 0.56.2 and said to be unknown when he answered
> Q1, its inserted value a closed address or an `i32`; `security-plan · decision/hyprland-recorder-string-allowlist`, the
> closed keep-list as P5 narrowed it — with four options: confirm both · only row 2 · only row 1 · both outside.
>
> His selection, verbatim: «Подтвердить обе (Recommended)»
>
> So: option 1 of the card, Confirm — both rows lie inside his P4 answers and land as his ratification.
>
> With this and the corrected Question 2 answer (the five rows land as the proposer wrote them, `area=standard-contracts`)
> the wrap resumes at Escalate and runs to its end.
