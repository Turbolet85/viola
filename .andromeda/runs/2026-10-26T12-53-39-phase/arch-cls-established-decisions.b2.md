# Classified — arch · established-decisions.b2

## Answers
established-decisions#17.1 · yes · every snapshot write goes through the one holder `Snapshots` (`src/run/snapshot.rs`), which the `links` rewrite of `link` and `unlink` must use; the wheel's own rules are not reached, a link moves no wheel
established-decisions#18.1 · yes · the links decision itself: the implicit link's shape and `since` rule, and the held explicit `viola link` / `viola unlink` with their channel methods, which this chunk lands
established-decisions#19.1 · no · the gate reads only at `send`, which this chunk does not add to; `link` and `unlink` are no `send` and read no gate, and the task's not-needed list names the governor
established-decisions#20.1 · yes · the chunk re-reads at P3 that no `viola-agent-claude`, `fixtures/claude` or `src/cmd/verify` file is touched and adds no row, probe, re-stamp or live start, and the entity records the twenty-four rows and their stamps
