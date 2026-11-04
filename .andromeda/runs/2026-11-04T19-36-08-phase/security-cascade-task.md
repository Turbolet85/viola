@task_prose part=chunk
On Linux and macOS the `viola run` wrapper runs its child under portable-pty's openpty backend through the same pty seam as Windows' ConPTY: spawn, read, write, resize, wait and kill behave as on Windows, as far as the fake agent can read them.
The full fake-agent suite becomes one suite on all three OSes: every end-to-end and integration test of a behaviour both Unix OSes share runs on ubuntu-latest and macos-latest, and each test left cfg(windows), cfg(unix) or cfg(target_os) names why its behaviour belongs to one OS.
It re-measures the obs gates G1 to G4 and the CI test, perf and lint jobs, which must read green on ubuntu-latest and macos-latest beside windows-2025, and it re-reads, on Unix with the fake agent, that a killed wrapper's child ends with it.
It brings the diagnostics schema into line on every OS: process-log lines, events.ndjson, the snapshots and every --json document carry the same fields and closed words, and a value one OS cannot fill reads its documented null or closed word.
It adds the integration-tier witness of the dim-only-on-stale-rows rule (a viola list row beside a stale wrapper under the outer PTY) to the Unix suite. No Windows behaviour, pin or test moves, there is no live claude session, and no new exit code is added.
@task_file n=1 path=tests/run_cli.rs mark=edits
what: its cfg(windows) cases and the reason each one carries (the pin run_child_ends_when_its_wrapper_is_terminated stays cfg(windows))
@task_file n=2 path=tests/support/home.rs mark=re-reads
what: Wrapper::kill, which waits up to 7 s for the recorded child to be gone by pid and start time in the kill-and-revive case
@task_file n=3 path=.github/workflows/ci.yml
@task_item n=1 part=needs
The Unix PTY path: portable-pty (pinned at 0.8.1) with its openpty backend behind the wrapper's pty seam. viola run resolves its child itself and hands portable-pty an absolute path. Channel handles and interprocess listeners stay non-inheritable, so no channel handle leaks into the child on Linux or macOS.
@task_item n=2 part=needs
The vt100 screen feed under catch_unwind on Unix. The human's bytes are written first, a parser panic poisons only the screen model, and passthrough to the human terminal goes on with a bounded feed queue.
@task_item n=3 part=needs
Unix IPC access control, now exercised on both Unix OSes. The socket sits in a verified per-user 0700 directory (never /tmp or the $TMPDIR root, never the Linux abstract namespace) and is chmod 0600 after bind, never through ListenerOptionsExt::mode, which fails the bind on macOS. try_overwrite stays off.
@task_item n=4 part=needs
The Unix server-side peer check: a peer whose euid differs from the server's is dropped before any frame is read. No env var, flag or feature seam may force a refusing peer-euid decision, and a Unix-only test seam needs its own Decisions Log entry.
@task_item n=5 part=needs
Client-side server verification on Unix. Linux compares the peer pid (SO_PEERCRED) and its start time. Both Unix OSes check the socket directory before connecting and admit only a peer with their own euid, refusing with socket-dir-refused or server-verify-failed and sending no frame.
@task_item n=6 part=needs
The macOS carve-out: there is no peer pid, so the macOS server check compares no pid or start time and server-verify-failed means an euid mismatch alone. No parity test may read it as a comparison, and no other frame or process borrows the carve-out.
@task_item n=7 part=needs
Home strict-modes on Unix: 0700 directories and 0600 files (the pinned viola at 0700), each written through replace_private with its mode set before any byte and never left to the umask. Any group or other bit on the home, an instance directory or diagnostics/ is refused, as is a symlinked diagnostics/, before the first read in every process.
@task_item n=8 part=needs
The Unix-only statusline shell-out: hook statusline runs the recorded command as /bin/sh -c with the command as one argument, its stdout capped and its wait bounded. The settings override is written on Unix only, and only for a pinned path of plain characters. It stays the only shell-out.
@task_item n=9 part=needs
One diagnostics schema across OSes: diag-line.v1.json process-log lines, events.ndjson, the snapshots and --json documents use the same closed words on every OS. External errors (CLI --json, MCP isError, channel error.data) stay sanitised, with closed codes (including the instance-unreachable details), no absolute paths, no upstream text and no internal type names.
@task_item n=10 part=needs
The NEVER-log floor applied to every OS's output. No R8-stripped CLAUDE* value goes into any log, diagnostic, snapshot or events line. User content and drift reports go only to the 0600 diagnostics/. The snapshot's recorded cwd stands only in the 0600 snapshot.
@task_item n=11 part=needs
The exit code and hook contract: the typed exit set stays 0, 1, 2, 10-14, 20, 21. viola hook exits 0 with no body and no stderr on every OS, a failing security check included. A security refusal never blocks the human.
@task_item n=12 part=needs
The stdin classifier's closed non-editing list (the wheel), which is carried in: a terminal reply such as the cursor-position report or a kitty graphics APC string joins the list only when measured by its exact grammar on the founder's word, never added on a guess.
@task_item n=13 part=needs
The v1-40 witness through viola list's human mode under the outer PTY. Rows are escaped with escape_row so each stays one line. NO_COLOR, TERM and COLORTERM are only compared and never echoed, and they decide only whether, and how deep, SGR bytes are written.
@task_item n=14 part=needs
The test-only seams FAKE_AGENT_PUMP_DELAY_MS (capped at 5 000 ms) and FAKE_AGENT_HOOK_PANIC (panics only on exactly 1), reached through the fake-agent feature that the Unix suite now runs. They configure nothing, are compiled out of release builds, and the release check refuses any test-only feature in the viola artifact.
@task_item n=15 part=needs
CI's G2 zero-panics gate on ubuntu-latest and macos-latest: exactly one exemption, the FAKE_AGENT_HOOK_PANIC seam's panic_location compared as a whole string, with its probe run first.
@task_item n=16 part=needs
The artifact canary secret-scan on the Unix legs. Its Unix-only class fails on a diagnostics file that is not 0600. The perf job's per-OS uploads (perf-<os>, diag-perf-<os>) and the test-home uploads go up only after the scan passes.
@task_item n=17 part=needs
The Windows-only controls a shared change to the pty seam or wrapper must leave as they are: the protected user plus SYSTEM listener DACL, the SQOS identification-level client open, the System32-only DLL search and the ConPTY companion sideload, the .cmd/.bat child refusal, and the Windows owner plus DACL strict-modes check.
@task_item n=18 part=needs
The test-side home keepers: a removal under target/e2e-home takes only a directory the test side itself created, a linked base is followed only to an absolute owner-only real directory, and the target is created 0700. Fake-agent tests now hit these on Linux and macOS.
@task_standing n=1
convention: each new test runs under its own --home and takes no host environment value into a child; the local pre-push gate launches children under env -i with exactly HOME and PATH, and any assignment of a host value is a boundary widening that needs a Decisions Log entry
home: Secret Management (Development)
@task_standing n=2
convention: no new test seam, environment read or flag may switch off any control in the plan; a new seam or env read, a Unix-only test seam included, needs its own Decisions Log entry
home: security-anti-patterns
@task_standing n=3
convention: each new CI job or step pins its actions by full commit SHA with a version comment, uses checkout with persist-credentials false, holds workflow permissions {} and job contents read, passes matrix and event values only through env:, adds no concurrency block, and passes zizmor
home: dependency-security
@task_standing n=4
convention: each new CI upload waits for the artifact canary secret-scan to pass, and every input it carries is synthetic
home: Secret Management (Secret scanning in CI)
@task_standing n=5
convention: each new log or diagnostics line keeps the NEVER-log floor: no stripped CLAUDE* value, no user content or drift report outside 0600 diagnostics/, and no absolute path, upstream text or internal type name in any external error
home: Bootstrap phases (logging-redaction-wire)
@task_standing n=6
convention: each new fixture is synthetic and is checked for home paths, usernames and tool input before it is committed
home: data-protection
@task_standing n=7
convention: no new G2 zero-panics exemption beyond the FAKE_AGENT_HOOK_PANIC seam's without its own Decisions Log entry
home: Dependency Security (CI integration)
@task_item n=1 part=not_needed
The v1.x release prerequisites (dist attestations and signing, cargo-auditable bin scans, self_update signatures, no rust-cache in a release workflow): v1 has no release.
@task_item n=2 part=not_needed
Repository fixtures recorded by viola verify --record and verify's trusted runs B to E: they need a live claude, and this chunk has no live session.
@task_item n=3 part=not_needed
The accepted residuals for a scratch footer reading of a newer CLI and a live round of viola plugin install, and the real-CLI probe prerequisite on the dev host: all live-session work this chunk does not do.
@task_item n=4 part=not_needed
Windows persistent-environment (registry) names read by viola run: a Windows-only read that this chunk leaves unchanged.
@task_item n=5 part=not_needed
windows-mutants.yml and the run --mutants Windows host mutation scratch: mutation testing belongs to the epoch-boundary audit, not this chunk.
@task_item n=6 part=not_needed
The nightly fuzz job, the fuzz/ workspace exemption and its separate lockfile audit: parser fuzzing, which the chunk does not touch.
@task_item n=7 part=not_needed
The frontend and the API Security section's frontend note: deferred to 0.2.0.
@task_item n=8 part=not_needed
Compliance triggers, account-flow bans, password hashing, SQL, file upload and webhook rows: all N/A for viola.
@task_item n=9 part=not_needed
events.ndjson retention and the local-availability accepted risk: an open item owned by arch, which this chunk does not take up.
@task_item n=10 part=not_needed
Signing credentials as future CI secrets and the rotation cadence rows: v1 holds no secrets.
@task_item n=11 part=not_needed
The bootstrap phase ordering for route and setup-project: a derivation for other skills, not a rule this chunk's code acts on.
