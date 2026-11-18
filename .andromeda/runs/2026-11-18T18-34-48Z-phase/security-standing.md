# security · standing conventions

- S1 · each recorded fixture file keeps only signature rows, passes the hygiene walk and is reviewed before commit · data-protection
  decision/fixtures-reviewed-before-commit · Fixtures reviewed before commit — Recorded fixture payloads are reviewed for home paths, usernames and tool input before committing.
  decision/viola-format-fixtures-hygiene · viola-format fixtures under the hygiene walk — Committed viola-format fixtures, the old state shapes and retired words, are held to the same fixture hygiene walk as recorded fixtures: no path, user or email. · guard: viola_format_fixtures_pass_hygiene
- S2 · each new test or live run uses its own home, kept out of git, and removes only the directories it created · secret-management
  decision/test-homes-gitignored · Test homes kept out of git — Local --home test directories are git-ignored so recorded state never lands in git.
  decision/tests-use-own-home · Tests use their own home — Development handles secrets as production does, and tests use their own --home directory.
- S3 · no secret, credential literal or local home lands in git with a new file · security-anti-patterns-never-do-these · security-anti-patterns
  contract/artifact-canary-scan-classes/claude-stripped · artifact canary scan classes · claude-stripped — The scan fails on any of the twelve stripped CLAUDE* canary needles.
  decision/no-credential-literals · No credential literals in code — No token, key or credential literal appears in source or in the embedded plugin files, which carry only exec paths.
  decision/no-hardcoded-credentials · No hardcoded credentials — Credentials are never hardcoded in source or in the include_str! plugin files.
  decision/no-secrets-in-git · No secrets or local homes in git — .env files, local --home test directories and signing material are never committed to git.
