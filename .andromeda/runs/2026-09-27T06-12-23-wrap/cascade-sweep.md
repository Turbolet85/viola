# Cascade sweep — dispositions (2026-09-27-instance-state-and-start-order)

Patterns: `cascade-patterns.toml` (20; every control fired on the pre-pass text, baseline a892917 = the pre-CI
commit's parent). Run 1 (`cascade-sweep.txt`) after the fan-out applies; run 2 (`cascade-sweep-2.txt`) after the
fold amendments and the leaf re-derivation. Every row of run 1 dispositioned below; run 2 prints only this pass's own
text, the true claims kept, and re-derived leaves.

## Master rows (run 1)
| row | disposition |
|---|---|
| architecture.md:50 awf-name (new) | this pass's text — the rationale names atomic-write-file as the rejected first choice (true) |
| obs-plan.md:72, :185 awf-name | §1 verbatim obs-scope copy — no change (playbook: Verbatim scope copy) |
| architecture.md:92 ref-answering (new) | this pass's text — "once the endpoint exists, a name with an answering endpoint refuses" (true, future step) |
| architecture.md:146 ref-live-ep | exit 21 "no live endpoint" — a true claim sharing the token |
| test-plan.md:529 ref-live-ep | STALE — "exit 1 for a live endpoint or `stale` heartbeat" → amended (fold) to a `live` / `stale` name or a tampered copy |
| obs-plan.md:421 live-age-first / 5.1 | §1 verbatim — no change |
| obs-plan.md:767 (§3), :1310, :1322 (§10) | the live→stale flip threshold of a running process (`heartbeat_age_ms > 5000`; "stale or gone") — true; no change |
| test-plan.md:1406 mock-instant | the general sync-time rule (mock_instant or the injected Clock) — true; `classify` takes the age as input |
| architecture.md:244 snap-ep-first | replay: fields no event carries, `endpoint` among them — true |
| security-plan.md:207@c1782, :500, :579; obs-plan.md:141 | the snapshot `endpoint` as a recorded reference / integrity-sensitive field once bound — true design, no change |
| test-plan.md:746 (edited) | this pass's text |
| test-plan.md:1742 keep-every-leg | B1 ruling record (history, "as ruled") — no change |
| architecture.md:411 stderr-allow (new) | this pass's text (re-derived from the code: no print allow in the root bin) |
| architecture.md:23 (edited) | this pass's text |
| security-plan.md:671 sha2-devdep | STALE — amended (fold): sha2 is a product dependency of `viola-state`, the dev-dep stays for the KAT |
| test-plan.md:1335 sha2-devdep | the KAT test's own dev-dependency — true |
| test-plan.md:1775 interim-ready | STALE — amended (fold): readiness now reads the snapshot and heartbeat; events joins with hooks |
| test-plan.md:174, :526, :1027 ss-third | this pass's text |
| obs-plan.md:307 ss-third | §1 verbatim — no change |
| obs-plan.md:869 ss-third (§4) | STALE — amended (fold): `session-start` joins with "Hooks to normalised events" |
| architecture.md:98, :372 (edited) | this pass's text |
| security-plan.md:265, :380 (edited) | this pass's text |
| (hand check) "join" near "heartbeat" in the four amended masters | 1 hit, test-plan.md:523 — this pass's "each joins as its surface lands", not a thread join: no master states the heartbeat guard joins (disproved claim 4 is plan-only) |

## Leaf rows (run 1) — all re-derived
`rules/events.md:29, :31` · `rules/security.md:15` (+ :32, the `env -i` carve-out, found by the leaf grep) ·
`docs/services/viola-state.md:6, :11, :20` (+ :38) · `docs/stack.md:18, :19` · `docs/services/viola.md:20` ·
`docs/obs-summary.md:32` · `docs/security-summary.md:66` · `rules/verification-harness.md:29` (+ :24, :43, :44) ·
`docs/tests-summary.md:25` (+ :20). `docs/security-summary.md:28` (snapshot `endpoint` as an integrity-sensitive field)
— true, no change. The extra lines came from `leaf-grep.txt` (a wider token grep over CLAUDE.md, every rule and docs
leaf); its other body rows are true claims (pins, hooks.json exec-form rules, `--kind` filter list, etc.).
CLAUDE.md `GENERATED:setup:*` recomputed from the amended sections — no block states a changed fact, no change.
Curation homes and judgment bases: 0 rows.

## Binds
test-plan §3 ↔ obs-plan §3: the harness changes (readiness staging, `logs --kind`, mutation no-keep, pre-push scratch)
live in test-plan §3; obs-plan §3 restates none of them except the run-file collision path (amended, O4) — consistent.
a11y ↔ obs schema: untouched.
