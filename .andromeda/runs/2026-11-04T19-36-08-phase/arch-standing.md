# arch · standing conventions

- S1 · every new test runs in its own viola home through --home, under target/e2e-home (viola-test-*), kept in CI for the obs gates, a new home removing only dirs whose recorded owner process is gone · occupied-resources
- S2 · every new end-to-end or integration test drives the fake agent named after `--`, never the real claude in CI · Established Decisions
- S3 · every OS-specific branch or cfg-gated test compiles and is tested on its own CI runner · Cross-cutting Patterns
- S4 · test-harness-only variables carry the AGENT_RUN_ prefix and are never read by viola; a test seam is read only under cfg(feature = "fake-agent") and absent from release builds · Conventions
- S5 · every new log line is codes only, one complete JSON object per write, in its role file under diagnostics/ and conforming to diag-line.v1.json; content-bearing detail only in the instance detail files · Cross-cutting Patterns
- S6 · every new test-only endpoint or evidence file sits outside the viola-<h12> namespace, unique per test (pid and label), its report kept when the test panics or is killed and removed when it passes · occupied-resources
- S7 · every new path a test or job writes under target/ is gitignored and recorded in Occupied Resources' Repository list · occupied-resources
- S8 · every new fake-agent scenario is a committed synthetic-only script validated against schemas/fake-script.v1.json · occupied-resources
