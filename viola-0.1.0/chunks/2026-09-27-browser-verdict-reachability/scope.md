# Scope — 2026-09-27-browser-verdict-reachability

**Working entry:** `viola-0.1.0/working-route.md:45` — "Browser verdict reachability — stub page, Playwright on three CI OSes and
the WSL pre-push, pipe to a recorded green verdict; capability reachable, not real assertions" (Epoch 2b — Windows slice I b:
events and ledger). Inserted by founder ruling W125 (2026-09-27, relayed by the overseer as relay item B).

**Intent (the val-1 anchor):** the browser test PIPE is proven end to end before any feature needs it. A stub spec runs
under Playwright on every CI OS and on the WSL pre-push leg, and its outcome travels through the harness into a recorded
`playwright` suite that the `gate` verb reads green. What is proven is that the capability is reachable. The real assertions are
out of scope.

## What this chunk builds

1. **The `e2e-web/` Node package, the pipe's minimum.** A tracked `e2e-web/package.json` pinning `@playwright/test@1.63.0` (the
   test-plan pin), a committed npm lockfile, `e2e-web/playwright.config.ts` with the JSON (`pw.json`) and JUnit (`pw-junit.xml`)
   reporters, headless Chromium, and a tracked `e2e-web/tsconfig.json`. One stub spec (`e2e-web/tests/*.spec.ts`) drives a
   stub page to one trivially true assertion. `@axe-core/playwright` is NOT pinned here: it belongs to the a11y harness entry
   (Epoch 8), which builds on this pipe. Verified at P3: architecture's tree stages the landing ("they land with the Web UI
   chunks"). a11y-plan §3 a11y-tooling-install's "already declared by tests" becomes false, so it is carried as a route note and a
   wrap amendment.
2. **The stub page.** `viola ui` does not exist yet (Epoch 8), so the page is a static stub the spec reaches without a viola
   process. [premise-corrected: of the three serving options, test-plan bans two. A Playwright `route` fulfilment breaks §3
   Browser-side controls ("`route.fulfill` and `route.abort` stay banned"). A fixture server breaks §8 ("no HTTP fake servers")
   and §11 ("The only socket allowed is 127.0.0.1 `viola ui`"). So the page is a `file://` stub under `e2e-web/`: no listener,
   no `webServer`, and the Epoch 8 `boot`-starts-`viola ui` shape stays open.]
3. **The harness pipe.** `viola-harness run --browser` (test-plan §3 `run` step 3): `npm ci --prefix e2e-web`, then
   `npx --prefix e2e-web playwright test`. The `playwright` suite in `run-summary.json` is built from `e2e-web/pw.json`,
   `e2e-web/pw-junit.xml` is copied as `junit-playwright.xml`, and `gate --require playwright` passes. Verified at P3:
   - Today `run --browser` is a clap `arguments` usage error, exit 2 (`viola-harness.rs:206–217`).
   - `gate.rs` already carries `playwright` in `SUITES` and `JUNIT` (`gate.rs:15–34`), so the gate needs no product change.
   - The arm, the flag and the archive entry are new.
4. **CI on three OSes.** The entry says "Playwright on three CI OSes". Verified at P3: the conflicting clauses span four plans:
   - test-plan §3 `run` step 3 (`browser-linux-only`), the §6 Drivers table, the §9 E2E row / Matrix builds, §12, §11 and §2;
   - a11y-plan §3, §9, §11 CI and D-A11Y-12;
   - design-system's "Linux is the CI render", which must survive the change;
   - architecture's CI/CD job list.

   The founder ruling is the newer authority, and each clause is a wrap amendment, not a phase edit. The step joins the existing
   3-OS `test` job (`ci.yml:18`), which keeps 8 jobs and 15 check-runs.
5. **The WSL pre-push leg.** The same browser step runs in the Linux pre-push leg (`viola-harness pre-push`). One set of
   Playwright/Node pins serves the CI legs and the WSL leg. `scripts/wsl-provision.sh` installs only CI's own pins, so the Node
   and Playwright versions and the Chromium install go into provisioning as CI's pins, never host values (security.md: the WSL
   distro installs only CI's pins, every call under `env -i`). Verified at P3 by measurement: the distro is Ubuntu 26.04
   (Playwright-supported), and under the leg's `env -i` PATH there is no `node`, no `npm` and no `~/.cache/ms-playwright`.
   Chromium's system libraries are root-only, so they follow the `build-essential` precedent (an install line printed, never
   sudo). [premise-corrected: "CI's own pins" presumes CI pins Node. It does not: CI uses runner-image Node (ubuntu and windows
   22.23.2, macOS 24.20.0, per the fetched runner-images readmes). The Node pin source is a P4 fork.]
6. **The side effects the entry names (state today, read at P1: no `e2e-web/`, no `package.json`, no `tsconfig.json`, no
   Playwright token in `ci.yml` or the scripts).**
   - The tracked `tsconfig.json` makes the code-graph TypeScript plane detected. Verified at P3: `scripts/code-graph.py`
     `detect_planes` (55–68) picks up a tracked `tsconfig.json`, and a missing indexer skips the plane with a recipe.
     `scip-typescript` is on this host, so the ts plane builds here, and every later graph query needs its plane argument.
   - The committed npm lockfile gives a supply-chain gate something to read. Verified at P3 as an open choice. security-plan
     admits a non-root lockfile only with its own audit, or as a ratified test-only exemption; the exemption needs the founder's
     live word (the wrapper-channel entry). The `fuzz/Cargo.lock` audit is the template. This is a P4 fork. The security.md rule
     "never widen an ignore" applies either way.

