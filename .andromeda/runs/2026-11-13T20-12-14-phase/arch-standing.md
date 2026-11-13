# arch · standing conventions

- S1 · each new or changed test runs against the fake agent in its own isolated viola home, never a live claude · established-decisions · ci-cd-approach
  decision/ci-ubuntu-fake-agent · CI on ubuntu against the fake agent — CI builds, lints and tests on ubuntu-latest native runners against the fake agent, with SHA-pinned actions and toolchains installed by rustup.
  decision/test-isolated-viola-home · Each test in its own viola home — Each test runs in its own viola home (`--home`), so parallel tests never share an endpoint, log or `budget.json`.
- S2 · an OS-gated test names its reason; no test gate names another OS · cross-cutting-patterns
- S3 · each new module or test support file joins the module tree (no orphan modules) and passes formatting and clippy with warnings denied and the print and dbg bans · build-system
  decision/fmt-and-clippy-gate · Formatting and clippy with warnings denied — Code passes the cargo fmt check and clippy over the whole workspace, all targets and the fake-agent feature, with every warning denied.
- S4 · end-to-end test runs leave zero panic lines; insta snapshots in check mode only · project-directory-structure
  decision/chaos-home-outside-gates · Chaos panic home outside the obs gates — The forced vt100 feed-panic test uses a home outside the e2e home tree, so its deliberate panic line falls outside the zero-panic gate, schema conformance and the secret scan; the test asserts the panic and parse-rejected lines itself. · why: the one named carve-out to the test-data rule · guard: chaos_feed_panic
  decision/snapshots-check-mode-only · Insta snapshots in check mode only — The agent crate's decision-body snapshots are only checked by tests, never written by them; the root integration tests take no snapshots.
  decision/zero-panics-gate · Zero panic lines in end-to-end runs — No panic role line may appear under the end-to-end home, checked fail-closed. · guard: obs G2 zero-panics gate · except: A panic whose location is the hook seam module.
- S5 · each test-side removal stays outside the working directory and test homes are removed by owner record only · occupied-resources
  decision/end-to-end-homes-kept-after-each-job · Homes kept after each job's end-to-end run — Each CI job keeps its harness and test homes after its end-to-end run, whatever the outcome (AGENT_RUN_KEEP_HOMES "1" on the job, if: always()), the viola-chaos-* home out. · why: The homes are kept in CI for the obs gates.
  decision/test-side-removal-outside-workdir · Test-side removals outside the working directory — Through the e2e-home link, the owner sweep, a test home's drop and harness cleanup create and delete outside the working directory, each taking only a dir the test side itself made. · why: a boundary widening, ratified
