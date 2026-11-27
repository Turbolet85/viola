@task_prose part=session
The epoch 5 cleanup mended the code audit's ranked defects; no IPC method, endpoint, MCP tool, export, port, socket, env var, config key, dependency or spec-master text was added or changed.
The starts of revive and adopt now share one start prologue with run: the instance strict-modes check, then the pinned-binary check (a strict-modes refusal written by the caller's writer under --json), then the wrapper log opens, in that order, checks moved unchanged; the instance check helper now has one definition, not two.
viola up's private copy of the refusal writer was retired; up now uses viola open's one refusal writer, with its four call sites, codes and exits unchanged.
The parent-chain walk behind focus's and open's ancestor walk was split into a pure walk over a parent table; its signature, its single process-table read, its pid-1 stop and the 8-pid limit at the callers are unchanged; the PTY pump's exit-drain bound became a private seam field, the product bound staying 500 ms.
The R2 live open script (test-only, operator-fired) now refuses with r2: red precondition=scratch or precondition=locked, exit 1, before it makes or starts anything, when its scratch home or runtime dir already exists or the desktop is locked, and skips its exit cleanup on that refusal so it closes no window.
Mutation testing: six verify-driven tests were added by name to the verify kill class in the ci and mutants profiles (kill bounds unchanged); the root crate's mutation baseline went from baseline-test-failure to 100.0 (326 caught, 0 missed) as an addendum to the audit record; new killing tests were added for the pump, the walk, the board scan and replay.
@task_file n=1 path=.config/nextest.toml
what: the test runner's profiles — the verify class's kill override in the ci and mutants profiles, edited to take six tests by name; kill bounds unchanged
@task_file n=2 path=tests/contract_lints.rs
what: the contract lint suite — its pin of the verify class's filter line, edited
@task_file n=3 path=crates/viola-pty/src/pump.rs
what: the wrapper's PTY pump — copies the child's output to the host terminal, drains it at exit and on respawn; edited (drain bound a seam field) and new tests
@task_file n=4 path=crates/viola-state/src/liveness.rs
what: process liveness and the ancestor walk of focus and open, held in memory over one sysinfo process-table read; edited (the walk split out)
@task_file n=5 path=crates/viola-state/src/board.rs
what: the board scan of instances behind viola list, under the home read rule; tests added only
@task_file n=6 path=crates/viola-state/src/replay.rs
what: the replay of the event log into the snapshot read; tests added only
@task_file n=7 path=scripts/r2-open.sh
what: the R2 live-round open script, test-only and operator-fired, acting on its own scratch home and its own terminal windows; edited with a precondition refusal
@task_file n=8 path=src/cmd/run.rs
what: viola run and the start shared by run, revive and adopt — the instance strict-modes check, the pinned-binary check and the wrapper log open; edited (one start prologue, the instance check moved here)
@task_file n=9 path=src/cmd/revive.rs
what: viola revive and revive --list — the start of revive, its strict-modes refusal; edited to call the shared start prologue
@task_file n=10 path=src/cmd/adopt.rs
what: viola adopt — the start of adopt, its strict-modes refusal; edited to call the shared start prologue
@task_file n=11 path=src/cmd/open.rs
what: viola open — its terminal spawn, ancestor walk and refusal writer, the writer now shared; edited
@task_file n=12 path=src/cmd/up.rs
what: viola up — its checks before its log and before a spawn, its refusals; edited, its own refusal writer copy deleted
@task_file n=13 path=tests/contract_error_bodies.rs
what: the error-body contract test table — refusal codes, exits and witnesses per verb; four up rows re-pointed to open's refusal writer
@task_item n=1 part=touches
Start checks the home tree before its log opens, and the home tree checked before any log opens: the order instance check, pinned-binary check, then the wrapper log is now held in one shared start prologue for run, revive and adopt; order kept, a row naming separate per-verb checks may be stale.
@task_item n=2 part=touches
strict-modes refusal for run and revive, and for viola adopt: the refusal is now written through the caller's strict-modes writer passed into the shared prologue under --json; codes and exits unchanged.
@task_item n=3 part=touches
revive checks strict modes before use, strict-modes check at start, state tree strict-modes check before first use: the instance check helper now has one definition, moved from revive and adopt into the shared start.
@task_item n=4 part=touches
A strict-modes refusal leaves no trace: indirect, through the reshaped start — the refusal still comes before the wrapper log opens.
@task_item n=5 part=touches
Pinned binary re-hashed before use, and revive's pinned-hash-mismatch code: indirect, the pinned-binary check now runs inside the shared prologue, after the instance check and before the log.
@task_item n=6 part=touches
Every product error writer named, and named exceptions to the error-enum convention: viola up's own refusal writer is retired; up and open now share one refusal writer, so a list naming up's copy is stale.
@task_item n=7 part=touches
viola up checks before its log and before a spawn, and the strict-modes refusal of viola up: up's refusals now go through open's writer; codes, exits and its four call sites unchanged.
@task_item n=8 part=touches
Focus's and open's ancestor walks held in memory, and focus raises only a window of the session's ancestry: the walk is now a pure walk over a parent table under the same single process-table read; the pid-1 stop and the 8-pid limit unchanged.
@task_item n=9 part=touches
sysinfo for process identity: indirect, the ancestor walk still reads one sysinfo table, now handed to the split-out walk.
@task_item n=10 part=touches
R7 placing script acts only on its own terminal, and R5 adopt script ends only its own processes: the R2 open script now joins them with a precondition refusal before anything starts and a cleanup skipped on that refusal.
@task_item n=11 part=touches
Mutation testing at the epoch audit: the root crate's reading moved from baseline-test-failure to 100.0 as an addendum; the verify kill class now takes six tests by name; a row holding the class membership or a kill bound reads unchanged bounds (20 s by 4, mutants default 5 s by 3).
@task_item n=12 part=touches
No mutants.out upload: indirect, a mutation run was made this session, its output kept in a scratch outside the repository.
@task_item n=13 part=touches
Pump delay test seam, test seams compiled out of release, test seams only under the fake-agent feature: the pump's private seams gained an exit-drain bound; the product passes the fixed 500 ms bound.
@task_item n=14 part=touches
Child output bounded and tolerant: indirect, the pump's drain at exit and on respawn now has killing tests; its bound unchanged.
@task_item n=15 part=touches
viola list checks before opening, the board holds the home to the read rule: a test was added for the instance scan's name order; behaviour unchanged.
@task_item n=16 part=touches
Replayed snapshot inert and replayed snapshot fallbacks: a test was added for the replay's last-start reading; behaviour unchanged.
@task_fact n=1 slug=authentication-authorization
fact: The R2 open script refuses, with r2: red precondition=scratch or precondition=locked and exit 1, before it makes or starts anything when its scratch home or its runtime dir already exists or the desktop does not read unlocked, and on that refusal skips its exit cleanup so it closes no window.
@task_fact n=2 slug=authentication-authorization
fact: run, revive and adopt start through one shared start prologue — the instance strict-modes check, then the pinned-binary check, then the wrapper log open, in that order — with each caller handing in its own strict-modes refusal writer.
@task_fact n=3 slug=error-handling
fact: viola open and viola up write their refusals through one shared refusal writer; up holds no copy of its own.
@task_item n=1 part=untouched
The wrapper channel methods, channel frames, channel errors and the IPC client refusals: no method, frame, code or check order changed.
@task_item n=2 part=untouched
The MCP tools, MCP stdin cap and MCP error results: no MCP surface changed.
@task_item n=3 part=untouched
The hook contract, its stdin limits and its fail-open rules: viola hook was not reached.
@task_item n=4 part=untouched
Socket directory, socket file modes and peer euid checks: no bind, connect or peer check changed.
@task_item n=5 part=untouched
Dependency security, pins, licence and advisory lists, cargo-deny and zizmor: no dependency or tool version changed.
@task_item n=6 part=untouched
Secret management, credential strips and the child environment: no env read or strip changed.
@task_item n=7 part=untouched
config.json keys and the tolerant config parse: no config key changed.
@task_item n=8 part=untouched
Verify's dialogs, stamps, probes and fixtures, and the version gate's behaviour: verify-driven tests were only named in a test runner class, no verify code changed.
@task_item n=9 part=untouched
The absences — hosting, network, release stage, accounts, secrets: no capability added or removed.
