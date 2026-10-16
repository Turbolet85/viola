# Codebase Research — 2026-10-16-remove-the-browser-leg

## Scope
- **Depth:** deep (removal across the harness, three workflows, two schemas, two obs modules) · **Reads:** 27 · **Globs/Greps:** 14
- **Harness rules consulted:** `.claude/rules/verification-harness.md` (read in full, its 17 Session Additions included: the pre-push launcher, `run --browser`, the `windows-mutants.yml` ten-item rule, the boot/status/cleanup document shapes) · `.claude/rules/ci.md` (in full, 6 additions: the operator pass order, the CI read through `ci.py`, the Windows-target lint before the push) · `.claude/rules/testing.md` (in full: remove-the-guard runs, literal oracles, no `#[ignore]` parking, the 2026-10-14 count-literal probe rule)
- **Platform issues consulted:** none — no runner-only bullet (Setup 5a read no red; HEAD `b6bbbd6` re-read green at P3) and no CI-reading entry outside the operator leg
- **External inputs:** none — the founder's ruling is the committed file `.andromeda/runs/2026-10-16T14-58-10-wrap/inputs/I1-restructure-cli-only.md.txt` (this repository); every other fact is in the tree

## Files inspected
- `.github/workflows/ci.yml` (full) — the workflow `env:` `NODE_PIN_*` block `:9-15`; the `test` job's browser pipe `:90-126` (Node (pinned), `npm ci --prefix e2e-web`, two `playwright install` steps, `run --browser` per shim); the `junit-<os>` upload's second path `:181`; `gate --require coverage,doctest,playwright` `:196`; the `supply-chain` job's `Node (pinned)` + `npm lockfile audit` steps `:451-466`. G2 `:133`, G4 `:137`, capture `:142`, secret-scan `:149` and the four scan-gated uploads `:155-191` sit between the browser pipe and the gate and do not depend on it.
- `.github/workflows/nightly.yml` (full) — `npm-advisories` job `:32-53`; `advisories` and `fuzz` untouched.
- `.github/workflows/windows-mutants.yml` (full) — the `viola-e2e` item `:51-53` lists three files, `browser.rs` the third; the item keeps two files after the cut, so the `- package:` item count stays ten.
- `tests/contract_windows_mutation_scope.rs` (`:1-200`) — derives the Windows-gated set from product sources (`cfg!(windows)` on a product line counts: `browser.rs:15` is why it is listed) and compares it with the yml; a listed file that no longer exists fails it, so the yml line must drop `browser.rs` in the same change.
- `crates/viola-e2e/src/harness/run.rs` (`:1-320`) — `mod browser` `:5`; `Selection::browser` `:43`; the doc comment `:55`; `test_suites` returns `(suites, refusal)` with the browser arm `:286-291`; `run_with` merges `browser_refusal` `:146-149`; `archive_sources` special-cases `playwright` `:173-189`; module doc `:1`; a test building a `playwright` suite `:772-777`.
- `crates/viola-e2e/src/harness/run/browser.rs` (`:1-160`) — the whole module (`NPM`, `CHROMIUM_PROBE`, `browser`, `playwright_suite`) and its tests to `:326`.
- `crates/viola-e2e/src/harness/gate.rs` (`:1-60`) — `SUITES: [&str; 9]` with `playwright` `:19`; `JUNIT: [&str; 5]` with `playwright` `:32`; test `gate_fails_failed_and_skipped_suites_playwright_included` `:272-283`.
- `crates/viola-e2e/src/bin/viola-harness.rs` (`:80-95`, `:168-180`) — `RunArgs::browser` `:90`, threaded into `Selection` `:175`.
- `crates/viola-e2e/src/harness/pre_push.rs` (full) — `CI_LINE` carries `NODE_PIN_VERSION` `:103`; `Fake::green` answers `node --version` `:167`, `run --browser` `:169` and the playwright gate `:171`; the green test iterates `["run", "browser", "gate"]` `:228` and orders `node --version` before `run --coverage` `:237`.
- `crates/viola-e2e/src/harness/pre_push/linux.rs` (full) — `NODE_BIN` `:17`; the launcher PATH `:45`; `node_pin` `:70-78`; the `tools` node check `:129`, `:149-154`; `linux_tests` runs coverage → browser → `gate --require coverage,doctest,playwright` `:195-219`; tests `pre_push_node_pin_reads_the_workflow_env_line` `:243`, `pre_push_node_missing_or_off_pin_is_named` `:261`, `pre_push_linux_browser_red_stops_before_the_gate` `:285`, `pre_push_linux_tests_run_coverage_browser_then_the_playwright_gate` `:304`, the `browser` absence assert `:420`, the launcher literal `:487-490`.
- `crates/viola-e2e/src/harness/status.rs` (full) — `"ui": null, "api_sessions_equal_list": null` `:26`; module doc `:1-2`.
- `crates/viola-e2e/src/harness/cleanup.rs` (`:55-80`) — `"port_free": null, "url_file_removed": null` `:70-71`; `SessionRecord` literals with `ui: None` / `cookie_file: None` `:275-278`, `:322-325`.
- `crates/viola-e2e/src/harness/mod.rs` (`:240-275`) — `SessionRecord { …, ui: Option<Value>, …, cookie_file: Option<PathBuf> }` `:252-263`, derive `Serialize, Deserialize`, no `deny_unknown_fields` (`git grep deny_unknown_fields -- crates/viola-e2e`: 0 hits), so an older `session.json` carrying both keys still parses after the fields go.
- `crates/viola-e2e/src/harness/boot.rs` (`:160-175`) — writes `ui: None` `:168`, `cookie_file: None` `:171`.
- `crates/viola-e2e/src/harness/secret_scan.rs` (`:1-210`, `:300-400`) — `CRITICAL` holds 13 rows, `token-query` (`?t=`) the 13th `:29`; `gui_tokens` `:181-198` / `url_token` `:200-207` feed the `gui-token` class `:131-135`; the `cookie` class has two forms: the generic `cookie:` header `:136-138` and the GUI cookie pair `viola_<port>=` (`cookie_pairs` `:169-178`). Tests that plant `?t=` only as a stock needle: `secret_scan_descends_into_the_capture_tree` `:357`, the two `chunk.diff` tests via `DIFF_LINE` `:372-391`, `secret_scan_marks_a_report_it_could_not_write` `:395`, and the case at `:475`.
- `crates/viola-e2e/tests/scan_patterns.rs` (`:40-80`) — asserts every canary planted under `tests/`/`src/` is in `CRITICAL ∪ CONTENT`; `?t=` is not a `canary-*` shape, so the table narrowing leaves it green.
- `crates/viola-e2e/src/harness/logs.rs` (`:245-282`) — `filters_match_named_values_and_treat_absent_as_null` plants a third line with `"process":"ui"` as an arbitrary non-matching value; `logs --process` takes a free string (`viola-harness.rs:58`), so this is a filter case, not a vocabulary pin.
- `crates/viola-core/src/obs.rs` (full) — `ObsEvent::{HttpRequest, LivenessChanged, SseOpened, SseClosed}` `:25-30`, `ALL: [Self; 19]` `:35`, `as_str` `:71-76`; `ObsProcess::Ui` `:94`, `ALL: [Self; 5]` `:99`, `as_str` `:106`; tests: `LOG_FORMAT_EVENTS: [&str; 19]` `:175`, the process literal `:210`, `set_ctx` second call uses `ObsProcess::Ui` `:223`.
- `src/obs.rs` (`:15-135`, `:370-410`) — doc comment naming axum `:21`; `VIOLA_TARGETS: [&str; 8]` with `viola_ui` `:30`; `role_file_name(process, instance, port)` `:54-68` (the `Ui` arm `:61` is the only one reading `port`); `open_role_file(home, process, instance, port)` `:75-87`; `viola_obs_init` passes `None` for the port `:135`; tests `#[case::ui]` / `#[case::ui_without_port]` / `#[case::run_without_name]` (port `Some(1)`) `:373-379`, `open_role_file(…, None, None)` `:397`, `would_enable("viola_ui::sse", …)` `:405`.
- `schemas/diag-line.v1.json` (full) — `process` enum with `ui` `:14`; message-const blocks for the four web events `:30`, `:32`, `:34`, `:35`; field blocks `http-request` `:84-87`, `liveness-changed` `:90-93`, `sse-*` `:97-100`; `process-start`'s `port` field `:72` (the `ui` listener's; no product `obs_event!` passes `port` — `git grep -n -E '"port"|port =' -- src/cmd src/run src/obs.rs` finds only a `config.json` test literal); `parse-rejected` parser value `last-event-id` `:102`; `$defs.event` `:125`; `$defs.liveness` `:130` (used only by `liveness-changed`).
- `schemas/diag-detail.v1.json` (`:1-30`) — the inlined event enum `:14` and the `process` enum with `ui` `:16`.
- `tests/contract_diag_schema.rs` (`:1-90`) — `LOG_FORMAT_EVENTS: [&str; 19]` `:20` pins both schemas' event enums.
- `scripts/agent-run.sh` / `scripts/agent-run.ps1` (full) — the header usage (`--browser` `sh:14`; `status` naming `/ready`, cookie-gated `/api/sessions`, `api_sessions_equal_list` `sh:16`; `boot` naming `viola ui` `sh:13`; `cleanup` naming port and url-file checks `sh:17`), `ui-restart` in the internal list (`sh:19`, `ps1:10`) and the dispatch (`sh:74`, `ps1:67`), the embedded-asset comments (`sh:37-41`, `ps1:29-32`).
- `.config/nextest.toml` (full) — `[test-groups] fixed-port` `:1-2` and its override `:4-7`.
- `deny.toml` (full), `deny-sync.toml` (full), `Cargo.toml` (`:190-197`) — the tokio-confinement wording `deny.toml:1`, `deny-sync.toml:12`, `Cargo.toml:194`; the `axum` `http2` feature ban `deny.toml:54-57`.
- `scripts/deny-probes.sh` (full) — 13 probes; `axum-http2` `:31`; its summary line `deny-probes: $banned/$total banned, control clean` `:83`.
- `.gitignore` (`:30-45`) — the `e2e-web` block `:38-44` (`node_modules/`, `*.tsbuildinfo`, four `e2e-web/` outputs).

