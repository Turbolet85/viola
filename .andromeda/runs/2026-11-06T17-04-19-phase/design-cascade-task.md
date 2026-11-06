@task_prose part=chunk
Closes viola-0.1.0 with mechanism and record work only: moves `portable-pty` off `=0.8.1` to a release whose graph drops `pin-utils` (RUSTSEC-2026-0441), with no ignore-list widening, so every gate and CI's seven jobs read green on Linux.
Removes or keeps with its reason each Windows remnant left in code (item 17), the masters following, and re-measures the remnant coordinates, `PTY_SIDELOAD_TEST_CASE` and the absent `tests/cmd/` at HEAD.
Runs the masters-agree check where items 1, 3, 6 and 16 meet: trycmd / assert_cmd tool-or-text, the test-child marker row, the one-enum-per-crate folds or ratifications, the `thiserror` derive census; lands F434's two whole-fact master edits.
Measures the channel-handle no-leak property on Linux and reads live, within two headless live sessions, `v1-53`'s `viola run` and a driver driving a wrapped builder through viola.
Brings the 17 unclaimed caps (v1-13 and v1-36 among them) to the founder for proved-or-deferred dispositions, recorded in the verification matrix; adds no verb, flag, env var, channel method or 0.2.0 frontend.
@task_file n=1 path=Cargo.toml mark=edits
what: the `portable-pty` dependency pin
@task_file n=2 path=Cargo.lock
@task_file n=3 path=deny.toml mark=re-reads
what: `cargo deny check advisories` read green with no ignore-list widening
@task_file n=4 path=crates/viola-pty
@task_file n=5 path=tests/cli_output_plain.rs mark=edits
what: the Windows remnants at lines 104 and 116
@task_file n=6 path=crates/viola-agent-claude/src/lib.rs mark=edits
what: the Windows remnant at line 2
@task_file n=7 path=crates/viola-e2e/src/harness/session.rs mark=edits
what: the Windows remnant at line 52
@task_file n=8 path=crates/viola-e2e/src/harness mark=edits
what: the `EXE_SUFFIX` joins
@task_file n=9 path=src/cmd/mod.rs mark=edits
what: `plain_drive`
@task_file n=10 path=tests/contract_os_gated_tests.rs mark=re-reads
what: the OS-gated census (39 OS-gated test functions, 2 gated files) that `v1-53`'s claim reads
@task_item n=1 part=needs
The cli surface's fixed human strings (the `viola list` board, the `viola send` readback mirror, `wait` / `last` result lines, the wheel, link and gate verb lines, `verify` / `run` lines, exit-code phraseology): item 1 decides whether trycmd / assert_cmd pins them under `tests/cmd/*.toml` or the masters' text drops that claim, so the masters-agree check reaches design-system wherever it states how these strings are pinned.
@task_item n=2 part=needs
`viola list` carries no link column in v1 and its six columns stay fixed (Iconography and the cli-list patterns): v1-36's clause «link set on the board» is deferred or re-worded against this, and the master text must agree with whichever the founder rules.
@task_item n=3 part=needs
`viola link` / `viola unlink` output phraseology and the `viola release <name> --budget` line (cli-verbs), with `mcp` as a hidden verb writing MCP frames only (Platform-Specific Notes): v1-13's clause on the MCP result for `link` and the budget release is deferred or re-worded against these.
@task_item n=4 part=needs
The colour decision order (`--json`, `viola run`, `hook` / `mcp`, `NO_COLOR`, `TERM=dumb`, non-terminal stdout) and the plain-output contract: removing the remnants in `tests/cli_output_plain.rs` and `plain_drive` must leave every plain-mode output and its assertions unchanged.
@task_item n=5 part=needs
The cli Platform-Specific Notes' Git Bash on Windows rewritten-path warning: where an item 17 remnant removal reaches that warning, the design-system line follows (removed, or kept with its reason) as one of the masters following.
@task_item n=6 part=needs
The cli Platform line (Linux terminals, CI on `ubuntu-latest`): the version done-check's «every gate green on Linux» and `v1-53`'s `ubuntu-latest` JUnit read sit on this platform statement, which must still agree once the remnants leave.
@task_item n=7 part=needs
Indirect, through the `Cargo.toml` edit: clap stays built without its `color` feature and no added or moved dependency reads a colour or terminal variable, so every styled byte stays the SGR module's own; the `portable-pty` move adds no dependency to the `viola` bin's styling.
@task_item n=8 part=needs
Indirect, through the `portable-pty` / `crates/viola-pty` move: `viola run` (and a passed `viola revive`) prints nothing once the child starts and emits no colour, glyph or cursor control, the terminal belonging to the child's screen; the new pty release must keep that passthrough.
@task_item n=9 part=needs
Indirect, through item 6's enum folds or ratifications (`ServeError`, `McpError`, `StampsError`, `StateError`, `PinError`, `StampsReadError`, `Refused`): the typed exit codes, their human stderr forms, the `--json` `error` / `refusal` documents and the closed `detail` words stay exactly as the exit-code phraseology states; stack traces never print and detail goes to `diagnostics/` only.
@task_item n=10 part=needs
Item 7's founder word on the socket-directory pair's wording: the `socket-dir-refused` cause of exit 21 and of `viola revive --json`'s start codes, and its fixed `unable:` / `hint:` pair, are where that wording lives on the cli surface.
@task_item n=11 part=needs
Item 7's founder word on the board's read rule: the `viola list` as-built rule (a wrapped row joined to a `claude agents --json` row reads `idle` or `busy`, else `unknown`; unwrapped rows after the read-only separator) is the rule ruled on.
@task_item n=12 part=needs
The live session in which a driver drives a wrapped builder through viola (and `v1-53`'s live `viola run`): its evidence reads the `viola send` readback mirror (`[RB] read back` with its cursor, or `unable` plus reason) and the `wait` result line, the send counting only once read back.
@task_item n=13 part=needs
v1-26's surfaces (the three overseer fix passes): re-read as honoured, where they fall on the cli surface's patterns.
@task_standing n=1
convention: any new or re-worded assertion over a CLI human string (a trycmd / assert_cmd case or a text pin) pins it exactly as the cli Component Patterns give it: ASCII only, two spaces between columns, results on stdout and refusals with their `hint:` lines on stderr
home: surface
@task_standing n=2
convention: any new test reading human output from a pipe, under `NO_COLOR`, `TERM=dumb` or `--json` expects no colour, no glyph and no SGR, the colour decision order's first match winning
home: surface
@task_item n=1 part=not_needed
The web-spa surface and every 0.2.0 frontend design input: Core Colors, Surface Scale, Text Hierarchy, Semantic Colors and domain status colours, Border Progression, Spacing, Depth Strategy, Border Radius; the frontend is out of this chunk.
@task_item n=2 part=not_needed
Typography's per-OS fallback stacks and their Windows-only faces (Bahnschrift, Cascadia Mono) and the Linux-fallback render assertions: a 0.2.0 frontend input no CI step renders; item 17's remnants are code sites, not these stacks.
@task_item n=3 part=not_needed
Brand Identity's personality, domain anchors, signature-element drawing and expression level, and the Motion duration scale: the chunk adds no surface and no motion.
@task_item n=4 part=not_needed
Iconography's web glyphs (`→`, `←`, `·`, the CSS strike), its library and size grid: web-only, the frontend deferred.
@task_item n=5 part=not_needed
The Self-Validation Protocol's Swap, Squint, Token and Sameness tests: they gate presenting new UI output, and the chunk presents none.
@task_item n=6 part=not_needed
The Rejected Defaults list (status dots, spinner and toast, auto-sorting, budget rings, wheel icons, shimmer and glow): design choices of a surface this chunk does not build or change.
@task_item n=7 part=not_needed
The cli Width rules (NAME truncation of unwrapped rows where width is known, no wrapping) and the list-row and message-mode control escapers: no table, row or escaper changes here.
@task_item n=8 part=not_needed
The `viola plugin install` four-line output and the `viola verify` step counter: neither verb's output is touched by the dependency move, the remnant removal or the master edits.
@task_item n=9 part=not_needed
The Design Decisions Log and its amendments archive: history, not a rule this chunk acts on.
