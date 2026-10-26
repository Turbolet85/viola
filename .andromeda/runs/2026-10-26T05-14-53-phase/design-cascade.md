# design · cascade read

## Carried
- (.andromeda/design-system-detail/surface.md:104) The scope holds viola's exit codes and refusal words unchanged across the tokio move, and the Tokio channel client behind `viola mcp`'s tools must keep carrying this refusal word and its object as before.
  > | 10 | `unable  human-typing` (+ `manual-pause`) | `{"v":1,"refusal":"human-typing","detail":…}` |
- (.andromeda/design-system-detail/surface.md:105) Exit 11's refusal word and its two window details must read the same after the bump, on the CLI and through the Tokio-built `viola mcp` path.
  > | 11 | `unable  budget-paused  five-hour` / `seven-day` | refusal object |
- (.andromeda/design-system-detail/surface.md:106) Exit 12's refusal word must read the same after the bump; a deprecation fix the bump forces must not alter it.
  > | 12 | `unable  unverified-cli` | refusal object |
- (.andromeda/design-system-detail/surface.md:107) Exit 13's six `not-delivered` details must stay exactly this closed set after the bump, whichever client carries them.
  > | 13 | `unable  not-delivered  input-not-ready` / `no-prompt-submitted` / `turn-running` / `unknown-dialog` / `control-character` / `empty-text` | refusal object |
- (.andromeda/design-system-detail/surface.md:108) Exit 14's `unknown` refusal must keep its opaque-detail form after the bump.
  > | 14 | `unable  unknown  <detail as opaque text>` | refusal object |
- (.andromeda/design-system-detail/surface.md:109) `wrapper-fault` is a code the Tokio-built `viola mcp` path returns as well, so its human form and `--json` document must stay as they are after the tokio move.
  > | 20 | `error: wrapper fault  <code>` | `{"v":1,"error":"wrapper-fault","detail":{…}}` |
- (.andromeda/design-system-detail/surface.md:110) `instance-unreachable` is the code the Tokio-built `viola mcp` tools return on a failed liveness pre-check, so this row's form and document, with stderr empty, must hold after the bump; and `viola mcp`, whose runtime is the tokio being moved, must keep writing only MCP frames on stdout.
  > | 21 | `unable  instance-unreachable` (no detail; the name is the omitted `<name>` field, as in every row), then the cause's hint (not running / unwrapped / strict-modes / server verification); `viola list`'s refused strict-modes check prints instead the fixed home-state or instance-state `unable:` / `hint:` pair (pattern 1, Refusal; the founder's own answer, 2026-10-13, relayed verbatim by the Viola overseer) | `{"v":1,"error":"instance-unreachable","detail":null}` (a per-cause `detail` code awaits an arch amendment), `list`'s refusal included, with stderr empty |
  (.andromeda/design-system.md:338)
  > Each typed exit code has its human stderr form and, where it has one, its `--json` document: 0 the result on stdout, 1 `error: internal error` or a fixed-message `unable:` refusal with its hint (a `--json` document only for `viola revive`'s strict-modes and preflight refusals), 2 clap usage, 10 to 14 an `unable` refusal with its reason (a refusal object under `--json`), 20 `error: wrapper fault`, 21 `unable  instance-unreachable` then the cause's hint, its `--json` document with stderr empty.
  (.andromeda/design-system.md:353)
  > **`mcp`** writes only MCP frames on stdout. Neither has a human design surface.
  (.andromeda/design-system.md:247)
  > `viola mcp` (stdout carries MCP frames only)

## Bindings
- **surface exit-code rows ↔ the body's `mcp` platform note**: the refusal words and fault codes of the exit-code table (.andromeda/design-system-detail/surface.md:104) through (.andromeda/design-system-detail/surface.md:110) are the same closed words `viola mcp`'s tools return over the Tokio channel client, and that verb's stdout stays MCP frames only (.andromeda/design-system.md:353); the tokio move sits under both, so both must read unchanged after it.
- **surface exit-code rows ↔ security plan (Error Handling)**: the MCP `isError` results carry codes only (`refusal`/`detail`, or `instance-unreachable`, `wrapper-fault`, `invalid-params`), the same words as rows 20 and 21 (.andromeda/design-system-detail/surface.md:109) and (.andromeda/design-system-detail/surface.md:110); a deprecation fix the bump forces in a Tokio-built path must change neither side.
