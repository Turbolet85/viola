# a11y · Visual Design Verification — category read

## Carried
- (.andromeda/a11y-plan-detail/visual-design-verification.md:8) The SGR spans stay these three and no others: a hint naming a session, the unwrapped-name hint among them, must write that name plain, because bold belongs only to NAME column values.
  > **CLI equivalent (no conformance claim):** amber SGR only on the `DIALOG` word, dim SGR on stale rows and bold SGR only on NAME column values (the callsign; design cli), always beside the word.
- (.andromeda/a11y-plan-detail/visual-design-verification.md:8) Every `--json` document and MCP-bound code this chunk adds or moves must leave the colour decision unchanged, with no SGR in any document of any cause.
  > it writes no SGR under `--json`, a passthrough role or any of those rungs
- (.andromeda/a11y-plan-detail/visual-design-verification.md:9) Every exit-21 and exit-1 cause's human output, including `run`'s start refusals after a passed preflight and each revive refusal pair that joins the catalogue, must hold this no-SGR rule and carry no ESC byte.
  > viola emits zero SGR of its own under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json` / `viola run` / a passed `viola revive`, and the revive refusal pairs and `--list` rows carry no ESC byte.
- (.andromeda/a11y-plan-detail/visual-design-verification.md:9) Each cause's refusal text must leave `unable` uncoloured, and this no-SGR discipline stays checked with these tools on all three OS legs when the refusal lines or their tests move.
  > `unable` / `fail` are never coloured. Checked with assert_cmd + portable-pty on `[windows-2025, macos-latest, ubuntu-latest]`.
- (.andromeda/a11y-plan.md:340) The body's Visual ban applies to every refusal, hint and document the catalogue touches, so no cause may be told apart by colour or carry viola's SGR on these rungs.
  > - NEVER colour CLI `unable` / `fail`, and never emit SGR under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json` / `viola run` / a passed `viola revive`.

## Bindings
- **No-SGR refusals ↔ the body's §6 and §11 Visual**: the detail's zero-SGR and no-ESC rule (.andromeda/a11y-plan-detail/visual-design-verification.md:9) is the same requirement as the §6 broad line (.andromeda/a11y-plan.md:221) and the Visual ban (.andromeda/a11y-plan.md:340); a catalogue change must satisfy all three.
- **Plain refusal text ↔ the body's §8 Error recovery**: the uncoloured `unable` (.andromeda/a11y-plan-detail/visual-design-verification.md:9) sits in the `unable` then final `hint:` layout each cause keeps (.andromeda/a11y-plan.md:257), so a cause's distinct hint is carried by its words alone.
- **Three-leg checks ↔ the body's §9 CI Integration**: the assert_cmd + portable-pty checks (.andromeda/a11y-plan-detail/visual-design-verification.md:9) run inside the existing per-OS `coverage` suite (.andromeda/a11y-plan.md:300), with no second driver and no new `suite` value (.andromeda/a11y-plan.md:301), which holds for any output-discipline case the exit-cause table adds.
