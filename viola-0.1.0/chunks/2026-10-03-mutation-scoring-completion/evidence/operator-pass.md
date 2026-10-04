# Operator pass — 2026-10-03-mutation-scoring-completion

## What /implement read (step 10, the native gate)
`bash scripts/agent-run.sh pre-push` (gate entry 18), on the final tree after every kill test, read:

    {"v":1,"cmd":"pre-push","ok":true,"stage":"linux-tests","linux":{"run":{"ok":true,"suites":[{"suite":"coverage","passed":959,"failed":0,…},{"suite":"doctest",…}]},"browser":{"ok":true,"suites":[{"suite":"playwright","passed":1,"failed":0,…}]},"gate":{"ok":true,"breaches":[]}}}

- **Stages.** `tools` (`cc`, the `rust-toolchain.toml` channel, ci.yml's test-job tool pins, `NODE_PIN_VERSION`, each
  through the native `env -i` launcher) → `linux-tests` (`run --coverage` → `run --browser` →
  `gate --require coverage,doctest,playwright`).
- **Timing.** 32.7 s on warm caches. The first gate pass, before the scoring kill tests, read the same shape with
  coverage 951 / 0.
- **The document** carries codes and counts only: no home, no repository path, no artifact path.

From now on this is the operator pass's gate (plan step 10): the operator re-runs it on the uncommitted tree before the
pre-CI commit, and a red stops the pass.

## The operator's entries (not fired by /implement)
| gate | entry | state |
|---|---|---|
| 22 | `gate.py hygiene` | owed to the operator, before the pre-CI commit |
| 23 | `git push origin HEAD` (pre-CI commit, then each fix commit) | owed to the operator |
| 24 | `ci.py conclusion --sha HEAD --wait 1800` | owed to the operator. Its read must name the final HEAD's run id and show `test (macos-latest)` green: item 7's witness for the `NotConnected` close-race arm (step 3) |

Results go below as the operator fires each one.

## Results (fired on the overseer's word, founder-delegated, 2026-10-04)
- **Entry 22, `gate.py hygiene`:** `hygiene: clean — read 34 (runs 29 · evidence 5) · trails 11 not read · binary 0 not
  read by P1`, exit 0. Every P1/P2/P3 control fired on its synthetic known positive. Read on the uncommitted tree,
  before the pre-CI commit.
