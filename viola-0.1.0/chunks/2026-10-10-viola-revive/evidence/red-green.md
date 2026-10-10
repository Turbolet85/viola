# Red and green readings — 2026-10-10-viola-revive

Two one-shot controls of plan step 11, taken at /implement on the Linux dev host, tree = HEAD `00c73fd` plus
this chunk's uncommitted edits. Each run is the harness command shown, its one JSON document read whole. Every
neutralising edit was read back at its site before its run, and its removal was read back before the green run
(the count of the neutralising text in the file is 0).

## Control A — the resume arguments (plan step 7)

Site: `src/cmd/revive.rs`, `preflight`, the line `let mut child_args = resume_args(&id, args.fork);`.
Neutralised by one line after it, `child_args.clear();`, so the launch is built without the resume arguments.

Command, both readings:
`bash scripts/agent-run.sh run --integration --filter 'binary(chaos_revive) | (binary(cli_revive) & test(/fork/))'`

| reading | clock (UTC, before the run) | exit | document | cases |
|---|---|---|---|---|
| red, neutralised | 2026-10-10T14:38:36Z | 1 | `"ok":false`, `passed 0, failed 2` | both FAIL |
| green, restored | 2026-10-10T14:38:49Z | 0 | `"ok":true`, `passed 2, failed 0` | both PASS |

The red, per case, as nextest printed it:
- `viola::chaos_revive revive_after_a_killed_wrapper_resumes_the_logged_session_in_the_recorded_directory`:
  ``assertion `left == right` failed`` · `left: String("startup")` · `right: "resume"`. The case is red on
  `cause`, as the plan predicted: without `--resume` the fake agent fires the recorded SessionStart unchanged.
- `viola::cli_revive revive_with_fork_reports_the_fork_session`: the same assertion, `left: String("startup")`,
  `right: "resume"`.

## Control B — the strict-modes check (plan step 3's guard, called from step 7)

Site: `src/cmd/revive.rs`, `preflight`, the first reading,
`check_instance(&home, &instance_dir).is_err().then(|| refuse(Refusal::StrictModes))`. Neutralised by putting
`false &&` in front of the check, so the reading never refuses.

Command, both readings:
`bash scripts/agent-run.sh run --integration --filter 'binary(cli_revive) & test(/another_user/)'`

| reading | clock (UTC, before the run) | exit | document | case |
|---|---|---|---|---|
| red, neutralised | 2026-10-10T14:39:07Z | 1 | `"ok":false`, `passed 0, failed 1` | FAIL, 10.521 s |
| green, restored | 2026-10-10T14:39:29Z | 0 | `"ok":true`, `passed 1, failed 0` | PASS, 3.529 s |

The red, as nextest printed it: `viola::cli_revive revive_refuses_state_another_user_could_write` panicked at
`tests/support/watch.rs:54:13` with `viola never exited`. With the check skipped the revive passed its
preflight over an instance directory with a group-write bit and started its child, so the call never returned
its refusal; the test's own 7 s bound failed it and its drop guard killed the process it had started. A
process list read after the green run showed no process from a `viola-test-*` home or a `claude-bin` directory.

## Control C — the same check on the `--list` arm (plan step 7)

The `--list` arm calls the check at a site of its own, so it got a case and a reading of its own:
`revive_list_refuses_state_another_user_could_write` (`cfg(unix)`, the log file given a group-write bit).

Site: `src/cmd/revive.rs`, `list`, the line `if check_instance(&home, &instance_dir).is_err() {`. Neutralised
by `false &&` in front of the check.

Command, both readings:
`bash scripts/agent-run.sh run --integration --filter 'binary(cli_revive) & test(/list_refuses/)'`

| reading | clock (UTC, before the run) | exit | document | case |
|---|---|---|---|---|
| red, neutralised | 2026-10-10T14:40:02Z | 1 | `"ok":false`, `passed 0, failed 1` | FAIL |
| green, restored | 2026-10-10T14:40:14Z | 0 | `"ok":true`, `passed 1, failed 0` | PASS |

The red, as nextest printed it: panicked at `tests/cli_revive.rs:335:5`, ``assertion `left == right` failed``
· `left: Some(0)` · `right: Some(1)`: with the check skipped `--list` exited 0 over the widened tree.

## What no control here reads

- All three ran under the default nextest profile, not the `mutants` profile.
- All three are Linux readings. The two `cfg(unix)` cases do not compile on Windows, where the check's verdict
  on a widened tree is the `viola-state` crate's own cases.
