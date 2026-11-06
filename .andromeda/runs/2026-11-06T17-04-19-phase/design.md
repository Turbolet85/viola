@extract_section name=relevance
partial — no new UI surface (scope §Boundaries excludes the 0.2.0 frontend); design reaches only through the CLI surface touched by item 17's remnant sites (tests/cli_output_plain.rs, plain_drive in src/cmd/mod.rs), the portable-pty move under `viola run`, item 4's v1-13/v1-36 dispositions, and the design-system master where "the masters follow" the remnant removal
@extract_section name=constraints
- design-system §Surface: cli › Tokens (platform-specific) (sec-2yd5l7qf) requires the colour decision order (`--json`, `viola run`, `hook`/`mcp`, `NO_COLOR`, `TERM=dumb`, non-terminal stdout) to yield no colour, no glyph and no SGR while the ASCII readback mirror `[RB]` / `[  ]` / `[/ ]` and `->` still print; removing the Windows remnants at tests/cli_output_plain.rs:104,116 and `plain_drive` must leave that order and its plain-output pins intact — whether `plain_drive` is a Windows-only path or the plain-mode driver itself is research's question.
- design-system §Surface: cli (sec-fwogi3jj) requires the CLI platform to be Linux terminals (CI on `ubuntu-latest`) with styling only from the hand-written SGR module in the `viola` bin and clap built without its `color` feature; a portable-pty bump (item 12) must add no dependency that reads a colour or terminal variable.
- design-system §Component Patterns (sec-qhdahq5c) requires `viola run` and `viola revive` to print nothing once the child starts, and start refusals to print only their fixed `unable:` / `hint:` pair with no path or pid; the portable-pty move off `=0.8.1` must keep that passthrough silence.
- design-system §Platform-Specific Notes (sec-c5p5tzq6) still names a Git Bash on Windows rewritten-path warning line; under item 17 ("the masters following") that line is a Windows remnant in this master to be removed or kept with its reason — whether the code still emits `warning: argument looks like a Git Bash rewritten path` is research's question.
- design-system §Iconography (sec-ucg62raw) requires `viola list` to carry no link column in v1 so its six columns stay fixed; item 4's v1-36 disposition ("link set on the board") must be a deferral or a re-wording consistent with that, never a new board column (scope §Boundaries forbids new surface too).
- design-system §Navigation Pattern (sec-c3cix4pc) requires flat verbs with `mcp` hidden from `viola --help`; the v1-13 disposition (MCP result for `link`) adds no verb or help row.
@extract_section name=patterns
- design-system §Component Patterns (sec-qhdahq5c) fixes the `viola send` readback mirror lines (`[  ] open`, `[RB] read back … cursor N`, `[/ ] unable … reason detail` + `hint:`) — the strings any trycmd/assert_cmd pin under item 1 would hold.
- design-system §Surface: cli › Tokens (sec-2yd5l7qf) fixes the stream split: results on stdout; `waiting:`, refusals, `hint:` lines and errors on stderr; one JSON document on stdout under `--json` with no hint.
- design-system §Component Patterns (sec-qhdahq5c) fixes exit-code phraseology (0/1/2/10–14/20/21 and exit 21's six closed `detail` causes) — the human/JSON forms the masters-agree check compares against tests.
- design-system §Surface: cli (sec-fwogi3jj) names std `IsTerminal` for TTY detection and a hand-written SGR module with no dependency.
@extract_section name=anti_patterns
- design-system §Per-Surface Bans (sec-yx5hed2k): never emit colour, glyphs or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb` or `viola run` (a passed `viola revive` likewise).
- design-system §Per-Surface Bans (sec-yx5hed2k): never print upstream text, paths, pids or anyhow chains in errors or hints, and never stack traces — remnant removal and dependency moves must not surface them.
- design-system §Rejected Defaults (sec-mu7ggq5u): no optimistic "Sent ✓"/spinner — a send counts only when read back, so the live driver session (hint clause 4) is judged by `[RB]` lines, not by `turn-ended`.
@extract_section name=bindings
- design ↔ tests: item 1's trycmd/assert_cmd decision pins CLI strings that design-system §Component Patterns (sec-qhdahq5c) owns; tests/cli_output_plain.rs is the plain-output pin of §Tokens (sec-2yd5l7qf).
- design ↔ architecture/test-plan (masters agree): item 17's remnant removal reaches design-system §Platform-Specific Notes (sec-c5p5tzq6) and the Linux platform line of §Surface: cli (sec-fwogi3jj).
- design ↔ verification matrix: v1-36's and v1-13's dispositions (item 4) bind to §Iconography (sec-ucg62raw, no link column in v1) and §Navigation Pattern (sec-c3cix4pc, `mcp` hidden).
- design ↔ a11y: the CLI's colour as second cue only (DIALOG word + amber, stale row + dim) binds to a11y use-of-colour; unchanged by this chunk.
@extract_section name=acceptance
- After the item 17 removals, a piped / `NO_COLOR` / `TERM=dumb` / `--json` run of `viola list` and `viola send` emits no SGR byte while `[RB]` / `[  ]` / `[/ ]` and `->` still print in human output (per design-system §Surface: cli › Tokens (platform-specific))
- `viola run` under the moved portable-pty prints nothing to the terminal once the child starts, and a start refusal prints only its fixed `unable:` / `hint:` pair with no path or pid (per design-system §Component Patterns)
- design-system §Platform-Specific Notes holds no Windows-only behaviour at wrap unless the code keeps it with a stated reason (per design-system §Platform-Specific Notes)
- The live driver-drives-builder session's `viola send` result reads `[RB] read back  <name>  <time>  cursor <n>` on stdout, exit 0 (per design-system §Signature Test)
