## Output Protocol

You are an iteration agent improving `D:/dev/projects/viola/.andromeda/runs/2026-09-24T01-27-58-tests/test-plan-draft.md`, the Comprehensive (2) test plan for **viola**. viola is a Rust 1.95 / edition 2024 workspace: the root `viola` bin plus `viola-core`, `viola-pty`, `viola-channel`, `viola-state`, `viola-agent-claude`, `viola-mcp` and `viola-ui`. It runs on std threads, with Tokio 1.53.1 only in `mcp`/`ui`. Its stack includes portable-pty `=0.8.1`, interprocess 2.4.4, axum 0.8.9 + SSE, rmcp 3.4.1 and a Lit 3.3.3 web page. Check your findings against:
- `upstream-context.md`, Architecture Excerpt / Stack, Design Excerpt (Brand Identity Anchors) and Layout Templates Excerpt (Signature Placements)
- `test-scope.md`, Sec 1–6
- `test-research.md`, the tool catalog

All three are in the same run dir.

Rules:

1. **Output:** patches (old → new) plus a changelog. Do NOT reproduce the full document.
2. **Patch format:**
   ```
   ### Patch N: <one-line description>
   **Old:**
   <exact text copied verbatim from the current draft, unique enough to locate>
   **New:**
   <replacement text>
   ```
   At most 8 patches per iteration. Spend them in this priority order:
   1. Downstream Readiness
   2. Tool Anchoring
   3. Test Scope Faithfulness
   4. Tier Calibration
   5. Performance Budget Coverage
   6. Cross-Surface Coordination Coverage
   7. Anti-Pattern Relevance
   8. E2E Selector Strategy Robustness

   Dimensions 1–3 are downstream-blocking: patch them first. Dimensions 4–5 are implementation-misleading: patch them only if budget remains. Dimensions 6–8 are signal-diluting: patch them only if the fix is small and cannot wait for a later iteration. Within one dimension, fix contradictions and false claims before gaps, and gaps before wording.
3. **Changelog:** exactly ONE line per iteration that classifies the iteration as a whole:
   `[Iteration N] [substantive|cosmetic] description`
   Never write one line per patch.
4. **PROHIBITED:**
   - reproducing the full document
   - restructuring sections without a concrete defect as the reason
   - labelling cosmetic changes as substantive
   - adding test code blocks longer than 5 lines, or any test-function bodies in any language (that is `/andromeda-implement` work)
   - adding security-plan content (threats, auth flows, crypto choices)
   - adding design tokens or component patterns (design plan)
   - adding obs-plan instrumentation schemas or observability / error-reporting platform picks (cross-references OK)
   - adding ARIA rules or a11y conformance criteria (a11y plan)
   - adding any human-in-the-loop tool or step (visual-review services, manual exploratory testing, recorder-authored tests, GUI-only result viewers)
   - editing historical entries in Section 12 Decisions Log (append only)
5. **If there are no issues:** output "No patches" and a single `[Iteration N] [cosmetic] no issues found` changelog line.

## Analysis Protocol

Follow these steps in order. Do not scan and patch in one pass.

1. **Read** the whole draft once (about 1500 lines) without noting anything. For §1 (lines ~20–420) and §6 (lines ~836–1160), use targeted `offset`/`limit` reads. Do not rely on memory.

