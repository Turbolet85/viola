# Fan-out results — 2026-09-29-h2-conpty-resize-probe

Seven Explore doc-agents, one parallel batch, the amendment-flow prompt verbatim; input: `chunk_dir/report.md`.
Every return was YAML plus comment lines (stripped: basis commentary only — no proposal text removed); no entity
mangling in the parsed values (no `&lt;` / `&gt;` / `&amp;` in any proposal field).

## Verdicts
- **architecture** — 2 proposals (D-arch-resources · D-arch-decisions).
- **security-plan** — `proposals: []` (stripped: a basis note that the `watch` mode is a test-child value, not a
  compiled-in seam like `FAKE_AGENT_PUMP_DELAY_MS`; Dependencies none).
- **design-system** — `proposals: []` (stripped: tokens n/a, no UI).
- **layout-templates** — `proposals: []` (stripped: no surface or region).
- **test-plan** — 1 proposal (D-tests-coverage); the agent's sweep claim: no other test-plan site states the
  resize-then-key claim (:125, :1369, :1893/:1897/:1901 read and not matching).
- **obs-plan** — `proposals: []` (stripped: no obs_event / span / field; PII n/a).
- **a11y-plan** — `proposals: []` (stripped: no UI; pump write path untouched).

## Proposals and dispositions

### P-A1 — architecture · §Occupied Resources → Filesystem (:396)
- detector: D-arch-resources · severity: warning
- change: the viola-pty-watch entry lists both files: `<temp dir>/viola-pty-watch/<test name>.report` (the child's
  report) and the sibling `<test name>.test.report` (the test's steps `resize-returned` · `key-written` ·
  `key-flushed` and `dsr-cpr {n}`); both kept on a panic or kill (`Drop` writes `dsr-cpr` on a panic not yet
  reported), removed on a pass; `viola` never reads or writes either.
- rationale: report Changes → Symbols (`report_paths`); Expected amendments (site :396).
- **Disposition: APPLY** — playbook "Accurate this-chunk addition" (routine): the named file is this chunk's, a
  test-only artifact of a kind the registry already enumerates at :396. Check 5: expected amendment 2 matched.

### P-A2 — architecture · §Established Decisions → [PTY] (:47)
- detector: D-arch-decisions · severity: warning
- change: add the as-built H2 fact — a key written right after a ConPTY resize can be lost: 13/200 on the runner
  (windows-2025-vs2026 20260922.246.2, ci#36527891850 on `d8b5051`), all class K, class R 0, `dsr-cpr 0` on every
  loss and on the host (10.0.26200.9457 / conhost 10.0.26100.8875); documented, not fixed; the product window
  stays (≈ 6.5 %) under the founder's open question; tests that write a key after a resize wait for the child's new
  size first (ci#36529038462: 0/200).
- rationale: report Cross-project claims, Insufficient fixes, Spec claims disproved, Expected amendments (site :47).
- **Disposition: APPLY, re-derived** — playbook "Accurate this-chunk addition" (routine). The proposal's collateral
  "it does not trigger the own-ConPTY swap" and "viola has no DSR reply to send" are NOT carried by the report:
  dropped from the applied text (amendment-flow §Apply 1). Scope named to the measurement (isolated loop under
  llvm-cov on that image). Check 5: expected amendment 1 matched.

### P-T1 — test-plan · §5 Integration Test Strategy → `Module ↔ PTY` row (:926)
- detector: D-tests-coverage · severity: warning
- change: the row adds the key-after-resize rule (the reshaped red test waits for the child's `size 120x40` before
  its key), the key-free witness `spawn_reports_a_resize_to_a_child_that_reads_no_key`, and the measured limit the
  tests do not cover (the product window, 13/200 class K, `dsr-cpr 0`, ci#36527891850; gotchas.md H2).
- **Disposition: APPLY, re-derived** — playbook "Accurate this-chunk addition" (routine). The proposal's trailing
  "Do not retry, `#[ignore]` or skip any H2 test" restates §10's standing zero-flakiness rule — not added to the
  row. Check 5: expected amendment 3 matched.

### O-T2 — test-plan · §10 Coverage thresholds → Stack adjustments (:1506) — raised by the orchestrator
- change: the propagation bullet keeps its rule for child `viola` processes of the product and names the one
  deliberate exception: the harness self-tests' nested cargo over the throwaway `mini` crate (package `viola`, never
  the product) removes cargo-llvm-cov's four names, so its binaries write no profile into the outer run.
- why raised: no detector fired on it, yet the report's Harness / gate surface bullet carries the fact and the bullet
  as written reads as covering every nested `viola`-named process (check 4/5 floor: a fact the report carries with a
  master site that states its opposite by omission).
- **Disposition: APPLY** — playbook "Accurate this-chunk addition" (routine): this chunk's change; the bullet's
  invariant (product children stay in coverage) holds. The mechanism is written with its epistemic status (the
  host measurement; the CI corrupt file attributed by inference).

## Validate — the six checks
1. Playbook — 4 × routine ("Accurate this-chunk addition"); no rule collision; no boundary widening (the viola-e2e
   change REMOVES names from a test child's environment; no product input class or crossing added).
2. Cross-contradiction — none: P-A1/P-A2 edit different arch sections; P-T1/O-T2 different test-plan sections.
3. Intent-consistency — the entry intent ("… then fixed or documented") and the plan's acceptance hold (document
   branch by the pre-stated rule). Scope record: 2 × `widening` carrying the operator's word "Fold every red into this
   chunk." — the justified branch; `serves step 9` holds (the pass's CI red). Push 2 (a verification push on the
   document branch) is a justified deviation within the operator's 3-push bound. No escalation.
4. Absence needs evidence — P-T1's "no other site" claim is re-checked by the cascade sweep (its rows dispositioned
   in `cascade-dispositions.md`), not taken from the agent.
