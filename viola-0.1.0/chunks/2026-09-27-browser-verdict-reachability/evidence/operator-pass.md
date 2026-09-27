# Operator pass — 2026-09-27 (run by the session on the overseer's word, founder-delegated)

Order directed by the overseer:
1. Fix plan entry 13 to run Playwright under the e2e-web config, and confirm no root `test-results/` is left.
2. Entry 44 `pre-push` on the uncommitted tree (a red stops the pass).
3. The pre-CI commit.
4. Entry 45's guarded fast-forward push (never forced).
5. Entry 46's `ci.py conclusion`. A CI red records its red-to-act time.
6. Stop and report.

## Before the pass: entry 13 (an operator-directed plan edit)

- Entry 13's `run` became `npx --no --prefix e2e-web playwright test --config e2e-web/playwright.config.ts --grep 'pipe:'`, and its
  `note` names why. From the repo root without `--config`, Playwright ran on its defaults: no `chromium` project, no reporters, and
  a root `test-results/.last-run.json`. That file was this run's own residue, and it was removed before the pass.
- `gate.py run --only 13`: green, exit 0, 1.91 s. It printed `[1/1] [chromium] › e2e-web\tests\pipe-reachability.spec.ts:7:5 › pipe:
  the stub page shows its heading`, `1 passed`. The `[chromium]` tag is the config's project.
- `ls -d test-results` at the repo root: `No such file or directory`. Its outputs (`e2e-web/test-results/`, `pw.json`,
  `pw-junit.xml`) are all git-ignored (`git status --ignored`: `!!`).

## Entry 44 — `bash scripts/agent-run.sh pre-push` (uncommitted tree) — GREEN

- 2026-09-27T19:31:14Z → 19:39:04Z (470 s), exit 0. Stdout was redirected to a file, never piped. `op-44.{out,start,end,rc}` are in
  the implement run dir (path-free: 1 line, no drive, home or mount path).
- `ok:true`, `stage:"union"`, gate `ok:true`, breaches `[]`:
  - sync: 62 files, tree `7a8bafa55bdf0681493aee88596ca0fea977e830` on HEAD `49f6444`;
  - Linux: `coverage` 628/628, **`browser`: `playwright` passed 1, failed 0, skipped 0**, and a gate (`coverage,doctest,playwright`)
    with `ok:true` and breaches `[]`;
  - windows-tests: `coverage` 640/640, gate ok;
  - `ubuntu-latest`: base `49f6444`, `counted`, 39 tested; `windows-2025`: base `49f6444`, `counted`, 39 tested; union: 0 breaches.
- `cache`: bytes 13 039 201 649 of a 42 949 672 960 cap, not cleaned. `scratch_bytes` 20 047 393 → 19 100 223;
  `windows_scratch_bytes` 43 017 098 → 24 804 000.

## Pre-CI commit, entry 45 (push), entry 46 (CI read)

- Pre-CI commit **`5f0a809`**: `chore(2026-09-27-browser-verdict-reachability): operator pre-CI commit, for the run this chunk's
  verdict reads` (`git add -A`, 67 files, 8002+/101−). The tree was clean after it.
- Entry 45: `git diff --quiet && git diff --cached --quiet && git push origin build/viola-0.1.0` → exit 0 at 2026-09-27T19:39:42Z,
  a fast-forward `49f6444..5f0a809`. Never forced. HEAD equals `origin/build/viola-0.1.0` (`5f0a8095ec12…`).
- Entry 46: `ci.py conclusion --sha HEAD --wait 5400`, 19:39:50Z → 19:49:09Z, exit 0:
  `5f0a8095ec12 verdict: green · checks 15/15 · wall 554 s · runs ci#36345175642 completed/success` (19 polls over 559 s).
- The browser pipe on the runners (`gh run view 36345175642 --json jobs`, the steps' conclusions):

  | job | Node (pinned) | npm ci | Chromium | Browser suite | Upload JUnit | Gate verdict (`coverage,doctest,playwright`) |
  |---|---|---|---|---|---|---|
  | test (windows-2025) | success | success | success (`install chromium`) | success (pwsh shim) | success | success |
  | test (macos-latest) | success | success | success (`install chromium`) | success (sh shim) | success | success |
  | test (ubuntu-latest) | success | success | success (`--with-deps`) | success (sh shim) | success | success |
  | supply-chain | success | — | — | npm lockfile audit (advisories, sources): success | — | — |

  - The plan's HYPOTHESIS that `unzip` exists in the windows-2025 runner's bash is **measured true**: `Node (pinned)` extracts the
    `win-x64` zip with `unzip -q`, and it succeeded on that leg. No `Expand-Archive` fallback is needed.
  - Not measured by this run: the `nightly.yml` `npm-advisories` job, which runs only on its weekly schedule or
    `workflow_dispatch`.
- CI red-to-first-action: **not applicable** (no CI red).
- CARRY 2 (the pty and harness-lifecycle watches): ci#36345175642 is the **3rd consecutive green CI run** with no recurrence of
  either watched name, after ci#36333711860 (`f0e6dbc`) and ci#36340086338 (`49f6444`). No local run of this chunk recurred either:
  every nextest and pre-push run here was green. The expiry count reads **3 of 3**. The retirement is the wrap's to record.
