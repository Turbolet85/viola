# Leftovers — dry-run listing (step 17)

Listed after M2's diff was recorded (`m2-diagnosis.md`). **Nothing was removed by /implement.** The removal is the
founder's, in the operator pass (P4 ruling). The `%TEMP%` sweep is a founder desk item (overseer, 2026-10-03).

The listing and the removal are one script, `evidence/leftovers.py`. It classes each dir by its measured viola-test shape,
never by age:
- a `.tmp*` git repo whose only tracked file is the harness fixture's `a.rs`;
- a throwaway cargo project whose package is named `viola`;
- an empty `.tmp*` dir;
- an ownerless `viola-test-*` home.

Any other `.tmp*` dir belongs to another tool and is counted only.

## Dry run, 2026-10-03 (after this run's gates)
| class | count |
|---|---|
| `%TEMP%` `temp-git-repo` | 1 574 |
| `%TEMP%` `temp-cargo-project` | 1 658 |
| `%TEMP%` `temp-empty` | 5 378 |
| `%TEMP%` other (not viola's shape, never touched) | 14 681 |
| `target/e2e-home/viola-test-*` without `owner.json` | 22 (see below) |

The before snapshot's 13 497 dirs "holding a `.git`" (`host-snapshot.md`) count any git repo. Only 1 574 are the harness
fixture's shape (`a.rs` the sole tracked file). The rest belong to other tools' tempdirs.

**Not listed for removal:** `target/baseline-target` and `target/conpty-seed`. The operator's `cargo clean` of 2026-10-03
removed them along with the 4 768 historic homes, so nothing of theirs remains to keep. 16 `viola-session-*` harness
homes without a record are also present; `agent-run cleanup` owns them, they are outside the owner rule, and they are listed
here only. Two P5 probe homes under `target/agent-run/p5/` (this run's) are gitignored scratch.

## The removal command (founder, operator pass)
```
python -X utf8 viola-0.1.0/chunks/2026-10-02-epoch-2b-cleanup/evidence/leftovers.py --apply
```
Run it from the repository root. It removes only the four viola classes above, clearing a read-only attribute where
removal needs that, and prints the counts it removed.
