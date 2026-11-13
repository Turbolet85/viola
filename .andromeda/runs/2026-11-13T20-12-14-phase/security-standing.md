# security · standing conventions

- S1 · each new or changed test uses its own home under a test-created directory, never the user's viola or Claude home, and removes only what it created · secret-management · security-anti-patterns
  decision/nothing-written-under-claude-home · Nothing written under the Claude home — viola writes nothing under ~/.claude.
  decision/test-homes-gitignored · Test homes kept out of git — Local --home test directories are git-ignored so recorded state never lands in git.
- S2 · each committed or recorded fixture is scrubbed of absolute paths, user words and emails, or refused whole, and walked by the hygiene test · data-protection · security-anti-patterns-never-do-these
- S3 · each test seam stays under the fake-agent feature and out of the release build · security-anti-patterns · input-validation
  decision/fake-agent-hook-panic-seam · Hook panic test seam — FAKE_AGENT_HOOK_PANIC exists only in test-feature builds and forces a fixed-payload hook panic only when exactly 1; the hook still fails open; it configures nothing, disables no control and widens no redaction. · why: Ratified.
  decision/test-support-dev-only · test-support enabled only for development — viola-channel's test-only test-support feature is enabled only by the root package's dev-dependencies.
