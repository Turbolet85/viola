# Cascade compare — a11y-plan · phase · 2026-10-26-server-verification-before-any-frame
E items 55 · held 6 · lost at classify 0 · lost at strong 0 · beside a placed fact - · cascade only 2 · listed 3 · body 32 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

## Listed — the full read carried these and the cascade dropped them as standing conventions: they reach the planner on the standing list, never as items; the blind judge reads them here

### cognitive-accessibility#3.1 · listed (lost at strong) · .andromeda/a11y-plan-detail/cognitive-accessibility.md:26
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:26) Each verb's exit-21 verification refusal must be pinned: the `unable … hint:` layout by trycmd or literal, the typed exit code 21 by assert_cmd, and its `--json` form shown to carry no hint text, on all three OS legs.
  > - **cli:** trycmd cases pin the `unable <reason> <detail>` + final `hint:` layout per refusal

### cognitive-accessibility#3.1 · listed (lost at strong) · .andromeda/a11y-plan-detail/cognitive-accessibility.md:26
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:26) The exit-code set the CI checks includes 21, and the `--json` hint-free assertion covers the changed verbs.
  > and assert_cmd checks the typed exit codes (`10`–`14`, `20`, `21`) and that `--json` carries no hint text. These run on all three OS legs.

### cognitive-accessibility#3.1 · listed (lost at strong) · .andromeda/a11y-plan-detail/cognitive-accessibility.md:26
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:26) The instance-unreachable `--json` document shape already pinned for `viola list` is the shape the changed verbs' unreachable document must stay consistent with (stdout one document, stderr empty).
  > under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty

### cognitive-accessibility#4.1 · listed (lost at strong) · .andromeda/a11y-plan-detail/cognitive-accessibility.md:35
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:35) If the chunk changes any fixed refusal or hint string of these verbs, the pinned string must be updated on purpose in the same change, or CI fails.
  > trycmd pins every fixed CLI string, except `viola list`'s, which literals pin

### cognitive-accessibility#4.1 · listed (lost at strong) · .andromeda/a11y-plan-detail/cognitive-accessibility.md:35
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:35) The pin is the gate for copy drift.
  > Any copy change therefore fails CI until the pinned string is updated on purpose.

### visual-design-verification#1.1 · listed (lost at classify) · .andromeda/a11y-plan-detail/visual-design-verification.md:9
- (.andromeda/a11y-plan-detail/visual-design-verification.md:9) The refusal a CLI verb (`send`, `wait`, `last`, `answer`, `pause`, `release`) prints when the server verification fails or cannot be made must carry no colour on its `unable` word, the same as every other refusal.
  > `unable` / `fail` are never coloured.

### visual-design-verification#1.1 · listed (lost at classify) · .andromeda/a11y-plan-detail/visual-design-verification.md:9
- (.andromeda/a11y-plan-detail/visual-design-verification.md:9) The new mismatch path in every client, its `--json` document and its stderr pair included, must write no SGR byte of viola's own in any of the named modes.
  > viola emits zero SGR of its own under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json` / `viola run` / a passed `viola revive`

## Held

- a11y-scope-summary#1.1
- a11y-scope-summary#2.1
- a11y-scope-summary#3.1
- aria-patterns-roles#2.1
- aria-patterns-roles#3.1
- cognitive-accessibility#2.1

## Cascade only

- a11y-scope-summary#4.1
- cognitive-accessibility#1.1
