# security extract

## Relevance
partial — a probe and possible fix inside the `viola-pty` Windows seam and its test child recorder. It adds no IPC method, listener, HTTP route or secret. What applies: the human-wins floor, PTY spawn and handle-inheritance rules, the portable-pty pin, error sanitization, the env-seam ban, CI workflow hardening if a probe job lands, and the WSL root-install CARRY if the chunk re-provisions.

## Constraints
- A resize fix or probe must never block, delay, drop or reorder a human keystroke. Refusals go only to automation. This matches the scope's own invariant (per security-plan §Security Anti-Patterns → Universal, "NEVER let a security refusal block the human").
- `PtyError`'s `Display` is hand-written. security-plan requires every message, including any new variant this chunk adds (such as a resize-observation or recorder failure), to be a fixed string that never interpolates a path, pid-bearing text or child output. Whether the current `PtyError` variants already meet this is research's question (per security-plan §Error Handling; §Bootstrap phases `error-sanitization-wire`).
- portable-pty stays `=0.8.1`. A bump or swap (the named candidate is portable-pty-psmux 0.9.7) needs three things: it must keep `CreateProcessW` with `bInheritHandles = 0`, the `RUSTSEC-2017-0008` ignore must be re-checked, and it must go to P4 as a fork (per security-plan §Data Protection → In transit "Handle inheritance"; §Dependency Security `deny.toml` additions / Pinning).
- Any new dependency or new `windows-sys` feature for the child-side size observation must meet all of these: crates.io only, inside the licence allowlist, no C-building crate, and no tokio in `viola-pty`'s graph (per security-plan §Dependency Security; §Threat Model Summary → Supply chain trust boundary).
- A knob that raises the hit rate (iterations, hold time, forced window) must not add an env var that any `viola` build reads, unless the var gets its own Decisions Log entry, stays under `cfg(feature = "fake-agent")`, is capped, and is absent from release builds. The recorder child is `pty_child_entry` in `crates/viola-pty/src/lib.rs`. Whether it is compiled only into test targets or can reach a release `viola` is research's question (per security-plan §Security Anti-Patterns → Universal; §Input Validation, the test-seam rows; §Secret Management → Storage).
- The PTY child the probe spawns must be a real executable resolved to an absolute path, never a `.cmd`/`.bat` (per security-plan §Input Validation, "Child executable resolution" row; §Security Anti-Patterns → Data Protection).
- A Windows probe job or loop in `ci.yml` must meet all of these: workflow `permissions: {}` and job `contents: read`; every `uses:` pinned by full commit SHA with a version comment; no `github.event` value inside `run:`; zizmor clean; uploads gated on the obs secret scan. If the plan touches `scripts/wsl-provision.sh` or ci.yml's `test`-job tool line, the WSL `--install-deps` hardening becomes a plan task: root may run only `apt-get install` over the allowlisted dry-run list. Otherwise the CARRY moves forward unchanged (per security-plan §Dependency Security → CI integration; §Secret Management → Development).

## Patterns to follow
- The `FAKE_AGENT_PUMP_DELAY_MS` precedent for a test-only timing lever: parsed as `u64`, capped at 5 s, `cfg(feature = "fake-agent")` only, and recorded as a boundary widening in the Decisions Log (per security-plan §Input Validation, seam row; §Security Decisions Log `2026-09-27`, Test seam).
- `scripts/release-check.sh` checks that a release build carries only `viola` and is built without `test-support` / `fake-agent`. That is the witness that recorder or probe code stays out of release builds (per security-plan §Security Decisions Log `2026-09-27` Conditions).
- The per-OS `perf` job is the shape for a dedicated CI job: tools come from its own `cargo install --locked` step, it adds no new `uses:`, and uploads go up only after the job's own `secret-scan` succeeds (per security-plan §Dependency Security → CI integration).
- Every child output read goes through `Read::take(MAX_FRAME)`, bounded and parsed tolerantly. That includes any new read of the recorder's report lines outside a test-only harness (per security-plan §Input Validation, Constants; §Security Anti-Patterns → Input).

## Anti-patterns to avoid
- NEVER protect a resize by holding back, dropping or reordering a human key on the `run` pump's write path (per security-plan §Security Anti-Patterns → Universal).
- NEVER add a new env seam that a `viola` build reads, or let a test seam reach a release build, without its own Decisions Log entry (per security-plan §Security Anti-Patterns → Universal).
- NEVER adopt a portable-pty replacement that calls `CreateProcessW` with `bInheritHandles = TRUE`, and NEVER reference a GitHub Action by a mutable ref (per security-plan §Security Anti-Patterns → Data Protection / → Code Patterns).

## Contract bindings
- security ↔ obs: any upload from a new probe job, and any committed evidence from the recorder, falls under obs-plan §9's artifact canary scan and the NEVER-log floor (security-plan §Bootstrap phases `secret-scanning-ci-gate` / `logging-redaction-wire`).
- security ↔ tests: a Windows probe job or loop binds test-plan's CI integration (one workflow, SHA-pinned, `contents: read`). Keeping recorder code out of release builds binds the `release-check.sh` test-only-feature gate.
- security ↔ arch: a portable-pty bump or swap follows arch's named swap path and must keep the handle-inheritance property. A documented ConPTY platform limit that lands as an architecture amendment goes through the wrap.

## Acceptance criteria contributions
- `cargo deny check` is green, and `bash scripts/deny-probes.sh` shows every ban still fires, including the per-sync-crate tokio ban covering `viola-pty` (per security-plan §Dependency Security).
- If `.github/workflows/` changes: `zizmor .github/workflows/` passes, every `uses:` is a full SHA, workflow `permissions: {}` and job `contents: read` hold, and no `github.event` appears in `run:` (per security-plan §Dependency Security → CI integration).
- `scripts/release-check.sh --probe` passes its cases, and `release-check.sh` reads `viola only`. Any new recorder or seam code sits behind a test-only cfg/feature (per security-plan §Security Anti-Patterns → Universal; §Security Decisions Log `2026-09-27`).
- A grep over the diff finds no new env var read in product crates without a matching Decisions Log entry, and no new `PtyError` `Display` arm that interpolates a path or child output (per security-plan §Security Anti-Patterns → Universal; §Error Handling).
