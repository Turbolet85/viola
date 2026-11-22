# security · standing conventions

- S1 · each new test runs under its own test-created home and removes only the directories it created · secret-management
  decision/e2e-home-removal-own-dirs-only · Test-side removal only of its own dirs — A test-side removal under target/e2e-home never takes anything but a dir the test side itself created.
  decision/probe-artifacts-cleanup · Probe artifacts owner-only and removed — Probe captures and dirs are owner-only and removed when verify passes, fails or is stopped, each run removing only what it created, never by pattern or listing. · except: a kill or a crash leaves them, and verify's own trust record in the Claude config, to be removed by hand
  decision/test-homes-gitignored · Test homes kept out of git — Local --home test directories are git-ignored so recorded state never lands in git.
  decision/test-removal-own-dirs-only · Test-side removal only of test-created dirs — A test-side removal under the e2e home takes only a directory the test side itself created, by its owner record, its own temp dir or its session name prefix, or another build's stamp dir whose lock it takes with try_lock.
  decision/verify-probe-dirs-cleanup · Verify probe directories removed by drop guards — Every probe directory of viola verify is 0700 and removed when verify passes, fails or is stopped by SIGINT or SIGTERM; a guard removes only what its own run created, never by a pattern or a listing, and no remover of an earlier run's directories exists. · except: SIGKILL, a crash or a failed handler install leave the directories, and verify's own trust record in the Claude config, to be removed by hand.
- S2 · each new test seam lives only under the fake-agent feature and is compiled out of the release build · security-anti-patterns · security-anti-patterns-never-do-these
  decision/fake-agent-hook-panic-seam · Hook panic test seam — FAKE_AGENT_HOOK_PANIC exists only in test-feature builds and forces a fixed-payload hook panic only when exactly 1; the hook still fails open; it configures nothing, disables no control and widens no redaction. · why: Ratified.
  decision/fake-agent-pump-delay-seam · Pump delay test seam — FAKE_AGENT_PUMP_DELAY_MS exists only in test-feature builds, a u64 of milliseconds capped at 5 000 that holds the run pump back after the child starts; it configures nothing, disables no control and widens no redaction. · why: Lets a test land a host resize in the spawn-to-pump window; ratified.
- S3 · each new seam or environment read gets its log entry · security-anti-patterns-never-do-these
- S4 · each new test or CI job runs with no real claude CLI and no secrets · threat-model-summary
  decision/no-credential-literals · No credential literals in code — No token, key or credential literal appears in source or in the embedded plugin files, which carry only exec paths.
- S5 · each new log or error line carries no user content or upstream text outside diagnostics and interpolates nothing · bootstrap-phases-derive-for-route-setup-project
  decision/agents-probe-log-content-free · Agents probe log holds no content — An agents-probe spawn pair is logged only under an open log and holds no row text, id, cwd or path.
  decision/agents-probe-log-redaction · Agents-probe logging redacted — Each agents-probe spawn pair is logged with no row text, id, cwd or path.
