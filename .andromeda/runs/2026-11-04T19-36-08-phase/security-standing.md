# security · standing conventions

- S1 · each new test runs under its own --home and takes no host environment value into a child; the local pre-push gate launches children under env -i with exactly HOME and PATH, and any assignment of a host value is a boundary widening that needs a Decisions Log entry · Secret Management (Development)
  (security-plan/secret-management fct-gecdpdpp) secrets
- S2 · no new test seam, environment read or flag may switch off any control in the plan; a new seam or env read, a Unix-only test seam included, needs its own Decisions Log entry · security-anti-patterns
- S3 · each new CI job or step pins its actions by full commit SHA with a version comment, uses checkout with persist-credentials false, holds workflow permissions {} and job contents read, passes matrix and event values only through env:, adds no concurrency block, and passes zizmor · dependency-security
  (security-plan/dependency-security fct-glgoi2kc) deps-supply-chain
  (security-plan/dependency-security fct-dj5p5k3a) deps-supply-chain
  (security-plan/threat-model-summary fct-dvr22577) boundary-supply-chain
- S4 · each new CI upload waits for the artifact canary secret-scan to pass, and every input it carries is synthetic · Secret Management (Secret scanning in CI)
- S5 · each new log or diagnostics line keeps the NEVER-log floor: no stripped CLAUDE* value, no user content or drift report outside 0600 diagnostics/, and no absolute path, upstream text or internal type name in any external error · Bootstrap phases (logging-redaction-wire)
  (security-plan/error-handling fct-2c6j473b) errors
  (security-plan/error-handling fct-mvo56ebm) errors
  (security-plan/error-handling fct-yrpufic3) errors
  (security-plan/error-handling fct-tr2s4okr) errors
- S6 · each new fixture is synthetic and is checked for home paths, usernames and tool input before it is committed · data-protection
  (security-plan/data-protection fct-cnkeyzag) data-rest
- S7 · no new G2 zero-panics exemption beyond the FAKE_AGENT_HOOK_PANIC seam's without its own Decisions Log entry · Dependency Security (CI integration)
