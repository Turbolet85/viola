
## 2026-09-27-browser-verdict-reachability — npm lockfile audit, pinned Node, operator-only root install in WSL
**Section:** §Dependency Security (Audit tool; Pinning, incl. the WSL2 bullet; CI integration: Job 4, nightly, Toolchains) · §Bootstrap phases (`dep-audit-tooling-install`, `dep-security-ci-gate`) · §Secret Management (Storage → Development) · §Security Decisions Log (new `2026-09-27` entry)
**Change:**
- The test-side npm graph `e2e-web/package-lock.json` (`@playwright/test` =1.63.0, 3 packages) gets its own gate: `scripts/npm-audit.sh` (advisories at every level + registry.npmjs.org-only sources; `--probe`; `--advisories-only`, the weekly `npm-advisories` twin), JSON in `target/npm-audit/`, never the uploaded `target/supply-chain/`.
- Node is pinned exactly: the `NODE_PIN_*` lines in ci.yml's workflow `env:` (their only home), parsed from the file text by `scripts/install-node.sh` (sha256 checked before extraction). Toolchains: Node from `Node (pinned)`, Chromium from Playwright's own install; no toolchain action, no new `uses:`.
- WSL2: the distro also installs the pinned Node and Chromium; `pre-push` refuses `node` off-pin; the `env -i` PATH gains only `<home>/.local/viola-node/bin`. "The script never runs sudo" became "the user run never runs sudo".
- Development: one root launch, `wsl.exe -d Ubuntu -u root … wsl-provision.sh --install-deps <user home>`, operator-only (never the gate tool, a harness command or a pre-push stage); it ran once on 2026-09-27 (28 packages). `wsl-exec.sh` is now "the one other user-level launcher" (was "one other launcher").
**Why:** founder ruling W125's pipe; P4 operator forks 1 and 3. The root launch is a boundary widening, ratified live by the overseer under the founder's 2026-09-27 ruling, operator-only, because as shipped root runs user-writable code; before any re-provision `--install-deps` must run only `apt-get install` over an allowlisted dry-run list (a route CARRY).
**Kept:** the Threat Model Summary is a verbatim upstream copy and was not amended; the facts live in §Dependency Security.
**Ref:** .andromeda/runs/2026-09-27T19-50-23-wrap/
