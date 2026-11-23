@task_prose part=chunk
Every session viola starts names its model and effort: viola run, and the start arm of viola revive, hand the claude child a model and an effort on every spawn, the pair the operator gave or else viola's configured default pair.
The pair is recorded per instance as a closed set of values, beside the lineage record (events log, snapshot, replay).
viola revive replays the recorded pair on its resume, so a resumed session runs on the pair its instance recorded.
It re-measures at P3 the claude CLI channel for model and effort, and what a resume does with them today; it builds no inheritance for open, up or adopt, and writes nothing to the user's own settings.
@task_item n=1 part=needs
The viola run argument surface and the viola revive start arm gain a model and an effort the operator may give; the viola --help usage and verb rows that list them stay served-verbs-only and pinned by literal.
@task_item n=2 part=needs
The child spawn of claude carries the model and the effort on every start, through the channel (flag, environment name or settings key) P3 measures; the argv is built from the parsed arguments, never from typed-back text.
@task_item n=3 part=needs
viola's configured default pair: where no pair is given the start uses the default; if the fork lands it in config.json, it is a new config.json key with a fixed name and closed set of values, read like the other config keys.
@task_item n=4 part=needs
The pair is recorded per instance in the events log and the snapshot, as a closed set of values, and replayed on rebuild beside the lineage record.
@task_item n=5 part=needs
viola revive's resume replays the instance's recorded pair onto the resumed claude child; a revive with no recorded pair (an instance from before this chunk) still resumes on the default.
@task_item n=6 part=needs
A model or effort value outside the closed set is refused at the argument surface: a usage error with exit 2, or a refusal as unable line with a kebab-case code word beside it and one hint line, never a stack trace or bare status; under --json it is one clean failure document with no hint.
@task_item n=7 part=needs
viola run, revive start and revive --json keep their existing refusal documents, start-refusal and revive --json refusal codes, exit codes and hint pairs unchanged while the pair is added to the start path.
@task_item n=8 part=needs
viola list board, viola list --json and the MCP list tool: whether the pair is shown is a fork; if shown, it is a word in its own column with fixed caption order, never carried by colour alone, no SGR off a terminal or in --json.
@task_item n=9 part=needs
During the TUI passthrough after the start, viola prints zero bytes of its own on the terminal; any diagnostic of the pair chosen or replayed goes to the run diagnostics log, never the terminal, with the log identity tags.
@task_item n=10 part=needs
The user's own claude settings stay byte-identical: the chunk writes nothing to them and reads nothing of the operator's real home claude settings in this phase.
@task_item n=11 part=needs
Fixed CLI strings the chunk adds (option help, refusal words, hints, any list caption) come from design copy and are pinned by literal tests; printed names are the parsed arguments.
@task_item n=12 part=needs
The new start and revive behaviour is proved by tests sequenced on records and a stand-in child, never a live claude start; live readings never gate, and no time bound falls on the human.
@task_item n=13 part=needs
The new tests run inside the existing suites as nextest failures, with no skips, zero flakiness and no retries, inside the coverage suite of the push/PR workflow; the gate breach fails the build.
@task_standing n=1
convention: each new fixed CLI string (help line, refusal word, hint, caption) is pinned by a literal test and comes from design copy
home: cognitive-accessibility · 8-cognitive-accessibility
@task_standing n=2
convention: each new refusal names a kebab-case code word beside unable, carries exactly one hint (none for a fault), and under --json is one clean document
home: cognitive-accessibility
@task_standing n=3
convention: each new a11y-bearing test runs inside the existing test suites and fails as a nextest failure
home: bootstrap-phases-derive-for-route-setup-project
@task_standing n=4
convention: each new test is deterministic, never skipped and never retried
home: 10-slo-invariants-a11y-budgets
@task_standing n=5
convention: each new test case runs inside the coverage suite of the GitHub Actions push/PR workflow, with no artifact or driver of its own
home: 9-ci-integration
@task_standing n=6
convention: each new log line of a run goes to the run diagnostics, never the terminal, and carries the log identity tags
home: a11y-scope-summary
@task_item n=1 part=not_needed
viola send, answer, wait and verify output contracts and their refusals: the chunk changes none of those verbs.
@task_item n=2 part=not_needed
viola link and viola unlink result lines and exit codes: the pair sits beside the lineage record, but the link verbs are not changed.
@task_item n=3 part=not_needed
Hook event and dialog calls, frame verb refusals, MCP frame tools and instance-unreachable details: the chunk touches no IPC frame or hook path.
@task_item n=4 part=not_needed
viola plugin install output and exits: no plugin change.
@task_item n=5 part=not_needed
Dialog hold, dialog-expired event lines and dialog amber colour: the chunk adds no dialog behaviour.
@task_item n=6 part=not_needed
Human-keystroke wheel rules, the stdin non-editing classifier, send-refused-while-typing and focus, mouse and resize handling: the chunk does not change input handling of a running session.
@task_item n=7 part=not_needed
Manual Orca and keyboard passes and screen-reader test setups: supplemental, they gate nothing this chunk builds.
@task_item n=8 part=not_needed
N/A areas for a web or graphical frontend (contrast, focus management, WCAG mapping, structured violation schema rows for pages) and the absences of i18n, touch and a rendered page.
@task_item n=9 part=not_needed
Switching a running session's model and effort: that is a later entry, not this chunk.
