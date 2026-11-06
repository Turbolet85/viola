@answer n=1 entity=metric-coverage#1.1 v=yes
why: the counter's home-level-only reading (detail files excluded) is the evidence read of the live sessions (item 10) and the detail-*.ndjson drop before any upload (item 11)
@answer n=2 entity=metric-coverage#1.2 v=no
why: a histogram of raw duration_ms samples; no scope item names latency samples
@answer n=3 entity=metric-coverage#1.3 v=yes
why: the liveness gauge is read via agent-run status, the call the live-session evidence reads through (item 10); its budget.read_at half is out of scope
@answer n=4 entity=metric-coverage#2.1 v=no
why: hook latency histogram and its hyperfine gate are not named by the chunk's scope or its gates; the perf.rs edits are EXE_SUFFIX joins only
@answer n=5 entity=metric-coverage#2.2 v=no
why: channel-call latency from channel-response lines; the chunk adds or changes no channel method or response
@answer n=6 entity=metric-coverage#2.3 v=no
why: send-to-readback latency; no scope item names the send path or send-confirmed lines
@answer n=7 entity=metric-coverage#2.4 v=no
why: send terminal-state counter; no scope item names send-confirmed or send-refused
@answer n=8 entity=metric-coverage#2.5 v=no
why: wait outcome counter over channel-response; the chunk changes no wait verb or its responses
@answer n=9 entity=metric-coverage#2.6 v=no
why: state-recovered counter from the torn-append heal, which not_needed item 5 places out of the chunk
@answer n=10 entity=metric-coverage#2.7 v=no
why: skipped-records gauge from list --json; no scope item touches the list verb or its skipped count
@answer n=11 entity=metric-coverage#2.8 v=no
why: parse-rejected counter per parser; no scope item names a parser or parse-rejected line
@answer n=12 entity=metric-coverage#2.9 v=no
why: unknown and v-rejected compat counters; no scope item names the refusal or version-rejection paths
@answer n=13 entity=metric-coverage#2.10 v=yes
why: the child-exit counter derives from process-exit{subject, child_exit_status, exit_source}, the exit path item 6 keeps through the portable-pty bump, including exit_source kill-fallback
@answer n=14 entity=metric-coverage#2.11 v=yes
why: the panic counter must read 0 on event:"panic", the zero error budget the done-check must clear (item 8, G2)
@answer n=15 entity=metric-coverage#3.1 v=yes
why: it names the socket-dir-refused detail code and its process-exit line, the socket-directory refusal wording item 14 ratifies or re-words
@answer n=16 entity=metric-coverage#3.2 v=no
why: the strict-modes refusal of client::refuse_untrusted is not the socket-directory refusal pair item 14 names, and no scope item touches it
@answer n=17 entity=metric-coverage#4.0 v=no
why: the report-only quantile snippet reads hook-decision duration_ms; no scope item names hook latency
