# design-system · the rows that bind 2026-11-14-budget-gate-removed — picked 53 of 325 (20 by every draw) · rule any · pin 1142

## brand-identity · ## Brand Identity
decision/budget-paused-stops-new-sends · Budget-paused as minimum-fuel state — The budget governor shows `5H` / `7D` figures, and `budget-paused` is a declared state that stops new sends while the running turn finishes.
decision/fixed-phraseology-vocabulary · Fixed phraseology vocabulary — Short fixed words (`open`, `RB`, `unable`, `human-typing`, `budget-paused`, `not-delivered`) are the vocabulary of every label, CLI column and refusal, with no filler and no guessing.
decision/readings-carry-their-age · Readings carry their age — Every budget reading carries its age (`read 4m ago`), and old information is shown as old (`expired`), never as current.
contract/flight-progress-strip/status · Flight-progress strip · STATUS — STATUS is the third of the six fixed fields.
contract/flight-progress-strip/wheel · Flight-progress strip · WHEEL — WHEEL is the fourth of the six fixed fields.

## color-palette · ## Color Palette
decision/skipped-count-stays-visible · Skipped count stays visible — A nonzero `skipped` count stays visible and is never auto-cleared.

## spacing · ## Spacing
decision/cli-two-space-columns · Two spaces between CLI columns — CLI spacing follows the fixed-width terminal grid, with two spaces between columns.

## iconography · ## Iconography
decision/list-six-fixed-columns · viola list has six fixed columns — `viola list` carries no link column, so its six columns stay fixed.
contract/cli-separators/two-spaces · CLI separators · two spaces — Two spaces separate columns, and a refusal's reason from its detail, as separate fields.
contract/viola-list/columns · viola list output · columns — `viola list` carries no link column in v1, so its six columns stay fixed.

## surface-cli · ## Surface: cli · twin surface
decision/expired-window-shown-expired · Expired budget window shown as expired — A budget window whose `resets_at` has passed still prints its figure followed by `expired`. · why: old information is shown as old
contract/cli-exit-codes/10-14 · CLI exit codes · 10 to 14 — An `unable` refusal with its reason; a refusal object under `--json`.
contract/cli-verb-output/release-budget · CLI verb output · viola release --budget — Prints one unstyled stdout line, `<name>  budget gate lifted until <time>  wheel human` when a pause stood, else `<name>  budget gate open  wheel human`.
contract/viola-list-table/bay-line · viola list table · BAY line — The BAY line is the ATIS without `resets` times (printed by `viola release <name> --budget`) and without a TAPE cell.
decision/list-refuses-board-on-strict-modes-failure · Strict-modes failure refuses the whole board — A failed strict-modes check of the home or any instance refuses the whole list board with exit 21, in the fixed unable and hint pair or, under --json, the instance-unreachable document with detail strict-modes-failed. · why: The MCP list tool reads the same refusal document through the shared board reader, and the list tests pin exit 21 and the document byte for byte.
decision/no-hint-for-faults · No hint for faults and unknown — `error: wrapper fault` (exit 20), `error: internal error` (exit 1) and `unknown` (exit 14) print no hint line; a fault's human line holds the code only and further detail goes to `diagnostics/`. · why: Faults are not refusals; the detail of `unknown` is a code word or nothing, so a hint would have to guess.
decision/start-refusals-show-no-path-or-pid · Start refusals show no path or pid — The exit-1 refusals of `viola run` and `viola revive` print one fixed-message line and one hint per cause, and none shows a path, a pid, the recorded directory or a logged id.
contract/cli-colour-tokens/dim · CLI colour tokens · dim — SGR dim is laid on a whole `stale` row; the attention colour wins for `DIALOG`.
contract/cli-refusal-line/hint · CLI refusal line · hint — A refusal is followed directly by one `hint:` line per reason, detail or cause.
contract/cli-refusal-line/two-space-form · CLI refusal line · two-space form — A refusal prints `unable  <name>  <reason>  <detail>` on stderr, two spaces between fields.
contract/cli-verb-output/release · CLI verb output · viola release — `viola release <name>` prints `<name>  wheel driver  you have control`.
contract/viola-list-table/columns · viola list table · columns — Under a BAY line the table has the columns NAME, LIVE, STATUS, WHEEL, DIALOG and CLI; unwrapped sessions follow a `-- UNWRAPPED - READ-ONLY --` line.
contract/viola-send-mirror/unable · viola send readback mirror · unable — `[/ ] unable  <name>  <reason>  <detail>` prints on stderr with exit 13, followed by its `hint:` line.
contract/viola-verify-output/step-line · viola verify output · step line — One static appended uncoloured stdout line per relied ledger row, ending `pass` or `fail`, counted against `MM`, the relied count.
contract/viola-verify-output/summary · viola verify output · summary — A summary line `stamped <ver>  <n> pass  <m> fail` follows, n + m being the relied count.