2. **Cross-reference.** Walk each of these checks and write down every mismatch before you patch anything:
   - **Harness commands:**
     - Every `viola-harness <subcommand>` and `agent-run <command>` in §6, §9 and §12 must exist in the §3 5-command implementation, with a body, flags, output JSON and exit codes.
     - Watch for `gate` (§9 Quality gates row) and `supervise` (§3 boot step 5).
   - **E2E boot model:**
     - §3 `run` step 2 says the Rust E2E tests self-boot through the rstest chain.
     - Check each `#### Scenario:` Steps list: does it call `agent-run boot --session …` or the rstest `stamped_home` fixture? Does its Cleanup say "harness `cleanup`" or "TempDir drop"? Every scenario must use one model consistently.
   - **Rust ↔ Playwright orchestration:**
     - Every "Playwright:" verification bullet inside a Rust `path_` scenario (Paths 2–6, E1, E3) needs an execution path. §3 `run` runs nextest (step 2) and Playwright (step 3) as separate steps.
     - Playwright's `globalSetup` boots its own `pw-<worker>` session.
   - **Test-scope Sec 5 trigger items:** each item in the §1 Coverage triggers "Required test type" lists must map to a named suite or file prefix in §4 / §5 / §6 with a verification signal. The §2 trigger → layer map alone is not coverage.
   - **Test-scope Sec 1 partially-testable entities:** each must name its boundary function or stub in §4 / §5 and not claim live coverage. Every "asserted at the unit boundary" claim in §6 or §12 must have a matching §4 bullet.
   - **Tool/version, 3 sites:** plan body (§3 / §5 / §6 / §9) ↔ §12 Key decisions ↔ `test-research.md` version and "Agent-runnable" configuration. Every CI-invoked binary also needs an install mechanism and a pinned version in §9 or §3 Bootstrap phases.
   - **Time seams:** every "injected clock" / `mock_instant` / `start_paused` use must be in the same process as the code whose time it controls. The catalog's mock_instant "global mode" is process-global only.
   - **Coverage thresholds:** §10 table ↔ §3 `--coverage` flags ↔ §3 Bootstrap `quality-gate-config-emit` ↔ Comprehensive defaults 85/80/95.
   - **Zero-flakiness:** §10 Zero-flakiness budget (no `test.skip` parking) ↔ §3 `run` step 3 skip semantics ↔ §11 CI/Quality ↔ §9 Build failure conditions.
   - **Ownership pointers:**
     - §3 Log format, §3 Bootstrap phases (`log-format-bind-with-obs`, closing paragraph) and §12 open questions must agree on who owns the harness-grepped log fields and the status shape.
     - The test plan defines these contracts; obs / a11y / route / setup-project read them later.
   - **§12 open questions** ↔ the section that raises each one: every open question must be referenced from the section it affects, and no section may rely on an unresolved open question as if it were resolved.
   - **§11 bans ↔ §9 / §12 decisions:** no §11 ban may contradict a deliberate choice recorded in §9 or §12.

3. **Check each dimension** below, with its anchor example as a calibration of the issue type. Anchors are verified against the current draft. Before patching, re-read the anchored lines, since earlier iterations may already have fixed them.

