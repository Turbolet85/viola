# a11y · A11y Scope Summary — category read

## Carried
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:8) The new `viola link` / `viola unlink` verbs fall in the assertable CLI entity: their result lines, refusals and `--json` documents must hold the output discipline in all three modes.
  > CLI verbs (`list`, `send`, `wait`, `last`, `answer`, `verify`, `pause`, `release`, `link`, `unlink`, `revive`, `plugin install`, `--help`) in human TTY, plain (non-TTY / `NO_COLOR` / `TERM=dumb`) and `--json` modes.
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:8) The two verbs write their frame through the same server check as the six frame verbs, so their impostor-endpoint refusal must take this exact shape (exit 21, stdout empty, the server-verify `hint:` last on stderr with no SGR, or the one `--json` document with stderr empty), and this six-verb list is the line the chunk extends when the verbs are served.
  > `send`, `wait`, `last`, `answer`, `pause` and `release` each refuse, before any frame, an endpoint whose serving process is not the recorded wrapper: exit 21, stdout empty and the server-verify `hint:` line last on stderr (plain ASCII, no SGR), or under `--json` `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty (the refusal's text: Cognitive Accessibility › Error recovery); on macOS no peer pid is read and the refusal cannot occur (the recorded gap); their `--json` codes and exit codes are unchanged
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:10) The verbs' a11y evidence must be output-discipline assertions on text and streams only, never a tool-based a11y claim.
  > **Assertability:** assertable. This covers output-discipline assertions on text and streams only. No automated a11y tool reaches a terminal.
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:49) The `--help` output must carry the `handoff` group label among the five, in this order, once the verbs are served.
  > `--help` group labels: board / traffic / wheel / handoff / setup (layout excerpt, Heading Hierarchy Anchors → cli).
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:54) The three result lines (`link … since …Z`, `link … already linked`, `unlink …`) and the verbs' refusals must carry no SGR, glyph or cursor control under `--json`, non-TTY, `NO_COLOR` or `TERM=dumb`.
  > there is no SGR, glyph or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run` or a passed `viola revive`;
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:55) The verbs' output must be static and linear.
  > output is static and linear, with no spinner or redraw;
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:56) A logged link name shown in a result line must have its C0/C1 control characters escaped (the chunk re-parses it through `ViolaName::try_new` before it is shown).
  > C0/C1 control characters are escaped except `\n` and `\t`
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:57) The verbs' output-discipline cases must pass on all three OS legs.
  > This must hold on `[windows-2025, macos-latest, ubuntu-latest]` (tests excerpt, Coverage Triggers → multi-platform).
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:76) The `handoff:` row is this chunk's to add, and this "as built" line moves with it when the verbs are served.
  > the `--help` group labels stand since chunk 2026-10-17-cli-output-discipline (board / traffic / wheel / setup over the served verbs; the `handoff:` row joins with "Explicit session links").
- (.andromeda/a11y-plan.md:29) The body's broad entity line names `link` and `unlink` among the assertable CLI verbs.
  > **Entity:** CLI verbs (`list`, `send`, `wait`, `last`, `answer`, `verify`, `pause`, `release`, `link`, `unlink`, `revive`, `plugin install`, `--help`) in human TTY, plain (non-TTY / `NO_COLOR` / `TERM=dumb`) and `--json` modes — assertable: output-discipline assertions on text and streams only; no automated a11y tool reaches a terminal.
