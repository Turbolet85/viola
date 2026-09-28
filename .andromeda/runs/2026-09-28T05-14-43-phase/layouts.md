# layouts extract

## Relevance
partial — the chunk adds no web-spa region and no human CLI output; the layout plan applies only through the cli surface's rule that `viola hook` has no human surface, the fixed `--help` verb table and the stream split, all of which the forced-panic case and the perf arm must leave as they are.

## Constraints
- layout-templates §Surface: cli / §Component — Header / banner requires that `viola hook` (and `viola mcp`) have no human surface: no banner, tagline, version line or other human-readable line, and this holds on the panic path too. So the forced-panic `viola hook` run must print nothing a human would read, on either stream. Whether the current panic hook already stays silent under the new seam is research's question.
- layout-templates §Output structure — `viola --help` fixes the grouped verb table (board / traffic / wheel / handoff / setup), and `hook` is not in it. The `FAKE_AGENT_HOOK_PANIC` seam and the perf arm must add no verb, no global flag and no help group to the product CLI. The seam is an env read that exists only under `fake-agent`, and `run --perf` is a harness flag, not a `viola` flag.
- layout-templates §Component — Primary navigation (verb structure) requires flat verbs and the same global flag set on every verb (`--home`, with `--json` as the machine view). Nothing in this chunk may add a perf- or panic-related flag to `viola` itself.
- layout-templates §Component — Primary content block 2 (Streams) requires that results go to stdout and diagnostics or refusals go to stderr, never mixed. For `viola hook`, stdout is kept for the decision body alone (CLAUDE.md invariant). On the forced panic, both streams stay empty: no body and no stderr line.
- layout-templates §Component — Footer / terminator: the exit code is the terminator, and there is no `done.` or summary banner. For the hook, exit 0 is the whole outcome, and no terminator text may be printed.

## Patterns to follow
- The "no human surface" pattern for machine-only verbs (layout-templates §Component — Header / banner): `viola hook` talks only through its exit code and an optional decision body on stdout. The panic seam follows the same silent fail-open shape as the head chunk's `tests/hook_fail_open.rs` cases.
- "Output as a contract" (layout-templates §IA notes): human columns and words are stable, and new fields go into `--json` first. This chunk changes no human output of any verb, and any new perf data goes only into harness artifacts (`perf-<hook>.json`), not into a `viola` verb's output.

## Anti-patterns to avoid
- Printing a panic message, backtrace or "fail-open" notice to stderr or stdout from `viola hook` (layout-templates §Component — Header / banner, "no human surface"). The payload and backtrace belong only in `detail-hook.ndjson`.
- Showing the seam or the perf arm in `viola --help` or in a `hint:` line (layout-templates §Output structure — `viola --help`; §Component — Primary navigation, Discoverability).

## Contract bindings
- layouts ↔ security/CLAUDE.md invariant: "no human surface" (layout-templates §Component — Header / banner) is the layout side of `viola hook always exits 0, never writes stderr, fails open with no body`. The forced-panic test asserts both at once.
- layouts ↔ tests harness: the `agent-run.* run --perf` output shape and `perf-*.json` are owned by test-plan §3 / §10, not by the layout plan, whose cli surface covers `viola` verbs only (layout-templates §Surface: cli, Primary screens).

## Acceptance criteria contributions
- (layouts) With `FAKE_AGENT_HOOK_PANIC` set on a `fake-agent` build, `viola hook` produces 0 bytes on stdout and 0 bytes on stderr (per layout-templates §Component — Header / banner, "`viola hook` / `viola mcp` have no human surface").
- (layouts) `viola --help` output is byte-identical before and after the chunk: the same five groups, and no `hook`, perf or panic entry (per layout-templates §Output structure — `viola --help`).
