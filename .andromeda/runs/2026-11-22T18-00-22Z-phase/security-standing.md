# security · standing conventions

- S1 · each new test runs under its own home and a test-owned Claude config, and removes only directories it created · secret-management · security-anti-patterns
  decision/test-homes-gitignored · Test homes kept out of git — Local --home test directories are git-ignored so recorded state never lands in git.
  decision/tests-use-own-home · Tests use their own home — Development handles secrets as production does, and tests use their own --home directory.
- S2 · each new test seam sits only under the fake-agent feature and is compiled out of the release build · security-anti-patterns · input-validation
  decision/fake-agent-hook-panic-seam · Hook panic test seam — FAKE_AGENT_HOOK_PANIC exists only in test-feature builds and forces a fixed-payload hook panic only when exactly 1; the hook still fails open; it configures nothing, disables no control and widens no redaction. · why: Ratified.
  decision/fake-agent-pump-delay-seam · Pump delay test seam — FAKE_AGENT_PUMP_DELAY_MS exists only in test-feature builds, a u64 of milliseconds capped at 5 000 that holds the run pump back after the child starts; it configures nothing, disables no control and widens no redaction. · why: Lets a test land a host resize in the spawn-to-pump window; ratified.
- S3 · each new seam or environment read added for a test carries its log entry · security-anti-patterns-never-do-these
- S4 · each new fixture or test output holds no path, username, credential value or transcript byte, scrubbed or refused whole · data-protection
  decision/fixture-hygiene · Recorded fixture hygiene — Recorded claude fixtures are never committed from sessions with real prompts, nor before being checked for home paths and usernames.
  decision/fixtures-reviewed-before-commit · Fixtures reviewed before commit — Recorded fixture payloads are reviewed for home paths, usernames and tool input before committing.
  decision/no-credential-literals · No credential literals in code — No token, key or credential literal appears in source or in the embedded plugin files, which carry only exec paths.
  decision/viola-format-fixtures-hygiene · viola-format fixtures under the hygiene walk — Committed viola-format fixtures, the old state shapes and retired words, are held to the same fixture hygiene walk as recorded fixtures: no path, user or email. · guard: viola_format_fixtures_pass_hygiene
- S5 · each new test runs in CI with no secrets and no real claude · threat-model-summary
