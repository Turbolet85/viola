# security · standing conventions

- S1 · each new test uses its own temporary home, kept out of git, and removes only the directories it created. · secret-management
  decision/e2e-home-removal-own-dirs-only · Test-side removal only of its own dirs — A test-side removal under target/e2e-home never takes anything but a dir the test side itself created.
  decision/no-secrets-in-git · No secrets or local homes in git — .env files, local --home test directories and signing material are never committed to git.
  decision/test-homes-gitignored · Test homes kept out of git — Local --home test directories are git-ignored so recorded state never lands in git.
  decision/test-removal-own-dirs-only · Test-side removal only of test-created dirs — A test-side removal under the e2e home takes only a directory the test side itself created, by its owner record, its own temp dir or its session name prefix.
  decision/tests-use-own-home · Tests use their own home — Development handles secrets as production does, and tests use their own --home directory.
- S2 · each new recorded fixture is scrubbed of paths, usernames, emails and user content, or refused whole, and reviewed before commit. · data-protection
  decision/fixture-hygiene · Recorded fixture hygiene — Recorded claude fixtures are never committed from sessions with real prompts, nor before being checked for home paths and usernames. · except: A workspace.repo naming this repository's own remote is kept.
  decision/verify-screen-read-against-literals · Verify screen read only against compiled literals — verify's children's screens are read only against compiled signature literals, and their row text leaves the process only as signature-only record fixtures.
  decision/verify-signature-only · Verify reads signatures only — viola verify's interactive children are read only against the compiled signature literals, their row text leaving the process only as signature-only record fixtures.
- S3 · each new log line carries closed codes only: no user content, upstream text, path, pid or identity value. · bootstrap-phases-derive-for-route-setup-project
  contract/channel-response-log/detail · channel-response log line · detail — The three -32602 faults log detail unsupported-version, invalid-params or release-from-driver at WARN on server and client lines, with no param name or value; every other channel-response line has no detail.
  contract/mcp-stdin/oversize-line · viola mcp stdin · oversize line — After the session ends one WARN parse-rejected parser mcp-stdin detail oversize count 1, no byte of the line; exit 0 for an initialized session, exit 1 with both streams empty when cut before initialize completes.
  contract/mcp-stdin/parse-rejected · MCP stdin refusal log · parse-rejected — An over-long MCP stdin line is recorded as one parse-rejected line holding no byte of it.
  decision/agents-probe-log-content-free · Agents probe log holds no content — An agents-probe spawn pair is logged only under an open log and holds no row text, id, cwd or path.
  decision/agents-probe-log-redaction · Agents-probe logging redacted — Each agents-probe spawn pair is logged with no row text, id, cwd or path.
  decision/no-identity-values-in-refusals · No identity values in verification output — No line or output of a server-verification refusal holds a pid, start time, path or endpoint.
  decision/no-messaging-secrets-in-outputs · No messaging secrets in any output — CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and any R8-stripped CLAUDE* value are never written to any log or stderr, events.ndjson, snapshots, diagnostics/ or fixtures.
  decision/no-statusline-command-in-logs · Statusline command never logged — No log line or span carries the statusline command.
- S4 · each new error or refusal message is fixed text that interpolates nothing and names no path. · error-handling
  contract/revive-refusal/no-session · revive refusal words · no-session — An --id the log does not hold is refused no-session before any spawn, and no refusal line or hint holds the recorded directory, a pid or a logged id.
- S5 · each new test-only seam or replayer lives under the test-only feature and is compiled out of release. · security-anti-patterns
  decision/fake-agent-hook-panic-seam · Hook panic test seam — FAKE_AGENT_HOOK_PANIC exists only in test-feature builds and forces a fixed-payload hook panic only when exactly 1; the hook still fails open; it configures nothing, disables no control and widens no redaction. · why: Ratified.
  decision/fake-agent-pump-delay-seam · Pump delay test seam — FAKE_AGENT_PUMP_DELAY_MS exists only in test-feature builds, a u64 of milliseconds capped at 5 000 that holds the run pump back after the child starts; it configures nothing, disables no control and widens no redaction. · why: Lets a test land a host resize in the spawn-to-pump window; ratified.
- S6 · each new external reader is bounded by the shared frame cap or named in the readers census, and parses tolerantly. · input-validation
  decision/hook-stdin-never-truncated · Hook stdin over-limit detected — Hook stdin is capped at MAX_FRAME and an over-limit payload is detected, never silently truncated.
  decision/mcp-stdin-line-cap · MCP stdin lines capped at the frame limit — Each MCP stdin line is capped at MAX_FRAME, its newline counted; an over-long line is refused whole, never truncated or skipped, and ends the session as a stdin close does, its warning holding no byte of it. · why: rmcp reads stdin lines unbounded, so viola hands it a capped reader
  decision/mcp-stdin-line-cap-on-reader · MCP stdin line cap on the reader — Each MCP stdin line is capped at MAX_FRAME, newline counted, by the reader handed to the SDK; a longer line is refused whole, never truncated or skipped, and ends the session as a stdin close does. · why: rmcp's stdio transport reads each line unbounded and its codec length bound covers only the writer.
- S7 · each new third-party dependency is exactly pinned, from crates.io, licence-allowed, builds no C and passes the deny gate. · dependency-security
  decision/bytes-advisory-floor · bytes version floor — The lockfile must resolve bytes >=1.11.1. · why: RUSTSEC-2026-0007
  decision/rustix-advisory-floor · rustix version floor — The root lockfile must resolve rustix >=1.1.6, so the advisory against 1.1.5 is cleared by the version and needs no ignore. · why: RUSTSEC-2026-0486
