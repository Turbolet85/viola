@task_prose part=chunk
Three pure moves that the Epoch 5 code audit ranked sixth. None of them changes behaviour.
Move 1 splits the viola up fleet re-open loop into a plan part (refusal, order, skip) and an act part (spawn, wait, line), so every function sits at cognitive 15 or under.
Move 2 takes the switch's respawn (restart) code out of the PTY pump into its own module of the PTY crate, and moves the pump's inline test module out too, so no file is over 800 code lines.
Move 3 takes the start's window step and trust step out of the shared start of run, revive and adopt to sit beside the window code and the trust record, and moves its test module out, so no file is over 800 code lines.
It re-measures tokei code lines and rust-code-analysis cognitive sums over every file it moves or creates, and re-runs all test layers green with equal test-case counts.
@task_file n=1 path=src/cmd/up.rs mark=edits
what: viola up, the fleet re-open command and its tests — edits: the function upped, split into its plan and its act
@task_file n=2 path=crates/viola-pty/src/pump.rs mark=edits
what: the PTY pump, the main loop that carries the PTY byte stream, with its inline test module — edits: the switch's respawn arm taken out of Pumping::run, the inline test module moved out
@task_file n=3 path=src/cmd/run.rs mark=edits
what: the shared start of run, revive and adopt (and through them open and up), with its test module — edits: run::start's window step call (open_window) and its trust step moved out, the test module moved out
@task_file n=4 path=src/run/window.rs
what: the start's window step, where the window is opened and the view is kept; the moved window and trust step code goes beside it
@task_item n=1 part=needs
The restructure's invariant. Three sets of units move or split. First, upped's fleet re-open: refusal, order, skip, then spawn, wait and the line it writes. Second, the switch's respawn in the PTY pump and the pump's tests. Third, the start's window step and trust step and the start's tests. Each keeps what it does, including the lines and spans it writes. The whole test layers green with no test body edited hold this, and so do test-case counts that are equal before and after the move.
@task_item n=2 part=needs
The measure the next audit reads. Size is tokei code lines per Rust file, with a file's inline test module counted in it, and no file may exceed 800. Complexity is the rust-code-analysis cognitive sum per function, and no function may exceed 15. Each moved file, each new file and each file left behind must stay under both ceilings.
@task_item n=3 part=needs
Where moved code may live. The respawn goes into a new module of the PTY crate, declared in that crate's mod tree. The window and trust steps go beside the window code and the trust record in the run layer. A new file exposes only what its callers already reach.
@task_item n=4 part=needs
How tests reach moved units. A moved test module may change only its path, module or name references. Each such re-point is named in the plan. Every moved function keeps its tests with it, so the diff mutation sites stay caught.
@task_item n=5 part=needs
Re-pointing. Wherever a test, a contract census, a lint list or a mod tree reads a path, module name or text that a move changes, it is re-pointed. A master row naming a moved path becomes an expected wrap amendment.
@task_item n=6 part=needs
The start span chain over the moved window step. The run.window span and its attribute allow-list stay attached to the moved window call. The window line opens no trace of its own.
@task_item n=7 part=needs
The trust-record log line that the start writes, and the trust-record write and removal, which stay uninstrumented. The moved trust step keeps writing that one line, with the same fields and in the same order among the run start events.
@task_item n=8 part=needs
Span rules for code that moves or splits: manual spans only, static low-cardinality span names, every instrument attribute skips all arguments, no err or ret span options, and no endpoint, path or env values on start spans.
@task_item n=9 part=needs
Event-emission rules for moved code: obs_event! is the only event macro, no raw tracing macros outside the obs module, no print macros in product code paths, and corr, conn and instance are passed explicitly. The split of upped adds no log line.
@task_item n=10 part=needs
Respawn in the pump: the PTY byte stream is never logged, there is no info logging in hot loops and no hot-path instrumentation, and the PTY seam spawn and kill-fallback log events stay where the respawn calls them.
@task_item n=11 part=needs
Panic containment over the moved pump code. Run threads stay inside catch_unwind, a run worker panic ends the session, and vt100 feed panics stay contained. The respawn arm stays on the thread whose catch site decides the exit.
@task_item n=12 part=needs
The viola up process log and its exit-1 process-exit detail. The plan part of upped (refusal, order, skip) and the act part (spawn, wait, line) write the same lines and detail codes as before.
@task_item n=13 part=needs
Error handling across the new function and module seams. Errors are logged once at module boundaries, anyhow is used only at the dispatch edge, typed errors keep fixed Display messages, and a new seam adds no second log line for an error.
@task_item n=14 part=needs
Obs code is mutation-tested like product code. A move changes the sites that the next audit's diff mutation run reads. No test may be deleted, merged or weakened to fit a size bound. No mutation run is planned or run in this chunk.
@task_item n=15 part=needs
The boot path changes through the shared start, so the harness smoke trio is held as operator entries. No live start, no window, no compositor dispatch and no real claude runs. Nothing of the operator's real home is read.
@task_standing n=1
convention: Each test file that a moved test module lands in joins the OS-gated test census.
home: ci-integration
@task_standing n=2
convention: Each integration test that creates a test home keeps that home under the one scanned E2E root, where it stays until the gates run.
home: 9-ci-integration · ci-integration
@task_standing n=3
convention: No test waits on a log line by sleeping.
home: 10-slo-invariants-telemetry-budgets
@task_standing n=4
convention: Each new product source file uses the obs_event macro, no raw tracing or print macros, and only instrument attributes that skip all arguments.
home: ci-integration · 11-obs-anti-patterns-never-do-these
@task_item n=1 part=not_needed
The MCP stdio server: its tools, frames, stdin bounds and refusal results. No move reaches the MCP role.
@task_item n=2 part=not_needed
The channel JSON-RPC wire: request params, error codes, response detail words and peer-credential checks. No move changes the channel.
@task_item n=3 part=not_needed
The dialog and hook path: hook stdout, fail-open, hook-decision detail and hook latency budgets. The chunk touches no hook.
@task_item n=4 part=not_needed
The send, wait, last and wheel paths: their spans, log fields, refusal details and confirm window. The chunk moves none of them.
@task_item n=5 part=not_needed
Exporters and reporters: OTel SDK, OTLP, Sentry, a future error reporter, network egress and their absences. The chunk adds no dependency and no egress.
@task_item n=6 part=not_needed
Derived metrics: counters, gauges, histograms, quantiles and their closed-enum grouping. The chunk adds no metric.
@task_item n=7 part=not_needed
The agent-run and harness logs output, list liveness and the config.json keys. No move reads or writes them.
