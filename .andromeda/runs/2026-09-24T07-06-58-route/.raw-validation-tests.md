# Tests validation — route draft

## Insert
- Before `Three-OS CI and headless harness`: **"Test runner and coverage tooling — nextest ci/mutants profiles, zero retries, fixed-port group, rstest/proptest/insta/assert_cmd dev-deps, cargo-llvm-cov, e2e-web Playwright pin"** (epoch: `Epoch 1 — Foundation`)
  Reason: The draft has no test-runner-install chunk, even though test-plan §3 Bootstrap phases puts it first (test-runner-install → 5-command-discipline-wire), and the harness `run` and the mutation gate in chunk 1 both call nextest.
- Between `Capability ledger and viola verify` and `Readiness gate and timing constants`: **"Fake-agent drift contract — fake agent's hook sequences and payloads equal recorded viola verify fixtures per CLI version, annotations forwarded, fixture schema check"** (epoch: `Epoch 2 — Windows slice I: wrapper, events, ledger`)
  Reason: Per test-plan §6 Contract suite and §7 Seed strategies, recorded fixtures come only from `viola verify` after scrubbing, so the drift contract cannot run before the ledger/verify chunk records them; writing fixtures by hand in Foundation would break §11 Test Data.

## Reorder
- Move `Linux live confirmation`, `Unix endpoint and home hardening` and `Linux and macOS parity` before `viola ui loopback server`
  Reason: Per test-plan §3 `run` step 3 and §9 (E2E row), the Playwright suite runs only on ubuntu, and every spec boots harness `viola run` sessions, so the whole Epoch 7 browser layer needs the Linux openpty wrapper and Unix endpoint first. Also, §9 fails the build when any test fails on any of the 3 OSes, so a later Unix slice leaves the ubuntu/macOS legs red.

## Rewrite
- `Fake-agent contract suite`: "Fake-agent contract suite — recorded per-CLI-version hook fixtures, scripted dialogs and modes, receipts, drift contract incl. annotations, scrubbed fixtures" → "Fake agent and test-data fixtures — scripted modes and control file, receipts, rstest temp-home fixture chain under target/e2e-home, fixture scrub-and-schema walk, committed proptest seeds"
  Reason: Per test-plan §3 test-data-bootstrap-wire and §7, the Foundation fixture chunk needs the `home → stamped_home → booted_wrapper` chain and per-test temp homes, while the recorded-fixture drift contract moves to after verify (see Insert).
- `Three-OS CI and headless harness`: "toolchain 1.96" → "pinned stable toolchain, per-OS agent-run artifact upload"
  Reason: Per test-plan §9 Matrix builds, the primary CI toolchain is the pinned current stable (≥1.96) and 1.96 is only the MSRV job; §9 Test report format also requires `agent-run-<os>` artifact upload.
- `Quality gates`: "hook perf budgets in their own job, seeded property and fuzz corpus replay" → "fmt/clippy -D warnings lint, MSRV 1.96 job, seeded property and fuzz corpus replay"
  Reason: Per test-plan §9 Lint and MSRV rows and §10 Build failure conditions, the lint and MSRV gates appear in no chunk. The perf job cannot run in Foundation because §3 `gate` treats a missing required `perf-<hook>.json` as a breach, and no hooks exist yet.
- `Hooks to normalised events`: "spine deadline" → "spine and SessionEnd deadlines gated by the perf job"
  Reason: Per test-plan §10 Performance budgets, the hyperfine `max` gate on hook deadlines can start only once hooks exist, so its job belongs with this chunk.
- `Strip-bay live page`: "cell-for-cell parity with viola list" → "cell-for-cell parity with viola list, Playwright halves of Paths 2–6, E1, E3"
  Reason: Per test-plan §3 `run` step 3 cross-runner rule and §11 Test Strategy, each multi-surface critical path's web bullets need a spec test that owns them, and the first chunk that can deliver them is the one where the page exists.
