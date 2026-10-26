# a11y · cascade read

## Carried
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:8) The CLI verb entity must hold `viola link` and `viola unlink` as served members in human TTY, plain and `--json` modes, and its frame verbs' server-verify refusal (exit 21, stdout empty, the hint last on stderr, the hint-free `--json` document, macOS the recorded gap) must name the two verbs once they write their frame through `client::connect`.
  > CLI verbs (`list`, `send`, `wait`, `last`, `answer`, `verify`, `pause`, `release`, `link`, `unlink`, `revive`, `plugin install`, `--help`) in human TTY, plain (non-TTY / `NO_COLOR` / `TERM=dumb`) and `--json` modes.
  (.andromeda/a11y-plan-detail/a11y-scope-summary.md:8)
  > `send`, `wait`, `last`, `answer`, `pause` and `release` each refuse, before any frame, an endpoint whose serving process is not the recorded wrapper: exit 21, stdout empty and the server-verify `hint:` line last on stderr (plain ASCII, no SGR), or under `--json` `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty (the refusal's text: Cognitive Accessibility › Error recovery); on macOS no peer pid is read and the refusal cannot occur (the recorded gap); their `--json` codes and exit codes are unchanged
  (.andromeda/a11y-plan-detail/a11y-scope-summary.md:10)
  > **Assertability:** assertable. This covers output-discipline assertions on text and streams only. No automated a11y tool reaches a terminal.
  (.andromeda/a11y-plan.md:29)
  > **Entity:** CLI verbs (`list`, `send`, `wait`, `last`, `answer`, `verify`, `pause`, `release`, `link`, `unlink`, `revive`, `plugin install`, `--help`) in human TTY, plain (non-TTY / `NO_COLOR` / `TERM=dumb`) and `--json` modes — assertable: output-discipline assertions on text and streams only; no automated a11y tool reaches a terminal.
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:18) Every refusal the two verbs print — the `-32602` answers (a `driven` that is not the serving wrapper, a `driver` equal to `driven`), the server-verify refusal and any snapshot-read refusal of the form the founder names at P4 — must be the word `unable` with plain detail, then `hint:` last on stderr with a typed exit code, and one hint-free document under `--json`; the server-verify refusal's pinned text and exit 21 must extend to `link` / `unlink`, asserted on windows-2025 and ubuntu-latest only.
  > **Pattern:** every error is a printed word followed by plain detail, and a recovery instruction where recovery exists.
  (.andromeda/a11y-plan-detail/cognitive-accessibility.md:19)
  > **CLI:** `unable  <reason>  <detail>` on stderr, then `hint:` as the last stderr line, with a typed exit code.
  (.andromeda/a11y-plan-detail/cognitive-accessibility.md:19)
  > CLI `send`, `wait`, `last`, `answer`, `pause` and `release` refuse an endpoint whose serving process is not the recorded wrapper with exit 21: `unable  <name>  instance-unreachable` on stderr, then `hint: the process on <name>'s endpoint is not its recorded wrapper; it was not contacted` as the last stderr line; under `--json` the `{"v":1,"error":"instance-unreachable","detail":null}` document with stderr empty and no hint.
  (.andromeda/a11y-plan.md:261)
  > (the frame verbs' server-verify refusal on windows-2025 and ubuntu-latest only: macOS reports no peer pid)
  (.andromeda/a11y-plan.md:345)
  > NEVER show error without recovery suggestion (per SC 3.3.3). Every human-mode CLI refusal except `unknown` (exit 14, opaque detail, design cli) ends with a `hint:` line.
  (.andromeda/a11y-plan.md:346)
  > a kebab-case reason/detail code (`not-delivered`, `human-typing`, `unverified-cli`) never appears without the plain word `unable`.
  (.andromeda/a11y-plan.md:347)
  > NEVER render a stack trace as the only error text in the CLI.
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:10) `viola --help` must gain its `handoff:` row over `link, unlink` between `wheel:` and `setup:`, plain ASCII, listing served verbs only, with the `HELP` literal of `help_is_the_grouped_verb_table` and `help_lists_only_served_verbs` moved on purpose in the same change, and the P6 structural-equivalent record must turn the row from owed to built.
  > the `--help` group labels built (chunk 2026-10-17-cli-output-discipline): `viola --help` and `viola help` print `Usage: viola [--home <dir>] <verb> [args]`, the rows `board:` · `traffic:` · `wheel:` · `setup:` over the served verbs only, then `Options:`, plain ASCII, pinned by the `HELP` literal in `help_is_the_grouped_verb_table` (`tests/cli_output_plain.rs`); `plugin install` in `setup:` built (chunk 2026-10-17-optional-plugin-install), the table's words read from the group rows alone, `plugin install` one entry and a bare `plugin` unserved, held by `help_lists_only_served_verbs`; the `handoff:` row is owned by "Explicit session links"
  (.andromeda/a11y-plan-detail/a11y-scope-summary.md:76)
  > the `--help` group labels stand since chunk 2026-10-17-cli-output-discipline (board / traffic / wheel / setup over the served verbs; the `handoff:` row joins with "Explicit session links").
  (.andromeda/a11y-plan.md:104)
  > the structural equivalents are the caption line, the `-- UNWRAPPED - READ-ONLY --` separator (only when unwrapped rows exist) and the `--help` group labels, whose `handoff:` row is owed to "Explicit session links".
  (.andromeda/a11y-plan.md:273)
  > every fixed CLI string is pinned, by trycmd or, for `viola list`, `viola --help`, `viola plugin install` and the server-verify hint, by literal, so any copy change fails CI until the pinned string is updated on purpose.
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:26) The edited `viola --help` output must keep carrying no ESC byte and no lone CR on Unix under the P6 checks, on all three OS legs.
  > **P6 — CLI board, wait/last, verify** (cli). No ARIA. trycmd / assert_cmd assert the non-TTY modes, and the portable-pty outer PTY asserts the TTY-only behaviours, on all three OS legs:
  (.andromeda/a11y-plan-detail/aria-patterns-roles.md:31)
  > `viola wait` and `viola --help` carry no ESC byte and no lone CR on Unix (`tests/support/sgr.rs` `redraws`);
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:37) The three result lines (`link … since …Z`, `link … already linked`, `unlink …`) and the `handoff:` group label must follow the cli surface's rules: static linear output with no spinner or redraw, no SGR, glyph or cursor control under `--json`, non-TTY, `NO_COLOR` or `TERM=dumb`, and any control character in a shown (logged) link name escaped except `\n` and `\t`.
  > **Surface:** cli (`viola` verbs)
  (.andromeda/a11y-plan-detail/a11y-scope-summary.md:49)
  > `--help` group labels: board / traffic / wheel / handoff / setup (layout excerpt, Heading Hierarchy Anchors → cli).
  (.andromeda/a11y-plan-detail/a11y-scope-summary.md:54)
  > there is no SGR, glyph or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run` or a passed `viola revive`;
  (.andromeda/a11y-plan-detail/a11y-scope-summary.md:55)
  > output is static and linear, with no spinner or redraw;
  (.andromeda/a11y-plan-detail/a11y-scope-summary.md:56)
  > C0/C1 control characters are escaped except `\n` and `\t`
  (.andromeda/a11y-plan.md:53)
  > output is static and linear; C0/C1 control characters are escaped except `\n` and `\t`, which `list` rows escape too.
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:19) The wrapper's served `link` / `unlink` method handling during `viola run` and a passed `viola revive` must write zero bytes of its own to the terminal, send its diagnostics to `diagnostics/` only, and never block, refuse or delay a human keystroke past the current atomic paste.
  > **Reason:** The rendered content belongs to the vendor (`claude`) and is not traced. The assertion boundary is viola's wrapper:
  (.andromeda/a11y-plan-detail/a11y-scope-summary.md:20)
  > viola writes zero bytes to the terminal except the child's output;
  (.andromeda/a11y-plan-detail/a11y-scope-summary.md:21)
  > human keystrokes are never blocked, refused or delayed past the current atomic paste;
  (.andromeda/a11y-plan-detail/a11y-scope-summary.md:70)
  > **Notes:** viola's diagnostics for `run` go to `diagnostics/`, never to the terminal (arch excerpt, A11y-Relevant Conventions → terminal output discipline).
  (.andromeda/a11y-plan.md:33)
  > viola writes zero bytes to the terminal except the child's output, human keystrokes are never blocked, refused or delayed past the current atomic paste
  (.andromeda/a11y-plan.md:55)
  > viola's diagnostics for `run` go to `diagnostics/`, never to the terminal
  (.andromeda/a11y-plan.md:334)
  > NEVER block, refuse or delay a human keystroke in `viola run`, or in a start a passed `viola revive` made, past the current atomic paste

## Bindings
- **scope-entities ↔ cognitive-errors**: the frame verbs' server-verify refusal list in the cli entity (.andromeda/a11y-plan-detail/a11y-scope-summary.md:8) and the error-recovery list with its pinned hint (.andromeda/a11y-plan-detail/cognitive-accessibility.md:19) must both gain `link` / `unlink` together, with the same exit 21, hint text and hint-free `--json` document, and the same windows-2025 / ubuntu-latest reach (.andromeda/a11y-plan.md:261).
- **aria-landmarks ↔ path-p6-board ↔ scope-surfaces**: the `handoff:` row lands in the `--help` table pinned by the `HELP` literal (.andromeda/a11y-plan-detail/aria-patterns-roles.md:10), is recorded as built in the P6 structural equivalent (.andromeda/a11y-plan-detail/a11y-scope-summary.md:76) and the cli surface's group labels (.andromeda/a11y-plan-detail/a11y-scope-summary.md:49), and must keep the P6 no-ESC / no-lone-CR check on `viola --help` (.andromeda/a11y-plan-detail/aria-patterns-roles.md:31).
- **scope-entities (tui) ↔ scope-surfaces (tui)**: the new wrapper method handling keeps the zero-bytes and keystroke boundary (.andromeda/a11y-plan-detail/a11y-scope-summary.md:20) and routes its diagnostics to `diagnostics/` (.andromeda/a11y-plan-detail/a11y-scope-summary.md:70).
- **scope-surfaces (cli) ↔ the body's link-name rule**: the result lines' C0/C1 escaping (.andromeda/a11y-plan-detail/a11y-scope-summary.md:56) sits beside the scope's re-parse of every shown logged link name through `ViolaName::try_new`, both applying to the same shown names (.andromeda/a11y-plan.md:53).
