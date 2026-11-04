@answer n=1 entity=metric-coverage#1.1 v=no
why: a report-side counter query recipe for the derived catalog, which the chunk does not need; its home-level and torn-line reading is a reference to G2 that the chunk's gate takes from its own scope item, not a rule its code or tests must act on.

@answer n=2 entity=metric-coverage#1.2 v=no
why: a histogram report recipe over raw samples with sort-based quantiles; the chunk derives no new metric and its perf reading is the hyperfine gate, not this row.

@answer n=3 entity=metric-coverage#1.3 v=no
why: a gauge recipe reading liveness through agent-run status; the chunk's liveness signal and its list --json reader are set by its own item, and this row states no liveness rule the chunk must honour.

@answer n=4 entity=metric-coverage#2.1 v=yes
why: names the hook-decision duration gate as hyperfine max, the reading the chunk's per-OS perf job runs over the timed hook rows; the metric name and buckets are not needed.

@answer n=5 entity=metric-coverage#2.2 v=no
why: a derived channel-latency metric row with no gate; the chunk's channel-response telemetry (corr and conn) does not depend on this duration reading.

@answer n=6 entity=metric-coverage#2.3 v=no
why: send readback latency is send-path behaviour in src/run/send.rs that the chunk neither changes nor names in its scope.

@answer n=7 entity=metric-coverage#2.4 v=no
why: a derived send-outcome counter row; no scope item states send terminal states, and the chunk derives no metric.

@answer n=8 entity=metric-coverage#2.5 v=no
why: a derived wait-outcome counter row; it states no line, detail or refusal the chunk must emit.

@answer n=9 entity=metric-coverage#2.6 v=no
why: a derived recovery counter reading state-recovered.detail; it states no detail word, and the words that must match on every OS are bound by the scope's own item, not by this row.

@answer n=10 entity=metric-coverage#2.7 v=no
why: a derived gauge naming list --json skipped as its source; it states no field value or schema rule that the chunk re-asserts.

@answer n=11 entity=metric-coverage#2.8 v=no
why: a parse-rejection counter for creator-explicit parsers, which the chunk does not touch.

@answer n=12 entity=metric-coverage#2.9 v=no
why: a derived compat counter mapping detail words to metrics; release-from-driver is a settled historical item the scope excludes, and the closed detail catalog is bound by its own item, not by this mapping.

@answer n=13 entity=metric-coverage#2.10 v=no
why: a derived child-exit counter over process-exit fields; it states no exit source or handle-wait rule, which the chunk's PTY-seam item carries.

@answer n=14 entity=metric-coverage#2.11 v=yes
why: states the zero-panic budget (event panic must be 0), which the chunk re-measures through G2 and the in-job gate exit codes.

@answer n=15 entity=metric-coverage#3.1 v=yes
why: states the CLI server-verify and socket-dir refusal lines (process-exit exit 21, detail server-verify-failed or socket-dir-refused, during connect, euid-only on macOS) and that MCP writes none, which the chunk's Unix endpoint telemetry and macOS carve-out must produce.

@answer n=16 entity=metric-coverage#3.2 v=yes
why: states that a CLI wait refused by the strict-modes check writes no line, a Unix refusal path that the chunk's closed detail catalog (strict-modes-failed) and channel telemetry must honour.

@answer n=17 entity=metric-coverage#4.0 v=yes
why: states the gate statistic as max (the gates use max, quantiles are report-only), which the chunk's per-OS hyperfine max perf job must run; the quantile snippet itself is not needed.