## Boundaries (out of scope)
- Real assertions, the bay specs (`bay-steady-state.spec.ts` …), axe, the a11y fixture and rows, the `a11y-violation` schema, the
  a11y verdict gate: these are Epoch 8 ("Web test toolchain and a11y harness", "A11y verdict across page states"), and they build
  on this pipe.
- `viola ui` and the Lit page.
- Harness `boot` sessions for browser tests (the per-test session fixture).

## Folded freight

### CARRY 1 (W125, the entry's own): see "What this chunk builds". The positioning basis is the founder's, relayed: those gates
sit ~44 of 50 chunks after base CI, the shape that cost another project ~588 runner-minutes in 11 days.

### CARRY 2: the two recurrence watches (from 2026-09-27-epoch-2-cleanup)
- `viola-pty tests::spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code` stays OPEN. It recurred once
  (2026-09-27 16:02:58Z, the operator pre-push windows leg's cargo-mutants unmutated baseline, `TIMEOUT [10.107s]`), with no
  capture. The recorder is in place (`CHILD_WITHIN` 7 s; report at `<temp dir>/viola-pty-watch/<test name>.report`). Match any
  recurrence to this name; a red with a captured report re-opens the cause. A green re-run never closes it.
- `viola-e2e::harness_lifecycle harness_session_boots_reports_logs_and_tears_down`: no recurrence.
- Expiry: both watches retire after 3 consecutive green CI runs with no recurrence of either name. The count is 1 of 3 so far
  (ci#36333711860 on `f0e6dbc`). This chunk records each CI run it reads against the count.
- H2 (a key lost within ~50 ms of a ConPTY resize, rstudio/rstudio#18884) is owed to the operator. It is not this chunk's work.

### Overseer direction at P1: the 8 root-package tests with a 10 s child wait (NOT a CARRY)
Each waits on a child with a 10 s bound, equal to the nextest `mutants` profile's kill (`slow-timeout` period 5 s × terminate-after
2 = 10 s). The pty-watch remove-the-guard pair measured the race (`2026-09-27-epoch-2-cleanup/evidence/pty-watch-recorder.md`):
7 s = the test fails itself and its dump survives; 10 s = a race; 12 s = killed, report lost. **Direction, as corrected by the
overseer:** LOWER each bound below the kill (7 s, as the viola-pty recorder did) and stream the child report to a known file.
Never raise a bound. (The session-start dashboard said "raise"; that wording was wrong and is corrected here.)

Coordinates, re-verified on disk at P1 (all read `from_secs(10)`):
- `tests/support/fake.rs:12` `WAIT_WITHIN`
- `tests/support/home.rs:19` `READY_WITHIN`
- `tests/support/outer_pty.rs:15` `EXIT_WITHIN` (pub; also used by `home.rs:270` and `run_cli.rs:70`)
- `tests/run_cli.rs:19` `READY_WITHIN`
- `tests/run_cli.rs:262` (an inline `Duration::from_secs(10)`)
- `tests/cli_instance_state.rs:220` (an inline `Duration::from_secs(10)`, the `-STOP` wait)
- `tests/contract_diag_schema.rs:244` and `:264` (inline, the raw-terminal wait and the exit wait)

"Stream the child report" per test means the evidence at the moment of a stuck wait: what the test was polling. It survives a
runner kill in a known file under `<temp dir>/viola-root-watch/<test name>.report` (or the pty recorder's dir; P4 picks), is kept
on failure or kill, and is removed by a passing test. It is also dumped in the test's own panic message at the 7 s bound.
Behaviour and assertions are unchanged. This is a recorder, not a fix.

Verified at P3. The count of 8 was re-derived by `grep -rnE 'from_secs\((1[0-9]|[2-9][0-9])\)'` over `tests`: 9 hits, one an
mtime back-date. What each loop polls:
- the fake-agent receipt (`fake.rs` `wait_for`);
- `Starts` + snapshot + heartbeat + `try_wait` (`home.rs` `wait_ready`);
- `try_wait` (`outer_pty.rs` `wait_exit`, also `finish` and `Drop`);
- the receipt `start` (`run_cli.rs` `wait_raw`);
- `run-builder.ndjson` `claude-child` (`run_cli.rs:262`);
- `ps -o stat=` (`cli_instance_state.rs:220`);
- the raw receipt and `try_wait` (`contract_diag_schema.rs:244`/`:264`).

Reports stay outside `target/e2e-home/**/diagnostics/` (G4, obs extract).

## CI verdict read at Setup 5a (the second fold source)
- Base: the last master flip `49f644472a11` (the epoch-2-cleanup wrap commit). Shas read: `49f644472a11` only.
- `49f644472a11`: **CI 49f644472a11: verdict not yet available.** ci#36340086338 was `in_progress` at 18:21Z (checks 15/15 listed,
  the oldest running `lint (windows-2025)` at 233 s). Wall-clock was not yet measurable, so it was neither folded nor read as
  green. The operator's CI read for this chunk uses the `ci.py conclusion` tool form (overseer directive 4).
- Re-read at P3 (the runner-only witness, the same run): `49f644472a11 verdict: green · checks 15/15 · wall 235 s · runs
  ci#36340086338 completed/success`. Nothing is owed to fold. CARRY 2's expiry count is now **2 of 3** (ci#36333711860 on
  `f0e6dbc`, then ci#36340086338 on `49f6444`), with no recurrence of either watched name.