4. **Out-of-scope discipline:** do NOT patch if a finding would require any of the following:
   - writing specific test code (`#[test]` / `#[tokio::test]` / `#[rstest]` + `fn test_*` / Playwright `test(...)` / `describe(...)` blocks longer than 5 lines: `/andromeda-implement` domain)
   - security-plan content (threats, auth flows, crypto configuration) — security's domain
   - design tokens / component patterns / typography picks (design's domain)
   - obs-plan instrumentation schemas (spans, metrics, traces) — obs' domain
   - accessibility attribute names or specific WCAG conformance claims — a11y's domain
   - naming a concrete observability / error-reporting platform

   Instead, check that the test plan states the boundary requirement: what the test layer must hold, not how it is implemented. Patch only if that boundary is unstated. Also never patch §1 wording that is a verbatim copy of `test-scope.md`. If the copied text is wrong upstream, append a §12 open question instead.

5. **Prioritise** by downstream impact:
   - highest: anything that makes setup-project unable to materialise `scripts/agent-run.{sh,ps1}` / `viola-harness`, or makes a §6 scenario impossible for an agent to run as written
   - then: false tool claims
   - then: coverage holes a green CI would hide
   - last: wording

## Analysis Dimensions

### 1. Downstream Readiness [priority: high]

- **setup-project** builds `viola-harness` from §3 alone.
  - §9 invokes `viola-harness gate` ("reads `junit-*.xml`, `outcomes.json`, …, and exits non-zero on any §10 breach"). §3 boot step 5 spawns `viola-harness supervise --session <id>`. Does §3 give either one a command body, flags, input artifact paths, a `{"v":1,"cmd":…}` output shape and exit codes?
  - `gate` reads `junit-*.xml`, and §9 uploads `junit-${{ matrix.os }}.xml`. But §2 and §3 name the nextest output `target/nextest/ci/junit.xml`, and `.config/nextest.toml` sets `junit.path = "junit.xml"`. Who renames the file?
  - Are `status.last_error` ("e.g. ready-503, list-exit-21, sessions-mismatch") and `run.suites[].suite` closed enums?
  - The §3 Status endpoint prose says "Its nested `list` and `api` members", but the JSON block defines `list` and `ui`. Which is correct?
- **Boot model:** §3 `run` step 2 says Rust E2E tests self-boot through rstest. Paths 2 and 7 call `agent-run boot --session …`, and Paths 3, 4 and 6 clean up with "harness `cleanup`". Which model does each `path_` test use? If it is the harness, how does a nextest process-per-test binary own a session id, and what stops two parallel tests colliding on the default `overseer` / `builder` instances?
- **obs ownership:**
  - §3 Log format says the test plan defines the required fields and "obs-plan may add fields but must not rename or remove these".
  - §3 Bootstrap `log-format-bind-with-obs` tells setup-project to "consume the obs-plan §3 log format JSON schema verbatim".
  - The closing Bootstrap paragraph says both log format and status endpoint shape "come from obs-plan §3". Yet §3 Status endpoint says the nested product shapes come from the arch GUI HTTP contract.

  Only one source of truth may stand. The test plan owns the harness-grepped contract, and obs is a downstream reader. Also: which `event` values carry which `corr` (channel request `id` vs `dialog_id` vs send `cursor`), and is the kebab-case `event` list closed?
- **route:** do cargo-mutants, cargo-fuzz (+ nightly toolchain), hyperfine, cargo-modules, zizmor, jq/jaq and Playwright `install --with-deps chromium` each have a named slot in §3 Bootstrap phases? `test-runner-install` names only cargo-nextest, and `coverage-tooling-install` names only cargo-llvm-cov.

**Anchor example:** Section 3 Test Harness Contract, 5-command implementation, `run` step 2, vs Section 6 Scenario: Path 2, Steps 1

> "The tests self-boot through the rstest home and wrapper fixture chain rather than through the harness session."

> "`agent-run boot --session p2 --instance builder --instance mute:--suppress-prompt-submit --instance local:--local-command-mode --ui`."

**Issue:** §3 says the nextest E2E layer (which includes `path_*`) never uses the harness session. Path 2 step 1 (and Path 7 step 1: "`agent-run boot --session p7 --unstamped --instance builder --ui`.") boots through the harness. Paths 3, 4 and 6 list "harness `cleanup`" as Cleanup. Path 1 and Path 5 use the rstest `stamped_home` / test-owned outer PTY. §3 never says how a nextest test gets its own `--session` id, or whether nested `agent-run` → `cargo run -p viola-e2e` inside a running `viola-e2e` test is allowed.

**Why this matters:** setup-project cannot tell whether `tests/support/` needs a harness-session client or only the rstest chain, and route cannot order `5-command-discipline-wire` against `test-data-bootstrap-wire`. `/andromeda-implement` will write Paths 2–7 two different ways.

**Secondary evidence (verified):**
- §9 Pipeline structure, Quality gates row: "| Quality gates | `viola-harness gate`, which reads `junit-*.xml`, `outcomes.json`, the llvm-cov JSON summary and the hyperfine JSON, and exits non-zero on any §10 breach | n/a | n/a |"
  - Search evidence: I grepped the draft for `viola-harness gate|supervise|harness gate|` `` `gate` ``. There are only two hits: line ~518 (boot step 5, `supervise`) and line ~1257 (§9). Neither is in §3's 5-command bodies, and the §3 `run` flag list (`--unit|--integration|…|--all`) has no `gate`.
- §3 Bootstrap phases, closing paragraph: "command (binding contract with obs' harness contract: log format and
  status endpoint shape come from obs-plan §3)." This contradicts §3 Log format ("obs-plan may add fields but must not rename or remove these. The harness greps on them.") and §3 Status endpoint ("verbatim product shapes from the arch GUI HTTP contract").

**Adversarial:** suppose setup-project follows `log-format-bind-with-obs` literally and waits for an obs-plan §3 schema that does not exist yet. What does `agent-run logs` emit, and do the `jq -e '.record.data.cursor'` assertions in §3 `logs` break silently? Or suppose obs later renames `corr` → `correlation_id` "verbatim from its own §3": which plan wins, and which Decisions Log rule catches it?

### 2. Tool Anchoring (Catalog ↔ Plan) [priority: high]

