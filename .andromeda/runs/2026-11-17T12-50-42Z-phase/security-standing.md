# security · standing conventions

- S1 · each new test uses its own viola home, never the user's, and test homes stay out of git · secret-management · bootstrap-phases
  decision/no-secrets-in-git · No secrets or local homes in git — .env files, local --home test directories and signing material are never committed to git.
  decision/test-homes-gitignored · Test homes kept out of git — Local --home test directories are git-ignored so recorded state never lands in git.
  decision/tests-use-own-home · Tests use their own home — Development handles secrets as production does, and tests use their own --home directory.
- S2 · each new test removes only directories it created itself, following only a safe linked base for the e2e home · security-anti-patterns-never-do-these · security-anti-patterns
- S3 · each new test seam or fake-agent hook it relies on is test-only, under the fake-agent feature, and compiled out of release · input-validation · security-anti-patterns
  decision/mcp-fixtures-via-test-only-tap · MCP fixtures recorded through a test-only tap — MCP exchange fixtures are recorded by a test-only tap that spawns viola mcp directly with no shell, caps each line, keeps its raw capture 0600 in a private directory never committed, and applies the same scrub and closed refusal codes. · guard: schemas/claude-mcp-fixture.v1.json
  decision/release-build-without-test-features · Release build free of test-only features — The release build is judged from its own compiler-artifact records and refused if it holds any executable other than viola or any artifact built with a test-only feature (test-support, fake-agent). · guard: scripts/release-check.sh, its --probe run first reading 5/5 refused with a clean control
- S4 · each new test socket sits in an owner-only per-test directory, never at the temp root · security-anti-patterns-never-do-these · security-anti-patterns
