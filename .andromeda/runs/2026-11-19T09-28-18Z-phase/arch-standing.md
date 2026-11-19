# arch · standing conventions

- S1 · each new recorded fixture file passes the recorder's scrub and hygiene walk, comes only from a clean verify, and screen fixtures keep only signature rows · occupied-resources
- S2 · each new or changed test runs against the fake agent in CI, never a live claude · ci-cd-approach
- S3 · each new test runs in its own viola home and every wait it takes stays inside the one shared root wait bound · established-decisions · occupied-resources
  decision/wait-report-kept-on-failure · Wait report kept on failure — A test wait writes a report created empty at its start, with a line only when the polled state changes, codes, counts and booleans only; the report is kept when the test panics or is killed and removed when it passes. · why: A test the runner kills keeps its evidence.
- S4 · each live start of the real claude is a local-only probe, never a CI job · ci-cd-approach
