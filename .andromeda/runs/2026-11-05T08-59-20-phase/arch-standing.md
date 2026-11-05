# arch · standing conventions

- S1 · each test runs in its own viola home (--home), so parallel tests never share an endpoint, a log or budget.json, and runs against the fake agent, never the real claude in CI · Established Decisions
- S2 · each test home sits under target/e2e-home and a new test home removes only the dirs whose recorded owner process is gone, never by age or name · occupied-resources
- S3 · each test-only evidence report outside the viola home is kept when the test panics or is killed and removed when it passes · occupied-resources
  (architecture/occupied-resources fct-jv5gqhbl) res-filesystem
- S4 · each test-harness-only environment variable uses the AGENT_RUN_ prefix and is never read by the viola binary · Conventions
- S5 · each committed fixture is synthetic, scrubbed and hygiene-walked, a fake-agent script validated against schemas/fake-script.v1.json · occupied-resources
- S6 · each log line is one complete JSON object per single write, codes only in the home-level role files and content only in the instance detail files, fixed by the obs-owned diag schemas · Cross-cutting Patterns
  (architecture/occupied-resources fct-bmahq2ma) res-filesystem
  (architecture/stack-and-technologies fct-rzlme45f) stack
  (architecture/stack-and-technologies fct-zse22tqm) stack
