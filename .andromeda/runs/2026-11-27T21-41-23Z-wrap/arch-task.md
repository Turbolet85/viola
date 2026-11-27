@task_prose part=session
Epoch 5 cleanup: the code audit's ranked defects mended, with no product behaviour changed and no IPC method, endpoint, export, port, socket or env var added or changed.
The start shared by run, revive and adopt now goes through one start prologue in the run verb: the instance trust check, then the checked executable, then the wrapper log open, in that order. revive and adopt each carried their own trust check before this; now there is one.
open and up now share one refused-window-step writer. up's private copy is retired, and the error-body rows that witnessed up's refusals now point at open's.
The PTY pump's exit drain takes its bound from the pump's seams, so the product bound stays 500 ms. The liveness parent-chain walk is split out of ancestors, which keeps its signature, its one process-table read and the 8-pid limit at its callers. Tests that kill mutants were added for the pump, the board scan, the replay and the walk.
The nextest verify class (20 s kill, 4 tries, in the ci and mutants profiles) now also takes six tests by name: the path4 dialog-rows test and five CLI version-gate run tests. With that, the root crate's mutation baseline passes: 372 in-diff mutants, 326 caught, 0 missed, 0 timeout, 46 unviable, score 100.0, filed as an addendum to the audit's reading.
The R2 window-placement script, test-only and fired by the operator, now refuses with a red precondition verdict, before anything is made or started, when its scratch home is not fresh or the desktop is locked. The three file splits were not built; they move to their own entry.
@task_file n=1 path=.config/nextest.toml
what: the test runner's profiles and per-class kill bounds — the verify class's filter line in the ci and mutants profiles, and the mutants profile's default timeout
@task_file n=2 path=tests/contract_lints.rs
what: the root crate's contract lint tests — the pin over the nextest verify class's filter line
@task_file n=3 path=crates/viola-pty/src/pump.rs
what: the PTY pump of viola-pty — copying child output to the host, respawn, and the bounded drain of child output in flight at the child's exit
@task_file n=4 path=crates/viola-state/src/liveness.rs
what: session liveness in viola-state — the process parent-chain walk read through sysinfo, behind ancestors, used by the R8 parent-identity floor
@task_file n=5 path=crates/viola-state/src/board.rs
what: the board read model in viola-state — the scan of the checked instances behind list (tests only)
@task_file n=6 path=crates/viola-state/src/replay.rs
what: the snapshot replay from the events log in viola-state — the ended-after-last-start read (tests only)
@task_file n=7 path=scripts/r2-open.sh
what: the operator-fired, test-only R2 live script that proves window placement on Hyprland for open, and its exit cleanup of the windows it made
@task_file n=8 path=src/cmd/run.rs
what: the run verb and the start path shared by run, revive and adopt — instance trust check, checked and pinned executable, wrapper log open, child spawned last
@task_file n=9 path=src/cmd/revive.rs
what: the revive verb — resuming a gone instance in place through the start path, and its list arm
@task_file n=10 path=src/cmd/adopt.rs
what: the adopt verb — taking an existing claude session under a wrapper through the start path
@task_file n=11 path=src/cmd/open.rs
what: the open verb — opening an instance's window on Hyprland, and its refusal writer for a refused window step
@task_file n=12 path=src/cmd/up.rs
what: the up verb — reopening the instances gone before the boot, parents first, through open's window step
@task_file n=13 path=tests/contract_error_bodies.rs
what: the contract test holding each CLI error body, with its code, exit and witness constant, for the open and up refusals among the others
@task_item n=1 part=touches
Rules on the order of a start (instance check before any read, strict-modes check before any log opens, child spawned last, start pins the bytes it checked) are now held by one shared start prologue that run, revive and adopt all call. The order did not change.
@task_item n=2 part=touches
Revive resumes in place through the start path, and adopt's start: both now call the shared start prologue in the run verb instead of each running its own trust check, exe check and log open.
@task_item n=3 part=touches
revive and adopt --json refusal codes for strict-modes-failed: the strict-modes refusal under --json is written by each caller's own writer, which it hands to the shared prologue. Without --json a home-tree refusal is written. Codes and exits did not change.
@task_item n=4 part=touches
The instance trust check (the hook and the start refuse an untrusted instance) now has one definition, shared by the start path and revive's list arm. Before this, revive and adopt each held a byte-equal copy.
@task_item n=5 part=touches
The viola open and viola up exit codes and their refused window-step bodies: one refusal writer, open's, now produces both verbs' bodies. The contract error-body rows on up now point at open's witness. Codes, exits and texts did not change.
@task_item n=6 part=touches
The up verb's window step goes through open (up reopens parents first, and only instances gone before the boot). Its four refusal sites now call open's shared writer.
@task_item n=7 part=touches
Child exit detected on the process handle, and the child exit record: the pump's drain of child output in flight at the exit takes its bound from the pump's seams. The product bound stays 500 ms. This includes the drain after a respawn.
@task_item n=8 part=touches
The R8 parent-identity floor and process liveness through sysinfo: the parent-chain walk is split out of ancestors. The walk stops at pid 1 and holds at most the max pids. The 8-pid limit stays at the callers (focus and open), and the one process-table read is unchanged.
@task_item n=9 part=touches
Mutation testing at the epoch boundary and mutation runs in their own target dir and NOCOW scratch: the root crate's mutation reading goes from baseline-test-failure to a score of 100.0. The baseline passes under the mutants profile once the verify class takes the six tests by name.
@task_item n=10 part=touches
Any row whose cell holds the verify class's kill bound or its membership: the 20 s × 4 bound is unchanged in both profiles, and the mutants default of 5 s × 3 is unchanged. Membership goes from the verify_window_ prefix plus the two drift replay tests to those plus six tests by name. A guard naming a verify_window_ test sits on this line.
@task_item n=11 part=touches
Window features target Hyprland and real CLI or live probes run only locally: the R2 live placement script now refuses with a red precondition verdict on a locked desktop or a scratch home that is not fresh. It refuses before anything starts, and skips its exit cleanup on that refusal.
@task_item n=12 part=touches
Test-side removals and live-rig hygiene: on a precondition refusal the R2 script reads no compositor client list and closes no child window.
@task_item n=13 part=touches
Board read model, list reads state directly, and snapshot replay recovering log-derived fields: tests that kill mutants were added for the checked-instance scan in name order and for ended-after-last-start with another kind after the start. Behaviour did not change.
@task_fact n=1 slug=established-decisions
fact: Run, revive and adopt start through one shared start prologue: the instance trust check, then the checked executable (a strict-modes refusal written by the caller's writer under --json, else a home-tree refusal), then the wrapper log open, in that order. No start path carries its own copy of these checks.
@task_fact n=2 slug=conventions
fact: up's refused window-step bodies come from open's one refusal writer, so up and open refuse a window step with the same body. No verb keeps its own copy of that writer.
@task_item n=1 part=untouched
The wrapper channel and its JSON-RPC methods, frames, fault codes, params and protocol version: no method, frame or code changed.
@task_item n=2 part=untouched
MCP tools, MCP results and the stdio MCP server: nothing of the MCP surface was reached.
@task_item n=3 part=untouched
Hooks, the hook tiers, the hook-to-kind map and hook exit codes and bodies: no hook path changed.
@task_item n=4 part=untouched
Normalised event kinds, event data, the event line shape and the events log: no kind or field was added or changed.
@task_item n=5 part=untouched
Instance snapshot, config.json, ledger stamps and other on-disk formats: no schema, key or writer changed.
@task_item n=6 part=untouched
Names: no binary, crate, command, env var, socket or other name was taken or freed.
@task_item n=7 part=untouched
Dependencies, cargo-deny policy, toolchain pin, MSRV and banned features: no dependency or version moved.
@task_item n=8 part=untouched
Send, wheel, dialog hold, paste and readiness gate rules: no send or wheel path was reached.
@task_item n=9 part=untouched
Verify and its probes, the capability ledger rows and the CLI version gate's behaviour: version-gate tests were only added to the verify kill class, and no gate logic changed.