## Graph impact (from the code-graph query; trace `tree-query-2026-10-16-remove-the-browser-leg.json`, plane rust, 38 rows, `db_state` fresh; `line` + 1 cited)
- **role_file_name** — refs at `src/obs.rs:81` (`open_role_file`) and `src/obs.rs:388` (its rstest) only — dropping its `port` parameter threads through no other file.
- **open_role_file** — refs at `src/obs.rs:135` (`viola_obs_init`, passes `None`) and `src/obs.rs:397` (test) only.
- **Ui** (`ObsProcess::Ui`, kind `type`) — `crates/viola-core/src/obs.rs:99`, `:106`, `:223` and `src/obs.rs:61`, `:377`, `:378` — no product emitter anywhere: every `viola_obs_init` caller (`src/cmd/client.rs:24`, `src/cmd/hook.rs:129`, `src/cmd/hook/statusline.rs:60`, `src/cmd/run.rs:181`, `src/cmd/verify.rs:64`, `git grep -n 'viola_obs_init('`) passes `Run` / `Hook` / `Cli`.
- **browser** — the fn `run/browser.rs` called at `run.rs:288`; the `Selection::browser` field read at `run.rs:287` and set at `viola-harness.rs:175`; the module's `JUNIT` const read at `run.rs:185`.
- **node_pin** — `pre_push/linux.rs:129` and its own test rows (`:246`-`:257`) only.
- **cookie_file** — `boot.rs:171`, `cleanup.rs:278`, `cleanup.rs:325` (all `None`), defined `mod.rs:262`.

