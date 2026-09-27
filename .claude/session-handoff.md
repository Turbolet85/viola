# Session Handoff

**Last Updated:** 2026-09-27T20:14:39Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the pre-CI commit `5f0a809` was already pushed)
**Status:** clean
**Last Commit:** 2026-09-27-browser-verdict-reachability — chore(2026-09-27-browser-verdict-reachability): the wrap commit (see `git log -1`)

## Position
- Done: 2026-09-27-browser-verdict-reachability (Epoch 2b). The browser pipe is proven end to end: a stub Playwright spec runs
  green on the three CI OSes and the WSL pre-push leg, and `gate … playwright` reads it. Operator pass: pre-CI commit `5f0a809`,
  CI ci#36345175642 15/15.
- Next: /andromeda-phase to promote + plan working-route:47 "Hooks to normalised events". Its P1 folds the `snapshot.json`
  replace fragility (CARRY: a retry on the sharing violation + a witness test holding the file open).

## Work done
- `run --browser` on every OS (not under `--all`), a pinned-download Node (`scripts/install-node.sh`, ci.yml `NODE_PIN_*`), the
  `e2e-web/` package + `file://` stub spec, the npm lockfile audit (`scripts/npm-audit.sh`, `supply-chain` + nightly), and the
  pre-push Linux leg running the browser suite.
- The 8 root test waits lowered to 7 s with a streamed `viola-root-watch` report (a recorder, not a fix).

## Drift resolved
- 62 amendments across architecture, security-plan, test-plan, a11y-plan, design-system (a new sidecar) and obs-plan. That is 56
  fan-out proposals (architecture 17, security 9, test 30), 3 raised at check 5, and 3 cascade folds; 3 were rejected (two Threat Model verbatim-copy sites, one
  history annotation).
- **E1, a boundary widening:** `wsl.exe -d Ubuntu -u root … wsl-provision.sh --install-deps`. The overseer ratified it live,
  under the founder's 2026-09-27 ruling, **operator-only** (never the gate tool, a harness command or a pre-push stage). It is
  recorded in the security-plan Decisions Log. A CARRY on :47 moves on until a re-provision: root must then run only
  `apt-get install` over an allowlisted dry-run list.

## Notes
- **Watches retired:** both CARRY 2 recurrence watches retire by their expiry, 3 consecutive green CI runs with no recurrence
  (ci#36333711860, ci#36340086338, ci#36345175642). The viola-pty recorder stays in the tests.
- **Owed to the founder:** H2, the product question (a key lost within ~50 ms of a ConPTY resize, rstudio/rstudio#18884), stays
  OPEN.
- **For the operator:**
  - the kept control home `target/e2e-home/viola-session-NETvJg`;
  - the 4 `%TEMP%/cargo-mutants-viola-*.tmp` dirs and `target/harness-check/`;
  - the `.wslconfig` memory cap;
  - `CARGO_BUILD_JOBS=16` kept.
- **Deferred learnings:** `recurrence-despite-learning: host-win32.md 2026-09-24 (a running .exe cannot be relinked) +
  verification-harness.md (a leaked process locks its .exe)`: an orphaned `viola-fake-agent.exe` from a red integration test
  locked `target/harness` and failed the boot smoke's build. Rejected at exactly 0.6: "a test recorder must read in the
  predicate's order" (now in test-plan §3) and "`install-deps --dry-run` exits 1 while packages are missing" (now in the :47
  CARRY).
- **Carried from earlier wraps:**
  - a `clean` guard's red half needs a stray clone-side file (0.8, cap);
  - a `cfg!()`-valued fn is an equivalent mutant on one OS's leg, so make it a const (0.8);
  - `check-runs` by sha mixes superseded runs after a force-push (0.8);
  - CARRY 2's third cargo-mutants fact and the `"777"` digit-substring sweep hazard (0.8).
  The code-metrics `mutation.survivors` correction is owed at the next ledger-mode audit.
- Last failed command: none.

## Session End Status
Completed normally at 2026-09-27 23:47:13
