@carried n=1 label=test-plan/e2e-test-strategy fact=fct-ja62ftpj
why: Item 15's Linux no-leak measurement must read the signal as stated here (no viola socket or home path in the child, the fixture's receipt and control files exempt by exact path), and the portable-pty move must keep the close_random_fds behaviour, or the reading is re-measured and re-worded.
quote: The child holds nothing of viola's: no socket and no path under the viola home, and fds 0–2 are the PTY; the fixture's own receipt and control files are exempt by exact path (`tests/tui_channel_fds.rs`). A count of "0/1/2 plus the PTY slave" does not hold: as measured at chunk 2026-09-27-wrapper-channel, the Linux child reads `[0,1,2,3,4]` (the fake agent's receipt and its `/dev/fd` handle), and portable-pty's `close_random_fds()` closes every fd above 2 at spawn, so a planted leak cannot go red either way.
@carried n=1 run=2 label=test-plan fact=fct-tenptmlu
quote: the child holds no viola socket or home path
@carried n=2 label=test-plan/e2e-test-strategy fact=fct-blbpbqaj
why: Item 15's measurement under the fake agent must read the running child's live /proc/<child pid>/fd table, never the fake agent's self-dump.
quote: On Unix the test reads the running child's live fd table (`/proc/<pid>/fd`), never the fake agent's self-dump, whose own receipt and `/dev/fd` handle show as extra fds.
@carried n=3 label=test-plan/e2e-test-strategy fact=fct-53es7vlo
why: The founder dispositions of v1-13's MCP link clause and v1-36's board link set must rest on these built CLI halves, the MCP side serving no link.
quote: **As landed: built.** `viola link` / `viola unlink` and the wrapper's `link` / `unlink` methods are served.
@carried n=3 run=2 label=test-plan fact=fct-7bm6a42y
quote: **As landed: built.** The driven wrapper serves `link` / `unlink`, and the scenario runs as `tests/cli_links.rs` `e3_two_links_and_a_send_leave_one_link_record` from `stamped_home`
@carried n=4 label=test-plan/e2e-test-strategy fact=fct-kylcwwrw
why: The founder disposition of v1-13's budget-release clause must rest on this built CLI half of `release --budget`, the MCP side giving only the refusal.
quote: The cli and ipc signals are root cases of `tests/cli_budget.rs` (`binary(cli_budget)`, fed through `tests/support/statusline.rs`, each from a stamped home): `path6_a_reading_over_the_threshold_refuses_the_send`, `path6_the_boundaries_and_the_window_word`, `path6_an_unknown_or_passed_reset_does_not_pause`, `path6_release_budget_lifts_the_pause`
@carried n=5 label=test-plan/e2e-test-strategy fact=fct-fbkvffnw
why: Item 8's survivor by construction must stay as recorded: the other-euid wiring at the client seams has no committed witness and no seam is added for it.
quote: a server of another euid (`server-verify-failed`, its wiring with no committed witness) each exit 0 with empty streams and no frame.
@carried n=5 run=2 label=test-plan/e2e-test-strategy fact=fct-fbkvffnw
quote: The seam `src/cmd/hook/seam.rs` fires only on exactly `1` and compiles only with feature `fake-agent`, so `cargo build --release --bin viola` never contains it
@carried n=5 run=3 label=test-plan fact=fct-lkgqv65p
quote: the euid wiring at the three client seams is held by no committed test (§1 Untestable zones).
@carried n=6 label=test-plan/e2e-test-strategy fact=fct-hhwc4gim
why: This Windows-only control wording is test-plan text that must follow item 17's remnant removal: removed, or kept with its reason, where the masters-agree check runs.
quote: Its two controls, on test-owned `CreateNamedPipeW` pipes read through the same client-end instrument, tell the bit apart: `security_negative_control_accept_remote_pipe_reads_the_bit_clear` (`PIPE_ACCEPT_REMOTE_CLIENTS`) reads it clear, and `security_negative_control_reject_remote_pipe_reads_the_bit_set` (`PIPE_REJECT_REMOTE_CLIENTS`) reads it set.
@carried n=7 label=test-plan/e2e-test-strategy fact=fct-nhle4u5a
why: The portable-pty move under viola-pty must keep the no-EOF chaos case green, exit read on the handle and the held output asserted, and the outer-PTY test driver must keep reading exit only through child.wait() on the new release.
quote: the no-EOF fake mode: the wrapper detects exit on the handle (`exit_source:"handle-wait"`). The output outlives the session leader only while the `--exit-no-eof` holder runs in its own process group (a holder in the leader's foreground group dies by SIGHUP when the leader exits), as measured in CI runs 36165685381 / 36166907442, and the held-output half is asserted
@carried n=7 run=2 label=test-plan fact=fct-47fi4ugg
quote: a no-EOF child's exit detected on the handle
@carried n=7 run=3 label=test-plan fact=fct-ktlsbc3h
quote: a portable-pty outer PTY over the `viola-pty` seam; exit only via `child.wait()`, the master reader drained on its own thread
@carried n=7 run=4 label=test-plan fact=fct-ukay3vyw
quote: NEVER detect child exit by reading master EOF; use `child.wait()` on the handle.
@carried n=8 label=test-plan/e2e-test-strategy fact=fct-nwgyqyjs
why: v1-53's live run on Linux must confirm the fake agent's Unix SIGHUP behaviour that this gated case witnesses on CI's one leg, and the live reading stays a local measurement in the chunk's evidence, never proof drawn from the fake agent.
quote: **A hangup landing while the fake agent exits** (`tests/cli_fake_agent.rs` `fake_agent_exit_outlives_a_hangup_that_lands_while_it_exits`, `cfg(target_os = "linux")`): the case reads the agent's SIGHUP disposition through `ps -o sigignore= -p <pid>` (`ignores_sighup`), a failed read carrying `ps`'s exit and both streams; it runs on CI's one leg, `ubuntu-latest` (§7 Fake agent).
@carried n=8 run=2 label=test-plan fact=fct-6ugas3oa
quote: A route entry's live readings of a real session are that chunk's own local measurement, kept in its `evidence/` with its own launcher, driver and readers (as at chunk 2026-10-08-first-live-test-and-self-drive): not a test layer, not harness surface, never in CI.
@carried n=8 run=3 label=test-plan fact=fct-egysilpo
quote: NEVER treat a green fake-agent run as proof of real-CLI behaviour.
@carried n=9 label=test-plan/e2e-test-strategy fact=fct-sn6e6lxw
why: A fold of item 6's divergent error enums and item 16's re-measured thiserror derive census must keep the error-body enumeration's guards green, and item 5's record must keep the pinned-exe re-hash pinned to its library.
quote: Its guards: `error_bodies_each_name_a_writer_and_a_witness_that_stand`, `channel_faults_are_the_seven_literal_bodies`, `product_error_writers_are_each_named` and `thiserror_messages_interpolate_nothing`, with the census self-check `census_finds_an_unnamed_writer_and_an_interpolated_message`.
@carried n=9 run=2 label=test-plan/e2e-test-strategy fact=fct-sn6e6lxw
quote: `tests/contract_content_hash.rs` pins the pinned-exe content hash (§1 "SHA-256 16-hex re-hash") to its library, `sha2` (a root dev-dependency)
@carried n=10 label=test-plan/e2e-test-strategy fact=fct-liulfbun
why: v1-53's claim re-reads this census, and item 17's remnant removal must leave it with no unnamed gate, no stale row and no gate naming another OS.
quote: its `GATES` table holds a `file`, `test`, `gate` and `reason` per row, and the census walks every `.rs` directly under `tests/` and in each `crates/*/tests/`, counting a leading `#![cfg(…)]` and a test function whose contiguous attribute run holds `#[test]`, `#[rstest]` or `#[tokio::test]` and a `#[cfg(…)]` or `#[cfg_attr(…)]` whose predicate names `windows`, `unix` or `target_os` outside its string values; `faults` reads three ways: an unnamed gate, a row that no longer stands, and a gate whose predicate names `windows` or `target_os = "macos"` (`another OS: …`) even when a row names it
@carried n=10 run=2 label=test-plan/e2e-test-strategy fact=fct-vgds3skp
quote: **The census count:** the census reads 39 OS-gated test functions and 2 gated files, 41 `GATES` rows (rule: the rows of `GATES` in `tests/contract_os_gated_tests.rs`; as measured at chunk 2026-11-05-linux-only-code-and-ci).
@binding n=1
**e2e-test-strategy (E2 fd reading) ↔ security-plan and the portable-pty dependency move**: item 15 measures the no-leak property through the live `/proc/<child pid>/fd` table under the fake agent, the receipt and `/dev/fd` handle exempt by exact path (test-plan/e2e-test-strategy fct-blbpbqaj) (test-plan/e2e-test-strategy fct-ja62ftpj), and only that reading lets security-plan fct-p7rzex2i leave «recorded, not established»; the `close_random_fds()` clause is re-measured on the new portable-pty release, and the E2 text is re-worded if the release changes it.
@binding n=2
**e2e-test-strategy (chaos no-EOF, outer-PTY driver) ↔ unit-test-strategy / integration-test-strategy (viola-pty seam, Module ↔ PTY)**: the portable-pty move keeps the no-EOF case's handle-wait exit and held output green (test-plan/e2e-test-strategy fct-nhle4u5a), and the test-side outer PTY, itself a portable-pty consumer, keeps reading exit only via `child.wait()` (test-plan fct-ktlsbc3h) (test-plan fct-ukay3vyw), beside the six-method seam and the openpty spawns (test-plan fct-sbml5vi6) (test-plan fct-nvtf2boy).
@binding n=3
**e2e-test-strategy (E3 and Path 6 as landed) ↔ the verification matrix (v1-13, v1-36 dispositions) and the MCP tool list**: the founder's deferral or re-worded acceptance for v1-13's MCP link and budget-release clauses and v1-36's board link set names the built CLI halves (test-plan/e2e-test-strategy fct-53es7vlo) (test-plan/e2e-test-strategy fct-kylcwwrw) against an MCP list of exactly five tools, never `release` or `link` (test-plan fct-apiugjwf).
@binding n=4
**e2e-test-strategy (OS-gated census) ↔ item 17 remnant removal and v1-53's claim**: removing the Windows remnants leaves the census with no fault, and its count of 39 functions, 2 files and 41 rows is re-read, or re-measured and re-worded where a removal moves a gate (test-plan/e2e-test-strategy fct-liulfbun) (test-plan/e2e-test-strategy fct-vgds3skp); the Linux-gated SIGHUP case is one of those rows and v1-53's live run confirms what it witnesses (test-plan/e2e-test-strategy fct-nwgyqyjs), and that live run stays a local measurement in the chunk's evidence (test-plan fct-6ugas3oa).
@binding n=5
**e2e-test-strategy (security sweep) ↔ scope-summary untestable zone and matrix v1-44**: item 8's always-admit survivor stays recorded as untestable, with the other-euid wiring left without a committed witness and no seam added (test-plan/e2e-test-strategy fct-fbkvffnw) (test-plan fct-lkgqv65p) (test-plan fct-upolcsa3).
@binding n=6
**e2e-test-strategy (security control negatives) ↔ item 17 and the test-plan's own Windows wording**: the `CreateNamedPipeW` controls' text, Windows-only, follows item 17's remnant removal under the masters-agree check, beside the per-OS perf and coverage wording (test-plan/e2e-test-strategy fct-hhwc4gim) (test-plan fct-3mj52n2x) (test-plan fct-vxnii5yh).
@binding n=7
**e2e-test-strategy (contract error bodies) ↔ architecture / conventions / obs-plan (one-enum-per-crate, thiserror census)**: an item 6 fold of `ServeError`, `StampsError`, `PinError`, `StampsReadError` or `Refused` keeps `product_error_writers_are_each_named` and `thiserror_messages_interpolate_nothing` green and the enumeration tables naming each writer, while item 16 re-words the derive count (test-plan/e2e-test-strategy fct-sn6e6lxw).