## Patterns detected
- **A gate leaves with its subject** (`ci.yml:446-450` fuzz lockfile audit precedent): the npm lockfile audit's subject is `e2e-web/package-lock.json`, so the audit, its script and the lockfile go in one change.
- **Contract test keeps a list equal to its source** (`tests/contract_windows_mutation_scope.rs`): the yml line and the deleted file move together.
- **Closed lists pinned by literal oracles** (`crates/viola-core/src/obs.rs:175`, `tests/contract_diag_schema.rs:20`, `gate.rs:14`): each narrowed list has a literal twin in a test that narrows with it.
- **Runner seam** (`run.rs:34`, `pre_push.rs:129-175` `Fake`): harness tests script tool answers by substring, so removing a stage removes its scripted answer and the order asserts that named it.
- **Stock needle in scan tests** (`secret_scan.rs:357-395`): `?t=` stands in for "a critical hit" in five tests unrelated to the GUI; the `claude-stripped` canary is the same-class needle that survives.

## Conventions to follow
- **Literal oracles**: a narrowed list's test states the expected literal, never the product const (`.claude/rules/testing.md` §What to assert; `crates/viola-core/src/obs.rs:175`).
- **Counts carry their rule**: a changed array length (`[&str; 19]` → 15, `SUITES: [&str; 9]` → 8, `CRITICAL` 13 → 12, deny probes 13 → 12) is restated with the list that produces it (Session Learnings, CLAUDE.md).
- **No `#[ignore]` parking**: a browser- or `ui`-bound test is deleted with its code (`testing.md` §Determinism).
- **Workflow hygiene**: SHA-pinned `uses:`, `permissions: {}` / `contents: read`, gate steps `shell: bash` (`ci.yml` throughout).
- **rustfmt runs with every scripted `.rs` write** (handoff standing rule; MEMORY.md).

