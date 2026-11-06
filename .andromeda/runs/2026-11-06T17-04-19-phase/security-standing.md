# security · standing conventions

- S1 · tests and live runs use their own --home; every child of the local gate is launched under env -i with exactly HOME and PATH, no host environment value crossing into a child · Secret Management
  (security-plan/secret-management fct-gecdpdpp) secrets
- S2 · no log line, diagnostic, evidence record or CI artifact carries a CLAUDE* stripped value, user content, an absolute path or tool input; diagnostics files are 0600 · Bootstrap phases (logging-redaction-wire)
  (security-plan/data-protection fct-cettrvwp) data-rest
  (security-plan/data-protection fct-jbwneego) data-rest
  (security-plan/error-handling fct-2iushqjk) errors
  (security-plan/error-handling fct-msfbrx4e) errors
  (security-plan/error-handling fct-wyrvqwn5) errors
  (security-plan/input-validation fct-22lqecbw) input
  (security-plan/threat-model-summary fct-qgluq6ji) threat-data
- S3 · every CI upload waits on the artifact canary secret scan, which never prints or writes the matched bytes · secret-management
- S4 · a test-only seam or environment read is compiled out of every release build, switches off no control, and needs its own Decisions Log entry · Security Anti-Patterns (Universal)
  (security-plan/input-validation fct-jd7jb3il) input
- S5 · any change to the CI workflows keeps actions pinned by full SHA with a version comment, workflow-level permissions empty and job-level contents read, and event values reaching steps only through env · dependency-security
  (security-plan/dependency-security fct-dj5p5k3a) deps-supply-chain
