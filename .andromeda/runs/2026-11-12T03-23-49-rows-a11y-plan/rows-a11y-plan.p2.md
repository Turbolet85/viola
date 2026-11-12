@row_add n=1 row=absence/touch-surface area=1-a11y-scope-summary rec=1
capability: A touch surface
@row_add n=2 row=decision/a11y-tier-single-source area=1-a11y-scope-summary rec=2
name: A11y tier as single source of rigor
rule: The a11y tier is the single source of truth for how much accessibility rigor the downstream skills apply, per chunk and in cross-cutting reads.
@row_none n=1 rec=3 type=S
@row_none n=2 rec=4 type=S
@row_none n=3 rec=5 type=S
@row_none n=4 rec=6 type=S
@row_add n=3 row=decision/no-sgr-without-tty area=2-a11y-strategy rec=7,25,32
name: No SGR without a colour-capable terminal
rule: The CLI emits no SGR of its own under a non-TTY, `NO_COLOR`, `TERM=dumb`, `--json`, `viola run` or a passed `viola revive`.
guard: literal pins over std::process::Command and the portable-pty outer PTY on ubuntu-latest
@row_add n=4 row=decision/word-beside-every-sgr area=2-a11y-strategy rec=7,25,32
name: Word beside every SGR span
rule: Every SGR span on the CLI sits beside its word, so each state is still carried by the word without the colour.
guard: literal pins over std::process::Command and the portable-pty outer PTY on ubuntu-latest
@row_add n=5 row=decision/human-keystrokes-never-blocked area=2-a11y-strategy rec=8,29
name: Human keystrokes never blocked
rule: On the tui, human keystrokes are never blocked or delayed by viola past the current atomic paste.
@row_add n=6 row=contract/cli-refusal/unable-line area=2-a11y-strategy rec=9,44
boundary: CLI refusal output
item: unable line
promise: A refusal prints `unable  <reason>  <detail>` on stderr, the plain word followed by plain detail; `viola revive`'s exit-1 refusals, `viola run`'s socket-directory refusal and `viola verify`'s interrupted pair print one `unable: <text>` line instead.
@row_add n=7 row=contract/cli-refusal/hint-line area=2-a11y-strategy rec=9,44
boundary: CLI refusal output
item: hint line
promise: A refusal ends with `hint:` as the last stderr line, carrying a recovery instruction where recovery exists.
@row_add n=8 row=decision/no-aaa-cognitive-escalation area=2-a11y-strategy rec=10,40
name: No AAA cognitive escalation
rule: Cognitive accessibility is not escalated to AAA; SC 2.2.6, 3.1.5 and 3.3.9 are not claimed, and only cognitive-relevant A/AA criteria with a v1 surface are covered.
why: no cognitive-accessibility trigger fired
@row_none n=5 rec=11 type=S
@row_none n=6 rec=12 type=S
@row_none n=7 rec=13 type=S
@row_none n=8 rec=14 type=S
@row_add n=9 row=decision/cli-structural-landmark-equivalents area=4-aria-patterns-roles rec=15
name: CLI structural equivalents of landmarks
rule: The CLI has no landmark roles; its structure is carried by the BAY context line, the uppercase caption line, the `-- UNWRAPPED - READ-ONLY --` separator and the `--help` group labels.
@row_add n=10 row=decision/tui-boundary-only area=4-aria-patterns-roles rec=15,20
name: Tui accessibility is boundary-only
rule: The tui is the vendor `claude` TUI, and viola's accessibility work on it is held only at its boundary, with no landmarks of its own.
@row_none n=9 rec=16 type=S
@row_none n=10 rec=17 type=S
@row_none n=11 rec=18 type=S
@row_add n=11 row=decision/cli-a11y-assertion-tools area=4-aria-patterns-roles rec=19
name: CLI a11y asserted as output discipline
rule: CLI accessibility is asserted as output discipline, by `std::process::Command` with literal pins and portable-pty outer-PTY SGR checks on the `ubuntu-latest` CI leg.
@row_add n=12 row=decision/no-terminal-conformance-claim area=4-aria-patterns-roles rec=19,25,43
name: No conformance claim for the terminal
rule: The CLI makes no WCAG conformance claim; its checks only stand in for the success criteria they mirror.
@row_add n=13 row=decision/portable-pty-outer-pty-driver area=4-aria-patterns-roles rec=20
name: portable-pty as the outer-PTY boundary driver
rule: The tui boundary is driven through a portable-pty outer PTY, pinned exactly at `=0.9.0`, asserting zero viola bytes and unblocked keystrokes.
@row_none n=12 rec=21 type=S
@row_none n=13 rec=22 type=5
@row_none n=14 rec=23 type=5
@row_add n=14 row=decision/send-refused-while-human-typing area=4-aria-patterns-roles rec=24
name: Send refused while the human types
rule: When the human takes the wheel, `send` is refused with `human-typing` on stderr and a typed exit code.
guard: std::process::Command asserts
@row_add n=15 row=decision/wheel-ignores-terminal-events area=4-aria-patterns-roles rec=24
name: Focus, mouse and resize never move the wheel
rule: On the tui, focus, mouse and resize sequences do not move the wheel, and viola originates zero bytes on the human's terminal.
guard: outer-PTY nextest cases on ubuntu-latest
@row_add n=16 row=decision/c0-c1-escaped area=4-aria-patterns-roles rec=25
name: C0 and C1 escaped in CLI output
rule: C0 and C1 control characters in CLI output are escaped.
guard: literal pins over std::process::Command on ubuntu-latest
@row_add n=17 row=decision/linear-static-output area=4-aria-patterns-roles rec=25
name: Linear static CLI output
rule: CLI output is linear and static.
guard: literal pins over std::process::Command on ubuntu-latest
@row_add n=18 row=decision/stale-row-dim area=4-aria-patterns-roles rec=26,32
name: Dim only on stale rows
rule: Dim SGR is applied only to stale rows, beside the word that carries the state.
guard: unit-tier tests and outer-PTY tests on ubuntu-latest
@row_none n=15 rec=27 type=S
@row_add n=19 row=decision/cli-no-interactive-input area=5-keyboard-navigation rec=28
name: No interactive CLI input
rule: The CLI has no interactive prompt, pager or redraw, and takes input only from stdin or `--file`.
@row_add n=20 row=decision/tui-terminal-belongs-to-child area=5-keyboard-navigation rec=29
name: Tui terminal belongs to the child
rule: On the tui, the terminal belongs to the wrapped child process, not to viola.
@row_none n=16 rec=30 type=S
@row_none n=17 rec=31 type=S
@row_add n=21 row=decision/dialog-amber-sgr area=6-visual-design-verification rec=32
name: Amber only on the DIALOG word
rule: Amber SGR is applied only to the `DIALOG` word.
@row_add n=22 row=decision/name-bold-sgr area=6-visual-design-verification rec=32
name: Bold only on NAME values
rule: Bold SGR is applied only to NAME values.
@row_add n=23 row=decision/unable-fail-never-coloured area=6-visual-design-verification rec=32
name: Refusal and failure words never coloured
rule: The words `unable` and `fail` are never coloured.
@row_add n=24 row=decision/name-truncation area=6-visual-design-verification rec=32
name: NAME truncation only on a narrow terminal
rule: An unwrapped NAME is cut with `...` only on a terminal too narrow and never inside a `\xHH` escape; a wrapped NAME, piped output and `--json` are never cut.
@row_none n=18 rec=33 type=S
@row_none n=19 rec=34 type=S
@row_none n=20 rec=35 type=S
@row_none n=21 rec=36 type=S
@row_add n=25 row=decision/screen-reader-target-cli area=7-screen-reader-support rec=37,19
name: Orca with GNOME Terminal for the cli
rule: Manual screen-reader checks of the cli use Orca with GNOME Terminal on Linux: linear output, refusals read as text, no escape noise.
@row_add n=26 row=decision/screen-reader-target-tui area=7-screen-reader-support rec=38,20
name: Orca with foot on Hyprland for the tui
rule: Manual screen-reader checks of the tui use Orca with foot on Hyprland on Linux.
@row_add n=27 row=decision/wrapped-tui-reads-as-unwrapped area=7-screen-reader-support rec=38
name: Wrapped TUI reads as unwrapped
rule: The wrapped `claude` TUI reads to a screen reader the same as the unwrapped one.
@row_none n=22 rec=39 type=S
@row_add n=28 row=absence/user-time-limit area=8-cognitive-accessibility rec=41,42
capability: A time limit on the user, and any timeout-extension component
@row_add n=29 row=decision/time-bounds-never-on-human area=8-cognitive-accessibility rec=43
name: Time bounds never fall on the human
rule: Every time bound viola holds (wait timeout and cap, dialog hold, send readiness gate, confirmation windows, status-line deadline, agents read, budget pause) falls on the driver, the user's command or viola's child; no key is blocked, refused or delayed by it.
@row_add n=30 row=contract/cli-refusal/exit-code area=8-cognitive-accessibility rec=44
boundary: CLI refusal output
item: exit code
promise: Every refusal ends with a typed exit code.
@row_add n=31 row=contract/cli-refusal/unknown-refusal area=8-cognitive-accessibility rec=44
boundary: CLI refusal output
item: the `unknown` refusal
promise: The `unknown` refusal exits 14 and is the one refusal with no hint; its detail is a code word or nothing.
@row_add n=32 row=contract/cli-refusal/fault area=8-cognitive-accessibility rec=44
boundary: CLI refusal output
item: fault
promise: Faults are not refusals and carry no hint.
@row_add n=33 row=contract/cli-refusal/json area=8-cognitive-accessibility rec=44
boundary: CLI refusal output
item: --json
promise: Under `--json` a refusal gives one document with no hint, except the `viola answer` and `viola send` exit-2 paths, which print a fixed stderr line and no document.
@row_none n=23 rec=45 type=0
what: the WCAG success criteria the error pattern is mapped to
@row_add n=34 row=decision/refusal-layout-literal-pins area=8-cognitive-accessibility rec=46
name: Refusal layout held by literal pins
rule: Each refusal's `unable <reason> <detail>` plus final `hint:` layout, its typed exit code and a hint-free `--json` are held by literal asserts in CI on ubuntu-latest.
guard: literal pins over std::process::Command on ubuntu-latest
@row_add n=35 row=decision/no-reading-level-gate area=8-cognitive-accessibility rec=47
name: No reading-level gate
rule: No reading-level target is gated and no readability scorer is introduced.
why: SC 3.1.5 Reading Level (AAA) is not claimed, and no readability scorer is in a11y-research
@row_add n=36 row=decision/fixed-copy-from-design area=8-cognitive-accessibility rec=48
name: Fixed CLI copy comes from design
rule: The CLI's fixed copy set (`no wrapped sessions — start one with  viola run <name> -- claude`, `unknown`, `n/a`) comes from design.
@row_add n=37 row=decision/reason-code-beside-unable area=8-cognitive-accessibility rec=48
name: Reason codes always beside unable
rule: Every kebab-case reason or detail code (`not-delivered`, `human-typing`, `unverified-cli`) always sits beside the plain word `unable`.
@row_add n=38 row=decision/no-bare-error-text area=8-cognitive-accessibility rec=48
name: No stack trace or bare status as error text
rule: The CLI never shows a stack trace or a bare status number as the only error text.
@row_add n=39 row=decision/fixed-cli-strings-pinned area=8-cognitive-accessibility rec=49
name: Fixed CLI strings pinned by literal
rule: Every fixed CLI string, hints and result lines included, is pinned by literal.
why: so any copy change fails CI until the pinned string is updated on purpose
@row_add n=40 row=contract/viola-list-caption/order area=8-cognitive-accessibility rec=50,51
boundary: viola list caption
item: order
promise: The caption order is `NAME · LIVE · STATUS · WHEEL · DIALOG · CLI`, pinned by the `CAPTION` literal.
@row_none n=24 rec=52 type=S
@row_none n=25 rec=53 type=S
@row_add n=41 row=decision/a11y-ci-github-actions area=9-ci-integration rec=54
name: A11y checks in the GitHub Actions push/PR workflow
rule: A11y checks run in the one GitHub Actions push/PR workflow on an `ubuntu-latest` matrix; the scheduled nightly workflow carries no a11y step.
@row_none n=26 rec=55 type=S
