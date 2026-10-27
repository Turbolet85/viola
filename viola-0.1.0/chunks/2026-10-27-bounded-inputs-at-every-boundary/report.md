# Report — 2026-10-27-bounded-inputs-at-every-boundary

**Chunk:** every external reader size-capped, depth-limited and closed-typed, names validated before any path join, the paste rule on both sides and a terminal-parser panic degraded to input-not-ready
**Date:** 2026-10-27T06:25:09Z
**Commits:** `607792cb chore(2026-10-27-bounded-inputs-at-every-boundary): operator pre-CI commit, for the run this chunk's verdict reads` (the one commit since `last_wrap` 2026-10-27T03:56:09Z that is this chunk's; `4b0bf776`, the last wrap's own commit, is its parent and the base of every reading here — `git log --format='%h %s' 4b0bf776..HEAD`)

## Changes (structured — detectors read this)
- **Files:** new `crates/viola-mcp/src/stdin.rs` · new `tests/contract_bounded_readers.rs` · modified `crates/viola-mcp/src/lib.rs` · `crates/viola-state/src/fs.rs` · `schemas/diag-line.v1.json` · `deny.toml` · `scripts/deny-probes.sh` · `tests/chaos_feed_panic.rs` · `crates/viola-e2e/tests/mcp_tools.rs` (9 files: `gate.py scope` at P1 — `scope: clean — changed 9 · listed 9 · recorded 0`); the ledger `viola-0.1.0/verification-matrix.json` (v1-46 `ref` set, `implemented`). `src/run/gate.rs` was edited only by step 7's two forced-behaviour controls and restored each time (implement's outcome, deviation 6; not in the changed set).
- **Symbols / APIs:**
  - `viola_mcp::stdin::CappedLines<R>` (`pub(crate)`, `crates/viola-mcp/src/stdin.rs:17-29`, its impls `:31-73` and `AsyncRead` `:75-109`; new-text listing): wraps an async byte source and yields whole lines of at most the cap bytes, the `\n` counted; a line of exactly the cap passes; a longer line releases no byte, sets the cut flag (`cut_flag()` → `Arc<AtomicBool>`, `:44-47`) and the reader then reads as end of stream; at the inner end a held unterminated line (≤ cap) is released. The cap is a constructor argument (`new(inner, cap: u64)`, `:32-42`).
  - `viola_mcp::serve` (`crates/viola-mcp/src/lib.rs`): the transport is now `(CappedLines::new(stdin, MAX_FRAME), stdout)` from `rmcp::transport::stdio()` (rmcp's `(reader, writer)` transport, the `transport-async-rw` path `transport-io` already enables — no rmcp feature change, `scripts/rmcp-features.sh` reads `server transport-io`). After the session ends, when the cut flag is set, `serve` writes exactly one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` at WARN through `obs_event!` (new private fn `reject_oversize_line`, `lib.rs:93-102`), no byte of the line, then returns what the session gave: `Ok(())` for an initialized session (the `mcp` verb's `process-exit` 0, exit 0, as at a closed stdin), `ServeError::Initialize` (exit 11) when the cut lands before `initialize` completes (deviation 1). Its one product caller `src/cmd/mcp.rs:36` is unchanged. Its doc comment now reads "until stdin closes or a stdin line passes `MAX_FRAME`, which ends the session as a close does".
  - `viola_state::fs::replace_private_shared` (`crates/viola-state/src/fs.rs`): its fallback compare now goes through the new private `holds_exactly(path, bytes)` (`fs.rs:359-368`), which opens the target and reads it through `take(bytes.len() + 1)` (saturating), never the whole file; the outcome equals a whole-file compare for an equal, a longer, a shorter, a different and an absent target. Callers unchanged (`pin_and_plugin`, `src/cmd/run.rs:464`; `pin_companions`).
  - No IPC method, MCP tool, endpoint, env var, CLI flag, exit code or error variant added (`ServeError` unchanged).
