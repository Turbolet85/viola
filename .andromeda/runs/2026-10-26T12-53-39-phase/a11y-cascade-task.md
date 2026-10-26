# Task — a11y · 2026-10-26-explicit-session-links

## The chunk
Serves the two channel methods `link` and `unlink` in the wrapper (today `-32601`): `link` records the pair and answers `{since, already_linked}`, `unlink` removes it and answers the empty object; a `driven` that is not the serving wrapper's name, or a `driver` equal to `driven`, is `-32602`.
Adds the two CLI verbs `viola link <driver> <driven>` and `viola unlink <driver> <driven>` with `--json` (the verb's instance is the driven one), and their three human result lines `link … since …Z`, `link … already linked`, `unlink …`.
Gives `LinkSet::remove` its first product caller (the `unlink` line), and parses every logged link name shown in a result line through `ViolaName::try_new` again; `viola list` gains no link column.
Adds `viola --help`'s `handoff:  link, unlink` row between `wheel:` and `setup:`, moving the pinned `HELP` literal with it; the verbs' frame path and snapshot-read form follow the boundary option the founder names at P4.
Proves `v1-13`'s `link` half: every human `link` line derivable from its machine result, the `--json` result carrying the same facts.

## Reached files
- src/run/links.rs · edits `LinkSet::remove` (its `cfg_attr(not(test), allow(dead_code))` ends), the served `link` / `unlink` methods
- src/cmd/mod.rs · edits the `--help` grouped verb table (the `handoff:` row), the verb dispatch
- src/cmd/client.rs
- tests/channel_links.rs · edits Scenario E3 steps 1 and 3, the raw-channel name negative
- tests/cli_output_plain.rs · edits `help_is_the_grouped_verb_table`, `help_lists_only_served_verbs`
- viola-0.1.0/verification-matrix.json · edits `v1-13` (its `link` half)
- .andromeda/layout-templates.md · re-reads §Output structure — `viola --help` (the `handoff:` row's placement)

## Needs
- The cli surface's verb entity gains served members: `viola link` and `viola unlink` in human TTY, plain (non-TTY / `NO_COLOR` / `TERM=dumb`) and `--json` modes, asserted by output-discipline assertions on text and streams only.
- The three human result lines (`link … since …Z`, `link … already linked`, `unlink …`): static, linear, one line each, with no cursor control, glyph or spinner of viola's own.
- The verbs' `--json` result: one document, no SGR, no `hint:`, carrying the same facts as the human line (`v1-13`'s derivability claim).
- SGR on the result lines, if any: none under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json`; bold only on a NAME value and always beside its word; a name in a result line or piped output is never cut with `...`.
- The verbs' refusals in human mode: `unable  <reason>  <detail>` on stderr, then `hint:` as the last stderr line, with a typed exit code; `unable` never coloured; each kebab-case code beside the plain word `unable`; no stack trace or bare status number as the only error text.
- The `-32602` answers (a `driven` that is not the serving wrapper, `driver` equal to `driven`) as they reach the CLI verbs: a refusal word with plain detail and a recovery hint, never the raw channel code alone.
- The frame verbs' server-verify refusal reached through `client::connect` (exit 21 `instance-unreachable` with its pinned hint): asserted on windows-2025 and ubuntu-latest only, macOS reporting no peer pid.
- Whichever snapshot-read form the founder names at P4 (the instance check ahead of the snapshot read, or the dated strict-modes gap): its refusal, if the verbs gain one, takes the same `unable` + `hint:` layout and a typed exit code, and the hint-free `--json` document.
- `viola --help`'s `handoff:  link, unlink` group label, between `wheel:` and `setup:`: the P6 structural equivalent (the `--help` group labels) whose `handoff:` row the plan records as owed to "Explicit session links" — this chunk lands it.
- The `HELP` literal moves with the row: `viola --help` is one of the fixed strings pinned by literal, so the copy change fails CI until the literal is updated on purpose.
- Every new fixed CLI string of the verbs (the three result lines, each refusal and its hint) pinned by trycmd or by literal.
- A logged link name shown in a result line passes `ViolaName::try_new` again, and any C0/C1 control character in shown text is escaped except `\n` and `\t`.
- The wrapper's new `link` / `unlink` method handling during `viola run` (and a passed `viola revive`): it writes zero bytes of its own to the terminal, sends its diagnostics only to `diagnostics/`, and never blocks, refuses or delays a human keystroke past the current atomic paste.
- The a11y plan's cli entity and P6 owed note name `link`, `unlink` and the `handoff:` row: the chunk's landing is what turns those from owed to served.

## Standing conventions
- S1 · Each new CLI output-discipline case runs inside `nextest-integration` / `nextest-e2e` locally and the per-OS `test` job's single `coverage` suite in CI, on all three OS legs, under `gate --require coverage,doctest` with no skip · §1 A11y Scope Summary (CI integration) and §9 CI Integration
- S2 · No new `suite` enum value (such as `a11y`) and no separate a11y driver outside `scripts/agent-run.* run` · §11 A11y Anti-Patterns › CI
- S3 · Each new refusal: trycmd pins the `unable <reason> <detail>` + final `hint:` layout, assert_cmd pins the typed exit code and a hint-free `--json` · .andromeda/a11y-plan-detail/cognitive-accessibility.md (Error recovery)
- S4 · Each new fixed CLI string pinned by trycmd or literal · .andromeda/a11y-plan-detail/cognitive-accessibility.md (Plain language)
- S5 · Each new human-mode output: no SGR under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json`, every SGR span beside its word, colour never the sole cue · .andromeda/a11y-plan-detail/visual-design-verification.md
- S6 · No WCAG conformance claim rests on assert_cmd / trycmd / portable-pty results · §11 A11y Anti-Patterns › Strategy
- S7 · Zero flakiness: `retries: 0`, no `#[ignore]`, no retry-once policy, no soft a11y SLO · §10 SLO Invariants & A11y Budgets and §11 › SLO

## Not needed
- The `viola list` board's `CAPTION` literal and caption order (SC 3.2.4): the chunk adds no link column and changes no caption.
- The board's amber `DIALOG`, dim stale rows, the `-- UNWRAPPED - READ-ONLY --` separator and the unwrapped NAME's narrow-terminal cut: board-only rendering this chunk does not touch.
- The board's dim-stale outer-PTY witness owed to "Linux and macOS parity".
- `viola wait` / `viola last` output, the `waiting: <name>` stderr line and `viola verify`'s `[NN/NN] … pass|fail` step lines: no change to those verbs.
- The `viola revive --json` owed document for a refusal after a passed preflight: owed to "Exit-cause code catalogue".
- The not-assertable plugin / stdio MCP / internal-crate entities: the chunk adds no MCP tool and no hook path, and the channel methods themselves carry no UI.
- The deferred 0.2.0 frontend (any `/api/links` page, its a11y tooling, ARIA landmarks for a page): 0.1.0 renders no page.
- The manual NVDA / VoiceOver / Orca passes: founder-owned and supplemental, not a deliverable of this chunk.
- The cognitive time-bound inventory (wait timeout, MCP wait cap, dialog hold, readiness gate, confirm windows, status-line deadline, agents read, budget pause): the scope adds no wait, hold, window or deadline.
- The Windows win32-input-mode mouse-report fact (F-W3) and the focus / mouse / resize wheel rules: the chunk moves no wheel and adds no input handling.
- The not-triggered triggers (motion, multi-language, target-size, regulated compliance, cognitive AAA escalation).
