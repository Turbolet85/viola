# design extract

## Relevance
partial — no rendered (web-spa) surface is in scope; only the CLI surface's output contract touches §5 (P6's usage test) and §2 (M1's `refuse_stale` killing test), and M2 fixes must not alter it.

## Constraints
- design-system §Surface: cli §Exit-code phraseology requires exit 2 to be clap usage text (never produced by `viola hook`); P6's reworked `unbuilt_selectors_and_unknown_commands_are_usage` test must keep asserting the exit-2 / usage contract for unknown commands, whatever form it takes for selectors that are now built.
- design-system §Surface: cli §Navigation Pattern requires the flat one-word verb set (`run · send · wait · last · list · answer · verify · pause · release · link · unlink · ui · plugin install`); a build-independent P6 test derives "unbuilt" against that set — no verb renamed or added to make the test pass.
- design-system §Surface: cli §Component Patterns 2 (the `viola run` start-refusal list) requires the stale-heartbeat refusal to print exactly the fixed line `unable: <name> is still running but not answering` plus its own hint, with no path and no pid; a killing test for `refuse_stale` (M1) that observes output must observe this phraseology, not a new one. Whether `refuse_stale` already emits it is research's question.
- design-system §Surface: cli §Component Patterns 5 (`run`) requires `viola run` to print nothing once the child starts; any M2 fix to the run pump or readiness path adds no stdout/stderr line to a running wrapper.
- design-system §Surface: cli §Platform-Specific Notes requires `hook` to write nothing to stderr and always exit 0, and stack traces never to print; M2 fixes touching `hook_fail_open` keep that surface empty.
- design-system §Surface: cli §Toolkit requires clap without its `color` feature (plain usage in every mode); no dependency change in this chunk adds colour or a terminal-variable reader.

## Patterns to follow
- Assert CLI output by its fixed-message words and the typed exit code from the §Surface: cli exit-code table, never by a path, pid or free text (per design-system §Surface: cli §Component Patterns 2 / 5).
- Refusal lines go to stderr with one `hint:` line after; under `--json` no hint is printed (per design-system §Surface: cli §Streams) — a test that checks a refusal checks the stream it lands on.
- The verb list in §Navigation Pattern is the reference set for any build-independent "unbuilt verb" check (P6).

## Anti-patterns to avoid
- Never let a refusal or error line carry a path, pid, stack trace or upstream text to make a test observable (per design-system §Surface: cli §Component Patterns 5 and §Platform-Specific Notes).
- Never print from `viola run` after the child starts, or from `viola hook` to stderr, as a diagnostic aid while chasing M2 (per design-system §Surface: cli §Colour decision order items 2–3).

## Contract bindings
- design-system §Surface: cli exit-code table ↔ architecture.md §Conventions (typed exit codes): the P6 and `refuse_stale` assertions bind to both; the design table is phraseology, arch owns the codes.
- design-system §Surface: cli §Platform-Specific Notes (hook silent, exit 0) ↔ security-plan / CLAUDE.md invariant "`viola hook` always exits 0, never writes stderr" — the `hook_fail_open` binary in M2's red set tests this shared contract.

## Acceptance criteria contributions
- (design) After P6, an unknown command still exits 2 with clap usage text, and the test passes regardless of which verbs are built (per design-system §Surface: cli §Exit-code phraseology / §Navigation Pattern).
- (design) If M1's `refuse_stale` kill observes output, the asserted stderr is exactly `unable: <name> is still running but not answering` followed by its §Component Patterns 2 hint, with no path or pid (per design-system §Surface: cli §Component Patterns 2).
- (design) No diff in this chunk adds or changes a human-output string, SGR token or stream assignment in `src/` (per design-system §Surface: cli §Tokens / §Streams).