- **Cross-process time:** Path 6 step 7 advances mock_instant inside the test to expire `budget_override_until` in a separately spawned `viola run builder` process. The §12 open question says tests will drive the send-confirmation window "through the injected clock". The catalog's mock_instant key detail recommends global mode only so that "the wrapper's heartbeat thread sees the advance", which means threads in one process. Resolve each cross-process time claim in one of two ways:
  - document a test-only cross-process seam: a `test-clock` feature on the fake-agent build plus a control input the wrapper reads, which must then be listed in §3 Bootstrap and §8
  - push the assertion down to the §4 root-bin unit (which already lists "`budget_override_until` expiry via mock_instant") and keep only the observable non-time parts in E2E
- **Install and version coverage for every CI-invoked tool:**
  - §9 installs only `cargo-nextest,cargo-llvm-cov,cargo-mutants,cargo-deny` via taiki-e/install-action v2.87.19, matching the catalog.
  - How are these installed, and at which catalog version?
    - hyperfine 1.20.0
    - cargo-modules (catalog 0.27.0; the plan gives no version anywhere)
    - cargo-fuzz 0.13.2 + nightly toolchain
    - jq 1.8.2 / jaq 3.1.1
  - Are cargo-deny 0.20.2 (or cargo-deny-action v2.1.1) and zizmor 1.30.1 (or zizmor-action v0.6.4) chosen, not left as "may alternatively"?
- **Tool fit:** Path 6 step 2 pipes "proptest-generated stdin" but then asserts fixed values (91; 89/86; exactly 90). Should that be fixed synthetic stdin, with proptest left to the §6 Property suite's hook-stdin parser property? Check the same for every E2E step that names a randomised generator.
- **Justified picks:** are the multi-candidate picks justified in §12 with the catalog's Agent-runnable mechanism?
  - eventsource-client 0.18.0 vs the raw reqwest parser (both are used)
  - nextest test groups vs serial_test

**Anchor example:** Section 6 Scenario: Path 6 — budget governor, Steps 7

> "Advance the injected clock (mock_instant) past `budget_override_until` and `send` to `builder` again."

**Issue:** `builder` is a `viola run` child process booted by the harness or a fixture. A `MockClock::advance` in the test process cannot reach the child's clock. The research catalog (mock_instant, Key detail) says: "If mock_instant is used, pick its global mode rather than thread-local, so the wrapper's heartbeat thread sees the advance". Global mode is in-process only. The Path 6 verification bullet "after the clock passes `budget_override_until`, `send` to `builder` exits 11 again" therefore cannot be produced as written.

**Why this matters:** an agent implementing Path 6 will either add a `sleep` (breaking §11 E2E "NEVER use `sleep(N)`" and zero-flakiness) or silently drop the assertion. The same defect lurks behind the §12 open question "Tests drive it through the injected clock and need the constant's location."

**Secondary evidence (verified):** §9 prose: "Tools are installed with taiki-e/install-action v2.87.19 (SHA-pinned) as `tool: cargo-nextest,cargo-llvm-cov,cargo-mutants,cargo-deny`."
- Search evidence: I grepped the draft for `install` (hits only at lines ~537, 653–654, 674, 1250, 1259), for `cargo-modules|cargo modules` (no version at any hit), and for `hyperfine` / `cargo +nightly fuzz` (versions stated, no install step at any hit). No install mechanism exists for hyperfine, cargo-modules or cargo-fuzz.

**Adversarial:** if `--perf` runs on `windows-2025` and hyperfine is absent from the runner image, does `agent-run run --perf` exit non-zero with a JSON reason, or does the `jq -e` gate read a missing `--export-json` file? Would `viola-harness gate` then treat "no perf JSON" as pass or breach?

### 3. Test Scope Faithfulness [priority: high]

- **Orphan trigger items:** three items from the §1 Coverage triggers must each map to a §4 / §5 / §6 suite with a file prefix (`cli_`, `hook_`, `cross_`, …) and a verification signal:
  - Vector 6 "a leading-slash argument is never taken as a prompt (§6 rewritten-path warning)"
  - Vector 6 "no env / flag / config disables a control"
  - multi-version-compat "every ledger row has a `viola verify` probe with a post-condition"

  The §2 trigger → layer map row "CLI and env (V6) | unit + integration | §4 viola-core/root bin; §5" is a pointer, not coverage. Is either V6 item actually in §4 root bin or §5 CLI?
