# Curation — wrap 2026-11-24T07-06-28Z (chunk 2026-11-24-viola-up)

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + host-linux.md: "An edit made by a script (python, sed) fires none of the Edit tool's PostToolUse hooks, the rustfmt one included: end a scripted change to a `.rs` file with `cargo fmt --all` in the same step, before any gate reads it." (confidence 0.8)
    Proof: implement run 2026-11-24T05-12-02Z-implement, the `cargo fmt --all --check` entry red once (two files a scripted python edit left unformatted), green after `cargo fmt` — its outcome file's Gates and Decisions & corrections («a scripted (python) edit fires no format hook, and `cargo fmt` must end that call»). Signals: verified by a real gate failure +0.4 · specific technical detail +0.2 · reached no other durable home +0.2.
  Tier 3 (.claude/docs/session-learnings.md): none
  Extended: Tier 2/testing.md: "2026-11-22: A rename of a product constant or a move of a nextest filter is swept over the test tree's string literals too…" + "a verb added to `viola --help` joins `tests/cli_output_plain.rs`' own served list in `help_lists_only_served_verbs` as well as the `HELP` literal: a sweep of the `setup:` row alone misses the second" (confidence 0.8)
    Proof: implement run 2026-11-24T05-12-02Z-implement, the readers' entry red once: `help_lists_only_served_verbs`' served list lacked `up` after the `HELP` literal gained it (the report's Outcome, Gates). Signals: verified by a real gate failure +0.4 · specific technical detail +0.2 · reached no other durable home +0.2.
  Filters: 2 dup · 3 task-specific / homed in a master · 0 conflict · 0 deferred (cap) · 2 recurrence-despite-learning (→ handoff)
  No-other-home: "a scripted edit fires no format hook: end it with cargo fmt" · "the help's second served-verb list"
  CLAUDE.md size: 136/200 · T1 4.8 KB, 0 over 600 B

## Dispositions — every Decisions & corrections entry weighed
Implement's *Decisions & corrections*:
- A CONTENDED host stops the whole layers — not a learning: the directive's own stop rule (`inputs#I13`), applied as written; the host-read rule stands in the directives and the handoff.
- The pre-push corrupt-profile red closed by its fix (step 22) — rejected: homed in a master this wrap (test-plan `decision/stand-in-terminal-tree-teardown`, tx 4601).
- Guard refusals (two `cd` outside the project) — not a learning: each cost its one re-issue (`guard-refused`).
- The open `tui_channel_fds` red stops the run before the pass — duplicate: CLAUDE.md «A red stays open until its cause is known…» applied as written.
- The `tui_channel_fds` red closed by its fix — rejected: homed in a master (test-plan `decision/child-fd-table-read-after-launch-settles`, tx 4601).
- A run-dir name read 40 s early — recurrence-despite-learning: CLAUDE.md «A timestamp written into any record … is read from `date -u` at the moment it is written» (→ handoff).
- Guard refusals of the first run (heredoc append, a pipe into `cut`, two estimated stamps) — not a learning: each cost its one re-issue.
- Sweep hazard, `help_lists_only_served_verbs`' second served list — extended (testing.md, above).
- Sweep hazard, a python scripted edit fires no format hook — applied (host-linux.md, above).
- Recurrence of testing.md 2026-09-24 (wait on the exact line asserted) — recurrence-despite-learning (→ handoff).

`## After implement`'s *Decisions & corrections after implement*:
- `inputs#I15`, the quiet-window hold — not a learning: an operator-side ruling for one pass.
- `inputs#I16`, whose word each row rests on — not a learning: the directive's routing of this wrap's rows.
- The observability rule corrected to the code — corrected at its source: obs-plan `contract/diag-subject/terminal` and `contract/process-log/child-and-probe-lifecycle` (tx 4602), the rule re-derived by the cascade.
- The test wrappers' lock files — not a learning here: a carry on the route (P5), no row or rule owns it.

This wrap's conversation:
- The overseer's Q2 reading (the recorded cwd inside an answered argv is a widening of its own) — duplicate: CLAUDE.md «his answer covers only what the question shown to him named, never the wider item».
- The store refusing the operator's lead «These» — task-specific: one rows-form refusal, recorded in the evolve record and `founder-forks.md` W2.
