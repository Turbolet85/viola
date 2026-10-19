# tests extract

## Relevance
partial — the chunk's two readings are live measurements of a real `claude` session, which test-plan places outside every test layer (local, `evidence/`, never CI); the domain binds the boundary of that measurement, the integration witness v1-42 already has, and the gates any forced edit must pass.

## Constraints
- A route entry's live readings of a real session are that chunk's own local measurement, kept in its `evidence/` with its own launcher, driver and readers: not a test layer, not harness surface, never in CI (per test-plan §6 E2E Test Strategy, "Skipped as untestable"). The precedence reading, the exit-1 reading and the trust-dialog rig therefore land as `viola-0.1.0/chunks/2026-10-18-plugin-precedence-live-readings/evidence/` records, not as nextest cases or `agent-run` suites.
- The real `claude` CLI and its CLI-native modals (the folder-trust dialog included) are an untestable zone in CI; they are reached locally only, through `viola verify` for the suites or a route entry's own live measurement (per test-plan §1 Test Scope Summary, Untestable zones).
- test-plan §11 Test Strategy bans running the real `claude` CLI, or `viola verify` against it, in CI; the suites reach it only through `agent-run run --local-live`, which refuses when `CI` is set, and a route entry's live measurement is local and never a CI step (per test-plan §11 Test Strategy).
- test-plan §11 Test Strategy bans reading a green fake-agent run as proof of real-CLI behaviour: v1-42's built integration half (`tests/cli_plugin_install.rs`, its cases and units named in test-plan §5 Integration Test Strategy) does not stand for the owed live half (which server or servers start, which answers `list`, the one `session-start`) (per test-plan §11 Test Strategy).
- test-plan §11 Test Strategy bans reading a Linux-only local run as proof for Windows or macOS: the 2.1.287 Linux dev host readings leave v1-42's Windows leg owed (per test-plan §11 Test Strategy).
- The readings are taken from the process logs `<home>/diagnostics/mcp.ndjson` and the event stream `instances/<name>/events.ndjson` in the shapes test-plan fixes: codes-only process-log lines with `timestamp`, `level`, `target`, a closed kebab-case `event`, `process` (`mcp` for the server), an absent `instance`/`corr` read the same as `null` (per test-plan §3 Log format).
- test-plan §10 requires zero flakiness: nextest `retries = 0`, a test that flakes once quarantined and fixed at the root cause within the chunk, never `#[ignore]`d; the carried `send_budget_paused_is_refused_with_the_window_word_and_no_cursor` case_4 watch is held as an observation by the operator's direction, and whether that holding squares with this budget on a recurrence is P4's to read (per test-plan §10 Zero-flakiness budget).

## Patterns to follow
- A live reading with its own launcher, driver and readers in the chunk's `evidence/`, as at chunk 2026-10-08-first-live-test-and-self-drive (per test-plan §6 E2E Test Strategy, "Skipped as untestable").
- Line readers over the ndjson streams in the `jq -c 'select(.event==…)'` form, one line per record, absent key equal to `null` (per test-plan §3 Log format, Agent parsing and Null encoding).
- A real-CLI `viola verify` the round needs (a re-stamp of a rig home) goes through `scripts/agent-run.sh run --local-live`, on the operator's host only, its `local-live` suite failure codes `build`, `verify-exit-<n>`, `verify-exit-none`, `row-missing` (per test-plan §3 5-command implementation, `run`).
- A rig home placed under `target/e2e-home/` on the Linux dev host sits on the operator-set tmpfs link `/tmp/viola-e2e-home-<uid>`; `test -L target/e2e-home && findmnt -n -o FSTYPE -T target/e2e-home/` reads the backing, and a home kept there does not survive a reboot (per test-plan §3 Test data bootstrap, Cleanup).

## Anti-patterns to avoid
- Folding a live reading into CI, a harness suite or a nextest case, or citing the fake-agent `tests/cli_plugin_install.rs` green as the live reading (per test-plan §11 Test Strategy).
- Any test the readings force taking its verdict from the rendered child screen (the trust dialog's rows included) rather than hook events, `events.ndjson`, channel payloads or the fake-agent receipt (per test-plan §11 E2E).
- Committing a rig `--home`, a token or a stripped `CLAUDE*` value into `evidence/` or a fixture (per test-plan §11 Test Data; per test-plan §3 Log format, Constraints).

## Contract bindings
- tests §3 Log format ↔ obs-plan §3 / §6: the `mcp.ndjson` and `events.ndjson` fields the readings read are tests §3's required fields, which obs-plan may extend but not rename; the event values a server start or exit writes are obs-plan's catalogue (which values `viola mcp` writes at its start and at a home-check refusal is research's question).
- tests §1 / §6 live-vs-CI split ↔ architecture CI/CD (Test-Relevant Conventions, Live vs CI split): the real CLI runs only locally.
- tests §3 Log format Constraints (no stripped `CLAUDE*` value in any line, the secret scan) ↔ security-plan NEVER-log floor, binding the `evidence/` records the round keeps.

## Acceptance criteria contributions
- No CI step, `agent-run` suite or nextest case starts the real `claude`; the precedence reading, the exit-1 reading and the trust-dialog method are recorded only under the chunk's `evidence/` (per test-plan §6 E2E Test Strategy; per test-plan §11 Test Strategy).
- `tests/cli_plugin_install.rs` and its named units stay green unchanged in the nextest gates on all three OSes, and v1-42's live half is claimed only from the recorded 2.1.287 reading, never from that suite (per test-plan §5 Integration Test Strategy; per test-plan §11 Test Strategy).
- v1-42's claim notes its Windows leg as owed on "Windows-only live measurements" rather than reading the Linux reading as covering it (per test-plan §11 Test Strategy).
- Any code or test edit the readings force passes the per-OS gates `--fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80` with nextest `retries = 0` (per test-plan §10 Quality Gates & Coverage Targets).