- **Unit-boundary claims:** E2 and the §12 open question say Windows channel-handle non-inheritance "is asserted at the product unit boundary (non-inheritable creation flag)". Does §4 viola-channel list that case? §4 viola-channel currently lists the frame codec, error mapping, FNV-1a vectors and the peer-credential decision only.
- **Entity attributes:** is each attribute that §1 gives an entity asserted somewhere? For example, the GUI token entity's "`ui/<port>.url` (0600, removed on graceful shutdown)": `cleanup` step 5 checks only that the file is absent.
- **Do NOT patch** the §1 tier justification "Four drivers … push the tier to Comprehensive" followed by five bullets. `test-scope.md` Sec 6 has the identical text (verified), so §1 is a faithful copy. If you want it fixed, append a §12 open question for the upstream `test-scope.md`.

**Anchor example:** Section 1 Test Scope Summary, Coverage triggers, security-vector-coverage (CLI and env)

> "    - no env / flag / config disables a control
>     - a leading-slash argument is never taken as a prompt (§6 rewritten-path warning)"

**Issue:** both required negative tests appear only in the §1 trigger list. No §4, §5 or §6 suite plans them.

Search evidence:
- (1) grep `leading-slash|leading slash|rewritten-path|rewritten path|slash command` returns only line ~342.
- (2) grep -i `disables a control|disable|kill.?switch|bypass` returns only line ~341 and the unrelated `bypassCSP` lines ~848 and ~1372.
- (3) I read §4 root bin (lines ~754–760) and §5 CLI (lines ~818–822) end to end: neither mentions slash arguments or control-disabling env/flags/config.

The same holds for "every ledger row has a `viola verify` probe with a post-condition" (line ~404). grep `ledger row|post-condition|probe` finds no suite, and Path 7 step 5 checks only the final `stamped … 0 fail` line, not each row.

**Why this matters:** test-scope Sec 5 triggers are the completeness contract. The phase loop distils §4–§6 per chunk, so a trigger that lives only in §1 is never implemented, and CI stays green while a Vector 6 control has no negative test.

**Adversarial:** suppose a patch adds these as bullets to the §2 trigger → layer map only. Does any §4–§6 suite file or `#[case]` table now own them? If not, the iteration has shuffled a pointer and created no coverage. Also: if the Windows non-inheritance unit case is added to §4, what does it assert without a child-side probe? And is the E2 claim still true on `windows-2025`?

### 4. Tier Calibration [priority: high]

- **Skip semantics:**
  - §3 `run` step 3 says the Playwright suite is "skipped (with the skip recorded as a suite in the summary) where Chromium is not installed".
  - §10 Zero-flakiness says "`#[ignore]` and `test.skip` are not allowed as a parking place".
  - §3 `run` exit semantics are "0 only if every selected suite passes".

  Does a skipped `playwright` suite count as passing? web-spa carries verification for Paths 2–6, E1 and E3. Can `run --all` or `viola-harness gate` exit 0 on ubuntu with the browser suite skipped? For Comprehensive tier, should a skip on ubuntu (the designated browser OS) be a hard failure, with a skip permitted only on OSes where §9 never runs `--browser`?
- **Coverage gate integrity:**
  - Is a 95 % function floor enforced per OS realistic, given `#[cfg(windows)]` / `#[cfg(unix)]` code?
  - Does the plan forbid widening the `--ignore-filename-regex` list (`viola-fake-agent|crates/viola-e2e|tests/support|fuzz/`) as a back door to lowering thresholds? §11 Quality bans only "lower coverage threshold".
  - `--coverage` "replaces step 1+2". Does the E2E layer's `LLVM_PROFILE_FILE` propagation into `viola run` children actually run under `cargo llvm-cov nextest`?
- **Layer double-counting:** the integration filterset `kind(test) & !binary(/^(path|tui|mcp|http|sse|cross|chaos)_/)` does not exclude `contract_`, and the E2E filterset includes `contract_`. Is the contract suite meant to run twice, and does `suites[]` then double-count it?
- **Already stated, do not re-add:** the `unviable` exemption and `"mutants":{"tested":0}` rule are already in §10 Mutation gate (verified).

**Anchor example:** Section 3 Test Harness Contract, `run` step 3, vs Section 10 Zero-flakiness budget

> "This step runs on ubuntu and is skipped (with the skip recorded as a suite in the summary) where Chromium is not installed."

> "`#[ignore]` and `test.skip` are not allowed as a parking place."

