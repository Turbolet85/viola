# security · standing conventions

- S1 · each new recorded fixture is scrubbed or refused whole, reviewed before commit and kept under the fixture hygiene walk · data-protection
  decision/viola-format-fixtures-hygiene · viola-format fixtures under the hygiene walk — Committed viola-format fixtures, the old state shapes and retired words, are held to the same fixture hygiene walk as recorded fixtures: no path, user or email. · guard: viola_format_fixtures_pass_hygiene
- S2 · each new test runs in its own home and never touches the operator's real Claude config · secret-management
  decision/test-homes-gitignored · Test homes kept out of git — Local --home test directories are git-ignored so recorded state never lands in git.
  decision/tests-use-own-home · Tests use their own home — Development handles secrets as production does, and tests use their own --home directory.
- S3 · each new seam or environment read needs a log entry, and each test seam stays under the fake-agent feature · security-anti-patterns-never-do-these · security-anti-patterns
  decision/fake-agent-hook-panic-seam · Hook panic test seam — FAKE_AGENT_HOOK_PANIC exists only in test-feature builds and forces a fixed-payload hook panic only when exactly 1; the hook still fails open; it configures nothing, disables no control and widens no redaction. · why: Ratified.
  decision/fake-agent-pump-delay-seam · Pump delay test seam — FAKE_AGENT_PUMP_DELAY_MS exists only in test-feature builds, a u64 of milliseconds capped at 5 000 that holds the run pump back after the child starts; it configures nothing, disables no control and widens no redaction. · why: Lets a test land a host resize in the spawn-to-pump window; ratified.
- S4 · each new error or refusal line interpolates no path, pid, id or upstream text · error-handling
  contract/revive-refusal/no-session · revive refusal words · no-session — An --id the log does not hold is refused no-session before any spawn, and no refusal line or hint holds the recorded directory, a pid or a logged id.
  decision/error-display-no-upstream-text · Error messages carry no upstream text — The Display output of the product error enums never interpolates upstream text or absolute paths.
  decision/error-messages-interpolate-nothing · Error messages interpolate nothing — Product error types carry fixed Display messages with no path or payload interpolated. · guard: thiserror_messages_interpolate_nothing
  decision/fixed-error-display-messages · Fixed error display messages — Every product error type's Display uses fixed messages, and variant fields holding paths or payloads are excluded from Display.
  decision/no-absolute-paths-in-external-errors · No absolute paths in external errors — No external error ever exposes an absolute path.
  decision/no-identity-values-in-refusals · No identity values in verification output — No line or output of a server-verification refusal holds a pid, start time, path or endpoint.
  decision/no-paths-in-error-output · No paths or type names in error output — Absolute paths, which carry the OS username, and internal type names never go to CLI --json error output, MCP isError results or channel error.data. · why: the OS username is the only PII in scope
  decision/refusals-hold-no-paths-pids-ids · Refusals hold no paths, pids or ids — No refusal line or hint holds the recorded directory, a pid or a logged id; revive --list prints only session-shaped ids.
