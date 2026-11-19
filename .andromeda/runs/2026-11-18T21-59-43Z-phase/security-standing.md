# security · standing conventions

- S1 · each new test runs under its own test-owned home, never the user's real viola home or Claude config · secret-management
  decision/mcp-fixtures-via-test-only-tap · MCP fixtures recorded through a test-only tap — MCP exchange fixtures are recorded by a test-only tap, started live by the standing rig under its private CLI config, that spawns viola mcp directly with no shell, caps each line, keeps its raw capture 0600 in a private directory never committed, and applies the same scrub and closed refusal codes. · guard: schemas/claude-mcp-fixture.v1.json
  decision/test-homes-gitignored · Test homes kept out of git — Local --home test directories are git-ignored so recorded state never lands in git.
- S2 · each new test seam lives only under the fake-agent feature and is compiled out of the release build · security-anti-patterns
  decision/fake-agent-hook-panic-seam · Hook panic test seam — FAKE_AGENT_HOOK_PANIC exists only in test-feature builds and forces a fixed-payload hook panic only when exactly 1; the hook still fails open; it configures nothing, disables no control and widens no redaction. · why: Ratified.
  decision/fake-agent-pump-delay-seam · Pump delay test seam — FAKE_AGENT_PUMP_DELAY_MS exists only in test-feature builds, a u64 of milliseconds capped at 5 000 that holds the run pump back after the child starts; it configures nothing, disables no control and widens no redaction. · why: Lets a test land a host resize in the spawn-to-pump window; ratified.
- S3 · each new output line carries fixed words, interpolates nothing from upstream, and honours the never-log floor on stderr · error-handling
  contract/revive-json/output-fields · revive --json output · output fields — revive --json prints codes, closed words and session-shaped ids only, never a path or pid.
  contract/revive-list-json/document · viola revive --list --json · sessions document — Prints {"v":1,"ok":{"sessions":[{"ts","cause","id"}]}} in log order: codes, closed words and session-shaped ids only, never a path or pid.
  decision/error-messages-interpolate-nothing · Error messages interpolate nothing — Product error types carry fixed Display messages with no path or payload interpolated. · guard: thiserror_messages_interpolate_nothing
  decision/fixed-error-display-messages · Fixed error display messages — Every product error type's Display uses fixed messages, and variant fields holding paths or payloads are excluded from Display.
  decision/stderr-never-log-floor · stderr honours the never-log floor — stderr never carries upstream text, tool input, drift-report values or any other value on the NEVER-log floor.
