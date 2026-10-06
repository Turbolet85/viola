# P5 baselines — 2026-10-06-local-command-send-outcomes

The eleven entries of the plan's gate block that are new to the project, each run once on the untouched tree
(HEAD `11c77f7c9349`), 2026-10-06T22:59Z to 23:01Z. Entry numbers are the block's order when the readings were taken.

The host was quiet for every reading: load 0.49 before the first, no file over 5 MB written under a sibling
project's build dir during any harness run, every harness run back in 0 s to 13 s. No reading has the stalled-start
shape.

| entry | what it runs | exit | reading | baseline |
|---|---|---|---|---|
| 4 | unit, `test(/send_local_command_/)` | 1 | `"ok":false`, `nextest-exit-4`, 0 selected of 1312 | red: the cases do not exist yet |
| 5 | unit, `test(/send_refusal_order/)` | 0 | `"passed":12,"failed":0`, 12 selected of 1312 | red by its atom, which wants 15 |
| 6 | unit, `test(/write_send_/)` | 0 | `"passed":3,"failed":0`, 3 selected of 1312 | red by its atom, which wants 4 |
| 7 | integration, the three outcome cases and the warning case by name | 0 | `"passed":1,"failed":0`, 1 selected of 326, 13 s | red by its atom, which wants 4 |
| 8 | integration, `test(/send_under_the_paste_hint_on_a_verified_cli/)` | 1 | `nextest-exit-4`, 0 selected of 326 | red: the function does not exist yet |
| 9 | integration, the guard case and the option case by name | 1 | `nextest-exit-4`, 0 selected of 326 | red: neither case exists yet |
| 11 | `git diff --quiet 11c77f7c9349 --` over the gate's two files, the ledger and `typed.rs` | 0 | no difference | green |
| 12 | `grep -c 'LOCAL_COMMANDS' src/run/send.rs` | 1 | last line `0` | red: the send path does not name the list yet |
| 13 | `grep -rnE` for a stamps read in the two send files | 1 | no output | green |
| 14 | the no-ignore, no-sleep, no-new-env guard | 0 | no output | green |
| 15 | `grep -rn 'seven 300 ms'` over the two comment files | 0 | two lines | red: both comments still say seven |

Entry 7 was read twice: once with three names (11 s) and once as written, with the fourth name added at P5 (13 s).
Both selected the one existing case, which waits out its 10 s window.

## Known-positive controls of the green inline guards
- **11:** the same read against `2fbc9545bee9` exits 1.
- **13:** the same pattern over `src/run/version_gate.rs` exits 0 with 2 lines.
- **14:** minted lines through the entry's own two greps (the script stayed in the session scratchpad):

| minted line | grep | exit | wanted |
|---|---|---|---|
| an added `#[ignore]` line | first | 0 | 0 |
| an added `thread::sleep` line | first | 0 | 0 |
| a `thread::sleep` line with no leading `+` | first | 1 | 1 |
| an added `env::var(` line | second | 0 | 0 |
| an added `env::var_os(` line | second | 0 | 0 |
| an added `vars_os()` line | second | 1 | 1 |

And for the red count of entry 12, its positive side: the same count over
`crates/viola-agent-claude/src/ledger.rs` prints 4 and exits 0.
