# a11y · Visual Design Verification — category read

## Carried
- (.andromeda/a11y-plan-detail/visual-design-verification.md:8) The `viola link` / `viola unlink` result lines (`link … since …Z`, `link … already linked`, `unlink …`) and their refusals must not add a styled span: viola's SGR set is closed to the `DIALOG` word, stale rows and the NAME column of the board, so a driver or driven name printed on a link result line is plain text, never bold.
  > amber SGR only on the `DIALOG` word, dim SGR on stale rows and bold SGR only on NAME column values (the callsign; design cli), always beside the word.
- (.andromeda/a11y-plan-detail/visual-design-verification.md:8) The two verbs' `--json` mode must carry no SGR, whatever the terminal and colour facts.
  > it writes no SGR under `--json`, a passthrough role or any of those rungs
- (.andromeda/a11y-plan-detail/visual-design-verification.md:9) The new verbs must emit zero SGR of their own on non-TTY, `NO_COLOR`, `TERM=dumb` and `--json` output, and their `unable` refusal lines (e.g. exit 21 `instance-unreachable`, a refused name) must never be coloured; the verbs' output-discipline cases run with assert_cmd on all three OS legs.
  > viola emits zero SGR of its own under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json` / `viola run` / a passed `viola revive`
- (.andromeda/a11y-plan-detail/visual-design-verification.md:9) The `unable` word on a `link` / `unlink` refusal stays uncoloured, and the check is assert_cmd across the three-OS matrix.
  > `unable` / `fail` are never coloured. Checked with assert_cmd + portable-pty on `[windows-2025, macos-latest, ubuntu-latest]`.
- (.andromeda/a11y-plan.md:221) The broad line the verbs must honour: every state the result lines convey (linked, already linked, unlinked) is carried by its printed word, and no SGR of viola's own appears in the non-TTY / `--json` rungs.
  > always beside the word, each state still carried by its word; no SGR of viola's own under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json` / `viola run` / a passed `viola revive`; `unable` / `fail` never coloured
- (.andromeda/a11y-plan.md:338) The `already linked` versus `since …Z` distinction must be conveyed in text, never by colour.
  > - NEVER convey information by color alone (state indicators must
  >   have text / icon supplement per SC 1.4.1)
  > - NEVER colour CLI `unable` / `fail`, and never emit SGR under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json` / `viola run` / a passed `viola revive`.

## Bindings
- **visual-sgr (closed SGR set) ↔ design-system §Surface: cli Tokens**: the amber / dim / bold tokens are the only SGR viola writes and are confined to `viola list`; the link verbs' result lines take none of them, (.andromeda/a11y-plan-detail/visual-design-verification.md:8)
- **no-SGR rungs and uncoloured `unable` ↔ a11y-plan §8 Error recovery (cognitive-errors)**: the verbs' refusals keep the `unable <reason> <detail>` + `hint:` layout in plain text, with no colour on `unable`, (.andromeda/a11y-plan.md:340)
- **assert_cmd three-OS check ↔ test-plan (per-OS `test` job, `coverage` suite)**: the verbs' no-SGR / plain-refusal cases run inside the existing suite on `[windows-2025, macos-latest, ubuntu-latest]`, with no new `suite` value, (.andromeda/a11y-plan-detail/visual-design-verification.md:9)
