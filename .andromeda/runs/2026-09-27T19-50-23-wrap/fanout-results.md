# Fan-out results — wrap 2026-09-27T19-50-23 · chunk 2026-09-27-browser-verdict-reachability

Seven Explore doc-agents ran in one parallel batch with the verbatim amendment-flow prompt. Each return was YAML (no
HTML entities in any value; nothing stripped but the trailing `#` commentary, which is summarised per doc below).

## Verdicts
- **design-system:** `proposals: []`. Commentary: no `hardcoded✗` in Coverage. The expected §Typography / Fonts amendment is
  outside D-design-tokens.
- **layout-templates:** `proposals: []`. Commentary: the stub is a test fixture, not a web-spa region. Its `Drivers` hit at :607
  is a different table.
- **a11y-plan:** `proposals: []`. Commentary: §3 CI integration (:275, :671, :1126, :1265) and D-A11Y-12 (:1332) still state
  ubuntu-only and `browser-linux-only`, which is outside its two detectors. Raised by the orchestrator at check 5.
- **obs-plan:** `proposals: []`. Commentary: §9 artifact table (:1243) and step order (:1287, :1288) still name nextest JUnit
  only, which is outside its three detectors. Raised by the orchestrator at check 5.
- **architecture:** 17 proposals (A1–A17).
- **security-plan:** 11 proposals (S1–S11).
- **test-plan:** 31 proposals (T1–T31: 18 D-tests-framework, 13 D-tests-obs-harness).

## Proposals and dispositions
Each line reads: detector · section · change (condensed from the return) → disposition [the check that decided it].

### architecture
- A1 D-arch-resources · Occupied Resources → Filesystem · register `<temp dir>/viola-root-watch/<test>.<label>.report` (test
  support `Watch`, 7 s, 8 sites, kept on panic) → **apply** [1: Accurate this-chunk addition].
- A2 D-arch-resources · Occupied Resources → Repository · register `target/npm-audit/audit.json` (outside `target/supply-chain/`)
  → **apply** [1].
- A3 D-arch-resources · Occupied Resources → Repository · the `e2e-web/test-results/` bullet → `node_modules/`, `pw.json`,
  `pw-junit.xml`, `test-results/` written from this chunk; a11y/lint still future → **apply** [1].
- A4 (dep A) · Repository · the `junit-<os>` upload carries 2 paths → **apply** [1].
- A5 (dep A) · Repository · run-archive holds the playwright JUnit (sourced from `pw-junit.xml`) → **apply** [1].
- A6 D-arch-resources · Filesystem · the Node/Chromium install sites (`$RUNNER_TEMP/node`, `~/.local/viola-node/`,
  `~/.cache/viola-provision/e2e-web/`, `~/.cache/ms-playwright/`) + the uid-0 `--install-deps` → **apply the sites; the uid-0
  clause is bound to escalation E1**.
- A7 D-arch-resources · Environment variables · `NODE_PIN_*` workflow data lines, never read by viola / viola-harness → **apply**
  [1: lands in the existing "Test-harness only, never read by `viola`" list].
- A8 D-arch-resources · Project directory structure · `scripts/install-node.sh`, `scripts/npm-audit.sh`,
  `e2e-web/package-lock.json`, `stub/pipe.html`, `tests/pipe-reachability.spec.ts` → **apply** [1].
- A9 D-arch-decisions · Stack and Technologies · a test-side browser e2e row (pinned Node v24.21.0, `@playwright/test` 1.63.0,
  bundled Chromium, npm audit) → **apply** [1: architecture already fixes `e2e-web/` as test-side Node (:428, :511); not a
  reversal].
- A10 (dep A) · Stack CI/CD row · WSL provisioning adds Node + Chromium; the Linux leg runs `run --browser` → **apply; the
  `--install-deps` uid-0 clause is bound to E1**.
- A11 (dep A) · Directory structure · the `e2e-web/package.json` comment pins `@playwright/test` only (axe deferred) → **apply**
  [1].
- A12 (dep A) · Directory structure · the `wsl-provision.sh` comment gains Node, Chromium and `--install-deps` → **apply; the
  `--install-deps` word is bound to E1**.
- A13 (dep A) · CI/CD approach · setup steps: pinned Node, per-OS npm ci + Chromium → **apply** [1].
- A14 (dep A) · CI/CD approach · the `test` job browser steps, `junit-playwright.xml`, the gate requires playwright → **apply**
  [1].
