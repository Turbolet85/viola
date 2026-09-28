# design extract

## Relevance
partial: only the cli surface applies (the `viola verify` human output, the catch-site `error: internal error` line, `run`'s silence once the child starts). No web-spa token, typography, spacing, radius, motion or iconography applies, because the readers of `verified` / `unverified-cli` (`viola list`, `viola ui`) are Epochs 5 and 8.

## Constraints
- design-system §Surface: cli → Component Patterns 5 (`verify`) requires `verify` human output to use the step-counter pattern, the project's only progress pattern. Each probe gets one static appended line of the form `[NN/TT] <row-id> <row words>  pass|fail`, and one summary line `stamped <ver>  <n> pass  <m> fail` follows. The word `fail` is uncoloured. The sample's `14` is illustrative: TT is the size of the compiled-in row set. The summary must match test-plan :290/:518 (`stamped <ver>  <n> pass  0 fail`, fields separated by two spaces).
- design-system §Surface: cli → Tokens (Colour decision order) and §Per-Surface Bans (cli) allow SGR only on `DIALOG` (amber) and `stale` rows (dim). `verify` output, including the `fail` word, the summary and every error, therefore carries no SGR, glyph or cursor control at any depth. Under `--json` the human lines are replaced entirely by the one JSON document on stdout.
- design-system §Surface: cli → Streams puts results and data on stdout and errors (`error: …`) and refusals on stderr. The summary `stamped …` line is a result, so it goes to stdout. The plan does not say which stream the per-row step lines use. Flag this for P4 so it is decided rather than assumed.
- design-system §Surface: cli → Exit-code phraseology (exit 1 row) and §Component Patterns 2 (the last hint bullet) require `verify`'s catch site to print exactly `error: internal error`. It is a fixed message on stderr, with no hint line, no chain, no path, and no `--json` document for exit 1 until arch amends it. §Platform-Specific Notes also says stack traces never print. Whether `src/human.rs`'s single writer already serves a non-`run` verb is research's question.
- design-system §Per-Surface Bans (cli) forbids printing upstream text, paths, pids or anyhow chains in errors or hints. A failing probe line must therefore name only the row id and its fixed row words plus `fail`. It must never name the `claude` CLI's output, the resolved executable path, the home path or a measured payload.
- design-system §Surface: cli → Width and §Per-Surface Bans (cli) require human output to be ASCII only, never wrapped and never hardcoded to a width. Step lines and the summary are single lines of ASCII words.
- design-system §Surface: cli → Component Patterns 5 (`run`) and Colour decision order item 2 require `viola run` to print nothing once the child starts. The new `--version` pre-spawn probe and the `cli_verified` stamps read must add no human line to `run`'s stdout or stderr on the success path. An unlisted or unverified version degrades silently to transport-only and is not a start refusal.

## Patterns to follow
- The `src/human.rs` one-writer pattern (the prior chunk's five `run` start refusals, one `write_all`) is the route for the catch-site line (scope CARRY 3). Whether it generalises unchanged to `verify` is research's question.
- design-system §Surface: cli → Navigation Pattern: `verify` is a flat, lower-case, single-word verb, grouped in `viola --help` under `setup: run, verify, plugin install`. Whether help grouping exists in the code yet is research's question. If it does not, the new verb must not contradict the grouping.
- design-system §Brand Identity (the "Verified aircraft type" line) and §Color Palette → Semantic Colors (`CliVerified::true|false` rows) set the vocabulary for the snapshot's `cli_verified`: the only human words are `<version> verified` and `<version> unverified-cli`. The snapshot field this chunk sets is what those later readers print. No new word (`ok`, `trusted`, `stamped` in a table) is coined for it.
- design-system §Anti-Patterns → Per-Surface Bans (cli), "NEVER print done/success without context": the summary always names the CLI version and both counts, never a bare `ok` / `done`.

## Anti-patterns to avoid
- design-system §Per-Surface Bans (cli): no spinner, progress bar, live-redrawing line or `\r` rewrite for the probe run. Step lines are appended and static, because a spinner presumes progress before it is confirmed.
- design-system §Per-Surface Bans (cli): no green `✓` / red `✗` prefixes, emoji or colour on `pass` / `fail`. The words carry the state.
- design-system §Per-Surface Bans (cli) and §Platform-Specific Notes: no stack trace and no anyhow chain on stderr. Detail goes only to `diagnostics/`.

## Contract bindings
- design ↔ test-plan: the `stamped <ver>  <n> pass  0 fail` summary (test-plan :290, :518) is a line the harness and CI fake-agent verify may parse, so its spacing and wording are one shared contract (design-system §Surface: cli → Component Patterns 5).
- design ↔ obs-plan §7: the catch-site line `error: internal error` is fixed by both plans. The chain goes only to the detail file (design-system §Surface: cli → Exit-code phraseology).
- design ↔ security-plan (NEVER-log floor, error sanitization): the cli ban on paths, upstream text and chains in human output enforces the same floor on `verify`'s stderr and step lines (design-system §Per-Surface Bans (cli)).
- design ↔ architecture (snapshot `cli_verified`, exit codes): the boolean this chunk sets feeds the later CLI column words `verified` / `unverified-cli` and the exit-12 `unable  unverified-cli` row, with the hint `run viola verify for this CLI version` (design-system §Surface: cli → Component Patterns 2 and Exit-code phraseology). The driver-verb refusal itself is out of scope (Epoch 3).
- design ↔ a11y: none new. Plain uncoloured words already satisfy not-colour-alone.

## Acceptance criteria contributions
- (design) `viola verify` against the fake agent prints one static `[NN/TT] <row-id> …  pass|fail` line per compiled-in row and ends with `stamped <ver>  <n> pass  0 fail` on stdout. TT equals the row count, and the output has no `\r` and no SGR/ESC byte (per design-system §Surface: cli → Component Patterns 5, §Per-Surface Bans (cli)).
- (design) An internal fault in `viola verify` exits 1 and writes exactly `error: internal error` plus a newline to stderr: no hint line, no path, no chain, no ESC byte (per design-system §Surface: cli → Exit-code phraseology).
- (design) All `viola verify` human output is ASCII only and holds no absolute path, username, token or upstream CLI text (per design-system §Surface: cli → Width, §Per-Surface Bans (cli)).
- (design) `viola run` with an unverified or unparseable child version writes no human line to stdout or stderr after the spawn, and the degrade is not reported as a start refusal (per design-system §Surface: cli → Component Patterns 5 (`run`), Tokens → Colour decision order).