## New files to create
- none

## Files to modify
- `e2e-web/` — deleted whole (five tracked files)
- `scripts/install-node.sh` — deleted
- `scripts/npm-audit.sh` — deleted
- `.github/workflows/ci.yml` — the Node pin env block, the browser pipe, the playwright JUnit path, `playwright` in the gate list and the supply-chain Node and npm steps removed
- `.github/workflows/nightly.yml` — the `npm-advisories` job removed
- `.github/workflows/windows-mutants.yml` — `browser.rs` dropped from the `viola-e2e` item
- `.gitignore` — the `e2e-web` block removed
- `crates/viola-e2e/src/harness/run/browser.rs` — deleted
- `crates/viola-e2e/src/harness/run.rs` — the browser module, selector, refusal arm and playwright archive removed
- `crates/viola-e2e/src/harness/gate.rs` — `playwright` out of `SUITES` and `JUNIT`, its test retargeted
- `crates/viola-e2e/src/bin/viola-harness.rs` — the `--browser` flag removed
- `crates/viola-e2e/src/harness/pre_push.rs` — the Node pin, browser answer and browser section out of its test fakes and asserts
- `crates/viola-e2e/src/harness/pre_push/linux.rs` — `NODE_BIN`, `node_pin`, the node tool check and the browser stage removed, the PATH narrowed, tests updated
- `crates/viola-e2e/src/harness/status.rs` — the `ui` and `api_sessions_equal_list` members removed
- `crates/viola-e2e/src/harness/cleanup.rs` — the `port_free` and `url_file_removed` keys and the record literals' `ui` / `cookie_file` removed
- `crates/viola-e2e/src/harness/mod.rs` — `SessionRecord::ui` and `SessionRecord::cookie_file` removed
- `crates/viola-e2e/src/harness/boot.rs` — the record literal's `ui` / `cookie_file` removed
- `crates/viola-e2e/src/harness/secret_scan.rs` — the GUI classes removed per the P4 disposition, stock-needle tests moved to a surviving class
- `crates/viola-e2e/src/harness/logs.rs` — the filter case's arbitrary `ui` process value replaced by `mcp`
- `crates/viola-core/src/obs.rs` — the four web events and `ObsProcess::Ui` removed, its tests narrowed
- `src/obs.rs` — the `viola_ui` target, the `Ui` arm and the `port` parameter removed, its tests narrowed
- `schemas/diag-line.v1.json` — the `ui` process, the four web events' blocks, `process-start`'s `port`, the `last-event-id` parser value and `$defs.liveness` removed
- `schemas/diag-detail.v1.json` — the `ui` process and the four web events removed from its enums
- `tests/contract_diag_schema.rs` — `LOG_FORMAT_EVENTS` narrowed to fifteen
- `scripts/agent-run.sh` — `--browser`, `ui-restart` and the GUI usage and asset comments removed
- `scripts/agent-run.ps1` — `ui-restart` and the asset comment removed
- `.config/nextest.toml` — the `fixed-port` group and its override removed
- `deny.toml` — the tokio-confinement comment reworded, the `axum` `http2` ban removed per the P4 disposition
- `deny-sync.toml` — the tokio-confinement reason reworded
- `Cargo.toml` — the interprocess comment reworded
- `scripts/deny-probes.sh` — the `axum-http2` probe removed with its ban

