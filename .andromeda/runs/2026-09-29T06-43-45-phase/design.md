# design extract

## Relevance
partial — no web-spa or rendered surface; the chunk touches the cli surface only where the harness and the test homes consume `viola verify`'s human output and refusals (items 1, 2, 5, 7). The obs log lines (item 7), the error-enum fold and the pipe-name fix carry no design surface.

## Constraints
- design-system §Surface: cli → Component Patterns 5 (`verify`) requires static stdout step-counter lines, one per ledger row (`[NN/MM] <row> <description>  pass|fail`), then a last stdout summary line `stamped <ver>  <n> pass  <m> fail`, with the word `fail` uncoloured. Anything that decides "stamped" from the verb's human output (item 1's `stamped` flag, item 2's boot step 4 / `verify-failed`) reads that fixed shape and nothing else. Whether today's `verify` already prints exactly this shape is research's question.
- design-system §Surface: cli → Exit-code phraseology (row 1) requires a `verify` refusal to exit 1 with one fixed-message `unable: …` stderr line (CLI not found / `.cmd` or `.bat` script / version unreadable / recorded payload holds a path or username) followed by its own hint. A verify run with a failing ledger row prints NO stderr word and still exits through its summary line. The harness's `verify-failed` outcome has to tell apart "refused (exit 1 + `unable:`)" and "ran, m > 0 fail". How the harness maps each one is a plan decision.
- design-system §Surface: cli → Exit-code phraseology requires that `verify` has no `--json` document until arch amends it (row 1: "no `--json` document until arch amends it"). The harness must not invent or expect a `verify --json` shape in this chunk.
- design-system §Surface: cli → Colour decision order (step 6, "stdout is not a terminal") plus §Brand Identity's per-surface table (0.0 row) require output under the harness and under test capture (piped) to carry no SGR, no glyphs and no cursor control. The harness can therefore match the plain ASCII lines byte for byte.
- design-system §Surface: cli → Streams requires results and data on stdout (verify's step lines and summary) and refusals, `hint:` lines and `error:` on stderr. A harness or test that parses verify reads each class from its own stream and never merges them.
- design-system §Surface: cli → Platform-Specific Notes / Exit-code phraseology require that errors print a fixed message, never a stack trace, path, pid or anyhow chain. The `StampError` → `AgentError` fold (item 7) keeps whatever human text verify prints for a malformed stamp as a fixed message.
- design-system §Brand Identity ("Verified aircraft type") requires that only a CLI version stamped by `viola verify` reads as `verified`, and an unstamped one as `unverified-cli`. The explicitly unstamped test-home form (item 1 [inferred]) is what exercises the `unverified-cli` word and its hint `run viola verify for this CLI version` (§Surface: cli → Component Patterns 2).

## Patterns to follow
- The step counter is the project's one progress pattern (design-system §Surface: cli → Component Patterns 5): static appended lines. The harness surfaces verify progress by passing these lines through or by summarising them. It never adds its own spinner or live redraw.
- Standard phraseology (design-system §Brand Identity, "Standard phraseology and 'unable' plus a reason"): new typed outcomes the harness adds (`verify-failed`, `live-in-ci`, the `--unstamped` opt-out) are short lower-case hyphenated words in the same vocabulary as the existing refusal words.
- The summary line's version token (`stamped 2.1.283 …`, design-system §Surface: cli → Component Patterns 5) is the recorded version item 6 moves the fake agent and `boot --cli-version` to. The fake agent's default and the stamp the verb reports name the same version.

## Anti-patterns to avoid
- No green `✓` / red `✗` prefixes, emoji, spinner or progress bar in any human line verify or the harness prints (design-system §Anti-Patterns → Per-Surface Bans, cli).
- No bare "done" / "success": a stamp outcome says what was stamped, meaning the version and the pass/fail counts (design-system §Anti-Patterns → Per-Surface Bans, cli, "NEVER print 'done' or 'success' without context").
- No path, pid or upstream text in a refusal, hint or error line. This covers verify's refusals under a test home, whose absolute temp path must not surface (design-system §Anti-Patterns → Per-Surface Bans, cli; binds to security-plan §Error Handling).

## Contract bindings
- design ↔ tests harness: test-plan §3's `boot` step 4 / `verify-failed` / readiness contract consumes verify's summary line and its exit-1 `unable:` refusal shape (design-system §Surface: cli → Component Patterns 5 and Exit-code phraseology). If the harness parses stdout text rather than reading a typed exit, the line shape becomes a test-side contract, and any later change to it must move both together.
- design ↔ architecture: verify's `--json` document is "pending an arch amendment" (design-system §Surface: cli → Exit-code phraseology). If the plan wants a machine-readable stamp result for the harness, that goes through an arch amendment, not a design-only change.
- design ↔ security: the fixed-message / no-path rule for verify refusals and the folded `AgentError` Display binds to security-plan §Error Handling (design-system §Anti-Patterns → Per-Surface Bans, cli).

## Acceptance criteria contributions
- (design) Under the harness and test capture (non-TTY), `viola verify`'s stdout holds only step-counter lines plus a final `stamped <ver>  <n> pass  <m> fail` line, and neither stream carries an SGR escape byte (`\x1b`) (per design-system §Surface: cli → Component Patterns 5 / Colour decision order).
- (design) A verify refusal seen by the harness is exactly one `unable: …` stderr line plus one `hint:` line, exit 1, with no path, pid or stack trace. A verify run with a failing row prints no stderr word (per design-system §Surface: cli → Exit-code phraseology).
- (design) The fake agent's default CLI version, `boot --cli-version`'s default and the version printed on verify's `stamped` summary line are the same recorded version (per design-system §Surface: cli → Component Patterns 5).
