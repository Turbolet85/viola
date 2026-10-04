# Remove-the-guard controls — 2026-10-04-wait-and-last

Each new guard test run once with its guard neutralised (red), then with the guard restored
(green, inside gate entry 8 and the full block). Run form: `cargo nextest run -p viola
--features fake-agent --test cli_wait_last -E 'test(<name>)'`, 2026-10-04, Linux dev host.

| guard | neutralised as | test | neutralised reading | restored reading |
|---|---|---|---|---|
| CARRY 2: one `error: internal error` printer (the catch site) | the `Send` dispatch arm prints `crate::human::internal_error()` again (`grep -c` = 1) | `cli_verbs_print_internal_error_once` | FAIL, exit 100: stderr `"error: internal error\nerror: internal error\n"` against `"error: internal error\n"` | PASS (gate entry 8) |
| message-mode escaper | `escape_message`'s condition gains `&& false` (nothing escaped) | `last_human_escapes_controls` | FAIL, exit 100: panic at the stdout literal assertion (`tests/cli_wait_last.rs:416`) | PASS (gate entry 8) |
| `last` agrees with the disk: the feed is updated under the append's lock hold (the operator pass's macOS fold) | `WaitFeed::appending` takes its lock after `append()` instead of before | `run::wait::tests::last_after_a_turn_on_disk_never_reads_the_turn_before_it` (`cargo nextest run -p viola --features fake-agent -E 'test(/last_after_a_turn_on_disk/)'`) | FAIL: `left: Null`, `right: "on disk"` — the reading the macOS runner gave | PASS (gate entries 4, 7) |

The unfiled-verb direction of CARRY 2 (zero lines) is the gate's own baseline: before this chunk
`role_of` filed `wait` / `last` nowhere and they did not exist (P5 baseline, entry 8 vacuous red).
