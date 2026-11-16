@history_item n=1 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (Surface: cli: 1 row) — added `decision/api-error-turn-end-shown-as-turn-end`: an API-error turn end is a plain `turn-ended` with one closed `cause` word in event data, no new human wording, status, colour or hint; PROVISIONAL on the overseer's delegate word. The direct precedent for this chunk: a new datum on the normalised `turn-ended` event that reaches `--json` / MCP `wait` only, with the human `wait` line and the board's STATUS unchanged, and a design row recording that choice.

@history_item n=2 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** (Surface: cli: 1 row) — `~ decision/api-error-turn-end-shown-as-turn-end`: the trace grew from the one word `api-error` to one closed cause word of nine in `--json` and MCP `wait` event data, with no human-output change; the confirm the delegate's, PROVISIONAL, under the founder's ruling A. The latest extension of `turn-ended`'s event data by the hop set this chunk's count follows (`--json`, MCP `wait`); its wording names "the only trace" of a turn end in machine data, so the plan should say whether the count needs any row change or a sibling row.

@history_item n=3 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — fixed `viola wait`'s result-line forms: every non-dialog kind takes the `turn-ended` form (`<kind>  <name>  <time>  cursor <n>`), and message mode escapes control characters as `\xHH` while `--json` stays serde-escaped. This is `contract/viola-wait-output/event-form`'s origin: the human `turn-ended` line carries no data fields, so a count added to the event must not grow that line.

@history_item n=4 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — narrowed the time form to "every other kind without a `dialog_id`" so `dialog-expired` takes the dialog form; added no colour row for the new kind. The wording `contract/viola-wait-output/event-form` now holds: `turn-ended` keeps the time form whatever its data carries.

@history_item n=5 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (version census 0.2.0 — brand, surface and bans scoped to the CLI) — kept the CLI's status guards (`turn-ended` among them, no-spinner `busy`) and scoped the spinner / progress-bar / live-redraw ban to a verb's output. These constrain how a scheduled wake may appear in human output (no live indicator, no new state glyph), and they stand behind `decision/turn-ended-never-fills-readback` and `decision/cli-tty-static-expression`.

@history_item n=6 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the catch site's `internal-error` document) — stated that `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site, under `--json` too. That is the source of `decision/silent-catch-sites` and `contract/cli-catch-site/silent-verbs`: the Stop hook path that will read `session_crons` must stay silent (no stderr, exit 0) on a malformed or unexpected shape.

@history_item n=7 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (code-word screen; exit 2's fixed lines; the witnessed `--json` failures) — both clients screen a peer's `refusal` / `detail` to code words, no upstream text reaches a document, and `--json` gained its exit-2 exception (`decision/json-one-document-on-stdout`'s except clause). This sets the precedent behind `decision/no-internals-in-errors` and the chunk's boundary that no wake's content (prompt, cron, id) reaches any output: only a number crosses, and the one-document `--json` rule holds unchanged.

@history_item n=8 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Brand Identity: 3 rows; Surface: cli: 4 rows) — `~ decision/fixed-phraseology-vocabulary` dropped `budget-paused` from the fixed words, and `~ contract/viola-list-table/bay-line` cut the BAY line to the skip counts (`BAY  skipped a - b - c`), with budget board cells removed whole. The vocabulary and board rows this chunk would extend if the count reaches `list` (P3's hop measurement): a new word or cell is a vocabulary and column change, against `decision/list-six-fixed-columns`.

@history_item n=9 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — recorded the board as built: escape every control character of a row as `\xHH` (`escape_row`), five planning forms, WHEEL padded to 23, and `list`'s exit-21 refusal (all later confirmed by the founder at 2026-10-13-t06-38-35-wrap). This is the source of `contract/viola-list-board/row-escaping` and `contract/viola-list-board/empty-board`, which bind any board change this chunk makes.

@history_item n=10 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — built the unwrapped rack: STATUS `idle` / `busy` from a `claude agents --json` join, and the `-- UNWRAPPED - READ-ONLY --` line with `n/a` in WHEEL, DIALOG and CLI. This is the source of `contract/viola-list-table/columns` and `contract/viola-list-board/unwrapped-row`; STATUS `idle` after a turn end is the reading `decision/api-error-turn-end-shown-as-turn-end` keeps, and a wake count must not change it.

@history_item n=11 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — built the colour layer (amber `DIALOG`, dim `stale` rows, spans over escaped padded cells, one write) and the width cut (NAME capped at the width less 65). This is the source of `contract/viola-list-board/colour-spans`, `name-column-width`, `uncoloured-lines` and `cli-colour-tokens/stale`: no third colour or new column may mark a scheduled wake (`decision/cli-colour-amber-and-dim-only`, `decision/colour-carries-meaning`), and the 65-column arithmetic fixes the row width.
