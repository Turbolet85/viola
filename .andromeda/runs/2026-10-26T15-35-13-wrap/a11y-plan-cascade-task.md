# Task — a11y-plan · 2026-10-26-explicit-session-links

## The session
The session built the CLI verbs `viola link <driver> <driven> [--json]` and `viola unlink <driver> <driven> [--json]`, which until now were held for the founder's word on the boundary widening. The driven instance is the verb's instance.
The driven instance's wrapper now serves the channel methods `link` and `unlink`, which used to answer `-32601`. A bad name, a foreign `driven` or a self-pair is refused `-32602` and no line is written. `LinkSet::remove` gets its first product caller, and a product `unlink` event line is written for the first time.
Each verb runs these steps in order. First the strict-modes check `check_instance` on the driven instance, before any log or snapshot read. Then the liveness pre-check. Then the server identity check, before its one frame (macOS stays on the recorded gap). There are now eight CLI frame verbs instead of six.
The session added three plain result lines, a `handoff:  link, unlink` row in `viola --help`, the `--json` documents and the verbs' exit-21 refusals. The help row was owed to "Explicit session links" and is retired as an owed item.
The `security_negatives_server_identity` table grows from 19 to 25 cases: eight verbs times two modes, plus the controls. `binary(channel_links)` joins the nextest `ci` profile's stamped-home kill list.
No spec master was edited by the chunk. No config key, schema, crate, dependency or colour token changed.

## Reached files
- src/cmd/link.rs
- src/cmd/unlink.rs
- tests/channel_links.rs
- src/run/links.rs
- src/cmd/run.rs
- src/cmd/mod.rs
- src/human/verbs.rs
- src/human.rs
- tests/channel_endpoint.rs
- tests/cli_output_plain.rs
- tests/security_negatives_server_identity.rs
- tests/cli_links.rs
- .config/nextest.toml
- viola-0.1.0/verification-matrix.json
- viola-0.1.0/working-route.md

## Touches
- The `--help` `handoff:` row is stated as owed to "Explicit session links" at a11y-plan.md:104 (§1 path 6 required ARIA roles) and in the matching path-p6-board line of a11y-scope-summary.md (line 76). The row is now built: `handoff:  link, unlink`, between `wheel:` and `setup:`. Both texts go stale.
- The `--help` group labels among the cli structural equivalents (aria-patterns-roles.md, the aria-landmarks group, line 10) still name the `handoff:` row as pending. It is now built and pinned by `help_is_the_grouped_verb_table` and `help_lists_only_served_verbs`.
- The Error recovery procedure (cognitive-accessibility.md:26) names the six frame verbs' exit-21 server-verify refusal. There are now eight (`link` and `unlink` added), and the identity table that witnesses it grows from 19 to 25 cases.
- The Error recovery verification scope (a11y-plan.md:261) runs the frame verbs' server-verify refusal on windows-2025 and ubuntu-latest only. The two new verbs join that set under the same macOS gap.
- Error recovery must now carry the link verbs' refusal set. A strict-modes refusal is exit 21 with the fixed instance-state `unable` / `hint:` pair. A stopped instance is exit 21 with the not-running hint last. A server mismatch is exit 21 with the server-verify hint. In `--json` mode each is one hint-free document with stderr empty.
- A wrapper `-32602` on `link` / `unlink`, for example `driver` equal to `driven`, reaches the CLI as a fault with exit 20. This is relevant to the rule that faults are not refusals and carry no hint.
- The Plain language verification (a11y-plan.md:273, and the plain-language procedure at cognitive-accessibility.md:35–36) lists the strings pinned by literal: `viola list`, `viola --help`, `viola plugin install` and the server-verify hint. The three new link result lines are now pinned by literal in unit and integration tests, and so is the changed `--help` table.
- The CLI verbs entity in scope-entities (a11y-plan.md:29 and its group in a11y-scope-summary.md) already names `link` and `unlink`. Check whether the detail text still describes them as held or not built. They are now served in human, plain and `--json` modes.
- The scope-surfaces rule escapes C0/C1 except `\n` and `\t`, and `list` rows escape those too. A `since` that is no RFC 3339 instant prints through `escape_message`, which keeps `\n`, `\t` and non-ASCII, so check that rule against the link line.
- The cli surface's no-SGR rule (scope-surfaces, visual-sgr) gains the three link result lines as output that is plain in every mode, with no SGR even on a TTY.
- The test inventory behind the output-discipline stand-ins grows. `tests/cli_links.rs` gains six cases (design lines byte for byte, stopped instance exit 21, widened instance refused before any log, channel pair holds no names, E3, restart). `tests/cli_output_plain.rs` gains the help pin. The identity table now holds eight verbs.
- There are now three `human::refuse` call sites with an `unable` / `hint:` pair (link adds one, census 14 to 15). Any count of refusing CLI sites, or any list of verbs that take the fixed instance-state pair, moves. Before this session those were `revive`, `list`, `plugin install` and `hook statusline`.
- Any count or list of CLI frame verbs (six to eight), or of verbs that run `check_instance` before any log, moves wherever a11y-plan states one.

