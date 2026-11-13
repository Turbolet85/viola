# security · standing conventions

- S1 · each new or re-recorded screen fixture is scrubbed of paths, user names and emails, keeps only signature rows, and is reviewed before commit · data-protection
- S2 · each new test runs under its own viola home, never the user's, and leaves no local home in git · secret-management
  decision/no-secrets-in-git · No secrets or local homes in git — .env files, local --home test directories and signing material are never committed to git.
  decision/test-homes-gitignored · Test homes kept out of git — Local --home test directories are git-ignored so recorded state never lands in git.
  decision/tests-use-own-home · Tests use their own home — Development handles secrets as production does, and tests use their own --home directory.
- S3 · each new test seam sits only under the fake-agent feature and is compiled out of release · security-anti-patterns
- S4 · each new seam or environment read needs a log entry · security-anti-patterns-never-do-these
  decision/new-env-read-needs-log-entry · New seam or environment read needs a log entry — No further test-only seam or environment read is added without its own Decisions Log entry.
