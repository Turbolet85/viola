# Curation — 2026-10-26-server-verification-before-any-frame

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  (none)
  Tier 2 (.claude/rules/*):                   + testing.md: "An OS-gated test module in product code stacks `#[cfg(test)]` first and the OS gate second …" (+ its pointer line in testing-src.md)
  Proof: implement block run 2's red — `contract_windows_mutation_scope` read `src/cmd/client.rs` as Windows-gated product code because the new `refused_inside` module's `#[cfg(all(test, not(target_os = "macos")))]` put its `cfg!(windows)` ahead of the file's first `#[cfg(test)]` (report, Reverted bullet; outcome deviation 7); and the operator pass's CI red, `lint (macos-latest)`: `unused import: super::*` at `src/cmd/last.rs:85:9` and `src/cmd/pause.rs:95:9`, ci#40295285860 (evidence/operator-pass.md §5). Score 0.4 (a real gate failure) + 0.2 (specific technical detail) = 0.6, + 0.2 no other home (no route annotation, no master body states the hazard, no hook refuses the act) = 0.8. It discharges the handoff's carried deferred learning on `is_windows_gate` (confidence 0.8), now measured.
  Tier 3 (.claude/docs/session-learnings.md): + "A plan's count is re-measured at the wrap, never copied"
  Proof: the plan's Expected amendments said «four product callers remain»; `grep -rn 'read_snapshot(' src/` at the wrap, every test-module hit read per file, measured three (`src/cmd/hook.rs:204`, `src/cmd/hook/statusline.rs:142`, `src/cmd/run.rs:379`), the hook's two reads merged by implement (report, Counts / qualifiers moved). Score 0.4 + 0.2 = 0.6, + 0.2 no other home (the sidecar Why is history, not a home) = 0.8.
  Tier 3 (.claude/docs/session-learnings.md): + "A wrap direction names the decision, never who ruled it"
  Proof: the classify read-back's four `provenance` rows on the fork-1 text (established-decisions.md:79, input-validation.md:52, test-plan.md:590, e2e-test-strategy.md:235) after a `· as:` direction asked «write it as his ruling»; the ruled re-spawn `apply-r3` removed them (cascade-dispositions.md). Score 0.4 + 0.2 = 0.6, + 0.2 no other home = 0.8.
  Corrected (in place, cap-exempt): .claude/docs/session-learnings.md "A sidecar entry payload needs a leading blank line before it is appended"
  Proof: `splice.py append --blank-before` at this wrap's Apply step 2 printed `a blank line put before the block (line N is not blank)` on all seven sidecars, each file's last line then the entry's `**Ref:**` (F354 (e), first live use): the entry's «adds no separator» and its `.append.md` recipe were false for the tool as it now stands.
  Filters: 0 dup · 0 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff) · 6 below threshold or homed elsewhere (below)
  Recurrence-despite-learning (→ handoff Deferred learnings): testing.md 2026-10-16 (copy an oracle from the first run's output) — a hand-typed sorted key list was red on its first run at implement; host-linux.md §Transports, Documents (the Write tool, whole-content) — a scratch python append to a run-dir document truncated it when its `open()` raised.
  CLAUDE.md size: 128/200 · T1 3.0 KB, 0 over 600 B

## Decisions & corrections — every entry's disposition
- Fork 1, option 1 (delegate PROVISIONAL, inputs#I3; the founder's own ruling, inputs#I10): not a learning — a decision; its homes are the masters (amended this wrap) and the route CARRY (P5).
- The P5 card's word (inputs#I4): not a learning — a decision; the four re-pins land at P5.
- The ETXTBSY fold (inputs#I5): rejected — homed elsewhere: its mechanism is amended into test-plan-detail/test-data-fixtures.md this wrap (sampler S2).
- The must-trace check (inputs#I6): not a learning — a confirmation of a standing rule.
- The `is_windows_gate` recurrence: applied — Tier 2 testing.md (above).
- The macOS-target check-only clippy recipe: rejected — homed elsewhere: amended into test-plan-detail/ci-integration.md and the commands.md leaf this wrap.
- The stamp hook's refusals of an estimated time: rejected — homed elsewhere: a hook already refuses the act.
- Guard refusals at implement and at this wrap: not a learning — each a guard's one re-issue (`guard-refused`).
- Testing learning 2026-10-16 recurred: → handoff, recurrence-despite-learning.
- The host-reachability gap (a test gated off macOS, a macOS-only unused import): applied — folded into the Tier 2 entry's second clause.
- Sweep hazard, the plan's stale count: applied — Tier 3 (above).
- Sweep hazard, pattern `macgap` never fired: rejected — Filter 4: 0.4 + 0.2 − 0.3 (a one-off) = 0.3.