**Issue:** the harness has a built-in, environment-driven skip for the only web-spa driver. Neither §3 exit semantics nor §9 Build failure conditions says whether `"suite":"playwright","skipped":n` fails `run` / `gate`. §10 bans skipping at test level but not at suite level, so a missing `npx playwright install` step on ubuntu turns every web-spa verification bullet into a silent pass.

**Why this matters:** for the tier that justifies Playwright, the cross-surface trigger and E3 "flips same frame" coverage can vanish from a green CI. Agent-driven development reads exit 0 as "done".

**Adversarial:** suppose the Chromium install step fails on a `ubuntu-latest` image bump. Walk the chain: §3 `run` step 3 → `suites[]` → `viola-harness gate` → §9 Build failure conditions. Which step turns red? If none does, which one sentence in §3 or §10 closes the hole without adding a retry?

### 5. Performance Budget Coverage [priority: high] [trigger: performance-budget trigger in test-scope Sec 5 AND test_tier=Comprehensive]

- **Column semantics:** the §10 table keeps p50 / p95 / p99 headers, but fills p50 / p95 with "reported" and redefines p99 as `max`. Should the columns be renamed (for example `Trend (hyperfine JSON)` | `Gate`) so downstream agents do not implement a literal p99?
- **Gate determinism:** with `--runs 30` on shared `windows-2025` / `macos-latest` runners, is a `max` gate deterministic under zero-flakiness? Is a `--warmup` count stated, and a noise rule that is not a retry?
- **Binary under test:** which binary does hyperfine time? `boot` builds a debug `cargo build --workspace --features fake-agent`, and §9 runs coverage instrumentation in the same matrix. Does the plan state that the perf gates run against a non-instrumented build, and at which profile?
- **Real-time smoke:** the SSE row's "real-time smoke sees one within 16 s per OS" (also E4) waits on real time. Is it reconciled with §11 Universal "NEVER use real time without injection"? Is it tied to a named suite and exit code, or should it be dropped in favour of the paused-clock gate alone?
- **Missing perf paths:** are these perf-critical paths from test-scope Sec 4/5 missing from the table, and if so, are they deliberately excluded in §12?
  - `send` → `prompt-submitted` confirmation latency
  - `wait` wake latency after `turn-ended`
  - `boot`'s 20 s / 10 s readiness deadlines
- **Provisional spine gate:** is there a rule that the provisional spine gate (`1.0 s`) is replaced by reading the product constant, not by editing a literal, once arch request (3) lands?

**Anchor example:** Section 10 Performance budgets, preamble and SSE row

> "The gate is on the `max` sample: Claude Code hook deadlines are hard cutoffs. p50 and p95 are reported for trend only. The p99 column equals `max` over the listed runs."

> "gate: a keep-alive comment is emitted at `advance(15 s)`; the real-time smoke sees one within 16 s per OS"

**Issue:** the table header promises percentiles that the gate never computes. The SSE row adds a real-time wait of up to 16 s on each OS, but no suite file, exit code or injection is named for it. §11 Universal says "NEVER use real time without injection (use deterministic clocks)".

Search evidence: grep -i `warmup` returns 0 hits. grep `--release` returns only the §9 Release build row (line ~1256). grep -i `instrument` returns only lines ~406 and ~1447, neither of which is about the perf build. So neither warm-up nor a non-instrumented perf profile is stated.

**Why this matters:** a debug-build or instrumented-build `max` on a noisy Windows runner can breach `1.0 s` intermittently. Under "no retries" that becomes an unfixable red chunk, or pressure to loosen the gate.

**Adversarial:** suppose `pre-tool-use` on an unverified CLI takes 0.4 s median but has one 1.1 s outlier from Windows Defender scanning the freshly built exe on its first run. With no warm-up, does the plan's gate fail the chunk? What rule, stated in §10 and not as a retry, makes the verdict deterministic?

### 6. Cross-Surface Coordination Coverage [priority: medium] [trigger: cross-surface-coordination in test-scope Sec 5 AND multiple surfaces in Sec 2]

- **Orchestration:** how do Rust scenario tests and Playwright share one product session? Path 2 step 5 says "Playwright (spec `bay-steady-state`) watches the tape while steps 2 and 4 run". But:
  - Rust `path_` tests run in §3 `run` step 2 (nextest).
  - Playwright specs run later in step 3, and `globalSetup` boots a separate `pw-<worker>` session.

  The same applies to the Playwright verification bullets in Paths 3, 4, 5 and 6, E1 and E3. Pick one mechanism and state it in §6 (not as code):
  - a Playwright spec that drives the CLI and channel steps itself against its harness session
  - a Rust test that drives Chromium through a documented bridge
  - moving the DOM assertions into per-scenario specs keyed to the same fake-agent script
