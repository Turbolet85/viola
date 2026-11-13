# arch · standing conventions

- S1 · each new test runs in its own isolated viola home, which only its owner removes · established-decisions · occupied-resources
- S2 · each new OS-gated test names its reason and names no other OS · cross-cutting-patterns
- S3 · each new test file or test-support module is declared in its module tree, compiles under clippy with warnings denied and the print bans, and stays out of product builds · build-system
- S4 · each new test runs in CI against the fake agent, never against a live claude · ci-cd-approach
- S5 · each new test keeps its evidence when it is killed, and its removals stay outside the working directory · occupied-resources
  decision/end-to-end-homes-kept-after-each-job · Homes kept after each job's end-to-end run — Each CI job keeps its harness and test homes after its end-to-end run, whatever the outcome (AGENT_RUN_KEEP_HOMES "1" on the job, if: always()), the viola-chaos-* home out. · why: The homes are kept in CI for the obs gates.
