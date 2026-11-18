# arch · standing conventions

- S1 · every recorded fixture file is scrubbed and hygiene-walked: no host path, user name, email, token or status-line text · occupied-resources
- S2 · screen fixtures keep only the signature rows; kept full screens stay local and are never committed or uploaded · occupied-resources
- S3 · each new test runs in its own isolated viola home, removed by its owner · established-decisions · occupied-resources
  decision/chaos-home-outside-gates · Chaos panic home outside the obs gates — The forced vt100 feed-panic test uses a home outside the e2e home tree, so its deliberate panic line falls outside the zero-panic gate, schema conformance and the secret scan; the test asserts the panic and parse-rejected lines itself. · why: the one named carve-out to the test-data rule · guard: chaos_feed_panic
  decision/test-home-owner-sweep · Test homes removed by owner record only — A test home sweep removes only dirs whose recorded owner process is gone, never by age or name and never a dir without a record, and every removal deletes the owner record last. · why: a removal that stops part-way keeps the record for a later sweep
  decision/test-homes-cleaned-by-owner · Test homes cleaned by owner process — Harness and test homes are kept in CI for the obs gates, and a new test home removes only the dirs whose recorded owner process is gone, never by age or name.
  decision/test-isolated-viola-home · Each test in its own viola home — Each test runs in its own viola home (`--home`), so parallel tests never share an endpoint or log.
  decision/test-side-removal-outside-workdir · Test-side removals outside the working directory — Through the e2e-home link, the owner sweep, a test home's drop and harness cleanup create and delete outside the working directory, each taking only a dir the test side itself made. · why: a boundary widening, ratified
- S4 · tests and CI jobs run against the fake agent, the real claude CLI only locally · ci-cd-approach