- **Driver and signal per surface:** does every surface a scenario lists have both a driver step and a transition signal?
  - Path 3 lists MCP `wait`, but only step 7 calls rmcp `wait`, on a dead instance. No live MCP `wait` wake is asserted.
  - Path 7 renders `cli_verified:false` (the CLI column) but omits web-spa from its surfaces.
- **Parity timing:** is `status.api_sessions_equal_list` asserted at the end of every multi-surface scenario or only in `cross_list_parity`? Do any parity checks belong at integration speed (axum-test + assert_cmd over one tempfile home) so that a failure localises before E2E?

**Anchor example:** Section 6 Scenario: Path 2, Steps 5, vs Section 3 `run` step 3

> "Playwright (spec `bay-steady-state`) watches the tape while steps 2 and 4 run."

> "Playwright `globalSetup` calls `agent-run boot --session pw-<worker>` and `globalTeardown` calls `agent-run cleanup --session pw-<worker>`."

**Issue:** steps 2 and 4 run inside a Rust `path_` binary in `crates/viola-e2e` during nextest (§3 `run` step 2). The Playwright spec lives in `e2e-web/tests/*.spec.ts` and runs afterwards (step 3), against its own `pw-<worker>` session and home. No mechanism is stated that lets the spec observe session `p2`'s tape while the Rust steps execute. The Path 2 bullets "the builder send line's `viola-readback` goes `data-rb="open"` → `"read"`" and the mute `data-rb="refused"` have no runnable owner.

**Why this matters:** the cross-surface-coordination trigger ("a `send` shows `[RB]` on the CLI and `data-rb="read"` on both the tape line and the outbound transfer marker") is the plan's main multi-surface guarantee. If the orchestration is undefined, `/andromeda-implement` will either drop the web half or write sleep-coupled cross-runner tests.

**Adversarial:** suppose each Playwright spec re-creates a scenario's CLI steps through `child_process` against its own session. Does the web-spa row then prove the same event crossed CLI → channel → disk → SSE → DOM? Or does it only prove two independent runs look alike, which is exactly the "separate fixtures" failure that cross-surface parity is meant to catch?

### 7. Anti-Pattern Relevance [priority: medium]

- **Template bans that conflict with the plan:** §11 still carries generic bans that contradict deliberate decisions:
  - CI "NEVER skip caching of dependencies" vs §12 "no npm or Playwright browser cache in CI"
  - CI "NEVER run sequentially when you can parallelize" vs the `fixed-port` group (`max-threads = 1`) and the single-job mutation / fuzz stages
  - Test Strategy "NEVER violate test pyramid" with no link to viola's layers

  Rewrite each one into a viola-scoped ban, or reconcile it with the Decisions Log.
- **Duplicated bans:** keep one canonical ban in §11 and use pointers elsewhere:
  - `.env_clear()` + `LLVM_PROFILE_FILE` appears in §3 Bootstrap `coverage-tooling-install`, the §6 drivers table, §10 Stack adjustments and §11 Integration.
  - `bypassCSP` / `toHaveScreenshot` appears in the §6 drivers table and §11 E2E.
  - `retries = 0` appears in §2, §3, §10 and §11.
- **Missing viola-specific bans:** these pitfalls are supported by the plan's own content but have no §11 ban (verified absent):
  - using a `claude agents --json` row `name` as a target
  - asserting any -32603 message other than `"internal error"`
  - letting `run` / `gate` pass with the browser suite skipped
  - widening the coverage exclusion regex
  - controlling another process's time with `mock_instant`
  - asserting a multi-surface critical path on one surface only

**Anchor example:** Section 11 Test Anti-Patterns § CI, vs Section 12 Key decisions, Browser caching

> "- NEVER skip caching of dependencies (slows agent feedback loop)"

> "**Browser caching:** no npm or Playwright browser cache in CI."

**Issue:** the §11 ban forbids exactly what the §12 decision (and the §9 E2E row "npm and browsers are not cached; see Decisions Log") chose on purpose. An agent that obeys §11 will add an npm / Playwright cache that no researched action supports.

