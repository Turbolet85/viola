# Fan-out results — 2026-10-04-confirmed-send-with-cl-1-records (wrap 2026-10-04T06-44-39)

Seven Explore doc-agents, one parallel batch, the amendment-flow prompt sent verbatim (report
`viola-0.1.0/chunks/2026-10-04-confirmed-send-with-cl-1-records/report.md`; keyed-contract renders for architecture,
test-plan, obs-plan, a11y-plan in this run dir). Every return was YAML after its leading `#` comment lines; no HTML
entity appeared (entities=0 by read); no raw twin warranted.

## Verdicts
| doc | proposals | stripped commentary (substance) |
|---|---|---|
| architecture | 21 | — |
| security-plan | 10 | D-security-deps no drift (serde / serde_json already in the stack, deny green); notes a Security Decisions Log record owed for the newly served `send` method (routed: rides the F3 escalation + its sidecar entry) |
| design-system | 0 | D-design-tokens: every new surface `tokens n/a`; expected amendment 10 is a wording change outside the detector |
| layout-templates | 0 | D-layout-surface: `viola send` maps to §Output structure — `viola send` (:405-425) and §Component — Hero (:519-525); the mirror lines, exit 21 line, wrapper-fault line and `--json` docs already stated (:409-422, :542-545); `unconfirmable` held (F2) |
| test-plan | 10 | D-tests-framework, D-tests-obs-harness no drift |
| obs-plan | 9 | — |
| a11y-plan | 0 | D-a11y-surface: no interactive element; §5 tui / §6 / §8 already match; D-a11y-obs-schema: schema unchanged |

## Proposals and dispositions
Disposition key: **apply** (routine) · **escalate E{n}** · **reject**. The check that decided it follows.

