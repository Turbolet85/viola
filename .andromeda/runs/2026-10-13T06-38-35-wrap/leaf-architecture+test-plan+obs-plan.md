group: architecture+test-plan+obs-plan
read: .claude/docs/commands.md · 95 · «- `scripts/agent-run.sh logs --instance <name> | jq -c 'sele»

- leaf: .claude/docs/commands.md:5   class: tree
  leaf-text: «`e2e-web/` lands with a later chunk»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:103 «e2e-web/                    # test-side Node only (Playwright; axe and the a11y lint land with the a11y chunks);»
  replace-with: «`e2e-web/` (the Playwright browser suite) exists, its axe and a11y lint landing with the a11y chunks»

- leaf: .claude/docs/commands.md:15   class: stale
  leaf-text: «local optional, on the host's own Node»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:156 «then `node --version` = `v` + ci.yml's workflow `NODE_PIN_VERSION`) · `linux-tests` (`run --coverage`, then `run --browser` (a red stops before the gate)»
  replace-with: «locally the Linux `pre-push` runs it under ci.yml's pinned Node at `~/.local/viola-node`»

- leaf: .claude/docs/commands.md:19   class: stale
  leaf-text: «(e2e-web only; never over `crates/viola-ui/`)»
  source: .andromeda/registries/contracts/architecture/build-system.md:15 «By founder ruling (2026-09-30) the page becomes a React + TypeScript bundle at the route's frontend-toolchain entry (Epoch 8's head), whose tsconfig puts the frontend sources in this ts plane»
  replace-with: «(e2e-web only until the route's frontend-toolchain entry, whose tsconfig puts the `crates/viola-ui/` frontend sources in the ts plane, founder ruling 2026-09-30)»

- leaf: .claude/docs/commands.md:31   class: stale
  leaf-text: «No selector or `--all` runs unit + integration»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:24 «With no selector and no `--local-live` the default is `--all`, which runs steps 1 and 2 in order»
  replace-with: «No selector (and no `--local-live`) or `--all` runs unit + integration, then the fake-agent E2E (steps 1 and 2)»

- leaf: .claude/docs/commands.md:36   class: stale
  leaf-text: «`run [--unit|--integration|--mutants|--coverage|--browser|--perf|--fuzz-replay|--all] [--filter] [--file <path>]... [--package <member>]`»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:53 «`--local-live` appends `viola verify` against the real `claude`, on the operator's host only.»
  replace-with: «`run [--unit|--integration|--mutants|--coverage|--browser|--perf|--fuzz-replay|--all] [--filter] [--file <path>]... [--package <member>] [--local-live]` (`--local-live`: after the selected suites, one `viola verify` against the real `claude` as suite `local-live`; under `CI` exit 2 `live-in-ci`)»

- leaf: .claude/docs/commands.md:36   class: omission
  leaf-text: «`status` → `list`/`ui` `null` + `instances[]`»
  source: .andromeda/registries/contracts/test-plan/status-endpoint-shape.md:19 «`list`, `ui`, `pid`, `uptime_ms` and `api_sessions_equal_list` are `null`»
  replace-with: «`status` → `list`/`ui`/`pid`/`uptime_ms`/`api_sessions_equal_list` `null` + `instances[]` (it runs no `viola list` yet: owed to "End-to-end path binary and harness legs")»

- leaf: .claude/docs/commands.md:36   class: stale
  leaf-text: «`cleanup` → `processes_gone`, endpoint/port/url `null`»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:92 «`endpoint_gone` is reported from the wrapper channel on; `port_free` and `url_file_removed` are `null` until their surfaces exist.»
  replace-with: «`cleanup` → `processes_gone`, `endpoint_gone`; port/url `null`»

- leaf: .claude/docs/commands.md:45   class: stale
  leaf-text: «a Rust delta deletes a stale `mutants.out/outcomes.json` first and reports `"verdict":"counted"`»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:46 «A diff whose `.rs` paths are ALL test targets (`tests/`, `benches/` or `examples/` at the root or under `crates/<member>/`) never reaches cargo-mutants either»
  replace-with: «one whose `.rs` paths are all test targets (`tests/`, `benches/`, `examples/`) skips it too and reports `"verdict":"test-only-rust-delta"`; any other Rust delta deletes a stale `mutants.out/outcomes.json` first and reports `"verdict":"counted"`»

- leaf: .claude/docs/commands.md:64   class: omission
  leaf-text: «exit 0 only when P equals N and K and S are 0»
  source: .andromeda/architecture.md:439 «it exits 0 only when every run passed, P equals N and K and S are 0, 1 on any other count, and 2 on a usage error or with no instrumented binary»
  replace-with: «exit 0 only when every run passed, P equals N and K and S are 0»