Search evidence for the missing bans (read §11 lines ~1335–1423 end to end, then grepped within that range):
- `agents`: 0 hits
- `ignore-filename|exclu`: 0 hits
- `mock_instant|another process|cross-process`: 0 hits
- `32603|internal error`: 0 hits
- `surface`: 0 hits
- `skip`: only cleanup, caching and quality-gate lines, none about skipped suites

**Why this matters:** contradictory bans make §11 untrustworthy, and agents stop reading it for the bans that matter, such as the R7 screen-parsing ban and the blanket-approval ban.

**Adversarial:** suppose the iteration only deletes the contradictory generic bans and adds no viola-specific replacement. Does §11 CI still have a single stack-grounded ban about caching (for example, rust-cache only in `ci.yml`), or does the domain shrink to bans already enforced elsewhere? Which §11 domain would then have no viola-specific ban at all?

### 8. E2E Selector Strategy Robustness [priority: medium] [trigger: E2E Section 6 present AND web-spa in test-scope Sec 2]

- **Already verified, do not patch:** every tag, attribute and copy string the plan uses is anchored in `upstream-context.md` Brand Identity Anchors / Signature Placements. This covers `viola-session-row`, `viola-atis`, `viola-transfer`, `viola-event-feed`, `viola-readback`, `data-dialog`, `data-liveness`, `data-wrapped`, `data-rb`, `gate open` | `budget-paused`, `unable · <reason> · <detail>` and `DIALOG <name> · viola`.
- **Cell targeting:** the gap is how individual cells are selected. The §6 Selector strategy lists roles `table`, `row`, `columnheader`, `log`, `status`, but not `cell`, which upstream lists (`role="cell"`). Meanwhile:
  - Path 3 asserts "the same STATUS cell transition"
  - Path 5 asserts "the WHEEL cell reads `human`"
  - Path 4 asserts "cell text `DIALOG question`"

  Is there a stated rule (for example, a cell located by row + its columnheader index) so that column-order drift fails one place? Is there a fallback convention for elements with no role and no signature anchor?
- **Synchronisation:** upstream defines `data-live` on SSE arrivals. Does the plan use it (or Playwright auto-waiting) for tape synchronisation, consistent with §11 E2E "NEVER use `sleep(N)`"?
- **axe scope:**
  - `AxeBuilder({page}).analyze()` with `violations == []` runs axe's full default rule set. Does "It guards the role-query contract; the rule set is owned by the a11y plan" hold, or does the test plan de facto gate a11y conformance?
  - The Security sweep says axe runs "over every GUI route", which includes JSON routes. Is that intended?

  Patch only the boundary statement. Do not add rules or WCAG claims.

**Anchor example:** Section 6 E2E Test Strategy, Selector strategy, vs Scenario: Path 5 Verification signal

> "- **Role-based queries** follow the design Brand Identity Anchors: `getByRole('table')` for racks, `'row'` for strips, `'columnheader'` for the six captions, `'log'` for the tape and `'status'` for the live region."

> "- Playwright: the WHEEL cell reads `human`, then `driver`."

**Issue:** the scenarios assert on specific cells (WHEEL, STATUS, DIALOG), but the selector strategy never says how a cell is addressed. `role="cell"` is anchored upstream (Brand Identity Anchors, ARIA roles), yet it is absent from the plan's role list, and no row × column rule is given. Implementers will fall back to positional `nth()` or the CSS classes (`.band`) that §11 bans.

Search evidence:
- (1) grep `getByRole|role="cell"|'cell'|nth(|column index|data-testid` returns only line ~859 (no `cell`).
- (2) grep -i `cell` returns only verification bullets (lines ~926, 953, 978) and unrelated CLI / §12 lines.
- (3) I read §6 Selector strategy (lines ~858–864) end to end.

**Why this matters:** WHEEL / STATUS / DIALOG cell assertions carry Paths 3–5 on web-spa. An unstated addressing rule produces brittle selectors that break when design reorders or adds a column, which the zero-flakiness rule then turns into red chunks.

**Adversarial:** suppose design adds a seventh caption, or the 760–1023 px layout wraps each strip to two lines (upstream Navigation). Which Path 3/4/5 cell assertions keep passing because they key on `columnheader` name, and which silently read the wrong cell because they key on position?
