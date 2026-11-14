@task_prose part=chunk
Pins by tests the one case missing on both MAX_FRAME bounds of the viola-hyprland seam: a hyprctl reply of exactly MAX_FRAME bytes is accepted, one byte more is oversize.
Site one is the bounded read of the hyprctl child's reply (read_within), driven through the fake hyprctl; site two is the dispatch reply parse (parse_dispatch), where MAX_FRAME bytes give Refused and MAX_FRAME + 1 give oversize.
Each new guard test carries a one-off remove-the-guard control mutating its site (> to >=), so the next audit's mutation of either site is caught.
Test-only: no product line changes unless a test proves one wrong; no mutation run, no requirement added, no capability claimed.
It re-reads that the clients reply parse's bound is already pinned by its existing exact-frame test, and leaves it as is.
@task_file n=1 path=crates/viola-hyprland/src/read.rs mark=edits
what: the Hyprland seam's bounded read of the hyprctl child's reply, capped at MAX_FRAME and refusing more as oversize, with its test module driving the fake hyprctl — edits: its test module (a new exact-frame test of read_within)
@task_file n=2 path=crates/viola-hyprland/src/reply.rs mark=edits
what: the Hyprland seam's reply parsers for hyprctl output (parse_dispatch, parse_clients), each bounded by MAX_FRAME with oversize past it, and their tests — edits: its test module (a new exact-frame test of parse_dispatch)
@task_item n=1 part=needs
The bounded read of the hyprctl reply: a reply of exactly MAX_FRAME bytes is accepted, MAX_FRAME + 1 bytes is the oversize error; the stdout drain reads at most MAX_FRAME + 1 bytes.
@task_item n=2 part=needs
The dispatch reply parse: exactly MAX_FRAME bytes is not oversize (it reads Refused, not being the success text), MAX_FRAME + 1 bytes is oversize.
@task_item n=3 part=needs
The MAX_FRAME value as the one bound shared by the hyprctl read and the hyprctl reply parsers; the tests build their replies from that constant, not a copied number.
@task_item n=4 part=needs
The oversize outcome of a hyprctl reply past the bound: the existing closed oversize word, the chunk adds no new detail word, event, field or log line.
@task_item n=5 part=needs
The hyprctl child spawn as the read's test reaches it: run through the fake hyprctl only, never a real hyprctl, a live Hyprland desktop or a live claude start.
@task_item n=6 part=needs
Each new guard test's one-off remove-the-guard control: the site mutated (> to >=) through the gate's control step under a kill deadline, the case named by the test's name alone, reached past fail-fast, holding only when that case ran and failed; the mutated site is never committed.
@task_item n=7 part=needs
No mutation run, local or dispatched, inside the chunk: the audit's two surviving mutants are answered by tests that would catch them, and no mutation output or diff is produced for upload.
@task_item n=8 part=needs
The hyprctl reply parse that already reads a parse failure as a rejected reply (hyprctl-json parser): the new tests prove the bound without changing how a rejected or oversize reply is reported.
@task_item n=9 part=needs
Test-only rule: a product line changes only where a new test proves it wrong, and then as its own item with its own acceptance; any change keeps the seam's fixed, pathless error messages.
@task_item n=10 part=needs
Every statement about how hyprctl, the crate or the compositor behaves is measured and cited, or marked unmeasurable; the tests assert on the error variant, never on message text.
@task_standing n=1
convention: each new test that runs a child or writes diagnostics keeps its home under the one scanned test-home root, kept until the gate steps run, and its emitted lines pass the schema and zero-panic gates
home: ci-integration · 9-ci-integration
@task_standing n=2
convention: each new fixture or fake reply is synthetic bytes, never recorded user content, and no fixture is a telemetry source
home: pii-scrubbing-compliance · 8-pii-scrubbing-compliance
@task_standing n=3
convention: a surviving mutant is a failure, so each new guard test must kill the mutant of its site, proved by its control
home: slo-invariants-telemetry-budgets · 10-slo-invariants-telemetry-budgets
@task_item n=1 part=not_needed
The viola mcp stdin MAX_FRAME contracts (line within the bound, oversize line, oversize before initialize, the mcp-stdin parse-rejected line): a different boundary, the MCP server's stdin framing, which this chunk neither reads nor tests.
@task_item n=2 part=not_needed
Channel JSON-RPC frames, request params, response errors and the conn/corr join between processes: the chunk touches no local socket or channel line.
@task_item n=3 part=not_needed
Send, wait, last, dialog, wheel, pause and release paths, with their log fields, span chains and refusal detail words: no product path changes.
@task_item n=4 part=not_needed
Exit-code and exit-cause detail tables (exit 1, 20, 21, socket-dir-refused, strict-modes-failed, server-verify-failed) and the CLI stderr pairs and --json documents: no CLI verb is touched.
@task_item n=5 part=not_needed
Hook and statusline behaviour, hook stdout and the hook and statusline perf budgets: the chunk adds no hook work.
@task_item n=6 part=not_needed
Panic capture outcomes by role, the panic line's keys and the panic hook's install order: no panic path changes.
@task_item n=7 part=not_needed
Error reporters, OTel exporters, OTLP or pulse-class receivers, network egress and hosted deployments: absences the chunk cannot reach.
@task_item n=8 part=not_needed
config.json keys, the budget gate and budget span attributes, heartbeat liveness and the list or status views: the chunk leaves configuration and state untouched.
@task_item n=9 part=not_needed
Service identity, process-start fields and the run start span attributes and process log: no process start changes.
@task_item n=10 part=not_needed
The agent-run logs harness command and its merge, filter and torn-line rules: the chunk reads no logs through it.