### architecture
- A1 D-arch-resources · §Conventions → Naming patterns, normalised event kinds (:163) · add `send-issued` · `send-confirmed` · `send-refused` → **apply** (playbook: Accurate this-chunk addition; Changes "Event records").
- A2 dependent-of A1 · §Standard Contracts → Event `data` per kind (:289-300) · the three send shapes; `prompt-submitted.origin` `driver` set only by the wrapper relabel → **apply** (Accurate this-chunk addition).
- A3 dependent-of A1 · §Standard Contracts wrapper-appended paragraph (:302) · send-* are wrapper-appended, log-only, never wake `wait` → **apply**.
- A4 D-arch-resources · §Standard Contracts → Channel methods `hook.event` (:285) · the driver relabel on an exact text match, settle after the append → **apply** (Accurate this-chunk addition; Changes "Driver relabel").
- A5 dependent-of A4 · [Delivery Confirmation] (:49) · exact-match relabel; no local-command rows → `not-delivered` until the rows land (F2) → **apply** (expected amendment 4; F2 is a hold, not a widening).
- A6 D-arch-resources · §Conventions → `RefusalReason` (:133) · `control-character` joins the `not-delivered` details, `validate_paste_text` both sides → **apply** (expected amendment 1, pre-authorised 2026-10-01).
- A7 D-arch-resources · §Conventions → `send` refusal order (:136) · control-character first; in-flight = `turn-running`; paste failure after issue = `input-not-ready` with cursor → **apply** (expected amendment 1; P4 lean, P5-reviewed).
- A8 dependent-of A7 · [Human Takeover / Wheel] (:70) · `turn-running` also covers one send in flight → **apply**.
- A9 D-arch-resources (escalate) · §Standard Contracts → CLI `--json` output (:308) · the contract fixes `{"v":1,"error":"wrapper-fault","detail":{"code","message","data"}}`; `viola send --json` shipped `{"v":1,"error":"wrapper-fault","code":<int>}` → **escalate E3** (check 3 intent-consistency: an unjustified divergence from a Standard Contract MCP is bound to mirror; the report calls `code` "this chunk's choice").
- A10 dependent-of A9 · §Conventions → MCP wrapper-fault `structuredContent` (:149) → **escalate E3** (atomic with A9).
- A11 D-arch-resources · §Conventions → CLI exit `1` (:139) · `send`'s unparseable reply; the `Send` arm prints `error: internal error` on `Err`; a panic in `viola send` prints no stderr line until `src/main.rs` maps `send` to the `cli` role → **apply** (Accurate this-chunk addition; the owner is a route pin at P5 per the overseer's pre-direction, never body text).
- A12 D-arch-resources · §Conventions → CLI exit `2` (:140) · `viola send` over-cap / non-UTF-8 text with its fixed line → **apply**.
- A13 D-arch-resources · §Occupied Resources → Repository `fuzz/` (:413) · targets 4 → 5 (`paste_text`), the corpus byte-exact under `.gitattributes` → **apply**.
- A14 dependent-of A13 · §Infrastructure Patterns → `project-directory-structure` (key file :68) · `fuzz_targets/{…,paste_text}.rs` → **apply** (the report names the site).
- A15 D-arch-resources · §Infrastructure Patterns → `project-directory-structure` (key file :17-21) · `src/run/send.rs`, `send` a caller of `src/human.rs` → **apply** (expected amendment 5; at apply also `src/cmd/send.rs` and the viola-pty `PasteHandle` seam, which the expected amendment names — re-derived from the report).
- A16 D-arch-resources · §Occupied Resources → Filesystem (:401-402) · register `<temp dir>/viola-chaos-*` (the F4 home) → **escalate E2** (F4 founder-visible; the record presumes the carve-out is accepted).
- A17 dependent-of A16 · §Occupied Resources → Repository `target/e2e-home` (:416) · "every harness and test home" gains the F4 exception → **escalate E2**.
- A18 D-arch-decisions · [Screen Model] (:48) · "falls back to delivery confirmation only" → the partial gate (`verdict(None)`) → **apply** (expected amendment 3; F1, the overseer's: "lands as an architecture [Screen Model] amendment at the wrap, not a widening"; report Spec claims disproved).
- A19 dependent-of A18 · Design Philosophy (:4) · "degrades to transport-only" gains the partial gate → **apply**.
- A20 D-arch-decisions · [Screen Model] (:48) · the bounded feed (256) and overflow poisoning → **apply** (expected amendment 3).
- A21 D-arch-decisions · §Infrastructure Patterns → `crate-dependency-direction` (key file :2) · viola-core gains serde (+ dev serde_json) → **apply** (Accurate this-chunk addition; serde already in §Stack).

### security-plan
- S1 D-security-input · §Input Validation → PTY output bytes (vt100) (:238) · the "Open: unbounded mpsc" clause → the bound as built → **apply** (Accurate this-chunk addition — the validation the row mandates is present; expected amendment 6 "the PTY output row's bound closed"; the detector's `escalate` is its default severity, the playbook decides).
- S2 D-security-input · §Input Validation → Paste text (:222) · `validate_paste_text -> Result<(), NotDelivered>` and the as-built sites → **apply** (Accurate this-chunk addition).
- S3 D-security-input · §Input Validation → Constants (:243) · `viola send`'s reader joins the `MAX_FRAME` consumers → **apply** (expected amendment 7).
- S4 dependent-of S3 · §Input Validation → CLI arguments / stdin (:230) · `send` text read beside `answer` → **apply**.
- S5 D-security-input · §Input Validation → Channel frames (:224) · `send` params → `-32602` `invalid params`, `data: null` → **apply** (Accurate this-chunk addition).
- S6 D-security-auth · §Authentication & Authorization → IPC client-side server verification (:205) · the F3 dated gap → **escalate E1** (playbook: Boundary widening — never routine; the founder's live ruling).
- S7 dependent-of S6 · `~/.viola/` access control (:207) · F3 snapshot read without strict-modes → **escalate E1**.
- S8 dependent-of S6 · CLI arguments / stdin, Home path clause (:230) → **escalate E1**.
- S9 dependent-of S6 · Own state files on read (:235) → **escalate E1**.
- S10 dependent-of S6 · §Security Anti-Patterns → Authentication (:524) · the ban's one dated exception gains `viola send` → **escalate E1**.
- (raised by the orchestrator, check 5 / expected amendment 6) the Security Decisions Log record for the newly served channel method `send` and its mapping to the IPC controls — the Log takes no new entry after U35; the record is the sidecar entry → **escalate E1** (it is the record of the same crossing).

### obs-plan
- O1 D-obs-instrumentation · §6 `detail` catalog (:835) · `oversize` covers `vt100-feed` → **apply** (expected amendment 9).
- O2 dependent-of O1 · §7 vt100 bullet (:915) · the poisoning line's detail is `panicked` or `oversize`; size messages carry the drop count → **apply**.
- O3 D-obs-instrumentation · §4 Scenario CL-1 required log fields (:644) · wrapper `send-refused` without `wheel` until `:80` → **reject** (playbook: Sequencing deferral — the spec is right, the wheel chunk owns it; ownership becomes a CARRY on the `:80` entry at P5).
- O4 D-obs-instrumentation · §4 Scenario CL-1 cleanup (:648) · `send.client` exit set gains 12 and 1; exit 21 `during` connect|call; 20 `wrapper-fault` → **apply** (Accurate this-chunk addition).
- O5 D-obs-instrumentation · §5 per-path metrics (:773) · `-32602` now also `send` invalid params; `viola.compat.v_rejected` over-counts until the causes are told apart → **apply** (Accurate this-chunk addition; `-32602` already had a second designed cause, `release` carrying `from`).
- O6 dependent-of O5 · §1 Obs Scope Summary (:472) → **apply** (playbook: Verbatim upstream copy kept current, founder 2026-10-04).
- O7 D-obs-instrumentation · §7 vt100 bullet (:915) · "the G2 question" answered by the F4 home → **escalate E2**.
- O8 dependent-of O7 · §9 Integration tests row (:1011) · F4 carve-out → **escalate E2**.
- O9 dependent-of O7 · §9 step 1 (:1044) · F4 exception → **escalate E2**.

### test-plan
- T1 D-tests-coverage · §2 Property-based row (:434) · `paste_text` joins the fuzz targets → **apply**.
- T2 dependent-of T1 · §6 Property suite (:1020, :1028) · `paste_text` joined, 8 seeds, byte-exact; the property landed at 512 → **apply**.
- T3 D-tests-coverage · §6 Path 2 `local` bullet (:749) · not-delivered until `:82` (F2) → **apply** (expected amendment 8; F2 hold).
- T4 dependent-of T3 · §1 Test Scope Summary (:233) → **apply** (Verbatim upstream copy kept current).
- T5 dependent-of T3 · §6 Path 2 Playwright bullet (:750) · local line's `unconfirmable` owed to `:82` → **apply**.
- T6 D-tests-coverage · §6 Path 2 surfaces (:734) · as built cli + channel + tui on 3 OSes; MCP/SSE/web owed to `:102`/`:131`/`:139` → **apply** (expected amendment 8).
- T7 D-tests-coverage (escalate) · §5 setup/teardown (:625) · the second home carve-out (F4) → **escalate E2**.
- T8 dependent-of T7 · §3 → `test-data-bootstrap` (key file :11, :15) → **escalate E2**.
- T9 dependent-of T7 · §7 Test data lifecycle (:1095) → **escalate E2**.
- T10 D-tests-coverage · §7 Fake agent modes (:1076) · `--vt100-panic-bytes` built → **apply**.

## Validate — the six checks
1. **Playbook** — routine: A1-A8, A11-A15, A18-A21, S1-S5, O1, O2, O4-O6, T1-T6, T10 (Accurate this-chunk addition; O6/T4 Verbatim upstream copy kept current); reject: O3 (Sequencing deferral); escalate: S6-S10 + the method record (Boundary widening, twice-ruled never routine), A16/A17/O7-O9/T7-T9 (F4, founder-visible by the overseer's P4 direction), A9/A10 (no rule; a shipped CLI contradicting a Standard Contract — unease).
2. **Cross-contradiction** — none: A18 and A20 edit [Screen Model] in complementary directions; O1/O2 and O7 edit the §7 bullet in compatible directions (O7 held with E2).
3. **Intent-consistency** — the report vs the working entry `:74` + plan acceptance: every acceptance MET. Divergences: the working entry's "unconfirmable local commands" and "/clear post-condition" are NOT delivered — justified (F2 hold, overseer P4; scope §5) → intent amended by A5/T3-T5; the CLI wrapper-fault `--json` shape diverges from the arch contract without a recorded justification → E3. Scope record: `Cargo.lock`, `fuzz/Cargo.lock` (mechanical, serve their manifests — hold), `.gitattributes` (in-intent, serves the corpus — holds: a byte-exact seed is step 2's own artifact).
4. **Absence needs evidence** — S1 "the only site" (the detector's grep for tee/unbounded/mpsc) and A1 "0 hits" (P1's regex count) are read again at the cascade sweep before any sidecar lands.
5. **Expected amendments** — 1 → A6-A8 · 2 → A1-A3 · 3 → A18-A20 · 4 → A5 · 5 → A14, A15 · 6 → S1, S2 + E1 · 7 → S3, S4 · 8 → T1, T2, T3-T6, T10 + E2 · 9 → O1, O2 + E2 · 10 → no proposal: layout-templates already states the as-built lines (:409-422, :542-545 per its detector) and design-system cli pattern 2's hint strings are the literals `src/human.rs` copies; the one as-built difference, the wrapper-fault `--json` shape, is E3 — disposed.
6. **Disproved claims** — "plan.md: expect no change to tests/channel_endpoint.rs" → a plan-only claim, no master states it → curation (P3) + the report's Deviations; "[Screen Model] falls back to delivery confirmation only" → A18.

## Escalations (HALT — the founder's live ruling)
- **E1 — F3:** `viola send` writes its `send` frame after a liveness-only pre-check (snapshot pid + start time + heartbeat), with no server identity check and no strict-modes check, until Epoch 6 `:109` / `:111`; and the record of `send` as a served channel method. Proposals S6-S10 + the method record.
- **E2 — F4:** `tests/chaos_feed_panic.rs` boots in `TestHome::outside_scan()`, a home outside G2, G4 and the secret scan, asserting its own panic and `parse-rejected` lines present. Proposals A16, A17, O7-O9, T7-T9.
- **E3 — the wrapper-fault `--json` shape:** as built `{"v":1,"error":"wrapper-fault","code":<int>}` vs the arch contract `{"v":1,"error":"wrapper-fault","detail":{"code","message","data"}}` (MCP mirrors it). Proposals A9, A10.

## Resolutions (the HALT dialog, 2026-10-04)
- **E3 — resolved:** "Fix the CLI to the contract" — the overseer (founder-delegated): "the contract is right; fold
  detail:{code,message,data} with a unit test now". Folded into `src/cmd/send.rs`; clippy clean,
  `run --unit --filter 'test(/cmd::send::tests::/)'` 29 passed. A9 / A10 → **reject** (the implementation now matches
  the contract; no master changes).
- **E1 — HELD for the founder** (the overseer's relay: "he is asleep and rules F3 live in the morning … Do not proceed
  past this halt and record nothing for F3"). S6-S10 + the method record stay unapplied.
- **E2 — HELD for the founder** ("he rules F4 in the morning … Record nothing for F4"). A16, A17, O7-O9, T7-T9 stay
  unapplied.
- Nothing applied yet: amendment-flow §Escalate — no apply until every escalation is resolved.

## The founder's ruling (live, 2026-10-04, relayed by the overseer through AskUserQuestion)
"E1 (F3): RATIFY the fourth dated gap until Epoch 6 (:109/:111 remove it). E2 (F4): ACCEPT the chaos-home carve-out as
the second named test-data carve-out, naming all three scans (G2, G4, secret scan). Record both as the founder ruling."
- E1 → S6-S10 + the method record: **apply**, recorded in the security-plan sidecar as the founder's ruling, relay named.
- E2 → A16, A17, O7-O9, T7-T9: **apply**, recorded in each owning sidecar as the founder's ruling, relay named.
- Every escalation resolved; apply proceeds.
