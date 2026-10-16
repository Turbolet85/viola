group: architecture+test-plan+obs-plan
read: .claude/docs/commands.md · 95 · «- `scripts/agent-run.sh logs --instance <name> | jq -c 'sele»

- leaf: .claude/docs/commands.md:5   class: omission
  leaf-text: «`e2e-web/` (the Playwright browser suite) exists, its axe and a11y lint landing with the a11y chunks.»
  source: .andromeda/test-plan.md:1017 «stands until the route entry "Remove the browser leg" removes it; that entry's wrap amends this text.»
  replace-with: «`e2e-web/` (the Playwright browser suite) exists, its axe and a11y lint landing with the a11y chunks; the browser test leg (`e2e-web/`, `scripts/install-node.sh` and the Node pin, `npm ci`, `scripts/npm-audit.sh`, Playwright and Chromium, `run --browser` and the `pre-push` browser stage) stands until the route entry "Remove the browser leg" removes it.»

- leaf: .claude/docs/commands.md:19   class: stale
  leaf-text: «(e2e-web only until the route's frontend-toolchain entry, whose tsconfig puts the `crates/viola-ui/` frontend sources in the ts plane, founder ruling 2026-09-30)»
  source: .andromeda/registries/contracts/architecture/build-system.md:15 «No tsconfig, package manifest or bundler config ever covers `crates/viola-ui/`; a CI-runnable guard lists any such file.»
  replace-with: «(`e2e-web/` only; no tsconfig, package manifest or bundler config ever covers `crates/viola-ui/`, and a CI-runnable guard lists any such file)»

- leaf: .claude/docs/commands.md:25   class: retired
  leaf-text: «`viola ui [--port <n>]` — serve the view-only page on 127.0.0.1 (prints the launch line once to stderr)»
  source: .andromeda/architecture.md:301 «- Subcommands: `run`, `revive`, `send`, `wait`, `last`, `list`, `answer`, `hook <event>`, `mcp`, `verify`, `pause`, `release` (incl. `--budget`), `link`, `unlink`, `plugin install`.»
  replace-with: «no `viola ui` subcommand: the frontend is deferred to 0.2.0; its technology is not chosen»

- leaf: .claude/docs/commands.md:30   class: stale
  leaf-text: «`boot [--session <id>] [--instance <name>[:<args>]]... [--ui] [--unstamped] [--cli-version <ver>] [--agents-mode recorded|oversize|malformed] [--statusline-echo]` — build, create a home, stamp via `viola verify` against the fake agent, start wrappers (+ UI), wait for readiness»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:4 «Command body: `scripts/agent-run.sh boot [--session <id>] [--instance <name>[:<fake-agent-args>]]... [--unstamped] [--cli-version <ver>] [--agents-mode recorded|oversize|malformed] [--statusline-echo]`»
  replace-with: «`boot [--session <id>] [--instance <name>[:<args>]]... [--unstamped] [--cli-version <ver>] [--agents-mode recorded|oversize|malformed] [--statusline-echo]` — build, create a home, stamp via `viola verify` against the fake agent, start wrappers, wait for readiness»

- leaf: .claude/docs/commands.md:32   class: stale
  leaf-text: «`status [--session <id>]` — `viola list --json` + `/ready` + `/api/sessions`, with `api_sessions_equal_list`»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:68 «- `ui` and `api_sessions_equal_list`: `null`»
  replace-with: «`status [--session <id>]` — `viola list --json`; `ui` and `api_sessions_equal_list` are `null`»

- leaf: .claude/docs/commands.md:33   class: stale
  leaf-text: «graceful stop, endpoint/port/url-file checks, home removal (idempotent)»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:82 «`endpoint_gone` is reported from the wrapper channel on; `port_free` and `url_file_removed` stay `null`.»
  replace-with: «graceful stop, process and endpoint checks (`port_free` and `url_file_removed` stay `null`), home removal (idempotent)»

- leaf: .claude/docs/commands.md:35   class: stale
  leaf-text: «`ui-restart --session <id>`»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:101 «- **`ui-restart --session <id>`**: the shims forward it unchanged; `viola-harness` implements no such subcommand.»
  replace-with: «`ui-restart --session <id>` (forwarded unchanged; `viola-harness` implements no such subcommand)»

- leaf: .claude/docs/commands.md:36   class: stale
  leaf-text: «`cleanup` → `processes_gone`, `endpoint_gone`; port/url `null`.»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:82 «`endpoint_gone` is reported from the wrapper channel on; `port_free` and `url_file_removed` stay `null`.»
  replace-with: «`cleanup` → `processes_gone`, `endpoint_gone`; `port_free` and `url_file_removed` stay `null`.»

- leaf: .claude/docs/commands.md:40   class: stale
  leaf-text: «`& !binary(/^(path|tui|mcp|http|sse|cross|chaos|contract)_/)`»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:21 «then `kind(test) & !binary(/^(path|tui|mcp|cross|chaos|contract)_/)`»
  replace-with: «`& !binary(/^(path|tui|mcp|cross|chaos|contract)_/)`»

- leaf: .claude/docs/commands.md:41   class: stale
  leaf-text: «-E 'binary(/^(path|tui|mcp|http|sse|cross|chaos|contract)_/)'»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:22 «-E 'binary(/^(path|tui|mcp|cross|chaos|contract)_/)'»
  replace-with: «-E 'binary(/^(path|tui|mcp|cross|chaos|contract)_/)'»

- leaf: .claude/docs/commands.md:76   class: stale
  leaf-text: «per obs-plan §8 item 6.»
  source: .andromeda/obs-plan.md:918 «uploading `fuzz/artifacts/` on `failure()` only, see §8 item 5»
  replace-with: «per obs-plan §8 item 5.»
