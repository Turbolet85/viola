@row_add n=1 row=decision/no-assertions-for-plugin-and-ipc area=1-a11y-scope-summary rec=1,2
name: Plugin and IPC channel not assertable
rule: The Claude Code plugin hooks, the stdio MCP server and the wrapper IPC channel are not-assertable and carry no assertions.
why: ipc-internal with no UI surface, machine-to-machine traffic only
@row_add n=2 row=decision/no-assertions-for-internal-crates area=1-a11y-scope-summary rec=4,2
name: Internal crates not assertable
rule: The internal crates of the modular monolith are not-assertable and carry no assertions.
why: library-only, with no UI surface
@row_add n=3 row=decision/no-wcag-conformance-claim area=1-a11y-scope-summary rec=5,6,7,8,9
name: No WCAG conformance claim
rule: No WCAG conformance is claimed for the cli or the `viola run` TUI, whether `viola run` or a passed `viola revive` started it; the output-discipline assertions stand in for SC 1.3.2, 1.4.1 and 3.3.1 and are not a11y-tool evidence.
why: WCAG does not apply directly to terminal output, and no automated a11y tool reaches terminal output
@row_add n=4 row=decision/no-sgr-in-plain-modes area=1-a11y-scope-summary rec=5,10,11,8
name: No SGR in plain modes
rule: No SGR, glyph or cursor control is written under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run` or a passed `viola revive`.
@row_add n=5 row=decision/paths-not-carried area=1-a11y-scope-summary rec=12
name: Paths not carried
rule: Tests Critical Path 1 (the `run` start sequence), E1 (unwrapped hooks no-op), E2 (`CLAUDE*` env strip) and security vectors 1–9 are not carried as paths; their user-visible outcomes are covered by Path 4.
why: they run on tui/ipc-internal with no viola-owned UI
@row_add n=6 row=decision/cognitive-accessibility-not-triggered area=1-a11y-scope-summary rec=14
name: Cognitive accessibility not triggered
rule: The cognitive-accessibility trigger is none: no cognitive-accessibility rigor is required.
why: the brief has no cognitive, older-user or language-proficiency signal
@row_add n=7 row=decision/tier-single-source-of-rigor area=1-a11y-scope-summary rec=16
name: Tier as single source of accessibility rigor
rule: The tier is the single source of truth for how much accessibility rigor downstream skills apply, in the phase loop per chunk and in cross-cutting reads from route and setup-project.
@row_add n=8 row=decision/human-keystrokes-never-blocked area=2-a11y-strategy rec=18,19,20,21
name: Human keystrokes never blocked
rule: A human keystroke in `viola run`, or in a start a passed `viola revive` made, is never blocked, refused or delayed past the current atomic paste.
@row_add n=9 row=decision/cli-output-discipline-assertion-tools area=4-aria-patterns-roles rec=22
name: CLI assertable as output discipline
rule: The cli is assertable as output discipline, with no conformance claim, by `std::process::Command` with literal pins and portable-pty outer-PTY SGR checks on the `ubuntu-latest` CI leg.
@row_add n=10 row=decision/no-terminal-conformance-claim area=4-aria-patterns-roles rec=22,24,25,26,27,28
name: No conformance claim for the terminal
rule: The cli checks make no conformance claim for the terminal; they only stand in for SC 1.3.2, 1.4.1 and 3.3.1.
@row_add n=11 row=decision/portable-pty-outer-pty-driver area=4-aria-patterns-roles rec=29,9
name: portable-pty as the outer-PTY boundary driver
rule: The tui boundary assertions run on the portable-pty `=0.9.0` outer-PTY driver, checking that viola emits zero terminal bytes of its own and that human keystrokes are never blocked or delayed.
@row_add n=12 row=decision/wrapped-tui-reads-as-unwrapped area=7-screen-reader-support rec=30,31
name: Wrapped TUI reads as unwrapped
rule: The wrapped `claude` TUI must read to a screen reader the same as unwrapped.
why: Must-Work O1
@row_add n=13 row=decision/manual-checks-supplemental-only area=11-a11y-anti-patterns-never-do-these rec=32
name: Manual checks are supplemental only
rule: Manual screen reader, keyboard or contrast checks are never the ONLY method; as a supplement to automated tools they are OK.
@row_add n=14 row=contract/revive-json-refusal/socket-dir-refused area=a11y-scope-summary rec=33,34
boundary: viola revive --json refusals
item: socket-dir-refused
promise: One exit-1 document with `socket-dir-refused` its code on stdout, in place of the human pair, before any snapshot, bind or child spawn, holding no path, pid or uid, with stderr empty and no hint.
