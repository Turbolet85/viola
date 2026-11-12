@row_none n=1 rec=1 type=S
@row_none n=2 rec=2 type=0
what: an editing note for the plan document itself
@row_none n=3 rec=3 type=S
@row_none n=4 rec=4 type=S
@row_add n=1 row=decision/a11y-tier-standard area=1-a11y-scope-summary rec=5
name: A11y tier Standard (1)
rule: The a11y tier is Standard (1): cli and tui get output-discipline and boundary assertions plus manual screen-reader passes, with no WCAG conformance claim.
@row_none n=5 rec=6 type=S
@row_add n=2 row=absence/web-or-graphical-frontend area=1-a11y-scope-summary rec=7
capability: A web or graphical frontend
@row_none n=6 rec=8 type=S
@row_add n=3 row=decision/no-a11y-tool-output-discipline-assertions area=1-a11y-scope-summary rec=21,9,18
name: Output-discipline assertions instead of an a11y tool
rule: cli and tui use no a11y tool: the existing process, literal-pin and portable-pty output-discipline assertions on text and streams stand in, with manual screen-reader passes as a supplement.
why: no automated a11y tool reaches a terminal
@row_add n=4 row=contract/viola-revive-json/start-refusal area=1-a11y-scope-summary rec=10
boundary: viola revive --json output
item: start refusal
promise: Every refusal of the start that revive shares with run prints one exit-1 document, pinned-hash-mismatch and squatted-name among them, and a program not found as internal-error.
@row_add n=5 row=contract/viola-revive-json/pump-child-error area=1-a11y-scope-summary rec=10
boundary: viola revive --json output
item: pump_child error
promise: A pump_child error prints no document: the child has held the terminal, and viola writes nothing past it.
@row_add n=6 row=contract/tui-passthrough/entry area=1-a11y-scope-summary rec=11
boundary: viola run TUI passthrough
item: entry
promise: The passthrough is entered by viola run and by a viola revive whose preflight passed.
@row_add n=7 row=contract/tui-passthrough/vendor-content area=1-a11y-scope-summary rec=11,19
boundary: viola run TUI passthrough
item: vendor content
promise: The rendered content belongs to the vendor claude and is never traced or parsed; no conformance claim is made about the claude TUI.
@row_add n=8 row=contract/tui-passthrough/viola-terminal-bytes area=1-a11y-scope-summary rec=11,19
boundary: viola run TUI passthrough
item: viola terminal bytes
promise: viola writes zero bytes of its own to the terminal; only the child's output reaches it.
@row_add n=9 row=contract/tui-passthrough/human-keystrokes area=1-a11y-scope-summary rec=11,19,22,35,36
boundary: viola run TUI passthrough
item: human keystrokes
promise: Human keystrokes are never blocked, refused or delayed past the current atomic paste.
@row_add n=10 row=contract/tui-passthrough/focus-mouse-resize-sequences area=1-a11y-scope-summary rec=11,35
boundary: viola run TUI passthrough
item: focus, mouse and resize sequences
promise: Focus, mouse and resize sequences and terminal replies pass through and do not count as human editing for wheel purposes.
@row_add n=11 row=contract/tui-passthrough/vt100-feed-panic area=1-a11y-scope-summary rec=11
boundary: viola run TUI passthrough
item: vt100 feed panic
promise: The boundary still holds after a contained vt100 feed panic: no viola literal reaches the outer terminal, and the child's own bytes still reach it.
@row_add n=12 row=decision/no-assertions-for-plugin-and-ipc area=1-a11y-scope-summary rec=13,12
name: Plugin and IPC channel not a11y-assertable
rule: The Claude Code plugin hooks, the stdio MCP server and the wrapper IPC channel carry no a11y assertions.
why: ipc-internal machine-to-machine traffic with no UI surface
@row_add n=13 row=decision/no-assertions-for-internal-crates area=1-a11y-scope-summary rec=14,12
name: Internal crates not a11y-assertable
rule: The internal crates of the modular monolith carry no a11y assertions.
why: library-only, with no UI surface
@row_add n=14 row=decision/single-vendor-content-boundary area=1-a11y-scope-summary rec=15
name: Single vendor-content boundary
rule: The viola run TUI passthrough, which a passed viola revive also enters, is the only boundary-only vendor-content zone.
@row_add n=15 row=decision/no-compliance-regime area=1-a11y-scope-summary rec=16,51,52
name: No accessibility compliance regime
rule: No accessibility compliance regime applies, so no regime-mandated WCAG success criterion binds any surface.
@row_none n=7 rec=17 type=S
@row_add n=16 row=decision/no-wcag-conformance-claim area=1-a11y-scope-summary rec=18,42
name: No WCAG conformance claim for terminal output
rule: No WCAG conformance claim may rest on the terminal output-discipline assertions.
why: WCAG does not apply directly to terminal output
@row_add n=17 row=decision/colour-second-cue-only area=1-a11y-scope-summary rec=18,36,42,47
name: Colour only a second cue
rule: Colour is never the sole cue on the CLI: the word always sits beside any SGR.
@row_add n=18 row=decision/no-sgr-in-plain-modes area=1-a11y-scope-summary rec=18,47
name: No SGR in plain modes
rule: No SGR, glyph or cursor control is written under --json, non-TTY, NO_COLOR, TERM=dumb, viola run or a passed viola revive.
@row_add n=19 row=decision/static-linear-output area=1-a11y-scope-summary rec=18,41,54
name: Static linear output
rule: viola's own output is static and linear, with no spinner or redraw.
@row_add n=20 row=contract/cli-output/control-characters area=1-a11y-scope-summary rec=18
boundary: CLI output
item: C0/C1 control characters
promise: C0/C1 control characters are escaped except \n and \t, which list rows escape too.
@row_add n=21 row=decision/tui-boundary-assertions-only area=1-a11y-scope-summary rec=19,22
name: TUI boundary assertions only
rule: The tui is asserted only at its boundary, through the portable-pty outer-PTY driver; the result is a boundary guarantee, never a conformance claim.
@row_add n=22 row=contract/tui-passthrough/run-diagnostics area=1-a11y-scope-summary rec=19
boundary: viola run TUI passthrough
item: run diagnostics
promise: viola's diagnostics for run go to diagnostics/, never to the terminal.
@row_none n=8 rec=20 type=S
@row_add n=23 row=decision/cli-not-keyboard-interactive area=1-a11y-scope-summary rec=22
name: CLI not keyboard-interactive
rule: The CLI has no prompt, TUI mode or pager; its input comes only from stdin or --file.
@row_add n=24 row=decision/manual-keyboard-pass area=1-a11y-scope-summary rec=22,48
name: Manual keyboard pass as supplement
rule: A manual keyboard pass over CLI and TUI supplements the automated keystroke boundary assertion.
@row_add n=25 row=decision/manual-orca-pass area=1-a11y-scope-summary rec=23,49
name: Manual Orca screen-reader pass
rule: A manual Orca screen-reader pass on Section 4 P4 and P6, founder-owned, supplements the automated assertions.
why: no blind or low-vision target users are named, so screen-reader priority is scoped to CLI/TUI linearity, not a full assistive-tech matrix
@row_add n=26 row=decision/no-browser-a11y-leg area=1-a11y-scope-summary rec=24
name: No browser a11y leg
rule: CI runs no browser a11y leg.
why: viola renders no page
@row_add n=27 row=decision/a11y-inside-existing-suites area=1-a11y-scope-summary rec=24
name: A11y assertions inside existing suites
rule: A11y and CLI output-discipline assertions run inside the existing nextest-integration, nextest-e2e and coverage suites; no new suite enum value such as a11y is added without a Decisions Log entry.
@row_add n=28 row=decision/ci-test-gate-no-skips area=1-a11y-scope-summary rec=24
name: CI test gate allows no skips
rule: The CI test job's gate requires coverage and doctest and never allows skips.
@row_none n=9 rec=25 type=S
@row_add n=29 row=decision/critical-path-send-readback area=1-a11y-scope-summary rec=27,26,25
name: Critical path: confirmed send and readback
rule: Confirmed send and readback, or refusal, through the viola send readback mirror [RB] is a must-be-accessible critical path.
@row_none n=10 rec=28 type=0
what: a list of source citations
@row_add n=30 row=decision/critical-path-dialog-answer area=1-a11y-scope-summary rec=30,29,25
name: Critical path: dialog pending, then answer
rule: A pending dialog answered with viola answer (question, permission, plan), including the unverified-cli refusal that withholds answers, is a must-be-accessible critical path.
@row_none n=11 rec=31 type=0
what: a list of source citations
@row_add n=31 row=decision/critical-path-human-takes-wheel area=1-a11y-scope-summary rec=33,32,25
name: Critical path: human takes the wheel
rule: The human taking the wheel in the tui, automation refused (send refused with human-typing) and release returning it, with pause and release on the cli, is a must-be-accessible critical path.
@row_none n=12 rec=34 type=0
what: an N/A ARIA-roles line for terminal surfaces
@row_add n=32 row=contract/cli-refusal/format area=1-a11y-scope-summary rec=36,42
boundary: CLI refusals
item: refusal format
promise: A refusal is written to stderr as unable … hint:, giving the reason and detail in text, with a typed exit code.
@row_none n=13 rec=37 type=0
what: a list of source citations
@row_add n=33 row=decision/critical-path-cli-board-output area=1-a11y-scope-summary rec=39,38,25
name: Critical path: CLI board, wait/last and verify output
rule: The CLI board, wait/last and verify output, with viola list's fixed captions and slot order, is a must-be-accessible critical path.
@row_add n=34 row=decision/terminal-structural-equivalents area=1-a11y-scope-summary rec=40
name: Terminal structural equivalents of ARIA roles
rule: In place of ARIA roles, terminal structure is carried by the caption line, the unwrapped separator and the --help group labels.
@row_add n=35 row=contract/viola-list/unwrapped-separator area=1-a11y-scope-summary rec=40
boundary: viola list output
item: -- UNWRAPPED - READ-ONLY --
promise: The separator appears only when unwrapped rows exist.
@row_add n=36 row=contract/viola-wait/waiting-line area=1-a11y-scope-summary rec=41
boundary: viola wait output
item: waiting: <name>
promise: The waiting line is written once on stderr, on a TTY only.
@row_add n=37 row=contract/viola-verify/step-line area=1-a11y-scope-summary rec=41
boundary: viola verify output
item: step line
promise: Each verify step prints a line [NN/NN] … pass|fail.
@row_add n=38 row=decision/output-discipline-stands-in-for-sc area=1-a11y-scope-summary rec=42
name: Output discipline stands in for WCAG SC
rule: Output-discipline assertions stand in for SC 1.3.2, 1.4.1 and 3.3.1: meaningful linear order, colour never the sole cue, and the refusal reason and detail given in text.
why: WCAG does not apply directly to terminal output
@row_none n=14 rec=43 type=0
what: a list of source citations
@row_add n=39 row=decision/paths-not-carried area=1-a11y-scope-summary rec=44
name: Paths not carried as a11y critical paths
rule: The run start sequence, unwrapped hooks no-op, CLAUDE env strip and security vectors 1–9 are not carried as a11y critical paths; their user-visible outcomes are covered by Path 4.
why: they run on tui or ipc-internal with no viola-owned UI
@row_none n=15 rec=45 type=S
@row_none n=16 rec=46 type=S
@row_none n=17 rec=50 type=S
@row_add n=40 row=decision/cognitive-accessibility-not-triggered area=1-a11y-scope-summary rec=53
name: Cognitive accessibility not triggered
rule: Cognitive accessibility is not a triggered a11y concern.
why: the brief has no cognitive, older-user or language-proficiency signal
@row_add n=41 row=absence/internationalization area=1-a11y-scope-summary rec=55
capability: Internationalization: no i18n, RTL or lang conventions
