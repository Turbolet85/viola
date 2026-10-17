
## 2026-10-17-end-to-end-path-binary-and-harness-legs — tests: run --e2e, the E2E regex widened, the integration complement
**Section:** §2 test locations; §3 Exit codes; §3 → 5-command implementation (`run` steps 1 and 2, Test selection); §3 → Bootstrap phases (derive for route / setup-project) (`[profile.ci]` verify-driven override)
**Change:**
- `run --e2e` is built: suite `nextest-e2e`, run third after unit and integration, in the default and `--all`, through the same nextest shape and target dir, a `--filter` intersecting it; alone it runs no doctest; the run-archive list stays unit, integration, coverage (was: a usage error until the first E2E binary).
- The E2E regex is `^(path[0-9]*|tui|mcp|cross|chaos|contract)_` (was `^(path|…)_`, which does not select a numbered Path binary): 20 binaries / 157 tests at this chunk's end. The integration layer is `kind(test) & !binary(…)`, its complement (was `kind(test)` until the first E2E-prefixed binary). An unmatched `binary()` regex is still a parse error, re-measured on 0.9.146 (0.9.133 kept as the first reading).
- §2: the viola-e2e form is `tests/{path<N>,mcp,cross}_<topic>.rs`; the root package carries numbered Path binaries too (`tests/path1_run_start.rs`); both report in the E2E layer.
- §3 Exit codes: `run --e2e` leaves the not-built example; the rule stands.
- The verify-driven 20 s override names twenty binaries (was eighteen): `path1_run_start` joined; `cross_list_rows`, already on the line, was missing from the list.
**Why:** The plan's binary names did not match the written regex; the widening is the Viola overseer's technical answer, founder-delegated, PROVISIONAL until the founder's Epoch 5 batch. Numbered Path binaries name the test plan's numbered Paths, and the widened pattern matches no other binary.
**Kept:** The "unbuilt selector is a usage error" rule, without its example.
**Ref:** .andromeda/runs/2026-10-17T01-57-16-wrap/
