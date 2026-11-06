@answer n=1 entity=metric-coverage#1.1 v=no
why: the counter groups the closed event and detail catalog, which the session left unchanged (no event kind or detail code added or removed)
@answer n=2 entity=metric-coverage#1.2 v=no
why: the raw duration_ms histogram per event is untouched, and no metric moved in section 5
@answer n=3 entity=metric-coverage#1.3 v=no
why: the budget.read_at and liveness gauges are harness snapshots the session did not move, since the ledger finding moves a row's reading and not the snapshot
@answer n=4 entity=metric-coverage#2.1 v=no
why: hook latency sits in the reached hook.rs, but the perf budgets and the run --perf and gate --require perf green stay unchanged
@answer n=5 entity=metric-coverage#2.2 v=no
why: channel call latency is a raw sample of channel-response duration_ms, and no metric moved
@answer n=6 entity=metric-coverage#2.3 v=no
why: the readback window cites src/run/send.rs, which the session does not reach, and send's logging rules are unchanged
@answer n=7 entity=metric-coverage#2.4 v=no
why: the send-confirmed and send-refused counts keep their logging rules, and the removed Git Bash stderr warning is not a counted line
@answer n=8 entity=metric-coverage#2.5 v=no
why: the wait outcome is channel-response{method:"wait"}, whose logging rules the CLI-1 wait scenario keeps unchanged
@answer n=9 entity=metric-coverage#2.6 v=no
why: the state-recovered detail is a catalog value the session did not add or remove, and no metric moved
@answer n=10 entity=metric-coverage#2.7 v=no
why: list --json skipped sits in the reached list.rs, but no metric moved and the claim holds on the changed file
@answer n=11 entity=metric-coverage#2.8 v=no
why: the parse-rejected parser and detail values are catalog entries the session left unchanged
@answer n=12 entity=metric-coverage#2.9 v=no
why: the compat counters read send-refused and channel-response codes whose catalog values and logging rules are unchanged
@answer n=13 entity=metric-coverage#2.10 v=no
why: the process-exit fields come from the PTY handle wait, which the 0.9.0 re-read leaves unchanged with no value moved
@answer n=14 entity=metric-coverage#2.11 v=no
why: the panic counter's must-be-zero invariant and the zero-unlogged-panics rule are unchanged
@answer n=15 entity=metric-coverage#3.1 v=no
why: the server-verify-failed and socket-dir-refused lines keep their wording and rule, and this row carries no PROVISIONAL mark to lose
@answer n=16 entity=metric-coverage#3.2 v=no
why: client::refuse_untrusted sits in the reached client.rs, but no event kind was added, so the refusal still writes no line
@answer n=17 entity=metric-coverage#4.0 v=no
why: the quantile snippet reads hook-decision.duration_ms from the reached hook.rs, but no metric moved and the perf gate stays green
