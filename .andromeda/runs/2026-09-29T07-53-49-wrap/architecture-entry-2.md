
## 2026-09-29-verify-stamped-test-homes-and-harness — verify's spawn pairs, the harness CI read, verify-stamped homes and the ninth root wait
**Section:** Occupied Resources → Environment variables (test-harness only); Occupied Resources → Filesystem (`diagnostics/` `cli-<name>.ndjson`; `viola-root-watch` Watch report); Occupied Resources → Repository (`target/e2e-home/…`)
**Change:**
- `cli-<name>.ndjson`: verify's self pair now holds two spawn pairs, `version-probe` (the `--version` read) then `verify-probe` (the print-mode probe), each `process-start{subject}` / `process-exit{subject, child_exit_status, duration_ms}` from verify's call-site wrapper; `run_bounded` stays unlogged, so `run`'s gate keeps one `version-probe` pair.
- `CI` registered as a test-harness-only read: `viola-harness` (bin and `run::run`) reads it for presence and passes it to `run_with` as `ci`; `run --local-live` under it is `{"v":1,"cmd":"run","ok":false,"reason":"live-in-ci"}`, exit 2, before any build, spawn or suite. `viola` never reads it.
- Repository: harness session homes are stamped at `boot` step 4 by `viola verify` against the fake agent (unless `--unstamped`), root rstest homes by `stamped_home` at the recorded 2.1.283, and `target/e2e-home/viola-live-<pid>/home` is the one real-`claude` verify home of the local-only `run --local-live`.
- The Watch report's bound is used by 9 root waits on a child (was 8): the ninth, `wait_endpoint_gone`, runs in `Wrapper::stop` / `stop_keep` after the exit until the recorded endpoint is unconnectable.
**Why:** each is a resource or contract shape this chunk landed; stamps come only from `viola verify`, and the real CLI never runs in CI.
**Ref:** .andromeda/runs/2026-09-29T07-53-49-wrap/
