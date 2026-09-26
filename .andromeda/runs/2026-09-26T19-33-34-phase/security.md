# security extract

## Relevance
Partial. The chunk changes CI wiring (`ci.yml` chunk base, mutants union job) and the scope of the artifact canary scan. It does not touch the product IPC, GUI or filesystem boundaries. Security applies to the workflow supply-chain rules, the secret-scan floor and the content rule for the unscanned verdict upload.

## Constraints
- A value from the event payload may reach a step only through `env:` (the plan's own example is `AGENT_RUN_CHUNK_BASE`), never through `${{ }}` inside `run:` (per security-plan §Dependency Security, CI integration). If the new base is resolved from git history in a step or in the harness, it still must not interpolate `${{ github.event.* }}` into `run:`. Whether any event-derived value survives the change is P3's question.
- The workflow keeps `permissions: {}` at workflow level and `contents: read` at job level. `actions/checkout` keeps `persist-credentials: false`. zizmor must keep passing on the ubuntu leg with no new `excessive-permissions`, template-injection or cache-poisoning findings (per security-plan §Dependency Security, CI integration). The `git log -G` pickaxe must work from the local `fetch-depth: 0` history without persisted credentials.
- Every `uses:` must be a full-SHA ref from the pinned set of five actions (checkout, rust-cache, install-action, upload-artifact, download-artifact; download-artifact serves the `mutants-verdict` job). Any new action needs a pin plus a plan amendment (per security-plan §Dependency Security, CI integration pinned list; §Threat Model Summary, Supply chain entry point).
- The workflows carry no `concurrency:` block. A concurrency group would cancel a pending push's mutation diff and its `always()` gate and upload chain (per security-plan §Dependency Security, CI integration, declined `concurrency-limits`). This matters directly for V17 fix-push sequences in the operator pass.
- Narrowing the secret-scan scope must not weaken the floor (per security-plan §Secret Management, "Secret scanning in CI"; §Bootstrap phases, `secret-scanning-ci-gate`):
  - Every class stays: `CLAUDE*` canaries, `?t=`, `cookie:` / `viola_<port>=`, the per-home `ui/*.url` token, the content canary, and non-0600 diagnostics on Unix.
  - Coverage of test-home diagnostics, the harness capture and the nextest JUnit report stays.
  - Every scan-gated upload (`diag-`, `junit-`, `harness-<os>`) still waits for the scan to pass.
  - The scan never prints or writes matched bytes.
  - The fix is to drop stale mutation-leg residue from the scan's scope, not to remove the `?t=` pattern.
- Each leg's `mutants-verdict-<os>.json` leaves CI unscanned only because of its content. It may hold repo-relative source locations and outcomes only: never an absolute path, argv, log path or test output (per security-plan §Bootstrap phases, `secret-scanning-ci-gate`; §Security Decisions Log `2026-09-24`). Any per-leg field the union verdict adds (for example which leg compiles a mutant's code, or an unviable outcome) must stay inside that content class.

## Patterns to follow
- Pass harness inputs to steps through `env:` (the existing `AGENT_RUN_CHUNK_BASE` pattern), per security-plan §Dependency Security, CI integration.
- The canary scan runs as `id: secret-scan` with `if: always()`, and uploads are gated on its success (per security-plan §Secret Management).
- Verdict files are harness-authored and hold names and outcomes only. `mutants-verdict` downloads them with the pinned download-artifact (per security-plan §Dependency Security, pinned list; §Bootstrap phases, `secret-scanning-ci-gate`).
- Toolchains come only from rustup steps, with no toolchain action (per security-plan §Dependency Security, CI integration).

## Anti-patterns to avoid
- Never reference a GitHub Action by `@stable`, `@vN` or any other mutable ref (per security-plan §Security Anti-Patterns, Code Patterns).
- Never put `${{ github.event.* }}` inside `run:`, and never add a `concurrency:` group (per security-plan §Dependency Security, CI integration).
- Never commit local `--home` test directories or recorded residue, and never let a scan or verdict step print matched secret bytes or absolute paths (per security-plan §Security Anti-Patterns, Secrets; §Bootstrap phases, `secret-scanning-ci-gate`).

## Contract bindings
- **Secret scan ↔ obs:** the scan (`viola-harness secret-scan`) is obs-plan §9's artifact canary scan, checked against this plan's NEVER-log floor (§Bootstrap phases, `logging-redaction-wire`). Changing its scope is obs's to specify; keeping the floor and the classes is security's.
- **Secret scan ↔ tests:** the CI security gate binds to tests §CI Integration: one `ci.yml`, with secret-scan as a gated step and the planted-secret red test kept.
- **Mutants union ↔ tests:** the per-leg union verdict and the chunk-base rule belong to test-plan and its harness (`gate.rs`, `mutants.rs::resolve_base`; test-plan wrap amendment). Security binds only the verdict file's content class and the `env:`-only rule for the base.

## Acceptance criteria contributions
- zizmor over `.github/workflows/` passes on the ubuntu leg, and a grep over the `run:` blocks of `ci.yml` finds no `${{ github.event` (per security-plan §Dependency Security, CI integration).
- Every `uses:` in `ci.yml` is a 40-hex SHA from the five-action pinned set, and `ci.yml` has no `concurrency:` key (per security-plan §Dependency Security, CI integration; §Security Anti-Patterns, Code Patterns).
- The planted-secret red test still fails the scan for a planted canary. After a prior `run --mutants` leaves `target/agent-run/chunk.diff`, the scan reads green on clean homes. Scan-gated uploads still depend on `secret-scan` success (per security-plan §Secret Management, "Secret scanning in CI").
- Every string in each leg's `mutants-verdict-<os>.json` is repo-relative (for example, names start with `crates/`) and carries no absolute path, argv or log text, including any new per-leg field (per security-plan §Bootstrap phases, `secret-scanning-ci-gate`; §Security Decisions Log `2026-09-24`).

## Relevant amendment history
- **2026-09-24-three-os-ci-headless-harness-skeleton:** fixed the SHA-pinned action set and `persist-credentials: false`, replaced the toolchain action with rustup, and made event-payload values reach steps only through `env:`. That is the rule the `AGENT_RUN_CHUNK_BASE` change must keep.
- **2026-09-24-observability-gates:** recorded that CI runs obs-plan §9's canary scan before every test-home upload, lists its classes and says it never prints matched bytes. This is the scan whose scope this chunk sets.
- **2026-09-24-quality-gates:** added download-artifact (for `mutants-verdict`) as the fifth pinned action. Declined zizmor's `concurrency-limits` because a concurrency group would drop a push's mutation diff. Removed the `mutants.out/` upload and ratified the per-leg verdict JSON as unscanned only by content ("repo-relative source locations only, never absolute paths"). A union-verdict format change must keep that ratification's basis.
- **2026-09-25-pty-wrapper-on-windows:** added the R8 identity-floor canaries. Its witness was "secret-scan 0 canaries". That chunk's `?t=` self-hit on `chunk.diff` residue is the unowned CARRY this chunk resolves.
