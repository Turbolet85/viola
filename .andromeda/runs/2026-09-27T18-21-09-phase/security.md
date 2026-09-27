# security extract

## Relevance
partial — no product surface (no listener, IPC, token or `viola ui` code); the security load is supply chain (a new npm ecosystem + lockfile, new CI steps and possibly a new action on three OSes) and the WSL `env -i` boundary for the pre-push browser leg.

## Constraints
- Every CI step this chunk adds (Node install, `npm ci`, `playwright install --with-deps chromium`, the browser harness run) keeps workflow-level `permissions: {}` and job-level `contents: read`, and it reads no event-payload value through `${{ }}` inside `run:`. Any value it does need reaches the step only through `env:` (per security-plan §Dependency Security, CI integration).
- A new third-party action (e.g. a Node setup action) is referenced by full commit SHA with a version comment, and it is added to the plan's pinned-action set with its resolved SHA. The plan says toolchains come from install steps with "no toolchain action", so a Node setup action is a spec delta against that clause. Whether the chunk installs Node without an action is P3/P4's call (per security-plan §Dependency Security, CI integration; §Security Anti-Patterns → Code Patterns).
- `zizmor .github/workflows/` stays green over the edited `ci.yml`. This covers cache poisoning: an npm or Playwright browser cache added to a job is in scope of that finding class. The finding is never silenced and no ignore is widened to reach green (per security-plan §Dependency Security, CI integration; §Threat Model Summary, Supply chain vector).
- The WSL pre-push browser leg installs only CI's own pins: the Node, `@playwright/test` and Chromium versions are parsed from CI's own source, never taken from a host value. Every WSL call runs under `env -i` (HOME + PATH, plus only the named distro-derived constants). A new assignment whose value comes from the host is a boundary widening that needs its own Decisions Log entry (per security-plan §Dependency Security, Pinning (WSL2 `Ubuntu` distro); §Secret Management, Development).
- No gate, harness command or plan entry runs the browser step, or any distro command, through `scripts/wsl-exec.sh`. Only its `--probe` is exempt (per security-plan §Secret Management, Development; §Security Decisions Log 2026-09-27 Conditions).
- The committed npm lockfile (`e2e-web/package-lock.json`) sits outside `cargo deny`'s root graph, as `fuzz/Cargo.lock` does. The plan admits a non-root lockfile only as an operator-ratified, test-only exemption that has its own advisory and source audit. The plan has no npm audit row today. Whether this chunk wires an npm audit gate, or records a ratified exemption and defers the gate, is P3/P4's decision, and neither route may add a `skip`/`allow` or widen an ignore (per security-plan §Dependency Security, `fuzz/` exemption precedent; §Threat Model Summary, Supply chain trust boundary).
- If the stub page is served by a fixture server rather than `file://` or a Playwright `route` fulfilment, that server is a new TCP listener. It binds loopback only (never `0.0.0.0`/`::`) and needs a Security Decisions Log entry before it lands (per security-plan §Security Anti-Patterns → Universal, "NEVER add a listener"; → Data Protection, loopback-only bind).

## Patterns to follow
- The `fuzz/` precedent for a second lockfile: a separate test-only package that is never linked into `viola` or joined to the root workspace, has a committed lockfile and its own audit step in the ubuntu `supply-chain` job (plus the weekly `nightly.yml` advisories), and is ratified in the Decisions Log (per security-plan §Dependency Security).
- The WSL provisioning pattern: `scripts/wsl-provision.sh` parses CI's own pin line and never runs sudo, and `pre-push` refuses `tool-pin-mismatch` when an installed tool differs from that line. Extend the same parse-and-refuse shape to the Node and Playwright pins. A root apt dependency (Playwright's `--with-deps` system libraries) is named as a `tool-missing`-style refusal, not installed with sudo (per security-plan §Dependency Security, Pinning).
- The harness capture passes the obs-owned artifact canary scan (`viola-harness secret-scan`, `if: always()`) before any upload, so `pw.json` and `junit-playwright.xml` under `target/agent-run/` are scanned, and every upload that carries them waits for the scan (per security-plan §Secret Management, Secret scanning in CI; §Bootstrap phases, secret-scanning-ci-gate).
- The release build carries `viola` only. `e2e-web/` stays test-side and never reaches a release artifact (per security-plan §Threat Model Summary, CI/CD; release-check row).

## Anti-patterns to avoid
- NEVER reference any action (including a new Node or browser setup action) by `@vN`, `@stable` or another mutable ref (per security-plan §Security Anti-Patterns → Code Patterns).
- NEVER let a host environment value cross into the WSL distro for the browser leg: no `WSLENV` forwarding and no host-derived `PATH`/`NODE_*`/`PLAYWRIGHT_*` assignment (per security-plan §Secret Management, Development).
- NEVER widen an ignore or silence zizmor or a supply-chain finding to reach green (per security-plan §Dependency Security, CI integration).

## Contract bindings
- security ↔ obs: the canary `secret-scan` (obs-plan §9 step 3) must cover the new `playwright` capture files before upload. Whether its read set already includes them is research's question.
- security ↔ tests: the test-plan §3 `run --browser` / §Toolchain amendments (three OSes plus the WSL leg) bind to this plan's CI integration rows (permissions, SHA pins, zizmor) and to its WSL `env -i` / CI-pins-only rule.
- security ↔ arch: a new npm ecosystem, and any new action or fixture listener, is a §Stack / §Occupied Resources delta that the Decisions Log must mirror.

## Acceptance criteria contributions
- `zizmor .github/workflows/` passes on the edited workflows, every `uses:` is a 40-hex SHA with a version comment, and `permissions: {}` / `contents: read` hold on every job the chunk touches (per security-plan §Dependency Security, CI integration).
- The pre-push WSL browser leg's canary reads 0 `CLAUDE*` variables inside the distro, and every new WSL call is an `env -i` launch that carries no host-valued assignment (per security-plan §Secret Management, Development).
- `grep -rn wsl-exec` over `crates/`, `src/`, `.github/` and `scripts/agent-run.*` finds no caller, and every plan `[[gate]]` hit is exactly `wsl-exec.sh --probe` (per security-plan §Secret Management, Development).
- The npm lockfile is committed, and it either has an advisory and source audit step in CI or a Security Decisions Log entry that ratifies it as test-only with the audit deferred. It is not left unaudited and unrecorded (per security-plan §Dependency Security, `fuzz/` exemption precedent).