## New facts
- F1 · cognitive-accessibility · `viola link` and `viola unlink` print one of three plain result lines in one write, with no SGR in any mode. The lines are `link    <driver> -> <driven>  since <HH:MM:SSZ>`, `link    <driver> -> <driven>  already linked` and `unlink  <driver> -> <driven>`, and they are pinned by literal in `link_verb_lines_are_the_design_lines_in_one_write` and `link_verbs_print_the_design_lines`.
- F2 · cognitive-accessibility · The time in the `link` line is the reply's `since` as UTC clock time in whole seconds with `Z`. The names printed are the verb's own parsed arguments, never names read back from the reply or the log.
- F3 · cognitive-accessibility · When the driven instance fails the strict-modes check, `viola link` and `viola unlink` refuse with exit 21 before any log opens. In human mode they print the fixed instance-state `unable` / `hint:` pair. Under `--json` they print exactly the `instance-unreachable` error document with detail null on stdout, with stderr empty.
- F4 · cognitive-accessibility · Against a stopped driven instance, `viola link` and `viola unlink` exit 21 with the not-running hint as the last stderr line (`link_verbs_against_a_stopped_instance_exit_21`).
- F5 · cognitive-accessibility · A pair the wrapper refuses as invalid params, such as `driver` equal to `driven`, reaches the link verbs as a fault with exit 20 and no hint. It is never refused client-side.
- F6 · cognitive-accessibility · Under `--json`, the link verbs answer one document `{"v":1,"ok":…}`: `link` carries `since` and `already_linked`, and `unlink` carries an empty object.
- F7 · a11y-scope-summary · A link `since` that is no RFC 3339 instant prints through `escape_message`, which keeps `\n`, `\t` and non-ASCII. This is the restore's `unknown`, or a value from a hand-altered log.
- F8 · aria-patterns-roles · The `viola --help` group labels include `handoff:` (listing `link, unlink`), placed between `wheel:` and `setup:`.

## Untouched
- Path P4, the human wheel and tui boundary (aria-patterns-roles path-p4-wheel and the tui boundary rules), is not reached. The link verbs type nothing into the child and never touch the wheel or human keystrokes.
- Timeout extensions (cognitive-timeouts) are not reached. The link verbs add no time bound that falls on the human.
- The `viola list` caption literal and its order (cognitive-consistency) are not reached. The board does not show links: that clause of `v1-36` is carried to "Version done-check", not built.
- The amber, dim and bold SGR rules and the NAME truncation (visual-design-verification, visual-sgr) are not reached. No colour token or SGR span was added, and the link lines are plain.
- The not-assertable wrapper IPC channel entity is not reached. The `link` and `unlink` channel methods are machine-to-machine and stay not-assertable.
- §3 harness contracts and §9 CI integration are not reached. There is no new suite value, driver or CI step: the nextest kill-line entry is a `ci` profile timeout list, not an a11y stage.
- The owed `viola revive --json` document after a passed preflight is not reached. It stays owed to "Exit-cause code catalogue".
- The screen reader manual pass targets (NVDA, VoiceOver, Orca on P4 and P6) and the not-triggered list (cognitive, motion, multi-language, target-size) are not reached.
