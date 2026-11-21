# security · standing conventions

- S1 · each new recorded fixture is scrubbed or refused whole, keeps only signature rows and is reviewed before commit · data-protection
- S2 · each new test uses its own home, never the operator's, and holds no secret · secret-management
  decision/no-hardcoded-credentials · No hardcoded credentials — Credentials are never hardcoded in source or in the include_str! plugin files.
  decision/test-homes-gitignored · Test homes kept out of git — Local --home test directories are git-ignored so recorded state never lands in git.
  decision/tests-use-own-home · Tests use their own home — Development handles secrets as production does, and tests use their own --home directory.
- S3 · each new test removes only the directories it created, and each new test seam stays under the fake-agent feature · security-anti-patterns
  decision/e2e-home-removal-own-dirs-only · Test-side removal only of its own dirs — A test-side removal under target/e2e-home never takes anything but a dir the test side itself created.
  decision/fake-agent-hook-panic-seam · Hook panic test seam — FAKE_AGENT_HOOK_PANIC exists only in test-feature builds and forces a fixed-payload hook panic only when exactly 1; the hook still fails open; it configures nothing, disables no control and widens no redaction. · why: Ratified.
  decision/fake-agent-pump-delay-seam · Pump delay test seam — FAKE_AGENT_PUMP_DELAY_MS exists only in test-feature builds, a u64 of milliseconds capped at 5 000 that holds the run pump back after the child starts; it configures nothing, disables no control and widens no redaction. · why: Lets a test land a host resize in the spawn-to-pump window; ratified.
  decision/test-removal-own-dirs-only · Test-side removal only of test-created dirs — A test-side removal under the e2e home takes only a directory the test side itself created, by its owner record, its own temp dir or its session name prefix, or another build's stamp dir whose lock it takes with try_lock.
- S4 · each new log line carries no CLAUDE value, path or upstream text, and goes to owner-only diagnostics · bootstrap-phases-derive-for-route-setup-project
  contract/artifact-canary-scan-classes/diagnostics-mode · artifact canary scan classes · diagnostics file mode — The scan fails on a diagnostics file whose mode is not 0600.
  decision/agents-probe-log-content-free · Agents probe log holds no content — An agents-probe spawn pair is logged only under an open log and holds no row text, id, cwd or path.
  decision/agents-probe-log-redaction · Agents-probe logging redacted — Each agents-probe spawn pair is logged with no row text, id, cwd or path.
  decision/cwd-only-in-snapshot · Recorded cwd kept to the snapshot — The snapshot's recorded cwd, which can carry the OS username, stands only in the 0600 snapshot and, as the key of the start's trust record, in the Claude config, never on a log line, error body or stdout.
  decision/diagnostics-open-fails-to-sink · A refused diagnostics open writes nowhere — Diagnostics opens run the strict-modes check on the existing diagnostics folder and file before narrowing anything; a refusal is a failed open, the writer goes to a sink and the detail line is dropped.
  decision/diagnostics-owner-only · Diagnostics files owner-only — Files under the instance diagnostics are mode 0600.
  decision/logs-owner-only · Logs owner-only — The event log, heartbeat and diagnostics files are 0600 inside 0700 directories, and their owner-only append open never follows a symbolic link: a link at the path is a failed open, and the 0600 mode is set through the open handle.
  decision/spawn-cwd-snapshot-only · Spawn directory kept in the snapshot only — The start's spawn directory lives only in the 0600 snapshot and, as the key of its trust record, in the Claude config, on no log line, error body or stdout, and is omitted from the snapshot when absent. · why: an absolute host path can carry the OS username
- S5 · each new environment read or seam gets its log entry · security-anti-patterns-never-do-these
