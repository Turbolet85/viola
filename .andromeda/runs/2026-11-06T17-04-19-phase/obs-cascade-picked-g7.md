@account n=1 entity=metric-coverage#1.1 v=not_in_scope
why: the generic jq Counter over closed `detail` values; the chunk adds no derived metric or grouping key, and a detail word's count follows its line by itself
@account n=2 entity=metric-coverage#1.3 v=not_in_scope
why: the Gauge over `budget.read_at` and `liveness` snapshots; the chunk touches neither budget nor heartbeat and the scope names no liveness reading
@account n=3 entity=metric-coverage#2.10 v=carried
why: `viola.child.exit` reads the child's `process-exit` and `exit_source`, which the `portable-pty` bump moves under the PTY seam and the killed-wrapper reading re-reads
@account n=4 entity=metric-coverage#2.11 v=carried
why: the done-check re-asserts the zero error budget, `viola.panic` must be 0
@account n=5 entity=metric-coverage#3.1 v=carried
why: records the CLI's `socket-dir-refused` line, whose wording the founder ratifies or re-words in this chunk
@account n=6 entity=log-coverage#1.1 v=carried
why: the `process-start` row the chunk edits through the whole-fact form (CARRY 14)
@account n=7 entity=log-coverage#1.2 v=carried
why: the child's `process-exit` `exit_source` the `portable-pty` bump must keep and the killed-wrapper reading rests on
@account n=8 entity=log-coverage#1.5 v=carried
why: paste through the bumped PTY seam logs `text_bytes` only, and the live sessions' sends are read through these lines
@account n=9 entity=log-coverage#1.6 v=carried
why: the dialog hold near the 3600 s ceiling is read live through the dialog lines' `deadline_hit` and `duration_ms`
@account n=10 entity=log-coverage#1.7 v=carried
why: the dialog hook's `hook-invoked` / `hook-decision` lines are the live dialog read's evidence
@account n=11 entity=log-coverage#1.8 v=carried
why: G2, re-run green at the wrap, counts panic lines and exempts the seam panic by its `panic_location`
@account n=12 entity=log-coverage#2.1 v=carried
why: names `socket-dir-refused` among exit 1's codes on the start arm, the wording the chunk rules
@account n=13 entity=log-coverage#2.2 v=carried
why: a closed schema word no build emits leaves with its emitter, the rule the `pty_backend` cut and the remnant removal follow
@account n=14 entity=log-coverage#2.3 v=carried
why: names `socket-dir-refused` among exit 21's codes, the wording the chunk rules
@account n=15 entity=log-coverage#2.5 v=carried
why: names `socket-dir-refused` among the hook's fail-open codes, the wording the chunk rules
@account n=16 entity=log-coverage#2.8 v=not_in_scope
why: the `RefusalReason` detail words; the chunk adds and re-words no product refusal, and the error-enum folds leave this closed list alone
@account n=17 entity=log-coverage#2.9 v=carried
why: defines the closed `socket-dir-refused` code and its lines per surface, the wording the chunk ratifies or re-words
@account n=18 entity=log-coverage#3.1 v=carried
why: the schema's closed `subject` enum stands on the `process-start` row the chunk edits
