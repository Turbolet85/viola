
## 2026-09-29-verify-stamped-test-homes-and-harness — fake-agent default 2.1.283; perf session stamped; H2 owner re-named
**Section:** §7 Test Data & Fixtures → Fake agent (Modes); §10 Quality Gates → Perf session; §5 Integration Test Strategy → Module ↔ PTY row (H2)
**Change:**
- Fake agent `DEFAULT_CLI_VERSION` is 2.1.283, the recorded set's version (was "stays 2.1.0"); its default `--version` answer is `2.1.283 (Claude Code)`, and the harness `boot --cli-version` default is the same.
- The `--perf` session boots with `BootOptions { …, stamp: true }` and is stamped at boot step 4 (was "No unstamped session exists: nothing is stamped yet").
- The Module ↔ PTY row's H2 owner is the working-route entry "First live test and self-drive" (was "the real-CLI verify entry"); `run --local-live` does not claim it.
**Why:** the fake agent answers at the version its fixtures were recorded at, so the wrapper's version gate reads a stamped version; the H2 owner wording matches architecture [PTY].
**Ref:** .andromeda/runs/2026-09-29T07-53-49-wrap/