5. Expected amendments — 3 / 3 matched (P-A2 · P-A1 · P-T1).
6. Disproved claims — (a) the overseer relay "the resize never reached the child": no master states it (one grep of
   `never reached|RESIZE itself|\[6n|\bCPR\b|cursor-position|cursor position` over the seven masters → 2 hits, both
   a11y-plan `never reached` at :530 and :757, read: the `TAPE connecting` UI state "never reached by a timeout" — not
   this claim); DISPOSED to the chunk evidence (`h2-reproduction.md`) — a scope-level hypothesis closed by
   measurement. (b) H2-CPR: no master states it (the same grep: 0 hits for `[6n` / `CPR` / cursor position);
   DISPOSED the same way, and carried into P-A2's `dsr-cpr 0` fact. (c) the plan's `run --e2e` gate: test-plan
   §3's command body lists `--e2e` as specified and §12 `2026-09-24` already rules "Unbuilt selectors are usage
   errors" — no master claim is false; DISPOSED to route-resolve (P5): the pin on working-route :85 "The board: viola
   list" with its owner (operator direction).

Escalations: 0.

## Post-apply correction (overseer direction, before the commit)
The overseer (web research, sources in the overseer log — a relay): microsoft/terminal's resize path does not flush the
input buffer, and no issue reports resize input loss, so "the loss is inside ConPTY" is not established; the measured
loss is between the pipe write and the Rust test child's `ReadConsoleW` read, and the real `claude` (Node/libuv,
`ReadConsoleInputW`) is unmeasured. P-A2 and P-T1's applied text said "below the seam" and stated the product window as
a fact for `viola run`; both bodies were re-worded (cause unestablished · product impact unmeasured for `claude` · owner
the real-CLI verify entry), their two sidecar entries re-worded to match (still on-form, `sidecar.py check` 0 off-form),
and the relayed source read written only as a labelled relay, never as fact (Tier-1: a relayed claim with no artifact
on disk is never a master fact). Leaves gotchas.md and services/viola-pty.md re-worded per the direction; the owner
pinned as a CARRY on working-route "First live test and self-drive". Re-sweep of the retired wording (`below the seam|
inside ConPTY|ConPTY / conhost|rstudio|can still be lost|lost below viola|cause sits below`) over `.andromeda/*.md`,
`.claude/**/*.md`, CLAUDE.md and the working route: 2 hits, both on the pending-frozen `[2026-09-29-h2-conpty-resize-probe]`
line 57 (its folded freight, stripped at the P7 flip-compaction; never edited at P5).