- (.andromeda/a11y-plan.md:53) The cli surface's rules bind the verbs' human lines: colour only a second cue, no SGR in the plain modes, static linear output, control characters escaped.
  > **Surface:** cli (`viola` verbs) — no automated a11y tool reaches terminal output: output-discipline assertions on the existing CLI drivers stand in, and no WCAG conformance claim may rest on them; colour is only a second cue; no SGR, glyph or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run` or a passed `viola revive`; output is static and linear; C0/C1 control characters are escaped except `\n` and `\t`, which `list` rows escape too.
- (.andromeda/a11y-plan.md:63) The verbs must add no interactive prompt (no confirm step), pager or TUI mode.
  > **cli:** keyboard is not interactive. There is no prompt, TUI mode or pager. Assertions cover stdin/`--file` input only (arch excerpt, Critical paths hint 7).
- (.andromeda/a11y-plan.md:73) The verbs' cases must add no new harness `suite` value.
  > **No new suite:** no new `suite` enum value (such as `a11y`) is added without a Decisions Log entry.
- (.andromeda/a11y-plan.md:74) The verbs' output-discipline cases run inside the existing suites, in CI the per-OS `coverage` suite on all three legs.
  > **CLI output discipline** runs inside `nextest-integration` / `nextest-e2e` (locally), and in CI inside the per-OS `test` job's single instrumented `coverage` suite, on all three OS legs.
- (.andromeda/a11y-plan.md:104) The `handoff:` row of the `--help` group labels is owed to this entry.
  > the `--help` group labels, whose `handoff:` row is owed to "Explicit session links".
- (.andromeda/a11y-plan.md:257) Every refusal of the two verbs (the `-32602` driven/driver refusals, `instance-unreachable`) must be `unable  <reason>  <detail>` on stderr then `hint:` last with a typed exit code, and one hint-free document under `--json`.
  > **Pattern:** every error is a printed word followed by plain detail, and a recovery instruction where recovery exists; on the CLI, `unable  <reason>  <detail>` on stderr (`viola revive`'s exit-1 refusals and `viola verify`'s interrupted pair: one `unable: <text>` line), then `hint:` as the last stderr line, with a typed exit code; the one refusal with no hint is `unknown` (exit 14), whose detail is opaque; faults are not refusals and carry no hint; `--json` gives one document with no hint.
- (.andromeda/a11y-plan.md:261) The verbs' refusal layout must be pinned by trycmd and their exit codes and hint-free `--json` by assert_cmd, the server-verify refusal on windows-2025 and ubuntu-latest only.
  > **Verification:** trycmd pins the `unable <reason> <detail>` + final `hint:` layout per refusal and assert_cmd the typed exit codes and a hint-free `--json`, on all three OS legs (the frame verbs' server-verify refusal on windows-2025 and ubuntu-latest only: macOS reports no peer pid).
- (.andromeda/a11y-plan.md:273) The verbs' fixed strings (the three result lines, their hints) and the new `viola --help` text must be pinned, so a copy change fails CI.
  > **Verification:** every fixed CLI string is pinned, by trycmd or, for `viola list`, `viola --help`, `viola plugin install` and the server-verify hint, by literal, so any copy change fails CI until the pinned string is updated on purpose.
- (.andromeda/a11y-plan.md:340) The verbs must never colour `unable` and never emit SGR in the plain or `--json` modes.
  > NEVER colour CLI `unable` / `fail`, and never emit SGR under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json` / `viola run` / a passed `viola revive`.
- (.andromeda/a11y-plan.md:345) Every human-mode refusal of the two verbs must end with a `hint:` line.
  > NEVER show error without recovery suggestion (per SC 3.3.3). Every human-mode CLI refusal except `unknown` (exit 14, opaque detail, design cli) ends with a `hint:` line.
- (.andromeda/a11y-plan.md:346) Each kebab-case reason/detail code the verbs print must sit beside the plain word `unable`.
  > a kebab-case reason/detail code (`not-delivered`, `human-typing`, `unverified-cli`) never appears without the plain word `unable`.

## Bindings
- **`--help` `handoff:` row ↔ layout-templates §Output structure — `viola --help` and tests (`help_is_the_grouped_verb_table`, `help_lists_only_served_verbs`)**: the group label order board / traffic / wheel / handoff / setup here must match the layout's `handoff:  link, unlink` row between `wheel:` and `setup:` and the pinned `HELP` literal, (.andromeda/a11y-plan-detail/a11y-scope-summary.md:49), (.andromeda/a11y-plan-detail/a11y-scope-summary.md:76), (.andromeda/a11y-plan.md:104), (.andromeda/a11y-plan.md:273)
- **server-verify refusal shape ↔ security-plan (server identity before any frame)**: the verbs' frame through `client::connect` gives the exit-21 `instance-unreachable` refusal whose a11y shape is fixed here; its server-verify `hint:` literal is the one already pinned, (.andromeda/a11y-plan-detail/a11y-scope-summary.md:8), (.andromeda/a11y-plan.md:261)
- **refusal text ↔ design cli (User-Facing Error Surfaces → CLI refusals) and architecture §Conventions (refusals, exit codes)**: the `-32602` driven/driver refusals and their typed exit codes surface as `unable … hint:` per this plan's Error recovery pattern, (.andromeda/a11y-plan.md:257), (.andromeda/a11y-plan.md:345)
- **result lines and logged link names ↔ security-plan input validation (`ViolaName::try_new`) and the second CARRY**: a logged name shown in a result line is re-parsed before display, and C0/C1 escaping stands here, (.andromeda/a11y-plan-detail/a11y-scope-summary.md:56)
- **output-discipline cases ↔ test-plan (harness `run`, `coverage` suite, three OS legs)**: no new suite; the cases run in the existing `coverage` suite on all three legs, (.andromeda/a11y-plan.md:73), (.andromeda/a11y-plan.md:74), (.andromeda/a11y-plan-detail/a11y-scope-summary.md:57)
