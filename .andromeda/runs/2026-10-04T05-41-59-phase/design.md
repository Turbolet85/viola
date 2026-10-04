# design extract

## Relevance
partial — only the CLI surface applies: the `viola send` `[RB]` readback mirror, its streams and its per-reason hints. Every web-spa token, motion and component is out of scope here because `viola-ui` and the readback tape are boundaried to `:137` / `:139`.

## Constraints
- The `viola send` readback mirror has four line shapes: `[  ] open … issued <ts>`, `[RB] read back … <ts>  cursor N`, `[/ ] unable … not-delivered  <detail>`, and `[  ] unconfirmable … local command, no measured post-condition`. Their stream, exit and column padding are fixed, and `unable` is padded to the readback-word column (per design-system §Surface: cli → Component Patterns 2; Streams). Lines are appended and never redrawn: no spinner and no in-place update (per design-system §Brand Identity expression table, cli rows; §Per-Surface Bans cli).
- Streams split: the `[RB]` and `unconfirmable` result lines go to stdout. The `open` issue line (TTY only), the `[/ ] unable` refusal and its `hint:` line directly after it go to stderr. Under `--json` the result or refusal is one JSON document on stdout with the typed exit, and no `hint:` line is printed (per design-system §Surface: cli → Streams). Which stream's `IsTerminal` gates the issue line is not settled. The plan's sample says "stderr, TTY only", while CARRY 4 reads "stdout-is-a-terminal". This is P4's to pin against layout-templates :405.
- The send mirror carries no colour at all. CLI colour is only amber `DIALOG` and dim `stale`, so every send line is plain in every mode. The ASCII mirror `[RB]` / `[  ]` / `[/ ]` still prints in piped, `NO_COLOR` and `TERM=dumb` output, and only `--json` replaces it (per design-system §Surface: cli → Tokens; Colour decision order; §Per-Surface Bans cli).
- Hint text is fixed per reason · detail. The `not-delivered` hints owed here are `control-character`, `input-not-ready` and `no-prompt-submitted`. `turn-running` and `unknown-dialog` belong to later entries, unless P4 pulls them in. An `instance-unreachable` refusal (exit 21) gets one hint per cause: not running, unwrapped, strict-modes or server verification. No hint names `viola release` (per design-system §Surface: cli → Component Patterns 2).
- Exit-code phraseology: 13 prints `unable  not-delivered  <detail>` with a refusal object under `--json`. 0 prints `[RB] read back` on stdout with `{"v":1,"ok":{…}}`. 21 prints `unable  instance-unreachable` with `{"v":1,"error":"instance-unreachable","detail":null}`. Exit 14 (`unknown`), 20 and 1 print no hint (per design-system §Surface: cli → Exit-code phraseology table).
- Human output is ASCII only. Lines are never wrapped, and nothing is truncated except the NAME of unwrapped `list` rows (per design-system §Surface: cli → Width).
- Prompt text comes only from stdin or `--file`. A Git Bash rewritten-path argument triggers one plain stderr line, `warning: argument looks like a Git Bash rewritten path` (per design-system §Surface: cli → Platform-Specific Notes).

## Patterns to follow
- Styled and unstyled human output goes through the root bin's own hand-written SGR / human-output module, with no new dependency and clap without `color` (per design-system §Surface: cli → Toolkit / Framework). This binds to CARRY 4's "extend `src/human.rs`". Whether `src/human.rs` already holds a stdout/stderr split or any SGR module is research's question.
- Phraseology words are the design. Use the fixed words `open`, `read back`, `unable`, `unconfirmable`, `not-delivered` and the typed detail codes verbatim as the vocabulary of every line (per design-system §Brand Identity → Domain anchors, "Standard phraseology").
- Confirm, never presume. The mirror reports a send only after the matching `prompt-submitted` or a measured post-condition. Otherwise it prints `unable` plus the typed reason, or `unconfirmable` (per design-system §Brand Identity → Signature element; §Rejected Defaults "Optimistic Sending… spinner").
- A result line says what was read back, by whom and at which cursor, padded as in the sample (per design-system §Per-Surface Bans cli, "never print done/success without context").

## Anti-patterns to avoid
- No `✓` / `✗` prefixes, emoji, spinners, progress bars, colour, or cursor control on any send line. The mirror brackets are the only prefixes (per design-system §Per-Surface Bans cli).
- No hint or refusal line quotes the sent text or any upstream text, and none prints a path, a pid or an anyhow chain. Errors are fixed messages and stack traces never print (per design-system §Per-Surface Bans cli; §Surface: cli → Component Patterns 2, last bullet; Platform-Specific Notes).
- Never mix data and messages. Results go to stdout only, and context, refusals and hints go to stderr only (per design-system §Per-Surface Bans cli).

## Contract bindings
- design ↔ layout-templates: the mirror sample and the stream and exit columns must match layout-templates :405 (`viola send` §Output structure) and :519 (§Hero output line). The issue-line TTY-gating stream is the open question above.
- design ↔ architecture §Conventions: exit codes 13, 21 and 0 and their `--json` shapes are architecture's. The design table only phrases them. The `--json` placement (here or `:98`) is P4's.
- design ↔ obs/CL-1 events: the web readback element's data dependency requires the CL-1 events to carry `send-issued` with `cursor` and `from`, `send-refused`, and an outcome record for `ok` / `confirmed:false` (unconfirmable). This is recorded for reconcile (per design-system §Surface: web-spa → Component Patterns 2, "Data dependency"). The web tape itself is out of scope, but the event payload this chunk lands is what it will read.
- design ↔ security-plan: the hint and refusal text floor (no upstream text, paths or pids) restates the NEVER-log floor and §Error Handling fixed messages.
- design ↔ tests: CARRY 3's ESC-bearing send must exit 13. Its human stderr then reads `[/ ] unable … not-delivered  control-character` plus the control-character hint.

## Acceptance criteria contributions
- A confirmed send prints exactly one `[RB] read back  <name>  <ts>  cursor <N>` line on stdout and exits 0. An unconfirmable local command prints `[  ] unconfirmable  <name>  local command, no measured post-condition` on stdout and exits 0. The `[  ] open` issue line appears only in TTY mode and is absent when piped (per design-system §Surface: cli → Component Patterns 2).
- Each `not-delivered` refusal (`control-character`, `input-not-ready`, `no-prompt-submitted`) prints `[/ ] unable  <name>  not-delivered  <detail>` on stderr, followed directly by that detail's fixed `hint:` line, and exits 13. Under `--json`, stdout holds one refusal document and no `hint:` line is printed (per design-system §Surface: cli → Component Patterns 2; Streams; Exit-code phraseology).
- `viola send` human output, both streams, in every mode (TTY, piped, `NO_COLOR`, `TERM=dumb`), contains no SGR/ESC byte and no non-ASCII byte (per design-system §Surface: cli → Tokens / Colour decision order; §Per-Surface Bans cli).
- No send result, refusal or hint line contains the sent prompt text, a filesystem path or a pid (per design-system §Per-Surface Bans cli).
