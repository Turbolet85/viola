# security extract

## Relevance
partial: the `FAKE_AGENT_HOOK_PANIC` seam is a boundary widening the plan gates, and the panic fail-open case is a security invariant. The hyperfine perf arm and the CI job touch supply-chain rules only (a pinned CI tool, workflow hardening, WSL provisioning).

## Constraints
- **The new seam needs its own Decisions Log entry.** security-plan §Security Anti-Patterns → Universal (the `FAKE_AGENT_PUMP_DELAY_MS` carve-out clause) and the Conditions of Decisions Log `2026-09-27` (test seam) require one for any further env seam. The seam's shape is fixed by the same sources:
  - it exists only under `cfg(feature = "fake-agent")`;
  - it is absent from every release build;
  - it switches off no control and widens no redaction.
  
  §Input Validation (the `FAKE_AGENT_PUMP_DELAY_MS` row) and §Secret Management → Storage ("the one variable outside `VIOLA_*` plumbing") currently name exactly one seam. Both are spec text that the wrap cascade must widen. Phase and implement do not amend them.
- **The seam's input is bounded and read in one place.** Per §Input Validation (the "Bound every input" floor and the closed-enum rule), the seam must be read only in the `viola hook` path. Its value is interpreted as a closed set, for example presence or one fixed value, and never as free text that reaches a path, a command or a log. It must not carry the `VIOLA_` prefix, per Decisions Log `2026-09-27` on prefix meaning. Which read shape the code uses is research's question.
- **`viola hook` fails open even on a panic.** Per §Error Handling → Internal logging (the Hook Contract) and §Security Anti-Patterns → Logging ("NEVER write to stderr from `viola hook`, and NEVER exit non-zero from it"), a forced panic must still end with:
  - exit 0;
  - zero bytes on stderr;
  - no decision body on stdout.
  
  Exit 2 is forbidden. Whether the panic hook and `panic = "unwind"` already give this on the real binary on every OS is research's question.
- **The panic detail line goes to diagnostics only, and holds no secrets.** Per §Bootstrap phases `logging-redaction-wire` and §Data Protection → Logs:
  - The panic payload and backtrace (the over-4 KiB `detail-hook.ndjson` line) go only to `instances/<name>/diagnostics/`, which is 0600 inside 0700 directories, created with an explicit mode and never the umask (§Security Anti-Patterns → Data Protection).
  - The line carries no value on the NEVER-log floor: no stripped `CLAUDE*` value and no token.
  - Absolute paths in the backtrace are allowed in diagnostics. They must never reach stderr, `--json` or any external error (§Error Handling → External responses).
  - The seam's panic message should be a fixed, synthetic string.
- **No timing-only control switch.** Per §Security Anti-Patterns → Universal (`config.json`, `VIOLA_*` env or CLI flag), the perf arm must time the real hook path. No flag, env var or config key may switch off or skip any control to make a hook fast enough. The deadline breach found at P1 is surfaced to the operator, not bypassed.
- **Pinning and installing hyperfine.** Per §Dependency Security → Pinning and CI integration:
  - hyperfine is an external CLI tool installed in CI. It is not a `Cargo.lock` member and adds nothing to `deny.toml`.
  - It is installed at the version CI pins, 1.20.0. Note the plan's tool-version syntax: external CLI tools are written as minimum floors in plan text, while the CI install line carries the exact version.
  - It comes through the already SHA-pinned `taiki-e/install-action` or `cargo install --locked`. No new mutable-ref `uses:` is added.
  - Workflow `permissions: {}` and job-level `contents: read` stay as they are.
  - No `${{ github.event.* }}` value goes inside `run:`.
- **WSL re-provisioning triggers CARRY 3.** Per §Secret Management → Development (the root launch paragraph) and the Conditions of Decisions Log `2026-09-27` (browser supply chain), the root-install fix must land first if this chunk re-provisions the WSL distro. That happens if hyperfine joins the `test`-job `tool:` line that `scripts/wsl-provision.sh` replays, or if the pre-push gains a perf stage. In that case root may run only `apt-get install` over an allowlisted dry-run list, and the fix lands in this chunk. Whether `wsl-provision.sh`'s parser would pick up a new `tool:` entry is research's question.

