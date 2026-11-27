CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "A pump case that asserts a skipped drain consumes the output end first"
    Proof: the report's Decisions & corrections (implement), the mutation-test-design fact; the case
    `pump_returns_at_once_when_the_output_ended_before_the_exit` passes and its control `drain-done` reads `holds`
    (`evidence/control-1.txt`: `control: 12 holds`). The fit reader judged the master row of the same fact
    `not_master` (the test code holds it), so it lands here, not in test-plan.
  Filters: 3 dup · 1 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff)
  CLAUDE.md size: 136/200 · T1 4.8 KB, 0 over 600 B

Dispositions, every Decisions & corrections entry:
- (implement) the overseer's word at the step 7 stop (inputs#I5) — not a learning: a one-off ruling of this run.
- (implement) the phase directive's standing rules — rejected, Filter 1 dup: CLAUDE.md's `date -u` and no-scheduled-wake
  learnings already hold them.
- (implement) the permission-classifier refusal of `--operator 27` (a verdict) — rejected, Filter 1 dup: the operator
  pass is where an operator leg fires; nothing new.
- (implement) the recurrence of host-linux «Transports» (a stray `cat > /dev/null` before a python heredoc blocked on
  stdin to 120 s) — rejected, Filter 1: a recurrence of a standing rule, `recurrence-despite-learning`, → handoff's
  deferred list beside the earlier ones.
- (implement) the sweep hazard `now() >= deadline` standing twice in `pump.rs` — rejected, Filter 2 task-specific.
- (implement) the mutation-test-design fact of the pump cases — applied, Tier 3 (above).
- (after implement) the overseer's word for the operator pass (inputs#I6) — not a learning: a one-off ruling.
- (after implement) the directive's item 2, the departures closed by name — not a learning: each is closed against
  this chunk's plan.
- (after implement) the directive's standing rules for this wrap — rejected, Filter 1 dup (as above).
- (this window) a delegate's technical word lands no master row the fit reader judges `not_master`, and a `word:`
  tag on a disposition line writes PROVISIONAL into the sidecar entry — not a project learning: the skill's own
  letter states both (`amendment-flow.md` §Validate check 7, the disposition line).
