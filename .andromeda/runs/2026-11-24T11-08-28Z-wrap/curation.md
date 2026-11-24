CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + security.md: "A new verb that starts through `run::start` inherits every effect of that start, its trust step (the Claude trust-flag write and the kept `CLAUDE_CONFIG_DIR` read) included: before naming the verb's surface or the masters' lists of starts, read the shared start's code for its writes and reads, not only the verb's own file."
    Proof: src/cmd/adopt.rs `started` → `run::start` (src/cmd/run.rs `trust_folder`, its `trust_word` reading `trust_record::trust_config_file(strip)`); the report's Changes named the path and neither effect; 21 master rows naming only run and revive's start were found and amended at this wrap's P2 (founder-forks.md Q2, `inputs#I7`).
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 3 task-specific · 0 conflict · 0 deferred

Decisions & corrections, each weighed:
- implement: the contended-host stop and the resume word (`inputs#I5`) — not a learning: a directive's own procedure, already a host-read rule of the overseer's.
- implement: «no new time bound» applied to a test-only script — rejected, Filter 2 (task-specific to `inputs#I4`).
- implement: sweep hazard «is_session_id accepts either hex case» — rejected, Filter 2: the shape's case now stands in the masters (architecture contract/cli-verbs/adopt), the parser's doc comment holds it.
- implement: sweep hazard «the fake's start receipt carries no argv» — not a learning here: now a test-plan master row (decision/child-argv-read-from-proc), its leaves carried by the cascade.
- after implement: the pass's hygiene respell of p5-card-1.md's two `/tmp/` paths — rejected, Filter 2 (one artifact's fix).
- after implement: the wrap directive (`inputs#I6`) — not a learning: a directive.
- this wrap: adopt's start inherits run's trust step — applied, Tier 2 security.md (above).
