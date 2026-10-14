# design extract

## Relevance
partial — no web-spa surface is touched; the design surface is the CLI output of `viola verify` (one step line per ledger row plus the summary) and, if `/compact` joins the local-command list, the readback of a `/compact` send.

## Constraints
- design-system §Surface: cli §Component Patterns (5. `viola verify`) requires one static stdout step line per member of `LedgerRow::ALL`, `[NN/MM] <row-id> <one-line description>  pass|fail`, then the summary line `stamped <ver>  <n> pass  <m> fail` as the last stdout line. Each row this chunk adds needs a row-id word and a fixed one-line description in that pattern. The master's literal row count and its row list ("eighteen rows today", the last eight named) are a drift item for every row that lands. Whether the code already derives MM and the step lines from `ALL` rather than from literals is research's question.
- design-system §Surface: cli §Component Patterns (5. `viola verify`) requires that a run with a failing row prints no stderr word, and that an interrupted run keeps its fixed `unable:` / `hint:` pair. A new probe shape (a resume child, a compaction turn, a statusline child, a `claude agents --json` start) adds no new stderr wording unless the master is amended.
- design-system §Surface: cli (Width) requires human output to be ASCII only and never wrapped. Each new description string is plain ASCII.
- design-system §Color Palette (Domain status colors, `Readback::read`) fills the readback only on a matching `prompt-submitted` or a met ledger post-condition, and `SessionStartCause::clear` is named as the post-condition fill of a `/clear` send. A `/compact` row with a post-condition would make `SessionStartCause::compact` such a fill. The master does not name it as one yet, so that is an amendment.
- design-system §Color Palette (Domain status colors, `CliVerified::false`) prints `unverified-cli` as a word with no colour. Every home stamped before the new rows reads it until it is re-stamped, and the master gives no special treatment for a stamp that predates them.

## Patterns to follow
- The existing row lines in design-system §Surface: cli §Component Patterns (5): `local-command-clear` (`/clear starts a new session and submits no prompt`) is the model for a `/compact` row, and `paste-newline` is the model for a short row id followed by a one-clause, effect-stated description.
- The step counter is the project's one progress pattern (design-system §Surface: cli §Component Patterns (5)). It uses a two-digit zero-padded `[NN/MM]` with static appended lines.
- The fixed-message `unable: <text>` / `hint:` form for `verify` refusals (design-system §Surface: cli, Streams, and the Exit-code phraseology row for exit 1).

## Anti-patterns to avoid
- No spinner, progress bar or live redraw while a long probe runs, such as a compaction turn or a resume start. `verify` prints step-counter lines only (design-system §Anti-Patterns (NEVER do these) §Per-Surface Bans, cli).
- No green `✓` / red `✗`, no emoji, and no colour on `pass` / `fail`. The word `fail` stays uncoloured (design-system §Anti-Patterns (NEVER do these) §Per-Surface Bans, cli).
- No upstream text, path or pid in a step line, summary, refusal or hint. That covers a resumed session id, the statusline command or its output, and a recorded directory (design-system §Anti-Patterns (NEVER do these) §Per-Surface Bans, cli).

## Contract bindings
- The verify step lines and the summary count bind to the capability ledger's `LedgerRow::ALL`, an architecture §Cross-cutting Patterns item (Capability ledger). They also bind to the test oracles that pin the row set by shape: the verdict vector, the stamp's rows object and the dialog-less summary. Each added row moves the printed MM, the summary's `n pass` and the master's row list together.
- A `/compact` local command with a post-condition binds the readback fill (design-system §Color Palette, Domain status colors, `Readback::read`) to the ledger's `PostCondition` and to the `SessionStartCause::compact` tape word.

## Acceptance criteria contributions
- `viola verify` prints exactly one `[NN/MM] <row-id> <description>  pass|fail` stdout line per member of `LedgerRow::ALL`, with MM equal to the new count. The last stdout line is `stamped <ver>  <n> pass  <m> fail`, with n + m = MM (per design-system §Surface: cli §Component Patterns (5)).
- Every new step line is plain ASCII with no SGR, glyph or spinner, and `fail` is uncoloured (per design-system §Anti-Patterns (NEVER do these) §Per-Surface Bans, cli).
- No new step line, summary, refusal or hint carries a session id, path, pid, statusline command or statusline output (per design-system §Anti-Patterns (NEVER do these) §Per-Surface Bans, cli).
- If `/compact` lands on the local-command list, a `/compact` send reads back only when its ledger post-condition is met. It is never presumed, and with no post-condition it is never filled (per design-system §Anti-Patterns (NEVER do these) §Rejected Defaults, and §Color Palette, Domain status colors, `Readback::read`).
