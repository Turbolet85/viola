# design extract

## Relevance
Partial, and only at the edges. This chunk has no web-spa surface and no human `viola` verb output. The only design rules that apply are the cli machine-output (0.0) rules and the no-stack-trace rule. They cover the harness's JSON output, the root `viola` bin's panic hook, and the CI render prerequisites that a later chunk will need.

## Constraints
- design-system §Brand Identity (per-surface expression table, row "cli, `--json` / non-TTY / …") sets machine output at expression 0.0: no colour, no glyphs, no cursor control, no spinner. `viola-harness` prints one JSON document for each of `boot · run · status · cleanup · logs`. That document is machine output of the same kind, so it should follow the 0.0 row. The plan does not name the harness itself; this is an application by analogy.
- design-system §Surface: cli → Streams, and §Anti-Patterns → Per-Surface Bans (cli) "NEVER mix data and messages": only results/data go to stdout. The harness's single `{"v":1,"cmd":…,"ok":…}` document must be the only stdout content. Any diagnostic or progress prose goes to stderr, or to `diagnostics/`.
- design-system §Surface: cli → Platform-Specific Notes ("Stack traces never print … full detail goes only to `…/diagnostics/`") and the cli ban "NEVER print stack traces". The panic hook installed as the first statement of `main` (scope item 6) must send panic detail to the `<home>/diagnostics/<role>.ndjson` sink, not to a backtrace on the terminal. Whether a panic currently prints a backtrace to stderr is research's question.
- design-system §Surface: cli → Width / Typography §cli ("Output is ASCII only"): all human-readable text in this chunk (stderr lines of `agent-run`, the fake agent's version answer) should be ASCII-only, with no emoji and no ✓/✗ prefixes.
- design-system §Typography "Assertions hold on the Linux fallback" (with §Surface: web-spa → Platform-Specific Notes, "Linux is the CI render") requires the ubuntu CI image to provide `fonts-dejavu-core` + `fonts-dejavu-extra` once headless GUI checks run. This chunk leaves `--browser` as grammar only (scope Boundaries), so the requirement is deferred, not an obligation now. The ubuntu leg of `ci.yml` should not be shaped in a way that blocks adding those packages later.

## Patterns to follow
- The typed-exit + single-JSON-document shape of design-system §Surface: cli → Exit-code phraseology (`{"v":1,"ok":{…}}`, and a typed `error`/`refusal` field instead of prose under JSON). The harness envelope `{"v":1,"cmd":…,"ok":…}` with typed `reason:"base-missing"` / `reason:"mutants-exit-<code>"` fits this convention. Machine causes are detail codes, never prose hints.
- Colour decision order, design-system §Surface: cli → Tokens: `--json` / non-TTY / `NO_COLOR` / `TERM=dumb` all mean no SGR. The harness is always consumed non-interactively (CI, agent shells), so it should emit no SGR regardless of terminal.
- "Say what happened, with context", design-system §Anti-Patterns → cli ban "NEVER print 'done' or 'success' without context". Any human-facing summary from `agent-run run` (suite selection, mutation verdict) should name what ran and its counts, not a bare "ok".

## Anti-patterns to avoid
- Spinners, progress bars or live-redrawing output during long harness steps such as `cargo mutants` or the nextest run (design-system §Anti-Patterns → cli "NEVER use a spinner, progress bar or live-redrawing dashboard").
- Colour, emoji or ✓/✗ glyphs in harness or CI-step output (design-system §Anti-Patterns → cli bans 1 and 3).
- Printing paths, pids or anyhow/serde error chains in human-facing error text (design-system §Anti-Patterns → cli "NEVER print upstream text, paths, pids or anyhow chains…"). This applies to any product-bin error in this chunk. The harness's own session JSON may carry paths as data, and that is test-plan's call.

## Contract bindings
- design ↔ obs: the no-stack-trace rule (design-system §Surface: cli → Platform-Specific Notes) binds to the obs-plan §3 panic-hook ordering. The panic hook is the first statement of `main` and writes to `diagnostics/<role>.ndjson`.
- design ↔ tests: the harness's stdout-is-one-JSON-document contract (test-plan §3) matches design's cli Streams rule. Parsing tests over the harness stdout are how that rule gets enforced.
- design ↔ tests/CI (deferred): the DejaVu font provisioning on the ubuntu CI leg (design-system §Typography) binds to the future headless GUI/browser suite, not this chunk.

## Acceptance criteria contributions
- Each `agent-run <command>` writes exactly one JSON document to stdout with no ANSI/SGR escape bytes, and any human-readable text goes to stderr (per design-system §Surface: cli → Streams).
- Harness and fake-agent human-readable output is ASCII-only, with no emoji or ✓/✗ prefixes (per design-system §Anti-Patterns → Per-Surface Bans, cli).
- A forced panic in the product process role prints no stack trace to the terminal. The detail lands in `<home>/diagnostics/<role>.ndjson` (per design-system §Surface: cli → Platform-Specific Notes).

## Relevant amendment history
(none) The sidecar `D:/dev/projects/viola/.andromeda/design-system-amendments.md` does not exist. Separately, design-system §Design Decisions Log has the Phase 4.5 review round 1 entry: fonts got per-OS fallback stacks because CI's headless GUI checks run on ubuntu. That entry sets the future requirement for DejaVu fonts on the ubuntu leg of the CI this chunk creates.
