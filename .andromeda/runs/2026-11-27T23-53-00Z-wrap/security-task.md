@task_prose part=session
The session split three oversized parts as pure moves under the Epoch 5 audit's size and complexity ceilings: viola up's planner, the pty pump's respawn, and viola run's window and trust steps.
viola up's single body was split into three private steps (select, plan, act) under the same cli.up span; up still calls it alone.
The pump's respawn (the child swap, the reap within its bound, the drain) moved into its own private module of the pty crate, and the pump loop took one helper; the crate's exported names are unchanged.
viola run's window step moved beside the window module and its trust-folder step moved into the trust record component; the trust-record INFO line keeps its event and detail word but its log target now reads viola::trust_record (was viola::cmd::run).
No public API, IPC method, endpoint, env var, socket, port, dependency, config or schema changed; nothing was retired; case counts are unchanged and 159 moved bodies were measured equal.
@task_file n=1 path=src/cmd/up.rs
what: viola up, the command that starts a set of instances; its checks before its log and before any spawn, now split into select, plan and act steps (edited)
@task_file n=2 path=crates/viola-pty/src/pump.rs
what: the PTY pump, the passthrough loop that copies child output, forwards resizes, writes bracketed pastes and keeps the wheel between human typing and the driver (edited)
@task_file n=3 path=crates/viola-pty/src/pump/restart.rs
what: the pump's respawn: swapping the wrapper's own child for a new one, reaping the old child within its bound and draining its output (new, moved from the pump)
@task_file n=4 path=crates/viola-pty/src/pump/test_cases.rs
what: the pump's unit tests: output drain, resize forwarding, worker panic kill, writer end (new, moved from the pump)
@task_file n=5 path=crates/viola-pty/src/pump/test_cases_paste.rs
what: the pump's paste tests: one bracketed paste written whole, a cut paste, human keys waiting behind a paste, refused pastes (new, moved from the pump)
@task_file n=6 path=crates/viola-pty/src/pump/test_cases_respawn.rs
what: the pump's respawn tests: the child swap, a child ignoring the signal, a failed spawn, the reap bound, the exit drain (new, moved from the pump)
@task_file n=7 path=src/cmd/run.rs
what: viola run, the wrapper's start shared by run, revive and adopt; its version gate, collision check, window step and trust step call sites (edited)
@task_file n=8 path=src/cmd/run/test_cases.rs
what: viola run's unit tests: the channel methods' routing, the release-from-driver refusal, hook event re-validation, the collision check over a replayed snapshot (new, moved from run)
@task_file n=9 path=src/cmd/run/test_cases_start.rs
what: viola run's start tests: the first snapshot's links, cwd, lineage, launch pair and window lines, and the hidden window, lineage and launch flags' parsing (new, moved from run)
@task_file n=10 path=src/run/window.rs
what: the wrapper's window step, which moves only its own terminal's window by the recorded origin and place; now holds open_window (edited)
@task_file n=11 path=src/trust_record.rs
what: the trust record writer, which writes the folder-trust flag at a verified start and skips the home and already trusted folders; now holds the trust-folder step and its closed outcome word (edited)
@task_file n=12 path=src/cmd/mod.rs
what: the CLI command module tree; viola run's module widened to crate visibility (edited)
@task_item n=1 part=touches
decision/up-checks-before-log-and-spawn: viola up's checks now sit across the new select, plan and act steps; the order before its log and before any spawn is held by a pure split, read it against the row.
@task_item n=2 part=touches
decision/switch-ends-only-its-own-child: the respawn that switch drives moved whole into the pump's own restart module; it still ends only the wrapper's own child.
@task_item n=3 part=touches
decision/human-typing-takes-wheel and decision/bridge-send-one-bracketed-paste and decision/send-text-is-one-bracketed-paste: the pump loop took a helper and its paste tests moved to their own file; the behaviour and the cases are unchanged.
@task_item n=4 part=touches
decision/window-step-moves-own-terminal-window and decision/hyprland-recorder-moves-own-window: the window step moved from viola run into the window module unchanged.
@task_item n=5 part=touches
decision/run-window-flags-checked and decision/window-origin-authenticates-nothing: the window ask with its origin and place fields widened to crate visibility so the window module can name it; the checks did not change.
@task_item n=6 part=touches
decision/start-writes-trust-record-flag-only and decision/start-trust-skips-trusted-folders and decision/trust-record-writer-guarded: the trust-folder step moved from viola run into the trust record component; only the start calls it.
@task_item n=7 part=touches
decision/trust-record-closed-words: the trust outcome word step moved beside the trust record writer, still private; the closed words did not change.
@task_item n=8 part=touches
The trust-record INFO log line's target moved to viola::trust_record from viola::cmd::run; any row naming the log line's target or its module path is touched, and the log filter lists the crate prefix viola so the line is still written.
@task_item n=9 part=touches
decision/verify-never-answers-import-dialog and contract/verify-refusal-codes/untrusted-folder: indirectly, through the trust step a verified start runs, which moved; verify's own callers did not change.
@task_item n=10 part=touches
decision/collision-check-needs-no-server-verify and decision/driver-release-refused and decision/release-from-driver-guard: their unit tests moved into viola run's test files; the checks and the cases are unchanged.
@task_item n=11 part=touches
decision/test-seams-fake-agent-only and decision/fake-agent-pump-delay-seam: the pump and run test modules now live in separate path files under the same module paths; the seams did not move.
@task_item n=12 part=touches
decision/mutation-testing-at-audit: the Epoch 5 audit's defects these splits close (files at or under 800 code lines, functions at or under cognitive 15); no row holds those figures.
@task_item n=1 part=untouched
Dependency security: no dependency, pin, toolchain, licence, advisory or rmcp feature changed.
@task_item n=2 part=untouched
Secret management: no environment strip, credential handling or canary scan changed.
@task_item n=3 part=untouched
The MCP server, its tools, its stdin cap and its errors: nothing in the MCP path changed.
@task_item n=4 part=untouched
viola hook and the hook payload contract: no hook parsing, frame or exit behaviour changed.
@task_item n=5 part=untouched
CI, release, fuzz and workflow rows: the harness and gate surface did not change.
@task_item n=6 part=untouched
The absences: no capability was added or removed.
@task_item n=7 part=untouched
viola list and the board scan, the pinned binary copy and its hash, and recorded fixtures: none was reached.
@task_item n=8 part=untouched
The IPC client seams: peer euid checks, server verification and the client refusal order did not change.
