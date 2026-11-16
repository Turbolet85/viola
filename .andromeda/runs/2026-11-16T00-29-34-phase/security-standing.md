# security · standing conventions

- S1 · each new test uses its own viola home, kept out of git, and removes only the directories it created · secret-management · bootstrap-phases
  decision/no-secrets-in-git · No secrets or local homes in git — .env files, local --home test directories and signing material are never committed to git.
  decision/test-homes-gitignored · Test homes kept out of git — Local --home test directories are git-ignored so recorded state never lands in git.
- S2 · each new fixture is scrubbed, under the hygiene walk and reviewed before commit · data-protection
  decision/no-messaging-secrets-in-outputs · No messaging secrets in any output — CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and any R8-stripped CLAUDE* value are never written to any log or stderr, events.ndjson, snapshots, diagnostics/ or fixtures.
- S3 · each new test seam or environment read lives only under the fake-agent feature and needs its log entry · security-anti-patterns-never-do-these · security-anti-patterns
  decision/fake-agent-hook-panic-seam · Hook panic test seam — FAKE_AGENT_HOOK_PANIC exists only in test-feature builds and forces a fixed-payload hook panic only when exactly 1; the hook still fails open; it configures nothing, disables no control and widens no redaction. · why: Ratified.
  decision/fake-agent-pump-delay-seam · Pump delay test seam — FAKE_AGENT_PUMP_DELAY_MS exists only in test-feature builds, a u64 of milliseconds capped at 5 000 that holds the run pump back after the child starts; it configures nothing, disables no control and widens no redaction. · why: Lets a test land a host resize in the spawn-to-pump window; ratified.
- S4 · each new guard test is proved by a one-off remove-the-guard control · threat-model-summary
