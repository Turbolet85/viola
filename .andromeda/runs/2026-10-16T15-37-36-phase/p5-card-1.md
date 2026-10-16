# P5 review — 2026-10-16-remove-the-browser-leg · Remove the browser leg

**Plan:** `viola-0.1.0/chunks/2026-10-16-remove-the-browser-leg/plan.md`. It has 16 steps and 25 gate entries
(8 seen green, 14 new with baselines, 3 operator), and 14 acceptance criteria. The base is `b6bbbd68428a`, with CI
green (`ci#38925218343`). The mechanical checks pass: 0 FAIL, 0 REQUIRED-RESOLUTION open. Validation-1 is aligned.

## Goal
The browser leg leaves 0.1.0:
- `e2e-web/`, Node, npm and Playwright come out of CI, the native pre-push and the harness;
- `install-node.sh` and `npm-audit.sh` are deleted;
- no gate runs, installs or checks `node` / `npm` / `npx`.

The GUI harness leftovers and the reserved web half of the obs vocabulary leave with it. Product behaviour is
unchanged (a preservation guard reads the product paths).

## What changes
- **CI:**
  - `ci.yml` loses the Node pin env, the browser pipe and the npm audit steps. The `test` job gates
    `coverage,doctest`, and `junit-<os>` carries the nextest JUnit alone.
  - `nightly.yml` loses `npm-advisories`.
  - `windows-mutants.yml` drops `browser.rs` and keeps ten items.
- **Harness:**
  - `run --browser` and `run/browser.rs` are gone.
  - `playwright` leaves `gate`'s suites (9 → 8), so `--require playwright` is usage `unknown-suite`.
  - pre-push: no `node` check, no browser stage, and the PATH loses `~/.local/viola-node/bin`. It stays `env -i`
    with HOME and PATH only.
- **Leftovers that go:**
  - status `ui` / `api_sessions_equal_list`;
  - cleanup `port_free` / `url_file_removed`;
  - `SessionRecord.ui` / `cookie_file`;
  - `ui-restart` in both shims;
  - nextest `fixed-port`;
  - the GUI scan forms `?t=`, `ui/*.url` token and `viola_<port>=` (fork 1). The `cookie:` header, every `CLAUDE*`
    canary and the content canary stay;
  - the axum `http2` ban with its probe, so the probes go 13 → 12 (fork 2).
- **Obs:**
  - `ObsEvent` 19 → 15 (`http-request`, `liveness-changed`, `sse-opened`, `sse-closed` leave);
  - `ObsProcess` 5 → 4 (`ui` leaves);
  - `VIOLA_TARGETS` loses `viola_ui`;
  - `role_file_name` / `open_role_file` lose the `port` parameter;
  - both schemas narrow to match, with `port`, `last-event-id` and `$defs.liveness` removed and
    `unevaluatedProperties: false` kept.

## Top acceptance criteria
- The browser-and-Node absence guard and the leftover-vocabulary guard read red at the base (145 and 74 hits, every
  one a line the steps remove) and green after.
- `gate --require playwright` exits 2 `unknown-suite`.
- `deny-probes: 12/12 banned, control clean`. `cargo deny check` and the sole-root tokio loop pass, and zizmor is
  clean.
- The pre-push launcher still crosses HOME and PATH only, and the canary pair still reads 0 / 1.
- G4 and `contract_diag_schema` read the narrowed schemas with zero failures. The booted smoke shows the new status
  and cleanup shapes.
- The CLI output-discipline and TUI boundary cases run unchanged in the gated `coverage` suite, and CI's final run
  reads green.

## Forks taken (P4, `inputs#I1`)
Both were answered by the operator as the overseer's word (technical, founder-delegated, 2026-10-16):
1. Every GUI secret-scan form leaves.
2. The axum `http2` ban leaves with its probe.

Leaned without asking (decisive per artifact): the obs web half, the null harness keys, `SessionRecord`'s GUI fields,
`ui-restart` and `fixed-port`.

## Capabilities
- None claimed: the entry names no matrix id.
- v1-08 (the ts plane) and v1-23 (`13/13`) are `verified`. Each gets a dated ledger note at the wrap, listed under
  the plan's Expected amendments.
- No concretization preview.

## Expected amendments (wrap)
All seven masters but layout-templates follow the code: architecture, security-plan, test-plan, obs-plan,
a11y-plan and design-system §Typography, each section named in the plan. a11y-plan's tier `Standard (1)` is not
touched. CLAUDE.md and the `.claude/` leaves follow by the cascade.

## Beyond the plan
none