## surface · ## Surface: cli · twin surface-cli
decision/no-hint-names-viola-release · No hint names viola release — No refusal hint ever names `viola release`; `release` stays a human verb. · why: The human-typing and budget-paused refusals reach drivers, and a driver's `release` is refused (`-32602`, `release-from-driver`, exit 20).
decision/wheel-column-width-23 · WHEEL column padded to 23 — The board's WHEEL column is padded to 23 characters, the width of its widest word `unknown - budget-paused` (a replayed row with a logged pause and no logged holder). · why: Ratified.
contract/cli-exit-codes/11 · CLI exit codes · 11 — `unable  budget-paused  five-hour` or `seven-day`; under `--json` the refusal object.
contract/peer-reply-screening/unlisted-code-word · peer reply screening · code word outside the refusal list — A code word outside the closed refusal list stays visible, by design, in the document and in `refused: <word>`, and exits 14 as `unknown` does.
contract/refusal-hints/budget-paused · refusal hints · budget-paused — `budget gate is closed; it lifts at the window reset, or when the human lifts it`
contract/refusal-hints/refusal-line · refusal hints · refusal line — A refusal prints `unable` plus `reason  detail`, then one `hint:` line keyed by the reason, or by reason and detail where listed; `send` and `answer` print the same hint for the same reason or detail, and a `not-delivered` refusal with an opaque detail or none prints no hint.
contract/viola-list-board/human-holder-budget-paused · viola list board · human holder under a budget pause — A `human` holder under a budget pause prints `human - budget-paused`.
contract/viola-list-board/reading-age · viola list board · reading age — The reading's age prints in whole minutes rounded down at every age (`read 0m ago`, `read 75m ago`), and `read unknown` for no instant, one that does not parse, or one ahead of now.
contract/viola-list-board/skip-counts · viola list board · skip counts — Nonzero skip counts print as bare numbers in key order (`skipped 2 - 0 - 1`).
contract/viola-list-board/window-figure · viola list board · window figure — A window's figure is rounded down to a whole number (`89.6` prints `89 %`).
contract/viola-release-budget/lifted-line · viola release --budget · lifted line — When a pause stood, `viola release <name> --budget` prints `<name>  budget gate lifted until <time>  wheel human`, one unstyled stdout line with the two-space gutter of the other verb lines.
contract/viola-release-budget/open-line · viola release --budget · open line — When no pause stood the line is `<name>  budget gate open  wheel human`, one unstyled stdout line.
contract/viola-release-budget/override-time · viola release --budget · override time — The time is the reply's `override_until`: `HH:MM:SSZ` on the current UTC date, `YYYY-MM-DD HH:MM:SSZ` on any other date; a value that is no RFC 3339 instant prints escaped.
contract/cli-colour-tokens/stale · CLI colour tokens · stale — SGR dim (`\x1b[2m`) at every depth over a whole `stale` row, first byte to last; `stale` always in LIVE; on a cocked stale row only `DIALOG` is amber and not dim; NAME on a stale row is dim and bold; every span reopens dim after its reset.
contract/cli-exit-codes/10 · CLI exit codes · 10 — `unable  human-typing` (+ `manual-pause`); under `--json` `{"v":1,"refusal":"human-typing","detail":…}`.
contract/cli-exit-codes/12 · CLI exit codes · 12 — `unable  unverified-cli`; under `--json` the refusal object.
contract/cli-exit-codes/13 · CLI exit codes · 13 — `unable  not-delivered` with `input-not-ready`, `no-prompt-submitted`, `turn-running`, `unknown-dialog`, `control-character` or `empty-text`; under `--json` the refusal object.
contract/cli-exit-codes/14 · CLI exit codes · 14 — `unable  unknown  <detail>`, the detail a code word or nothing; under `--json` the refusal object.
contract/cli-stderr/hint-line · CLI stderr · hint line — Each refusal's `hint:` line follows it directly on stderr.
contract/cli-stderr/refusal-line · CLI stderr · refusal line — A refusal prints `unable  <name>  <reason>  <detail>`, two-space separated; `viola send` prefixes the `[/ ]` mirror and pads `unable` to the readback-word column.
contract/viola-list-board/colour-spans · viola list board · colour spans — Colour spans are laid over the escaped, padded cells, so the columns are the plain board's, and the board stays one write.
contract/viola-list-board/empty-board · viola list board · empty board — The empty board prints the BAY line, then the caption row, then its one line.
contract/viola-list-board/name-column-width · viola list board · NAME column width — On a terminal of known width NAME's column is the largest of the caption's 4, the longest wrapped NAME and the longest unwrapped NAME capped at the width less the 65 columns the rest of an unwrapped row takes.
contract/viola-list-board/name-cut · viola list board · NAME cut — An unwrapped NAME longer than its column is cut to the column less 3 characters and ends in `...`, never splitting an escape; a wrapped NAME is never cut; a piped board or `--json` keeps every name whole.
contract/viola-list-board/uncoloured-lines · viola list board · uncoloured lines — The BAY line, the caption, the empty-board line and the separator carry no SGR.
contract/wrapper-fault-detail/other-invalid-params · wrapper fault --json detail · other -32602 — Any other `-32602` reads `invalid params` with null `data`.
contract/wrapper-fault-detail/release-from-driver · wrapper fault --json detail · -32602 release-from-driver — `-32602` with `data.reason` exactly `release-from-driver` reads that body.

