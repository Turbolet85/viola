# Unix-only witness (plan step 7, overseer direction 6) — implement, 2026-09-27

One-shot, `evidence/witness.py`; reading in `witness-reading.json`, logs `witness-host.log` / `witness-pre-push.log`.

- **Plant:** `#[cfg(unix)] #[test] fn pre_push_planted_unix_red() { panic!("planted unix red") }` in
  `crates/viola-e2e/src/harness/pre_push.rs`'s test module — in the Windows working tree only, never committed.
- **This host cannot see it:** `bash scripts/agent-run.sh run --unit --filter 'test(/pre_push_planted_unix_red/)'`
  → exit 1, `nextest-unit` failures `["nextest-exit-4"]` (the test is compiled out on Windows; nothing selected).
- **The gate catches it before the push:** `bash scripts/agent-run.sh pre-push` → exit 1, `ok:false`,
  `stage:"linux-tests"`, failures `["viola-e2e harness::pre_push::tests::pre_push_planted_unix_red"]`, no leg run.
  Its `sync` (`files` 161, tree `99cc2045…`) carried the uncommitted plant to Linux — the dirty-file proof on the
  gate's own path.
- **Removed:** the original bytes restored and asserted identical. The remove-the-plant green readings are the three
  consecutive green `pre-push` runs over the same tree (`linux-red-investigation.md` §Fixed).
