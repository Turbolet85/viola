# layouts extract

## Relevance
partial — no web-spa surface is created or changed; the cli surface is touched only by §5 (P6, the `unbuilt_selectors_and_unknown_commands_are_usage` test over the verb set and usage output) and read, not changed, by §7 (P5 measures `viola run` passthrough). §§1–4, 6, 8, 9 are test/mutation/rule work outside layout.

## Constraints
- The verb set is flat: one lower-case word per verb, `plugin install` the only two-word verb, no nested noun hierarchy, no interactive prompt, TUI mode or pager. A P6 rewrite that enumerates "built" vs "unbuilt" selectors must take its verb list from this structure, not invent selectors (per layout-templates §Surface: cli · Component — Primary navigation (verb structure); §IA notes "Command model").
- `viola --help` is the grouped verb table (board / traffic / wheel / handoff / setup) and `viola <verb> --help` is clap's per-verb usage; a build-independent P6 must not assert a help layout that contradicts this grouping. Whether the current help output already matches the grouping is research's question (per layout-templates §Output structure — `viola --help`).
- Streams never mix: results to stdout; refusals, hints, errors, `waiting:` and the issue line to stderr. A usage/unknown-command assertion in P6 reads stderr for the error and expects nothing human-readable on stdout (per layout-templates §Component — Primary content block 2 "Streams").
- `viola run <name> -- claude` prints nothing while the child runs, and its exit-1 start refusals are `unable: <text>` + `hint:` with no paths and no pids. P5's measurement legs (`Stop-Process` on the wrapper, tab close) must not add wrapper output to the terminal to observe the child; observation goes through `Get-Process` / `claude agents --json`, outside the surface (per layout-templates §Output structure — `viola run`; §Component — Header / banner "No banner anywhere").
- The human columns and words are a stable contract LLM drivers may read; a test fix may tighten assertions on them but must not change them (per layout-templates §IA notes "Output as a contract").

## Patterns to follow
- Verb grouping as the single source of the verb inventory: board `list, ui` · traffic `send, wait, last, answer` · wheel `pause, release` · handoff `link, unlink` · setup `run, verify, plugin install` (per layout-templates §Output structure — `viola --help`). A build-independent "unknown command" probe uses a word outside this set.
- Refusal shape `unable  <name>  <reason>  <detail>` / exit-1 fixed-message `unable: <text>` followed by `hint:` as the last stderr line, with no hint for `internal error` / `wrapper fault` / `unknown` (per layout-templates §Component — Primary content block 2; §Component — Footer / terminator).
- Expression level 0.0 for non-TTY output: no colour, no non-ASCII glyph, no cursor control — test-captured output (piped) is this level (per layout-templates §Surface: cli "Expression level").

## Anti-patterns to avoid
- A test that pins "unbuilt" verbs to a usage error by name — the recurring 3-chunk red P6 names; per the flat-verb structure every listed verb is a planned verb, so its unbuilt state is transient (per layout-templates §Component — Primary navigation (verb structure)).
- Any hint that names `viola release`, a path or a pid, if a P6 or M2 fix touches refusal/hint text (per layout-templates §Component — Primary navigation "Discoverability"; §Component — Primary content block 2 "Hint line").

## Contract bindings
- cli usage/unknown-command exit code ↔ architecture §Conventions (exit codes) — the layout plan fixes streams and line forms but not the exit code of a clap usage error; P6's expected code is arch's, not this plan's.
- cli refusal line forms ↔ design-system cli pattern 2 (the cause list for exit-1 `unable:` lines, cited by layout-templates §Component — Primary content block 2).

## Acceptance criteria contributions
- (layouts) P6's rewritten test passes regardless of which verbs in the grouped set are built, and its unknown-command case uses a word outside board/traffic/wheel/handoff/setup (per layout-templates §Output structure — `viola --help`; §Component — Primary navigation (verb structure)).
- (layouts) P6's usage/unknown-command case asserts the error on stderr and no human result line on stdout (per layout-templates §Component — Primary content block 2 "Streams").
- (layouts) No human-facing cli line (list columns, refusal/hint words, `--help` grouping) changes in this chunk's diff; any change is a design change shown at P4 (per layout-templates §IA notes "Output as a contract").