## Patterns to follow
- **Follow the existing seam.** Model the new seam on `FAKE_AGENT_PUMP_DELAY_MS`: a `#[cfg(feature = "fake-agent")]` function in the verb's own module, read nowhere else, recorded in a Decisions Log entry with Decision / Rationale / Boundary widening / Conditions / Witness / By (per security-plan §Security Decisions Log `2026-09-27`, test seam).
- **Rely on the release check.** `scripts/release-check.sh` `judge` already refuses any `compiler-artifact` whose `features` hold `fake-agent`, and `--probe` proves the refusals. The seam relies on this rather than adding a new release path (per §Security Decisions Log `2026-09-27`, test-only feature proven absent; §Dependency Security → CI integration).
- **Use synthetic payloads.** hyperfine's hook stdin payloads come from committed, reviewed `fixtures/claude/<cli-version>/` payloads or synthetic ones. No real prompts, home paths or usernames (per §Data Protection → Repository fixtures; §Security Anti-Patterns → Data Protection).
- **Let the canary scan cover the new artifacts.** `perf-<hook>.json` lands under `target/agent-run/` and the panic detail line lands in test-home diagnostics. Both fall inside the canary scan's reach, which must stay green before any scan-gated upload (per §Secret Management → Secret scanning in CI; §Bootstrap phases `secret-scanning-ci-gate`).

## Anti-patterns to avoid
- **No seam in a release build, and no new seam without an entry.** Never let `FAKE_AGENT_HOOK_PANIC` compile into a non-`fake-agent` build, and never add any other panic or timing seam without its own Decisions Log entry (per §Security Anti-Patterns → Universal).
- **Never break fail-open.** Never write to stderr from `viola hook` and never exit non-zero from it, including under the forced panic (per §Security Anti-Patterns → Logging). Never widen the ratified `hook.event` no-verification exception to any other frame, verb or process, the perf arm's clients included (per §Security Anti-Patterns → Authentication; Decisions Log `2026-09-28`).
- **Never weaken a gate to reach green.** Never reference an Action by a mutable ref, never widen an ignore and never silence zizmor to get the new perf job green (per §Security Anti-Patterns → Code Patterns; §Dependency Security → CI integration).

## Contract bindings
- **security ↔ tests (Vector 6).** The seam's `cli_controls_not_disableable.rs` row binds security-plan §Security Anti-Patterns → Universal to the test-plan Vector 6 rstest table, which covers every `VIOLA_*` env var, global flag and `config.json` key. Whether this chunk creates that file with only the seam's row or with the full table is research's question.
- **security ↔ tests (fail-open).** The forced-panic fail-open case binds §Error Handling (Hook Contract) to test-plan §6 Security sweep and root `tests/hook_fail_open.rs`.
- **security ↔ obs.** The over-4 KiB `detail-hook.ndjson` line binds §Bootstrap phases `logging-redaction-wire` (diagnostics only, 0600) to obs D-28 (whole lines under one `write_all`) and to obs-plan §9's secret-scan canary gate.
- **security ↔ CI.** The perf job binds §Dependency Security → CI integration (SHA pins, permissions, zizmor) to test-plan §9 / obs-plan §10, whichever job shape is chosen.

## Acceptance criteria contributions
- **Fail-open under a forced panic.** A `fake-agent` build of `viola hook` with `FAKE_AGENT_HOOK_PANIC` set, run on each CI OS:
  - exits 0;
  - writes 0 bytes to stderr and 0 bytes to stdout;
  - leaves exactly one whole panic line in `diagnostics/detail-hook.ndjson`, with 0600 mode checked on Unix.
  
  A default-feature build with the same variable set does not panic and behaves as without it.
  (per security-plan §Error Handling → Internal logging; §Security Anti-Patterns → Logging / Universal)
- **Nothing test-only reaches the release build.** `scripts/release-check.sh` reads `viola only` and `release-check --probe` reads 5/5 refused with the control clean. `grep -rn FAKE_AGENT_HOOK_PANIC src/ crates/` finds the read only behind `cfg(feature = "fake-agent")` in the hook verb. (per §Security Decisions Log `2026-09-27`; §Security Anti-Patterns → Universal)
- **The supply-chain gates stay green.** With the perf job and the hyperfine 1.20.0 install:
  - `zizmor .github/workflows/` passes;
  - every new `uses:` is a full commit SHA with a version comment;
  - `permissions: {}` / `contents: read` are unchanged;
  - `cargo deny check` passes with no new ignore, skip or allow.
  
  (per §Dependency Security → CI integration; §Security Anti-Patterns → Code Patterns)
- **Canary scan and Decisions Log entry.** `viola-harness secret-scan` passes over a run that holds `perf-*.json` and the forced-panic diagnostics. The wrap report carries the seam's Decisions Log entry text (Decision / Boundary widening citing the founder's 2026-09-28 ratification / Conditions / Witness) for the cascade to record. (per §Secret Management → Secret scanning in CI; §Security Anti-Patterns → Universal)
