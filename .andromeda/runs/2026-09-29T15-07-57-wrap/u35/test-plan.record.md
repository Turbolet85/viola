## 2026-09-29-t15-07-57-wrap — registry migration (U35): the test-plan Decisions Log leaves the body
**Section:** §12 Test Decisions Log · §2 Test Strategy · §3 → 5-command implementation (its key file) · §4 Unit Test Strategy · §5 Integration Test Strategy · §9 CI Integration · §10 Quality Gates & Coverage Targets
**Change:** the log moved verbatim to test-plan-amendments-archive.md (20 entries); 8 lifts:
- §2: why `crates/viola-e2e` is a test-only crate (tokio clients kept off the root and sync crates; never a tokio-ban root, no `wrappers` allowlist, no `--exclude`).
- §3 → 5-command implementation `boot` (hand-landed in its key file after the migration): the UI port race — `ui-port-taken`, never retried; `viola ui --port 0` requested from arch.
- §3 → 5-command implementation `pre-push` (the same): the operator pass runs it on the uncommitted tree before the pre-CI commit; a red stops the pass.
- §4: what the token-compare source-scan test asserts (compared only inside the single compare function).
- §5: the piped driver answers DA1 `ESC[c` with `ESC[?1;0c`; viola sends no DA1 answer.
- §9: caches — rust-cache only; npm and Playwright browsers uncached, and why.
- §9: no `concurrency:` block in `ci.yml` / `nightly.yml`; zizmor `concurrency-limits` declined (2 low).
- §10: why criterion 0.8.2 is not a gate; hyperfine is the perf gate.
**Why:** a Decisions Log is keyed by time — history, not current truth; its in-force items now stand in the body
**Ref:** .andromeda/runs/2026-09-29T15-07-57-wrap/
