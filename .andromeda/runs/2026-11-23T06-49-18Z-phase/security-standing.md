# security · standing conventions

- S1 · each new test runs in its own viola home and never touches the user's real home or Claude home · secret-management
  decision/test-homes-gitignored · Test homes kept out of git — Local --home test directories are git-ignored so recorded state never lands in git.
  decision/tests-use-own-home · Tests use their own home — Development handles secrets as production does, and tests use their own --home directory.
- S2 · each new test seam sits only under the fake-agent feature and is compiled out of release · security-anti-patterns
  decision/fake-agent-hook-panic-seam · Hook panic test seam — FAKE_AGENT_HOOK_PANIC exists only in test-feature builds and forces a fixed-payload hook panic only when exactly 1; the hook still fails open; it configures nothing, disables no control and widens no redaction. · why: Ratified.
  decision/fake-agent-pump-delay-seam · Pump delay test seam — FAKE_AGENT_PUMP_DELAY_MS exists only in test-feature builds, a u64 of milliseconds capped at 5 000 that holds the run pump back after the child starts; it configures nothing, disables no control and widens no redaction. · why: Lets a test land a host resize in the spawn-to-pump window; ratified.
- S3 · each new recorded fixture is scrubbed of paths, names and emails and reviewed before commit · security-anti-patterns-never-do-these
  contract/fixture-refusal-codes/absolute-path · verify's unclean-fixture refusal codes · absolute-path — A closed check code of the fixture refusal, for a surviving home, drive-letter or user-folder path.
  contract/fixture-refusal-codes/email · verify's unclean-fixture refusal codes · email — A closed check code of the fixture refusal, for a surviving email-shaped token.
  contract/fixture-refusal-codes/home-path · verify's unclean-fixture refusal codes · home-path — A closed check code of the fixture refusal, for a surviving home path.
  contract/fixture-refusal-codes/username · verify's unclean-fixture refusal codes · username — A closed check code of the fixture refusal, for a surviving user word.
  decision/viola-format-fixtures-hygiene · viola-format fixtures under the hygiene walk — Committed viola-format fixtures, the old state shapes and retired words, are held to the same fixture hygiene walk as recorded fixtures: no path, user or email. · guard: viola_format_fixtures_pass_hygiene
- S4 · each new log line holds no user content and no CLAUDE values; full detail goes to diagnostics only · bootstrap-phases-derive-for-route-setup-project
  decision/agents-probe-log-content-free · Agents probe log holds no content — An agents-probe spawn pair is logged only under an open log and holds no row text, id, cwd or path.
  decision/agents-probe-log-redaction · Agents-probe logging redacted — Each agents-probe spawn pair is logged with no row text, id, cwd or path.
  decision/drift-reports-diagnostics-only · Drift reports only in diagnostics — Deserialisation drift reports are written only to the diagnostics directory. · why: drift reports can quote tool input