- **Crates / modules:** `viola-mcp` gains module `stdin` (crate-private). No crate added or removed.
- **Dependencies:** none added or bumped (`Cargo.lock` untouched — the guard entry's `guard: clean`, its allow list excluding `Cargo.lock`).
- **Schema / config:** `schemas/diag-line.v1.json` (`:87`): the `parse-rejected.parser` enum gains `mcp-stdin` (now `claude-agents-json · vt100-feed · config-json · ledger-stamps · hook-stdin · channel-frame · mcp-stdin · prompt-submitted · paste-text · state-entry`); `detail` already held `oversize`. `deny.toml` (`:58-62`): a `[[bans.features]]` table `crate = "serde_json"`, `deny = ["unbounded_depth"]`, reason "every parse of external bytes keeps serde_json's default recursion limit". No `config.json` key.
- **Spec-master edits:** none.
- **Retired:** none.
- **Counts / qualifiers moved:**
  - deny probe verdict `deny-probes: 13/13 banned, control clean` → `deny-probes: 14/14 banned, control clean` (the rule: the `probes=(` rows of `scripts/deny-probes.sh`, now 13 at lines 20–32, plus the planted channel probe `channel-tokio-edge`; measured by implement's entry 14 and by the pre-push run) — pattern `probe-count`: 4 standing · 4 changed (`security-plan-detail/dependency-security.md:14` · `test-plan.md:794` · `test-plan-detail/ci-integration.md:15` (also "the twelve bans one probe each" → thirteen) · `registries/contracts/architecture/build-system.md:8` @c3044) · leaf 1 (`.claude/docs/commands.md:60`, the cascade's).
  - the `parse-rejected` `parser` catalog: 9 → 10 values (`mcp-stdin` added; schema `:87`) — pattern `parser-catalog`: 3 standing · 1 changed (`obs-plan-detail/log-coverage.md:20`, the parser list) · 1 changed (`log-coverage.md:34`, its per-parser details gain `oversize` (`mcp-stdin` past `MAX_FRAME`)) · 1 no-change (`architecture.md:177`, the channel's own `channel-frame` line, a true claim); pattern `parse-counts`: 3 standing · 1 changed (`obs-plan-detail/obs-scope-summary.md:500`, the per-parser parse-failure counts list gains the MCP stdin) · 2 no-change (`log-coverage.md:34`, counted above; `metric-coverage.md:27`, the counter keyed `parser`, no list).
  - the new enumeration's tables: `BOUNDARIES` 40 rows, `EXCEPTIONS` 2 rows (the new-text listing: 40 `Boundary {` blocks at `tests/contract_bounded_readers.rs:30-389`, `EXCEPTIONS` `:392-405`) — a new count, no master site (pattern `enumeration`: 1 standing, `security-plan.md:330`, a Bootstrap-phase line "Add the `MAX_FRAME` const and the `Read::take` wrappers on every external reader" — no-change, true).
  - the WATCH tally on Path 1's CI timeout: 0/3 → 1/3 green — pattern `path1-watch`: 1 standing (`test-plan-detail/e2e-test-strategy.md:38`, the Path 1 binary's landed line, no tally there) · no-change; the tally lives on the working route's WATCH (P5).
  - Sweep reach: 14 patterns · 11 moved facts, 11 with a pattern (the report's own count: the facts listed under this bullet, Symbols / APIs and Spec claims disproved, each naming its id below) · owner pattern control unfired (the refusal: `control: owner-title, owner-generic, mcp-stdin never fired over the seven masters' pre-pass text, bodies and sets (607792cb)`) · by line 2 lines · 3 offsets. Total line: `total (14 patterns) · 243 rows over 67 files` (trail `cascade-2026-10-27-bounded-inputs-at-every-boundary-pre-pass.json`).
- **Dev-tool versions:** none.
- **Harness / gate surface:** `scripts/deny-probes.sh` gains the probe row `serde_json-unbounded-depth` (`:32`; its diagnostic `error[feature-banned]: feature 'unbounded_depth' for crate 'serde_json = `), its verdict line moving to `14/14` (above). No agent-run verb, CI step or status shape changed; the new E2E binary `contract_bounded_readers` runs under `run --e2e` (a `contract_` binary, the E2E layer).
- **Cross-project / external claims:**
  - rmcp 3.4.1 (crates.io source, read at phase P3 in the cargo registry): `AsyncRwTransport::receive` reads each stdin line with an unbounded `read_until` (`src/transport/async_rw.rs` lines 125–140: `read_until` at 137, `Ok(0) => return None` at 140), and `JsonRpcMessageCodec::new_with_max_length` serves only the writer side (`FramedWrite`) — the basis for bounding the reader viola hands rmcp (plan §Constraints; research §Files inspected).
  - CI: `ci#40458923095` over `607792cb8178` (the pre-CI commit; this wrap's commit adds records on top of that tree) — `verdict: green · checks 15/15 · wall 719 s`, conclusion success (`evidence/operator-pass.md`, entry 27).
  - inputs (`inputs.py verify`, wrap P1: `inputs: 4 entries — unchanged 3 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 1 · uncited 2 · unparsed 0`; the two uncited entries, I3 and I4, are cited here):
    - `I1 · ../additional/viola-overseer/phase-bounded-inputs-directive.md · copy no-repo · unchanged` — the phase directive (ordinary chunk, the Path 1 WATCH carried and tallied, `df -h /tmp` before a stamped-home gate).
    - `I2 · ../additional/viola-overseer/phase-bounded-inputs-ruling-g1.md · copy no-repo · unchanged` — overseer1's ruling under the founder's night word: the one-off second re-spawn of design-cascade-g1, and the rmcp unbounded `read_until` on `viola mcp` stdin named this chunk's own subject.
    - inputs#I3 · `message: the operator relaying the overseer, overseer1 founder-delegated, 2026-10-27 05:34Z` · copy message · n/a — the P5 word «yes» on `.andromeda/runs/2026-10-27T04-04-46-phase/p5-card-1.md` (the newest and only card): the plan, the v1-46 claim less the resume-cursor clause and the Beyond-the-plan CARRY approved; authority: delegate overseer1 (founder-delegated), PROVISIONAL until the founder's own word (and so restated by inputs#I4 item 5). Its `## Beyond the plan` rows, verbatim:
      - A CARRY on «Windows-only live measurements»: after a contained vt100 feed panic under a 1-column ConPTY, read how the child's panicking wide character (`e4 b8 ad`) reaches the outer terminal on a Windows host, and give `tests/chaos_feed_panic.rs` its Windows byte oracle from that reading (this chunk asserts the bytes on Unix and only viola's literal absence on Windows).
    - inputs#I4 · `../additional/viola-overseer/wrap-bounded-inputs-directive.md` · copy no-repo · unchanged (snapped at this wrap, `--step wrap`) — the operator's wrap directive: an ordinary chunk with no forced sample; every transcript and the phase, implement and wrap run dirs kept; stop at a hard cascade failure, or at agents writing nothing or doing the orchestrator's work (F352), with no re-spawn around it; a check-7 `contradiction` escalates as written (amendment-flow :295), raised at the P2 halt for the operator, the last wrap's five self-applied rows no precedent; from implement's outcome, an owner (or a note in the masters' test-plan, as the skill decides) for the uncovered pre-initialize stdin cut and for the five boundary rows whose witness never drives past the cap; the Path 1 WATCH tally recorded at 1/3 green (15.010 s); the P5 approval PROVISIONAL; the wrap run whole, stopping only at a 95 % context alarm.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - `.andromeda/architecture-detail/established-decisions.md:23` ([Screen Model]) states that a caught vt100 panic and a message the full queue drops each give `parse-rejected{parser:"vt100-feed", detail:"oversize"}` per poisoning episode. The code writes `panicked` for a caught panic (`src/run/gate.rs:49` `parse_rejected("panicked")`, asserted by the unit case at `src/run/gate.rs:443`) and `oversize` for a dropped copy (`src/run/gate.rs:72`, asserted at `:487`); `security-plan-detail/input-validation.md:64`, `obs-plan-detail/error-capture-reporting.md:31` and `obs-plan-detail/log-coverage.md:34` already say so. Plan-carried (Expected amendments, architecture). Pattern `screen-poison`: 15 standing · 1 changed (`established-decisions.md:23`) · 14 no-change (true claims of the poisoning without the detail, or naming it correctly: `architecture.md:4`, `established-decisions.md:25,211,213`, `security-plan.md:160`, `input-validation.md:64`, `threat-model-summary.md:148` @c2638 and @c2909 (Run B's wait rows — no-change, no-change), `e2e-test-strategy.md:355`, `obs-plan.md:535`, `error-capture-reporting.md:18,31`, `log-coverage.md:34`; and `security-plan.md:282,308`, zizmor's cache poisoning, another subject).
- **Expected amendments (from plan):**
  - security-plan §Input Validation — a `viola mcp` stdin row; the Constants list gaining `viola mcp`'s stdin and `replace_private_shared`'s compare read; the own-executable reads named as no external input — carried: Symbols / APIs (`serve`, `replace_private_shared`), the enumeration's `EXCEPTIONS` (Coverage of new surfaces). Search: `mcp-stdin` 0 sites in any master (its control unfired — no master names the MCP stdin; the row is new text); `constants` security-plan 2 (`security-plan.md:176`, `input-validation.md:90` — both change: the list gains the two reads); `compare-read` security-plan 3 (`data-protection.md:14,18,28` — no-change, true claims of the replace's behaviour, the read's bound stated at the Constants sites), architecture 2 (`architecture.md:321`, `established-decisions.md:53` — no-change, same reason); `own-exe` security-plan 4 (`authentication-authorization.md:44,45`, `data-protection.md:17`, `input-validation.md:32` — no-change, the reads' purpose; the exception is new text beside Constants), architecture 5, obs-plan 6, a11y-plan 1 (no-change, the same `checked_exe` / `pin_exe` mechanism named for the home-tree check); `mcp-surface` security-plan 12 (`authentication-authorization.md:27,66`, `bootstrap-phases.md:15`, `data-protection.md:71`, `error-handling.md:12,13`, `input-validation.md:31,34,53` @c752, `security-anti-patterns.md:9`, `threat-model-summary.md:56,127` — no-change, the frame tools' instance checks; `input-validation.md:34`, the home resolution of the hidden `viola mcp`, is the nearest neighbour for the new row).
  - security-plan §Dependencies — the serde_json `unbounded_depth` feature ban and the deny probe count `13/13` → `14/14` — carried: Schema / config (`deny.toml`), Counts / qualifiers moved (`probe-count`), Harness / gate surface. Search: `depth-ban` security-plan 3 (`security-plan.md:136` — no-change, the channel row's "never `unbounded_depth`" stands; `security-plan.md:452` "NEVER enable serde_json's `unbounded_depth` feature." — change: now a `deny.toml` feature ban with a firing probe; `input-validation.md:17` — no-change); `probe-count` security-plan 1 (`dependency-security.md:14` — change), test-plan 2, architecture 1 (the registry key `build-system.md:8` — change).
  - obs-plan §6 Log Coverage — `parse-rejected`'s `parser` gains `mcp-stdin` (a Decisions Log entry) with `oversize` its one detail, and the per-parser parse-failure counts list gains it; obs-plan §1/§4's MCP surface rows name the bounded stdin — carried: Schema / config, Counts / qualifiers moved (`parser-catalog`, `parse-counts`). Search: `parser-catalog` obs-plan 2 (both change), architecture 1 (no-change); `parse-counts` obs-plan 3 (1 change); `mcp-surface` obs-plan 16 (`obs-plan.md:125` the §1 Surface line "ipc-internal (`viola mcp`: stdio MCP server, Tokio allowed)" — the site for the bounded stdin; the other 15 — `error-capture-reporting.md:25,37`, `log-coverage.md:37,38,44`, `obs-scope-summary.md:398`, `pii-scrubbing-compliance.md:39`, `slo-invariants-telemetry-budgets.md:11,17`, `span-trace-coverage.md:201,202`, `telemetry-strategy.md:33`, registry `logging-stack.md:10`, `otel-sdk-init.md:16,17` — no-change, the frame tools' and `list` tool's lines). The Decisions Log entry is the obs detector's to write.
  - architecture §Established Decisions [MCP] — the stdin bound; [Screen Model] — the `parse-rejected` detail of a poisoning episode `panicked` for a caught panic and `oversize` for a dropped copy — carried: Symbols / APIs (`serve`), Spec claims disproved (`screen-poison`). Search: `mcp-stdin` 0; `mcp-surface` architecture 15 (`architecture.md:153,327,351`, `conventions.md:65`, `established-decisions.md:234,256,271`, `occupied-resources.md:71,123,141 @c996,161,217`, `stack-and-technologies.md:27`, `standard-contracts.md:93`, registry `project-directory-structure.md:29` — no-change; [MCP]'s own body is where the bound is new text, its line the detector locates); `screen-poison` architecture 5 (1 change, above).
  - test-plan §5 / §6 — `tests/contract_bounded_readers.rs` (the enumeration), the `mcp_tools` oversize case, the chaos test's passthrough half; test-plan §9 — the deny probe count — carried: Files, Symbols / APIs, Coverage of new surfaces, Counts / qualifiers moved. Search: `enumeration` test-plan 0 (new text); `mcp-e2e` test-plan 8 (`e2e-test-strategy.md:14,47,75,100,133,184,203`, registry `bootstrap-phases-derive-for-route-setup-project.md:10` @c1773 — no-change: the Paths 2–6 legs and the stamped-session list; the oversize case starts from its own fresh home, beside `mcp_tools_a_widened_home_exits_1_and_prints_nothing`, which already did); `chaos-panic` test-plan 4 (`test-plan.md:431,694`, `integration-test-strategy.md:40`, registry `test-data-bootstrap.md:12` — no-change, the outside-scan home); `passthrough` test-plan 16 (`test-plan.md:227` "forced vt100 panic → `not-delivered`/`input-not-ready` with passthrough continuing" and `:616`, `e2e-test-strategy.md:336` "human passthrough still reaches the receipt" — change candidates: the passthrough now also asserted on the outer terminal (Unix bytes, every-OS literal absence); the other 13 — `test-plan.md:25,58,102,471`, `e2e-test-strategy.md:12,161,162,353,358`, `test-data-fixtures.md:43,69`, `test-scope-summary.md:52`, registry `bootstrap-phases-derive-for-route-setup-project.md:10` @c2262 — no-change, the statusline pass-through or the tui surface); `probe-count` test-plan 2 (change).
  - a11y-plan §1 — the tui boundary's panic case now witnessed for the passthrough (Unix bytes; every OS literal absence), its Windows byte reading owed — carried: Symbols / APIs (none; the test), Coverage of new surfaces, the P5 card's CARRY. Search: `chaos-panic` a11y-plan 0; `passthrough` a11y-plan 8 (`a11y-plan.md:33` and `a11y-scope-summary.md:40`, the tui Entity lines — the sites for the witness; `a11y-plan.md:55,145`, `a11y-scope-summary.md:8 @c1124,102`, `visual-design-verification.md:8`, registry `keyboard-test-harness.md:3` @c5010, @c5082, @c5277 (the statusline pass-through test — no-change ×3) — no-change).
  - The other masters' `passthrough` rows (architecture 14, security-plan 5 — `security-plan.md:160` and `input-validation.md:64` already state the passthrough continues —, design-system 2, layout-templates 3, obs-plan 5) and `mcp-surface` rows (design-system 1, layout-templates 3, test-plan 9, a11y-plan 1): no-change — true claims sharing the token (the statusline pass-through, the tui surface, `viola mcp` having no human surface).
- **Coverage of new surfaces:**
  - `viola mcp` stdin line cap (`CappedLines` at `MAX_FRAME`) → validation ✓ (a line past the cap passes no byte and ends the session; refuse whole, never truncate) · instrumentation log ✓ (one `parse-rejected{mcp-stdin, oversize, count:1}` WARN via `obs_event!`) · PII redacted ✓ (no byte of the line; the E2E case reads a canary for in every home-level file) · tests unit (6 `stdin::tests` cases: `capped_lines_*`, `stdin.rs:188-240`) / integ (the child case `serve_ends_the_session_at_a_line_past_max_frame`) / e2e (`mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line`, `mcp_tools.rs:441-559`) · a11y n/a (no human surface) · tokens n/a
  - `replace_private_shared` compare read bound → validation ✓ (`take(bytes.len()+1)`) · instrumentation n/a (no new line) · PII n/a · tests unit (`replace_private_shared_reads_no_more_than_one_byte_past_the_bytes`, `fs.rs:385-419`) · a11y n/a · tokens n/a
  - serde_json depth ban → validation ✓ (`deny.toml` feature ban) · tests probe (`deny-probes.sh` row; `serde_json_unbounded_depth_is_banned_in_deny_toml`) · others n/a
  - the bounded-reader enumeration and census (`tests/contract_bounded_readers.rs`: `boundaries_each_name_a_bounded_read_that_stands`, `product_readers_are_each_bounded_or_named`, `census_finds_an_unbounded_read_and_passes_a_bounded_one`, `serde_json_unbounded_depth_is_banned_in_deny_toml`) → tests e2e ✓ · others n/a. Five `BOUNDARIES` rows name a witness that reads the bounded read but drives nothing past its cap: the status-line command's stdout (`statusline_shell_out_passes_the_output_through_byte_for_byte`), the child pipe drain shared by `claude --version` and `claude agents --json` (`run_bounded_returns_the_exit_and_stdout`), the three `config.json` readers for the thresholds, the hold and `claude_env_keep` (their type-rule tests) — their cap held by the needle row and the census alone (deviation 2).
  - the tui passthrough after a feed panic (`chaos_feed_panic_refuses_send_and_keeps_passthrough`, `tests/chaos_feed_panic.rs:109-130`) → tests e2e ✓ (Unix: the child's `e4 b8 ad` reach the outer terminal within `WITHIN`; every OS: none of `panic`, `parse-rejected`, `vt100`, `unable`, `hint:` on it) · a11y literal-absence ✓ on every OS, byte reading `unrunnable-here` on Windows (ConPTY re-renders a wide character on a 1-column screen; owed by the P5 card's CARRY on «Windows-only live measurements»)

## Deviations from intent
From implement's outcome (`.andromeda/runs/2026-10-27T05-37-33-implement/outcome-2026-10-27-bounded-inputs-at-every-boundary.md`), each with its justification:
1. **Step 2, a cut before `initialize` completes.** As built the line is written whenever the reader recorded a cut; a cut landing before the session initialized keeps `ServeError::Initialize` (exit 11), what a closed stdin before `initialize` gives — the cut read "as stdin's close". Not in the acceptance and **driven by no test** (the uncovered pre-initialize stdin cut, inputs#I4 item 4: owner given at P5).
2. **Step 8, five rows whose witness does not drive the cap** (listed under Coverage of new surfaces): each names the closest existing test, as step 8 asks; no test was added outside the lists; the needle row and the census hold their caps (inputs#I4 item 4: owner given at P5, or a note in test-plan through the P2 detectors).
3. **Step 8, rows beyond the plan's listing** — the plan's list is "the scope, not the count": the table splits the server connection beside the frame reader, the child pipe drain, the vt100 feed panic beside its channel bound, verify's shell-verdict file, and the two capture arms and the `--answers` file as rows of their own (40 rows).
4. **Step 8, the exception match** — the census judges "bounded" over the statement up to the call, and matches an `EXCEPTIONS` needle against the call's whole source line, because the statement ends at the call's `(` and the executable argument the needle names lies outside it.
5. **Step 5, the case's shape** — `holds_exactly` is checked on every OS against an equal, a longer, a shorter, a different and an absent target; the read-only-dir refusal of a longer target is Unix-only, matching the existing case's stand-in (Windows uses a read-only file instead).
6. **Controls outside the lists** — `src/run/gate.rs` edited for step 7's two forced-behaviour controls and restored from a copy each time (`git diff --stat src/run/gate.rs` empty afterwards).

scope record: none — gate.py scope clean, 0 recorded (`scope: clean — changed 9 · listed 9 · recorded 0 (companion 0 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 538`).

## Decisions & corrections
- The operator's direction for the implement run (its invocation, 2026-10-27): every test or control that spawns `viola run`, `viola mcp` or a wrapper gets a kill deadline, each long run is journalled before it is waited on, and process listings are read whole. Applied: the serve child kills at 5 s, the `mcp_tools` child at 8 s; the controls ran under `timeout` and nextest's `ci` profile; `journal.md` holds entries 1–2; every census read whole (implement's outcome, Decisions & corrections).
- rmcp 3.4.1's stdio transport reads each line with an unbounded `read_until`, and `JsonRpcMessageCodec::new_with_max_length` serves only its writer: a bound on rmcp's input belongs to the reader viola hands it, never to the codec (plan §Constraints; research; inputs#I2 named it this chunk's subject).
- An over-long MCP line is refused whole and ends the session — never truncated (a padded valid request would be dispatched) and never skipped (the peer's request would go unanswered with no signal); the wrapper channel's close-on-oversize rule is the precedent (plan §Constraints, leaned on architecture §Standard Contracts).
- The reader signals the cut as end of stream, never as an I/O error, so rmcp writes nothing of its own (its `tracing::error!` is OFF in viola's sink anyway); the one line is viola's (plan §Implementation notes).
- A Windows byte oracle for the panicking wide character is not assertable here: ConPTY re-renders the child's output and a wide character on a 1-column screen has no guaranteed rendering — the reading is owed to «Windows-only live measurements» (P5 card's Beyond-the-plan row, inputs#I3, PROVISIONAL).
- A census hazard: a statement-scoped `take(` check cannot see an exception's argument past the call's `(`, so the exception needle is matched against the whole source line (deviation 4).
- Guard refusals: bash-guard's `cd` arm refused a `cd` into the rmcp registry during implement, re-issued with absolute paths (one re-issue); at this session's new-session the same arm refused a `cd` into a skill references dir (one re-issue).
- A hook refusal: the PostToolUse stamp-ahead hook refused an evidence stamp written from an estimated time (+9 min), corrected from `date -u`; a second estimated stamp, written by a python script that fires no hook, was corrected after a clock read (+3 min) — an evidence stamp is read from `date -u`, never estimated (implement's outcome).
- The `andromeda:walks-tree` token was written into the new census test, since it walks the product sources (implement's outcome).
- The phase's P2 (p5-card-1 §P2 notes): design-cascade-g1 halted on `BODY_UNDER_ENTITY` after its one retry; overseer1's ruling (inputs#I2) ran a one-off second re-spawn, which passed; the ruled re-spawn turned five sibling brand-identity items into runs under the one item named — the check prints only its first row, so the other four were the same defect.
- The P5 approval is PROVISIONAL (delegate overseer1) until the founder's own word (inputs#I3; restated by inputs#I4 item 5).
- The wrap directive (inputs#I4): check 7 `contradiction` verdicts escalate as written, even when a companion proposal rewrites the contradicted line; the last wrap's five self-applied rows (F359) stand as logged but are no precedent.
- A hook refusal at this wrap: the stamp-ahead hook refused this report's own `Date` written from an estimate 14 min ahead of the clock, corrected from `date -u` (2026-10-27T06:25:09Z) — the same estimated-stamp hazard implement met, recurring.
- A host fact met at this session's start: python's `yaml` module is absent on the Linux dev host, so `state.yaml` was read by eye at new-session's check 12.

## Outcome
Acceptance criteria, re-asserted against the diff:
- (security) v1-46 — every external reader size-capped, depth-limited and closed-typed, names validated before any path join, the paste rule client- and wrapper-side, a terminal-parser panic degraded to `input-not-ready` without harming passthrough (its resume-cursor clause out of scope per its notes, the founder's ruling of 2026-10-16): **met** — `contract_bounded_readers` green (entry `run --e2e --filter 'binary(contract_bounded_readers)'`: nextest-e2e 11 passed, 0 failed), every boundary row's named witness green in the block (entries `run --unit`, the integration filter, the e2e filter). Five rows' witnesses read the bound without driving past it (deviation 2) — the criterion as written ("every boundary row's named witness green") holds; the gap is a test-depth gap, routed at P5.
- (security) a `viola mcp` line past `MAX_FRAME` ends the session after the earlier lines are answered — exit 0, stderr empty, stdout only the earlier frames, one `parse-rejected{mcp-stdin, oversize, count:1}` holding no byte: **met** (`mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line` and the child case, green on all three CI OSes, `evidence/operator-pass.md` table). A cut before `initialize` exits 11 (deviation 1) — outside this criterion's "after the lines before it are answered".
- (security) exactly `MAX_FRAME` with its `\n` and `MAX_FRAME − 1` pass whole, `MAX_FRAME + 1` is cut across any split: **met** (`stdin::tests`, `capped_lines_at_max_frame_passes_the_cap_and_cuts_one_byte_more` with the literal `16 * 1024 * 1024`).
- (security) `replace_private_shared` reads at most `bytes.len() + 1`, outcome a whole-file compare's: **met** (`replace_private_shared_reads_no_more_than_one_byte_past_the_bytes` and the two existing cases; the census red at exactly `crates/viola-state/src/fs.rs:354` with step 5 undone — `evidence/controls.md`).
- (security) `deny.toml` bans serde_json `unbounded_depth`, `cargo deny check` clean, `deny-probes: 14/14 banned, control clean`: **met**.
- (security) the census reads every product reader bounded in-statement or an `EXCEPTIONS` row, its self-check reads a planted unbounded read as a hit: **met**.
- (a11y) after a forced feed panic: no viola literal on the outer terminal on every OS, the child's `e4 b8 ad` on Unix, `send` refused `input-not-ready` (exit 13), a typed key reaching the child: **met** (green on ubuntu, macOS and windows-2025; Windows reads the literal absence only, its byte reading owed to «Windows-only live measurements»).
- (obs) the schema admits `parse-rejected{parser:"mcp-stdin"}` and the step-4 line validates against it; G2 zero panics outside the seam exemption and the chaos home: **met** (entry `g2-zero-panics.sh` `g2: clean`).
- (tests) the paste rule's client and wrapper witnesses, the hook stdin fail-open matrix, the raw-channel names, the frame cap on both sides, the MCP handlers' re-validation and the closed config values stay green: **met** (the integration entry, 184 passed).
- (tests) `run --unit` and pre-push green, coverage per OS at or above 85 / 95 / 80: **met** (`run --unit` 2575 passed; pre-push `ok:true`, coverage 3251 passed, 0 failed, gate breaches none).

Gates (implement's first firing, `entries 27 · green 23 · red 0 · recorded 1 · timeout 0 · not-run 3`, trail `.andromeda/runs/2026-10-27T05-37-33-implement/gate-2026-10-27-bounded-inputs-at-every-boundary.json`; the three not-run entries are the operator's, run by the operator pass):
- `df -h /tmp` — recorded (`tmpfs 32G 11G 21G 35% /tmp`)
- `cargo fmt --all --check` — green · `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` — green · `CARGO_TARGET_DIR=target/wincheck cargo clippy … --target x86_64-pc-windows-msvc -- -D warnings` — green · the G1 `rg … '#\[(tracing::)?instrument\b(?!\(\s*skip_all\b)' .` — green (exit 1, no output) · `bash scripts/lint-probes.sh` — green
- `bash scripts/agent-run.sh run --unit` — green (2575 passed, 0 failed)
- `cargo nextest run -p viola-mcp -E 'test(/(^|::)stdin::tests::/) | test(/(^|::)tests::serve_/)'` — green (13 tests run: 13 passed, 63 skipped) · `cargo nextest run -p viola-state -E 'test(/(^|::)tests::replace_private_shared/)'` — green (3 passed, 305 skipped)
- `cargo nextest run --workspace --features fake-agent -E 'binary(channel_paste_validation) | … | binary(hook_statusline)'` — green (184 passed, 0 skipped)
- `bash scripts/agent-run.sh run --e2e --filter 'binary(chaos_feed_panic) | binary(mcp_tools) | binary(contract_diag_schema) | binary(chaos_snapshot_replay) | binary(chaos_forward_compat)'` — green (nextest-e2e 49 passed, 0 failed) · `… --filter 'binary(contract_bounded_readers)'` — green (11 passed, 0 failed)
- `cargo deny check` — green · `bash scripts/deny-probes.sh` — green (`deny-probes: 14/14 banned, control clean`) · `cargo deny --manifest-path crates/viola-state/Cargo.toml --config deny-sync.toml check bans` — green · `bash scripts/rmcp-features.sh` — green (`rmcp-features: server transport-io`) · `gate.py guard …` — green (`guard: allowed 9 … · changed 9 · excluded 0`, then `guard: clean`)
- smoke: `agent-run.sh cleanup --session p-bounded-smoke` — green · `boot --session p-bounded-smoke --unstamped --instance builder` — green · `status` — green · `bash scripts/g2-zero-panics.sh` — green (`g2: clean`) · `agent-run.sh schema-check` — green (`files 55, lines 154, failures []`) · `cleanup` — green (`processes_gone`, `endpoint_gone` true). The boot path changed (step 5 runs at every start): implement's P3 smoke drove a booted home by hand — boot ready, status ready, one live `viola mcp` (`initialize`, `notifications/initialized`, `tools/list`, stdin closed: exit 0, stderr empty, two result frames with the five tools, `mcp.ndjson` holding only `process-start` and `process-exit`), cleanup `processes_gone`, `endpoint_gone`, `home_removed` true.
- `bash scripts/agent-run.sh pre-push` — green (`ok:true`, stage `linux-tests`, coverage 3251 passed, 0 failed, breaches none); re-run by the operator pass on the uncommitted tree, green the same.
- `gate.py hygiene` (leg operator) — green by the operator pass (`hygiene: clean — read 529 (runs 523 · evidence 1 · inputs 5) · …`) · `git diff --quiet && git diff --cached --quiet && git push origin HEAD` (leg operator) — green (`history moved: refs/remotes/origin/build/viola-0.1.0 4b0bf776→607792cb`) · `ci.py conclusion --sha HEAD --wait 2400` (leg operator) — green (`607792cb8178 verdict: green · checks 15/15 · wall 719 s · runs ci#40458923095 completed/success`), every OS-sensitive case PASS on ubuntu, macOS and windows-2025 (`evidence/operator-pass.md`).
- No deferral: Rust delta 7 files, every gate ran.

Watches:
- Path 1's CI timeout (`path1_run_start_one_home_in_order`, killed at the ci profile's 20 s line on ubuntu in an earlier run) · 1 green run [ci#40458923095: PASS 15.010 s on `test (ubuntu-latest)`, `SLOW [> 10.000s]`, under the 20 s kill; macOS 6.927 s, windows-2025 6.418 s] · tally 0/3 → 1/3, no cause claimed for the slower time (the earlier greens 13.468 s, 8.219 s; the red 20.089 s); no case of this chunk touches Path 1's start sequence (inputs#I4 item 4).

Outcome basis: the operator pass ran — Setup 4's commit list `607792cb` alone (no fix commit), the final HEAD's CI run `ci#40458923095` recorded in `evidence/operator-pass.md` (the record P7 re-verifies); implement's report (`outcome-2026-10-27-bounded-inputs-at-every-boundary.md`, the one `*-implement/` dir holding this marker's gate trail) for what only it holds — its deviations, its controls (`evidence/controls.md`: steps 1–2 6 of 13 cases red with the guard removed, the child case killed at its 5 s bound; step 5 the `same+` row red; step 7 both halves red; step 8 the census red at `fs.rs:354` with step 5 undone), its smoke and its census.

Process hygiene: implement's census (its outcome table): every test child and both harness sessions terminated by their own runs; 8 `viola` processes of other trees left running (the overseer pair's bridge and sessions — not this chunk's). Re-measured at this wrap by `gate.py census --comm viola --comm viola-fake-agent --comm viola-harness --args projects/viola/target` (this run dir): `census: 8 running`, every one under another tree (`~/.local/viola-product`, `~/dev/projects/additional/viola-lab/prototype`, the `~/.viola` pinned `mcp`); none from this tree.
## New text, by line
Generated by `cites.py added` (cites v1.7); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 4b0bf776 (the parent of the oldest pre-CI commit 607792cb) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### crates/viola-e2e/tests/mcp_tools.rs — added 134 line(s) in 1 range(s)
added: 427-560
- 427-439 @428 «fn files_under(dir: &Path) -> Vec<PathBuf> {»
  - 430-437 «for entry in std::fs::read_dir(dir).expect("read_dir") {»
- 441-559 @446 «fn mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line() {»
  - 455-466 «let mut child = Command::new(bin_dir.join(format!("viola{}", std::env::consts::EXE_SUFFIX)))»
  - 470-474 «let out = std::thread::spawn(move || {»
  - 475-479 «let err = std::thread::spawn(move || {»
  - 481-498 «let writer = std::thread::spawn(move || {»
  - 500-510 «let status = loop {»
  - 515-519 «assert_eq!(»
  - 522-526 «let frames: Vec<Value> = String::from_utf8(stdout)»
  - 532-535 «let rejected: Vec<&Value> = lines»
  - 540-542 «let schema: Value = serde_json::from_slice(»
  - 545-548 «let errors: Vec<String> = validator»
  - 551-558 «for file in files_under(&home) {»
### crates/viola-mcp/src/lib.rs — added 81 line(s) in 9 range(s)
added: 8 · 14 · 19-20 · 63-64 · 76-87 · 89 · 93-103 · 130-131 · 144-192
- 93-102 @94 «fn reject_oversize_line() {»
  - 95-101 «obs_event!(»
  - 147-156 «fn child(stdin: Stdio) -> std::process::Child {»
  - 158-163 @160 «fn serve_on_a_closed_stdin_is_the_initialize_error() {»
### crates/viola-mcp/src/stdin.rs — new file · 241 line(s)
- 17-29 @18 «pub(crate) struct CappedLines<R> {»
- 31-73 «impl<R> CappedLines<R> {»
  - 32-42 «pub(crate) fn new(inner: R, cap: u64) -> Self {»
  - 44-47 @45 «pub(crate) fn cut_flag(&self) -> Arc<AtomicBool> {»
  - 49-51 «fn fits(&self, len: usize) -> bool {»
  - 53-72 @54 «fn take_in(&mut self, mut bytes: &[u8]) {»
- 75-109 «impl<R: AsyncRead + Unpin> AsyncRead for CappedLines<R> {»
  - 76-108 «fn poll_read(»
- 111-241 @112 «mod tests {»
  - 120-125 @121 «struct Pieces {»
  - 127-153 «impl AsyncRead for Pieces {»
  - 155-180 @156 «fn lines_through(pieces: Vec<Vec<u8>>, cap: u64) -> (Vec<Vec<u8>>, bool) {»
  - 182-186 «fn line(len: usize) -> Vec<u8> {»
  - 188-200 @194 «fn capped_lines_passes_a_line_within_the_cap_whole_and_cuts_a_longer_one(»
  - 202-206 @203 «fn capped_lines_yields_a_fitting_line_then_ends_at_an_over_long_one() {»
  - 208-215 @209 «fn capped_lines_restarts_the_count_after_each_newline() {»
  - 217-221 @218 «fn capped_lines_cuts_an_over_long_line_split_across_reads() {»
  - 223-227 @224 «fn capped_lines_joins_a_fitting_line_split_across_reads() {»
  - 229-240 @230 «fn capped_lines_at_max_frame_passes_the_cap_and_cuts_one_byte_more() {»
### crates/viola-state/src/fs.rs — added 49 line(s) in 4 range(s)
added: 7 · 354 · 359-369 · 385-420
- 359-368 @361 «fn holds_exactly(path: &Path, bytes: &[u8]) -> bool {»
  - 362-364 «let Ok(file) = File::open(path) else {»
  - 385-419 @389 «fn replace_private_shared_reads_no_more_than_one_byte_past_the_bytes() {»
### deny.toml — added 5 line(s) in 1 range(s)
added: 58-62
### schemas/diag-line.v1.json — added 1 line(s) in 1 range(s)
added: 87
### scripts/deny-probes.sh — added 1 line(s) in 1 range(s)
added: 32
### tests/chaos_feed_panic.rs — added 22 line(s) in 1 range(s)
added: 109-130
  - 110-121 @113 «{»
  - 123-130 «for literal in ["panic", "parse-rejected", "vt100", "unable", "hint:"] {»
### tests/contract_bounded_readers.rs — new file · 774 line(s)
- 14-24 @15 «struct Boundary {»
- 29-390 «const BOUNDARIES: &[Boundary] = &[»
  - 30-38 «Boundary {»
  - 39-47 «Boundary {»
  - 48-56 «Boundary {»
  - 57-65 «Boundary {»
  - 66-74 «Boundary {»
  - 75-83 «Boundary {»
  - 84-92 «Boundary {»
  - 93-101 «Boundary {»
  - 102-110 «Boundary {»
  - 111-119 «Boundary {»
  - 120-128 «Boundary {»
  - 129-137 «Boundary {»
  - 138-146 «Boundary {»
  - 147-155 «Boundary {»
  - 156-164 «Boundary {»
  - 165-173 «Boundary {»
  - 174-182 «Boundary {»
  - 183-191 «Boundary {»
  - 192-200 «Boundary {»
  - 201-209 «Boundary {»
  - 210-218 «Boundary {»
  - 219-227 «Boundary {»
  - 228-236 «Boundary {»
  - 237-245 «Boundary {»
  - 246-254 «Boundary {»
  - 255-263 «Boundary {»
  - 264-272 «Boundary {»
  - 273-281 «Boundary {»
  - 282-290 «Boundary {»
  - 291-299 «Boundary {»
  - 300-308 «Boundary {»
  - 309-317 «Boundary {»
  - 318-326 «Boundary {»
  - 327-335 «Boundary {»
  - 336-344 «Boundary {»
  - 345-353 «Boundary {»
  - 354-362 «Boundary {»
  - 363-371 «Boundary {»
  - 372-380 «Boundary {»
  - 381-389 «Boundary {»
- 392-405 @394 «const EXCEPTIONS: &[(&str, &str, &str)] = &[»
  - 395-399 «(»
  - 400-404 «(»
- 407-415 «const READ_CALLS: [&str; 7] = [»
- 417-425 «const PRODUCT_ROOTS: [&str; 7] = [»
- 427-429 «fn workspace() -> PathBuf {»
- 431-433 «fn read(path: &Path) -> String {»
- 435-444 «fn rs_files(dir: &Path, out: &mut Vec<PathBuf>) {»
  - 436-443 «for entry in fs::read_dir(dir).into_iter().flatten().flatten() {»
- 446-452 @447 «fn relative(path: &Path, root: &Path) -> String {»
  - 448-451 «path.strip_prefix(root)»
- 454-469 @456 «fn product_files(root: &Path) -> Vec<(String, String)> {»
  - 458-460 «for dir in PRODUCT_ROOTS {»
  - 461-468 «files»
- 471-549 @473 «fn code_only(text: &str) -> Vec<u8> {»
  - 476-482 «let blank = |out: &mut Vec<u8>, from: usize, to: usize| {»
  - 485-547 «while i < bytes.len() {»
- 551-556 @552 «fn raw_string_opening(rest: &[u8]) -> Option<(usize, usize)> {»
- 558-574 @559 «fn char_literal_len(rest: &[u8]) -> Option<usize> {»
  - 560-563 «if rest.get(1) == Some(&b'\\') {»
  - 564-570 «let text = std::str::from_utf8(&rest[1..rest.len().min(6)])»
- 576-606 @577 «fn without_test_items(code: &mut [u8]) {»
  - 580-605 «while let Some(found) = code[at..].windows(marker.len()).position(|w| w == marker) {»
- 608-612 @609 «struct Hit {»
- 614-618 «impl Hit {»
  - 615-617 «fn bounded(&self) -> bool {»
- 620-648 @621 «fn reader_calls(text: &str) -> Vec<Hit> {»
  - 625-632 «for call in READ_CALLS.map(str::as_bytes) {»
  - 635-647 «opens»
- 650-657 @651 «fn defined_functions(root: &Path) -> String {»
  - 653-655 «for dir in ["src", "crates", "tests"] {»
- 659-687 @660 «fn boundaries_each_name_a_bounded_read_that_stands() {»
  - 664-679 «for b in BOUNDARIES {»
  - 680-685 «for (file, needle, why) in EXCEPTIONS {»
- 689-717 @690 «fn product_readers_are_each_bounded_or_named() {»
  - 694-708 «for (file, text) in product_files(&root) {»
  - 709-712 «assert!(»
- 752-761 «fn census_finds_an_unbounded_read_and_passes_a_bounded_one(»
  - 756-759 «let got: Vec<(usize, bool)> = reader_calls(text)»
- 763-774 @764 «fn serde_json_unbounded_depth_is_banned_in_deny_toml() {»
  - 766-772 «let banned = deny.split("[[bans.features]]").skip(1).any(|table| {»
