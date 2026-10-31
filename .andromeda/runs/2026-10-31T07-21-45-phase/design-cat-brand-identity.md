# design · Brand Identity — category read

## Carried
- (.andromeda/design-system-detail/brand-identity.md:21) Every CLI `--json` error document this chunk fixes must stay at expression 0.0, with no styling byte inside or around it.
  > | cli, `--json` / non-TTY / `NO_COLOR` / `TERM=dumb` / `viola run` passthrough | 0.0 | No colour, no glyphs, no cursor control, no spinner.
- (.andromeda/design-system.md:21) Every code and fixed message the chunk names on a refusal or error must be a short fixed word of the project's phraseology, never filler or a guess.
  > **Standard phraseology and "unable" plus a reason.** Short fixed words (`open`, `RB`, `unable`, `human-typing`, `budget-paused`, `not-delivered`) are the vocabulary of every label, CLI column and refusal. There is no filler and no guessing.
- (.andromeda/design-system.md:382) This ban is the chunk's property on the CLI: no error or hint may carry upstream text, a path, a pid or an anyhow chain holding a serde source, and the witnesses must prove it.
  > **NEVER print upstream text, paths, pids or anyhow chains with serde sources in errors or hints.** This is the security plan's NEVER-log floor, and human output of `last` / `list` escapes C0/C1 controls.
- (.andromeda/design-system.md:379) An error document under `--json` is parsed by an LLM driver, so it must carry no colour, glyph or cursor control.
  > **NEVER emit colour, glyphs or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb` or `viola run`.** LLM drivers parse this output
- (.andromeda/design-system.md:385) A failure that reaches any of the three surfaces must print no stack trace; its full detail may go only to `diagnostics/`.
  > **NEVER print stack traces.** Full detail goes to `diagnostics/` only.
- (.andromeda/design-system.md:356) Every product error `Display` reaching a surface must be a fixed message, with the detail it drops kept only in the instance's diagnostics.
  > **Stack traces** never print. Errors are fixed messages, and full detail goes only to `instances/<name>/diagnostics/`.
- (.andromeda/design-system.md:355) An MCP tool's `isError` result must travel as an MCP frame on stdout, and `viola mcp` must gain no other stdout output and no human design surface.
  > **`mcp`** writes only MCP frames on stdout. Neither has a human design surface.
- (.andromeda/design-system.md:263) A CLI `--json` failure must be one JSON document on stdout with the typed exit code and no `hint:` line, its typed fields being the whole answer.
  > For `--json`, the whole result, including refusals and `error` objects, is one JSON document on stdout with the typed exit code. No `hint:` line is printed under `--json`: the typed `refusal` / `error` fields are the whole answer.
- (.andromeda/design-system.md:303) The chunk's closed `detail` set touches this owed item: a per-cause `detail` code stands pending under the arch amendment named here, and faults and `unknown` must get no hint.
  > Under `--json` no hint prints; a per-cause `detail` code is pending, owner: the arch amendment naming them. Faults and `unknown` get no hint.
- (.andromeda/design-system.md:338) The chunk must keep each typed exit code's `--json` form as written here: exit 1 has a document only for `viola revive`'s two refusals, and exit 21's document leaves stderr empty.
  > 1 `error: internal error` or a fixed-message `unable:` refusal with its hint (a `--json` document only for `viola revive`'s strict-modes and preflight refusals), 2 clap usage, 10 to 14 an `unable` refusal with its reason (a refusal object under `--json`), 20 `error: wrapper fault`, 21 `unable  instance-unreachable` then the cause's hint, or for a refused strict-modes check its fixed home-state or instance-state `unable:` / `hint:` pair instead (pattern 1 and the paragraphs below), its `--json` document with stderr empty.
- (.andromeda/design-system.md:334) The chunk's enumeration of error bodies must leave this owed document to its named owner, which the chunk's boundaries place out of scope, and must not build it.
  > A refusal `run`'s start makes after a passed preflight prints no `--json` document: pending, owner: the working-route entry "Exit-cause code catalogue".

## Bindings
- **The errors-and-hints ban ↔ security plan**: the body names this ban as the security plan's NEVER-log floor, so the design ban and the plan's floor must hold as one property across the three surfaces (.andromeda/design-system.md:382)
- **Expression 0.0 for `--json` ↔ Surface: cli**: the category's 0.0 row and the colour decision order's first match, `--json`, must give the same result for an error document (.andromeda/design-system-detail/brand-identity.md:21) (.andromeda/design-system.md:245)
- **Exit-code phraseology ↔ architecture**: the body's forms are those of the typed codes from architecture.md, so a change to an error's code or exit must be read against both (.andromeda/design-system.md:336)
- **Pending `detail` codes ↔ architecture**: the per-cause `detail` codes are owed to an arch amendment, so any such code the chunk names must be read against that owner (.andromeda/design-system.md:303)
- **Owed `--json` start-refusal document ↔ working route**: the document stays with the working-route entry "Exit-cause code catalogue", the chunk's stated boundary (.andromeda/design-system.md:334)
