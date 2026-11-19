# security · standing conventions

- S1 · each new recorded fixture is scrubbed or refused whole, kept local until reviewed, and reviewed before commit · data-protection
- S2 · each new test runs under its own test-owned home and removes only the dirs it created · secret-management · security-anti-patterns-never-do-these
  decision/tests-use-own-home · Tests use their own home — Development handles secrets as production does, and tests use their own --home directory.
- S3 · each new test seam lives only under the fake-agent feature and is compiled out of the release build · security-anti-patterns · input-validation
  decision/fake-agent-hook-panic-seam · Hook panic test seam — FAKE_AGENT_HOOK_PANIC exists only in test-feature builds and forces a fixed-payload hook panic only when exactly 1; the hook still fails open; it configures nothing, disables no control and widens no redaction. · why: Ratified.
  decision/fake-agent-pump-delay-seam · Pump delay test seam — FAKE_AGENT_PUMP_DELAY_MS exists only in test-feature builds, a u64 of milliseconds capped at 5 000 that holds the run pump back after the child starts; it configures nothing, disables no control and widens no redaction. · why: Lets a test land a host resize in the spawn-to-pump window; ratified.
- S4 · each new log line carries no CLAUDE value, no credential and no upstream text; user content goes only to diagnostics · bootstrap-phases-derive-for-route-setup-project
  decision/agents-probe-log-content-free · Agents probe log holds no content — An agents-probe spawn pair is logged only under an open log and holds no row text, id, cwd or path.
  decision/agents-probe-log-redaction · Agents-probe logging redacted — Each agents-probe spawn pair is logged with no row text, id, cwd or path.
  decision/user-content-only-to-diagnostics · User content only in diagnostics — Outside the content-carrying contract payloads, user content, tool input and drift reports go only to the instance diagnostics, never to stderr, CLI --json errors, MCP isError results or channel error.data. · except: events.ndjson, wait and last results and hook.dialog carry content by design
