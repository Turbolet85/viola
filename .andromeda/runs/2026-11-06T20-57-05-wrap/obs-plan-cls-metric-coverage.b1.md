@answer n=1 entity=metric-coverage#1.1 v=no
why: the Counter derives from send-refused grouping whose event and detail values the session did not change, and section 5 records no metric moved
@answer n=2 entity=metric-coverage#1.2 v=no
why: the Histogram samples duration_ms per event, and no event kind moved and section 5 records no metric moved
@answer n=3 entity=metric-coverage#1.3 v=no
why: the Gauge snapshots budget.read_at and liveness, and the session changes the statusline ledger reading, not the read_at snapshot
@answer n=4 entity=metric-coverage#2.1 v=no
why: hook-decision duration_ms in src/cmd/hook.rs keeps its gate, and the session re-ran perf green with budgets unchanged
@answer n=5 entity=metric-coverage#2.2 v=no
why: channel-response duration_ms keeps its logging, and section 5 records no metric moved
@answer n=6 entity=metric-coverage#2.3 v=no
why: it cites src/run/send.rs, which the change does not reach, and send logging rules are unchanged
@answer n=7 entity=metric-coverage#2.4 v=no
why: the send-confirmed and send-refused lines are unchanged, and only the Git Bash stderr warning left send without being a logged event
@answer n=8 entity=metric-coverage#2.5 v=no
why: the channel-response wait outcome and the wait logging rules are unchanged by the session
@answer n=9 entity=metric-coverage#2.6 v=no
why: the state-recovered event and its detail are unchanged, and section 5 records no metric moved despite the chaos test file being cleaned
@answer n=10 entity=metric-coverage#2.7 v=no
why: the list --json skipped field keeps its meaning, and section 5 records no metric moved despite list.rs being cleaned
@answer n=11 entity=metric-coverage#2.8 v=no
why: parse-rejected and its parser and detail values are unchanged, since no event kind or detail code was added or removed
@answer n=12 entity=metric-coverage#2.9 v=no
why: the send-refused unknown and invalid-params and release-from-driver detail codes are unchanged, since no detail code was added or removed
@answer n=13 entity=metric-coverage#2.10 v=no
why: the process-exit fields were re-read on 0.9.0 with exit from the handle wait unchanged and no value moved
@answer n=14 entity=metric-coverage#2.11 v=no
why: the panic counter must stay zero and the zero-unlogged-panics invariant is unchanged per section 10
@answer n=15 entity=metric-coverage#3.1 v=no
why: the socket-dir-refused row carries no PROVISIONAL mark, and the socket-directory pair wording was confirmed with its rule unchanged
@answer n=16 entity=metric-coverage#3.2 v=no
why: the strict-modes refusal rule (exit 21, no line written) is unchanged and the row carries no PROVISIONAL mark to lift
@answer n=17 entity=metric-coverage#4.0 v=no
why: the hook-decision quantile snippet reads unchanged output, and perf green with budgets unchanged keeps its gates
