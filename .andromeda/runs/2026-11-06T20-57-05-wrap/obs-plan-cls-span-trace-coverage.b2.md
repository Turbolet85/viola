@answer n=1 entity=span-trace-coverage#5.1 v=yes mode=stale
why: the session reaches crates/viola-state/src/pin.rs, where the pin copy placement rule lives, and no report line shows that placement claim holds
@answer n=2 entity=span-trace-coverage#6.1 v=no
why: the report holds the CL-1 send trace and logging rules unchanged, and the only send change, the Git Bash stderr warning, is not named in this text
@answer n=3 entity=span-trace-coverage#6.2 v=no
why: the report holds the CL-1 send span attributes unchanged, and the Git Bash stderr warning is not among them
@answer n=4 entity=span-trace-coverage#6.3 v=no
why: the report holds the CL-1 send log fields unchanged, and the Git Bash stderr warning is not among them
@answer n=5 entity=span-trace-coverage#6.4 v=no
why: the report holds the CL-1 send cleanup and exit codes unchanged, and the Git Bash stderr warning is not among them
@answer n=6 entity=span-trace-coverage#7.1 v=no
why: the report holds the wait and last logging rules unchanged, and the session moves no wait or last span
@answer n=7 entity=span-trace-coverage#7.2 v=no
why: the report holds the wait and last span attributes unchanged
@answer n=8 entity=span-trace-coverage#7.3 v=no
why: the report holds the wait and last log fields unchanged
@answer n=9 entity=span-trace-coverage#7.4 v=no
why: the report holds the wait and last cleanup rule unchanged
@answer n=10 entity=span-trace-coverage#8.1 v=yes mode=stale
why: the session reaches src/cmd/hook.rs and src/cmd/client.rs, which carry the dialog hook's instance check and the CLI answer's refusal path, and the report holds no dialog-path line
@answer n=11 entity=span-trace-coverage#8.2 v=yes mode=stale
why: the session reaches src/cmd/hook.rs, which carries the hook.decision_emit span and the dialog hold bound this attribute list reads, and the report holds no dialog-path line
@answer n=12 entity=span-trace-coverage#8.3 v=yes mode=stale
why: the unverified-CLI dialog branch is among the v1-27 clauses deferred to 0.2.0, so the branch's driver-side refusal and hook log lines in src/cmd/hook.rs and src/cmd/client.rs read unread
@answer n=13 entity=span-trace-coverage#8.4 v=yes mode=stale
why: the session reaches src/cmd/hook.rs, which writes the dialog's hook-decision and deadline_hit, and the report holds no dialog-path line
@answer n=14 entity=span-trace-coverage#9.1 v=no
why: the report holds the wheel and release logging rules unchanged, and the only send change, the Git Bash stderr warning, is not named here
@answer n=15 entity=span-trace-coverage#9.2 v=no
why: the report holds the wheel and release span attributes unchanged, and no wheel span moved
@answer n=16 entity=span-trace-coverage#9.3 v=no
why: the report holds the wheel and release log fields unchanged, including the release-from-driver and budget-gate lines
@answer n=17 entity=span-trace-coverage#9.4 v=no
why: the report holds the wheel transition cleanup and the wheel event audit rule unchanged
@answer n=18 entity=span-trace-coverage#10.1 v=no
why: the budget gate stays unchanged per the fact, so the budget governor's span names hold, and the session moves only the ledger row's reading
@answer n=19 entity=span-trace-coverage#10.2 v=no
why: the budget gate stays unchanged per the fact, so the budget governor's span attributes hold, and the session moves only the ledger row's reading
@answer n=20 entity=span-trace-coverage#10.3 v=yes mode=add-to
why: the account-plan ledger finding F1 joins the budget-paused refusal line here, which rests on the statusline-rate-limits row reading the account's plan (owner 0.2.0)
@answer n=21 entity=span-trace-coverage#10.4 v=no
why: the budget gate stays unchanged per the fact, so the statusline shell-out cleanup and the gate's close rules hold
@answer n=22 entity=span-trace-coverage#11.1 v=no
why: touch 4 re-reads the process-start claude-child rows on 0.9.0 with no value moved, and the env-strip fields here are not the pty_backend or sideload cells the session cut
@answer n=23 entity=span-trace-coverage#12.1 v=no
why: the link product line and its channel-request pair are unchanged, the closed event kinds gained none, and the session reaches no run::links code
@answer n=24 entity=span-trace-coverage#13.1 v=no
why: the explicit link and unlink methods live in run::links, which the session does not reach, and no event kind moved
@answer n=25 entity=span-trace-coverage#14.1 v=yes mode=stale
why: src/cmd/list.rs and src/cmd/hook.rs, the callers that write the state-recovered line, are reached by the session, and the report holds only the event kind, not those call sites
