@carried n=1 label=test-plan/e2e-test-strategy fact=fct-imixgk6n
why: Any fake-agent reading of the CARRY on the wheel's closed non-editing list that the chunk takes up on Unix must assert this wheel record, or its absence, as Path 5 defines it.
quote: A `wheel{holder:"human",cause:"human-input"}` record is logged.
@carried n=2 label=test-plan/e2e-test-strategy fact=fct-kn7ryhs7
why: The Unix-only statusline pass-through is one of the OS-specific behaviours the chunk names as justifying a one-OS test, so its gates must carry this reason and hold on both Unix runners while Windows keeps running no command.
quote: The marker equality is Unix's: there `run` writes the instance's `settings.json` override and the arm runs the recorded command as `/bin/sh -c <command>` under a 5 s bound (PROVISIONAL). On Windows `run` writes no override and the arm runs no command (`override_document` and `shell_argv` return `None` for every input). Exit 0 with empty stderr holds on every OS.
@carried n=2 run=2 label=test-plan/e2e-test-strategy fact=fct-bo4nzuwn
quote: `agent-run boot --statusline-echo` and the harness-session form of this step landed at chunk 2026-10-17-end-to-end-path-binary-and-harness-legs as `crates/viola-e2e/tests/path6_budget_harness.rs` `path6_two_instance_release_over_a_harness_session` (steps 1 and 7 over `harness_session` booted with the flag; the planted source read 0600 on Unix).
@carried n=2 run=3 label=test-plan/e2e-test-strategy fact=fct-kylcwwrw
quote: Step 6 is `hook_statusline_a_home_another_user_can_write_runs_nothing_however_often` in `tests/hook_statusline.rs` (`cfg(unix)`, two invocations, no role line added).
@carried n=2 run=4 label=test-plan/e2e-test-strategy fact=fct-kylcwwrw
quote: on Unix the statusline arm prints the marker output and files the marker, and on Windows (`cfg(not(unix))`, run on `windows-2025` in CI) it prints nothing and files no marker.
@carried n=2 run=5 label=test-plan fact=fct-jtonrpse
quote: `budget.json` holds the fed reading and, on Unix, the statusline output passes through byte for byte;
@carried n=2 run=6 label=test-plan fact=fct-mwv2sqc7
quote: a restart rewrites `plugin/` with absolute hook commands, never a PATH lookup, and on Unix the `settings.json` override.
@carried n=3 label=test-plan/e2e-test-strategy fact=fct-ja62ftpj
why: The Unix child-fd and env-strip readings run under openpty on both Unix runners, and their measured fd set is the Linux child's, so the macOS leg must be read against this rule and never assumed from a Linux run.
quote: On Unix, the child holds nothing of viola's: no socket and no path under the viola home, and fds 0–2 are the PTY; the fixture's own receipt and control files are exempt by exact path (`tests/tui_channel_fds.rs`). A count of "0/1/2 plus the PTY slave" does not hold: as measured at chunk 2026-09-27-wrapper-channel, the Linux child reads `[0,1,2,3,4]` (the fake agent's receipt and its `/dev/fd` handle), and portable-pty's `close_random_fds()` closes every fd above 2 at spawn, so a planted leak cannot go red either way. On Windows, handle non-inheritance is asserted at the product unit boundary.
@carried n=3 run=2 label=test-plan/e2e-test-strategy fact=fct-blbpbqaj
quote: On Unix the test reads the running child's live fd table (`/proc/<pid>/fd`), never the fake agent's self-dump, whose own receipt and `/dev/fd` handle show as extra fds.
@carried n=3 run=3 label=test-plan/e2e-test-strategy fact=fct-ja62ftpj
quote: On Unix a `claude_env_keep` name survives.
@carried n=3 run=4 label=test-plan fact=fct-tenptmlu
quote: on Unix a `claude_env_keep` name survives and the child holds no viola socket or home path
@carried n=4 label=test-plan/e2e-test-strategy fact=fct-7jgwgzx2
why: The hook fail-open matrix's Unix-run rows must keep exit 0 with empty streams on both Unix runners, its server-identity rows held off macOS by the named carve-out.
quote: on Linux and Windows, a server that is not the recorded wrapper (`server-verify-failed`); on Unix, beyond this matrix, a socket directory the check refuses (`socket-dir-refused`, `security_negative_a_hook_fails_open_on_the_socket_directory` in `tests/security_negatives_socket_dir.rs`) and a server of another euid (`server-verify-failed`, its wiring with no committed witness) each exit 0 with empty streams and no frame.
@carried n=4 run=2 label=test-plan/e2e-test-strategy fact=fct-7jgwgzx2
quote: The impostor rows, each under `cfg_attr(not(target_os = "macos"), …)`, are `case::impostor` in `hook_fails_open_silently_within_the_spine_bound` and `question_impostor` and `permission_impostor` in `hook_dialog_tier_fails_open_silently`
@carried n=4 run=3 label=test-plan fact=fct-4aawmv2h
quote: and, on Linux and Windows, a server that is not the recorded wrapper and, on Unix, a socket directory the check refuses or a server of another euid (its decision held by injected euids, §1 Untestable zones), each exit 0 with no body
@carried n=5 label=test-plan/e2e-test-strategy fact=fct-eoindiy7
why: The macOS carve-out must stay a gate with its named reason, so no test reads the macOS server check as a pid or start-time comparison.
quote: server verification fails. On Linux and Windows: `snapshot.json` pid or start time does not match the listening process (a same-user impostor listener on the recorded endpoint), as a `#[cfg(not(target_os = "macos"))]` case. On Linux and macOS a server of another euid reads `server-verify-failed` too, through the client euid check, its decision held by injected values and each seam's wiring by no committed test; on macOS that is the word's whole meaning, the client reading no peer pid and comparing no server (the named carve-out, owned by no route entry).
@carried n=5 run=2 label=test-plan/e2e-test-strategy fact=fct-7lzg7pgb
quote: **Server identity** (`#![cfg(not(target_os = "macos"))]`, IPC client-side server verification).
@carried n=5 run=3 label=test-plan fact=fct-ndx7lm2p
quote: beside the table, the server-verification cause is a case of `tests/security_negatives_server_identity.rs` on Linux and Windows (macOS compares no peer pid, the named carve-out `server_verdict_rows` holds, so these negatives compile out there; a loosened socket directory on Linux and macOS is a case of `tests/security_negatives_socket_dir.rs`)
@carried n=6 label=test-plan/e2e-test-strategy fact=fct-eoindiy7
why: The exit codes and closed `detail` words must read the same on every OS, and the matrix's Unix-run cases assert them, its one-OS rows keeping their named gates.
quote: As landed, exit 21's six closed `detail` causes are `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed` and `socket-dir-refused`, on the eight frame verbs, `list` and `plugin install` (`socket-dir-refused` on the eight frame verbs alone), with `error` `instance-unreachable` and MCP's text `refused: instance-unreachable`.
@carried n=6 run=2 label=test-plan/e2e-test-strategy fact=fct-eoindiy7
quote: the PTY child is `.cmd` / `.bat` (`#[cfg(windows)]`)
@carried n=6 run=3 label=test-plan/e2e-test-strategy fact=fct-eoindiy7
quote: `link_verbs_refuse_a_widened_instance_before_any_log` (`cfg(unix)`) makes the driven instance's directory group-writable while its wrapper runs;
@carried n=7 label=test-plan/e2e-test-strategy fact=fct-7lzg7pgb
why: The Unix-only security negatives are one-OS tests whose named reason (Unix socket directory and modes) must stand where each gate does and hold green on both Unix runners.
quote: **Unix socket directory refusal** (`#![cfg(unix)]`, IPC access control).
@carried n=7 run=2 label=test-plan/e2e-test-strategy fact=fct-7lzg7pgb
quote: off Unix the check returns `Ok(())`, the DACL and the pid comparison staying the controls.
@carried n=7 run=3 label=test-plan/e2e-test-strategy fact=fct-7lzg7pgb
quote: **Umask-independent modes** (`#[cfg(unix)]`).
@carried n=7 run=4 label=test-plan fact=fct-4uc5ehox
quote: the Unix socket directory refusal (`tests/security_negatives_socket_dir.rs`, Linux and macOS: a directory at 0755 or a symlink to a 0700 directory refuses `viola run` exit 1 before any bind,
@carried n=8 label=test-plan/e2e-test-strategy fact=fct-hdfefjry
why: The v1-40 stale-row witness reads `viola list` under the same outer PTY, where on Unix the SGR stream reaches the outer terminal unre-rendered, so it must assert on the verb's SGR bytes alone.
quote: on Windows the terminal case reads each escaped name as text under a 200-column terminal (ConPTY re-renders the stream with escapes of its own) and the no-ESC assertion is Unix-only; the binary sits on the nextest 20 s kill line.
@carried n=8 run=2 label=test-plan fact=fct-gnzxb6ex
quote: TTY rendering is asserted only on the SGR sequences in verb output under the outer PTY, never on the child agent's screen.
@carried n=9 label=test-plan/e2e-test-strategy fact=fct-6whu4pj5
why: Exit read on the openpty handle and never on EOF is the chunk's first need, and this measured Linux and macOS split decides which half each Unix runner may assert.
quote: the no-EOF fake mode: the wrapper detects exit on the handle (`exit_source:"handle-wait"`). Whether the output outlives the session leader is OS-specific, as measured in CI runs 36165685381 / 36166907442: natively on ConPTY; on Linux only while the `--exit-no-eof` holder runs in its own process group (a holder in the leader's foreground group dies by SIGHUP when the leader exits); on macOS the output ends at the leader's exit even with the holder alive in its own group. The held-output half is therefore asserted on Windows + Linux only, and on macOS exit on the handle is shown by the handle wait alone (`tests/tui_pty_seam.rs`) and a `session-end` record is logged
@carried n=9 run=2 label=test-plan/e2e-test-strategy fact=fct-6whu4pj5
quote: SIGSTOP the wrapper (Unix) for over 5 s: `liveness:"stale"`, and a new `run` exits 1 with `stale-heartbeat` (`run_refuses_a_stale_name`, on the `Frozen` guard); after SIGKILL: omitted as gone
@carried n=9 run=3 label=test-plan fact=fct-47fi4ugg
quote: a SIGSTOPped wrapper read `stale` and its name refused to a new `run` with `stale-heartbeat`
@carried n=9 run=4 label=test-plan fact=fct-x7oaiapn
quote: macOS local runs, which are CI-only.
@carried n=10 label=test-plan/e2e-test-strategy fact=fct-6whu4pj5
why: The outer-PTY driver on Unix must read the forced vt100 panic by byte search, the child's own bytes reaching the outer terminal and no viola literal appearing, as the chunk names.
quote: forced vt100 panic mode (`chaos_feed_panic_refuses_send_and_keeps_passthrough`, `tests/chaos_feed_panic.rs`): `send` exits 13 with `input-not-ready`, human passthrough still reaches the receipt (a typed key reaches the child, nothing is typed), and the outer terminal shows the child's own panicking bytes (`e4 b8 ad`, Unix) and none of `panic`, `parse-rejected`, `vt100`, `unable`, `hint:` (every OS)
@carried n=10 run=2 label=test-plan fact=fct-ktlsbc3h
quote: a portable-pty outer PTY over the `viola-pty` seam; exit only via `child.wait()`, the master reader drained on its own thread, verdicts from `events.ndjson`, the receipt and the exit status, and for the forced vt100 panic chaos case a byte search of the drained master reader's stream (the child's own bytes on Unix, no viola literal on every OS), never a parse of the rendered screen.
@carried n=11 label=test-plan/e2e-test-strategy fact=fct-iplgfuqy
why: The per-OS test job the chunk re-reads green on ubuntu-latest and macos-latest runs this scan and the G4 schema check over every Unix home before its uploads.
quote: The same scan runs in CI over `target/e2e-home/**`, the harness capture in `target/agent-run/` except `run --mutants`' `target/agent-run/chunk.diff` (repository source text, excluded from the scan and from the `harness-<os>` upload), and `target/nextest/ci/junit.xml` before any upload (obs-plan §9 step order).
@carried n=11 run=2 label=test-plan/e2e-test-strategy fact=fct-iplgfuqy
quote: `diagnostics/` files are 0600 on Unix
@carried n=11 run=3 label=test-plan/e2e-test-strategy fact=fct-iplgfuqy
quote: `v1-47`'s "after every end-to-end test" reads "after each job's end-to-end run": `AGENT_RUN_KEEP_HOMES` `"1"` on the job and `if: always()` (`.github/workflows/ci.yml`), the `viola-chaos-*` home out (`.andromeda/obs-plan-detail/ci-integration.md`).
@carried n=11 run=4 label=test-plan fact=fct-nfrombzn
quote: after the test steps, every home-level `diagnostics/*.ndjson` line under `target/e2e-home/**` is validated with jsonschema 0.57.0 against obs-plan's `schemas/diag-line.v1.json`, and every `instances/*/diagnostics/detail-*.ndjson` line against `schemas/diag-detail.v1.json`.
@binding n=1
**e2e-test-strategy ↔ ci-integration**: each one-OS split carried here (the macOS carve-out, the Unix-only socket directory, the no-EOF held-output half) is read only on its own runner of the matrix `windows-2025`, `macos-latest`, `ubuntu-latest` (test-plan fct-shyrfs6v), and no Linux-only local run stands for the macOS leg (test-plan fct-lktzdsoq); the measured split itself is (test-plan/e2e-test-strategy fct-6whu4pj5).
@binding n=2
**e2e-test-strategy ↔ quality-gates-coverage-targets**: a scenario or suite case moved off `cfg(windows)` onto the Unix runners changes what each Unix job's coverage gate counts, because the gate is enforced per OS (test-plan fct-ryyndnld); the gates it moves are those named at (test-plan/e2e-test-strategy fct-7lzg7pgb) and (test-plan/e2e-test-strategy fct-kylcwwrw).
@binding n=3
**e2e-test-strategy ↔ integration-test-strategy**: the E2E exit-on-handle reading over openpty (test-plan/e2e-test-strategy fct-6whu4pj5) sits beside the Module ↔ PTY boundary over real openpty spawns of the fake agent, a resize propagated and a key written after it (test-plan fct-nvtf2boy); each is asserted at its own layer.
@binding n=4
**e2e-test-strategy ↔ test-data-fixtures**: what the no-EOF holder's output and a killed wrapper's hangup reach on Linux and macOS (test-plan/e2e-test-strategy fct-6whu4pj5) rests on the fake agent's Unix process groups and its SIGHUP-ignoring exit (test-plan fct-7o4nbra2).
@binding n=5
**e2e-test-strategy ↔ obs diagnostics schema**: the closed `detail` words of the exit-cause matrix (test-plan/e2e-test-strategy fct-eoindiy7) and every Unix home's diagnostics lines validated against `schemas/diag-line.v1.json` (test-plan fct-nfrombzn) hold one schema and one word set on every OS, never an OS-only word.
@binding n=6
**e2e-test-strategy ↔ test anti-patterns (E2E)**: the v1-40 stale-row witness under the outer PTY asserts only on verb SGR bytes (test-plan fct-gnzxb6ex) and waits out staleness only as the product's own status field, as the SIGSTOP chaos case does (test-plan/e2e-test-strategy fct-6whu4pj5), the one real-time wait the ban allows (test-plan fct-45jodiii).
