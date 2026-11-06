@answer n=1 entity=span-trace-coverage#5.1 v=yes mode=stale
why: crates/viola-state/src/pin.rs is a reached file (task file n=16) and the pin_copy placement claim lives in it, not shown to hold
@answer n=2 entity=span-trace-coverage#6.1 v=yes mode=stale
why: send.rs, client.rs and hook.rs are reached files (task files n=6, 7, 8) and the send instance check and server check live in them
@answer n=3 entity=span-trace-coverage#6.2 v=yes mode=stale
why: send.rs is a reached file and the pty.paste_write text_bytes count, with its CR and LF normalization, lives in it
@answer n=4 entity=span-trace-coverage#6.3 v=yes mode=stale
why: send.rs and client.rs are reached files and the send-refused and channel-response log fields live in them
@answer n=5 entity=span-trace-coverage#6.4 v=yes mode=stale
why: client.rs is a reached file and the send.client process-exit code list, including exit 21 details, lives in it
@answer n=6 entity=span-trace-coverage#7.1 v=yes mode=stale
why: client.rs is a reached file and the wait.block and last.client spans with their server check live in it
@answer n=7 entity=span-trace-coverage#7.2 v=yes mode=stale
why: client.rs is a reached file and the wait.block attributes after and timeout_ms live in it
@answer n=8 entity=span-trace-coverage#7.3 v=yes mode=stale
why: client.rs is a reached file and the wait and last channel-request, channel-response and exit-21 detail lines live in it
@answer n=9 entity=span-trace-coverage#7.4 v=yes mode=stale
why: client.rs is a reached file and the wait.block cleanup on socket-directory and server-verification refusal lives in it
@answer n=10 entity=span-trace-coverage#8.1 v=yes mode=stale
why: hook.rs and client.rs are reached files and the dialog hook server check and the CLI answer path live in them
@answer n=11 entity=span-trace-coverage#8.2 v=yes mode=stale
why: hook.rs is a reached file and the hook.decision_emit attribute of the dialog path lives in it
@answer n=12 entity=span-trace-coverage#8.3 v=yes mode=stale
why: hook.rs is a reached file and its fail-open dialog log lines, with the socket-directory and server-verification refusals, live in it
@answer n=13 entity=span-trace-coverage#8.4 v=yes mode=stale
why: hook.rs is a reached file and its dialog read backstop and hook-decision line live in it
@answer n=14 entity=span-trace-coverage#9.1 v=yes mode=stale
why: client.rs is a reached file and the pause and release client server check lives in it
@answer n=15 entity=span-trace-coverage#9.2 v=no
why: the run.wheel_transition span lives in the run wheel worker, which is in no reached file, and the session changes none of its attributes
@answer n=16 entity=span-trace-coverage#9.3 v=yes mode=stale
why: client.rs is a reached file and the release client refusals and channel lines live in it
@answer n=17 entity=span-trace-coverage#9.4 v=no
why: run.wheel_transition is a point-in-time span and the wheel audit event, neither in a reached file and neither moved by the session
@answer n=18 entity=span-trace-coverage#10.1 v=no
why: the hook.statusline, state.budget_write and run.budget_eval spans live in statusline and budget code, none of them a reached file
@answer n=19 entity=span-trace-coverage#10.2 v=yes mode=add-to
why: F1 names the statusline-rate-limits ledger row reading the account plan, with five_hour unknown and only seven_day on claude 2.1.287, a fact that joins the state.budget_write five_hour_pct and seven_day_pct attributes
@answer n=20 entity=span-trace-coverage#10.3 v=yes mode=stale
why: src/cmd/hook.rs is a reached file and the statusline reject_stdin and hook-decision lines are written from it
@answer n=21 entity=span-trace-coverage#10.4 v=yes mode=stale
why: src/cmd/hook.rs is a reached file and the STATUSLINE_DEADLINE statusline bound lives in it
@answer n=22 entity=span-trace-coverage#11.1 v=yes mode=stale
why: crates/viola-agent-claude/src/lib.rs is a reached file and the claude-child environment strip and its Windows-measured IDENTITY_FLOOR live in it
@answer n=23 entity=span-trace-coverage#12.1 v=no
why: the link and unlink records live in run::links, which is in no reached file, and the session adds no link behavior
@answer n=24 entity=span-trace-coverage#13.1 v=no
why: the explicit link and unlink methods live in run code, which is in no reached file, and the session changes none of them
@answer n=25 entity=span-trace-coverage#14.1 v=yes mode=stale
why: src/cmd/list.rs and src/cmd/hook.rs are reached files and name the state-recovered callers that live in them
