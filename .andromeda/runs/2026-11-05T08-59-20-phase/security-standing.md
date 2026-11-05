# security · standing conventions

- S1 · each CI job or step added or kept holds job-level contents: read under workflow permissions {}, uses SHA-pinned actions with a version comment and checkout with persist-credentials: false, and passes event or matrix values only through env: · dependency-security
  (security-plan/dependency-security fct-dj5p5k3a) deps-supply-chain
- S2 · each scan-gated upload in CI waits for the secret-scan step (if: always()) to pass, and no scan ever prints or writes the matched bytes · Bootstrap phases (secret-scanning-ci-gate)
- S3 · each test runs against its own --home, and every child of the pre-push gate runs under env -i with exactly HOME and PATH, so no host value crosses into a child · secret-management
  (security-plan/secret-management fct-gecdpdpp) secrets
- S4 · each test seam or environment read lives only under a test-only feature compiled out of release builds, switches off no control, and any new one needs its own Decisions Log entry · security-anti-patterns
  (security-plan/security-anti-patterns fct-bg5w767t) bans
- S5 · each log, diagnostics or error line obeys the NEVER-log floor: no user content, tool input, absolute path or stripped CLAUDE* value outside diagnostics/, and diagnostics files are 0600 · Bootstrap phases (logging-redaction-wire)
- S6 · each fixture or test input is synthetic, with no real prompts, home paths or usernames committed · data-protection
  (security-plan/data-protection fct-cnkeyzag) data-rest
- S7 · each new G2 zero-panics exemption needs its own Decisions Log entry · Dependency Security
