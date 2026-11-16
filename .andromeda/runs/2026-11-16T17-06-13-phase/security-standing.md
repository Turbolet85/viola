# security · standing conventions

- S1 · each new recorded payload or fixture is scrubbed or refused whole and reviewed before commit · data-protection
  contract/fixture-refusal-codes/absolute-path · verify's unclean-fixture refusal codes · absolute-path — A closed check code of the fixture refusal, for a surviving absolute path.
  contract/fixture-refusal-codes/email · verify's unclean-fixture refusal codes · email — A closed check code of the fixture refusal, for a surviving email-shaped token.
  contract/fixture-refusal-codes/home-path · verify's unclean-fixture refusal codes · home-path — A closed check code of the fixture refusal, for a surviving home path.
  contract/fixture-refusal-codes/username · verify's unclean-fixture refusal codes · username — A closed check code of the fixture refusal, for a surviving user word.
  contract/fixture-refusal-line/form · verify's unclean-fixture refusal · line form — The refusal reads `unable: a recorded fixture is not clean: <file> <code>`, or `<file> row <n>[ seam] <code>` for a screen, naming the file and a closed code, never the content.
  decision/mcp-fixtures-via-test-only-tap · MCP fixtures recorded through a test-only tap — MCP exchange fixtures are recorded by a test-only tap that spawns viola mcp directly with no shell, caps each line, keeps its raw capture 0600 in a private directory never committed, and applies the same scrub and closed refusal codes. · guard: schemas/claude-mcp-fixture.v1.json
  decision/viola-format-fixtures-hygiene · viola-format fixtures under the hygiene walk — Committed viola-format fixtures, the old state shapes and retired words, are held to the same fixture hygiene walk as recorded fixtures: no path, user or email. · guard: viola_format_fixtures_pass_hygiene
- S2 · each new test runs under its own viola home, never the user's · secret-management
  contract/cli-state-root/home-flag · CLI state root selection · --home — Chooses which state tree, snapshot, endpoint and ledger stamps are used.
  contract/cli-state-root/viola-dir · CLI state root selection · VIOLA_DIR — Chooses which state tree, snapshot, endpoint and ledger stamps are used.
  decision/test-homes-gitignored · Test homes kept out of git — Local --home test directories are git-ignored so recorded state never lands in git.
  decision/tests-use-own-home · Tests use their own home — Development handles secrets as production does, and tests use their own --home directory.
- S3 · each new test seam lives only under the fake-agent feature and is compiled out of release · security-anti-patterns · security-anti-patterns-never-do-these
  decision/fake-agent-hook-panic-seam · Hook panic test seam — FAKE_AGENT_HOOK_PANIC exists only in test-feature builds and forces a fixed-payload hook panic only when exactly 1; the hook still fails open; it configures nothing, disables no control and widens no redaction. · why: Ratified.
  decision/fake-agent-pump-delay-seam · Pump delay test seam — FAKE_AGENT_PUMP_DELAY_MS exists only in test-feature builds, a u64 of milliseconds capped at 5 000 that holds the run pump back after the child starts; it configures nothing, disables no control and widens no redaction. · why: Lets a test land a host resize in the spawn-to-pump window; ratified.
- S4 · each new log or diagnostics line honours the never-log floor and carries no upstream text · bootstrap-phases-derive-for-route-setup-project · bootstrap-phases
