group: architecture+test-plan+obs-plan
read: .claude/docs/commands.md · 89 · «- `scripts/agent-run.sh logs --instance <name> | jq -c 'sele»
read: CLAUDE.md · 127 · «<!-- USER:session-learnings end -->»

- leaf: .claude/docs/commands.md:27   class: stale
  leaf-text: «(`--statusline-echo` and `--agents-mode` are not built; both are owed to the route entry "End-to-end path binary and harness legs"; `--agents-mode` will write the fake agent's `agents-mode` file beside the session's `bin/claude[.exe]` copy, setting no env var)»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:4 «`--statusline-echo` (built at chunk 2026-10-17-end-to-end-path-binary-and-harness-legs; clap `conflicts_with` `--unstamped`»
  replace-with: «(both built: `--statusline-echo`, which conflicts with `--unstamped`, runs after the stamp and before the wrappers start, writing `<home>/statusline-source.json` (0600) whose command is `<bin dir>/viola-fake-agent[.exe] statusline-echo <home>/fake/statusline.marker`, for Path 6; `--agents-mode` (default `recorded`, an unknown value usage `arguments`) writes the fake agent's `agents-mode` file (0600) beside the session's `bin/claude[.exe]` copy after the stamp and before `supervise.json`, setting no env var, and is recorded as `agents_mode` in `session.json` and `supervise.json`)»

- leaf: .claude/docs/commands.md:33   class: stale
  leaf-text: «`boot [--session] [--instance <name>[:<args>]]... [--cli-version]`»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:4 «`--agents-mode recorded|oversize|malformed` (built at the same chunk; an unknown value is usage `arguments`)»
  replace-with: «`boot [--session] [--instance <name>[:<args>]]... [--unstamped] [--cli-version] [--agents-mode recorded|oversize|malformed] [--statusline-echo]`»

- leaf: .claude/docs/commands.md:33   class: stale
  leaf-text: «`run [--unit|--integration|--mutants|--coverage|--perf|--fuzz-replay|--all]»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:22 «suite `nextest-e2e`, run third after step 1's two nextest runs (`run --e2e`; in the default and `--all`), through the same nextest shape and target dir, a `--filter` intersecting it.»
  replace-with: «`run [--unit|--integration|--e2e|--mutants|--coverage|--perf|--fuzz-replay|--all]»

- leaf: .claude/docs/commands.md:33   class: stale
  leaf-text: «`status` → `list`/`pid`/`uptime_ms` `null` + `instances[]` (it runs no `viola list` yet: owed to "End-to-end path binary and harness legs")»
  source: .andromeda/registries/contracts/test-plan/status-endpoint-shape.md:17 «`list` is the parsed document, the exit-21 error document included; `pid` and `uptime_ms` stay `null`; `instances` keeps the pid + start-time `alive`.»
  replace-with: «`status` → `list` the parsed `viola list --json` document (run from `supervise.json`'s bin dir; `null` when it exits 0 with no JSON object), `pid`/`uptime_ms` `null`, `instances[]` keeping the pid + start-time `alive`, `last_error` `list-exit-<code>` or `list-exit-none`»

- leaf: .claude/docs/commands.md:37   class: stale
  leaf-text: «`cargo nextest run --workspace --features fake-agent --profile ci -E 'kind(test)'` — integration (gains `& !binary(/^(path|tui|mcp|cross|chaos|contract)_/)` once an E2E binary exists: nextest rejects an unmatched `binary()` regex)»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:21 «integration: `kind(test) & !binary(/^(path[0-9]*|tui|mcp|cross|chaos|contract)_/)`, the complement of step 2's layer, so a test binary reports in exactly one of the two.»
  replace-with: «`cargo nextest run --workspace --features fake-agent --profile ci -E 'kind(test) & !binary(/^(path[0-9]*|tui|mcp|cross|chaos|contract)_/)'` — integration (the complement of the E2E layer, so a test binary reports in exactly one; nextest rejects an unmatched `binary()` regex, and the root `chaos_`, `contract_`, `cross_`, `tui_` and `path<N>_` binaries match it)»

- leaf: .claude/docs/commands.md:38   class: stale
  leaf-text: «-E 'binary(/^(path|tui|mcp|cross|chaos|contract)_/)'` — fake-agent E2E»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:22 «`[0-9]*` after `path` admits the numbered Path binaries (`path1_run_start`, `path6_budget_harness`), which the written `^(path|…)_` did not select»
  replace-with: «-E 'binary(/^(path[0-9]*|tui|mcp|cross|chaos|contract)_/)'` — fake-agent E2E (`run --e2e`, suite `nextest-e2e`, run third after unit and integration in the default and `--all`; `[0-9]*` admits the numbered Path binaries `path1_run_start` and `path6_budget_harness`; `run --e2e` alone runs no doctest)»
