# arch · standing conventions

- S1 · each new test runs in its own isolated viola home, its home removed by owner record only, its sockets under the short test root · occupied-resources
  contract/e2e-home-refusal/headroom-message · e2e-home base refusal · headroom message — A base below the floor is refused with the fixed message `e2e-home backing: {free} MiB free, below the {floor} MiB floor`, the two counts and no path.
  contract/harness-environment/agent-run-keep-failed · test harness environment · AGENT_RUN_KEEP_FAILED — The value 1 keeps a failing test's home for post-mortem; run --mutants sets it to 0 on the cargo mutants command.
  contract/harness-environment/agent-run-keep-homes · test harness environment · AGENT_RUN_KEEP_HOMES — The value 1 keeps the test and session homes for the CI gates.
  decision/dev-host-e2e-home-on-tmpfs · Dev-host test homes on a tmpfs link — On the Linux dev host the e2e-home base is a link to a tmpfs, an environment fact named by no variable, flag or config key; the home path strings are unchanged and CI homes stay on the runner's disk. · why: so a start's pinned copy does not wait on the shared volume's dirty-page budget
  decision/e2e-home-headroom-floor · e2e-home backing headroom floor — A linked e2e-home base whose filesystem has less than 9 GiB free is refused; the floor is one compiled constant, equal in both keepers and set by no variable, flag or config key. · why: the tmpfs's EDQUOT reading of about 8 GiB free plus 1 GiB
  decision/e2e-home-owner-only-headroom · End-to-end home base checked before use — The keepers of the dev host's end-to-end home base refuse a target that is not a real owner-only folder or whose filesystem has less than the 9 GiB headroom floor free.
  decision/linked-e2e-home-base-strict · Linked e2e-home base must be a private real directory — A linked e2e-home base must have an absolute target, created with mode 0700 when gone and never chmodded after, and is accepted only when it reads as a real directory with no group or other bit; anything else refuses the start with a fixed message naming no path.
  decision/test-sockets-under-short-tmp-root · Test channel sockets under a short /tmp root — Test channel sockets bind under a test-owned short root directory made under /tmp, never under TMPDIR; the product endpoint socket is not under that root. · why: So the socket path length bound holds whatever the length of TMPDIR.
- S2 · each new test runs against the fake agent, never a live claude, on Linux in CI · ci-cd-approach
- S3 · each new log line is codes-only, emitted through obs_event!, with no content, address, pid or workspace · cross-cutting-patterns
- S4 · each new end-to-end test holds zero panic lines and keeps insta snapshots in check mode only · project-directory-structure
  decision/snapshots-check-mode-only · Insta snapshots in check mode only — The agent crate's decision-body snapshots are only checked by tests, never written by them; the root integration tests take no snapshots.
  decision/zero-panics-gate · Zero panic lines in end-to-end runs — No panic role line may appear under the end-to-end home, checked fail-closed. · guard: obs G2 zero-panics gate · except: A panic whose location is the hook seam module.
- S5 · each new module is wired into the module tree with no orphan module, no print macro and no tracing level macro · build-system
  decision/no-orphan-modules · No orphan modules — No lib or bin target holds an orphan module; the check is a CI gate and is proven to fire on planted orphans. · guard: scripts/orphans-check.sh
- S6 · each new CLI flag, subcommand and --json field follows the CLI flag style, snake_case field names and kebab-case enum values · conventions
  contract/refusal-reason/detail · RefusalReason · detail — A kebab-case string from a closed set per reason, extended only in viola-core; readers treat an unrecognised code-word detail as opaque text, never as an error.
  contract/refusal-reason/human-typing · RefusalReason · human-typing — Serialised kebab-case; its detail is null or manual-pause.
  contract/refusal-reason/not-delivered · RefusalReason · not-delivered — Serialised kebab-case; its detail is one of input-not-ready, no-prompt-submitted, turn-running, unknown-dialog, control-character, empty-text, paste-cut.
  contract/refusal-reason/unverified-cli · RefusalReason · unverified-cli — Serialised kebab-case; its detail is null.
- S7 · each new test keeps the coverage floors enforced by the harness gate · ci-cd-approach
  decision/coverage-floors · Coverage floors — The test job runs the instrumented suite with coverage floors 85/95/80, then doctest, and its gate requires coverage and doctest. · guard: gate --require coverage,doctest
  decision/coverage-floors-gate · Coverage floors enforced by the harness gate — Coverage is measured by cargo-llvm-cov and the harness gate enforces the coverage floors.
