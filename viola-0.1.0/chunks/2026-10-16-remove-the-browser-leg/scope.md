# Scope — 2026-10-16-remove-the-browser-leg · Remove the browser leg

**Working entry** (`viola-0.1.0/working-route.md`, the first markerless entry, eighth under
`### Epoch 5 — Driver surface`, directly after "Resume and compact ledger rows"): Remove the browser leg — the browser
test leg leaves 0.1.0: every gate green with no Node on the host, no browser step in any workflow, pre-push or
harness — plus three CARRY blocks (folded below; completeness check `route.py pins --run-dir` → this run dir's trail:
three rows on the entry, 1278 · 530 · 1221 chars, no abstention).

**The operator's direction at take-up:** none beyond the invocation (`/andromeda-phase`, no flags). No external input
is snapshotted at P1: the founder's ruling this entry rests on is already a committed file of this repository,
`.andromeda/runs/2026-10-16T14-58-10-wrap/inputs/I1-restructure-cli-only.md.txt` (that wrap's snapshot I1, item 3),
re-read at P1 and matching CARRY 1. This chunk's own `inputs#I1` is the P4 forks' answer (snapped at P4).

**CI read at Setup 5a** (the last wrap's flip `3fc2e61` through HEAD `b6bbbd6`, `ci.py conclusion`, this run dir's
trail `ci-no-marker.json`):
- `b6bbbd68428a`: **in progress** — `CI b6bbbd68428a: verdict not yet available`, checks 15/15 listed, run
  `ci#38925218343` open. Not folded and not read as green at Setup. Re-read at P3 (`ci.py conclusion`): **green**,
  checks 15/15, wall 561 s, `ci#38925218343` completed/success.
- `3fc2e61c79a3`: **green**, checks 15/15, wall 507 s, `ci#38919168159` completed/success.
- No red or `not green` to disposition.

## What this chunk builds

The removal of every browser-test and GUI-harness site the 0.1.0 tree still carries, so that 0.1.0 is a pure CLI in its
test infrastructure as the founder ruled on 2026-10-16. Nothing in the product's behaviour changes. Every site below was
re-found at P1 by a `git grep` over the tree at `b6bbbd6` (the line numbers are P1's reading, hypotheses for P3).

### 1. The browser leg itself (CARRY 1: the founder's ruling, his chosen option "the browser infrastructure is removed by a 0.1.0 chunk")
- `e2e-web/` deleted whole: `package.json`, `package-lock.json`, `playwright.config.ts`, `tsconfig.json`,
  `tests/pipe-reachability.spec.ts`. Its `tsconfig.json` is the only tracked one (`git ls-files '*tsconfig.json'`, P1),
  so after it goes nothing feeds the ts code-graph plane.
- `.github/workflows/ci.yml`:
  - the workflow `env:` Node pin (`NODE_PIN_VERSION` and the three `NODE_PIN_SHA256_*`, `:9-15`);
  - the `test` job's browser pipe (`:90-126`: Node (pinned), `npm ci --prefix e2e-web`, the two
    `playwright install` steps, `agent-run.* run --browser`);
  - the `junit-playwright.xml` artifact (`:181`) and `playwright` in `gate --require coverage,doctest,playwright` (`:196`);
  - the `supply-chain` job's Node (pinned) and npm lockfile audit steps (`:451-466`).
- `.github/workflows/nightly.yml`: the `npm-advisories` job (`:32-53`).
- `.github/workflows/windows-mutants.yml:53`: `crates/viola-e2e/src/harness/run/browser.rs` in the mutants file list.
- The harness's `run --browser`:
  - `crates/viola-e2e/src/harness/run/browser.rs` (the whole module and its tests);
  - its callers in `run.rs` (`mod browser`, `Selection::browser`, the `browser-missing` refusal, the junit source,
    the `playwright` archive);
  - `gate.rs` (the `playwright` suite name, `:19`, `:32`, and its test);
  - `scripts/agent-run.sh` (`--browser` in the `run` usage, `:14`).
- `crates/viola-e2e/src/bin/viola-harness.rs:90,175` (the `browser` flag). The entry did not name it; the sweep found
  it (verified at P3: the graph's only setter of `Selection::browser`).
- The pre-push's Node pin check and browser stage:
  - `crates/viola-e2e/src/harness/pre_push.rs` (`CI_LINE`'s `NODE_PIN_VERSION`, `node --version`, the `browser`
    section, `:103`, `:167-237`);
  - `pre_push/linux.rs` (`NODE_BIN`, `node_pin`, the node tool check, the `run --browser` stage, and their tests).
- The pre-push's child `PATH` (`linux.rs:45`, `:488`) loses its `<home>/.local/viola-node/bin` element (verified at P3).
  That is a narrowing of the `/usr/bin/env -i` launch that `.claude/rules/security.md` (Dependencies and CI) states
  exactly, so the rule and its security-plan source amend to match.
- `scripts/install-node.sh` and `scripts/npm-audit.sh` deleted.
- `.gitignore:38-44` (the `e2e-web` block: `node_modules/`, `*.tsbuildinfo` and four `e2e-web/` outputs). The entry
  did not name it (verified at P3).
- The `viola-ui` leftovers (no `crates/viola-ui` exists):
  - the tokio-confinement wording in `deny.toml:1`, `deny-sync.toml:12` and `Cargo.toml:194`;
  - the `viola_ui` target in `src/obs.rs`'s filter (`:30`) and its test (`:405`).

### 2. The masters follow the code (CARRY 2: the overseer's answer at the 2026-10-16T14-58-10 wrap, technical, founder-delegated)
- The masters stay true to the tree. Their browser-test-infrastructure text stood at that wrap, marked as owned by
  this entry, and **this entry's wrap amends it when the code goes**:
  - the Node pin, npm and the npm audit;
  - `e2e-web/` and Playwright;
  - the CI browser job and the pre-push browser stage;
  - the ts code-graph plane.
- The ledger notes on v1-08 (the ts plane for `e2e-web/`) and v1-23 (`e2e-web/` in the architecture's tree), dated that
  day, name the same ruling.
- The amendments are the wrap's (phase never amends a spec source; verified against the skill's own rule). This chunk's plan names them as
  Expected amendments, and its implement writes none. CLAUDE.md's key-directory bullet for `e2e-web/` and the
  leaves that state the Node pin (`.claude/rules/security.md`, `.claude/rules/ci.md`) follow by the wrap's cascade.

### 3. The GUI harness and gate leftovers (CARRY 3: measured in the tree at `3fc2e61`, kept in the masters by the same rule)
Each is this entry's to remove, or to keep with the masters amended to match:
- `crates/viola-e2e/src/harness/status.rs:26`: the `null` `ui` and `api_sessions_equal_list` members.
- `cleanup.rs:70-71`: the `null` `port_free` and `url_file_removed` keys.
- `secret_scan.rs`: the `token-query` and `gui-token` classes (security-plan Secret Management, obs-plan §9;
  `:29`, `:133`, `:190`, and their tests).
- `scripts/agent-run.sh` / `agent-run.ps1`: the `ui-restart` forwarding (`sh:19`, `:74`; `ps1:10`, `:67`) and the
  `stale-embedded-assets` comment (`sh:40`; `ps1:31`).
  - The `status` usage comment (`sh:16`) naming `/ready`, the cookie-gated `/api/sessions` and
    `api_sessions_equal_list` belongs to the same leftover, and so do `boot`'s `+ viola ui` (`sh:13`) and `cleanup`'s
    "port / url-file checks" (`sh:17`) (verified at P3). `viola-harness` defines no `ui-restart` subcommand
    (`git grep -E 'ui-restart|ui_restart|UiRestart'`: the two shims only), so the shims forward a verb clap refuses.
- `.config/nextest.toml`: the `fixed-port` group (`:2`, `:7`).
- `deny.toml:56`: the `axum` `http2` feature ban.
  - `axum` is in no lockfile (`name = "axum"` in `Cargo.lock` and `fuzz/Cargo.lock`: 0 and 0, P3), so the ban
    guards a crate the tree never builds (verified).
  - Its probe `scripts/deny-probes.sh:31` (`axum-http2`) goes with it, or both stay (verified: the probe builds its
    own throwaway project, so it fires whether or not axum is in the graph).
- The closed obs vocabulary's web half (the operator's second answer at that wrap, overseer, technical,
  founder-delegated: true to the tree, marked reserved with no emitter in 0.1.0):
  - `crates/viola-core/src/obs.rs`'s `ObsEvent::{HttpRequest, LivenessChanged, SseOpened, SseClosed}` and
    `ObsProcess::Ui`;
  - `schemas/diag-line.v1.json` / `diag-detail.v1.json`'s `ui` process value and their `http-request`,
    `liveness-changed` and `sse-*` blocks;
  - all of these removed together with the obs-plan catalogue rows and the bound test-plan / obs-plan §3 Log-format
    text, in one pass.
  - The readers that pin the vocabulary by shape go with it (verified at P3):
    - `crates/viola-core/src/obs.rs:175-223` (the names list, the `["run","hook","mcp","ui","cli"]` assertion, a
      `ObsProcess::Ui` case);
    - `src/obs.rs:61` (the `ui-{port}` file name) and its `#[case::ui]` / `#[case::ui_without_port]` rows (`:377-378`);
      the `port` parameter of `role_file_name` / `open_role_file` is read only by that arm (graph: callers in
      `src/obs.rs` alone), and `targets_enable_viola_crates_at_the_level_and_nothing_else` asserts `viola_ui::sse`
      (`:405`);
    - `tests/contract_diag_schema.rs:20-40`.
  - `[premise-corrected: logs.rs:262 plants "process":"ui" as an arbitrary non-matching value for a free-string
    filter (viola-harness.rs:58), not a vocabulary pin]` `crates/viola-e2e/src/harness/logs.rs`'s filter case uses
    `ui` only as a third process name (`:262`, `:277`); it changes to `mcp` so no test names the removed role.
  - 0.1.0 has no `ui` emitter (verified at P3: every `viola_obs_init` caller passes `Run`, `Hook` or `Cli`, and
    `viola_obs_init` passes no port), so no logged line on any home carries these kinds. Even if one did, a reader
    skips and counts an unknown kind (Session Learnings: never `deny_unknown_fields`), so narrowing the closed set
    strands no reader.
  - `[added at P3: schemas/diag-line.v1.json:72,102,130; obs history 2026-10-16T14-58-10-wrap kept them reserved]`
    The schema's other web-only values go in the same pass: `process-start`'s `port` field (the `ui` listener's;
    no product `obs_event!` passes `port`), the `parse-rejected` parser value `last-event-id` (the SSE resume
    header's), and `$defs.liveness` (read only by `liveness-changed`).

### 4. Leftovers the entry did not name (found at P3)
- `[added at P3: crates/viola-e2e/src/harness/mod.rs:252-263, boot.rs:168,171, cleanup.rs:275-278,322-325; tests
  history 2026-10-16T14-58-10-wrap names cookie_file among the kept leftovers]` `SessionRecord`'s `ui: Option<Value>`
  and `cookie_file: Option<PathBuf>` fields, always `None`. No `deny_unknown_fields` in `viola-e2e`, so an older
  `session.json` carrying both keys still parses after they go.
- `[added at P3: secret_scan.rs:136-140,169-178; tests and security histories name the cookie class among the kept
  leftovers]` The `cookie` scan class has two forms: the generic `cookie:` header and the GUI cookie pair
  `viola_<port>=`. The entry named only `token-query` and `gui-token`. Its disposition is P4's.
- `[added at P3: src/obs.rs:21]` `src/obs.rs`'s doc comment naming `axum` among the OFF targets.

## Boundaries
- **Out:** the product's behaviour. No CLI verb, wire contract, event line or refusal changes. The only obs change is
  the reserved web half above, which has no emitter.
- **Out:** `refs/` (the founder's brief and options notes): they mention a browser as design history, not
  infrastructure.
- **Out:** `.andromeda/cache/ts/` and the code-graph tool: Andromeda's, not this repository's code. The plane drops by
  its own detection once no `tsconfig.json` is tracked.
- **Out:** the 0.2.0 frontend: its technology is not chosen. Nothing here pre-builds or reserves any of it beyond what
  CARRY 3 keeps.
- **Out:** the web-only leaves named for the founder at that wrap (`.claude/rules/frontend.md`, `.claude/rules/a11y.md`,
  `.claude/docs/services/viola-ui.md`). They are his batch, not this entry's.

## Acceptance anchor (the entry's own words)
- Every gate green with **no Node on the host**.
- **No browser step** in any workflow, the pre-push or the harness.
- "No Node on the host" is read as: no gate, workflow job, pre-push stage or harness verb runs, installs or checks for
  `node` / `npm` / `npx`. It is witnessed by a search of the tree that cannot pass vacuously (it must first find the
  sites above at the base — verified at P3: the P1 sweep reads 20 files at `b6bbbd6`), not by uninstalling Node
  from the dev host.
