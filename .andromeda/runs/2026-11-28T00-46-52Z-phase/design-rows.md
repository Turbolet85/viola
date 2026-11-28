# design-system · the rows that bind 2026-11-28-console-scaffold — picked 95 of 361 (62 by every draw) · rule any · pin 5229

## brand-identity · ## Brand Identity
decision/cli-has-no-motion · No motion in the CLI — The cli (0.2 on a TTY, 0.0 otherwise) has no motion at all.
decision/cli-tty-static-expression · Static expression for the human CLI — Human TTY CLI output is at expression 0.2: static fixed-column rows and one static context header, nothing redrawn in place and no spinners.
decision/expression-level · Expression level 0.2 — The base expression level `0.2` is the single source of truth for animation intensity, layout freedom, interaction complexity and motion across the whole project.
decision/fixed-phraseology-vocabulary · Fixed phraseology vocabulary — Short fixed words (`open`, `RB`, `unable`, `human-typing`, `not-delivered`) are the vocabulary of every label, CLI column and refusal, with no filler and no guessing.
decision/rejected-style-presets · Rejected style presets — The Dark Mode (OLED), Real-Time Monitoring (pulsing dots, blink, toasts) and HUD / Sci-Fi FUI (glow, telemetry animation) presets are not used.

## color-palette · ## Color Palette
decision/amber-only-for-dialog-pending · Arrival amber for dialog-pending only — Arrival-holder amber #D97706 is the only attention colour, used for `dialog_pending` only. · except: never for errors, warnings, budget or hover
decision/busy-not-animated · Busy has no animation — The `busy` state has no spinner, pulse or animation.
decision/no-green-accent · No run-green accent — The run-green accent of the Developer Tool palette is not used; amber is the single sharp accent.
contract/colour-palette/accent · Colour palette · Accent #D97706 — The only attention colour: the `DIALOG` word of a pending dialog in `viola list` human output, as truecolor or its 256- and 16-colour fallbacks; never used for errors, warnings or budget.

## typography · ## Typography
decision/cli-ascii-only · ASCII-only CLI output — CLI output is ASCII only, separators included, at high density.
decision/stale-rows-sgr-dim · Stale rows in SGR dim — In the cli, SGR dim marks `stale` rows and amber marks the `DIALOG` word.

## spacing · ## Spacing
decision/cli-two-space-columns · Two spaces between CLI columns — CLI spacing follows the fixed-width terminal grid, with two spaces between columns.

## iconography · ## Iconography
decision/no-decorative-icons · No decorative icons — Icons clarify, not decorate: no robot or person icons, no status dots, no checkmarks and no warning triangles, since every such state is a printed word.
decision/text-glyphs-only · Custom text glyphs only — Iconography is custom text glyphs only; phraseology words carry the meaning.
contract/cli-glyphs/arrow · CLI glyphs · -> — The arrow glyph is `->`, with no inbound form.
contract/cli-glyphs/field-separator · CLI glyphs · " - " — ` - ` separates parts inside one field.
contract/cli-separators/space-hyphen-space · CLI separators · ` - ` — Space, hyphen, space replaces a middle dot inside one field or label (`skipped 2 - 0 - 1`).
contract/cli-separators/two-spaces · CLI separators · two spaces — Two spaces separate columns, and a refusal's reason from its detail, as separate fields.

