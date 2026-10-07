# The keepers' remove-the-guard readings (step 3)

One-shot runs on the Linux dev host, 2026-10-07, between 07:42Z and 07:47Z, each through the harness
(`bash scripts/agent-run.sh run …`), its document to a scratch file. Each neutralising edit was confirmed in the
file by a `grep` before its run, and each restore the same way; after the last restore no neutralised line is left
(`grep -c` of the two neutralising spellings reads 0 in all three files) and `cargo fmt --all --check` exits 0. No
net change.

## Where the guard was removed
The plan names the call site. The cases call the keeper itself, each over its own temp dir, so a swap at the call
site (`TestHome::new`, `boot`'s `start`) cannot reach them. The guard was removed inside each keeper instead: its
link arm switched off, which leaves exactly the bare `create_dir_all` the keeper replaced.

## Pair 1: the root keeper (`prepare_home_base`, `tests/support/home.rs`)
Command: `run --integration --filter 'test(/home_base_backing_gone_target_is_made_again_owner_only/)'`.

| reading | link arm | exit | `nextest-integration` | archive |
|---|---|---|---|---|
| red | off | 1 | 0 passed, 1 failed | `target/run-archive/651` |
| green | restored | 0 | 1 passed, 0 failed | `target/run-archive/652` |

The red's failure line: `panicked at tests/support/home.rs:136:30: e2e-home: Os { code: 17, kind: AlreadyExists,
message: "File exists" }`. Rust's `create_dir_all` over a link whose target is gone takes the already-exists arm,
as research measured through python.

## Pair 2: the harness keeper (`Workspace::ensure_e2e_home`, `crates/viola-e2e/src/harness/mod.rs`)
Command: `run --unit --filter 'test(/e2e_home_backing_gone_target_is_made_again_owner_only/)'`.

| reading | link arm | exit | `nextest-unit` | archive |
|---|---|---|---|---|
| red | off | 1 | 0 passed, 1 failed | `target/run-archive/653` |
| green | restored | 0 | 1 passed, 0 failed | `target/run-archive/654` |

The red's failure line: `panicked at crates/viola-e2e/src/harness/mod.rs:398:30: made again: Os { code: 17, kind:
AlreadyExists, message: "File exists" }`.

## Pair 3: the three refusals and the `local_live` call (beyond the plan's two pairs)
`.claude/rules/testing.md` (2026-09-25) asks a remove-the-guard run of every new guard test, so the refusal cases
and the `local_live` case were read too, in one run per keeper: the relative-target, real-directory and owner-only
checks switched off in both keepers, and the keeper call in `local_live` switched off.

| run | exit | counts | red cases | archive |
|---|---|---|---|---|
| `run --integration --filter 'test(/home_base_backing_/)'`, checks off | 1 | 3 passed, 3 failed | `…_relative_target_is_refused`, `…_group_or_other_bit_is_refused`, `…_target_that_is_a_link_is_refused` | `target/run-archive/655` |
| `run --unit --filter 'test(/e2e_home_backing_/)'`, checks and call off | 1 | 3 passed, 4 failed | the same three, and `e2e_home_backing_local_live_refused_base_fails_before_the_verify` | `target/run-archive/656` |
| `run --integration --filter 'test(/home_base_backing_/)'`, restored | 0 | 6 passed, 0 failed | none | `target/run-archive/657` |
| `run --unit --filter 'test(/e2e_home_backing_/)'`, restored | 0 | 7 passed, 0 failed | none | `target/run-archive/658` |

With the relative-target check off, each keeper made an empty directory named `backing` under the test's working
directory (the repository root, and `crates/viola-e2e/`): the relative target resolved there. Both were empty, mode
0700, and were removed with `rmdir` before the restore.
