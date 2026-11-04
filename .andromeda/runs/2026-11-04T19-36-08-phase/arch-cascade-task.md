@task_prose part=chunk
Brings Linux and macOS to parity with Windows: the `viola run` wrapper there runs its child under portable-pty's openpty backend through the same `pty` seam as ConPTY (spawn, read, write, resize, wait, kill behaving as the Windows wrapper's do under the fake agent).
Makes the full fake-agent suite one suite on all three OSes: every end-to-end and integration test reading a behaviour both Unix OSes share runs on ubuntu-latest and macos-latest, and any test left cfg(windows), cfg(unix) or cfg(target_os) carries its one-OS reason where it stands.
Keeps one diagnostics schema across OSes: process-log lines, events.ndjson, the snapshots and every --json document carry the same fields and closed words, a field one OS cannot fill reading its documented null or closed word.
Adds the integration-tier witness of v1-40 (dim only on stale rows) under the outer PTY on the Unix suite, and re-reads the obs gates G1 to G4 and the CI test, perf and lint jobs green on ubuntu-latest and macos-latest beside windows-2025.
Windows stays the live-supported target: no Windows behaviour, pin or test moves; no live claude session runs; macOS keeps its recorded no-peer-pid gap.
@task_file n=1 path=.github/workflows/ci.yml mark=re-reads
what: the test, perf and lint jobs over the three OSes (their matrix.os), read green on ubuntu-latest and macos-latest beside windows-2025
@task_file n=2 path=tests/run_cli.rs
@task_file n=3 path=tests/support/home.rs
@task_file n=4 path=schemas/diag-line.v1.json
@task_item n=1 part=needs
The PTY decision and its pty seam (spawn, read, write, resize, wait, kill) over portable-pty pinned =0.8.1, here its openpty backend on Linux and macOS: child exit is detected on the process handle, never on EOF; viola never holds, queues or re-orders a human key behind a resize and never answers DA1 itself.
@task_item n=2 part=needs
The fake agent as the CI stand-in: the test-only viola-fake-agent bin behind feature fake-agent, its control and receipt files under a test home's fake/ dir, the committed fake-script scenarios, and the test seams FAKE_AGENT_PUMP_DELAY_MS and FAKE_AGENT_HOOK_PANIC read only under that feature.
@task_item n=3 part=needs
The CI/CD decision the Unix suite runs under: the GitHub Actions three-OS matrix against the fake agent, the fake agent named after `--`, the child resolved to an absolute path and spawned directly, each test in its own viola home through --home so parallel tests share no endpoint, log or budget.json.
@task_item n=4 part=needs
The obs gates G1 bare instrument, G2 zero panics, G3 no abort panic strategy and G4 schema conformance, read over the test homes under target/e2e-home kept in CI, with the viola-chaos home the named carve-out outside G2, G4 and the secret scan.
@task_item n=5 part=needs
The diagnostics line schemas diag-line.v1.json and diag-detail.v1.json (obs-owned, default-deny, pinned by tests/contract_diag_schema.rs) and the role files under diagnostics/: codes only, one line per write, the same on every OS.
@task_item n=6 part=needs
The own-format contracts the one-schema rule covers: the ndjson event line and its per-kind data, the snapshot envelope and the instance snapshot fields, and the CLI --json documents with their closed error and detail words (UnreachableCause, revive refusal codes).
@task_item n=7 part=needs
Indirect, through the data model conventions: optional fields are omitted on write and readers treat missing and null the same, so a field one OS cannot fill must still read its documented null or closed word under that convention, never an OS-only word.
@task_item n=8 part=needs
The Linux and macOS IPC endpoint: the Unix socket viola-<h12>.sock in a per-user 0700 socket dir (never the /tmp or $TMPDIR root, checked before bind and every connect, socket-dir-refused left as found), chmod 0600 after bind, both sides admitting only the owning user's euid.
@task_item n=9 part=needs
The macOS carve-out: macOS reports no peer pid, so the server check a frame-writing client makes against the snapshot's pid and started_at compares nothing there; that check runs on Linux and Windows only, and no test reads macOS as a comparison.
@task_item n=10 part=needs
Indirect, through the peer-euid check: no env, flag or feature seam may force a refusing peer-euid decision, and any Unix-only test seam the suite needs takes a Decisions Log entry and must fit the naming rule's ratified test-seam exceptions.
@task_item n=11 part=needs
Session liveness as the v1-40 witness reads it: the heartbeat touched every 1 s, a beat older than 5 s stale, pid plus process start time deciding gone; a viola list beside a stale wrapper reads SGR dim across the whole row, NAME dim and bold.
@task_item n=12 part=needs
The human output colour decision viola list makes under the outer PTY: NO_COLOR, TERM and COLORTERM read never set, deciding only the SGR bytes, and the board width taken from viola_pty::host_size().
@task_item n=13 part=needs
The kill-and-revive test support: Wrapper::kill in tests/support/home.rs and the root Watch bound WITHIN of 7 s with its viola-root-watch evidence report, by which a killed wrapper's child is read gone by pid and start time on Unix; the standalone pin run_child_ends_when_its_wrapper_is_terminated stays cfg(windows).
@task_item n=14 part=needs
OS-differing behaviours a parity test must read per OS: the strict-modes rules (Unix no group or other bit, Windows the DACL; events.ndjson the write rule on Unix, the read rule on Windows), the per-session settings.json override written only on Unix, hook statusline's /bin/sh -c shell-out (none on Windows) and the claude_env_keep Unix R8 pass-list.
@task_item n=15 part=needs
The cross-platform discipline: every OS-specific branch compiles and is tested on its CI runner, nothing shells out but hook statusline, child processes are spawned directly, endpoint names hashed and short.
@task_item n=16 part=needs
Indirect, through Tokio containment and the sync crates: any new code on the run or pty path uses std threads and blocking I/O, viola-pty staying a sync crate under the sole-root tokio ban.
@task_item n=17 part=needs
Indirect, through the bounded readers rule: any product reader of external bytes the openpty path adds is bounded in its own statement or named as an exception, censused by tests/contract_bounded_readers.rs.
@task_item n=18 part=needs
The typed exit-code set stays 0, 1, 2, 10 to 14, 20, 21, and viola hook keeps its contract: always exit 0, no body when no decision, nothing on stderr.
@task_item n=19 part=needs
The wheel's closed non-editing list as the CARRY names it: measured only on foot 1.28.0; a terminal answering CSI ? 6 n or the kitty graphics APC query writes a reply that still counts as typing, and a shape joins the list only when measured by its exact grammar on the founder's word.
@task_item n=20 part=needs
The test-only channel endpoints viola-test-chan-<pid>-<label> (a socket of that base name off Windows) and the viola-pty-watch report files of the real-PTY tests, whose dsr-cpr count reads the CSI 6 n queries the rig saw.
@task_item n=21 part=needs
The capabilities this entry answers to: v1-53 (Linux and macOS reach parity with Windows: the parity suite, the macOS gap record, the live Linux leg read against earlier live runs) and v1-40's integration-tier witness.
@task_item n=22 part=needs
The cross-platform design philosophy: Windows first, all three OS paths built and CI-tested on every commit against the fake agent, no design choice depending on tmux, a shell or POSIX-only behaviour; the first live run was on the Linux dev host.
@task_standing n=1
convention: every new test runs in its own viola home through --home, under target/e2e-home (viola-test-*), kept in CI for the obs gates, a new home removing only dirs whose recorded owner process is gone
home: occupied-resources
@task_standing n=2
convention: every new end-to-end or integration test drives the fake agent named after `--`, never the real claude in CI
home: Established Decisions
@task_standing n=3
convention: every OS-specific branch or cfg-gated test compiles and is tested on its own CI runner
home: Cross-cutting Patterns
@task_standing n=4
convention: test-harness-only variables carry the AGENT_RUN_ prefix and are never read by viola; a test seam is read only under cfg(feature = "fake-agent") and absent from release builds
home: Conventions
@task_standing n=5
convention: every new log line is codes only, one complete JSON object per write, in its role file under diagnostics/ and conforming to diag-line.v1.json; content-bearing detail only in the instance detail files
home: Cross-cutting Patterns
@task_standing n=6
convention: every new test-only endpoint or evidence file sits outside the viola-<h12> namespace, unique per test (pid and label), its report kept when the test panics or is killed and removed when it passes
home: occupied-resources
@task_standing n=7
convention: every new path a test or job writes under target/ is gitignored and recorded in Occupied Resources' Repository list
home: occupied-resources
@task_standing n=8
convention: every new fake-agent scenario is a committed synthetic-only script validated against schemas/fake-script.v1.json
home: occupied-resources
@task_item n=1 part=not_needed
The MCP server: rmcp, its stdio tools, its MAX_FRAME stdin cap and its isError result shapes; the chunk adds no tool and changes no MCP surface.
@task_item n=2 part=not_needed
The budget governor: thresholds, budget.json, the budget-gate event and release --budget; nothing in the scope reaches the gate.
@task_item n=3 part=not_needed
Links: the link and unlink verbs, the implicit link on send or answer and the link set replay.
@task_item n=4 part=not_needed
The capability ledger, ledger stamps, viola verify and its probe dirs, and fixture recording under fixtures/claude: they need a live claude, and this chunk runs none.
@task_item n=5 part=not_needed
The ConPTY sideload: the vendored conpty.dll and OpenConsole.exe, scripts/conpty-vendor.sh, the companions' pinning and the target/conpty-seed copies; Windows-only and unmoved.
@task_item n=6 part=not_needed
The Windows named-pipe client's SQOS Identification open and the channel_sqos_open test pipe.
@task_item n=7 part=not_needed
The open Windows-only live measurements (a key lost after a resize, the sideloaded ConPTY's DA1 start stall), owned by the entry Windows-only live measurements and kept there.
@task_item n=8 part=not_needed
The frontend and the GUI control scope, deferred to 0.2.0.
@task_item n=9 part=not_needed
The dialog answer mapping and the answer, hook.dialog and permission continuation contracts; the chunk changes no dialog behaviour.
@task_item n=10 part=not_needed
The separate fuzz workspace, its nightly toolchain, targets and corpus, and the nightly fuzz job.
@task_item n=11 part=not_needed
The mutation arm: target/mutants and the Windows host mutation scratch outside the repository.
@task_item n=12 part=not_needed
Hosting, mobile, AI/ML and public distribution (dist releases, signing, winget, Homebrew, Scoop, self_update): none apply to v1 parity.
@task_item n=13 part=not_needed
The pinned-copy and plugin layout under bin/ and plugin/, the plugin install marketplace and its one-tree two-copies question, owned by Version done-check.
@task_item n=14 part=not_needed
The profraw census and the coverage LCOV output, dev-host witnesses no CI job runs.