## surface-cli · ## Surface: cli · twin surface
decision/ascii-mirror-words-always-print · ASCII mirror marks are words, not glyphs — A glyph is a non-ASCII symbol; the ASCII readback mirror `[RB]` / `[  ]` / `[/ ]` and `->` are words that print in every human output, piped, `NO_COLOR` and `TERM=dumb` included, and only `--json` replaces them with its JSON document.
decision/ascii-only-human-output · ASCII-only human output — Human output is ASCII only (`->`, `[RB]`, `[/ ]`, `[  ]`).
decision/clap-without-colour-feature · clap built without its colour feature — Argument parsing uses clap (derive) built without its `color` feature, so clap's help and usage output is plain in every mode and no dependency reads a colour or terminal variable. · why: every styled byte of human output is viola's own SGR module's
decision/columns-never-read · COLUMNS is never read — Terminal width comes only from stdout's own terminal; `COLUMNS` is never read and a piped stdout has no width.
decision/flat-cli-verbs · Flat CLI verbs — CLI verbs are flat, each one lower-case word, with no nested subcommand groups. · except: `plugin install`
decision/fleet-console-not-verb-output · Fleet console is no verb's output — The terminal fleet console is no verb's output, so verb output rules do not govern its redraw.
decision/hand-written-sgr-module · Hand-written SGR styling module — Styling is a small hand-written SGR module in the `viola` bin. · why: adds no dependency
decision/hint-never-quotes-text · Hints never quote text — A hint never quotes the sent text or any upstream text.
decision/lines-never-wrapped · Lines never wrapped — CLI output lines are never wrapped.
decision/linux-terminal-platform · Linux terminals as the CLI platform — The cli targets Linux terminals: the core on any current distribution, CI on `ubuntu-latest`.
decision/no-handoff-blue-in-cli · No handoff blue in the CLI — The CLI never uses the handoff blue colour.
decision/no-hint-for-faults · No hint for faults and unknown — `error: wrapper fault` (exit 20), `error: internal error` (exit 1) and `unknown` (exit 14) print no hint line; a fault's human line holds the code only and further detail goes to `diagnostics/`. · why: Faults are not refusals; the detail of `unknown` is a code word or nothing, so a hint would have to guess.
decision/no-hint-names-release · No hint names viola release — No hint names `viola release`. · why: a driver's `release` is refused
decision/no-interactive-prompt-or-pager · No interactive prompt or pager — No verb has an interactive prompt or a pager.
decision/no-spinner · No spinner or elapsed-time counter — The CLI shows no spinner and no elapsed-time counter.
decision/no-stack-traces · No stack traces — Stack traces never print; errors are fixed messages and full detail goes only to the instance's diagnostics.
decision/no-terminal-background · No background colours — CLI output never sets a background colour. · why: the terminal background belongs to the user
decision/step-counter-only-progress-pattern · Step counter as the one progress pattern — The step counter of `verify` is the project's one progress pattern.
decision/truncate-only-unwrapped-name · Only an unwrapped NAME is truncated — Where stdout's own terminal width is known, human tables truncate only the NAME of unwrapped rows (`...`); otherwise nothing is truncated.
decision/tty-detection-std-isterminal · TTY detection by std IsTerminal — TTY detection uses std `IsTerminal`.
contract/cli-colour-decision-order/json · CLI colour decision order · --json — First match of the colour decision order: `--json` output has no colour, no glyph and no SGR.
contract/cli-colour-decision-order/no-color · CLI colour decision order · NO_COLOR — Fourth match of the colour decision order: with `NO_COLOR` set (present, an empty value included) output has no colour, no glyph and no SGR.
contract/cli-colour-decision-order/stdout-not-terminal · CLI colour decision order · stdout is not a terminal — Sixth match of the colour decision order: when stdout is not a terminal output has no colour, no glyph and no SGR.
contract/cli-colour-decision-order/term-dumb · CLI colour decision order · TERM=dumb — Fifth match of the colour decision order: with `TERM=dumb` output has no colour, no glyph and no SGR.
contract/cli-colour-decision-order/viola-hook · CLI colour decision order · viola hook — Third match of the colour decision order: `viola hook` prints no colour, glyph or SGR and never writes stderr.
contract/cli-colour-decision-order/viola-mcp · CLI colour decision order · viola mcp — Third match of the colour decision order: `viola mcp` prints no colour, glyph or SGR, and its stdout carries MCP frames only.
contract/cli-colour-decision-order/viola-run · CLI colour decision order · viola run — Second match of the colour decision order: under `viola run` no colour, glyph or SGR is printed, and while the child runs nothing is printed at all.
contract/cli-colour-tokens/attention · CLI colour tokens · attention — Arrival amber #D97706 on the `DIALOG` word only, always followed by its kind word, which stays plain: `\x1b[38;2;217;119;6m` in truecolor, `\x1b[38;5;172m` at 256 colours, `\x1b[33m` at 16.
contract/cli-exit-codes/0 · CLI exit codes · 0 — No stderr first word (TTY context lines only); the result line goes to stdout; under `--json` the document `{"v":1,"ok":{…}}`.
contract/cli-exit-codes/1 · CLI exit codes · 1 — `error: internal error` (fixed; no chain, path or hint) or a fixed `unable:` refusal of `run`, `revive`, `open`, `focus` or `verify` and its hint; `--json` only for revive's shared-start, open's and focus's refusals and the catch site's `{"v":1,"error":"internal-error","detail":null}`; stderr empty.
contract/cli-exit-codes/10-14 · CLI exit codes · 10 to 14 — An `unable` refusal with its reason; a refusal object under `--json`; 11 is retired (was `budget-paused`) and never reused.
contract/cli-exit-codes/2 · CLI exit codes · 2 — Clap usage text (never from `viola hook`), or the fixed `UNUSABLE_RESPONSE` line (oversize `viola answer`), `UNUSABLE_TEXT` line (unusable `viola send`) or `PAIR_AFTER_DASHES` line (a pair after `--` of `run` or `revive`) on stderr, whatever `--json` says; no document.
contract/help-output/verb-groups · viola --help output · verb groups — `viola --help` lists the served verbs as `board: list, focus` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: open, run, revive, adopt, up, switch, verify, plugin install`, then `Options:`.
contract/viola-verify-output/step-line · viola verify output · step line — One static appended uncoloured stdout line per relied ledger row, counted against `MM`, the relied count, ending in one verdict word of `pass`, `absent`, `fail`; `absent` is written only on the `browser-tools-prompt` row, when no run met the prompt, and counts as a pass on that row alone.
contract/viola-verify-output/summary · viola verify output · summary — A summary line `stamped <ver>  <n> pass  <m> fail` follows, n + m being the relied count, `n` counting the rows read `pass` and the `browser-tools-prompt` row read `absent`.
decision/list-is-the-board · viola list is the board — `viola list` is the board, `verify` and `plugin install` act on no strip, `up` acts on the whole fleet or on the instances it names, and every other verb a person runs acts on one strip.
decision/list-rows-escape-all-controls · list rows escape every control character — Every field of `claude agents --json` origin has C0/C1 controls escaped as hex text, and in `list` table rows `\n` and `\t` are escaped too, so every row stays one fixed-width line. · why: stricter than the security plan's floor of keeping `\n` and `\t`; ratified · except: `wait` / `last` message text keeps `\n` and `\t`
decision/mcp-hidden-verb · mcp is a hidden verb — `mcp` and `hook` have no human design surface, and `mcp` appears in no `viola --help` row. · why: `mcp` is run by the plugin, never by a person.
decision/start-refusals-show-no-path-or-pid · Start refusals show no path or pid — The exit-1 refusals of `viola run`, `viola revive` and `viola adopt` print one fixed-message line and one hint per cause, and none shows a path, a pid, the recorded directory or a logged id.
contract/cli-callers/human · CLI callers · human on a TTY — The human calls the cli on a TTY.
contract/cli-colour-tokens/dim · CLI colour tokens · dim — SGR dim is laid on a whole `stale` row; the attention colour wins for `DIALOG`.
contract/cli-colour-tokens/reset · CLI colour tokens · reset — `\x1b[0m` at every colour depth, after every styled span.
contract/cli-message-escaping/control-characters · CLI message-mode escaping · control characters — In `wait`, `last` and a non-RFC 3339 `since` of `viola link`, every decoded control character (C0, DEL, C1) except `\n` and `\t` prints as `\xHH`, two uppercase hex digits; nothing is stripped.
contract/cli-refusal-line/fixed-unable-form · CLI refusal line · fixed unable form — The exit-1 refusals of `run`, `revive`, `adopt`, `open`, `focus`, `up` and `verify` and the exit-21 strict-modes refusals of `list`, `link`, `unlink`, `focus`, `send`, `wait`, `last`, `answer`, `pause`, `release` and `switch` print the fixed `unable: <text>` form.
contract/cli-refusal-line/hint · CLI refusal line · hint — A refusal is followed directly by one `hint:` line per reason, detail or cause.
contract/cli-refusal-line/two-space-form · CLI refusal line · two-space form — A refusal prints `unable  <name>  <reason>  <detail>` on stderr, two spaces between fields.
contract/cli-streams/stderr · CLI streams · stderr — stderr carries context lines, refusals with each refusal's `hint:` line directly after it, and errors.
contract/cli-surface/output-streams · CLI surface · stdout/stderr — The cli surface is the stdout and stderr of the `viola` binary.

## surface · ## Surface: cli · twin surface-cli
decision/cli-colour-amber-and-dim-only · CLI colour limited to amber and dim — CLI output colours nothing but the `DIALOG` word and `stale` rows, so no blue or other colour SGR is ever emitted. · why: The quiz limits CLI colour to amber and dim.
decision/no-hint-names-viola-release · No hint names viola release — No refusal hint ever names `viola release`; `release` stays a human verb. · why: The human-typing refusal reaches drivers, and a driver's `release` is refused (`-32602`, `release-from-driver`, exit 20).
decision/terminal-background-never-set · Terminal background never set — The CLI never sets a background colour, at any colour depth. · why: The terminal background belongs to the user.
contract/cli-exit-codes/11 · CLI exit codes · 11 — Retired (was `budget-paused`) and never reused; a `budget-paused` refusal from an older wrapper reads `unknown`, exit 14.
contract/cli-stderr/error-line · CLI stderr · error line — Errors print on stderr as `error: …`.
contract/human-tables/width-source · human tables · width source — The table width is stdout's own terminal size; `COLUMNS` is not read, and a piped stdout has no width.
contract/viola-help/verb-groups · viola --help · verb groups — `viola --help` lists the served verbs only, grouped `board: list, focus` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: open, run, revive, adopt, up, switch, verify, plugin install`, then `Options:`.
decision/one-progress-pattern · One progress pattern — A step counter with static appended stdout lines is the project's one progress pattern.
contract/cli-colour-tokens/handoff-blue · CLI colour tokens · handoff blue — Handoff blue #5B8DB8 is not used at any colour depth and no index is reserved for it; link output (`overseer -> builder`) is plain.
contract/cli-exit-codes/13 · CLI exit codes · 13 — `unable  not-delivered` with `input-not-ready`, `no-prompt-submitted`, `turn-running`, `unknown-dialog`, `control-character`, `empty-text`, `paste-cut`, `dialog-pending`, `no-session`, `child-not-ended` or `resume-failed`; under `--json` the refusal object.
contract/cli-stderr/hint-line · CLI stderr · hint line — Each refusal's `hint:` line follows it directly on stderr.
contract/cli-stderr/refusal-line · CLI stderr · refusal line — A refusal prints `unable  <name>  <reason>  <detail>`, two-space separated; `viola send` prefixes the `[/ ]` mirror and pads `unable` to the readback-word column.
contract/refusal-hints/not-delivered-input-not-ready · refusal hints · not-delivered · input-not-ready — `<name> was not ready for input; send again, and if it repeats a human must look at the session`
contract/refusal-hints/not-delivered-no-prompt-submitted · refusal hints · not-delivered · no-prompt-submitted — `<name> did not submit the prompt; check it, then send again`
contract/refusal-hints/not-delivered-paste-cut · refusal hints · not-delivered · paste-cut — `<name> did not take the whole text in time; a human must look at the session before the next send`, printed in human mode only.
contract/refusal-hints/refusal-line · refusal hints · refusal line — A refusal prints `unable` plus `reason  detail`, then one `hint:` line keyed by the reason, or by reason and detail where listed; `send` and `answer` print the same hint for the same reason or detail, and a `not-delivered` refusal with an opaque detail or none prints no hint.
contract/viola-list-board/strict-modes-scan · viola list board · strict-modes scan — The scan holds the home to the read rule and `instances/` to the write rule, then each instance's set; a home another user may read refuses the whole board with the home-state pair, with or without an instance.
contract/viola-verify-output/summary-line · viola verify output · summary line — The last stdout line is a summary whose counts add up to `MM`: `stamped <version>  <n> pass  <m> fail`, `n` counting the rows read `pass` and the `browser-tools-prompt` row read `absent`; a failing row prints no stderr word.

## anti-patterns-never-do-these · ## Anti-Patterns (NEVER do these)
decision/colour-carries-meaning · Colour carries meaning, never decoration — Every colour communicates meaning: amber means `dialog_pending` and nothing else, blue means handoff and nothing else, and no third hue is used. · why: Any third hue has no domain meaning.
decision/colour-only-as-second-cue · Colour only as a second cue — Only `DIALOG` (amber) and `stale` rows (dim) are coloured, and never without their word beside them. · why: The quiz confirms colour as a second cue only.
decision/escape-controls-in-human-output · Control characters escaped in human output — Human output of `last` and `list` escapes C0/C1 control characters.
decision/no-check-glyph-or-emoji-prefixes · No check-mark or emoji prefixes — Output never uses green check / red cross prefixes or emoji; the readback mirror `[RB]` / `[  ]` / `[/ ]` and the words `read back` / `unable` are the prefixes.
decision/no-hardcoded-widths-or-wrapping · No hardcoded widths or wrapping — Widths are never hardcoded; output is truncated only where the terminal width is known, and never wrapped.
decision/no-internals-in-errors · No internals in errors or hints — Errors and hints never print upstream text, paths, pids or anyhow chains with serde sources. · why: The security plan's NEVER-log floor.
decision/no-shimmer-or-glow-theme · No skeleton shimmer or glow theme — No skeleton shimmer and no terminal-green or neon-glow theme; missing values print `unknown` in their box at once. · why: Nothing glows in a tower cab unless the lamp is on it.
decision/plain-output-modes · Plain output modes — No colour, glyphs, cursor control or spinner are emitted under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run`, or a passed `viola revive`. · why: LLM drivers parse this output, and `viola run`'s terminal belongs to the child's screen.
decision/stdout-data-stderr-messages · Data on stdout, messages on stderr — Results go to stdout; `waiting:`, refusals with their `hint:` lines, and errors go to stderr.
decision/no-spinners-in-verb-output · No spinners in verb output — A verb's output has no spinner, progress bar or live-redrawing dashboard; `wait` prints one static `waiting:` line and `verify` prints step-counter lines. · why: A spinner is progress presumed before it is confirmed.
decision/no-traffic-light-status-dots · No traffic-light status dots — Status is never a green/yellow/red dot; `stale` is a coasting lamp-off track, not red, and the only colour state is the cocked strip's amber beside its `DIALOG` word. · why: A strip prints its status word in a fixed column.

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
name/command/focus — focus
name/command/hook — hook
name/command/last — last
name/command/link — link
name/command/list — list
name/command/mcp — mcp
name/command/open — open
name/command/pause — pause
name/command/plugin-install — plugin install
name/command/release — release
name/command/revive — revive
name/command/run — run
name/command/send — send
name/command/unlink — unlink
name/command/up — up
name/command/verify — verify
name/command/wait — wait

## self-validation-protocol · ## Self-Validation Protocol
4. Token Test › N/A — viola renders no page, so no stylesheet tokens to trace. ¶ 4. Token Test › N/A — viola renders no page, so no stylesheet token to trace.
