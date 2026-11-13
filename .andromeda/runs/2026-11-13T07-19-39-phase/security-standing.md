# security · standing conventions

- S1 · every new test makes and uses its own owner-only home and never the user's real viola or Claude home · secret-management
- S2 · every new test or helper removes only directories it created itself · security-anti-patterns-never-do-these · security-anti-patterns
- S3 · every new test, fixture or helper holds no credential literal, secret or local home, and nothing of a test home is committed to git · security-anti-patterns-never-do-these
  decision/fixture-hygiene · Recorded fixture hygiene — Recorded claude fixtures are never committed from sessions with real prompts, nor before being checked for home paths and usernames. · except: A workspace.repo naming this repository's own remote is kept.
  decision/fixtures-reviewed-before-commit · Fixtures reviewed before commit — Recorded fixture payloads are reviewed for home paths, usernames and tool input before committing.
  decision/no-credential-literals · No credential literals in code — No token, key or credential literal appears in source or in the embedded plugin files, which carry only exec paths.
  decision/no-hardcoded-credentials · No hardcoded credentials — Credentials are never hardcoded in source or in the include_str! plugin files.
- S4 · every new test seam or test-only code stays under the test-only feature and is compiled out of release · security-anti-patterns · input-validation
  decision/test-support-dev-only · test-support enabled only for development — viola-channel's test-only test-support feature is enabled only by the root package's dev-dependencies.
