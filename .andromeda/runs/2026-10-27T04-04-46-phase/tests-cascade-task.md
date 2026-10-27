# Task — tests · 2026-10-27-bounded-inputs-at-every-boundary

## The chunk
Makes every input that enters a `viola` process bounded and validated where it enters: a size cap (`Read::take(MAX_FRAME)` or the `MAX_FRAME + 1` over-size form) before every external reader buffers, serde_json at its default depth with no `unbounded_depth`, closed enums or validated newtypes for every externally read decision, and `ViolaName::try_new` (or a closed-shape validator) before every path join.
A reader found without a bound is bounded; the chunk adds a unit-level witness per property and per boundary, plus an enumeration of the external readers, each with its cap, depth and type rule, so that a new reader with no bound shows up.
It re-verifies the paste rule (`validate_paste_text`, `control-character` / `empty-text`, never a strip) at every client (CLI `send`, MCP `send`) and authoritatively in the wrapper before the first wheel read.
It re-verifies that a vt100 screen-model panic under `catch_unwind` degrades readiness to `input-not-ready` while the child's bytes still pass through to the human's terminal.
It tallies the Path 1 CI-timeout WATCH (`path1_run_start_one_home_in_order` against the ci profile's 20 s kill line) on this chunk's settled CI reads; the kill line does not move.

## Reached files
- src/run/gate.rs · re-reads the `catch_unwind` site behind the vt100 panic degrade to `input-not-ready`
- crates/viola-agent-claude/src/screen.rs · re-reads the `catch_unwind` site of the screen model and its passthrough

## Needs
- Size caps on external readers: `hook` stdin (incl. `hook statusline`), `mcp` stdin, channel frames on client and server, the child PTY output into the vt100 feed, `claude agents --json` and `claude --version` output, `send`'s `--file` / stdin text, and the status line command's stdout; each read through `take(MAX_FRAME)` or `MAX_FRAME + 1`
- Size caps on files read under the home or `~/.claude`: `config.json`, `snapshot.json`, `events.ndjson`, `budget.json`, `ledger/stamps.json`, `statusline-source.json`, `settings.json`; an oversize file read as its reader's failure rule says (no reading, a default, a skip counted)
- IPC coverage trigger: a frame over `MAX_FRAME` gives `-32600` and a close; the frame codec unit cases and the property case at `MAX_FRAME` ± 1
- Hook stdin coverage trigger: oversize stdin and malformed JSON fail open (exit 0, empty stderr, no body) in the security-sweep fail-open matrix, within the spine `Instant` bound
- Paste-injection coverage trigger: ESC, other C0, DEL and C1 in `send.text` and free-text answers refused `not-delivered` / `control-character` ahead of every other refusal, at the client and at the wrapper; LF / CR / TAB and multi-byte UTF-8 accepted; an empty typed text refused `empty-text`; the `typed_text` rule
- `validate_paste_text` unit cases and its proptest property over decoded chars vs raw bytes (UTF-8 continuation bytes never trip it)
- Wrapper-channel integration case where wrapper-side paste validation's `control-character` wins over every other refusal; the root bin's `send` and `answer` refusal-ordering unit cases
- MCP coverage trigger: a non-`ViolaName` `target`, a non-`u64` `dialog_id`, an open-string `behavior` and control chars in `text` each refused by the handler as `invalid-params` with code-only payloads and no frame written (`crates/viola-mcp/tests/handlers.rs`)
- CLI and env coverage trigger: path-traversal names (`../x`, uppercase, `/`, empty) refused by `ViolaName` / `parse_viola_name` before a path join, for CLI positionals, `VIOLA_NAME`, channel and MCP `name` / `driver` / `driven` (raw-channel `link` / `unlink` name validation in `tests/channel_links.rs`), a revive `--id` (the session-id shape), and a pinned key; oversize `answer` JSON
- Closed config values: `config.json`'s `five_hour` / `seven_day` into `Percent` (0–100; 101 and negatives refused) and `dialog_hold_s` into `DialogHold` (60–3600); a bad value takes its default with one `parse-rejected{parser:"config-json"}` line per key
- Closed decision types from outside: hook event kinds, dialog decisions, channel methods and params, MCP tool arguments, a `claude agents --json` row's status; an unknown value refused or skipped and counted per its format (`RefusalReason` `#[serde(other)] unknown` to exit 14, unknown event kinds / fields counted in `skipped`)
- Child-spawn coverage trigger: oversize or malformed `claude agents --json` reads as `unknown`, driven through the fake agent's `agents_mode` (`oversize` · `malformed`) and its answer files beside its exe, under a test-scoped `PATH`
- Filesystem coverage trigger: an over-long ndjson line counted in `skipped.torn_lines`; the `events.ndjson` reader's three skip counts never panic
- Depth limit: no `unbounded_depth` anywhere in the product graph; a deeply nested payload is refused at serde_json's default recursion limit, never a panic or stack overflow
- Hook payload bounds: a question's `option_details` keeps `description` / `preview` only as strings (else `null`), and the Stop's `background_tasks` is carried only as its length
- Child-spawn coverage trigger and chaos suite: a forced vt100 panic refuses `send` `not-delivered` / `input-not-ready` while passthrough continues (`tests/chaos_feed_panic.rs`, which boots outside the scans); the property case of the vt100 feed under `catch_unwind`
- Property suite: the hook stdin and statusline parsers and the `claude agents --json` parser at `cases: 512`; a cargo-fuzz target joins the separate `fuzz/` workspace only as its parser lands, with corpus replay
- Unit tests that plant an oversize or malformed state file (`budget.json`, `snapshot.json`, `ledger/stamps.json`) do so only by writing through the product's writer and then padding, replacing or re-versioning those bytes; booted-wrapper tests never hand-write them
- The proof's enumeration of external readers, each with its cap, depth and type rule, held as a check a reader can run, so a new reader with no bound fails visibly
- Path 1 WATCH: `path1_run_start_one_home_in_order` tallied on this chunk's settled CI reads against the nextest ci profile's unchanged 20 s line; a recurrence reopens the owner question, and no retry or moved kill line is a fix under the zero-flakiness budget
- The `v1-46` capability claim's evidence, its `Last-Event-ID` / web clause out per its own notes

## Standing conventions
- S1 · Test functions named `<subject>_<condition>_<expected>`; integration binaries by the `<surface>_<topic>.rs` prefix (`cli_`, `hook_`, `channel_`, `chaos_`, … in root; `path_`, `mcp_`, `cross_` in `viola-e2e`) · .andromeda/test-plan-detail/test-strategy.md
- S2 · Table-driven cases as rstest `#[case::readable_label]`, except `viola-core` (literal tables in one `#[test]`) and `viola-e2e` (a labelled case table, no rstest edge) · .andromeda/test-plan-detail/unit-test-strategy.md
- S3 · Every proptest block at `cases: 512`, its seeds kept in committed `proptest-regressions/` · .andromeda/test-plan-detail/test-strategy.md
- S4 · Each test makes its own fresh home through `viola_state::fs::create_private_dir` at a not-yet-existing path under `target/e2e-home`, with started wrappers held by a `Drop` guard and nothing shared between tests · .andromeda/test-plan-detail/integration-test-strategy.md
- S5 · Child env is set per child with `Command::env`, never `std::env::set_var`; `.env_clear()` re-adds `LLVM_PROFILE_FILE`; a test-started `viola` drops `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN` · .andromeda/test-plan-detail/integration-test-strategy.md
- S6 · No sleep for synchronisation: waits key on `events.ndjson` byte offsets or `viola wait --after`, and live-appended files are read only through `tests/support/ndjson.rs` · .andromeda/test-plan-detail/test-strategy.md
- S7 · Oracles are literals in the test (refusal codes, exit codes, the `MAX_FRAME` boundary), never imported from the product's own lists; refusals are compared by serde kebab value and integer exit, never `Debug` · §11 Test Anti-Patterns of the body
- S8 · Each new fixture or fake script holds synthetic text only and passes the fixture-hygiene walk (path, username, email, schema) · .andromeda/test-plan-detail/test-data-fixtures.md
- S9 · Every E2E test is followed by the error-sanitisation and secret scan and the diagnostics schema conformance check · .andromeda/test-plan-detail/e2e-test-strategy.md
- S10 · Coverage gates hold per OS (lines 85, functions 95, regions 80), and the coverage ignore regex is never widened · .andromeda/test-plan-detail/quality-gates-coverage-targets.md
- S11 · Zero flakiness: nextest `retries = 0`, no `#[ignore]` parking, and a flake keeps the chunk red until its root cause is fixed · .andromeda/test-plan-detail/quality-gates-coverage-targets.md

## Not needed
- Path 4's dialog → `answer` decision bodies and their insta snapshots per CLI version: the chunk bounds the dialog's inputs, not its decision bodies
- Path 5's wheel scenario (human keystroke, `pause`, `release`): the chunk adds no wheel behaviour
- Path 6's statusline pass-through and `release --budget` override: only the config value bounds and the bounded file reads count
- Path 7's unverified CLI version gate and its `stamped … pass … fail` verify line
- Scenario E2's R8 `CLAUDE*` env strip and its canary scan legs
- Scenario E5's snapshot replay recovery legs (links, budget pause, `already-live` on a fresh beat)
- The exit-cause matrix (`cross_exit_causes.rs`) and its per-cause hints: owed to "Exit-cause code catalogue"
- Socket directory, peer-credential, server-identity and home-hardening negatives: owed to "Unix endpoint and home hardening"
- Windows-only live readings and the Windows DACL descriptor stubs: owed to "Windows-only live measurements"
- `list` row integrity's C0/C1 output escaping: that is the output side, not an input bound
- Scenario E4 and every frontend surface: deferred to 0.2.0
- The `Last-Event-ID` / resume-cursor cap clause: carried to `.andromeda/residuals.md` for 0.2.0
- The mutation gate: judged at the epoch boundary by `/andromeda-code-audit`, never per chunk
- The harness session record, its command grammar and the `status` endpoint shape
