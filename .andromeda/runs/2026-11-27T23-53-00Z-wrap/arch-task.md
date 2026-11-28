@task_prose part=session
The session closed the three Epoch 5 splits as pure moves under the audit's size and complexity ceilings: no behaviour, public API, IPC method, endpoint, env var, socket, schema or config changed.
The up command's body was split into three private steps (select, plan, act); the up entry function keeps its span and stays the verb's sole callee.
The PTY pump's respawn code moved into a new private restart module of the viola-pty crate, re-exported unchanged; the pump loop took one helper that computes the next respawn request.
The run start's window-opening step moved into the window module and its trust-folder step into the trust-record module; the run command module was widened to crate visibility so the window module can name the window ask.
The trust-record INFO log line is still written once per verified start with the same event and detail word, but its tracing target now reads viola::trust_record instead of viola::cmd::run.
Measured: every touched file at or under 800 code lines, every function at or under cognitive 15; test case counts unchanged; nothing retired.
@task_file n=1 path=src/cmd/up.rs
what: the up verb, which re-opens instances gone before the boot, parents first, writing no state of its own; its body now split into select, plan and act steps
@task_file n=2 path=crates/viola-pty/src/pump.rs
what: the PTY pump of the viola-pty crate, the passthrough loop copying child output and human keys, resizes and paste, now re-exporting the respawn surface from a private submodule
@task_file n=3 path=crates/viola-pty/src/pump/restart.rs
what: new private module of the PTY pump holding the child respawn (swap the child in place, reap within a bound, drain the old output)
@task_file n=4 path=crates/viola-pty/src/pump/test_cases.rs
what: new home of the PTY pump's unit tests (the same test module path), the pump's passthrough, resize and drain cases
@task_file n=5 path=crates/viola-pty/src/pump/test_cases_paste.rs
what: new test submodule of the PTY pump holding the paste-bound and bracketed-paste cases
@task_file n=6 path=crates/viola-pty/src/pump/test_cases_respawn.rs
what: new test submodule of the PTY pump holding the respawn cases and their helpers
@task_file n=7 path=src/cmd/run.rs
what: the run start path shared by run, revive and adopt, in the start order that spawns the child last; its window and trust steps moved out
@task_file n=8 path=src/run/window.rs
what: the start's window step on the Hyprland compositor seam, now also holding the step that opens the instance window
@task_file n=9 path=src/trust_record.rs
what: the trust-record writer that writes and removes the folder trust record, now also holding the start's trust-folder step and its code word
@task_file n=10 path=src/cmd/mod.rs
what: the CLI command module tree of the root bin, its run command module widened to crate visibility
@task_file n=11 path=src/cmd/run/test_cases.rs
what: new home of the run command's unit tests (the same test module path): channel methods routing, hook events, collision check, start spans
@task_file n=12 path=src/cmd/run/test_cases_start.rs
what: new test submodule of the run command holding the start-state, snapshot cwd, run-arguments and lineage cases
@task_item n=1 part=touches
The process-log line of the trust record changed its tracing target from viola::cmd::run to viola::trust_record: any row on codes-only process logs, tracing process logs or the per-module log filter that names the target of that line is touched; the crate prefix viola still covers it.
@task_item n=2 part=touches
The trust-record ledger row and verify's own trust record: the start's trust-folder step now lives with the trust-record writer, behaviour and code words unchanged.
@task_item n=3 part=touches
The trust writer code words: the start's word for the trust step is now chosen inside the trust-record module; the closed set is unchanged.
@task_item n=4 part=touches
The run start order (child spawned last) and run pinning the bytes it checked: the start path was restructured by the window and trust steps moving out, order unchanged.
@task_item n=5 part=touches
Window features targeting Hyprland and the Hyprland seam: the step that opens the instance window moved into the window module and is now crate-visible.
@task_item n=6 part=touches
The up selection rule, up re-opening parents before children and up writing no state: the up verb's body was split into select, plan and act, behaviour unchanged.
@task_item n=7 part=touches
Model and effort switched in place and revive resuming in place: both rest on the child respawn, which moved into a new private restart module of the pump.
@task_item n=8 part=touches
viola-pty depending on no viola crate and knowing no agent, and portable-pty behind the pty seam: the crate gained a private restart module, its exported names unchanged.
@task_item n=9 part=touches
Passthrough never waiting on the screen model, a human key never held behind a resize, and the paste bound: the pump loop now takes one helper for the next respawn request.
@task_item n=10 part=touches
No orphan modules: new modules were added (the pump's restart module, the pump's paste and respawn test modules, the run command's start test module), each wired into its parent.
@task_item n=11 part=touches
Module dependency graph as a review: the run command module is now crate-visible, and the window module and the trust-record module now carry steps of the start.
@task_item n=12 part=touches
Mutation testing at the epoch boundary: the three splits close item 6 of the Epoch 5 audit's ranked defects.
@task_item n=1 part=untouched
Wire contracts: channel methods, frames, fault codes, refusal reasons, MCP tools and results, CLI verbs, exit codes and --json documents are unchanged.
@task_item n=2 part=untouched
On-disk state: the events log, instance snapshot, ledger stamps, config.json and their field sets are unchanged.
@task_item n=3 part=untouched
Hooks, the plugin, the capability ledger rows other than the trust record, and the dialog hold are not reached.
@task_item n=4 part=untouched
Stack, dependencies, toolchain, cargo-deny policy, CI workflows, coverage floors and the pre-push gate are unchanged; no new dependency or dev tool.
@task_item n=5 part=untouched
Occupied names: no binary, crate, command, environment variable, socket or other name was taken or freed.
@task_item n=6 part=untouched
Cognitive-accessibility rows share only the word cognitive with the complexity measure; no row of architecture holds a file size or a cognitive bound.
@task_item n=7 part=untouched
Rows that use the word target in another sense (a CLI or MCP target instance, a build or fuzz target, the OS target) are not touched by the log line's target move.