## anti-patterns-never-do-these · ## Anti-Patterns (NEVER do these)
decision/budget-as-ruled-figures · Budget as figures in ruled fields — Budget prints as figures in ruled fields with the ATIS reading age, never as rings or coloured bars; `budget-paused` is a word, not a colour.

# design-system · its absences, names and overviews, whole

## brand-identity · ## Brand Identity
N/A — viola has no web or graphical frontend, so no frontend expression level.
overview/brand-identity — The direction the CLI carries, and the terminal fleet console inherits, is a control-tower strip bay after dark: buff paper strips in anthracite holder boots on ruled aluminium racks. Every session holds the same slot for its whole life, and the only warm light is the one arrival holder cocked out of line when a session needs the human.

## color-palette · ## Color Palette
Surface Scale (elevation hierarchy) › N/A — viola renders no page, so no surface scale. ¶ Text Hierarchy › N/A — viola renders no page, so no text hierarchy; the cli's ink is the terminal's (Surface: cli). ¶ Semantic Colors › N/A — viola renders no page, so no colour states; the cli's status guards below stand.

## typography · ## Typography
N/A — viola renders no page; the cli's type is the terminal's (below).

## spacing · ## Spacing
N/A — viola renders no page; the cli's spacing is the terminal grid (below).

## depth-strategy · ## Depth Strategy
N/A — viola renders no page, so no depth tokens; the cli draws no depth.

## border-radius · ## Border Radius
N/A — viola renders no page, so no radius tokens.
absence/rendered-page — A rendered page: viola renders no page, so no radius, motion or spacing tokens; the cli's type is the terminal's

## surface-web-spa · ## Surface: web-spa
absence/web-page-surface — A web page or web surface of any kind

## surface-cli · ## Surface: cli · twin surface
absence/cli-tape — A tape in the CLI
name/command/answer — answer
name/command/hook — hook
name/command/last — last
name/command/link — link
name/command/list — list
name/command/mcp — mcp
name/command/pause — pause
name/command/plugin-install — plugin install
name/command/release — release
name/command/revive — revive
name/command/run — run
name/command/send — send
name/command/unlink — unlink
name/command/verify — verify
name/command/wait — wait

## self-validation-protocol · ## Self-Validation Protocol
4. Token Test › N/A — viola renders no page, so no stylesheet tokens to trace. ¶ 4. Token Test › N/A — viola renders no page, so no stylesheet token to trace.
