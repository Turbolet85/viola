@row_add n=1 row=decision/zero-unlogged-panics area=10-slo-invariants-telemetry-budgets rec=1,2,3,4
name: Zero unlogged panics
rule: Every panic in run, hook, mcp and cli after init step 4 produces exactly one single-line ERROR panic record in its home-level role file, with no multi-line backtrace there, and hook still exits 0; retry-once policies for panic handling are forbidden.
guard: G2
except: the bounded exemptions where no file exists, so no line can be written; G2 alone leaves out the fake-agent seam's deliberate panic from its count.
@row_add n=2 row=decision/no-sleep-based-log-waits area=10-slo-invariants-telemetry-budgets rec=5,6
name: No sleep-based log waits
rule: No log-based wait relies on sleeps.
@row_add n=3 row=decision/hook-obs-overhead-budget area=10-slo-invariants-telemetry-budgets rec=7,8
name: Hook observability overhead budget
rule: Obs overhead on hook, logger init being one open plus appends, is held inside the same max < 1.0 s perf gate, which holds only when every hook command runs with VIOLA_NAME set and a home under target/e2e-home.
guard: gate --require perf
@row_add n=4 row=decision/no-print-macros area=11-obs-anti-patterns-never-do-these rec=9,10,11,12
name: No print macros in product code paths
rule: print!, println!, eprint!, eprintln! and dbg! are never used in the hook, run or mcp code paths of the product crates, and every product member inherits the workspace lints.
why: They bypass the file sink and break the Hook contract, the run terminal rule and MCP framing.
guard: clippy print_stdout, print_stderr and dbg_macro at deny
except: viola-harness, the fake agent and the output modules that own --json, human CLI stderr and the hook decision body.
@row_add n=5 row=decision/pinned-actions area=11-obs-anti-patterns-never-do-these rec=13
name: Actions pinned by commit SHA
rule: Actions are never used unpinned; actions/upload-artifact is pinned by commit SHA.
guard: zizmor
@row_add n=6 row=absence/otel-trace-export area=ci-integration rec=14
capability: OTel trace export: no exporter
@row_add n=7 row=contract/cli-internal-error/json-document area=error-capture-reporting rec=15,16
boundary: cli internal error
item: --json document
promise: Under --json, read among the arguments before a --, the cli catch site prints {"v":1,"error":"internal-error","detail":null} on stdout and stderr stays empty.
@row_add n=8 row=contract/mcp-stdin/oversize-line area=error-capture-reporting rec=17,18,19,20
boundary: viola mcp stdin
item: line past MAX_FRAME
promise: A line past MAX_FRAME is refused whole, gives no isError result and ends the session as a closed stdin: exit 0 after initialize, exit 1 before it with both streams empty; its one record is parse-rejected mcp-stdin oversize, no byte of the line.
@row_add n=9 row=contract/hook-decision-detail/server-verify-failed area=log-coverage rec=21,22,23,24
boundary: hook-decision detail
item: server-verify-failed
promise: viola hook writes it at WARN on a server of another euid, a server that is not the recorded wrapper, or a check that cannot be made; no frame is sent, the hook fails open, and on the dialog path it follows hook-invoked with decision_emitted false.
@row_add n=10 row=contract/hook-decision-detail/channel-unreachable area=log-coverage rec=21,22
boundary: hook-decision detail
item: channel-unreachable
promise: Written when the channel cannot be reached, a replayed or absent snapshot included; no frame is sent, the hook fails open and SessionEnd takes its direct append.
@row_add n=11 row=contract/cli-process-exit/server-verify-failed area=metric-coverage rec=25,26,27
boundary: CLI process-exit diagnostic line
item: server-verify-failed
promise: The CLI's server-verification refusal writes one process-exit with subject self, exit_code 21, detail server-verify-failed and during connect at WARN in cli-<name>.ndjson, with no pid, start time, path or endpoint; MCP's wait tool writes no line for it.
