# CARRY 5 — `remove_session_home_removes_a_boot_created_parent` over a dangling `target/e2e-home`

Plan step 6's witness, taken by /andromeda-implement on 2026-10-17 on the Linux dev host. It mutates nothing in the
tree: the subject is a scratch workspace exported from HEAD `6042af635c3e` with `git archive`, under the session
scratchpad (`$TMPDIR/claude-1000/…/scratchpad/carry5/ws`), built in its own target dir `target/carry5-witness`.

**Setup.** The scratch workspace's `target/e2e-home` is a link to an absolute directory that does not exist
(`…/scratchpad/carry5/backing-gone`), the state a host reboot leaves when the link's tmpfs target is gone. `test -e`
on the link read 1 (dangling) before each run.

**Command (both readings).** `cargo nextest run -p viola-e2e --lib --no-fail-fast -E
'test(/(^|::)tests::remove_session_home_removes_a_boot_created_parent$/)'`, run from the scratch workspace with
`CARGO_TARGET_DIR=<repo>/target/carry5-witness`.

## Red before (HEAD's case, `fs::create_dir_all(ws.e2e_home())`), 2026-10-17T01:32Z

```
        FAIL [   0.005s] (1/1) viola-e2e harness::cleanup::tests::remove_session_home_removes_a_boot_created_parent
    thread 'harness::cleanup::tests::remove_session_home_removes_a_boot_created_parent' (66280) panicked at crates/viola-e2e/src/harness/cleanup.rs:362:43:
    mkdir: Os { code: 17, kind: AlreadyExists, message: "File exists" }
     Summary [   0.006s] 1 test run: 0 passed, 1 failed, 228 skipped
```

Exit 100.

## Green after (the case through `ws.ensure_e2e_home()`, the one-line fix this chunk lands), 2026-10-17T01:32:25Z

The same line replaced in the scratch copy alone (`ws.ensure_e2e_home().expect("e2e home");`), the link still
dangling:

```
        PASS [   0.004s] (1/1) viola-e2e harness::cleanup::tests::remove_session_home_removes_a_boot_created_parent
     Summary [   0.005s] 1 test run: 1 passed, 228 skipped
```

Exit 0. After the run the link's target exists as `drwx------` (the keeper made it again, owner-only).
