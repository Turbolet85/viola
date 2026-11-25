@leaf_group group=architecture-3
@leaf_read leaf=.claude/docs/services/viola.md lines=58
last: - `doc:architecture` · `.claude/rules/api.md` · `.claude/rul
@leaf_read leaf=.claude/docs/stack.md lines=66
last: 5. Commit as `chore: bump {tool} to {version}`
@leaf_read leaf=.claude/docs/workflow.md lines=67
last: This project uses a **reference-based** CLAUDE.md architectu
@leaf_edit n=1 leaf=.claude/docs/services/viola.md line=35 class=omission
leaf-text: `program-not-found` · `pinned-hash-mismatch` · `version-unreadable` · `strict-modes-failed` · `unclean-fixture`
source: ctr-y3qkqg22 «contract/viola-verify-refusal-codes/untrusted-folder · viola verify process-exit refusal codes · untrusted-folder — Written on the process-exit verify logs under a parsed VIOLA_NAME when a probe run's first screen read»
replace-with: `program-not-found` · `pinned-hash-mismatch` · `version-unreadable` · `strict-modes-failed` · `unclean-fixture` · `untrusted-folder`
@leaf_edit n=2 leaf=.claude/docs/services/viola.md line=6 class=omission
leaf-text: a refusal printing the home pair and exiting 1 with no step line, no summary and no line;
source: dec-ixmg7xtr «decision/verify-refuses-untrusted-folder · Verify refuses an untrusted folder — viola verify refuses with untrusted-folder, exit 1, when any of Runs B to E reads the trust or imports form on its first screen (Runs A»
replace-with: a refusal printing the home pair and exiting 1 with no step line, no summary and no line; any of Runs B to E reading the trust or imports form on its first screen (Runs A and F never read) refuses `untrusted-folder`, exit 1, with no step line, no summary and no stamp, its probe folders removed;
@leaf_edit n=3 leaf=.claude/docs/services/viola.md line=20 class=stale
leaf-text: an entry already attempted, a red one included, never verified again automatically
source: dec-56xxcfin «decision/no-automatic-reverify-of-red · No automatic verify of an earlier red version — An automatic verify runs only for a version whose stamps entry is short of a relied row; an entry holding every relied row that»
replace-with: an entry short of a relied row verified again, an entry holding every relied row that did not verify never verified again automatically, and an `untrusted-folder` refusal recording nothing and leaving the version un-attempted
@leaf_edit n=4 leaf=.claude/docs/stack.md line=14 class=omission
leaf-text: before a bind (`viola run`, `revive`'s start)
source: ctr-uwztwbnt «contract/frame-verb-exit-detail/socket-dir-refused · CLI frame verb process-exit detail · socket-dir-refused — Exit 21 with during connect, at WARN, no channel-request line; run's, revive's and adopt's start write it with exit_code 1.»
replace-with: before a bind (`viola run`, `revive`'s and `adopt`'s start)