## Sweeps
- `NODE_PIN|playwright|--browser|e2e-web|install-node|npm-audit|viola_ui|viola-ui` (case-insensitive) over the tree outside `.andromeda/`, `viola-0.1.0/`, `.claude/`: 20 files · 20 changed or deleted (`git grep -I -l -i -E …`, P1).
- `browser|ui-restart|stale-embedded-assets|fixed-port|http2|port_free|url_file_removed|api_sessions_equal_list|token-query|gui-token|HttpRequest|LivenessChanged|SseOpened|SseClosed|ObsProcess::Ui|"ui"|sse-|http-request|liveness-changed` over the same scope minus `e2e-web/`, `fixtures/`, `CLAUDE.md`: 125 hits · all in listed files except `refs/session-memory-options.md` (5) and `refs/viola-brief.md` (1) — no change (design history, out of scope).
- `-w last-event-id|LastEventId|port` over `src crates schemas tests`: 14 hits · 7 changed (`src/obs.rs` port param, `schemas/diag-line.v1.json:5,72,102`) · 7 no change (`secret_scan.rs` doc comments go with their functions; `src/obs.rs:445` and `tests/run_cli.rs:342` plant an unknown `config.json` key `port`, a tolerance test).
- `dejavu` (case-insensitive) over the tree outside `.andromeda/`, `viola-0.1.0/`, `.claude/`: 0 hits — no workflow or harness step installs a font package; design's font check reads "no site at base".
- `default_port` over `src crates tests`: 0 hits — the `fixed-port` group has no member.
- `viola-ui|viola_ui` outside the masters: 6 hits · 6 changed (`Cargo.toml:194`, `deny-sync.toml:12`, `deny.toml:1`, `scripts/agent-run.sh:39`, `src/obs.rs:30`, `:405`); no CI-runnable tsconfig guard over `crates/viola-ui/` exists (`git grep -n -i -E 'tsconfig|bundler' -- tests crates scripts .github`: only `scripts/code-graph.py`'s plane detection).
- `ui-restart|ui_restart|UiRestart`: 4 hits, all in the two shims; `viola-harness` defines no such subcommand, so the shims forward a verb clap refuses.
- `token-query|gui-token|gui_tokens` outside `secret_scan.rs`: 0 hits.
- `name = "axum"` in `Cargo.lock` and `fuzz/Cargo.lock`: 0 and 0 — the ban guards a crate no graph holds.
- `deny_unknown_fields` in `crates/viola-e2e`: 0 hits.

## Open questions
- Does each CARRY 3 leftover go, or stay with the masters amended to match (the entry leaves it to this chunk)? → blocks: plan-decision (P4: decisive leans where an artifact settles it; the secret-scan classes are asked).