- A15 (dep A) · CI/CD approach · supply-chain npm lockfile audit → **apply** [1].
- A16 (dep A) · CI/CD approach · `nightly.yml` three jobs (+ `npm-advisories`) → **apply** [1].
- A17 (dep A) · Directory structure · the `ci.yml` / `nightly.yml` tree comments → **apply** [1].

### security-plan
- S1 D-security-deps · Dependency Security → Audit tool · the npm audit + registry-only sources over the e2e-web lockfile →
  **apply** [1: severity escalate by detector default; the fact is the chunk's own, and the operator fork 3 of P4 settled "npm
  audit wired now, no exemption"].
- S2 D-security-deps · Pinning · the committed npm lockfile (`@playwright/test` =1.63.0) and the sha256-pinned Node → **apply**
  [1; fork 1 settled the pinned download].
- S3 (dep S) · Pinning (WSL2 bullet) · Node, Chromium, the `node` pin check, the constant PATH component; "never runs sudo"
  kept; root `--install-deps` → **the non-root half applies; the root half is escalation E1**.
- S4 (dep S) · CI integration, supply-chain bullet · the npm lockfile audit step → **apply** [1].
- S5 (dep S) · CI integration, nightly bullet · `npm-advisories` → **apply** [1].
- S6 (dep S) · CI integration, Toolchains · Node from a verified download, no action → **apply** [1].
- S7 (dep S) · Threat Model Summary → Supply chain → **reject** [1: playbook "Verbatim upstream copy (other masters)"; the fact
  lands in §Dependency Security via S1, S2 and S6].
- S8 (dep S) · Threat Model Summary → Infrastructure CI/CD → **reject** [1: same rule; lands via S4, S5 and S6].
- S9 (dep S) · Bootstrap `dep-audit-tooling-install` · `npm-audit.sh`, `install-node.sh` → **apply** [1].
- S10 (dep S) · Bootstrap `dep-security-ci-gate` · the npm audit step and its nightly twin → **apply** [1].
- S11 D-security-auth · Secret Management → Storage → Development · a third WSL launch form: a plan entry runs `wsl.exe -d Ubuntu
  -u root … env -i HOME=/root PATH=… wsl-provision.sh --install-deps <distro home>` as uid 0, running the user-provisioned node
  and Playwright CLI; a boundary widening → **ESCALATE (E1)** [1: playbook "Boundary widening" + "what ratifies it" — halt for
  the live answer; the wrap direction says the same].

### test-plan
- T1 D-tests-framework · §2 E2E row · Playwright 1.63.0 headless Chromium on all 3 OSes under pinned Node → **apply** [1].
- T2 (dep T-f) · §1 web-spa driver · 3 OSes; render/contrast stay ubuntu-judged → **apply** [1].
- T3 (dep T-f) · §1 multi-os-compat · the browser suite on all three runners → **apply** [1].
- T4 (dep T-f) · §6 Drivers web-spa row · headless (3 OSes); the axe pin deferred to Epoch 8 → **apply** [1].
- T5 (dep T-f) · §2 naming · `pipe-reachability.spec.ts` (`pipe:` titles) as the one non-layout spec → **apply** [1].
- T6 (dep T-f) · §3 `test-runner-install` Node side · `@playwright/test` 1.63.0 only + a strict noEmit tsconfig; axe deferred →
  **apply** [1].
- T7 (dep T-f) · §3 `ci-tool-install` · pinned Node + per-OS npm ci + Chromium; WSL same, system libraries via `--install-deps`
  uid 0 → **apply; the uid-0 clause is bound to E1**.
- T8 (dep T-f) · §3 Test selection · the one-browser-test form carries `--config e2e-web/playwright.config.ts` → **apply** [1 +
  check 6: disposes the report's entry-13 disproof].
- T9 (dep T-f) · §9 E2E row · `run --browser` in the per-OS `test` job; step order intact; the a11y lint placement kept →
  **apply** [1].
- T10 (dep T-f) · §9 Coverage report row · the per-OS gate `coverage,doctest,playwright` → **apply** [1].
- T11 (dep T-f) · §9 Matrix builds · Playwright no longer ubuntu-only → **apply** [1].
- T12 (dep T-f) · §9 tool-install paragraph · pinned Node + per-OS Chromium; WSL Node; `pre-push` `node` refusals → **apply**
  [1].
- T13 (dep T-f) · §9 Supply-chain row · the npm audit + sources step + the nightly twin → **apply** [1].
- T14 (dep T-f) · §2 V9 row · the npm audit + nightly npm-advisories → **apply** [1].
- T15 (dep T-f) · §9 Test report format · `junit-<os>` carries 2 paths → **apply** [1].
- T16 (dep T-f) · §12 Browser caching · runner-image Node → pinned download (no setup-node, no cache) → **apply** [1].
- T17 (dep T-f) · §12 initial entry · annotate the web-spa driver line "superseded" → **reject** [1, no rule, uneasy: a
  Decisions Log entry is history; T18's new dated entry records the supersession, so the old entry stays as written].
- T18 (dep T-f) · §12 · a new dated entry (W125) → **apply** [1 + §3 Closed enums "a new value needs a Decisions Log entry"].
- T19 D-tests-obs-harness · §3 `run` step 3 OS arms · every OS under `--browser` only; `browser-linux-only` retired; the arms
  named → **apply** [1].
- T20 (dep T-o) · §3 `run` step 3 invocation · delete stale reports; `npm ci` (`npm.cmd`), the Chromium probe, `node …/cli.js
  test`; no shell or npx → **apply** [1].
- T21 (dep T-o) · §3 `run` default · `--all` runs steps 1, 2, 4; browser only under `--browser` → **apply** [1; fork 2].
- T22 (dep T-o) · §1 harness run order · item 3 `--browser`-only → **apply** [1].
- T23 (dep T-o) · §3 `run` step 2 · the root bound 7 s (`WITHIN`, 8 waits), the streamed report → **apply** [1].
- T24 (dep T-o) · §3 `gate` CI placement · `coverage,doctest,playwright` in the per-OS test job → **apply** [1].
- T25 (dep T-o) · §3 `run` Output `archived` · the playwright JUnit joins → **apply** [1].
- T26 (dep T-o) · §3 `pre-push` env PATH · the constant `<home>/.local/viola-node/bin` → **apply** [1: security-plan already rules
  a distro-derived constant is not a widening].
- T27 (dep T-o) · §3 `pre-push` stages · `tools` checks node; `linux-tests` runs 3 commands → **apply** [1].
- T28 (dep T-o) · §3 `pre-push` document · `linux{run,browser,gate}` → **apply** [1].
- T29 (dep T-o) · §3 Closed enums · run reason `browser-missing` → **apply** [1].
- T30 (dep T-o) · §3 Closed enums · pre-push detail `node` → **apply** [1].
- T31 (dep T-o) · §11 CI ban · every OS; `--all` dropped → **apply** [1].

## Validate checks 2–6
- **2 Cross-contradiction:** none. The test-plan and arch CI/CD proposals state the same facts. The only shared-subject pair is
  S3 / A10 / T7 on `--install-deps`, and all three are bound to E1.
- **3 Intent-consistency:** the report matches the working-route entry and the plan's acceptance. The divergences are justified
  in the report (fork 2's `--all`; the operator-directed entry 13 and operator pass). Intent is unchanged.
- **4 Absence needs evidence:** the report's 0-hit claims (`npm audit` 0, `W125` 0) rest on the report's `sites.py` sweep. The
  cascade sweep re-reads every hit.
- **5 Expected-amendments reconciliation:** not proposed by any detector, so raised by the orchestrator:
  - O1 **a11y-plan** §3 CI integration / §9 / §11 CI / D-A11Y-12: the pipe runs on 3 OSes, and the a11y verdict stays
    ubuntu-bound. Also §3 a11y-tooling-install: the axe pin is deferred. Routine: the report substantiates it (Cross-project
    ci#36345175642; Dependencies: no axe).
  - O2 **design-system** §Typography / §Surface web-spa → Platform-Specific Notes (Fonts): render and contrast assertions stay
    judged on the ubuntu leg while the pipe runs on 3 OSes. Routine.
  - O3 **obs-plan** §9 artifact table (:1243), step order (:1287, :1288) and Test report format (:1750):
    `junit-playwright.xml` in `junit-<os>`, and the browser steps in the `test` job before the scan. Routine.
  - Every other plan entry is matched by a proposal: test-plan (T1–T31), security (S1–S6, S9, S10), arch (A1–A17).
- **6 Disproved claims:**
  - (a) the `install-deps --dry-run` exit: plan-only (0 master hits for `dry-run` in `wsl-provision` context) → routed to P3
    curation.
  - (b) entry 13 without `--config` → T8.
  - (c) and (d) confirmed hypotheses (unzip on windows-2025; `npm.cmd` spawn): no disposition owed.
- **Escalation E1 (HALT):** the root install through `wsl -u root` (S11; the root halves of S3, A6, A10, A12, T7).
