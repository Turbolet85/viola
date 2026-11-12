@row_add n=1 row=contract/strict-modes-refusal/exit-21 area=surface-cli rec=1
boundary: Instance strict-modes refusal
item: exit 21
promise: A refused strict-modes check of the driven instance by `viola link` / `unlink`, or of the instance of `send`, `wait`, `last`, `answer`, `pause` and `release`, is made before any frame, exits 21, and writes no frame and no line.
@row_add n=2 row=contract/strict-modes-refusal/human-output area=surface-cli rec=1
boundary: Instance strict-modes refusal
item: unable: / hint: pair
promise: The refusal prints the instance-state `unable:` / `hint:` pair naming that instance, as `viola list`'s does.
@row_add n=3 row=contract/strict-modes-refusal/json-document area=surface-cli rec=1
boundary: Instance strict-modes refusal
item: --json document
promise: Under `--json` the refusal prints the `instance-unreachable` document with `detail` `strict-modes-failed`, and stderr stays empty.
@row_add n=4 row=decision/flat-cli-verbs area=surface-cli rec=2
name: Flat CLI verbs
rule: CLI verbs are flat, each one lower-case word, with no nested subcommand groups.
except: `plugin install`
@row_add n=5 row=name/command/run area=surface-cli rec=2
class: command
name: run
@row_add n=6 row=name/command/revive area=surface-cli rec=2
class: command
name: revive
@row_add n=7 row=name/command/send area=surface-cli rec=2
class: command
name: send
@row_add n=8 row=name/command/wait area=surface-cli rec=2
class: command
name: wait
@row_add n=9 row=name/command/last area=surface-cli rec=2
class: command
name: last
@row_add n=10 row=name/command/list area=surface-cli rec=2
class: command
name: list
@row_add n=11 row=name/command/answer area=surface-cli rec=2
class: command
name: answer
@row_add n=12 row=name/command/verify area=surface-cli rec=2
class: command
name: verify
@row_add n=13 row=name/command/pause area=surface-cli rec=2
class: command
name: pause
@row_add n=14 row=name/command/release area=surface-cli rec=2
class: command
name: release
@row_add n=15 row=name/command/link area=surface-cli rec=2
class: command
name: link
@row_add n=16 row=name/command/unlink area=surface-cli rec=2
class: command
name: unlink
@row_add n=17 row=name/command/plugin-install area=surface-cli rec=2
class: command
name: plugin install
@row_add n=18 row=decision/list-is-the-board area=surface-cli rec=3
name: viola list is the board
rule: `viola list` is the board, and every other verb acts on one strip.
@row_add n=19 row=decision/no-interactive-prompt-or-pager area=surface-cli rec=3
name: No interactive prompt or pager
rule: No verb has an interactive prompt or a pager.
@row_add n=20 row=decision/fleet-console-not-verb-output area=surface-cli rec=3,21
name: Fleet console is no verb's output
rule: The terminal fleet console is no verb's output, so verb output rules do not govern its redraw.
@row_add n=21 row=contract/help-output/verb-groups area=surface-cli rec=4
boundary: viola --help output
item: verb groups
promise: `viola --help` lists the served verbs as `board: list` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: run, revive, verify, plugin install`, then `Options:`.
@row_add n=22 row=decision/prompt-text-from-stdin-or-file area=surface-cli rec=5
name: Prompt text from stdin or file only
rule: Prompt text comes only from stdin or `--file`.
@row_add n=23 row=contract/hook-verb/stderr area=surface-cli rec=6
boundary: The hook verb
item: stderr
promise: `hook` writes nothing to stderr.
@row_add n=24 row=contract/hook-verb/exit-code area=surface-cli rec=6
boundary: The hook verb
item: exit code
promise: `hook` always exits 0.
@row_add n=25 row=contract/mcp-verb/stdout area=surface-cli rec=6
boundary: The mcp verb
item: stdout
promise: `mcp` writes only MCP frames on stdout.
@row_add n=26 row=decision/mcp-hidden-verb area=surface-cli rec=6
name: mcp is a hidden verb
rule: `mcp` and `hook` have no human design surface, and `mcp` appears in no `viola --help` row.
why: `mcp` is run by the plugin, never by a person.
@row_add n=27 row=name/command/hook area=surface-cli rec=6
class: command
name: hook
@row_add n=28 row=name/command/mcp area=surface-cli rec=6
class: command
name: mcp
@row_add n=29 row=decision/no-stack-traces area=surface-cli rec=7,26
name: No stack traces
rule: Stack traces never print; errors are fixed messages and full detail goes only to the instance's diagnostics.
@row_none n=1 rec=8 type=S
@row_add n=30 row=decision/colour-carries-meaning area=anti-patterns-never-do-these rec=9
name: Colour carries meaning, never decoration
rule: Every colour communicates meaning: amber means `dialog_pending` and nothing else, blue means handoff and nothing else, and no third hue is used.
why: Any third hue has no domain meaning.
@row_add n=31 row=decision/no-convergent-safe-choices area=anti-patterns-never-do-these rec=10
name: No convergence on safe design choices
rule: The design never converges on common safe choices across generations; the readback box and the cocked strip stand in their place.
why: Agent dashboards converge on dots, toasts and graphs.
@row_add n=32 row=decision/no-traffic-light-status-dots area=anti-patterns-never-do-these rec=11
name: No traffic-light status dots
rule: Status is never a green/yellow/red dot; `stale` is a coasting lamp-off track, not red, and the only colour state is the cocked strip's amber beside its `DIALOG` word.
why: A strip prints its status word in a fixed column.
@row_add n=33 row=decision/no-optimistic-send-confirmation area=anti-patterns-never-do-these rec=12
name: No optimistic send confirmation
rule: The readback box stays `open` until the matching `prompt-submitted` or the command's post-condition, then fills or is struck with `unable` plus the typed reason; no spinner, green mark or toast.
why: A clearance does not count until it is read back.
@row_add n=34 row=decision/no-auto-sorting-strips area=anti-patterns-never-do-these rec=13
name: No auto-sorting of strips
rule: Strips needing input or with recent sessions are never re-ranked to the top; a strip is cocked in place.
why: Re-ranking breaks the founder's memory of where each session lives, and the `DIALOG` word already says "needs you".
@row_add n=35 row=decision/budget-as-ruled-figures area=anti-patterns-never-do-these rec=14
name: Budget as figures in ruled fields
rule: Budget prints as figures in ruled fields with the ATIS reading age, never as rings or coloured bars; `budget-paused` is a word, not a colour.
@row_add n=36 row=decision/wheel-as-words area=anti-patterns-never-do-these rec=15
name: Wheel shown as words
rule: The WHEEL field prints `human` / `driver`, never icons, emoji or a toggle; taking control is spoken with `viola pause`.
why: Taking control is never implied by a switch.
@row_add n=37 row=decision/no-shimmer-or-glow-theme area=anti-patterns-never-do-these rec=16
name: No skeleton shimmer or glow theme
rule: No skeleton shimmer and no terminal-green or neon-glow theme; missing values print `unknown` in their box at once.
why: Nothing glows in a tower cab unless the lamp is on it.
@row_none n=2 rec=17 type=S
@row_add n=38 row=decision/no-check-glyph-or-emoji-prefixes area=anti-patterns-never-do-these rec=18
name: No check-mark or emoji prefixes
rule: Output never uses green check / red cross prefixes or emoji; the readback mirror `[RB]` / `[  ]` / `[/ ]` and the words `read back` / `unable` are the prefixes.
@row_add n=39 row=decision/colour-only-as-second-cue area=anti-patterns-never-do-these rec=19,41
name: Colour only as a second cue
rule: Only `DIALOG` (amber) and `stale` rows (dim) are coloured, and never without their word beside them.
why: The quiz confirms colour as a second cue only.
@row_add n=40 row=decision/plain-output-modes area=anti-patterns-never-do-these rec=20,42
name: Plain output modes
rule: No colour, glyphs, cursor control or spinner are emitted under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run`, or a passed `viola revive`.
why: LLM drivers parse this output, and `viola run`'s terminal belongs to the child's screen.
@row_add n=41 row=decision/no-spinners-in-verb-output area=anti-patterns-never-do-these rec=21
name: No spinners in verb output
rule: A verb's output has no spinner, progress bar or live-redrawing dashboard; `wait` prints one static `waiting:` line and `verify` prints step-counter lines.
why: A spinner is progress presumed before it is confirmed.
@row_add n=42 row=decision/stdout-data-stderr-messages area=anti-patterns-never-do-these rec=22
name: Data on stdout, messages on stderr
rule: Results go to stdout; `waiting:`, refusals with their `hint:` lines, and errors go to stderr.
@row_add n=43 row=decision/no-internals-in-errors area=anti-patterns-never-do-these rec=23
name: No internals in errors or hints
rule: Errors and hints never print upstream text, paths, pids or anyhow chains with serde sources.
why: The security plan's NEVER-log floor.
@row_add n=44 row=decision/escape-controls-in-human-output area=anti-patterns-never-do-these rec=23
name: Control characters escaped in human output
rule: Human output of `last` and `list` escapes C0/C1 control characters.
@row_add n=45 row=decision/confirmations-carry-context area=anti-patterns-never-do-these rec=24
name: Confirmations carry context
rule: Output never says "done" or "success" bare; it says what was read back, by whom, and at which cursor.
@row_add n=46 row=decision/no-hardcoded-widths-or-wrapping area=anti-patterns-never-do-these rec=25
name: No hardcoded widths or wrapping
rule: Widths are never hardcoded; output is truncated only where the terminal width is known, and never wrapped.
@row_none n=3 rec=27 type=S
@row_add n=47 row=decision/swap-test area=self-validation-protocol rec=29,28
name: Swap test
rule: Before UI output is presented, a design that would not feel meaningfully different with a default typeface, default colours and a standard layout is redone with intent.
@row_add n=48 row=decision/squint-test area=self-validation-protocol rec=30,28
name: Squint test
rule: Before UI output is presented, hierarchy must still be perceivable with blurred eyes and nothing may jump out harshly.
except: `open` and `unconfirmable` readback boxes look identical under the squint, and only the word cell tells them apart, as intended.
@row_add n=49 row=decision/readback-box-signature-check area=self-validation-protocol rec=31,28
name: Readback box signature check
rule: Before UI output is presented, the readback box must be present and drawn the same in its one place, the `[RB]` / `[  ]` / `[/ ]` column of CLI `viola send`.
@row_add n=50 row=decision/domain-not-statistical-design area=self-validation-protocol rec=32,28
name: Design from the product's domain
rule: Before UI output is presented, an interface another AI would produce from a similar prompt fails; the interface must emerge from this product's domain exploration.
@row_none n=4 rec=33 type=S
@row_none n=5 rec=34 type=S
@row_add n=51 row=absence/rendered-page area=border-radius rec=35,54,55
capability: A rendered page: no graphical surface, so no radius, motion or spacing tokens
@row_none n=6 rec=36 type=0
what: a legend item of the generic expression-motion scale
@row_none n=7 rec=37 type=0
what: a legend item of the generic expression-motion scale
@row_none n=8 rec=38 type=0
what: a legend item of the generic expression-motion scale
@row_none n=9 rec=39 type=0
what: a legend item of the generic expression-motion scale
@row_none n=10 rec=40 type=0
what: a legend item of the generic expression-motion scale
@row_add n=52 row=decision/cli-tty-static-expression area=brand-identity rec=41
name: Static expression for the human CLI
rule: Human TTY CLI output is at expression 0.2: static fixed-column rows and one static context header, nothing redrawn in place and no spinners.
@row_add n=53 row=decision/run-prints-nothing-while-wrapped area=brand-identity rec=42
name: viola run silent while wrapped
rule: `viola run` prints nothing at all while the wrapped `claude` TUI runs, and neither does a `viola revive` that passed its preflight.
@row_add n=54 row=contract/colour-palette/primary area=color-palette rec=43
boundary: Colour palette
item: Primary #E6D8AE
promise: Primary ink, focus rings, information-carrying borders (readback box outline and strike, `budget-paused`, nonzero `skipped` and `TAPE stopped` boxes, error strips) and the read-back fill.
@row_add n=55 row=contract/colour-palette/secondary area=color-palette rec=44
boundary: Colour palette
item: Secondary #9C9278
promise: Secondary ink on anthracite (timestamps, reading ages, column captions, `n/a`), the field-grid rule inside strips, the 1px box around an ATIS `expired` word, and all ink of a `stale` strip.
@row_add n=56 row=contract/colour-palette/accent area=color-palette rec=45
boundary: Colour palette
item: Accent #D97706
promise: The only attention colour: the holder band and `DIALOG` word of a cocked strip and the CLI `DIALOG` word in `viola list`; never used for errors, warnings, budget or hover.
@row_add n=57 row=contract/colour-palette/handoff area=color-palette rec=46
boundary: Colour palette
item: Handoff #5B8DB8
promise: Transfer-marker arrows and names, the marker's 2px left tick, and the names in `link` / `unlink` tape lines only; never a fill, never text on holder, never a status.
@row_add n=58 row=contract/border-intensity/subtle area=color-palette rec=47
boundary: Border intensities
item: Subtle #4A5057
promise: Decorative rack-rail rules: rack separators, strip outer edge, non-cocked band slot, bare-strip dashed outline, `stale` strip field grid, tape minute rule, `session-end` rule and the rule under the ATIS header; those ATIS cells are never boxed.
@row_add n=59 row=contract/border-intensity/standard area=color-palette rec=48
boundary: Border intensities
item: Standard #9C9278
promise: The field-grid rule between cells inside a lit strip, and the 1px box around an ATIS `expired` word, quieter than the boxes of declared states.
@row_add n=60 row=contract/border-intensity/emphasis area=color-palette rec=49
boundary: Border intensities
item: Emphasis #E6D8AE
promise: Borders whose presence is the state: readback box outline and strike, `budget-paused` box, nonzero `skipped` boxes, ATIS `TAPE stopped` box, error strips.
@row_add n=61 row=contract/border-intensity/focus area=color-palette rec=50
boundary: Border intensities
item: Focus #E6D8AE
promise: A 2px solid focus ring, offset 2px, radius 0.
@row_add n=62 row=contract/border-intensity/handoff area=color-palette rec=51
boundary: Border intensities
item: Handoff #5B8DB8
promise: Only the 2px left tick of the outbound and inbound transfer markers, in the rack gap on anthracite; the only blue rule, never on holder, never a strip or cell border.
@row_add n=63 row=contract/cli-transfer-arrow/ascii-arrow area=iconography rec=52
boundary: CLI transfer arrows
item: ->
promise: The CLI prints transfers with ASCII `->` and has no inbound form; links print only as `link` / `unlink` verb output (`overseer -> builder`).
@row_add n=64 row=decision/list-six-fixed-columns area=iconography rec=52
name: viola list has six fixed columns
rule: `viola list` carries no link column, so its six columns stay fixed.
@row_add n=65 row=decision/cli-ascii-only area=iconography rec=53
name: CLI is ASCII only
rule: CLI output uses ASCII characters only, separators included.
@row_add n=66 row=contract/cli-separators/two-spaces area=iconography rec=53
boundary: CLI separators
item: two spaces
promise: Two spaces separate columns, and a refusal's reason from its detail, as separate fields.
@row_add n=67 row=contract/cli-separators/space-hyphen-space area=iconography rec=53
boundary: CLI separators
item: ` - `
promise: Space, hyphen, space replaces a middle dot inside one field or label (`driver - budget-paused`).
