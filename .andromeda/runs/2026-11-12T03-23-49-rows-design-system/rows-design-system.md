@row_none n=1 rec=fct-5eqdszko@1 type=0
what: a brand personality statement in metaphor
@row_none n=2 rec=fct-ggkugwi5@1 type=S
@row_add n=1 row=decision/one-strip-per-run-instance area=brand-identity rec=fct-2fkrm5qy@1
name: One flight-progress strip per instance
rule: Every `viola run` instance is shown as exactly one flight-progress strip.
@row_add n=2 row=contract/flight-progress-strip/name area=brand-identity rec=fct-2fkrm5qy@1
boundary: Flight-progress strip
item: NAME
promise: NAME is the first of the six fixed fields, set at callsign weight.
@row_add n=3 row=contract/flight-progress-strip/live area=brand-identity rec=fct-2fkrm5qy@1
boundary: Flight-progress strip
item: LIVE
promise: LIVE is the second of the six fixed fields.
@row_add n=4 row=contract/flight-progress-strip/status area=brand-identity rec=fct-2fkrm5qy@1
boundary: Flight-progress strip
item: STATUS
promise: STATUS is the third of the six fixed fields.
@row_add n=5 row=contract/flight-progress-strip/wheel area=brand-identity rec=fct-2fkrm5qy@1
boundary: Flight-progress strip
item: WHEEL
promise: WHEEL is the fourth of the six fixed fields.
@row_add n=6 row=contract/flight-progress-strip/dialog area=brand-identity rec=fct-2fkrm5qy@1
boundary: Flight-progress strip
item: DIALOG
promise: DIALOG is the fifth of the six fixed fields.
@row_add n=7 row=contract/flight-progress-strip/cli area=brand-identity rec=fct-2fkrm5qy@1
boundary: Flight-progress strip
item: CLI
promise: CLI is the sixth and last of the six fixed fields.
@row_add n=8 row=contract/flight-progress-strip/no-reading area=brand-identity rec=fct-2fkrm5qy@1,fct-oudd7ujq@2
boundary: Flight-progress strip
item: field with no reading
promise: A field with no reading prints `unknown` in its box, never a shimmer or a guess.
@row_add n=9 row=decision/stable-session-slots area=brand-identity rec=fct-kzedbiyf@1
name: Stable session slots
rule: Every session keeps a stable slot in the strip bay for its whole life, and nothing is re-sorted.
@row_add n=10 row=decision/dialog-pending-cocked-strip area=brand-identity rec=fct-sovfvsa2@1,fct-ve72ntsx@2
name: Dialog-pending shown as a cocked strip
rule: A `dialog_pending` session keeps its slot and shows the word `DIALOG` plus the dialog kind, the word in arrival-holder amber on the CLI.
@row_add n=11 row=contract/send-readback/prompt area=brand-identity rec=fct-l76aktpo@1,fct-v5ralmpr@1
boundary: Send readback
item: prompt send
promise: A prompt send counts, and its readback box fills, only once the matching `prompt-submitted` reads it back.
@row_add n=12 row=contract/send-readback/clear area=brand-identity rec=fct-l76aktpo@1,fct-v5ralmpr@1
boundary: Send readback
item: /clear
promise: A `/clear` send counts only once its post-condition, a `session-start` with cause `clear` (a new session), is observed.
@row_add n=13 row=contract/send-readback/compact area=brand-identity rec=fct-l76aktpo@1,fct-v5ralmpr@1
boundary: Send readback
item: /compact
promise: On a verified CLI a `/compact` send counts only once a `session-start` of cause `compact` carrying the session's own id is observed.
@row_add n=14 row=contract/send-readback/unconfirmable area=brand-identity rec=fct-l76aktpo@1
boundary: Send readback
item: unconfirmable
promise: On an unverified CLI a `/compact` send stays `unconfirmable`.
@row_add n=15 row=contract/send-readback/unable area=brand-identity rec=fct-l76aktpo@1
boundary: Send readback
item: unable
promise: A send not read back is reported as `unable` plus a typed reason.
@row_add n=16 row=contract/handoff/transfer-marker area=brand-identity rec=fct-dpdlxhra@1
boundary: Handoff display
item: transfer marker
promise: Transfer of control is shown as a `driver → driven` transfer marker.
@row_add n=17 row=decision/event-tape-played-in-order area=brand-identity rec=fct-dpdlxhra@1
name: Event tape played in order
rule: The ndjson event tape is played in order, with `skipped` counts kept where they happened.
@row_add n=18 row=decision/bare-strips-never-target area=brand-identity rec=fct-ghjs2mx3@1,fct-juf7qttk@2
name: Unwrapped sessions are never a target
rule: Wrapped instances sit in a holder; unwrapped sessions are bare strips, visible but never a `target`.
@row_add n=19 row=decision/stale-strip-dimmed-not-deleted area=brand-identity rec=fct-2ks73rzm@1
name: Stale strip dimmed, never deleted
rule: A strip of `stale` liveness is dimmed to lamp-off, never deleted, and keeps its slot.
@row_add n=20 row=decision/budget-paused-stops-new-sends area=brand-identity rec=fct-iqsnuw4w@1
name: Budget-paused as minimum-fuel state
rule: The budget governor shows `5H` / `7D` figures, and `budget-paused` is a declared state that stops new sends while the running turn finishes.
@row_add n=21 row=decision/readings-carry-their-age area=brand-identity rec=fct-xvme6vpp@1,fct-oudd7ujq@2
name: Readings carry their age
rule: Every budget reading carries its age (`read 4m ago`), and old information is shown as old (`expired`), never as current.
@row_add n=22 row=contract/cli-verification/verified area=brand-identity rec=fct-tmhb2v3c@1
boundary: CLI verification state
item: verified
promise: A CLI version stamped by `viola verify` prints `verified`.
@row_add n=23 row=contract/cli-verification/unverified-cli area=brand-identity rec=fct-tmhb2v3c@1
boundary: CLI verification state
item: unverified-cli
promise: An unstamped CLI build runs transport only, shown as `unverified-cli`.
@row_add n=24 row=contract/flight-progress-strip/wheel-human area=brand-identity rec=fct-vgjq5gux@1
boundary: Flight-progress strip
item: human
promise: A human holding control shows `human` in WHEEL at weight 600.
@row_add n=25 row=decision/pause-release-transfer-control area=brand-identity rec=fct-vgjq5gux@1
name: Pause and release as control transfer
rule: `viola pause` is the human's "I have control" and `viola release` is "you have control".
@row_add n=26 row=decision/fixed-phraseology-vocabulary area=brand-identity rec=fct-geooqybe@1
name: Fixed phraseology vocabulary
rule: Short fixed words (`open`, `RB`, `unable`, `human-typing`, `budget-paused`, `not-delivered`) are the vocabulary of every label, CLI column and refusal, with no filler and no guessing.
@row_add n=27 row=decision/readback-box area=brand-identity rec=fct-zdzmu6qp@2
name: The readback box as signature element
rule: The readback box is a fixed-width, 1px-ruled square cell whose one place is the CLI mirror, with three drawn states each also printed as a word.
@row_add n=28 row=contract/readback-box/open area=brand-identity rec=fct-tx2uihaa@1
boundary: Readback box
item: open
promise: The `open` state is drawn as a buff outline on anthracite.
@row_add n=29 row=contract/readback-box/rb area=brand-identity rec=fct-v5ralmpr@1
boundary: Readback box
item: RB / read back
promise: The `RB` state is filled solid buff with `RB` in graphite, filled instantly when the send's readback lands.
@row_add n=30 row=contract/readback-box/unable area=brand-identity rec=fct-k5hjyw65@1
boundary: Readback box
item: unable
promise: The `unable` state is the outline plus one 1px `/` strike, with the typed reason in the next cell.
@row_add n=31 row=contract/readback-box/unconfirmable area=brand-identity rec=fct-a6oxvtbj@1
boundary: Readback box
item: unconfirmable
promise: The printed word `unconfirmable` reuses the open drawing and is never filled.
@row_add n=32 row=decision/turn-ended-never-fills-readback area=brand-identity rec=fct-a6oxvtbj@1,fct-juf7qttk@2
name: Turn-ended never fills the readback box
rule: A `turn-ended` event never fills a readback box.
@row_add n=33 row=contract/readback-box/cli-mirror area=brand-identity rec=fct-a6oxvtbj@1,fct-qwc3u462@1
boundary: Readback box
item: CLI mirror
promise: The CLI mirrors the box states as `[RB]` / `[  ]` / `[/ ]`.
@row_add n=34 row=decision/expression-level area=brand-identity rec=fct-cggcilgt@1,fct-4m7ag2lh@2,fct-wjefhzv3@1
name: Expression level 0.2
rule: The base expression level `0.2` is the single source of truth for animation intensity, layout freedom, interaction complexity and motion across the whole project.
@row_none n=3 rec=fct-raot4n4o@1 type=S
@row_add n=35 row=decision/cli-has-no-motion area=brand-identity rec=fct-4m7ag2lh@2
name: No motion in the CLI
rule: The cli (0.2 on a TTY, 0.0 otherwise) has no motion at all.
@row_add n=36 row=overview/brand-identity area=brand-identity rec=fct-iy6nu2f4@2
text: The direction the CLI carries, and the terminal fleet console inherits, is a control-tower strip bay after dark: buff paper strips in anthracite holder boots on ruled aluminium racks. Every session holds the same slot for its whole life, and the only warm light is the one arrival holder cocked out of line when a session needs the human.
@row_add n=37 row=decision/no-success-before-confirmation area=brand-identity rec=fct-oudd7ujq@2
name: Nothing announces success before confirmation
rule: Nothing announces success before it is confirmed; sends stay visibly open until they are read back.
@row_add n=38 row=decision/strip-board-density area=brand-identity rec=fct-oudd7ujq@2
name: Calm strip-board density
rule: The density is a strip board's, not a monitoring dashboard's: fixed fields, tabular numbers, one attention colour and one handoff colour, everything else paper and ink.
@row_add n=39 row=decision/style-direction area=brand-identity rec=fct-oudd7ujq@2
name: Precision and Density style direction
rule: The style direction is Precision & Density as primary with Utility & Function as secondary.
@row_add n=40 row=decision/rejected-style-presets area=brand-identity rec=fct-oudd7ujq@2
name: Rejected style presets
rule: The Dark Mode (OLED), Real-Time Monitoring (pulsing dots, blink, toasts) and HUD / Sci-Fi FUI (glow, telemetry animation) presets are not used.
@row_none n=4 rec=fct-le6pp75z@1 type=S
@row_add n=41 row=decision/monochrome-palette-ladder area=color-palette rec=fct-zwz5bcg2@1
name: Monochrome palette ladder
rule: The palette is a monochrome ladder of three surfaces (#1E2124, #2A2E33, #4A5057) and three inks (#E6D8AE, #9C9278, #3B3A36).
why: All values come from the Color World of the tower cab strip bay at night.
@row_add n=42 row=decision/two-hues-one-role-each area=color-palette rec=fct-zwz5bcg2@1
name: Two hues, each locked to one role
rule: Only two hues are allowed beyond the monochrome ladder, and each is locked to one role.
@row_add n=43 row=decision/amber-only-for-dialog-pending area=color-palette rec=fct-sjonlmz3@1,fct-w6mrbusn@1
name: Arrival amber for dialog-pending only
rule: Arrival-holder amber #D97706 is the only attention colour, used for `dialog_pending` only.
except: never for errors, warnings, budget or hover
@row_add n=44 row=decision/blue-only-for-transfer-markers area=color-palette rec=fct-oav5xuxc@1,fct-w6mrbusn@1
name: Departure blue for handoff only
rule: Departure-holder blue #5B8DB8 is used for transfer markers and `link` / `unlink` names only, never as a fill, never as text on holder, never for a status.
@row_add n=45 row=decision/handoff-blue-lightened area=color-palette rec=fct-uqrncnsb@1
name: Handoff blue lightened for contrast
rule: The route blue is #5B8DB8, lightened from #2563EB.
why: #2563EB reaches only 3.5:1 contrast and fails.
@row_add n=46 row=decision/no-green-accent area=color-palette rec=fct-uqrncnsb@1
name: No run-green accent
rule: The run-green accent of the Developer Tool palette is not used; amber is the single sharp accent.
@row_add n=47 row=contract/color-roles/primary area=color-palette rec=fct-w6mrbusn@1
boundary: Colour roles
item: Primary #E6D8AE
promise: Strip buff is the primary ink, focus rings, information-carrying borders and the read-back fill.
@row_add n=48 row=contract/color-roles/secondary area=color-palette rec=fct-w6mrbusn@1
boundary: Colour roles
item: Secondary #9C9278
promise: Lamp-off buff is the secondary ink on anthracite, the in-strip field grid and all ink of a `stale` strip.
@row_add n=49 row=decision/refused-send-folds-into-send-line area=color-palette rec=fct-juf7qttk@2
name: Refused send folds into its send line
rule: A refused send folds into its send line, never a separate line.
@row_add n=50 row=decision/skipped-count-stays-visible area=color-palette rec=fct-juf7qttk@2
name: Skipped count stays visible
rule: A nonzero `skipped` count stays visible and is never auto-cleared.
@row_add n=51 row=decision/bare-strip-never-cocked area=color-palette rec=fct-juf7qttk@2
name: Bare strip never cocked
rule: A bare (unwrapped) strip is never shown cocked.
@row_add n=52 row=decision/busy-not-animated area=color-palette rec=fct-juf7qttk@2
name: Busy has no animation
rule: The `busy` state has no spinner, pulse or animation.
@row_none n=5 rec=fct-ajwnfbif@1 type=8
@row_add n=53 row=contract/borders/subtle area=color-palette rec=fct-o7ualksx@1
boundary: Border progression
item: Subtle #4A5057
promise: The subtle rail border is decorative only (2.0:1).
@row_add n=54 row=contract/borders/standard area=color-palette rec=fct-o7ualksx@1
boundary: Border progression
item: Standard #9C9278
promise: The standard border is the field grid inside a lit strip and the box around an ATIS `expired` word.
@row_add n=55 row=contract/borders/emphasis area=color-palette rec=fct-o7ualksx@1
boundary: Border progression
item: Emphasis #E6D8AE
promise: The emphasis border is for borders whose presence is the state.
@row_add n=56 row=contract/borders/focus area=color-palette rec=fct-o7ualksx@1
boundary: Border progression
item: Focus #E6D8AE
promise: The focus ring is 2px solid, offset 2px, radius 0.
@row_add n=57 row=contract/borders/handoff area=color-palette rec=fct-o7ualksx@1
boundary: Border progression
item: Handoff #5B8DB8
promise: The handoff border is only the transfer markers' 2px left tick, the only blue rule, never on holder, never a strip or cell border.
@row_none n=6 rec=fct-lx6q6b3t@1 type=S
@row_add n=58 row=decision/cli-terminal-font area=typography rec=fct-qwc3u462@1
name: CLI uses the terminal's own font
rule: The cli uses the user's configured terminal monospace; the brand shows through the fixed-column structure and the phraseology words.
@row_add n=59 row=decision/bold-only-name-column area=typography rec=fct-qwc3u462@1
name: Bold only for the NAME column
rule: In the cli, bold is used only for the NAME column.
@row_add n=60 row=decision/stale-rows-sgr-dim area=typography rec=fct-qwc3u462@1
name: Stale rows in SGR dim
rule: In the cli, SGR dim marks `stale` rows and amber marks the `DIALOG` word.
@row_none n=7 rec=fct-trso3zrq@1 type=S
@row_add n=61 row=decision/cli-two-space-columns area=spacing rec=fct-ae76qcah@1,fct-ukpiycjz@2
name: Two spaces between CLI columns
rule: CLI spacing follows the fixed-width terminal grid, with two spaces between columns.
@row_none n=8 rec=fct-herdsm72@1 type=S
@row_none n=9 rec=fct-fyedyzk2@1 type=S
@row_none n=10 rec=fct-7ghkfhm4@1 type=S
@row_none n=11 rec=fct-3inm2aon@1 type=S
@row_add n=62 row=decision/text-glyphs-only area=iconography rec=fct-le2hvglu@1
name: Custom text glyphs only
rule: Iconography is custom text glyphs only; phraseology words carry the meaning.
@row_add n=63 row=contract/cli-glyphs/arrow area=iconography rec=fct-ukpiycjz@2
boundary: CLI glyphs
item: ->
promise: The arrow glyph is `->`, with no inbound form.
@row_add n=64 row=contract/cli-glyphs/field-separator area=iconography rec=fct-ukpiycjz@2
boundary: CLI glyphs
item: " - "
promise: ` - ` separates parts inside one field.
@row_add n=65 row=contract/viola-list/columns area=iconography rec=fct-ukpiycjz@2
boundary: viola list output
item: columns
promise: `viola list` carries no link column in v1, so its six columns stay fixed.
@row_add n=66 row=decision/no-decorative-icons area=iconography rec=fct-3pqyewvo@1
name: No decorative icons
rule: Icons clarify, not decorate: no robot or person icons, no status dots, no checkmarks and no warning triangles, since every such state is a printed word.
@row_none n=12 rec=fct-x2nilrtw@1 type=S
@row_add n=67 row=absence/web-page-surface area=surface-web-spa rec=fct-ll5xgfku@2,fct-gjdxui7c@2
capability: A web page or web surface of any kind
@row_none n=13 rec=fct-wj2ah4kx@1 type=S
@row_none n=14 rec=fct-b3qkww4h@1 type=S
@row_add n=68 row=decision/linux-terminal-platform area=surface-cli rec=fct-7z5o76y5@2
name: Linux terminals as the CLI platform
rule: The cli targets Linux terminals: the core on any current distribution, CI on `ubuntu-latest`.
@row_add n=69 row=contract/cli-surface/output-streams area=surface-cli rec=fct-rj57r756@1
boundary: CLI surface
item: stdout/stderr
promise: The cli surface is the stdout and stderr of the `viola` binary.
@row_add n=70 row=contract/cli-callers/human area=surface-cli rec=fct-35irlzoz@1
boundary: CLI callers
item: human on a TTY
promise: The human calls the cli on a TTY.
@row_add n=71 row=contract/cli-callers/llm-driver area=surface-cli rec=fct-35irlzoz@1
boundary: CLI callers
item: LLM driver sessions
promise: LLM driver sessions call the cli via `--json` or MCP.
@row_none n=15 rec=fct-f2egz6qm@1 type=S
@row_add n=72 row=decision/clap-without-colour-feature area=surface-cli rec=fct-3n5f23up@1
name: clap built without its colour feature
rule: Argument parsing uses clap (derive) built without its `color` feature, so clap's help and usage output is plain in every mode and no dependency reads a colour or terminal variable.
why: every styled byte of human output is viola's own SGR module's
@row_add n=73 row=decision/tty-detection-std-isterminal area=surface-cli rec=fct-7yh2z5by@1
name: TTY detection by std IsTerminal
rule: TTY detection uses std `IsTerminal`.
@row_add n=74 row=decision/hand-written-sgr-module area=surface-cli rec=fct-qu7ywsq4@1
name: Hand-written SGR styling module
rule: Styling is a small hand-written SGR module in the `viola` bin.
why: adds no dependency
@row_add n=75 row=contract/cli-colour-tokens/dim area=surface-cli rec=fct-dufqiv2l@1,fct-erfdav33@1
boundary: CLI colour tokens
item: dim
promise: SGR dim is laid on a whole `stale` row; the attention colour wins for `DIALOG`.
@row_add n=76 row=contract/cli-colour-tokens/bold area=surface-cli rec=fct-dufqiv2l@1,fct-erfdav33@1
boundary: CLI colour tokens
item: bold
promise: NAME values are bold.
@row_add n=77 row=decision/no-handoff-blue-in-cli area=surface-cli rec=fct-dufqiv2l@1
name: No handoff blue in the CLI
rule: The CLI never uses the handoff blue colour.
@row_add n=78 row=decision/no-terminal-background area=surface-cli rec=fct-dufqiv2l@1
name: No background colours
rule: CLI output never sets a background colour.
why: the terminal background belongs to the user
@row_add n=79 row=decision/ascii-mirror-words-always-print area=surface-cli rec=fct-w37zqv3v@1
name: ASCII mirror marks are words, not glyphs
rule: A glyph is a non-ASCII symbol; the ASCII readback mirror `[RB]` / `[  ]` / `[/ ]` and `->` are words that print in every human output, piped, `NO_COLOR` and `TERM=dumb` included, and only `--json` replaces them with its JSON document.
@row_add n=80 row=contract/cli-colour-decision-order/json area=surface-cli rec=fct-vhpny3ot@1,fct-w37zqv3v@1
boundary: CLI colour decision order
item: --json
promise: First match of the colour decision order: `--json` output has no colour, no glyph and no SGR.
@row_add n=81 row=contract/cli-colour-decision-order/viola-run area=surface-cli rec=fct-hod2cupz@1,fct-w37zqv3v@1
boundary: CLI colour decision order
item: viola run
promise: Second match of the colour decision order: under `viola run` no colour, glyph or SGR is printed, and while the child runs nothing is printed at all.
@row_add n=82 row=contract/cli-colour-decision-order/viola-hook area=surface-cli rec=fct-gu2srphh@1,fct-w37zqv3v@1
boundary: CLI colour decision order
item: viola hook
promise: Third match of the colour decision order: `viola hook` prints no colour, glyph or SGR and never writes stderr.
@row_add n=83 row=contract/cli-colour-decision-order/viola-mcp area=surface-cli rec=fct-gu2srphh@1,fct-w37zqv3v@1
boundary: CLI colour decision order
item: viola mcp
promise: Third match of the colour decision order: `viola mcp` prints no colour, glyph or SGR, and its stdout carries MCP frames only.
@row_add n=84 row=contract/cli-colour-decision-order/no-color area=surface-cli rec=fct-tgbprndw@1,fct-w37zqv3v@1
boundary: CLI colour decision order
item: NO_COLOR
promise: Fourth match of the colour decision order: with `NO_COLOR` set (present, an empty value included) output has no colour, no glyph and no SGR.
@row_add n=85 row=contract/cli-colour-decision-order/term-dumb area=surface-cli rec=fct-6xnn7dqw@1,fct-w37zqv3v@1
boundary: CLI colour decision order
item: TERM=dumb
promise: Fifth match of the colour decision order: with `TERM=dumb` output has no colour, no glyph and no SGR.
@row_add n=86 row=contract/cli-colour-decision-order/stdout-not-terminal area=surface-cli rec=fct-kbiy6u23@2,fct-w37zqv3v@1
boundary: CLI colour decision order
item: stdout is not a terminal
promise: Sixth match of the colour decision order: when stdout is not a terminal output has no colour, no glyph and no SGR.
@row_add n=87 row=contract/cli-colour-depth/truecolor area=surface-cli rec=fct-kvkzosks@1,fct-ukqwfmzr@1
boundary: CLI colour depth
item: truecolor
promise: Where colour applies, the depth is chosen once per process, first match wins: truecolor if `COLORTERM` is `truecolor` or `24bit`.
@row_add n=88 row=contract/cli-colour-depth/256-colour area=surface-cli rec=fct-vzhczpnh@2,fct-ukqwfmzr@1
boundary: CLI colour depth
item: 256-colour
promise: Where colour applies and truecolor did not match, the depth is 256-colour if `TERM` contains `256color`, chosen once per process.
@row_none n=16 rec=fct-wbrmyjrf@1 type=S
@row_add n=89 row=contract/cli-streams/stdout area=surface-cli rec=fct-rjt4bruj@1
boundary: CLI streams
item: stdout
promise: stdout carries results and data: tables, send/wait results, `last` text.
@row_add n=90 row=contract/cli-streams/stderr area=surface-cli rec=fct-zolhtb6x@1
boundary: CLI streams
item: stderr
promise: stderr carries context lines, refusals with each refusal's `hint:` line directly after it, and errors.
@row_add n=91 row=contract/cli-refusal-line/two-space-form area=surface-cli rec=fct-zolhtb6x@1,fct-5xeughsz@1
boundary: CLI refusal line
item: two-space form
promise: A refusal prints `unable  <name>  <reason>  <detail>` on stderr, two spaces between fields.
@row_add n=92 row=contract/cli-refusal-line/fixed-unable-form area=surface-cli rec=fct-zolhtb6x@1
boundary: CLI refusal line
item: fixed unable form
promise: The exit-1 refusals of `run`, `revive` and `verify` and the exit-21 strict-modes refusals of `list`, `link`, `unlink`, `send`, `wait`, `last`, `answer`, `pause` and `release` print the fixed `unable: <text>` form.
@row_add n=93 row=contract/cli-refusal-line/hint area=surface-cli rec=fct-5xeughsz@1,fct-zolhtb6x@1
boundary: CLI refusal line
item: hint
promise: A refusal is followed directly by one `hint:` line per reason, detail or cause.
@row_add n=94 row=decision/json-one-document-on-stdout area=surface-cli rec=fct-6xd7opxn@1
name: --json result is one stdout document
rule: Under `--json` the whole result, refusals and `error` objects included, is one JSON document on stdout with the typed exit code.
except: an oversize `viola answer` and an unusable `viola send` exit 2 with their fixed stderr line, no document; a `pump_child` error under `viola revive --json` prints none, the child holding the terminal
@row_add n=95 row=decision/silent-catch-sites area=surface-cli rec=fct-6xd7opxn@1
name: Silent catch sites
rule: At the catch site, `run`, `revive`'s start, `mcp` and `hook` stay silent.
@row_add n=96 row=decision/no-hint-under-json area=surface-cli rec=fct-6xd7opxn@1,fct-5xeughsz@1
name: No hint line under --json
rule: No `hint:` line is printed under `--json`; the typed `refusal` / `error` fields and their `detail` are the whole answer, exit 21's cause being its `detail`.
@row_none n=17 rec=fct-yvy7sgi6@1 type=S
@row_add n=97 row=decision/truncate-only-unwrapped-name area=surface-cli rec=fct-rhkxdzp7@1,fct-fldutxrq@1
name: Only an unwrapped NAME is truncated
rule: Where stdout's own terminal width is known, human tables truncate only the NAME of unwrapped rows (`...`); otherwise nothing is truncated.
@row_add n=98 row=decision/columns-never-read area=surface-cli rec=fct-rhkxdzp7@1
name: COLUMNS is never read
rule: Terminal width comes only from stdout's own terminal; `COLUMNS` is never read and a piped stdout has no width.
@row_add n=99 row=decision/lines-never-wrapped area=surface-cli rec=fct-fldutxrq@1
name: Lines never wrapped
rule: CLI output lines are never wrapped.
@row_add n=100 row=decision/ascii-only-human-output area=surface-cli rec=fct-xlz4sgiu@1
name: ASCII-only human output
rule: Human output is ASCII only (`->`, `[RB]`, `[/ ]`, `[  ]`).
@row_none n=18 rec=fct-eiyxwqq7@1 type=S
@row_add n=101 row=contract/viola-list-table/columns area=surface-cli rec=fct-pcmgk4u5@1
boundary: viola list table
item: columns
promise: Under a BAY line the table has the columns NAME, LIVE, STATUS, WHEEL, DIALOG and CLI; unwrapped sessions follow a `-- UNWRAPPED - READ-ONLY --` line.
@row_add n=102 row=contract/viola-list-table/row-order area=surface-cli rec=fct-6pisha67@1
boundary: viola list table
item: row order
promise: Rows are ordered by name.
@row_add n=103 row=contract/viola-list-table/bay-line area=surface-cli rec=fct-dbxcldrz@1
boundary: viola list table
item: BAY line
promise: The BAY line is the ATIS without `resets` times (printed by `viola release <name> --budget`) and without a TAPE cell.
@row_add n=104 row=decision/expired-window-shown-expired area=surface-cli rec=fct-dbxcldrz@1
name: Expired budget window shown as expired
rule: A budget window whose `resets_at` has passed still prints its figure followed by `expired`.
why: old information is shown as old
@row_add n=105 row=decision/list-rows-escape-all-controls area=surface-cli rec=fct-hefy4zzc@1
name: list rows escape every control character
rule: Every field of `claude agents --json` origin has C0/C1 controls escaped as hex text, and in `list` table rows `\n` and `\t` are escaped too, so every row stays one fixed-width line.
why: stricter than the security plan's floor of keeping `\n` and `\t`; ratified
except: `wait` / `last` message text keeps `\n` and `\t`
@row_add n=106 row=contract/viola-list-table/empty area=surface-cli rec=fct-7y3vb6h4@1
boundary: viola list table
item: empty
promise: With no wrapped sessions the board prints `no wrapped sessions  start one: viola run <name> -- claude`.
@row_none n=19 rec=fct-mfvo2or6@1 type=4
relied: A failed strict-modes check refuses the whole board with exit 21, in the fixed `unable:` / `hint:` form or under `--json` the `instance-unreachable` document with `detail` `strict-modes-failed`.
@row_none n=20 rec=fct-dgyptkjb@1 type=S
@row_add n=107 row=contract/viola-send-mirror/open area=surface-cli rec=fct-ahmeujab@1
boundary: viola send readback mirror
item: open
promise: `[  ] open  <name>  issued <time>` prints on stderr, TTY only.
@row_add n=108 row=contract/viola-send-mirror/read-back area=surface-cli rec=fct-ahmeujab@1
boundary: viola send readback mirror
item: read back
promise: `[RB] read back  <name>  <time>  cursor <n>` prints on stdout with exit 0.
@row_add n=109 row=contract/viola-send-mirror/unable area=surface-cli rec=fct-ahmeujab@1
boundary: viola send readback mirror
item: unable
promise: `[/ ] unable  <name>  <reason>  <detail>` prints on stderr with exit 13, followed by its `hint:` line.
@row_add n=110 row=contract/viola-send-mirror/unconfirmable area=surface-cli rec=fct-ahmeujab@1
boundary: viola send readback mirror
item: unconfirmable
promise: `[  ] unconfirmable  <name>  local command, no measured post-condition` prints on stdout with exit 0.
@row_add n=111 row=contract/cli-strict-modes-refusal/instance-state-pair area=surface-cli rec=fct-5xeughsz@1
boundary: CLI strict-modes refusal
item: instance-state pair
promise: The exit-21 strict-modes refusal of `send`, `wait`, `last`, `answer`, `pause` and `release` prints the fixed instance-state `unable:` / `hint:` pair, not the `[/ ]` mirror.
@row_add n=112 row=contract/cli-start-refusal/fixed-message area=surface-cli rec=fct-5xeughsz@1,fct-w4pl2fni@1
boundary: run and revive start refusals
item: fixed message
promise: A start refusal of `run` or `revive` prints a fixed-message `unable:` line and one hint per cause; `revive`'s causes are read in a fixed order that stops at the first.
@row_add n=113 row=decision/no-hint-names-release area=surface-cli rec=fct-5xeughsz@1
name: No hint names viola release
rule: No hint names `viola release`.
why: a driver's `release` is refused
@row_add n=114 row=decision/hint-never-quotes-text area=surface-cli rec=fct-syupqkpm@1
name: Hints never quote text
rule: A hint never quotes the sent text or any upstream text.
@row_none n=21 rec=fct-vucxgwl4@1 type=S
@row_add n=115 row=contract/viola-wait-output/waiting-line area=surface-cli rec=fct-iee6axpq@1
boundary: viola wait output
item: waiting line
promise: `wait` prints one static `waiting: <name>` line on stderr, TTY only, before its one stdout result line.
@row_add n=116 row=contract/viola-wait-output/event-form area=surface-cli rec=fct-iee6axpq@1
boundary: viola wait output
item: event form
promise: `turn-ended`, `session-end` and every other kind without a `dialog_id` print `<kind>  <name>  <time>  cursor <n>` on stdout.
@row_add n=117 row=contract/viola-wait-output/dialog-form area=surface-cli rec=fct-iee6axpq@1
boundary: viola wait output
item: dialog form
promise: `question`, `permission`, `plan` and `dialog-expired` print `<kind>  <name>  dialog <id>  cursor <n>` on stdout, uncoloured, with `dialog unknown` when the event carries no `dialog_id`.
@row_add n=118 row=contract/viola-wait-output/timed-out area=surface-cli rec=fct-iee6axpq@1
boundary: viola wait output
item: timed out
promise: A timeout prints `timed out  <name>  <ms> ms` on stdout.
@row_add n=119 row=decision/no-spinner area=surface-cli rec=fct-qr5hytcv@1
name: No spinner or elapsed-time counter
rule: The CLI shows no spinner and no elapsed-time counter.
@row_add n=120 row=contract/viola-last-output/header area=surface-cli rec=fct-j6urnvep@1
boundary: viola last output
item: header
promise: `last` prints `last  <name>  <kind> <time>` on stderr, TTY only.
@row_add n=121 row=contract/viola-last-output/message area=surface-cli rec=fct-j6urnvep@1
boundary: viola last output
item: message
promise: `last` prints the message text on stdout with C0/C1 escaped, or `no message` on stderr when the message is `null`.
@row_add n=122 row=contract/cli-message-escaping/control-characters area=surface-cli rec=fct-gbzjruf2@1
boundary: CLI message-mode escaping
item: control characters
promise: In `wait`, `last` and a non-RFC 3339 `since` of `viola link`, every decoded control character (C0, DEL, C1) except `\n` and `\t` prints as `\xHH`, two uppercase hex digits; nothing is stripped.
@row_add n=123 row=contract/cli-message-escaping/json area=surface-cli rec=fct-gbzjruf2@1
boundary: CLI message-mode escaping
item: --json
promise: `--json` output stays serde-escaped.
@row_none n=22 rec=fct-mnnc3kso@1 type=S
@row_add n=124 row=contract/cli-verb-output/pause area=surface-cli rec=fct-s5g62jbv@1
boundary: CLI verb output
item: viola pause
promise: `viola pause <name>` prints `<name>  wheel human  manual-pause  I have control`.
@row_add n=125 row=contract/cli-verb-output/release area=surface-cli rec=fct-3wxbibgp@1
boundary: CLI verb output
item: viola release
promise: `viola release <name>` prints `<name>  wheel driver  you have control`.
@row_add n=126 row=contract/cli-verb-output/release-budget area=surface-cli rec=fct-ybxjcc2w@1
boundary: CLI verb output
item: viola release --budget
promise: Prints one unstyled stdout line, `<name>  budget gate lifted until <time>  wheel human` when a pause stood, else `<name>  budget gate open  wheel human`.
@row_add n=127 row=contract/cli-verb-output/link area=surface-cli rec=fct-glxbi6ql@1
boundary: CLI verb output
item: viola link
promise: `viola link <a> <b>` prints `link    <a> -> <b>  since <time>`, or `link    <a> -> <b>  already linked` for an existing pair.
@row_add n=128 row=contract/cli-verb-output/unlink area=surface-cli rec=fct-vmpiq5lh@1
boundary: CLI verb output
item: viola unlink
promise: `viola unlink <a> <b>` prints `unlink  <a> -> <b>`.
@row_add n=129 row=contract/cli-verb-output/answer area=surface-cli rec=fct-b2gogylx@1
boundary: CLI verb output
item: viola answer
promise: `viola answer <name> <id>` prints `answered  <name>  dialog <id>`, or `unable  <name>  not-delivered  unknown-dialog`.
@row_add n=130 row=contract/cli-verb-output/plugin-install area=surface-cli rec=fct-65srfuid@1
boundary: CLI verb output
item: viola plugin install
promise: Prints four plain ASCII stdout lines in one write, no SGR: `viola@viola  written  <dir>`, `run these to add it to Claude Code:`, `  claude plugin marketplace add <dir>`, `  claude plugin install viola@viola`; `<dir>` is quoted only where the host's shell would split it.
@row_none n=23 rec=fct-2uvmlzuw@1 type=S
@row_add n=131 row=decision/step-counter-only-progress-pattern area=surface-cli rec=fct-nz6d6pbn@1
name: Step counter as the one progress pattern
rule: The step counter of `verify` is the project's one progress pattern.
@row_add n=132 row=contract/viola-verify-output/summary area=surface-cli rec=fct-nz6d6pbn@1
boundary: viola verify output
item: summary
promise: A summary line `stamped <ver>  <n> pass  <m> fail` follows, n + m being the relied count.
@row_add n=133 row=contract/viola-verify-output/stopped-or-refused area=surface-cli rec=fct-nz6d6pbn@1
boundary: viola verify output
item: stopped or refused run
promise: A run stopped by Ctrl-C or TERM before its rows are checked, or refused by its home tree's strict-modes check before its log, prints no step line and no summary, only its fixed `unable:` / `hint:` pair, and exits 1.
@row_add n=134 row=contract/viola-verify-output/stamps-refused area=surface-cli rec=fct-nz6d6pbn@1
boundary: viola verify output
item: stamps.json update refused
promise: A `stamps.json` update refused by its strict-modes check prints the home-state `unable:` / `hint:` pair.
@row_add n=135 row=contract/viola-run-output/child-started area=surface-cli rec=fct-w4pl2fni@1,fct-2t54hjhs@2
boundary: viola run and revive output
item: child started
promise: `run` and `revive` print nothing once the child starts.
@row_add n=136 row=contract/viola-revive-output/list area=surface-cli rec=fct-2t54hjhs@2
boundary: viola revive output
item: --list
promise: `revive --list` prints one plain stdout line per logged session.
@row_add n=137 row=contract/revive-json-refusal-code/strict-modes-failed area=surface-cli rec=fct-2t54hjhs@2
boundary: revive --json refusal code
item: strict-modes-failed
promise: One of the closed set of codes a `revive --json` start refusal document carries.
@row_add n=138 row=contract/revive-json-refusal-code/already-live area=surface-cli rec=fct-2t54hjhs@2
boundary: revive --json refusal code
item: already-live
promise: One of the closed set of codes a `revive --json` start refusal document carries.
@row_add n=139 row=contract/revive-json-refusal-code/no-session area=surface-cli rec=fct-2t54hjhs@2
boundary: revive --json refusal code
item: no-session
promise: One of the closed set of codes a `revive --json` start refusal document carries.
@row_add n=140 row=contract/revive-json-refusal-code/session-live area=surface-cli rec=fct-2t54hjhs@2
boundary: revive --json refusal code
item: session-live
promise: One of the closed set of codes a `revive --json` start refusal document carries.
@row_add n=141 row=contract/revive-json-refusal-code/cwd-missing area=surface-cli rec=fct-2t54hjhs@2
boundary: revive --json refusal code
item: cwd-missing
promise: One of the closed set of codes a `revive --json` start refusal document carries.
@row_add n=142 row=contract/revive-json-refusal-code/stale-heartbeat area=surface-cli rec=fct-2t54hjhs@2
boundary: revive --json refusal code
item: stale-heartbeat
promise: One of the closed set of codes a `revive --json` start refusal document carries.
@row_add n=143 row=contract/revive-json-refusal-code/pinned-hash-mismatch area=surface-cli rec=fct-2t54hjhs@2
boundary: revive --json refusal code
item: pinned-hash-mismatch
promise: One of the closed set of codes a `revive --json` start refusal document carries.
@row_add n=144 row=contract/revive-json-refusal-code/squatted-name area=surface-cli rec=fct-2t54hjhs@2
boundary: revive --json refusal code
item: squatted-name
promise: One of the closed set of codes a `revive --json` start refusal document carries.
@row_add n=145 row=contract/revive-json-refusal-code/socket-dir-refused area=surface-cli rec=fct-2t54hjhs@2
boundary: revive --json refusal code
item: socket-dir-refused
promise: One of the closed set of codes a `revive --json` start refusal document carries.
@row_add n=146 row=contract/revive-json-refusal-code/internal-error area=surface-cli rec=fct-2t54hjhs@2
boundary: revive --json refusal code
item: internal-error
promise: One of the closed set of codes a `revive --json` start refusal document carries.
@row_none n=24 rec=fct-nrsgugtu@1 type=S
@row_add n=147 row=contract/cli-exit-codes/10-14 area=surface-cli rec=fct-tj4jq2zv@1
boundary: CLI exit codes
item: 10 to 14
promise: An `unable` refusal with its reason; a refusal object under `--json`.
@row_add n=148 row=contract/exit-21-cause/strict-modes-failed area=surface-cli rec=fct-tj4jq2zv@1
boundary: exit 21 cause
item: strict-modes-failed
promise: One of the closed set of causes named in exit 21's `detail`.
@row_add n=149 row=contract/exit-21-cause/instance-dead area=surface-cli rec=fct-tj4jq2zv@1
boundary: exit 21 cause
item: instance-dead
promise: One of the closed set of causes named in exit 21's `detail`.
@row_add n=150 row=contract/exit-21-cause/unwrapped-name area=surface-cli rec=fct-tj4jq2zv@1
boundary: exit 21 cause
item: unwrapped-name
promise: One of the closed set of causes named in exit 21's `detail`.
@row_add n=151 row=contract/exit-21-cause/endpoint-lost area=surface-cli rec=fct-tj4jq2zv@1
boundary: exit 21 cause
item: endpoint-lost
promise: One of the closed set of causes named in exit 21's `detail`.
@row_add n=152 row=contract/exit-21-cause/server-verify-failed area=surface-cli rec=fct-tj4jq2zv@1
boundary: exit 21 cause
item: server-verify-failed
promise: One of the closed set of causes named in exit 21's `detail`.
@row_add n=153 row=contract/exit-21-cause/socket-dir-refused area=surface-cli rec=fct-tj4jq2zv@1
boundary: exit 21 cause
item: socket-dir-refused
promise: One of the closed set of causes named in exit 21's `detail`.
@row_add n=154 row=contract/cli-strict-modes-refusal/plugin-install area=surface-cli rec=fct-t6bk3igt@1
boundary: CLI strict-modes refusal
item: viola plugin install
promise: A refused strict-modes check of `viola plugin install` exits 21 with the home-state `unable:` / `hint:` pair as `viola list`'s does, or under `--json` the `instance-unreachable` document with `detail` `strict-modes-failed`, stderr empty.
@row_add n=155 row=contract/strict-modes-refusal/exit-21 area=surface-cli rec=fct-ui5ydw4a@1
boundary: Instance strict-modes refusal
item: exit 21
promise: A refused strict-modes check of the driven instance by `viola link` / `unlink`, or of the instance of `send`, `wait`, `last`, `answer`, `pause` and `release`, is made before any frame, exits 21, and writes no frame and no line.
@row_add n=156 row=contract/strict-modes-refusal/human-output area=surface-cli rec=fct-ui5ydw4a@1
boundary: Instance strict-modes refusal
item: unable: / hint: pair
promise: The refusal prints the instance-state `unable:` / `hint:` pair naming that instance, as `viola list`'s does.
@row_add n=157 row=contract/strict-modes-refusal/json-document area=surface-cli rec=fct-ui5ydw4a@1
boundary: Instance strict-modes refusal
item: --json document
promise: Under `--json` the refusal prints the `instance-unreachable` document with `detail` `strict-modes-failed`, and stderr stays empty.
@row_add n=158 row=decision/flat-cli-verbs area=surface-cli rec=fct-jcytj7ki@1
name: Flat CLI verbs
rule: CLI verbs are flat, each one lower-case word, with no nested subcommand groups.
except: `plugin install`
@row_add n=159 row=name/command/run area=surface-cli rec=fct-jcytj7ki@1
class: command
name: run
@row_add n=160 row=name/command/revive area=surface-cli rec=fct-jcytj7ki@1
class: command
name: revive
@row_add n=161 row=name/command/send area=surface-cli rec=fct-jcytj7ki@1
class: command
name: send
@row_add n=162 row=name/command/wait area=surface-cli rec=fct-jcytj7ki@1
class: command
name: wait
@row_add n=163 row=name/command/last area=surface-cli rec=fct-jcytj7ki@1
class: command
name: last
@row_add n=164 row=name/command/list area=surface-cli rec=fct-jcytj7ki@1
class: command
name: list
@row_add n=165 row=name/command/answer area=surface-cli rec=fct-jcytj7ki@1
class: command
name: answer
@row_add n=166 row=name/command/verify area=surface-cli rec=fct-jcytj7ki@1
class: command
name: verify
@row_add n=167 row=name/command/pause area=surface-cli rec=fct-jcytj7ki@1
class: command
name: pause
@row_add n=168 row=name/command/release area=surface-cli rec=fct-jcytj7ki@1
class: command
name: release
@row_add n=169 row=name/command/link area=surface-cli rec=fct-jcytj7ki@1
class: command
name: link
@row_add n=170 row=name/command/unlink area=surface-cli rec=fct-jcytj7ki@1
class: command
name: unlink
@row_add n=171 row=name/command/plugin-install area=surface-cli rec=fct-jcytj7ki@1
class: command
name: plugin install
@row_add n=172 row=decision/list-is-the-board area=surface-cli rec=fct-3l74p2hv@2
name: viola list is the board
rule: `viola list` is the board, and every other verb acts on one strip.
@row_add n=173 row=decision/no-interactive-prompt-or-pager area=surface-cli rec=fct-3l74p2hv@2
name: No interactive prompt or pager
rule: No verb has an interactive prompt or a pager.
@row_add n=174 row=decision/fleet-console-not-verb-output area=surface-cli rec=fct-3l74p2hv@2,fct-izuy43cb@2
name: Fleet console is no verb's output
rule: The terminal fleet console is no verb's output, so verb output rules do not govern its redraw.
@row_add n=175 row=contract/help-output/verb-groups area=surface-cli rec=fct-yh7gnttc@1
boundary: viola --help output
item: verb groups
promise: `viola --help` lists the served verbs as `board: list` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: run, revive, verify, plugin install`, then `Options:`.
@row_add n=176 row=decision/prompt-text-from-stdin-or-file area=surface-cli rec=fct-7u3hdiu2@2
name: Prompt text from stdin or file only
rule: Prompt text comes only from stdin or `--file`.
@row_add n=177 row=contract/hook-verb/stderr area=surface-cli rec=fct-cj5433yq@1
boundary: The hook verb
item: stderr
promise: `hook` writes nothing to stderr.
@row_add n=178 row=contract/hook-verb/exit-code area=surface-cli rec=fct-cj5433yq@1
boundary: The hook verb
item: exit code
promise: `hook` always exits 0.
@row_add n=179 row=contract/mcp-verb/stdout area=surface-cli rec=fct-cj5433yq@1
boundary: The mcp verb
item: stdout
promise: `mcp` writes only MCP frames on stdout.
@row_add n=180 row=decision/mcp-hidden-verb area=surface-cli rec=fct-cj5433yq@1
name: mcp is a hidden verb
rule: `mcp` and `hook` have no human design surface, and `mcp` appears in no `viola --help` row.
why: `mcp` is run by the plugin, never by a person.
@row_add n=181 row=name/command/hook area=surface-cli rec=fct-cj5433yq@1
class: command
name: hook
@row_add n=182 row=name/command/mcp area=surface-cli rec=fct-cj5433yq@1
class: command
name: mcp
@row_add n=183 row=decision/no-stack-traces area=surface-cli rec=fct-zumogwyu@1,fct-wc7eewp7@1
name: No stack traces
rule: Stack traces never print; errors are fixed messages and full detail goes only to the instance's diagnostics.
@row_none n=25 rec=fct-7btysobr@1 type=S
@row_add n=184 row=decision/colour-carries-meaning area=anti-patterns-never-do-these rec=fct-zrpewlh4@1
name: Colour carries meaning, never decoration
rule: Every colour communicates meaning: amber means `dialog_pending` and nothing else, blue means handoff and nothing else, and no third hue is used.
why: Any third hue has no domain meaning.
@row_add n=185 row=decision/no-convergent-safe-choices area=anti-patterns-never-do-these rec=fct-gdfaropc@1
name: No convergence on safe design choices
rule: The design never converges on common safe choices across generations; the readback box and the cocked strip stand in their place.
why: Agent dashboards converge on dots, toasts and graphs.
@row_add n=186 row=decision/no-traffic-light-status-dots area=anti-patterns-never-do-these rec=fct-ltouy2sa@1
name: No traffic-light status dots
rule: Status is never a green/yellow/red dot; `stale` is a coasting lamp-off track, not red, and the only colour state is the cocked strip's amber beside its `DIALOG` word.
why: A strip prints its status word in a fixed column.
@row_add n=187 row=decision/no-optimistic-send-confirmation area=anti-patterns-never-do-these rec=fct-jkj5umki@1
name: No optimistic send confirmation
rule: The readback box stays `open` until the matching `prompt-submitted` or the command's post-condition, then fills or is struck with `unable` plus the typed reason; no spinner, green mark or toast.
why: A clearance does not count until it is read back.
@row_add n=188 row=decision/no-auto-sorting-strips area=anti-patterns-never-do-these rec=fct-m4eli7qh@2
name: No auto-sorting of strips
rule: Strips needing input or with recent sessions are never re-ranked to the top; a strip is cocked in place.
why: Re-ranking breaks the founder's memory of where each session lives, and the `DIALOG` word already says "needs you".
@row_add n=189 row=decision/budget-as-ruled-figures area=anti-patterns-never-do-these rec=fct-szsoezvc@1
name: Budget as figures in ruled fields
rule: Budget prints as figures in ruled fields with the ATIS reading age, never as rings or coloured bars; `budget-paused` is a word, not a colour.
@row_add n=190 row=decision/wheel-as-words area=anti-patterns-never-do-these rec=fct-cr4x4576@1
name: Wheel shown as words
rule: The WHEEL field prints `human` / `driver`, never icons, emoji or a toggle; taking control is spoken with `viola pause`.
why: Taking control is never implied by a switch.
@row_add n=191 row=decision/no-shimmer-or-glow-theme area=anti-patterns-never-do-these rec=fct-tj74erhr@1
name: No skeleton shimmer or glow theme
rule: No skeleton shimmer and no terminal-green or neon-glow theme; missing values print `unknown` in their box at once.
why: Nothing glows in a tower cab unless the lamp is on it.
@row_none n=26 rec=fct-rvsvzrdz@1 type=S
@row_add n=192 row=decision/no-check-glyph-or-emoji-prefixes area=anti-patterns-never-do-these rec=fct-pbywd7ut@1
name: No check-mark or emoji prefixes
rule: Output never uses green check / red cross prefixes or emoji; the readback mirror `[RB]` / `[  ]` / `[/ ]` and the words `read back` / `unable` are the prefixes.
@row_add n=193 row=decision/colour-only-as-second-cue area=anti-patterns-never-do-these rec=fct-ibtjtsuk@1,fct-zdmbmsfz@1
name: Colour only as a second cue
rule: Only `DIALOG` (amber) and `stale` rows (dim) are coloured, and never without their word beside them.
why: The quiz confirms colour as a second cue only.
@row_add n=194 row=decision/plain-output-modes area=anti-patterns-never-do-these rec=fct-jphnedpy@1,fct-e37xe7yc@1
name: Plain output modes
rule: No colour, glyphs, cursor control or spinner are emitted under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run`, or a passed `viola revive`.
why: LLM drivers parse this output, and `viola run`'s terminal belongs to the child's screen.
@row_add n=195 row=decision/no-spinners-in-verb-output area=anti-patterns-never-do-these rec=fct-izuy43cb@2
name: No spinners in verb output
rule: A verb's output has no spinner, progress bar or live-redrawing dashboard; `wait` prints one static `waiting:` line and `verify` prints step-counter lines.
why: A spinner is progress presumed before it is confirmed.
@row_add n=196 row=decision/stdout-data-stderr-messages area=anti-patterns-never-do-these rec=fct-oeutfz7w@1
name: Data on stdout, messages on stderr
rule: Results go to stdout; `waiting:`, refusals with their `hint:` lines, and errors go to stderr.
@row_add n=197 row=decision/no-internals-in-errors area=anti-patterns-never-do-these rec=fct-b7z6jyxk@1
name: No internals in errors or hints
rule: Errors and hints never print upstream text, paths, pids or anyhow chains with serde sources.
why: The security plan's NEVER-log floor.
@row_add n=198 row=decision/escape-controls-in-human-output area=anti-patterns-never-do-these rec=fct-b7z6jyxk@1
name: Control characters escaped in human output
rule: Human output of `last` and `list` escapes C0/C1 control characters.
@row_add n=199 row=decision/confirmations-carry-context area=anti-patterns-never-do-these rec=fct-blptucet@1
name: Confirmations carry context
rule: Output never says "done" or "success" bare; it says what was read back, by whom, and at which cursor.
@row_add n=200 row=decision/no-hardcoded-widths-or-wrapping area=anti-patterns-never-do-these rec=fct-olwc37rq@1
name: No hardcoded widths or wrapping
rule: Widths are never hardcoded; output is truncated only where the terminal width is known, and never wrapped.
@row_none n=27 rec=fct-oljfb7kw@1 type=S
@row_add n=201 row=decision/swap-test area=self-validation-protocol rec=fct-gzgahwnt@1,fct-xohxr7g5@1
name: Swap test
rule: Before UI output is presented, a design that would not feel meaningfully different with a default typeface, default colours and a standard layout is redone with intent.
@row_add n=202 row=decision/squint-test area=self-validation-protocol rec=fct-3fbi5jyf@1,fct-xohxr7g5@1
name: Squint test
rule: Before UI output is presented, hierarchy must still be perceivable with blurred eyes and nothing may jump out harshly.
except: `open` and `unconfirmable` readback boxes look identical under the squint, and only the word cell tells them apart, as intended.
@row_add n=203 row=decision/readback-box-signature-check area=self-validation-protocol rec=fct-4be5q2ri@2,fct-xohxr7g5@1
name: Readback box signature check
rule: Before UI output is presented, the readback box must be present and drawn the same in its one place, the `[RB]` / `[  ]` / `[/ ]` column of CLI `viola send`.
@row_add n=204 row=decision/domain-not-statistical-design area=self-validation-protocol rec=fct-g3srhwja@1,fct-xohxr7g5@1
name: Design from the product's domain
rule: Before UI output is presented, an interface another AI would produce from a similar prompt fails; the interface must emerge from this product's domain exploration.
@row_none n=28 rec=fct-ybq4pqty@1 type=S
@row_none n=29 rec=fct-tmafcfek@1 type=S
@row_none n=30 rec=fct-ok535url@1 type=0
what: a legend item of the generic expression-motion scale
@row_none n=31 rec=fct-mwfv3azd@1 type=0
what: a legend item of the generic expression-motion scale
@row_none n=32 rec=fct-hvwzk24f@1 type=0
what: a legend item of the generic expression-motion scale
@row_none n=33 rec=fct-kh7y66hg@1 type=0
what: a legend item of the generic expression-motion scale
@row_none n=34 rec=fct-a2vvlynz@1 type=0
what: a legend item of the generic expression-motion scale
@row_add n=205 row=decision/cli-tty-static-expression area=brand-identity rec=fct-zdmbmsfz@1
name: Static expression for the human CLI
rule: Human TTY CLI output is at expression 0.2: static fixed-column rows and one static context header, nothing redrawn in place and no spinners.
@row_add n=206 row=decision/run-prints-nothing-while-wrapped area=brand-identity rec=fct-e37xe7yc@1
name: viola run silent while wrapped
rule: `viola run` prints nothing at all while the wrapped `claude` TUI runs, and neither does a `viola revive` that passed its preflight.
@row_add n=207 row=contract/colour-palette/primary area=color-palette rec=fct-yz3mxga4@1
boundary: Colour palette
item: Primary #E6D8AE
promise: Primary ink, focus rings, information-carrying borders (readback box outline and strike, `budget-paused`, nonzero `skipped` and `TAPE stopped` boxes, error strips) and the read-back fill.
@row_add n=208 row=contract/colour-palette/secondary area=color-palette rec=fct-a6fekuxk@1
boundary: Colour palette
item: Secondary #9C9278
promise: Secondary ink on anthracite (timestamps, reading ages, column captions, `n/a`), the field-grid rule inside strips, the 1px box around an ATIS `expired` word, and all ink of a `stale` strip.
@row_add n=209 row=contract/colour-palette/accent area=color-palette rec=fct-dxnsp32p@1
boundary: Colour palette
item: Accent #D97706
promise: The only attention colour: the holder band and `DIALOG` word of a cocked strip and the CLI `DIALOG` word in `viola list`; never used for errors, warnings, budget or hover.
@row_add n=210 row=contract/colour-palette/handoff area=color-palette rec=fct-ylpqayvy@1
boundary: Colour palette
item: Handoff #5B8DB8
promise: Transfer-marker arrows and names, the marker's 2px left tick, and the names in `link` / `unlink` tape lines only; never a fill, never text on holder, never a status.
@row_add n=211 row=contract/border-intensity/subtle area=color-palette rec=fct-x44f65ku@1
boundary: Border intensities
item: Subtle #4A5057
promise: Decorative rack-rail rules: rack separators, strip outer edge, non-cocked band slot, bare-strip dashed outline, `stale` strip field grid, tape minute rule, `session-end` rule and the rule under the ATIS header; those ATIS cells are never boxed.
@row_add n=212 row=contract/border-intensity/standard area=color-palette rec=fct-xfrnso35@1
boundary: Border intensities
item: Standard #9C9278
promise: The field-grid rule between cells inside a lit strip, and the 1px box around an ATIS `expired` word, quieter than the boxes of declared states.
@row_add n=213 row=contract/border-intensity/emphasis area=color-palette rec=fct-3ejoqstu@1
boundary: Border intensities
item: Emphasis #E6D8AE
promise: Borders whose presence is the state: readback box outline and strike, `budget-paused` box, nonzero `skipped` boxes, ATIS `TAPE stopped` box, error strips.
@row_add n=214 row=contract/border-intensity/focus area=color-palette rec=fct-atxelyw2@1
boundary: Border intensities
item: Focus #E6D8AE
promise: A 2px solid focus ring, offset 2px, radius 0.
@row_add n=215 row=contract/border-intensity/handoff area=color-palette rec=fct-hx4kd35c@1
boundary: Border intensities
item: Handoff #5B8DB8
promise: Only the 2px left tick of the outbound and inbound transfer markers, in the rack gap on anthracite; the only blue rule, never on holder, never a strip or cell border.
@row_add n=216 row=contract/cli-transfer-arrow/ascii-arrow area=iconography rec=fct-2enypnm7@2
boundary: CLI transfer arrows
item: ->
promise: The CLI prints transfers with ASCII `->` and has no inbound form; links print only as `link` / `unlink` verb output (`overseer -> builder`).
@row_add n=217 row=decision/list-six-fixed-columns area=iconography rec=fct-2enypnm7@2
name: viola list has six fixed columns
rule: `viola list` carries no link column, so its six columns stay fixed.
@row_add n=218 row=contract/cli-separators/two-spaces area=iconography rec=fct-g2p3izdb@2
boundary: CLI separators
item: two spaces
promise: Two spaces separate columns, and a refusal's reason from its detail, as separate fields.
@row_add n=219 row=contract/cli-separators/space-hyphen-space area=iconography rec=fct-g2p3izdb@2
boundary: CLI separators
item: ` - `
promise: Space, hyphen, space replaces a middle dot inside one field or label (`driver - budget-paused`).
@row_add n=220 row=contract/cli-colour-tokens/stale area=surface rec=fct-qgesfouf@1,fct-5r2kfdmj@1
boundary: CLI colour tokens
item: stale
promise: SGR dim (`\x1b[2m`) at every depth over a whole `stale` row, first byte to last; `stale` always in LIVE; on a cocked stale row only `DIALOG` is amber and not dim; NAME on a stale row is dim and bold; every span reopens dim after its reset.
@row_add n=221 row=contract/cli-colour-tokens/callsign area=surface rec=fct-23ap43hv@1,fct-5r2kfdmj@1
boundary: CLI colour tokens
item: callsign
promise: Bold (`\x1b[1m`) at every colour depth, on every NAME column value.
@row_add n=222 row=contract/cli-colour-tokens/handoff-blue area=surface rec=fct-ashdaqa2@1
boundary: CLI colour tokens
item: handoff blue
promise: Handoff blue #5B8DB8 is not used at any colour depth and no index is reserved for it; link output (`overseer -> builder`) is plain.
@row_add n=223 row=decision/cli-colour-amber-and-dim-only area=surface rec=fct-ashdaqa2@1
name: CLI colour limited to amber and dim
rule: CLI output colours nothing but the `DIALOG` word and `stale` rows, so no blue or other colour SGR is ever emitted.
why: The quiz limits CLI colour to amber and dim.
@row_add n=224 row=decision/terminal-background-never-set area=surface rec=fct-6f4vi7a3@1
name: Terminal background never set
rule: The CLI never sets a background colour, at any colour depth.
why: The terminal background belongs to the user.
@row_add n=225 row=contract/cli-stderr/context-lines area=surface rec=fct-pi5kme4c@1
boundary: CLI stderr
item: context lines
promise: Context lines (`waiting: <name>`, the issue line of a send) go to stderr.
@row_add n=226 row=contract/cli-stderr/refusal-line area=surface rec=fct-pi5kme4c@1
boundary: CLI stderr
item: refusal line
promise: A refusal prints `unable  <name>  <reason>  <detail>`, two-space separated; `viola send` prefixes the `[/ ]` mirror and pads `unable` to the readback-word column.
@row_add n=227 row=contract/cli-stderr/fixed-message-refusal area=surface rec=fct-pi5kme4c@1
boundary: CLI stderr
item: fixed message form
promise: The exit-1 refusals of `viola run`, `revive` and `verify`, `viola list`'s exit-21 strict-modes refusal, and the exit-21 strict-modes refusals of `link`, `unlink`, `send`, `wait`, `last`, `answer`, `pause` and `release` use `unable: <text>`.
@row_add n=228 row=contract/cli-stderr/hint-line area=surface rec=fct-pi5kme4c@1
boundary: CLI stderr
item: hint line
promise: Each refusal's `hint:` line follows it directly on stderr.
@row_add n=229 row=contract/cli-stderr/error-line area=surface rec=fct-pi5kme4c@1
boundary: CLI stderr
item: error line
promise: Errors print on stderr as `error: …`.
@row_add n=230 row=contract/peer-reply-screening/code-word area=surface rec=fct-4dwulxx5@1
boundary: peer reply screening
item: code word
promise: A code word is 1 to 64 bytes of lower-case letters, digits and hyphens, a lower-case letter first; both clients, the CLI and the MCP server, screen a peer's `refusal` and `detail` through it.
@row_add n=231 row=contract/peer-reply-screening/refusal-not-code-word area=surface rec=fct-y6uokfat@1
boundary: peer reply screening
item: refusal that is no code word
promise: A peer's `refusal` that is no code word reads `unknown`.
@row_add n=232 row=contract/peer-reply-screening/string-detail-not-code-word area=surface rec=fct-y6uokfat@1
boundary: peer reply screening
item: string detail that is no code word
promise: A peer's string `detail` that is no code word reads as no detail, null under `--json`.
@row_add n=233 row=contract/peer-reply-screening/non-string-detail area=surface rec=fct-y6uokfat@1
boundary: peer reply screening
item: non-string detail
promise: A peer reply with a non-string `detail` is no known reply.
@row_add n=234 row=contract/peer-reply-screening/unlisted-code-word area=surface rec=fct-kr6nq63b@1
boundary: peer reply screening
item: code word outside the refusal list
promise: A code word outside the closed refusal list stays visible, by design, in the document and in `refused: <word>`, and exits 14 as `unknown` does.
@row_add n=235 row=contract/human-tables/name-truncation area=surface rec=fct-kn56sdzi@2
boundary: human tables
item: NAME truncation
promise: Where the width is detectable, human tables truncate only the NAME of unwrapped rows, ending it in `...`.
@row_add n=236 row=contract/human-tables/width-source area=surface rec=fct-kn56sdzi@2
boundary: human tables
item: width source
promise: The table width is stdout's own terminal size; `COLUMNS` is not read, and a piped stdout has no width.
@row_add n=237 row=contract/viola-list-board/status area=surface rec=fct-5r2kfdmj@1
boundary: viola list board
item: STATUS
promise: A wrapped row joined to a `claude agents --json` row, by child pid or agent session id, reads STATUS `idle` or `busy`; any other wrapped row reads `unknown`.
@row_add n=238 row=contract/viola-list-board/unwrapped-separator area=surface rec=fct-5r2kfdmj@1
boundary: viola list board
item: unwrapped separator
promise: When the agents read lists a session no wrapped row joins, the `-- UNWRAPPED - READ-ONLY --` line follows the wrapped rows, or the empty-board line when there are none; with no unwrapped row no separator prints.
@row_add n=239 row=contract/viola-list-board/unwrapped-row area=surface rec=fct-5r2kfdmj@1
boundary: viola list board
item: unwrapped row
promise: One row per unwrapped session, sorted by name: NAME the session's `sessionId`, LIVE `live`, its STATUS, and `n/a` in WHEEL, DIALOG and CLI.
@row_add n=240 row=contract/viola-list-board/failed-agents-read area=surface rec=fct-5r2kfdmj@1
boundary: viola list board
item: failed agents read
promise: A failed, oversize or malformed `claude agents --json` read prints the board as with no `claude`.
@row_add n=241 row=contract/viola-list-board/name-column-width area=surface rec=fct-5r2kfdmj@1
boundary: viola list board
item: NAME column width
promise: On a terminal of known width NAME's column is the largest of the caption's 4, the longest wrapped NAME and the longest unwrapped NAME capped at the width less the 65 columns the rest of an unwrapped row takes.
@row_add n=242 row=contract/viola-list-board/name-cut area=surface rec=fct-5r2kfdmj@1
boundary: viola list board
item: NAME cut
promise: An unwrapped NAME longer than its column is cut to the column less 3 characters and ends in `...`, never splitting an escape; a wrapped NAME is never cut; a piped board or `--json` keeps every name whole.
@row_add n=243 row=contract/viola-list-board/uncoloured-lines area=surface rec=fct-5r2kfdmj@1
boundary: viola list board
item: uncoloured lines
promise: The BAY line, the caption, the empty-board line and the separator carry no SGR.
@row_add n=244 row=contract/viola-list-board/colour-spans area=surface rec=fct-5r2kfdmj@1
boundary: viola list board
item: colour spans
promise: Colour spans are laid over the escaped, padded cells, so the columns are the plain board's, and the board stays one write.
@row_add n=245 row=contract/viola-list-board/row-escaping area=surface rec=fct-5r2kfdmj@1
boundary: viola list board
item: row escaping
promise: The row mode escapes every control character of a row, DEL and C1 included, as `\xHH`; NAME's width is counted in characters.
@row_add n=246 row=decision/wheel-column-width-23 area=surface rec=fct-5r2kfdmj@1
name: WHEEL column padded to 23
rule: The board's WHEEL column is padded to 23 characters, the width of its widest word `unknown - budget-paused` (a replayed row with a logged pause and no logged holder).
why: Ratified.
@row_add n=247 row=contract/viola-list-board/window-figure area=surface rec=fct-5r2kfdmj@1
boundary: viola list board
item: window figure
promise: A window's figure is rounded down to a whole number (`89.6` prints `89 %`).
@row_add n=248 row=contract/viola-list-board/reading-age area=surface rec=fct-5r2kfdmj@1
boundary: viola list board
item: reading age
promise: The reading's age prints in whole minutes rounded down at every age (`read 0m ago`, `read 75m ago`), and `read unknown` for no instant, one that does not parse, or one ahead of now.
@row_add n=249 row=contract/viola-list-board/skip-counts area=surface rec=fct-5r2kfdmj@1
boundary: viola list board
item: skip counts
promise: Nonzero skip counts print as bare numbers in key order (`skipped 2 - 0 - 1`).
@row_add n=250 row=contract/viola-list-board/human-holder-budget-paused area=surface rec=fct-5r2kfdmj@1
boundary: viola list board
item: human holder under a budget pause
promise: A `human` holder under a budget pause prints `human - budget-paused`.
@row_add n=251 row=contract/viola-list-board/empty-board area=surface rec=fct-5r2kfdmj@1
boundary: viola list board
item: empty board
promise: The empty board prints the BAY line, then the caption row, then its one line.
@row_add n=252 row=contract/viola-list-board/strict-modes-refusal area=surface rec=fct-t547mw3y@1
boundary: viola list board
item: strict-modes refusal
promise: A failed strict-modes check of the home, `instances/` or any one instance refuses the whole board with exit 21, nothing from state printed, in the fixed `unable:` / `hint:` form.
@row_add n=253 row=contract/fixed-refusal-pairs/home-state area=surface rec=fct-t547mw3y@1,fct-pbvocw2a@1,fct-xi5ne4sz@1
boundary: fixed refusal pairs
item: home-state pair
promise: `unable: the viola home is not private to you` then `hint: the viola home must be readable only by you; viola does not change its permissions`.
@row_add n=254 row=contract/fixed-refusal-pairs/instance-state area=surface rec=fct-t547mw3y@1,fct-pbvocw2a@1
boundary: fixed refusal pairs
item: instance-state pair
promise: `unable: <name>'s state files can be written by another user` then `hint: viola will not read them; make the viola home and its files owner-only`, naming the instance checked.
@row_add n=255 row=contract/viola-list-board/json-refusal area=surface rec=fct-t547mw3y@1
boundary: viola list board
item: --json refusal
promise: Under `--json` the strict-modes refusal, the board's only one, is one stdout document `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, stderr empty.
@row_add n=256 row=contract/viola-list-board/strict-modes-scan area=surface rec=fct-nmng4ymy@2
boundary: viola list board
item: strict-modes scan
promise: The scan holds the home to the read rule and `instances/` to the write rule, then each instance's set; a home another user may read refuses the whole board with the home-state pair, with or without an instance.
@row_none n=35 rec=fct-ypei347w@2 type=5
@row_add n=257 row=contract/refusal-hints/refusal-line area=surface rec=fct-pbvocw2a@1
boundary: refusal hints
item: refusal line
promise: A refusal prints `unable` plus `reason  detail`, then one `hint:` line keyed by the reason, or by reason and detail where listed.
@row_add n=258 row=contract/refusal-hints/human-typing area=surface rec=fct-pbvocw2a@1
boundary: refusal hints
item: human-typing
promise: `the human has the wheel; send again after the human hands it back`
@row_add n=259 row=contract/refusal-hints/budget-paused area=surface rec=fct-pbvocw2a@1
boundary: refusal hints
item: budget-paused
promise: `budget gate is closed; it lifts at the window reset, or when the human lifts it`
@row_add n=260 row=decision/no-hint-names-viola-release area=surface rec=fct-pbvocw2a@1
name: No hint names viola release
rule: No refusal hint ever names `viola release`; `release` stays a human verb.
why: The human-typing and budget-paused refusals reach drivers, and a driver's `release` is refused (`-32602`, `release-from-driver`, exit 20).
@row_add n=261 row=contract/refusal-hints/unverified-cli area=surface rec=fct-pbvocw2a@1
boundary: refusal hints
item: unverified-cli
promise: `run viola verify for this CLI version`
@row_add n=262 row=contract/refusal-hints/not-delivered-turn-running area=surface rec=fct-pbvocw2a@1
boundary: refusal hints
item: not-delivered · turn-running
promise: `a turn is running; viola wait <name> first`
@row_add n=263 row=contract/refusal-hints/not-delivered-control-character area=surface rec=fct-pbvocw2a@1
boundary: refusal hints
item: not-delivered · control-character
promise: `the text contains a control character (only LF, CR, TAB are allowed)`
@row_add n=264 row=contract/refusal-hints/not-delivered-empty-text area=surface rec=fct-pbvocw2a@1
boundary: refusal hints
item: not-delivered · empty-text
promise: `the text is empty once its trailing newlines are removed; send a text with content`
@row_add n=265 row=contract/refusal-hints/not-delivered-input-not-ready area=surface rec=fct-pbvocw2a@1
boundary: refusal hints
item: not-delivered · input-not-ready
promise: `<name> was not ready for input; send again, and if it repeats a human must look at the session`
@row_add n=266 row=contract/refusal-hints/not-delivered-no-prompt-submitted area=surface rec=fct-pbvocw2a@1
boundary: refusal hints
item: not-delivered · no-prompt-submitted
promise: `<name> did not submit the prompt; check it, then send again`
@row_add n=267 row=contract/refusal-hints/not-delivered-unknown-dialog area=surface rec=fct-pbvocw2a@1
boundary: refusal hints
item: not-delivered · unknown-dialog
promise: `that dialog is not pending; viola list shows the current DIALOG`
@row_add n=268 row=decision/instance-unreachable-hint-per-cause area=surface rec=fct-pbvocw2a@1
name: One instance-unreachable hint per cause
rule: The `instance-unreachable` refusal (exit 21) prints one hint per cause.
why: The exit code is shared by several causes.
@row_add n=269 row=contract/refusal-hints/not-running area=surface rec=fct-pbvocw2a@1,fct-hrikc24v@1
boundary: refusal hints
item: not-running
promise: When no wrapper is running for the name: `<name> is not running; viola list shows the live instances`; `not-running` is the hint key of the log's `instance-dead`.
@row_add n=270 row=contract/refusal-hints/unwrapped-name area=surface rec=fct-pbvocw2a@1,fct-hrikc24v@1
boundary: refusal hints
item: unwrapped-name
promise: When the name is an unwrapped session: `<name> is not a wrapped session; only sessions started with viola run take commands`
@row_add n=271 row=contract/refusal-hints/endpoint-lost area=surface rec=fct-pbvocw2a@1,fct-hrikc24v@1
boundary: refusal hints
item: endpoint-lost
promise: When a wrapper is lost with the call open: `<name>'s wrapper went away during the call; viola list shows whether it is still running`, last on stderr, plain ASCII.
@row_add n=272 row=contract/frame-verb-refusals/strict-modes area=surface rec=fct-pbvocw2a@1
boundary: frame verb refusals
item: strict-modes
promise: A failed strict-modes check (`send` after reading its text; `wait`, `last`, `answer`, `pause`, `release` first; each before any frame) prints the instance-state pair, no `unable  instance-unreachable` line or mirror; `--json`: the `strict-modes-failed` document.
@row_add n=273 row=contract/refusal-hints/server-verify area=surface rec=fct-pbvocw2a@1,fct-hrikc24v@1
boundary: refusal hints
item: server-verify
promise: `the process on <name>'s endpoint is not its recorded wrapper; it was not contacted`, printed byte for byte by the eight frame-writing verbs, plain ASCII with no SGR, holding no pid, start time, path or endpoint.
@row_add n=274 row=contract/refusal-hints/socket-dir area=surface rec=fct-pbvocw2a@1,fct-hrikc24v@1
boundary: refusal hints
item: socket-dir
promise: The socket-directory hint is `run`'s: `viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`.
@row_add n=275 row=contract/viola-run-start-refusals/already-live area=surface rec=fct-pbvocw2a@1
boundary: viola run start refusals
item: already-live
promise: `unable: <name> is already live` then `hint: viola list`; a live holder and a fresh replayed beat log `already-live`.
@row_add n=276 row=contract/viola-run-start-refusals/stale-heartbeat area=surface rec=fct-pbvocw2a@1
boundary: viola run start refusals
item: stale-heartbeat
promise: A stale heartbeat with a live pid prints `unable: <name> is still running but not answering` then `hint: viola list shows it as stale; stop that process before starting <name> again`, and logs `stale-heartbeat`.
@row_add n=277 row=contract/viola-run-start-refusals/squatted-name area=surface rec=fct-pbvocw2a@1
boundary: viola run start refusals
item: squatted-name
promise: `unable: the endpoint for <name> is held by another process` then `hint: another process holds this name's endpoint; stop it or pick another name`.
@row_add n=278 row=contract/viola-run-start-refusals/pinned-hash-mismatch area=surface rec=fct-pbvocw2a@1
boundary: viola run start refusals
item: pinned-hash-mismatch
promise: A SHA-256 mismatch prints `unable: the pinned viola copy failed its integrity check` then `hint: the pinned copy was changed after it was written, so viola will not run it`.
@row_add n=279 row=contract/viola-run-start-refusals/home-tree-check area=surface rec=fct-pbvocw2a@1
boundary: viola run start refusals
item: home-tree check
promise: At start, before any log and with no child, the home, `ledger/`, `bin/`, `plugin/`, the pinned copy and its `plugin/<key>/` are checked, each that exists; a home-level refusal prints the home-state pair, an instance-level one the instance-state pair.
@row_add n=280 row=contract/viola-run-start-refusals/socket-dir area=surface rec=fct-pbvocw2a@1
boundary: viola run start refusals
item: socket directory
promise: Before any snapshot, child or bind: `unable: the socket directory for <name> cannot be trusted` then `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`.
@row_add n=281 row=contract/viola-revive-refusals/order area=surface rec=fct-pbvocw2a@1
boundary: viola revive refusals
item: order
promise: `viola revive` reads its refusal causes in a fixed order that stops at the first refusal; its already-live and stale causes print `run`'s lines and hints unchanged.
@row_add n=282 row=contract/viola-revive-refusals/no-session area=surface rec=fct-pbvocw2a@1
boundary: viola revive refusals
item: no-session
promise: `unable: <name> has no logged session to resume` then `hint: start one: viola run <name> -- claude`.
@row_add n=283 row=contract/viola-revive-refusals/no-such-id area=surface rec=fct-pbvocw2a@1
boundary: viola revive refusals
item: no logged session with that id
promise: An `--id` the log does not hold prints `unable: <name> has no logged session with that id` then `hint: viola revive <name> --list`.
@row_add n=284 row=contract/viola-revive-refusals/session-live area=surface rec=fct-pbvocw2a@1
boundary: viola revive refusals
item: session-live
promise: When one agents read lists the logged id live in another `claude` (skipped under `--fork`; a failed read passes): `unable: <name>'s session is open in another claude` then `hint: close it there first, or start a copy: viola revive <name> --fork`.
@row_add n=285 row=contract/viola-revive-refusals/cwd-missing area=surface rec=fct-pbvocw2a@1
boundary: viola revive refusals
item: cwd-missing
promise: `unable: <name>'s recorded directory is missing` then `hint: resume it by hand from a directory you choose: viola run <name> -- claude --resume <id>, ids from viola revive <name> --list`.
@row_add n=286 row=contract/viola-revive-refusals/home-tree-check area=surface rec=fct-pbvocw2a@1
boundary: viola revive refusals
item: home-tree check
promise: The home-tree check of its start, after a passed preflight and before any log, with no child, prints the home-state or instance-state pair, or under `--json` the `strict-modes-failed` document.
@row_add n=287 row=contract/viola-revive-refusals/socket-dir area=surface rec=fct-pbvocw2a@1
boundary: viola revive refusals
item: socket directory
promise: A refused socket directory at `viola revive`'s start prints `run`'s socket-directory pair.
@row_add n=288 row=decision/hints-human-mode-only area=surface rec=fct-pbvocw2a@1
name: Hints in human mode only
rule: Hints print in human mode only; under `--json` no hint text is printed and the machine view carries the cause as a detail code, never as prose.
@row_add n=289 row=contract/log-detail-codes/server-verify-failed area=surface rec=fct-pbvocw2a@1
boundary: log detail codes
item: server-verify-failed
promise: Written to the log by `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` at their exit-21 server-verification refusal, whose `--json` document reads the same word.
@row_add n=290 row=contract/log-detail-codes/socket-dir-refused area=surface rec=fct-pbvocw2a@1
boundary: log detail codes
item: socket-dir-refused
promise: Written to the log by `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` at their exit-21 socket-directory refusal, and at `run`'s and `revive`'s start.
@row_add n=291 row=contract/viola-revive-refusals/strict-modes-no-log area=surface rec=fct-pbvocw2a@1
boundary: viola revive refusals
item: strict-modes
promise: `viola revive`'s strict-modes refusal writes no log line and carries no logged detail code: the stderr pair, or under `--json` the `strict-modes-failed` document, and exit 1.
@row_add n=292 row=decision/no-hint-for-unknown area=surface rec=fct-pbvocw2a@1
name: No hint for unknown
rule: The `unknown` refusal (exit 14) prints no hint line.
why: Its detail is a code word or nothing, and a hint would have to guess.
@row_add n=293 row=contract/frame-verb-refusals/socket-dir area=surface rec=fct-fjfzfyz5@1
boundary: frame verb refusals
item: socket directory
promise: A refused socket directory exits 21 before any frame: `unable  <name>  instance-unreachable` (`send` in its mirror's form), then the `socket-dir` hint, no strict-modes pair; `--json`: `detail` `socket-dir-refused`, stderr empty.
@row_add n=294 row=contract/viola-release-budget/lifted-line area=surface rec=fct-tkxomkux@1
boundary: viola release --budget
item: lifted line
promise: When a pause stood, `viola release <name> --budget` prints `<name>  budget gate lifted until <time>  wheel human`, one unstyled stdout line with the two-space gutter of the other verb lines.
@row_add n=295 row=contract/viola-release-budget/override-time area=surface rec=fct-tkxomkux@1
boundary: viola release --budget
item: override time
promise: The time is the reply's `override_until`: `HH:MM:SSZ` on the current UTC date, `YYYY-MM-DD HH:MM:SSZ` on any other date; a value that is no RFC 3339 instant prints escaped.
@row_add n=296 row=contract/viola-release-budget/open-line area=surface rec=fct-tkxomkux@1
boundary: viola release --budget
item: open line
promise: When no pause stood the line is `<name>  budget gate open  wheel human`, one unstyled stdout line.
@row_add n=297 row=contract/viola-link-output/result-line area=surface rec=fct-327x7jz3@1
boundary: viola link and unlink output
item: result line
promise: `viola link` and `viola unlink` print one result line on stdout in one write, plain ASCII with no SGR whatever the terminal, and exit 0.
@row_add n=298 row=contract/viola-link-output/printed-names area=surface rec=fct-327x7jz3@1
boundary: viola link and unlink output
item: printed names
promise: The names printed are the verb's own parsed arguments, never names read back from the reply or the log.
@row_add n=299 row=contract/viola-link-output/since area=surface rec=fct-clb6azzi@1
boundary: viola link and unlink output
item: since
promise: The `link` line's `since` is the reply's `since` as UTC clock time in whole seconds with `Z` (`HH:MM:SSZ`); a `since` that is no RFC 3339 instant prints through the message-mode escaper.
@row_add n=300 row=contract/viola-link-output/repeat-link area=surface rec=fct-dfvj3huj@1
boundary: viola link and unlink output
item: repeat link
promise: A second `viola link` of a held pair prints the `already linked` form, the wrapper keeping the first `since`, and exits 0.
@row_add n=301 row=contract/viola-link-output/unlink-not-held area=surface rec=fct-dfvj3huj@1
boundary: viola link and unlink output
item: unlink of a pair not held
promise: `viola unlink` of a pair that is not held prints the same `unlink  <driver> -> <driven>` line and exits 0.
@row_add n=302 row=decision/one-progress-pattern area=surface rec=fct-sx3u7aez@2
name: One progress pattern
rule: A step counter with static appended stdout lines is the project's one progress pattern.
@row_add n=303 row=contract/viola-verify-output/summary-line area=surface rec=fct-sx3u7aez@2,fct-sedcqxww@2
boundary: viola verify output
item: summary line
promise: The last stdout line is a summary whose counts add up to `MM`: `stamped <version>  <n> pass  <m> fail`; a failing row prints no stderr word.
@row_add n=304 row=contract/viola-verify-output/fail-word area=surface rec=fct-sx3u7aez@2
boundary: viola verify output
item: fail word
promise: The word `fail` is uncoloured.
@row_add n=305 row=contract/viola-verify-output/help area=surface rec=fct-sx3u7aez@2
boundary: viola verify output
item: --help
promise: `viola verify --help` carries one static ASCII paragraph naming no path: run it from a folder you trust in Claude Code, and an unapproved external CLAUDE.md import blocks the probe.
@row_add n=306 row=contract/viola-verify-refusals/interrupted area=surface rec=fct-sx3u7aez@2,fct-sedcqxww@2
boundary: viola verify refusals
item: interrupted run
promise: A run stopped by Ctrl-C or TERM before its rows are checked prints no step line and no summary; stderr carries exactly `unable: verify was stopped before it finished` then `hint: run viola verify again to stamp this CLI`; exit 1.
@row_add n=307 row=contract/viola-verify-refusals/home-tree-check area=surface rec=fct-xi5ne4sz@1,fct-sedcqxww@2
boundary: viola verify refusals
item: home-tree check
promise: Before its log, `verify` checks the home, `ledger/`, `bin/` and `plugin/`, each that exists; a refusal prints no step line and no summary, only the home-state pair, and exits 1.
@row_add n=308 row=contract/viola-verify-refusals/stamps-update area=surface rec=fct-xi5ne4sz@1,fct-sedcqxww@2
boundary: viola verify refusals
item: stamps.json update
promise: A `stamps.json` update refused by its strict-modes check prints the home-state pair and exits 1.
@row_add n=309 row=contract/strict-modes/home-under-regular-file area=surface rec=fct-bctn63fi@2
boundary: strict-modes refusals
item: home under a regular file
promise: A home under a regular file cannot be read and is a strict-modes refusal, not the catch-site line: `verify` prints the home-state pair (exit 1), the frame verbs the instance-state pair (exit 21).
@row_add n=310 row=contract/viola-revive-output/after-child-start area=surface rec=fct-gray6vne@1
boundary: viola revive output
item: after the child starts
promise: `viola revive`, as `run`, prints nothing once the child starts.
@row_add n=311 row=contract/viola-revive-output/list-line area=surface rec=fct-gray6vne@1
boundary: viola revive output
item: --list line
promise: `viola revive <name> --list` prints one static stdout line per logged session, in log order, `<time>  <cause>  <session id>`, two spaces between fields: no header, no colour, plain ASCII.
@row_add n=312 row=contract/viola-revive-output/list-json area=surface rec=fct-gray6vne@1
boundary: viola revive output
item: --list --json
promise: `viola revive <name> --list --json` prints `{"v":1,"ok":{"sessions":[…]}}`.
@row_add n=313 row=contract/viola-revive-json-codes/strict-modes-failed area=surface rec=fct-mep5dn6i@2
boundary: viola revive --json exit-1 codes
item: strict-modes-failed
promise: `strict-modes-failed` is one of the closed exit-1 codes of `viola revive --json`.
@row_add n=314 row=contract/viola-revive-json-codes/already-live area=surface rec=fct-mep5dn6i@2
boundary: viola revive --json exit-1 codes
item: already-live
promise: `already-live` is one of the closed exit-1 codes of `viola revive --json`.
@row_add n=315 row=contract/viola-revive-json-codes/no-session area=surface rec=fct-mep5dn6i@2
boundary: viola revive --json exit-1 codes
item: no-session
promise: `no-session` is one of the closed exit-1 codes of `viola revive --json`.
@row_add n=316 row=contract/viola-revive-json-codes/session-live area=surface rec=fct-mep5dn6i@2
boundary: viola revive --json exit-1 codes
item: session-live
promise: `session-live` is one of the closed exit-1 codes of `viola revive --json`.
@row_add n=317 row=contract/viola-revive-json-codes/cwd-missing area=surface rec=fct-mep5dn6i@2
boundary: viola revive --json exit-1 codes
item: cwd-missing
promise: `cwd-missing` is one of the closed exit-1 codes of `viola revive --json`.
@row_add n=318 row=contract/viola-revive-json-codes/stale-heartbeat area=surface rec=fct-mep5dn6i@2
boundary: viola revive --json exit-1 codes
item: stale-heartbeat
promise: `stale-heartbeat` is one of the closed exit-1 codes of `viola revive --json`.
@row_add n=319 row=contract/viola-revive-json-codes/pinned-hash-mismatch area=surface rec=fct-mep5dn6i@2
boundary: viola revive --json exit-1 codes
item: pinned-hash-mismatch
promise: `pinned-hash-mismatch` is one of the closed exit-1 codes of `viola revive --json`.
@row_add n=320 row=contract/viola-revive-json-codes/squatted-name area=surface rec=fct-mep5dn6i@2
boundary: viola revive --json exit-1 codes
item: squatted-name
promise: `squatted-name` is one of the closed exit-1 codes of `viola revive --json`.
@row_add n=321 row=contract/viola-revive-json-codes/socket-dir-refused area=surface rec=fct-mep5dn6i@2,fct-gul3bnzl@1
boundary: viola revive --json exit-1 codes
item: socket-dir-refused
promise: A failed create of the socket directory and a refused check both print one stdout document with the code `socket-dir-refused`, no hint, stderr empty.
@row_add n=322 row=contract/viola-revive-json-codes/internal-error area=surface rec=fct-mep5dn6i@2,fct-4md437yp@1
boundary: viola revive --json exit-1 codes
item: internal-error
promise: A program not found, a cause with no human pair, prints `internal-error`; an anyhow error of the start prints the same document, and the catch site logs its chain and its line.
@row_add n=323 row=decision/no-json-after-child-held-terminal area=surface rec=fct-xtefeabf@1,fct-sedcqxww@2
name: No document once the child held the terminal
rule: An error while pumping the child under `viola revive --json` prints no document.
why: The child has held the terminal and viola writes nothing past it.
@row_add n=324 row=contract/wrapper-socket/refused-peer area=surface rec=fct-f2zfjygd@1
boundary: wrapper socket
item: refused peer
promise: A peer the wrapper refuses is dropped before any frame: no line written, nothing on the terminal the child holds, no wheel moved; the client reads end-of-file.
@row_add n=325 row=contract/viola-verify-refusals/cli-not-found area=surface rec=fct-sedcqxww@2
boundary: viola verify refusals
item: CLI not found
promise: `unable: the claude CLI was not found`, followed by its own hint, exit 1.
@row_add n=326 row=contract/viola-verify-refusals/version-unreadable area=surface rec=fct-sedcqxww@2
boundary: viola verify refusals
item: version unreadable
promise: `unable: the CLI version could not be read`, followed by its own hint, exit 1.
@row_add n=327 row=contract/viola-verify-refusals/unclean-fixture area=surface rec=fct-sedcqxww@2
boundary: viola verify refusals
item: unclean fixture
promise: `unable: a recorded fixture is not clean: <file> <code>` (a screen: `<file> row <n>[ seam] <code>`), codes `home-path` · `absolute-path` · `username` · `email`, never the content, followed by its hint, exit 1.
@row_add n=328 row=contract/cli-exit-codes/10 area=surface rec=fct-f5dfwzmp@1
boundary: CLI exit codes
item: 10
promise: `unable  human-typing` (+ `manual-pause`); under `--json` `{"v":1,"refusal":"human-typing","detail":…}`.
@row_add n=329 row=contract/cli-exit-codes/11 area=surface rec=fct-spwfscwz@1
boundary: CLI exit codes
item: 11
promise: `unable  budget-paused  five-hour` or `seven-day`; under `--json` the refusal object.
@row_add n=330 row=contract/cli-exit-codes/12 area=surface rec=fct-vocxcyzi@1
boundary: CLI exit codes
item: 12
promise: `unable  unverified-cli`; under `--json` the refusal object.
@row_add n=331 row=contract/cli-exit-codes/13 area=surface rec=fct-k7c3yar5@1
boundary: CLI exit codes
item: 13
promise: `unable  not-delivered` with `input-not-ready`, `no-prompt-submitted`, `turn-running`, `unknown-dialog`, `control-character` or `empty-text`; under `--json` the refusal object.
@row_add n=332 row=contract/cli-exit-codes/14 area=surface rec=fct-f6euya2e@1
boundary: CLI exit codes
item: 14
promise: `unable  unknown  <detail>`, the detail a code word or nothing; under `--json` the refusal object.
@row_add n=333 row=contract/exit-21-json-cause/strict-modes-failed area=surface rec=fct-hgef4h2n@1
boundary: exit 21 --json causes
item: strict-modes-failed
promise: The `list`, `link`, `unlink` and `plugin install` refusals and the six frame verbs' strict-modes refusals read `strict-modes-failed`.
@row_add n=334 row=contract/exit-21-json-cause/instance-dead area=surface rec=fct-hgef4h2n@1,fct-zp4y2mcv@1,fct-aej2ym2d@1,fct-ihyxsaec@1
boundary: exit 21 --json causes
item: instance-dead
promise: With no live endpoint, an instance directory that is a directory reads `instance-dead`, as do a failed connect and a frame verb against a stale instance; stale heartbeat is exit 1's cause alone.
@row_add n=335 row=contract/exit-21-json-cause/unwrapped-name area=surface rec=fct-hgef4h2n@1,fct-zp4y2mcv@1
boundary: exit 21 --json causes
item: unwrapped-name
promise: With no live endpoint, a frame verb whose instance directory, read after the strict-modes check, is absent, a regular file or a symlink reads `unwrapped-name`.
@row_add n=336 row=contract/exit-21-json-cause/endpoint-lost area=surface rec=fct-hgef4h2n@1,fct-aej2ym2d@1
boundary: exit 21 --json causes
item: endpoint-lost
promise: A wrapper lost with the call open reads `endpoint-lost`.
@row_add n=337 row=contract/exit-21-json-cause/server-verify-failed area=surface rec=fct-hgef4h2n@1,fct-pxdkcmfq@2
boundary: exit 21 --json causes
item: server-verify-failed
promise: A process on the endpoint that is not the recorded wrapper (pid or start time differs, the check cannot be made, or the peer's euid is not this user's or cannot be read) gets no frame; the eight frame-writing verbs read `server-verify-failed`.
@row_add n=338 row=contract/exit-21-json-cause/socket-dir-refused area=surface rec=fct-hgef4h2n@1
boundary: exit 21 --json causes
item: socket-dir-refused
promise: `socket-dir-refused` is one of the six closed causes of exit 21's `--json` document.
@row_add n=339 row=contract/viola-link-json/ok area=surface rec=fct-7mkzemsx@1
boundary: viola link --json
item: ok
promise: Exit 0 prints `{"v":1,"ok":{"since":<ts>,"already_linked":<bool>}}`, `<ts>` the time the wrapper recorded the link.
@row_add n=340 row=contract/viola-unlink-json/ok area=surface rec=fct-7mkzemsx@1
boundary: viola unlink --json
item: ok
promise: Exit 0 prints `{"v":1,"ok":{}}`.
@row_add n=341 row=contract/viola-link-output/refused-pair area=surface rec=fct-3vwzogmp@1
boundary: viola link and unlink output
item: refused pair
promise: A pair the wrapper refuses (`-32602`; a `driver` equal to `driven`, which the CLI never refuses itself) exits 20: `error: wrapper fault  <code>`, or under `--json` the `wrapper-fault` document.
@row_add n=342 row=contract/wrapper-fault-detail/message area=surface rec=fct-3b32n36m@1
boundary: wrapper fault --json detail
item: message
promise: Exit 20's `--json` `detail` is closed for the five known codes, and `message` is each code's fixed text.
@row_add n=343 row=contract/wrapper-fault-detail/unsupported-version area=surface rec=fct-3b32n36m@1
boundary: wrapper fault --json detail
item: -32602 unsupported version
promise: `-32602` with an integer `data.supported` and a version-text `data.wrapper` reads the unsupported-version body with those two values.
@row_add n=344 row=contract/wrapper-fault-detail/release-from-driver area=surface rec=fct-3b32n36m@1
boundary: wrapper fault --json detail
item: -32602 release-from-driver
promise: `-32602` with `data.reason` exactly `release-from-driver` reads that body.
@row_add n=345 row=contract/wrapper-fault-detail/other-invalid-params area=surface rec=fct-3b32n36m@1
boundary: wrapper fault --json detail
item: other -32602
promise: Any other `-32602` reads `invalid params` with null `data`.
@row_add n=346 row=contract/wrapper-fault-detail/other-known-codes area=surface rec=fct-3b32n36m@1
boundary: wrapper fault --json detail
item: other known codes
promise: Every known code other than `-32602` has null `data`.
@row_add n=347 row=contract/wrapper-fault-detail/unknown-code area=surface rec=fct-utjcf3wu@1
boundary: wrapper fault --json detail
item: code outside the five
promise: A code outside the five known prints `error: wrapper fault  <code>`, and under `--json` `{"v":1,"error":"wrapper-fault","detail":{"code":<code>,"message":"wrapper fault","data":null}}`; MCP carries the same fixed `message`.
@row_add n=348 row=decision/json-failure-documents-clean area=surface rec=fct-kqtzijw6@1
name: Clean --json failure documents
rule: Each CLI `--json` failure prints one document, exits its literal code, leaves stderr empty, and holds no ESC, no path and no upstream text.
guard: tests/cli_error_bodies.rs
@row_add n=349 row=contract/cli-catch-site/json-internal-error area=surface rec=fct-arozinj6@1
boundary: CLI catch site
item: --json internal error
promise: Under `--json` the catch site prints the `internal-error` document on stdout in place of `error: internal error`, stderr then empty; without `--json` it prints that line, byte for byte.
@row_add n=350 row=contract/cli-catch-site/json-flag-scope area=surface rec=fct-jh6curc2@1
boundary: CLI catch site
item: --json flag scope
promise: The catch site counts `--json` only among the arguments before a `--`, read from argv.
@row_add n=351 row=contract/cli-catch-site/silent-verbs area=surface rec=fct-ue5y52fu@1
boundary: CLI catch site
item: silent verbs
promise: `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site, under `--json` too.
@row_add n=352 row=contract/viola-help/verb-groups area=surface rec=fct-pjtiz7xa@1
boundary: viola --help
item: verb groups
promise: `viola --help` lists the served verbs only, grouped `board: list` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: run, revive, verify, plugin install`, then `Options:`.
@row_add n=353 row=decision/cli-ascii-only area=typography rec=fct-qwc3u462@1,fct-ukpiycjz@2,fct-g2p3izdb@2
name: ASCII-only CLI output
rule: CLI output is ASCII only, separators included, at high density.
@row_add n=354 row=contract/cli-colour-tokens/attention area=surface-cli rec=fct-dufqiv2l@1,fct-erfdav33@1,fct-muqp52c7@1,fct-5r2kfdmj@1
boundary: CLI colour tokens
item: attention
promise: Arrival amber #D97706 on the `DIALOG` word only, always followed by its kind word, which stays plain: `\x1b[38;2;217;119;6m` in truecolor, `\x1b[38;5;172m` at 256 colours, `\x1b[33m` at 16.
@row_add n=355 row=contract/cli-colour-tokens/reset area=surface-cli rec=fct-dufqiv2l@1,fct-qe6n467m@1
boundary: CLI colour tokens
item: reset
promise: `\x1b[0m` at every colour depth, after every styled span.
@row_add n=356 row=contract/cli-colour-depth/16-colour area=surface-cli rec=fct-2osj5znw@1,fct-ukqwfmzr@1
boundary: CLI colour depth
item: 16-colour
promise: Where no earlier depth matches, the depth is the 16-colour fallback column; the depth is chosen once per process, first match wins.
@row_add n=357 row=absence/cli-tape area=surface-cli rec=fct-rhkxdzp7@1,fct-dbxcldrz@1,fct-kn56sdzi@2
capability: A tape in the CLI
@row_add n=358 row=decision/no-hint-for-faults area=surface-cli rec=fct-5xeughsz@1,fct-pbvocw2a@1
name: No hint for faults and unknown
rule: `error: wrapper fault` (exit 20), `error: internal error` (exit 1) and `unknown` (exit 14) print no hint line; a fault's human line holds the code only and further detail goes to `diagnostics/`.
why: Faults are not refusals; the detail of `unknown` is a code word or nothing, so a hint would have to guess.
@row_add n=359 row=contract/viola-verify-output/step-line area=surface-cli rec=fct-nz6d6pbn@1,fct-sx3u7aez@2
boundary: viola verify output
item: step line
promise: One static appended uncoloured stdout line per relied ledger row, ending `pass` or `fail`, counted against `MM`, the relied count.
@row_add n=360 row=decision/start-refusals-show-no-path-or-pid area=surface-cli rec=fct-w4pl2fni@1,fct-pbvocw2a@1
name: Start refusals show no path or pid
rule: The exit-1 refusals of `viola run` and `viola revive` print one fixed-message line and one hint per cause, and none shows a path, a pid, the recorded directory or a logged id.
@row_add n=361 row=contract/viola-revive-output/json-refusal area=surface-cli rec=fct-2t54hjhs@2,fct-gray6vne@1,fct-mep5dn6i@2,fct-sedcqxww@2
boundary: viola revive output
item: --json refusal
promise: Under `--json` every refusal of the start `revive` shares with `run` prints one exit-1 stdout document `{"v":1,"error":"<code>","detail":null}` and no hint, stderr empty; `run` itself prints none of these documents; a `pump_child` error prints none.
@row_add n=362 row=contract/cli-exit-codes/0 area=surface-cli rec=fct-tj4jq2zv@1,fct-nr7mro52@1
boundary: CLI exit codes
item: 0
promise: No stderr first word (TTY context lines only); the result line goes to stdout; under `--json` the document `{"v":1,"ok":{…}}`.
@row_add n=363 row=contract/cli-exit-codes/1 area=surface-cli rec=fct-tj4jq2zv@1,fct-sedcqxww@2
boundary: CLI exit codes
item: 1
promise: `error: internal error` (fixed; no chain, paths or hint), or a fixed `unable:` refusal of `run`, `revive` or `verify` with its hint; `--json` only for `revive`'s shared-start refusals and the catch site's `{"v":1,"error":"internal-error","detail":null}`, stderr empty.
@row_add n=364 row=contract/cli-exit-codes/2 area=surface-cli rec=fct-tj4jq2zv@1,fct-4zlzvxze@1
boundary: CLI exit codes
item: 2
promise: Clap usage text (never from `viola hook`), or the fixed `UNUSABLE_RESPONSE` line (oversize `viola answer`) or `UNUSABLE_TEXT` line (unusable `viola send`) on stderr, whatever `--json` says; no document.
@row_add n=365 row=contract/cli-exit-codes/20 area=surface-cli rec=fct-tj4jq2zv@1,fct-adve5pgh@1
boundary: CLI exit codes
item: 20
promise: `error: wrapper fault  <code>`; under `--json` `{"v":1,"error":"wrapper-fault","detail":{…}}`.
@row_add n=366 row=contract/cli-exit-codes/21 area=surface-cli rec=fct-tj4jq2zv@1,fct-hgef4h2n@1
boundary: CLI exit codes
item: 21
promise: `unable  instance-unreachable` then the cause's hint; strict-modes refusals print the fixed home-state or instance-state `unable:` / `hint:` pair instead, with no frame; under `--json` `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, stderr empty.
@row_add n=367 row=absence/rendered-page area=border-radius rec=fct-k5ssxpy7@2,fct-hdk4heck@2,fct-rcoxrota@2,fct-73hf7cl5@2
capability: A rendered page: viola renders no page, so no radius, motion or spacing tokens; the cli's type is the terminal's
