@fact id=fct-4m5t4osf expect_rev=4
old: | assert_cmd 2.2.2 + predicates 3.1.4; human layouts pinned by literal in their test files
new: | `std::process::Command` on the `viola` binary; each fixed string, the human layouts included, pinned by a literal in its test file

@fact id=fct-ennvbmje expect_rev=1
old: | assert_cmd 2.2.2 with `Instant` deadline |
new: | `std::process::Command` with `Instant` deadline |

@fact id=fct-fbkvffnw expect_rev=1
old: root `hook_fail_open.rs` (assert_cmd).
new: root `hook_fail_open.rs` (`std::process::Command`).

@fact id=fct-aajqyzhy expect_rev=1
old: **Tools:** assert_cmd 2.2.2
new: **Tools:** `std::process::Command` on the `viola` binary

@fact id=fct-sik7tvk4 expect_rev=1
old: use the assert_cmd target-dir lookup (§3 boot step 5)
new: take the binary dir from the test executable's own target dir (§3 boot step 5)

@fact id=fct-lzdm6e7w expect_rev=5
old: trycmd `tests/cmd/*.toml` covers the `viola-send` mirror `[RB]`/`[  ]`/`[/ ]`.
new: `tests/cli_send.rs` pins the `viola-send` mirror `[RB]`/`[  ]`/`[/ ]` by literal.
old: (`help_is_the_grouped_verb_table`), not by a trycmd case (chunk 2026-10-17-cli-output-discipline)
new: (`help_is_the_grouped_verb_table`, chunk 2026-10-17-cli-output-discipline)
old: pinned by literals, not by a trycmd case (chunk 2026-10-11-the-board-viola-list)
new: pinned by literals (chunk 2026-10-11-the-board-viola-list)
old: human lines by literal assert_cmd asserts against
new: human lines by literal asserts against
old: back it. A trycmd case for the verify step counter arrives with a redaction-heavy consumer
new: back it.
old: snapbox redactions `[..]` mask timestamps, cursors and temp paths
new: no manifest holds `assert_cmd`, `predicates`, `trycmd` or `snapbox`, and there is no `tests/cmd/`: each fixed string is a literal in its test file

@fact id=fct-xmkdtzxa expect_rev=2
old: Use the existing `assert_cmd` / literal-pin / portable-pty output-discipline assertions
new: Use the existing `std::process::Command` / literal-pin / portable-pty output-discipline assertions

@fact id=fct-yzqlnwsx expect_rev=3
old: | assert_cmd + predicates + literal pins;
new: | `std::process::Command` + literal pins;

@fact id=fct-jkkyxmll expect_rev=2
old: with a typed exit code (assert_cmd);
new: with a typed exit code (`std::process::Command`);

@fact id=fct-ijnl2u6y expect_rev=3
old: literal pins and assert_cmd assert the non-TTY modes
new: literal pins over `std::process::Command` assert the non-TTY modes

@fact id=fct-2jdilj7v expect_rev=3
old: and assert_cmd the typed exit codes and a hint-free `--json`
new: and literal asserts over `std::process::Command` hold the typed exit codes and a hint-free `--json`

@fact id=fct-s4vibfo3 expect_rev=3
old: | assert_cmd + predicates + literal pins;
new: | `std::process::Command` + literal pins;

@fact id=fct-ms7djxet expect_rev=2
old: assert_cmd / literal-pin / portable-pty results
new: `std::process::Command` / literal-pin / portable-pty results

@fact id=fct-vtox22ts expect_rev=3
old: `assert_cmd` with `predicates`, the fixed strings pinned by literal
new: `std::process::Command` on the `viola` binary, the fixed strings pinned by literal

@fact id=fct-dx4kie52 expect_rev=2
old: with a typed exit code (assert_cmd).
new: with a typed exit code (`std::process::Command`).

@fact id=fct-uok334yy expect_rev=4
old: Literal pins and assert_cmd assert the non-TTY modes
new: Literal pins over `std::process::Command` assert the non-TTY modes
old: (NAME · LIVE · STATUS · WHEEL · DIALOG · CLI), not a trycmd case: the
new: (NAME · LIVE · STATUS · WHEEL · DIALOG · CLI): the
old: checked with assert_cmd on `ubuntu-latest`:
new: checked with `std::process::Command` on `ubuntu-latest`:

@fact id=fct-g3h7n6e2 expect_rev=3
old: and assert_cmd checks the typed exit codes
new: and `std::process::Command` cases check the typed exit codes

@fact id=fct-es7le4as expect_rev=3
old: Checked with assert_cmd + portable-pty on
new: Checked with `std::process::Command` + portable-pty on
