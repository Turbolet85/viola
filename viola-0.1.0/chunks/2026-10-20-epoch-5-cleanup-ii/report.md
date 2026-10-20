# Report — 2026-10-20-epoch-5-cleanup-ii

**Chunk:** the Epoch 5 boundary's three split-off debts (survivors, sizes, recorded tools/list) closed so the next audit
reads them closed
**Date:** 2026-10-20
**Commits:** `2cdfb0dd chore(2026-10-20-epoch-5-cleanup-ii): operator pre-CI commit, for the run this chunk's verdict
reads` (on `844de976`, the last wrap's flip; `git log --format='%h %s' 844de976..HEAD`)

## Changes (structured — detectors read this)
- **Files:** 30 changed (`gate.py scope` at this wrap: `clean — changed 30 · listed 29 · recorded 1 (companion 1)`):
  - item (5) — `crates/viola-agent-claude/src/ledger.rs`, `crates/viola-agent-claude/src/ledger/test_cases.rs` (new,
    2752 lines), `src/cmd/verify/typed.rs` + `src/cmd/verify/typed/{runs,turns,pty,run,test_support}.rs` (new),
    `src/human/board.rs` + `src/human/board/{pairs,bay,cells,rows,test_support}.rs` (new), `crates/viola-mcp/Cargo.toml`,
    `Cargo.lock`;
  - item (4) — `crates/viola-agent-claude/src/statusline.rs`, `crates/viola-state/src/liveness.rs`,
    `crates/viola-e2e/src/harness/run/mutants/scratch.rs`, `crates/viola-e2e/src/harness/run/mutants.rs`,
    `crates/viola-e2e/src/harness/session.rs`, `crates/viola-e2e/src/harness/status.rs`, `crates/viola-mcp/src/lib.rs`,
    `crates/viola-mcp/src/tools.rs`;
  - item (7) — `crates/viola-e2e/src/bin/viola-mcp-tap.rs` (new, 312 lines), `crates/viola-e2e/Cargo.toml`,
    `crates/viola-e2e/tests/mcp_recorded_exchange.rs` (new, 226 lines), `schemas/claude-mcp-fixture.v1.json` (new),
    `fixtures/claude/2.1.302/Mcp.tools-list-1.json` (new, 199 lines), `tests/contract_fixture_hygiene.rs`.
  - Not changed (plan-listed as conditional): `tests/contract_fake_agent_drift.rs`, `.config/nextest.toml`.
- **Symbols / APIs:**
  - **Removed:** `viola_agent_claude::ledger::LedgerRow::relied_here` (`pub const fn`, its only caller `relied()`,
    research §Graph impact; a name grep over `crates src tests` reads none, gate entry `grep -rn 'relied_here' crates src
    tests` exit 1). `LedgerRow::relied()` now filters on `relied_on(WRAPPED_HERE)` (`ledger.rs:154-157`, new text);
    `relied_on` gains a doc comment (`ledger.rs:140-141`). Its behaviour is unchanged (24 on Unix, 21 elsewhere).
  - **Changed signature (test-only crate):** `viola_e2e::harness::run::mutants::scratch::prepare(root: &Path, host:
    bool)` (`scratch.rs:46-49`); its one caller `mutate()` passes `scratch::HOST_SCRATCH` (`mutants.rs:224`). Behaviour
    unchanged on every host.
  - **New private fns (test-only crate):** `session::flag_set(Result<String, VarError>) -> bool` (`session.rs:40-43`,
    true for exactly `Ok("1")`) and `session::drop_steps(cleaned, keep) -> (bool, bool)` (`session.rs:45-49`), both
    called by `impl Drop for HarnessSession`; `status::capped_document(impl Read) -> Option<Value>` (`status.rs:71-78`,
    `take(MAX_FRAME + 1)` then `len <= MAX_FRAME`), called by `read_list` (`status.rs:69`).
  - **Moved, paths kept:** `src/cmd/verify/typed.rs` is a module root (`RunDirs`, `Inputs`, `RunStep`, `measure`,
    `POLL`) over `typed/runs.rs` (Runs A–E, `trusted_args`, `TrustedDir`), `typed/turns.rs` (`poll_until`, `wait_for`,
    `count_of`, `input_box_up`, `turn`, `agents_read`, `framing_turns`, `added_turn_ended`, `local_command_answered`,
    `probe_until`, `ms`, `max_gap`, `wait_capture`, `captured`), `typed/pty.rs` (`KeyPipe`, `SharedPty`, `Feed`, `Sink`,
    `Settled`, `settled`, `box_wait`), `typed/run.rs` (`Run`, `ctrl_c_ending`, `ended_by`; `run.rs:60-70` `pub(super)
    struct Run`); `src/cmd/verify.rs` keeps `typed::measure`, `typed::RunDirs`, `typed::Inputs`.
    `src/human/board.rs` is a module root re-exporting `BayBudget` (`bay.rs`), `home_state_pair` /
    `instance_state_pair` (`pairs.rs`) and `write_board` (`rows.rs:82-106`); `cells.rs` holds the cells, spans,
    `board_line`, `cut`; `src/human.rs:16`'s path is unchanged.
  - **Test-only module form:** `ledger.rs:1517-1520` — `#[cfg(test)] #[path = "ledger/test_cases.rs"] mod tests;` with a
    comment naming why (the orphans gate, below): every case stays `ledger::tests::…` (198 cases, `cargo nextest run -p
    viola-agent-claude -E 'test(/ledger::tests::/)'`, equal to the base listing's count).
  - **New test-only bin:** `viola-mcp-tap` (`crates/viola-e2e/src/bin/viola-mcp-tap.rs`): `record <capture> <viola exe>
    <args…>` (`record` 77-129: a direct spawn, no shell and no PATH; each line through `take(MAX_FRAME)`, `next_line`
    57-66; one append per direction-tagged line, the capture created 0600, `open_capture` 46-55; exits with the child)
    and `fixture <capture> <out>` (`fixture` 191-224: `exchange` 144-167 takes the client frames up to and including the
    first `tools/list` request and each request's result keyed by method; `ledger::scrub` (home → `~`, user word →
    `<user>`), then `fault` 181-189, `ledger::screen_fault` per string and key, refusing with a closed code
    `home-path` · `absolute-path` · `username` · `email` and the file name, never content).
  - **New tests:** the nine kill cases (`statusline.rs:452-461`, `liveness.rs:156-171`, `scratch.rs:190-213`,
    `session.rs:160-171`, `:173-183`, `:185-206`, `status.rs:124-142`, `tools.rs:470-528`, `lib.rs:109-133`), the child
    entry `serve_child_entry` (`lib.rs:93-107`, marker `MCP_SERVE_TEST_CHILD` set by `Command::env` on the child alone),
    the tap's 3 unit cases, `mcp_recorded_exchange_replays_the_client_frames_and_keeps_the_accepted_result_shape`
    (`mcp_recorded_exchange.rs:181-226`), and 11 `contract_fixture_hygiene` cases (`mcp_fixtures_pass_hygiene`
    405-419, `planted_clean_mcp_exchange_passes`, `planted_dirty_mcp_exchange_is_rejected` ×3,
    `planted_mcp_schema_violation_is_rejected` ×6); the walker `claude_fixtures_walks_every_version_dir_and_only_json_files`
    gains an `Mcp.` row (`:334`, `:366-369`).
- **Crates / modules:** new modules `src/cmd/verify/typed/{runs,turns,pty,run}` + `#[cfg(test)] test_support`,
  `src/human/board/{pairs,bay,cells,rows}` + `#[cfg(test)] test_support`; no crate added or removed.
- **Dependencies:** `crates/viola-mcp/Cargo.toml` — `serde.workspace = true` dropped (`evidence/serde.md`: `cargo
  check -p viola-mcp --all-targets` exit 0, `cargo machete crates/viola-mcp` clean; `Cargo.lock` one line, the
  `viola-mcp` entry's `"serde"`); `crates/viola-e2e/Cargo.toml` — gains `viola-agent-claude = { path =
  "../viola-agent-claude" }` (`Cargo.toml:29-30`, for the scrub) and the `[[bin]]` `viola-mcp-tap` (`:18-22`). No external
  crate added; `cargo deny check` exit 0; `bash scripts/rmcp-features.sh` exit 0.
- **Schema / config:** new `schemas/claude-mcp-fixture.v1.json` — the `Mcp.` class: `v` const 1; `client` an array of
  JSON-RPC frames (each `jsonrpc` "2.0", `method` a string) that `contains` a `tools/list` request with an `id`;
  `results` an object keyed by method, `tools/list` required with a `tools` array of named tools, any other key an
  object. New fixture class `fixtures/claude/<cli-version>/Mcp.<variant>.json` (`Mcp.tools-list-1.json` in `2.1.302`),
  walked by `tests/contract_fixture_hygiene.rs` against that schema plus an email check; `claude_fixtures` now leaves
  `Mcp.` files out (`:63-68`). Scrub/redaction shape: the tap applies the `--record` scrub and its four closed codes.
- **Spec-master edits:** none.
- **Retired:** `LedgerRow::relied_here` (no master names it — pattern `relied-here`, 14 standing master rows, every one
  `LedgerRow::relied()` / `relied_on`, none `relied_here`); the single-file `src/cmd/verify/typed.rs` and
  `src/human/board.rs` (now module roots); the `viola-mcp` → `serde` edge.
- **Counts / qualifiers moved:**
  - `fixtures/claude/2.1.302` holds 30 files, was 29 (`ls fixtures/claude/2.1.302 | wc -l`: 30; at `844de976`, `git
    ls-tree --name-only 844de976 fixtures/claude/2.1.302/ | wc -l`: 29) — the `Mcp.` exchange. Its clause "recorded whole
    by one `viola verify --record` round" no longer holds: the `Mcp.` file is the tap's. Pattern `set-302-count`: 1
    standing master row, `architecture.md:374` at @c3803 ("recorded whole by one" — stale), @c3856 ("29 files" — stale),
    @c3887 (the rule `ls fixtures/claude/2.1.302 | wc -l` — true, kept).
  - Test listings: every case kept its name, 0 removed in every package (`evidence/sizes-after.md`); no master states a
    test-case count (the Tier 1 rule) — no master site.
  - Sizes (tokei code lines, the audit's M1/M2): `typed.rs` 1212 → the root 59 and children ≤ 355; `board.rs` 949 → the
    root 9 and children ≤ 596; `ledger.rs` 3773 → 1214 + `ledger/test_cases.rs` (`evidence/sizes-before.md`,
    `sizes-after.md`) — no master site states these (pattern `typed-split` / `board-split` rows name paths, not sizes).
  - Unchanged, read: `obs_event!` spawn-pair sites in `src/cmd/verify` 2 → 2; `human::refuse(` calls 14 → 14.
  - Sweep reach: 20 patterns · 19 moved facts, 15 with a pattern (the report's own count: F1–F19, of which F3
    `ledger-tests`, F7b `tap-name`, F12 `capped-doc` read 0 by an unfired control and F13 is the curation-only rule) ·
    owner pattern control unfired · by line 15 lines · 39 offsets (14 lines · 36 offsets from the 15-pattern call, 1
    line · 3 offsets from `repo-fixtures`, and `set-302-count`'s line already counted).
- **Dev-tool versions:** none — claude re-read at `2.1.302 (Claude Code)` by its install path
  (`~/.local/share/mise/installs/claude/2.1.302/claude --version`, dev host, before each live start); cargo-modules
  re-read at 0.27.0 (its behaviour measured, below), unchanged.
- **Harness / gate surface:** `viola-harness`'s `scratch::prepare` takes its host flag from `mutate()`; the harness's
  `status` reads `list --json` through `capped_document` (gate entry `bash scripts/agent-run.sh status --session
  p-e5c2-smoke` green); `HarnessSession`'s drop now decides through `flag_set` / `drop_steps`. No new agent command,
  no status-shape change.
- **Cross-project / external claims:**
  - CI: ci#39477233397 on `2cdfb0dd` — attempt 1 `failure` (`test (macos-latest)`, `release (macos-latest)` cancelled
    with 0 steps run and no runner: "The job was not acquired by Runner of type hosted even after multiple attempts";
    13/15 success); attempt 2, re-run by the overseer (inputs#I6 item 2), `success` 15/15 — read by this wrap:
    `ci.py conclusion --sha 2cdfb0dd` → `verdict: green · checks 15/15 · attempt 2 · wall 2863 s`. The attempt-2
    `test (macos-latest)` job (118828554674, 19:18:12Z–19:31:12Z) reads `3001 tests run: 3001 passed`; `test
    (windows-2025)` (attempt 1, 118816236855) `2982 tests run: 2982 passed`.
  - The live CLI: claude 2.1.302 on the dev host, two print-mode starts (`evidence/live-tap.md`,
    `evidence/live-sessions.ndjson`). Measured on start 2: on protocol `2026-07-28` the client sends `server/discover`
    (its JSON-RPC id a string) and then `tools/list`, each with `_meta` (`io.modelcontextprotocol/protocolVersion`,
    `…/clientInfo` naming the CLI product, `…/clientCapabilities`), and no `initialize` or `notifications/initialized`;
    viola's two results each carry `ttlMs` 0 and `cacheScope` `private`, the `server/discover` result `resultType`,
    `supportedVersions` (5, ending `2026-07-28`), `capabilities.tools` and `_meta` `io.modelcontextprotocol/serverInfo`.
    Measured by `claude --help` (2.1.302): `--mcp-config <configs...>` is variadic.
  - cargo-modules 0.27.0 (probe crates in the session scratchpad, `cargo modules orphans --lib --deny --target
    x86_64-unknown-linux-gnu`): an out-of-line `#[cfg(test)]` module file is reported as an orphan unless its stem begins
    `test_` — `test_support` and `test_helpers` rc 0; `tests`, `testing`, `support`, `foo`, `unit` rc 1; neither
    `#[path]` nor an inner `#![cfg(test)]` changes it; `#[path = "a/test_cases.rs"] mod tests;` rc 0.
  - Inputs (`inputs.py verify`, fired before this report, run dir `.andromeda/runs/2026-10-20T19-32-00-wrap/`):
    - inputs#I1 · `../additional/viola-overseer/phase-cleanup2-directive.md` · copy no-repo · unchanged — cited (scope,
      research, plan).
    - inputs#I2 · message · copy · n/a — the founder's own three P4 answers («Один старт», «2.1.302», «Один чанк»);
      cited (research, plan).
    - inputs#I3 · message · copy · n/a — the P5 word on `.andromeda/runs/2026-10-20T10-26-22-phase/p5-card-1.md`, delegate
      authority (the Viola overseer, 2026-10-20T11:05Z: "yes — the overseer's word on p5-card-1.md … Beyond-the-plan row 1:
      the founder frees /tmp himself before implement"); PROVISIONAL as a delegate word on the freight. The card's
      `## Beyond the plan` rows, verbatim:
      | # | proposal | owner |
      |---|---|---|
      | 1 | Free the `/tmp` tmpfs to at least 14 GiB before `/andromeda-implement`'s `pre-push`, for example by removing old `$TMPDIR/claude-1000` session dirs or earlier chunks' kept `viola-live-*` homes under `$TMPDIR/viola-e2e-home-1000`. Implement stops at step 1 below that. | the founder, at his desk (nothing outside the work tree is removed by this session) |
      Discharged before implement: inputs#I4 reads "/tmp freed by the founder (15G free at 15:48Z)", and implement read
      `df -h /tmp` 15G free at 15:48Z.
    - inputs#I4 · message · copy · n/a — the implement directive (the operator seat, 2026-10-20T15:48Z): one live start on
      2.1.302 ledgered before it; no mutation run or dispatch; the operator pass, then stop before the wrap.
    - inputs#I5 · `../additional/viola-overseer/relay-cleanup2-answers.md` · copy no-repo · unchanged — dialog 15's
      answers: Q1 the founder's own word «Ещё один, промпт через stdin» (the cap rises to 2); Q2 the overseer's
      `#[path] test_cases.rs`; Q3 the overseer's "Yes, the plan's wording" for gate entry 20.
    - inputs#I6 · `../additional/viola-overseer/wrap-cleanup2-directive.md` · copy no-repo · unchanged — this wrap's
      directive: P1 only (two-window form); the outcome basis is ci#39477233397 attempt 2; the founder's words; after this
      chunk «остальные пусть после чанков встают» (via overseer1, 2026-10-20): route-resolve adapts the tail as usual and
      no next skill follows this wrap.
    - `inputs: 6 entries — unchanged 3 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 3 · uncited
      4 · unparsed 0` — I3–I6 printed `UNCITED` before this report existed; each is cited here.
- **Reverted / negative API facts:** the `Mcp.` class document first named the three `initialize`-flavoured client
  frames (plan step 15) and was adapted after start 2 measured `server/discover` (the tap's `exchange`, the schema's
  `prefixItems`, the planted cases) — nothing of the first form shipped. A child marker `VIOLA_MCP_SERVE_TEST_CHILD` was
  renamed `MCP_SERVE_TEST_CHILD` before any gate (the `VIOLA_*` namespace is product plumbing).
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - `scope.md` §Item (5) premise ("cargo-modules 0.27.0 … does not report `src/run/send/test_support.rs`, an out-of-line
    `#[cfg(test)] mod`, as an orphan … so `.claude/rules/testing.md`'s 2026-09-25 rule … is stale") — measured true only
    for a `test_`-prefixed stem (Cross-project bullet); `ledger/tests.rs` failed gate entry 7 (`orphaned module tests at
    crates/viola-agent-claude/src/ledger/tests.rs`). `.claude/rules/testing.md:59` (curation) stands with an exception,
    not stale; no master site (pattern `orphans`: 20 standing master rows, each the gate's own description, none stating
    the out-of-line rule).
  - `test-plan.md:1027` (§9 Lint row, @c919) "the probe fires on a planted orphan, last line `orphans-check probes: 1/1
    fired, control clean`" — the gate's own log at this chunk reads `orphans-check probes: 2/2 fired, control clean`
    (gate entry `bash scripts/orphans-check.sh --probe && bash scripts/orphans-check.sh`, log `7.3.log`). Pattern
    `orphans` row `test-plan.md:1027` @c919/@c938 — stale; the master site is that one line. Pre-existing:
    `scripts/orphans-check.sh` is unchanged since `844de976` (`git diff --quiet 844de976 -- scripts/orphans-check.sh`),
    and there it holds two `probe_case fire` rows and prints `orphans-check probes: 2/2 fired, control clean`
    (`git show 844de976:scripts/orphans-check.sh`, lines 127-133).
  - Plan step 15 / `scope.md` §Item (7): the class document holds "the client's `initialize` request, its
    `notifications/initialized`, and its `tools/list` request" — measured false for claude 2.1.302 on `2026-07-28`
    (Cross-project bullet). Master: pattern `mcp-handshake` 3 standing rows (`architecture.md:30`, `:51` the spec version
    — true; `:152` "negotiates protocol `2026-07-28`" — true, and the site where the measured frames belong).
  - Plan step 18's argv `claude -p --strict-mcp-config --mcp-config <file> '<prompt>'` — start 1 exited 1 in 252 ms with no
    server spawned; HYPOTHESIS (stderr unrecorded): the variadic `--mcp-config` took the prompt. Not a master claim (no
    master site — the argv lives in the plan alone).
  - Research §Sweeps: "`cfg(windows)\|cfg(unix)\|cfg(not` over … `ledger.rs`: 3 hits" — the moved tests held two more
    `cfg!(windows)` oracles (`verified` cases) the Windows scope contract reads (`is_windows_gate`); research, not a master.
- **Expected amendments (from plan):**
  - architecture §Infrastructure Patterns → Project directory structure (`typed/`, `board/`, `ledger/tests.rs`) — carried
    (Symbols / APIs; Files): `ledger/tests.rs` is `ledger/test_cases.rs` (inputs#I5 Q2). Sites: `typed-split` 5 standing
    (`registries/contracts/architecture/project-directory-structure.md:33` the tree — stale; `architecture.md:101` @c4162
    `src/cmd/verify/typed.rs`, @c4550 `typed::agents_read`, @c5258 `typed::trusted_args` — paths that now resolve under
    `typed::turns` / `typed::runs`, stale as file cites; `security-plan.md:215` `src/cmd/verify/typed.rs` `resume_run` —
    now `typed/runs.rs`; `test-plan.md:591` @c4487 and `:1095` @c4149,4431,4791 `typed.rs` — now its children);
    `board-split` 10 standing (`project-directory-structure.md:23` the tree — stale; `:70` viola-state's `board.rs` — a
    different file, no-change; `test-plan.md:590`, `a11y-plan.md:303` "the `src/human/board.rs` units" — now under
    `src/human/board/`; `test-plan.md:526`, `:620`, `a11y-plan.md:232`, `:234`, `:236`, `:258` name units only — true,
    no-change); `ledger-tests` 0 (control unfired).
  - architecture §Infrastructure Patterns → Crate dependency direction (`viola-mcp` loses `serde`; `viola-e2e` gains
    `viola-agent-claude`) — carried (Dependencies). Sites: `mcp-serde` 1 standing
    (`registries/contracts/architecture/crate-dependency-direction.md:7` — stale); `e2e-edges` 2 standing
    (`architecture.md:332` the crate list — candidate; `architecture.md:108` the tokio features — no-change).
  - architecture §Occupied Resources (the `viola-mcp-tap` bin, the `Mcp.` class, the schema) — carried (Symbols / APIs;
    Schema). Sites: `tap-name` 0 (control unfired — absent from every master); `mcp-tap` 41 standing, every row
    `viola-harness` (true, no-change ×41; `project-directory-structure.md:82` "test-only: viola-harness (…)" the bin list
    where the tap joins); `fixture-classes` 11 standing (`architecture.md:374` ×3 and `:379` the schemas, `test-plan.md:379`
    ×3, `:970` ×2, `project-directory-structure.md:53` schemas, `:109` fixtures — the class enumerations the `Mcp.` class
    joins; `architecture.md:101` @c14860, `security-plan.md:115`, `test-plan.md:936`, `:962`,
    `crate-dependency-direction.md:6` — Run B's agents read, no-change).
  - test-plan §4 / §5 / §8 (moved units re-cited by their new files; `relied_here` gone) — carried. Sites: the
    `board-split` / `typed-split` rows above; `relied-here` 14 standing, all `LedgerRow::relied()` / `relied_on` — true,
    no-change ×14 (no master names `relied_here`).
  - test-plan §7 (the `Mcp.` class in the hygiene walk) — carried. Sites: `hygiene-walk` 6 standing (`test-plan.md:965`
    heading, `:966`, `:970` @c25 the walk's classes — stale by omission; `:1174` the owner exception — no-change;
    `architecture.md:374` @c271, `:378` "validates every…" — candidates); `fixture-classes` `test-plan.md:379`.
  - test-plan §6 (the replay case in the contract suite) — carried. Sites: `mcp-replay` 8 standing, every one
    `mcp_tools` (`test-plan.md:624`, `:657`, `:678`, `:698`, `:725`, `:772`, `:846`,
    `registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10`) — true, no-change; `ttl-cache` 2
    standing (`architecture.md:152`, `test-plan.md:126` "its result carrying `ttlMs` 0 and `cacheScope` `private`" — the
    site where the replay's gate on them belongs).
  - security-plan §Data Protection (Repository fixtures) — the `Mcp.` recording through the test-side tap, the same scrub
    and refusal; a boundary widening the founder answered himself (inputs#I2 Q1; the second start inputs#I5 Q1) — carried
    (Symbols / APIs; Cross-project). Sites: `repo-fixtures` 13 standing (`security-plan.md:258` ×2 "**Repository
    fixtures:** `fixtures/claude/<cli-version>/` i…" — the site; `security-plan.md:115`, `:117`, `:147`, `:252`, `:504`,
    `architecture.md:17`, `:101`, `:374` ×4, `layout-templates.md:204`, `test-plan.md:379`, `:507`,
    `ci-cd-approach.md:19` — `verify --record` facts, true, no-change).
  - `.claude/rules/testing.md` 2026-09-25 "stale on cargo-modules 0.27.0" — superseded: the rule holds except for a
    `test_`-prefixed stem (Spec claims disproved); P3's curation, `testing.md:59` (pattern `orphans`, curation 1 row).
- **Coverage of new surfaces:**
  - `viola-mcp-tap record` (test-only bin) → validation `take(MAX_FRAME)` per line, a direct spawn, the capture 0600✓ ·
    instrumentation n/a (test-only, stderr usage line only) · PII n/a (the capture stays in a private dir, never
    committed) · tests unit (3) + the live round · a11y n/a · tokens n/a
  - `viola-mcp-tap fixture` → validation the `--record` scrub + four closed refusal codes✓ · instrumentation n/a · PII
    redacted✓ (scrub; identity read before commit, `evidence/live-tap.md`) · tests unit + `contract_fixture_hygiene` ·
    a11y n/a · tokens n/a
  - the `Mcp.` fixture class → validation its schema + hygiene walk + email check✓ · instrumentation n/a · PII
    redacted✓ · tests contract (11) + E2E replay · a11y n/a · tokens n/a

## Deviations from intent
- **Step 3 — the test module file is `ledger/test_cases.rs`**, declared `#[cfg(test)] #[path =
  "ledger/test_cases.rs"] mod tests;` — justified: cargo-modules 0.27 flags any out-of-line test module whose stem does
  not begin `test_` (measured); the operator's word (inputs#I5 Q2) chose this form over a renamed module or undoing the
  move. Two more `cfg!(windows)` oracles in the moved tests were restated `!cfg!(unix)` (same truth on the three CI OSes),
  the planned fold's form, so the Windows scope contract does not read the file as Windows-gated.
- **Step 13 — the served-duplex case** speaks protocol `2026-07-28` and ends its client by `shutdown()`: dropping one
  half of a `tokio::io::split` never signals EOF (two hung runs at implement, terminated).
- **Step 14 — the child entry** `serve_child_entry` is one more listed case beside the nine (the plan's design of a
  child half); its marker carries no `VIOLA_` prefix (`PTY_SIDELOAD_TEST_CASE` precedent).
- **Step 15 — the class is the measured handshake**: client frames through the first `tools/list`, results keyed by
  method (plan: `initialize`, `notifications/initialized`, `tools/list`, two results) — justified by start 2's capture;
  in-intent (the intent is to record what the live CLI sends). The tap's refusal reuses `ledger::screen_fault`, the one
  public check covering all four codes (`ledger::unclean` covers two). The tap carries 3 unit cases.
- **Step 17 — the replay** takes the `tools/list` response by its request's id, not by position (follows step 15).
- **Step 18 — two starts, not one**: start 1 in the plan's argv yielded no recording (exit 1, 252 ms, no server); start 2
  on the founder's own word (inputs#I5 Q1) gave the prompt on stdin and recorded the exchange. Both ran the ledgered
  build `5a63d1c72218cf00` / `276835b941422a52`: before start 2 the two `target/debug` names were relinked by `ln -f` to
  cargo's own artifacts and their hashes read equal to the ledger's.
- **Gate entry 20 (the preservation guard)** reads red by the plan's wording: its `--allow` regex holds research's two
  directory items `src/cmd/verify/typed/` and `src/human/board/`, which never match a path whole; the 10 files `outside`
  are exactly their children (inputs#I5 Q3, below).
- **Staging**: a synthetic capture's fixture was placed at the fixture path for one read before start 1 and removed.
- **The operator pass** respelled 3 + 3 temp-dir paths `/tmp/<segment>` → `$TMPDIR/<segment>` in the phase run's
  `p5-card-1.md` and `p5-prepush-red.md` (hygiene's P1 `tmp` rows), nothing else changed (`evidence/operator-pass.md`).
- **Scope record** (`gate.py scope` at this wrap: `scope: clean — changed 30 · listed 29 · recorded 1 (companion 1 ·
  mechanical 0 · in-intent 0 · widening 0) · absorbed 10 · excluded 74`):
  - companion — `crates/viola-agent-claude/src/ledger/test_cases.rs` · serves step 3 · word: "#[path] test_cases.rs:
    `#[cfg(test)] #[path = \"ledger/test_cases.rs\"] mod tests;` keeps every test path `ledger::tests::…`. Its
    scope-record line carries this word." — the Viola overseer, 2026-10-20, relayed verbatim by the operator seat
    (inputs#I5, dialog 15 Q2).

## Decisions & corrections
- The founder's own word, item (7)'s boundary widening (the CLI's MCP client through the test-only tap, the scrubbed
  frames committed as a fixture): «Один старт» at P4 (inputs#I2 Q1), then «Ещё один, промпт через stdin» at dialog 15
  (inputs#I5 Q1) after start 1 yielded nothing — the cap 2, both starts ledgered before they ran (16:28:32Z, 18:31:58Z).
  The wrap halts on this widening once, as one answer card (inputs#I6 item 3; p5-card-1.md "The boundary widening").
- The overseer's technical words (inputs#I5): Q2 `#[path] test_cases.rs`; Q3 gate entry 20 reads `red — the plan's
  wording: 20 against research's two directory items`.
- The operator's word at implement (inputs#I4) and at this wrap (inputs#I6): no mutation run or dispatch (held: none
  made); the operator pass in the builder session; P1 only here; after this chunk «остальные пусть после чанков
  встают» — no next skill follows this wrap (P5 adapts the tail as usual).
- A measured CLI fact for the masters (curation candidate, Tier 1 class "design against the installed CLI, measured"):
  claude 2.1.302 on MCP protocol `2026-07-28` opens with `server/discover` and sends no `initialize` /
  `notifications/initialized`; evidence `evidence/live-tap.md` (start 2's capture).
- A measured CLI fact: `claude --mcp-config <configs...>` is variadic (2.1.302 `--help`), so a positional prompt after
  it is not a prompt — a live rig gives the prompt on stdin, or puts nothing after the option (start 1, exit 1).
- A rig rule (curation candidate): a live start's rig records the CLI's stderr (to a private file, its byte count and
  hash prefix in the record) — start 1's capped failure text was lost because the rig printed stdout's count only.
- A sweep hazard (curation candidate, extends `.claude/rules/testing.md:59`): cargo-modules 0.27.0 skips an out-of-line
  `#[cfg(test)]` module file only when its stem begins `test_`; `tests`, `testing`, `support`, `foo`, `unit` are
  flagged, `#[path]` and inner `#![cfg(test)]` do not change it — a premise read on one `test_support` file (scope.md)
  generalised wrongly.
- A sweep hazard (curation candidate): the Windows scope contract's `is_windows_gate` reads `cfg!(windows)` as well as
  `cfg(windows` — a research sweep on `cfg(windows)` alone missed two oracles in the moved ledger tests.
- A test-writing hazard (curation candidate): dropping one half of a `tokio::io::split` duplex never signals EOF to the
  peer; a test that ends a served session calls `shutdown()` on its write half (two hung runs, 600 s and 300 s).
- A gate-authoring hazard (curation candidate, P5 control coverage): a guard's `--allow` regex matches a path WHOLE, so a
  research directory item (`src/human/board/`) never matches its children; P5's controls changed a plain listed file and
  an outside file, never one under a directory item (gate entry 20, `guard: outside 10 files`).
- A build-identity hazard (curation candidate): `target/debug/viola` is a hard link cargo re-points per feature set — a
  `fake-agent` test run between a rig's setup and its start swaps the binary (setup read `e8d82882501399d5`, the start
  `5a63d1c72218cf00`); read the hash right before a live start, and restore a ledgered build by `ln -f` from
  `target/debug/deps/`.
- A CI hazard (curation candidate): a hosted macOS arm64 job can be cancelled with 0 steps ("not acquired by Runner of
  type hosted") — not a red of the tree; read the job's annotations and runner before any reading of its cause, and the
  macOS leg stays unread until a re-run (the overseer re-ran it: attempt 2 green).
- Hygiene at the operator pass: a phase run's P5 records carried `/tmp/<segment>` paths; respelled `$TMPDIR/…`
  (curation candidate for phase: write temp paths in the run-dir spelling at authoring).
- Bash guard refusals this chunk: 4 at implement (a `cd` out of the root ×2, a cat heredoc to a file, a pipeline tool's
  output into a filter), each re-issued once in the guarded form; one AskUserQuestion dialog rejected by the operator,
  who pointed to the relay file instead (the dialog had already been answered elsewhere).
- A recurrence of a standing learning: `.claude/rules/testing.md` 2026-09-25 (an out-of-line `#[cfg(test)] mod` is an
  orphan) — held at gate entry 7, with the `test_` exception.

## Outcome
- **Acceptance criteria**, re-asserted against the diff:
  - (tests) 16 survivors in `evidence/survivors.ndjson`, 10 `killed` and 6 `restated` (#1, #6, #8, #9, #10, #11), each red
    by its own panic line under the `mutants` profile with the mutant applied by hand and green reverted byte-exact; none
    "equivalent" or "recorded", none from a `cargo mutants` run; the nine-case entry reads `9 tests run: 9 passed` —
    MET. (#1's panic line names `ledger/tests.rs:79`, the file's name at that reading.)
  - (arch) each `viola-mcp` / `viola-state` kill sits in its own crate's tests; no manifest gains an edge to `viola-e2e`;
    `cargo deny check` exit 0 — MET.
  - (tests) the listings equal but for moved module segments and added cases, 0 removed (`evidence/sizes-after.md`) —
    MET; the added cases beyond the nine are named in Symbols / APIs.
  - (arch) the orphans gate exits 0, every caller compiles at its unchanged path, `contract_windows_mutation_scope`
    passes — MET (on the `ledger/test_cases.rs` form, inputs#I5 Q2).
  - (tests) tokei `last line 0`; `grep -c -E '^mod tests \{' …ledger.rs` `last line 0` — MET.
  - (design / layouts / a11y) the board and verify output unchanged: `cli_list`, `cli_verify`, `cli_output_plain`,
    `cli_plugin_install` and `cross_list_rows` pass, no literal edited, every named unit exists under its own name —
    MET.
  - (obs) G1 exit 1 no output; lint-probes exit 0; refuse census 14; spawn-pair sites 2 → 2 — MET.
  - (security) `cargo machete crates/viola-mcp` exit 0, no `ignored` entry — MET.
  - (security) `bash scripts/rmcp-features.sh` exit 0; `viola-e2e`'s rmcp stays `client` — MET.
  - (tests / security) `fixtures/claude/2.1.302/Mcp.tools-list-1.json` exists, made by the live start (start 2; the
    criterion's "the one start" — the founder raised the cap, inputs#I5 Q1), its ledger rows in
    `evidence/live-sessions.ndjson`, hygiene-clean with its planted home-path, user, email and schema cases rejected; no
    `CLAUDE*` value, home prefix, `/home/`, `/Users/`, `\Users\`, user word or email-shaped token (a grep for `/home/`,
    `/Users/`, the host user word and `@`: 0 hits) — MET.
  - (tests) `run --e2e --filter 'binary(mcp_recorded_exchange)'` `"ok":true`; tool names in order; both planted negatives
    read missing — MET.
  - (tests) `pre-push` `"ok":true` stage `linux-tests`; the final HEAD's CI `verdict: green` (ci#39477233397 attempt 2) —
    MET.
- **Gates** (implement's last block firing, trail `.andromeda/runs/2026-10-20T15-48-36-implement/gate-2026-10-20-epoch-5-cleanup-ii.json`:
  `entries 30 · green 25 · red 1 · recorded 1 · not-run 3`):
  - `df -h /tmp` — recorded (exit 0; 15G free of 32G).
  - `cargo fmt --all --check` — green · `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` —
    green · `CARGO_TARGET_DIR=target/wincheck cargo clippy … --target x86_64-pc-windows-msvc -- -D warnings` — green ·
    G1 `rg -n -U --pcre2 --type rust '#\[(tracing::)?instrument\b(?!\(\s*skip_all\b)' .` — green (exit 1, no output) ·
    `bash scripts/lint-probes.sh` — green.
  - `bash scripts/orphans-check.sh --probe && bash scripts/orphans-check.sh` — green (red at the first firing on
    `ledger/tests.rs`; green after inputs#I5 Q2's form).
  - `bash scripts/rmcp-features.sh` — green.
  - `bash scripts/agent-run.sh run --unit` — green (`"ok":true`).
  - `cargo nextest run --workspace --features fake-agent -E 'test(/(^|::)tests::(…nine…)(::|$)/)'` — green (`9 tests run:
    9 passed`).
  - `tokei --files -o json src/cmd/verify src/human | python3 -c …` — green (`last line 0`) · `grep -c -E '^mod tests \{'
    crates/viola-agent-claude/src/ledger.rs` — green (exit 1, `last line 0`) · `grep -rhoE 'human::refuse\(' src | wc -l`
    — green (`last line 14`) · `grep -rn 'relied_here' crates src tests` — green (exit 1, no output).
  - `bash scripts/agent-run.sh run --integration --filter 'binary(cli_verify) | binary(cli_list) | binary(cli_output_plain)
    | binary(cli_plugin_install)'` — green.
  - `bash scripts/agent-run.sh run --e2e --filter 'binary(contract_windows_mutation_scope) | binary(contract_fixture_hygiene)
    | binary(contract_fake_agent_drift) | binary(contract_ledger_probes) | binary(contract_lints) | binary(cross_list_rows)
    | binary(mcp_tools)'` — green (red before the recording, only `mcp_fixtures_pass_hygiene`'s empty walk).
  - `bash scripts/agent-run.sh run --e2e --filter 'binary(mcp_recorded_exchange)'` — green (red before the recording by
    construction).
  - `cargo deny check` — green · `cargo machete crates/viola-mcp` — green.
  - `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py guard … --base 844de976… --allow
    '(…)' -- src crates tests schemas plugin scripts fixtures .github .config Cargo.toml Cargo.lock` — `red — the plan's
    wording: 20 against research's two directory items · word: "Yes, the plan's wording" — the Viola overseer,
    2026-10-20, relayed verbatim by the operator seat (inputs#I5 Q3)`. Its pass line: `guard: allowed 19 — …
    companions excluded 1 — crates/viola-agent-claude/src/ledger/test_cases.rs (serves step 3) · changed 30 · excluded
    0`, then `outside` the 5 `src/cmd/verify/typed/*.rs` and 5 `src/human/board/*.rs`, `guard: outside 10 files`.
  - smoke: `bash scripts/agent-run.sh cleanup --session p-e5c2-smoke` — green · `… boot --session p-e5c2-smoke
    --instance builder` — green · `… status --session p-e5c2-smoke` — green · `bash scripts/g2-zero-panics.sh` — green ·
    `bash scripts/agent-run.sh schema-check` — green · `… cleanup --session p-e5c2-smoke` — green (`processes_gone` and
    `endpoint_gone` true).
  - `bash scripts/agent-run.sh pre-push` — green (`"ok":true`, stage `linux-tests`, 96.78 s).
  - operator legs (`evidence/operator-pass.md`): `gate.py hygiene` — refused 2 files (temp paths), clean after the
    respell and again over the pass record · `git diff --quiet && git diff --cached --quiet && git push origin HEAD` —
    green, `history moved: refs/remotes/origin/build/viola-0.1.0 844de976→2cdfb0dd` · `ci.py conclusion --sha HEAD
    --wait 2400` — `verdict: red` on attempt 1 (two macOS jobs cancelled with no runner — not the tree's: 0 steps), then
    `verdict: green · checks 15/15 · attempt 2` read by this wrap (Cross-project bullet).
  - P3 smoke: the plan's smoke entries ran as P2 gates (green both firings).
- **Watches:** the macOS readiness stall (`wrapper <name> not ready`) · 1 green run [ci#39477233397 attempt 2, job
  118828554674: 3001 passed, 0 `not ready` lines] → 2/3 (since 2026-10-19-newer-claude-cli-rows).
- **Outcome basis:** the operator pass ran — the final state is `2cdfb0dd` (`git log --format='%h %s' 844de976..HEAD`:
  the pre-CI commit alone, no fix commit) and its CI run ci#39477233397 attempt 2, recorded in `evidence/operator-pass.md`
  (attempt 1) and read again here (attempt 2). Implement's persisted report
  `.andromeda/runs/2026-10-20T15-48-36-implement/outcome-2026-10-20-epoch-5-cleanup-ii.md` (the one implement run dir of
  this marker; its STUCK version superseded by its own rewrite after inputs#I5) is the basis for what only it holds; this
  window held that run whole. inputs#I5 and inputs#I6 are the directives between implement and this report.
- **Process hygiene:** implement's census (outcome file): two hung `viola-mcp` test binaries — this run, terminated;
  `claude -p` start 1 — exited 1 by itself, no server; start 2 with its `viola-mcp-tap` and `viola mcp` — exited 0 by
  themselves (`mcp.ndjson` `process-exit` 0); the smoke session — terminated by its cleanup entry. Re-measured here:
  `gate.py census --args projects/viola/target --args viola-mcp-tap --args mcp-live-tap` (this wrap's run dir) —
  `census: none running`.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 844de976 (the parent of the oldest pre-CI commit 2cdfb0dd) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### Cargo.lock — added 1 line(s) in 1 range(s)
added: 1925
### crates/viola-agent-claude/src/ledger.rs — added 9 line(s) in 4 range(s)
added: 140-141 · 154-157 · 1517 · 1519-1520
### crates/viola-agent-claude/src/ledger/test_cases.rs — new file · 2752 line(s)
- 4-67 @5 «fn ledger_rows_are_the_twenty_four_measured_behaviours_in_order() {»
  - 7-35 «assert_eq!(»
  - 37-65 «assert_eq!(»
- 69-89 @70 «fn relied_rows_drop_the_three_statusline_rows_where_the_status_line_is_not_wrapped() {»
  - 71-77 «let ids = |wrapped: bool| -> Vec<&str> {»
- 91-98 @92 «fn probe_prompt_is_fixed_ascii_without_tags() {»
  - 93-96 «assert_eq!(»
- 100-123 @118 «fn parse_version_takes_only_the_documented_first_line(»
- 125-196 @126 «const PROBE_HOOKS_JSON: &str = r#"{»
  - 127-195 «"hooks": {»
- 199-214 @200 «fn capture_plugin_files_register_the_spine_on_the_pinned_path() {»
  - 201-205 «let files = capture_plugin_files(»
  - 207-211 «assert_eq!(»
- 216-224 @217 «fn capture_plugin_files_escape_a_path_as_json() {»
  - 220-223 «assert_eq!(»
- 226-243 @227 «fn event_names_are_the_cli_pascal_names() {»
  - 229-242 «assert_eq!(»
- 245-260 @255 «fn parse_capture_file_name_reads_event_and_k(»
- 262-270 @263 «fn capture_file_name_round_trips() {»
  - 266-269 «assert_eq!(»
- 272-274 «fn capture(event: HookEvent, payload: Value) -> Capture {»
- 276-296 @277 «fn clean_captures() -> Vec<Capture> {»
  - 278-295 «vec![»
- 298-304 «fn clean_run() -> ProbeRun {»
  - 299-303 «ProbeRun {»
- 306-319 @307 «fn clean_typed() -> TypedRun {»
  - 308-318 «TypedRun {»
- 321-323 «fn rows(list: &[&str]) -> Vec<String> {»
- 325-335 «fn probes(run: &ProbeRun) -> Probes {»
  - 326-334 «Probes {»
- 337-342 «fn verdicts(run: &ProbeRun) -> Vec<bool> {»
  - 338-341 «LedgerRow::ALL»
- 344-350 @345 «fn capture_read_keeps_the_size_and_a_parsed_payload() {»
- 352-355 @353 «fn check_passes_every_row_on_a_clean_probe() {»
- 357-369 @358 «fn check_shim_resolution_needs_a_real_executable_that_answered() {»
  - 359-362 «let script = ProbeRun {»
  - 364-367 «let silent = ProbeRun {»
- 371-397 @372 «fn check_spine_hooks_needs_the_four_in_order_each_naming_itself() {»
- 399-411 @405 «fn check_session_start_fields_fails_without_both_fields(#[case] payload: Value) {»
- 413-423 @414 «fn check_prompt_verbatim_needs_the_exact_prompt() {»
  - 416-419 «run.captures[1] = capture(»
- 425-432 @426 «fn check_stop_message_needs_a_string() {»
- 434-442 @435 «fn check_largest_hook_payload_holds_at_the_cap_and_fails_past_it() {»
- 444-452 @445 «fn check_reads_the_first_capture_of_an_event() {»
  - 447-450 «run.captures.push(capture(»
- 454-469 @455 «fn largest_keeps_the_biggest_per_event_in_arrival_order() {»
  - 456-460 «let sized = |event, bytes| Capture {»
  - 461-466 «let got = largest(&[»
- 471-473 «fn all_pass() -> Vec<(LedgerRow, bool)> {»
- 475-477 «fn doc(bytes: &[u8]) -> Value {»
- 479-525 @480 «fn merge_stamp_writes_the_envelope_for_a_first_version() {»
  - 481-489 «let bytes = merge_stamp(»
  - 490-523 «assert_eq!(»
- 527-567 @528 «fn merge_stamp_keeps_unknown_versions_and_fields_and_replaces_its_own() {»
  - 529-535 «let existing = json!({»
  - 539-547 «let bytes = merge_stamp(»
  - 558-565 «assert_eq!(»
- 569-587 @574 «fn merge_stamp_replaces_bytes_of_the_wrong_shape_whole(#[case] existing: &[u8]) {»
  - 575-583 «let bytes = merge_stamp(»
- 589-604 @590 «fn merge_stamp_fills_a_data_object_without_versions() {»
  - 592-600 «let bytes = merge_stamp(»
- 606-632 @607 «fn verified_needs_every_row_pass_under_that_version() {»
  - 609-617 «let full = merge_stamp(»
  - 622-630 «let short = merge_stamp(»
- 634-720 @639 «fn verified_reads_a_fourteen_row_stamp_as_unverified() {»
  - 640-655 «let rows: Map<String, Value> = [»
  - 661-663 «for id in ["long-paste-wrapper", "tag-escaping", "local-command-clear"] {»
  - 664-668 «assert_eq!(»
  - 670-674 «assert_eq!(»
  - 677-681 «assert_eq!(»
  - 683-687 «assert_eq!(»
  - 689-693 «assert_eq!(»
  - 694-700 «for id in [»
  - 701-705 «assert_eq!(»
  - 707-711 «assert_eq!(»
  - 713-717 «assert_eq!(»
- 722-740 @725 «fn verified_reads_a_failed_statusline_row_by_the_relied_set() {»
  - 726-729 «let results: Vec<(LedgerRow, bool)> = LedgerRow::ALL»
  - 730-738 «let stamp = merge_stamp(»
- 742-751 @749 «fn verified_refuses_a_malformed_envelope(#[case] bytes: &[u8]) {»
- 753-760 @754 «fn verified_reads_an_envelope_without_data_as_unverified() {»
  - 756-759 «assert_eq!(»
- 762-774 @768 «fn scrub_rewrites_the_home_in_both_spellings(»
- 776-781 @777 «fn scrub_folds_the_home_case_only_when_asked() {»
- 788-794 @792 «fn scrub_rewrites_the_user_as_a_whole_word(#[case] text: &str, #[case] expected: &str) {»
- 796-810 @797 «fn scrub_walks_keys_arrays_and_nested_objects_and_keeps_other_values() {»
  - 798-802 «let payload = json!({»
  - 804-807 «assert_eq!(»
- 812-816 @813 «fn scrub_with_no_home_and_no_user_changes_nothing() {»
- 818-831 @829 «fn is_clean_refuses_a_path_or_the_user(#[case] value: Value) {»
- 833-843 @841 «fn is_clean_passes_scrubbed_text(#[case] value: Value) {»
- 845-849 @846 «fn is_clean_ignores_an_empty_or_placeholder_user() {»
- 851-864 «fn typed_check(row: LedgerRow, typed: TypedRun) -> bool {»
  - 852-863 «check(»
- 866-885 @867 «fn check_typed_rows_fail_without_a_measurement() {»
  - 868-876 «for row in [»
  - 878-884 «assert_eq!(»
- 887-892 «fn verdicts_with(typed: TypedRun) -> Vec<bool> {»
  - 888-891 «LedgerRow::ALL»
- 894-905 @899 «fn check_modal_signature_needs_a_modal_literal(#[case] row: &str, #[case] expected: bool) {»
  - 900-903 «let typed = TypedRun {»
- 907-914 @908 «fn check_input_box_signature_long_turn_holding_the_literal_passes() {»
  - 909-912 «let long_turn = TypedRun {»
- 916-931 @917 «fn check_input_box_signature_long_turn_holding_a_modal_fails() {»
  - 918-930 «for modal in [»
- 933-945 @934 «fn check_input_box_signature_long_turn_of_none_fails() {»
  - 935-938 «let long_turn = TypedRun {»
  - 940-943 «let without = TypedRun {»
- 947-986 @948 «fn check_input_box_signature_needs_both_screens_without_a_modal() {»
  - 950-953 «let no_turn = TypedRun {»
  - 955-958 «let turn_without = TypedRun {»
  - 960-963 «let ready_without = TypedRun {»
  - 965-985 «for modal in [»
- 988-1005 @994 «fn check_quiet_period_bounds_both_settles(»
  - 999-1003 «let typed = TypedRun {»
- 1007-1018 @1012 «fn check_confirm_window_bounds_the_latency(#[case] ms: Option<u64>, #[case] expected: bool) {»
  - 1013-1016 «let typed = TypedRun {»
- 1020-1041 @1021 «fn signature_rows_keep_only_literal_rows() {»
  - 1022-1029 «let live = rows(&[»
  - 1030-1040 «assert_eq!(»
- 1043-1051 «fn clean(live: &[&str]) -> bool {»
  - 1045-1050 «screen_is_clean(»
- 1053-1067 @1054 «fn screen_is_clean_passes_a_measured_screen() {»
  - 1055-1060 «assert!(clean(&[»
  - 1061-1066 «assert!(clean(&[»
- 1069-1080 @1078 «fn screen_is_clean_refuses_a_kept_row_or_seam(#[case] live: &[&str]) {»
- 1082-1091 @1083 «fn screen_is_clean_reads_no_dropped_row_alone() {»
  - 1084-1090 «assert!(clean(&[»
- 1093-1112 @1094 «fn unclean_names_the_first_check_that_fails() {»
  - 1099-1102 «assert_eq!(»
  - 1103-1106 «assert_eq!(»
  - 1107-1110 «assert_eq!(»
- 1114-1117 «fn fault(live: &[&str]) -> Option<ScreenFault> {»
- 1119-1170 @1120 «fn screen_fault_names_the_row_the_seam_and_the_check() {»
  - 1122-1129 «assert_eq!(»
  - 1130-1137 «assert_eq!(»
  - 1138-1145 «assert_eq!(»
  - 1146-1153 «assert_eq!(»
  - 1154-1161 «assert_eq!(»
  - 1162-1169 «assert_eq!(»
- 1172-1185 @1183 «fn has_email_reads_local_at_domain_dot_tld(#[case] text: &str, #[case] expected: bool) {»
- 1187-1197 @1188 «fn tool(event: HookEvent, name: &str, id: Option<&str>, extra: Value) -> Capture {»
  - 1190-1192 «if let Some(id) = id {»
  - 1193-1195 «if let (Some(payload), Value::Object(extra)) = (payload.as_object_mut(), extra) {»
- 1199-1201 «fn prompted(prompt: &str) -> Capture {»
- 1203-1208 «fn two_questions() -> Value {»
  - 1204-1207 «json!({"questions": [»
- 1210-1212 «fn one_question(text: &str) -> Value {»
- 1218-1294 @1220 «fn clean_dialogs() -> DialogRuns {»
  - 1222-1229 «let answered = json!({»
  - 1231-1293 «DialogRuns {»
- 1296-1301 «const DIALOG_ROWS: [LedgerRow; 4] = [»
- 1303-1316 «fn dialog_check(row: LedgerRow, dialogs: DialogRuns) -> bool {»
  - 1304-1315 «check(»
- 1318-1323 @1319 «fn check_dialog_rows_pass_on_the_recorded_shape() {»
  - 1320-1322 «for row in DIALOG_ROWS {»
- 1325-1345 @1327 «fn check_dialog_rows_fail_when_no_dialog_was_raised() {»
  - 1329-1339 «let silent = DialogRuns {»
  - 1340-1344 «for dialogs in [DialogRuns::default(), silent] {»
- 1347-1350 @1348 «fn wrapped(text: &str) -> String {»
- 1352-1355 @1353 «const TAG_PROMPT: &str = "viola verify probe: the next part is literal sample text and not \»
- 1357-1362 «fn session_start(source: &str, id: &str) -> Capture {»
  - 1358-1361 «capture(»
- 1364-1366 «fn session_end(reason: &str) -> Capture {»
- 1368-1370 @1369 «const NEWLINE_PROMPT: &str = "viola verify probe: this paste holds one line break here\nand \»
- 1372-1392 @1375 «fn clean_trusted() -> Vec<Capture> {»
  - 1377-1391 «vec![»
- 1397-1401 @1399 «fn clean_resumed() -> Vec<Capture> {»
- 1406-1419 «fn agents_row(pid: Option<u32>, id: &str, status: &str) -> Value {»
  - 1407-1414 «let mut row = json!({»
  - 1415-1417 «if let Some(pid) = pid {»
- 1421-1426 «fn agents_of(child_pid: Option<u32>, rows: &[Value]) -> AgentsCapture {»
  - 1422-1425 «AgentsCapture {»
- 1428-1437 @1429 «fn clean_agents() -> AgentsCapture {»
  - 1430-1436 «agents_of(»
- 1439-1447 «fn agents_check(agents: Option<AgentsCapture>) -> bool {»
  - 1440-1446 «check(»
- 1449-1463 @1450 «fn check_agents_join_passes_on_the_recorded_shape() {»
  - 1452-1455 «let busy = agents_of(»
  - 1459-1462 «assert!(»
- 1465-1513 @1466 «fn check_agents_join_fails_on_each_broken_post_condition() {»
  - 1468-1500 «for (label, rows) in [»
  - 1501-1504 «let after_clear = agents_of(»
  - 1505-1508 «let cleared = Probes {»
  - 1509-1512 «assert!(»
- 1515-1535 @1516 «fn check_agents_join_fails_with_no_capture() {»
  - 1518-1521 «let no_pid = AgentsCapture {»
  - 1523-1529 «for bytes in [b"".as_slice(), b"not json", br#"{"pid":4242}"#] {»
  - 1530-1533 «let no_run_b = Probes {»
- 1537-1556 @1538 «fn agents_variant_keeps_only_run_b_s_own_row() {»
  - 1539-1542 «assert_eq!(»
  - 1545-1548 «let no_pid = AgentsCapture {»
  - 1550-1553 «let unparsed = AgentsCapture {»
- 1558-1571 «fn resume_check(trusted: Vec<Capture>, resumed: Vec<Capture>) -> bool {»
  - 1559-1570 «check(»
- 1573-1590 @1574 «fn check_resume_session_passes_on_the_recorded_shape() {»
  - 1576-1579 «let upper = vec![session_start(»
  - 1582-1585 «let ended = vec![»
  - 1586-1589 «assert!(»
- 1592-1611 @1593 «fn check_resume_session_fails_on_another_source() {»
  - 1594-1597 «for source in ["startup", "clear", "compact", ""] {»
  - 1598-1601 «let unnamed = vec![capture(»
  - 1603-1606 «let resume_second = vec![»
  - 1607-1610 «assert!(»
- 1613-1630 @1614 «fn check_resume_session_fails_on_another_id() {»
  - 1615-1618 «for id in ["3f2b9c1e-7a4d-4e8b-9c0f-1d2e3a4b5c6e", "s-2", ""] {»
  - 1619-1622 «let no_id = vec![capture(»
  - 1626-1629 «assert!(»
- 1632-1637 @1633 «fn check_resume_session_fails_with_no_run_e_capture() {»
- 1639-1659 @1640 «fn check_resume_session_fails_when_run_b_first_id_is_off_the_session_id_shape() {»
  - 1641-1652 «for id in [»
  - 1655-1658 «assert!(»
- 1661-1667 @1662 «fn resume_target_is_run_b_first_session_id_of_the_shape() {»
- 1669-1682 @1670 «fn resume_variants_name_run_e_first_session_start_resume_1() {»
  - 1671-1675 «let resumed = vec![»
- 1684-1690 «const FRAMING_ROWS: [LedgerRow; 5] = [»
- 1692-1705 «fn framing_check(row: LedgerRow, trusted: Vec<Capture>) -> bool {»
  - 1693-1704 «check(»
- 1707-1711 «fn with_trusted(edit: impl FnOnce(&mut Vec<Capture>)) -> Vec<Capture> {»
- 1713-1718 @1714 «fn check_framing_rows_pass_on_the_recorded_shape() {»
  - 1715-1717 «for row in FRAMING_ROWS {»
- 1720-1741 @1723 «fn check_paste_rows_fail_without_a_matching_capture() {»
  - 1724-1727 «let silent: Vec<Capture> = clean_trusted()»
  - 1728-1738 «for (row, at, text) in [»
- 1743-1752 @1744 «fn check_paste_newline_passes_on_the_one_prompt_holding_the_lf() {»
  - 1747-1750 «let alone = vec![prompted(»
- 1754-1773 @1756 «fn check_paste_newline_fails_when_the_lf_split_the_prompt() {»
  - 1758-1764 «let split = with_trusted(|t| {»
  - 1766-1771 «let joined = with_trusted(|t| {»
- 1775-1790 @1776 «fn check_paste_newline_fails_without_a_matching_capture() {»
  - 1778-1781 «let silent: Vec<Capture> = clean_trusted()»
- 1792-1824 @1793 «fn check_local_command_clear_fails_on_each_broken_post_condition() {»
  - 1795-1797 «let no_start = with_trusted(|t| {»
  - 1805-1807 «let no_session = with_trusted(|t| {»
  - 1811-1814 «let before_the_tag_turn = with_trusted(|t| {»
  - 1815-1818 «assert!(»
  - 1819-1821 «let no_tag_turn = with_trusted(|t| {»
- 1826-1839 «fn compact_check(trusted: Vec<Capture>, typed: TypedRun) -> bool {»
  - 1827-1838 «check(»
- 1841-1849 @1842 «fn check_local_command_compact_passes_on_the_recorded_shape() {»
  - 1844-1847 «let at_the_bound = TypedRun {»
- 1851-1890 @1852 «fn check_local_command_compact_fails_on_each_broken_post_condition() {»
  - 1858-1860 «let no_id = with_trusted(|t| {»
  - 1865-1868 «assert!(»
  - 1869-1872 «let before_the_newline_turn = with_trusted(|t| {»
  - 1873-1876 «assert!(»
  - 1877-1880 «let past = TypedRun {»
  - 1881-1884 «assert!(»
  - 1885-1888 «let unmeasured = TypedRun {»
- 1892-1909 @1893 «fn check_local_command_compact_fails_without_a_capture() {»
  - 1894-1896 «let no_start = with_trusted(|t| {»
  - 1897-1900 «assert!(»
  - 1901-1903 «let no_newline_turn = with_trusted(|t| {»
  - 1904-1907 «assert!(»
- 1911-1922 @1912 «fn limited(five_hour: Value, seven_day: Value) -> StatusLineCapture {»
  - 1913-1917 «let payload = json!({»
  - 1918-1921 «StatusLineCapture {»
- 1924-1926 «fn window(used: Value, resets_at: Value) -> Value {»
- 1928-1942 @1930 «fn clean_statusline() -> Vec<StatusLineCapture> {»
  - 1932-1941 «vec![»
- 1944-1957 «fn statusline_check(row: LedgerRow, statusline: Vec<StatusLineCapture>) -> bool {»
  - 1945-1956 «check(»
- 1959-1968 @1960 «fn check_statusline_override_passes_on_a_filed_payload() {»
  - 1963-1966 «let other_shell = vec![StatusLineCapture {»
- 1970-1977 @1973 «fn check_statusline_override_fails_when_run_b_ran_and_its_status_line_filed_nothing() {»
- 1979-1991 @1980 «fn check_statusline_override_fails_with_no_capture() {»
  - 1981-1989 «let none = Probes {»
- 1993-1999 @1994 «fn check_statusline_shell_passes_when_every_payload_ran_through_bin_sh() {»
  - 1995-1998 «assert!(statusline_check(»
- 2001-2006 @2002 «fn check_statusline_shell_fails_on_one_other_shell_among_bin_sh() {»
- 2008-2011 @2009 «fn check_statusline_shell_fails_with_no_capture() {»
- 2013-2030 @2014 «fn check_statusline_rate_limits_passes_on_two_whole_number_windows() {»
  - 2017-2020 «let bounds = limited(»
  - 2022-2025 «let other_shell = StatusLineCapture {»
  - 2026-2029 «assert!(»
- 2032-2089 @2033 «fn check_statusline_rate_limits_fails_on_each_broken_window() {»
  - 2037-2082 «for (why, five_hour, seven_day) in [»
  - 2083-2086 «let not_object = StatusLineCapture {»
- 2091-2097 @2092 «fn check_statusline_rate_limits_fails_with_no_capture() {»
  - 2093-2096 «assert!(!statusline_check(»
- 2099-2107 @2100 «fn shell_verdict_reads_only_the_exact_word() {»
  - 2104-2106 «for other in [&b"other"[..], b"bin-sh\n", b"BIN-SH", b"", b"/bin/sh"] {»
- 2109-2132 @2110 «fn statusline_variants_name_the_first_without_and_the_first_with_rate_limits() {»
  - 2112-2115 «let second_default = StatusLineCapture {»
  - 2119-2122 «assert_eq!(»
  - 2127-2130 «let names: Vec<String> = statusline_variants(&only_first)»
- 2134-2154 @2135 «fn statusline_capture_file_names_round_trip() {»
  - 2138-2141 «assert_eq!(»
  - 2142-2152 «for other in [»
- 2156-2218 @2157 «fn framing_variants_name_the_three_prompts_and_the_hooks_the_local_command_fired() {»
  - 2160-2169 «let at: Vec<(&str, usize)> = named»
  - 2170-2180 «assert_eq!(»
  - 2187-2190 «assert_eq!(»
  - 2191-2193 «let no_compact = with_trusted(|t| {»
  - 2195-2198 «assert_eq!(»
  - 2199-2206 «let echoed = vec![»
  - 2209-2211 «let no_newline_turn = with_trusted(|t| {»
  - 2212-2215 «let stems: Vec<&str> = framing_variants(&no_newline_turn)»
- 2220-2262 @2222 «fn framing_texts_are_fixed_ascii_each_with_its_stem() {»
  - 2224-2229 «let long = format!(»
  - 2232-2237 «assert_eq!(»
  - 2239-2243 «for text in [PROBE_LONG_PASTE, PROBE_TAG_PASTE] {»
  - 2244-2248 «assert_eq!(»
  - 2258-2260 «for other in [PROBE_PROMPT, "/remote-control", "/compact ", "/clear ", ""] {»
- 2264-2282 @2265 «fn local_commands_are_the_three_known_ones_and_clear_and_compact_are_probed() {»
  - 2266-2273 «assert_eq!(»
  - 2276-2280 «let probed: Vec<&str> = LOCAL_COMMANDS»
- 2284-2288 «fn with_questions(edit: impl FnOnce(&mut Vec<Capture>)) -> DialogRuns {»
- 2290-2294 «fn with_plan(edit: impl FnOnce(&mut Vec<Capture>)) -> DialogRuns {»
- 2296-2303 «fn post_response(answers: Value, annotations: Value) -> Capture {»
  - 2297-2302 «tool(»
- 2305-2327 @2306 «fn check_question_answer_needs_the_label_its_own_post_and_no_permission_request() {»
  - 2308-2313 «let wrong_label = with_questions(|q| {»
  - 2315-2317 «let other_id = with_questions(|q| {»
  - 2319-2321 «let raised = with_questions(|q| {»
  - 2323-2325 «let first_option_only = with_questions(|q| {»
- 2329-2355 @2330 «fn check_question_notes_needs_the_free_text_and_the_note() {»
  - 2332-2337 «let no_free_text = with_questions(|q| {»
  - 2339-2344 «let no_note = with_questions(|q| {»
  - 2346-2353 «let one = with_questions(|q| {»
- 2357-2389 @2358 «fn check_plan_approve_revise_reads_the_revise_the_re_plan_and_the_approve() {»
  - 2359-2362 «assert!(!dialog_check(»
  - 2363-2365 «let approve_not_taken = with_plan(|p| {»
  - 2366-2369 «assert!(!dialog_check(»
  - 2370-2372 «let no_revise = with_plan(|p| {»
  - 2374-2376 «let other_call = with_plan(|p| {»
  - 2378-2380 «let starts_with_a_request = with_plan(|p| {»
  - 2381-2384 «assert!(!dialog_check(»
  - 2385-2387 «let re_plan_missing = with_plan(|p| {»
- 2391-2415 @2392 «fn check_dialog_concurrency_needs_two_distinct_calls_each_answered() {»
  - 2393-2395 «let one = with_questions(|q| {»
  - 2397-2400 «let same_id = with_questions(|q| {»
  - 2402-2404 «let unanswered = with_questions(|q| {»
  - 2406-2409 «let three = with_questions(|q| {»
  - 2411-2413 «let no_id = with_questions(|q| {»
- 2417-2424 @2419 «fn check_dialog_concurrency_with_a_post_for_a_third_id_is_not_answered() {»
  - 2420-2422 «let third_id = with_questions(|q| {»
- 2426-2446 @2427 «fn parallel_both_before_first_post_reads_the_claim_order() {»
  - 2428-2431 «assert_eq!(»
  - 2434-2437 «let unanswered = with_questions(|q| {»
  - 2440-2442 «let one = with_questions(|q| {»
- 2448-2482 @2449 «fn dialog_variants_name_each_call_of_each_turn() {»
  - 2451-2454 «let names: Vec<String> = dialog_variants(&dialogs)»
  - 2455-2471 «assert_eq!(»
  - 2472-2476 «let posts: Vec<Option<&str>> = dialog_variants(&dialogs)»
  - 2477-2480 «assert_eq!(»
- 2484-2501 @2487 «fn dialog_variants_with_a_post_of_an_unknown_id_beside_an_identified_call_records_no_post() {»
  - 2488-2495 «let dialogs = DialogRuns {»
  - 2496-2499 «let names: Vec<String> = dialog_variants(&dialogs)»
- 2503-2523 @2505 «fn dialog_variants_keep_the_first_capture_of_a_variant() {»
  - 2507-2510 «dialogs.questions.insert(»
  - 2511-2515 «let kept: Vec<&Capture> = dialog_variants(&dialogs)»
  - 2517-2522 «assert!(»
- 2525-2541 @2526 «fn dialog_prompts_are_fixed_ascii_without_tags_each_with_its_stem() {»
  - 2527-2532 «for (run, prompt, stem) in DIALOG_TURNS {»
  - 2538-2540 «for text in [PROBE_FREE_TEXT, PROBE_NOTE, PROBE_REVISE] {»
- 2543-2553 @2544 «fn dialog_settings_are_the_ask_rule_and_the_plans_directory() {»
  - 2545-2548 «assert_eq!(»
  - 2549-2552 «assert_eq!(»
- 2555-2594 @2556 «fn probe_answers_are_the_compiled_table() {»
  - 2558-2567 «assert_eq!(»
  - 2568-2570 «for answer in ProbeAnswer::ALL {»
  - 2572-2582 «let table: Vec<String> = PROBE_ANSWERS»
  - 2583-2593 «assert_eq!(»
- 2596-2599 «fn body_of(event: HookEvent, payload: &Value, answer: ProbeAnswer) -> Option<Value> {»
  - 2597-2598 «probe_body(event, payload.to_string().as_bytes(), answer)»
- 2601-2642 @2602 «fn probe_body_builds_each_answer_through_the_product_mapping() {»
  - 2607-2610 «assert_eq!(»
  - 2611-2614 «assert_eq!(»
  - 2617-2620 «assert_eq!(»
  - 2623-2624 «let plan =»
  - 2626-2629 «assert_eq!(»
  - 2631-2635 «assert_eq!(»
  - 2638-2641 «assert_eq!(»
- 2644-2654 @2645 «fn probe_body_with_one_question_answers_it_and_notes_it() {»
  - 2650-2653 «assert_eq!(»
- 2656-2673 @2666 «fn probe_body_is_none_where_the_answer_cannot_map(»
- 2675-2683 @2676 «fn probe_body_with_no_questions_answers_nothing_by_first_option() {»
  - 2678-2682 «assert_eq!(»
- 2685-2752 @2686 «fn capture_plugin_files_for_the_dialog_tier_add_the_three_events_and_the_answers() {»
  - 2687-2693 «let files = capture_plugin_files(»
  - 2695-2700 «let events: Vec<&str> = doc["hooks"]»
  - 2701-2712 «assert_eq!(»
  - 2715-2725 «assert_eq!(»
  - 2726-2729 «assert_eq!(»
  - 2730-2740 «assert_eq!(»
  - 2742-2745 «assert_eq!(»
### crates/viola-agent-claude/src/statusline.rs — added 11 line(s) in 1 range(s)
added: 452-462
  - 452-461 @453 «fn statusline_probe_document_on_a_wrapped_host_is_the_capture_command_document() {»
### crates/viola-e2e/Cargo.toml — added 7 line(s) in 2 range(s)
added: 18-22 · 29-30
### crates/viola-e2e/src/bin/viola-mcp-tap.rs — new file · 312 line(s)
- 27-28 «const USAGE: &str =»
- 34-44 «fn main() -> ExitCode {»
  - 36-43 «match args.first().map(String::as_str) {»
- 46-55 «fn open_capture(path: &str) -> io::Result<File> {»
  - 49-53 @50 «{»
- 57-66 @58 «fn next_line(reader: &mut impl BufRead) -> Option<Vec<u8>> {»
  - 61-65 «match read {»
- 68-75 @69 «fn file_line(capture: &Mutex<File>, dir: &str, line: &[u8]) {»
- 77-129 «fn record(capture: &str, viola: &str, args: &[String]) -> ExitCode {»
  - 78-81 «let Ok(file) = open_capture(capture) else {»
  - 83-88 «let spawned = Command::new(viola)»
  - 89-92 «let Ok(mut child) = spawned else {»
  - 93-97 «let (Some(mut to_child), Some(from_child)) = (child.stdin.take(), child.stdout.take()) else {»
  - 98-110 «let back = {»
  - 112-121 «while let Some(line) = next_line(&mut reader) {»
  - 125-128 «match status.ok().and_then(|s| s.code()) {»
- 131-142 @133 «fn frames(text: &str) -> Vec<(String, Map<String, Value>)> {»
  - 134-141 «text.lines()»
- 144-167 @147 «fn exchange(frames: &[(String, Map<String, Value>)]) -> Option<Value> {»
  - 148-149 «let method =»
  - 152-165 «for (_, frame) in frames.iter().filter(|(dir, _)| dir == CLIENT) {»
- 169-179 «fn texts<'a>(value: &'a Value, out: &mut Vec<&'a str>) {»
  - 170-178 «match value {»
- 181-189 @182 «fn fault(doc: &Value, home: &str, user: &str) -> Option<&'static str> {»
  - 185-188 «all.into_iter().find_map(|text| {»
- 191-224 «fn fixture(capture: &Path, out: &Path) -> ExitCode {»
  - 192-195 «let name = out»
  - 198-201 «if read.is_err() {»
  - 202-205 «let Some(doc) = exchange(&frames(&text)) else {»
  - 208-211 «let user = user_home»
  - 213-216 «if let Some(code) = fault(&scrubbed, &home, &user) {»
  - 219-222 «if std::fs::write(out, body).is_err() {»
- 226-312 @227 «mod tests {»
  - 230-235 «fn capture_of(lines: &[(&str, Value)]) -> String {»
  - 237-264 «fn planted() -> String {»
  - 266-283 @267 «fn exchange_takes_the_client_frames_through_tools_list_and_each_result_by_method() {»
  - 285-293 @286 «fn exchange_of_a_capture_without_tools_list_is_none() {»
  - 295-311 @296 «fn fault_names_each_closed_code_and_a_clean_document_none() {»
### crates/viola-e2e/src/harness/run/mutants.rs — added 1 line(s) in 1 range(s)
added: 224
### crates/viola-e2e/src/harness/run/mutants/scratch.rs — added 42 line(s) in 7 range(s)
added: 46-49 · 165-167 · 176 · 183 · 190-214 · 255-258 · 279-282
  - 190-213 @193 «fn prepare_on_the_scratch_host_empties_the_scratch_and_off_it_is_none() {»
### crates/viola-e2e/src/harness/session.rs — added 67 line(s) in 5 range(s)
added: 6 · 40-50 · 125-126 · 129-133 · 160-207
- 40-43 @41 «fn flag_set(read: Result<String, VarError>) -> bool {»
- 45-49 @47 «fn drop_steps(cleaned: bool, keep: bool) -> (bool, bool) {»
  - 160-171 @161 «fn flag_set_is_exactly_one() {»
  - 173-183 @174 «fn drop_steps_clean_an_uncleaned_session_and_remove_an_unkept_home() {»
  - 185-206 @188 «fn dropped_session_removes_its_home_unless_kept() {»
### crates/viola-e2e/src/harness/status.rs — added 32 line(s) in 6 range(s)
added: 5 · 69 · 71-78 · 80 · 83 · 124-143
  - 124-142 @127 «fn capped_document_takes_a_document_of_exactly_the_bound_and_refuses_one_byte_more() {»
### crates/viola-e2e/tests/mcp_recorded_exchange.rs — new file · 226 line(s)
- 20-42 @21 «fn recorded(root: &Path) -> Vec<PathBuf> {»
  - 22-39 «let mut found: Vec<PathBuf> = std::fs::read_dir(root)»
- 44-54 «fn type_word(value: &Value) -> &'static str {»
  - 45-53 «match value {»
- 56-88 @58 «fn missing(recorded: &Value, served: &Value, path: &str, out: &mut Vec<String>) {»
  - 59-62 «if type_word(recorded) != type_word(served) {»
  - 63-87 «match (recorded, served) {»
- 90-94 «fn shape_gaps(recorded: &Value, served: &Value) -> Vec<String> {»
- 96-103 «fn tool_names(result: &Value) -> Vec<String> {»
  - 97-102 «result["tools"]»
- 105-111 @107 «struct Served {»
- 113-179 «impl Served {»
  - 114-139 «fn spawn(s: &HarnessSession) -> Self {»
  - 141-161 @142 «fn send(&mut self, frame: &Value) -> Option<Value> {»
  - 163-178 @164 «fn close(mut self) -> Option<i32> {»
- 181-226 @182 «fn mcp_recorded_exchange_replays_the_client_frames_and_keeps_the_accepted_result_shape() {»
  - 185-193 «let session = harness_session(»
  - 194-224 «for path in walked {»
### crates/viola-mcp/Cargo.toml — nothing added (lines removed only)
### crates/viola-mcp/src/lib.rs — added 53 line(s) in 1 range(s)
added: 82-134
- 83-134 @84 «mod tests {»
  - 93-107 @96 «fn serve_child_entry() {»
  - 109-133 @111 «fn serve_on_a_closed_stdin_is_the_initialize_error() {»
### crates/viola-mcp/src/tools.rs — added 60 line(s) in 1 range(s)
added: 470-529
  - 470-528 @474 «fn served_initialize_advertises_tools_and_tools_list_carries_the_five() {»
### crates/viola-state/src/liveness.rs — added 17 line(s) in 1 range(s)
added: 156-172
  - 156-171 @157 «fn live_endpoint_of_a_live_instance_is_its_endpoint_and_of_a_stale_one_none() {»
### fixtures/claude/2.1.302/Mcp.tools-list-1.json — new file · 199 line(s)
- 1-199 «{»
  - 3-56 «"client": [»
  - 57-198 «"results": {»
### schemas/claude-mcp-fixture.v1.json — new file · 47 line(s)
- 1-47 «{»
  - 8-46 «"properties": {»
### src/cmd/verify/typed.rs — added 13 line(s) in 4 range(s)
added: 21-23 · 25-27 · 29-34 · 94
### src/cmd/verify/typed/pty.rs — new file · 408 line(s)
- 14-18 @15 «pub(super) struct KeyPipe {»
- 20-27 «impl KeyPipe {»
  - 21-26 «pub(super) fn new(rx: Receiver<Vec<u8>>) -> Self {»
- 29-42 «impl Read for KeyPipe {»
  - 30-41 «fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {»
- 48-52 «impl<P> SharedPty<P> {»
  - 49-51 «fn lock(&self) -> MutexGuard<'_, P> {»
- 54-82 «impl<P: Pty> Pty for SharedPty<P> {»
  - 55-57 «fn reader(&mut self) -> Result<Box<dyn Read + Send>, PtyError> {»
  - 59-61 «fn writer(&mut self) -> Result<Box<dyn Write + Send>, PtyError> {»
  - 63-65 «fn resize(&mut self, size: Size) -> Result<(), PtyError> {»
  - 67-69 «fn try_wait(&mut self) -> Result<Option<u32>, PtyError> {»
  - 71-73 «fn kill(&mut self) -> Result<(), PtyError> {»
  - 75-77 «fn close(&mut self) {»
  - 79-81 «fn child_pid(&self) -> Option<u32> {»
- 84-90 @85 «pub(super) struct Feed {»
- 92-110 «impl Feed {»
  - 93-109 @94 «fn take(&mut self, bytes: &[u8], now: Instant) {»
- 112-115 «pub(super) struct Sink {»
- 117-130 «impl Write for Sink {»
  - 118-125 «fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {»
  - 127-129 «fn flush(&mut self) -> io::Result<()> {»
- 132-135 «pub(super) struct Settled {»
- 137-159 @139 «pub(super) fn settled(feed: &Feed, from: Instant, now: Instant) -> Option<Settled> {»
  - 141-143 «if now < quiet_at {»
  - 145-147 «let literal = rows»
  - 149-158 «if literal {»
- 161-184 @166 «pub(super) fn box_wait(feed: &Feed, from: Instant, now: Instant) -> Option<Settled> {»
  - 167-172 «if feed.screen.is_poisoned() {»
  - 174-176 «if now < quiet_at {»
  - 178-183 «rows.iter()»
- 186-408 @187 «mod tests {»
  - 193-202 «fn feed(text: &[u8], at: Instant) -> Feed {»
  - 204-212 @205 «fn settled_with_a_literal_settles_at_the_quiet_instant() {»
  - 214-227 @215 «fn settled_without_a_literal_waits_out_the_maximum() {»
  - 229-240 @230 «fn settled_counts_quiet_from_the_later_of_the_output_and_the_start() {»
  - 242-253 @243 «fn box_wait_keeps_waiting_past_the_gate_maximum_without_a_literal() {»
  - 255-274 @256 «fn box_wait_ends_on_a_quiet_literal_with_its_rows() {»
  - 276-284 @277 «fn box_wait_ends_at_once_on_a_poisoned_screen() {»
  - 286-298 @287 «fn feed_past_the_frame_cap_poisons_and_settles_without_rows() {»
  - 300-310 @301 «fn feed_reaching_exactly_the_frame_cap_does_not_poison_and_one_byte_past_does() {»
  - 312-323 @313 «fn feed_a_vt100_panic_poisons() {»
  - 325-337 @326 «fn key_pipe_reads_each_message_then_ends_when_the_sender_is_gone() {»
  - 339-343 @340 «struct Recording {»
  - 345-374 «impl Pty for Recording {»
  - 376-378 «fn shared() -> SharedPty<Recording> {»
  - 380-395 @381 «fn shared_pty_resize_reaches_its_pty_with_the_size() {»
  - 397-402 @398 «fn shared_pty_close_reaches_its_pty_once() {»
  - 404-407 @405 «fn shared_pty_child_pid_is_its_ptys() {»
### src/cmd/verify/typed/run.rs — new file · 331 line(s)
- 26-47 @29 «fn ctrl_c_ending(»
  - 36-39 «if stopped() {»
  - 41-43 «if !ended(clock.now() + CTRL_C_AGAIN) && !stopped() {»
  - 44-46 «if !ended(probe_until(clock)) {»
- 49-58 @50 «fn ended_by(clock: &dyn Clock, until: Instant, finished: impl Fn() -> bool) -> bool {»
  - 51-56 «while !finished() {»
- 60-70 @61 «pub(super) struct Run {»
- 72-228 «impl Run {»
  - 73-125 «pub(super) fn spawn(»
  - 127-129 «pub(super) fn feed(&self) -> MutexGuard<'_, Feed> {»
  - 131-134 @132 «pub(super) fn paste_text(&self, text: &str) -> bool {»
  - 136-145 @138 «pub(super) fn settle(&self, from: Instant) -> Option<Settled> {»
  - 147-157 @149 «pub(super) fn box_wait(&self, from: Instant) -> Option<Vec<String>> {»
  - 159-163 @160 «pub(super) fn input_box_settles(&self) -> bool {»
  - 165-170 «pub(super) fn child_pid(&self) -> Option<u32> {»
  - 172-178 «fn kill(&self) {»
  - 180-184 @181 «pub(super) fn end_by_kill(self) {»
  - 186-205 @188 «pub(super) fn end_by_ctrl_c(self, keys: &Sender<Vec<u8>>) {»
  - 207-227 @210 «fn finish(self) {»
- 230-331 @231 «mod tests {»
  - 239-268 @244 «fn ctrl_c_ending_sends_again_only_when_the_first_did_not_end_it(»
  - 270-295 @274 «fn ctrl_c_ending_at_the_stop_sends_no_more_and_kills(»
  - 297-310 @299 «fn ended_by_still_waits_with_the_stop_set() {»
  - 312-330 @317 «fn ended_by_reads_true_once_finished_and_false_once_the_deadline_passes(»
### src/cmd/verify/typed/runs.rs — new file · 260 line(s)
- 11-14 «use viola_agent_claude::ledger::{»
- 22-25 «use super::turns::{»
- 27-40 @28 «pub(super) fn untrusted_run(»
- 42-88 @45 «pub(super) fn trusted_run(»
  - 54-60 «let run = Run::spawn(»
  - 62-64 «typed.ready_settle_ms = ready»
  - 66-69 «if !input_box_up(typed.ready.as_deref()) {»
  - 73-77 «let agents = typed»
  - 78-83 «typed.compact_latency_ms = framing_turns(»
- 90-100 @91 «fn trusted_args(args: &[OsString], inputs: &Inputs<'_>) -> Vec<OsString> {»
  - 95-98 «if let Some(settings) = inputs.statusline_settings {»
- 102-143 @105 «pub(super) fn dialog_run(»
  - 116-138 «if run.input_box_settles() {»
- 145-176 @148 «pub(super) fn plan_run(»
  - 163-171 «if run.input_box_settles() && run.paste_text(PLAN_PROMPT) {»
- 178-209 @181 «pub(super) fn resume_run(»
  - 187-189 «let Some(id) = ledger::resume_target(&trusted) else {»
  - 196-202 «let started = wait_capture(»
  - 203-205 «if let Some(started) = started {»
- 214-222 «impl TrustedDir {»
  - 215-221 «fn create(cwd: &Path, suffix: &str) -> io::Result<Self> {»
- 224-228 «impl Drop for TrustedDir {»
  - 225-227 «fn drop(&mut self) {»
- 230-260 @231 «mod tests {»
  - 234-259 @235 «fn trusted_dir_is_private_and_removed_on_drop() {»
### src/cmd/verify/typed/test_support.rs — new file · 17 line(s)
- 7-9 «pub(super) fn ms_(n: u64) -> Duration {»
- 13-17 «impl Clock for FixedClock {»
  - 14-16 «fn now(&self) -> Instant {»
### src/cmd/verify/typed/turns.rs — new file · 419 line(s)
- 8-11 «use viola_agent_claude::ledger::{»
- 19-41 @22 «pub(super) fn poll_until<T>(»
  - 28-40 «loop {»
- 43-51 @44 «pub(super) fn wait_for(»
- 53-70 @56 «pub(super) fn count_of(dir: &Path, event: HookEvent) -> usize {»
  - 57-69 «std::fs::read_dir(dir)»
- 72-78 @73 «pub(super) fn input_box_up(rows: Option<&[String]>) -> bool {»
  - 74-77 «rows.is_some_and(|rows| {»
- 80-104 @81 «pub(super) fn turn(run: &Run, captures: &Path, typed: &mut TypedRun) {»
  - 84-86 «if !run.paste_text(PROBE_PROMPT) {»
  - 88-94 «let submitted = wait_capture(»
  - 96-98 «let Some(stopped) = wait_capture(captures, HookEvent::Stop, clock, until, run.stop) else {»
  - 100-103 «if let Some(settled) = run.settle(stopped) {»
- 106-117 @108 «pub(super) fn agents_read(run: &Run, inputs: &Inputs<'_>, dir: &Path) -> Option<AgentsCapture> {»
  - 109-111 «if (run.stop)() {»
  - 113-116 «(!read.stopped).then(|| AgentsCapture {»
- 119-179 @126 «pub(super) fn framing_turns(»
  - 134-150 «for text in [PROBE_LONG_PASTE, PROBE_TAG_PASTE, PROBE_NEWLINE_PASTE] {»
  - 151-153 «if !input_box_up(rows.as_deref()) {»
  - 157-159 «if !run.paste_text(PROBE_COMPACT_COMMAND) {»
  - 160-162 «let compacted = wait_for(clock, probe_until(clock), run.stop, || {»
  - 164-166 «if !input_box_up(run.box_wait(compacted).as_deref()) {»
  - 169-171 «if !run.paste_text(PROBE_LOCAL_COMMAND) {»
  - 172-174 «let answered = wait_for(clock, probe_until(clock), run.stop, || {»
  - 175-177 «if let Some(answered) = answered {»
- 181-185 @182 «fn added_turn_ended(captures: &Path, prompts: usize, stops: usize) -> bool {»
  - 183-184 «count_of(captures, HookEvent::UserPromptSubmit) > prompts»
- 187-192 @189 «fn local_command_answered(captures: &Path, starts: usize, prompts: usize) -> bool {»
  - 190-191 «count_of(captures, HookEvent::SessionStart) > starts»
- 194-197 @195 «pub(super) fn probe_until(clock: &dyn Clock) -> Instant {»
- 199-201 «pub(super) fn ms(d: Duration) -> u64 {»
- 203-213 @205 «pub(super) fn max_gap(chunks: &[Instant], from: Instant, to: Instant) -> u64 {»
  - 208-211 «for at in chunks.iter().filter(|at| **at >= from && **at <= to) {»
- 215-227 @217 «pub(super) fn wait_capture(»
  - 224-226 «poll_until(clock, until, &stopped, |now| {»
- 229-231 «fn captured(dir: &Path, event: HookEvent) -> bool {»
- 233-419 @234 «mod tests {»
  - 242-244 «fn rows(list: &[&str]) -> Vec<String> {»
  - 246-257 @247 «fn input_box_up_needs_the_input_box_and_no_modal() {»
  - 259-268 @260 «fn max_gap_reads_the_edges_and_the_gaps_between() {»
  - 270-274 @271 «fn max_gap_leaves_out_a_chunk_after_the_window() {»
  - 276-285 @277 «fn captured_reads_only_the_named_event() {»
  - 287-299 @290 «fn captured_reads_no_capture_while_the_claim_is_empty() {»
  - 301-305 «fn plant(dir: &Path, event: &str, n: usize) {»
  - 307-324 @313 «fn added_turn_ended_needs_a_new_prompt_and_a_new_stop(»
  - 326-345 @331 «fn local_command_answered_needs_a_new_start_or_a_new_prompt(»
  - 347-351 @348 «fn probe_until_is_the_probe_deadline_after_now() {»
  - 353-363 @354 «fn poll_until_returns_at_once_at_the_stop_without_asking() {»
  - 365-375 @366 «fn poll_until_ends_at_a_stop_set_while_it_waits() {»
  - 377-399 @380 «fn poll_until_without_the_stop_waits_to_its_answer_or_its_deadline(»
  - 401-418 @402 «fn wait_for_and_wait_capture_end_at_the_stop_over_a_held_condition() {»
### src/human/board.rs — added 8 line(s) in 3 range(s)
added: 5-8 · 10 · 12-14
### src/human/board/bay.rs — new file · 184 line(s)
- 8-12 @9 «pub(crate) struct BayBudget<'a> {»
- 14-28 @16 «fn window_cell(label: &str, window: &Reading<BudgetWindow>, now: DateTime<Utc>) -> String {»
  - 17-20 «let Reading::Known(BudgetWindow {»
  - 21-23 «else {»
  - 24-25 «let expired = matches!(resets_at, Reading::Known(at)»
- 30-41 @32 «fn read_cell(read_at: &Reading<String>, now: DateTime<Utc>) -> String {»
  - 33-36 «let at = match read_at {»
  - 37-40 «match at.filter(|at| *at <= now) {»
- 43-72 @45 «pub(crate) fn bay_line(»
  - 50-67 «let (five_hour, seven_day, read, gate) = match budget {»
  - 68-71 «format!(»
- 74-184 @75 «mod tests {»
  - 80-85 «fn window(used: Reading<f64>, resets_at: &str) -> Reading<BudgetWindow> {»
  - 87-99 «fn stamped(»
  - 101-111 @102 «fn bay_line_without_a_reading_is_the_unknown_line() {»
  - 113-136 @114 «fn bay_line_with_a_reading_shows_the_figures_the_age_and_the_gate() {»
  - 152-164 @154 «fn bay_line_window_cell(#[case] five_hour: Reading<BudgetWindow>, #[case] cell: &str) {»
  - 166-178 @173 «fn bay_line_read_cell(#[case] read_at: &str, #[case] cell: &str) {»
  - 180-183 @181 «fn bay_line_read_cell_of_an_unknown_instant_is_unknown() {»
### src/human/board/cells.rs — new file · 197 line(s)
- 15-22 «pub(super) fn wheel_cell(item: &Item) -> String {»
  - 17-21 «if item.budget_paused == Some(true) {»
- 27-35 «pub(super) fn dialog_cell(item: &Item) -> String {»
  - 28-30 «if !item.dialog_pending {»
  - 31-34 «match item.dialog_kind.as_deref() {»
- 37-49 «pub(super) fn cli_cell(item: &Item) -> String {»
  - 38-48 «match &item.cli_version {»
- 51-60 @53 «pub(super) struct Spans {»
- 62-64 «fn padding(text: &str, width: usize) -> String {»
- 66-100 @67 «pub(super) fn board_line(»
  - 73-93 «let (shown_name, shown_dialog, open, close) = match styled {»
  - 94-99 «format!(»
- 107-110 @109 «pub(super) const UNWRAPPED_REST: usize =»
- 114-121 @115 «fn escape_at(chars: &[char], at: usize) -> bool {»
  - 116-120 «chars.get(at) == Some(&'\\')»
- 123-134 @125 «pub(super) fn cut(name: &str, column: usize) -> String {»
  - 128-130 «if let Some(at) = (keep.saturating_sub(3)..keep).find(|&at| escape_at(&chars, at)) {»
- 136-197 @137 «mod tests {»
  - 142-157 @148 «fn board_row_wheel_cell(»
  - 159-175 @166 «fn board_row_dialog_cell(»
  - 187-196 «fn board_row_cli_cell(»
### src/human/board/pairs.rs — new file · 40 line(s)
- 3-9 @4 «pub(crate) fn instance_state_pair(name: &str) -> (String, String) {»
  - 5-8 «(»
- 11-17 @12 «pub(crate) fn home_state_pair() -> (&'static str, &'static str) {»
  - 13-16 «(»
- 19-40 @20 «mod tests {»
  - 23-39 @24 «fn instance_and_home_state_pairs_are_the_fixed_texts() {»
### src/human/board/rows.rs — new file · 651 line(s)
- 11-14 «use super::cells::{»
- 16-76 @21 «pub(crate) fn board_rows(»
  - 27-34 «let longest = |wrapped: bool| {»
  - 35-38 «let unwrapped = match width {»
  - 40-44 «let mut rows = vec![board_line(»
  - 45-74 «for (item, name) in items.iter().zip(names) {»
- 82-106 @84 «pub(crate) fn write_board(»
  - 96-98 «if wrapped == 0 {»
  - 99-102 «if wrapped < board.items.len() {»
- 108-651 @109 «mod tests {»
  - 116-117 «const CAPTION: &str =»
  - 118-119 «const DEFAULT_ROW: &str =»
  - 121-131 @122 «fn board_row_caption_and_default_row_are_the_literals() {»
  - 133-158 @134 «fn board_row_every_value_starts_in_its_captions_column() {»
  - 160-171 @161 «fn board_row_a_32_character_name_prints_whole_and_moves_the_columns_right() {»
  - 173-183 @174 «fn board_row_rows_keep_the_boards_name_order_whatever_their_state() {»
  - 185-197 @186 «fn board_row_the_empty_board_is_three_lines_in_one_write() {»
  - 199-205 «fn unwrapped(name: &str, status: viola_state::board::SessionStatus) -> Item {»
  - 207-223 @211 «fn board_row_status_cell_is_the_status_word(»
  - 227-244 @228 «fn board_row_the_separator_and_an_unwrapped_row_are_the_literals() {»
  - 246-275 @249 «fn board_row_an_unwrapped_name_with_controls_prints_on_one_line_in_the_captions_columns() {»
  - 277-295 @278 «fn board_row_an_empty_wrapped_rack_with_unwrapped_rows_keeps_the_empty_board_line() {»
  - 297-306 @298 «fn board_row_no_separator_without_an_unwrapped_row() {»
  - 308-327 @309 «fn board_row_a_board_is_one_write_of_printable_ascii_and_lf() {»
  - 329-343 @330 «fn board_row_width_none_cuts_nothing() {»
  - 345-374 @347 «fn board_row_width_cuts_an_unwrapped_name_to_fit_the_width() {»
  - 376-384 @377 «fn board_row_width_never_cuts_a_wrapped_name() {»
  - 386-419 @388 «fn board_row_width_never_cuts_inside_an_escape() {»
  - 421-438 @422 «fn board_row_width_too_small_keeps_the_captions_four_columns() {»
  - 440-466 @441 «fn board_row_width_an_unwrapped_name_that_fits_is_never_cut() {»
  - 472-476 «fn pending(mut row: Item) -> Item {»
  - 478-490 @479 «fn plain(text: &str) -> String {»
  - 492-515 @496 «fn board_row_style_a_live_row_bolds_its_name_and_ambers_the_dialog_word(»
  - 517-545 @521 «fn board_row_style_a_stale_row_is_dim_end_to_end_and_only_dialog_is_amber(»
  - 547-560 @548 «fn board_row_style_an_unwrapped_row_bolds_only_its_name() {»
  - 562-588 @563 «fn board_row_style_the_context_lines_carry_no_sgr() {»
  - 590-613 @596 «fn board_row_style_keeps_the_plain_boards_columns(#[case] depth: Depth) {»
  - 615-650 @622 «fn board_row_style_an_escaped_control_reads_the_same_at_every_depth(»
### src/human/board/test_support.rs — new file · 37 line(s)
- 7-11 «pub(super) fn at(text: &str) -> DateTime<Utc> {»
  - 8-10 «DateTime::parse_from_rfc3339(text)»
- 15-21 «pub(super) fn counts(unknown_kinds: u64, unknown_fields: u64, torn_lines: u64) -> SkippedTotal {»
  - 16-20 «SkippedTotal {»
- 23-37 «pub(super) fn item(name: &str) -> Item {»
  - 24-36 «Item {»
### tests/contract_fixture_hygiene.rs — added 131 line(s) in 8 range(s)
added: 3-4 · 41-44 · 63-68 · 72 · 76-80 · 334 · 366-369 · 373-480
- 41-43 «fn mcp_schema() -> Value {»
- 63-65 «fn is_mcp(path: &Path) -> bool {»
- 76-79 @77 «fn mcp_fixtures(root: &Path) -> Vec<PathBuf> {»
  - 366-369 «assert_eq!(»
- 373-378 @375 «enum McpViolation {»
- 380-390 «fn texts<'a>(value: &'a Value, out: &mut Vec<&'a str>) {»
  - 381-389 «match value {»
- 392-403 @393 «fn check_mcp(bytes: &[u8], user: Option<&str>) -> Result<(), McpViolation> {»
  - 395-396 «let doc: Value =»
  - 399-401 «if all.iter().any(|text| has_email(text)) {»
- 405-419 @406 «fn mcp_fixtures_pass_hygiene() {»
  - 410-418 «for path in walked {»
- 421-437 @423 «fn mcp_exchange(client_info: &Value) -> Value {»
  - 424-425 «let meta = json!({"_meta": {"io.modelcontextprotocol/protocolVersion": "2026-07-28",»
  - 426-436 «json!({»
- 439-446 @440 «fn planted_clean_mcp_exchange_passes() {»
  - 442-445 «assert_eq!(»
- 448-461 @452 «fn planted_dirty_mcp_exchange_is_rejected(»
  - 457-460 «assert_eq!(»
- 463-479 @470 «fn planted_mcp_schema_violation_is_rejected(#[case] change: Value) {»
  - 472-474 «if let (Some(doc), Some(change)) = (doc.as_object_mut(), change.as_object()) {»
  - 475-478 «assert_eq!(»
