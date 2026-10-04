# Fan-out results — 2026-10-04-readiness-gate-and-timing-constants

Seven Explore doc-agents, one parallel batch, the amendment-flow prompt verbatim (contracts line for architecture,
test-plan, obs-plan, a11y-plan; dropped for the three un-migrated docs). Returns arrived as YAML (with `#` comment
notes after the list on four docs; the notes were read and are summarised in each verdict line). No HTML entities in
any return (`&lt;`/`&gt;`/`&amp;` absent — probe by reading: entities=0). No raw twin: no `proposals: []` return was
changed by stripping beyond its trailing comment notes, and every return parsed.

## Verdicts
- **design-system** — `proposals: []`. Notes: every new surface in Coverage carries `tokens n/a`; no UI element.
- **layout-templates** — `proposals: []`. Notes: no user-facing surface; `viola run` stays passthrough (layout-templates.md:362, :493-502).
- **a11y-plan** — `proposals: []`. Notes: no interactive element; tee byte-identical (a11y-plan.md:94-99, :184-191 hold); no schema change.
- **security-plan** — 1 proposal. Notes: D-security-deps and D-security-auth clean; sweep found :245, :588, :591 still true; FYI :354 (G2's one exemption) is `:74`'s question.
- **obs-plan** — 2 proposals. Notes: Tee no per-chunk span (§11 :1117); `run.readiness_gate` span is `:74`'s; parse-rejected matches the §6 catalog; §10 / G2 panic budget is `:74`'s question.
- **test-plan** — 8 proposals.
- **architecture** — 9 proposals.

## Proposals and dispositions
Playbook rule applied to every proposal below unless noted: **"Accurate this-chunk addition"** (routine — the named
thing is this chunk's per the report's Changes; the invariant survives). Check 2 (cross-contradiction): none — the
test-plan §3 key file takes two proposals on different clauses (fuzz seeds :52, perf bound :149). Check 3
(intent-consistency): the report's deviations are justified in-intent (step 11's third site, `start`'s handle,
`Screen::size`, the economy witness the overseer confirmed); scope record empty, `gate.py scope` clean.

### security-plan
S1. D-security-input (escalate by detector) · §Input Validation → PTY output bytes (vt100) row (:238) — record the
landed degrade: Tee + feed thread, catch_unwind on feed and resize, poisoned until a size change, one parse-rejected
vt100-feed/panicked WARN per poisoning, the measured small-size triggers, `host_size` filters zero sizes, the unbounded
tee→feed mpsc an open item owed to `:74`. → **apply** (routine: accurate this-chunk addition; NOT a boundary widening —
the feed reads the same PTY output bytes the row already governs, no new crossing; the open item is the overseer's
recorded direction (1)).

### obs-plan
O1. D-obs-instrumentation · §7 → Error classes captured (:915) — the vt100 panic is caught on run's feed thread, not in
`run.readiness_gate`; one parse-rejected per poisoning; the panic hook's `event:"panic"` + detail lines also written
once per poisoning; `send-refused{input-not-ready}` and the G2 question are `:74`'s. → **apply** (routine; the
panic-hook clause is written with its epistemic status — read in `src/main.rs:138-181` (research M3), not witnessed at
run level).
O2. D-obs-instrumentation (dependent-of O1) · §7 → Panic hooks → `run` worker threads (:901) — name the feed thread as a
contained worker (catch per feed/resize, no Err to main). → **apply** (routine; same claim).

### test-plan
T1. D-tests-coverage · §6 cargo-fuzz paragraph (:1028) — `vt100_feed` joins (6 seeds, guard pair). → **apply**.
T2. D-tests-coverage (dep T1) · §2 Property-based row (:434) — target list gains `vt100_feed`. → **apply**.
T3. D-tests-coverage (dep T1) · §3 → 5-command-implementation (:52) — fuzz-replay seed list gains `vt100_feed`: 6. → **apply**.
T4. D-tests-coverage · §6 Property suite (:1025) — vt100 feed property marked landed (512 cases, its test id). → **apply**.
T5. D-tests-obs-harness · §3 → 5-command-implementation (:149) — perf bound `viola_core::SPINE_DEADLINE`; `SPINE_DEADLINE_S` retired. → **apply**.
T6. D-tests-obs-harness (dep T5) · §10 Spine deadline (:1220) — provisional wording retired; `CONNECT_DEADLINE` asserted below. → **apply**.
T7. D-tests-obs-harness (dep T5) · §10 perf table spine-hooks row (:1226). → **apply**.
T8. D-tests-obs-harness (dep T5) · §10 perf table pre-tool-use row (:1227). → **apply**.

### architecture
A1. D-arch-decisions · [Screen Model] (:48) — mechanism as landed; QUIET_PERIOD / GATE_MAX_WAIT and the signature
format PROVISIONAL; rows HELD → `:82` / founder ruling; `verdict` consumer `:74`. → **apply** (routine; directions (2),(3)).
A2. D-arch-decisions (dep A1) · §Stack Screen model row (:18) — feed thread teed from the pump output, not the pump thread. → **apply**.
A3. D-arch-decisions (dep A1) · [CLI Version Compatibility] (:91) — screen-signature rows HELD, not landed with the gate. → **apply** (direction (2)).
A4. D-arch-decisions · [Delivery Confirmation] (:49) — `CONFIRM_WINDOW_FALLBACK` = 10 s PROVISIONAL. → **apply** (direction (3)).
A5. D-arch-decisions · [Hook Transport] (:64) — `viola_core::SPINE_DEADLINE` named; hook 750 ms = `CONNECT_DEADLINE`. → **apply**.
A6. D-arch-decisions · §Infrastructure Patterns → crate-dependency-direction — viola-agent-claude depends on vt100 =0.16.2. → **apply**.
A7. D-arch-resources · Occupied Resources `fuzz/` (:413) — 4th target `vt100_feed` (6 seeds). → **apply**.
A8. D-arch-resources (dep A7) · project-directory-structure `fuzz_targets` line (:67). → **apply**.
A9. D-arch-resources · project-directory-structure — viola-core gains SPINE_DEADLINE + Clock/SystemClock; viola-agent-claude
the screen module; `src/run/gate.rs`. → **apply** (the tree is a CATEGORY-grain register, but its per-crate comments
already enumerate content of this grain — `RefusalReason, ViolaName, Percent` — so this is not registry over-reach).

### Orchestrator raises (Validate checks 5 and 6)
R1. check 5 · test-plan §7 Fake agent Modes line (:1076) — `--vt100-panic-bytes` still lands with `:74`'s chaos case; its
bytes are now measured (a 24×1 PTY + a wide char such as `e4 b8 ad`). → **apply** (routine; report Expected amendments).
R2. check 6 · test-plan §3 → bootstrap-phases-derive-for-route-setup-project (:15) — the 21 half-removed repos' cause is
now measured two-sided (detached `git maintenance`), and the fixture repos run with `maintenance.auto=false`. → **apply**
(routine; report Spec claims disproved, evidence/carry4.md).

Check 5 coverage (every plan Expected-amendments entry): [Screen Model] A1 · [Delivery Confirmation] A4 · [Hook
Transport] A5 · directory structure A8/A9 · test-plan §10 T6-T8 · §7 Fake agent R1 · §6 Property suite T1/T4 · obs §7 O1 ·
security PTY row S1 · v1-21 — nothing to write. Check 6: libfuzzer-sys premise — report-only (no master states it);
CARRY 4 — R2.

Escalations: 0.
