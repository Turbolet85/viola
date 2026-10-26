# Apply — a11y-plan

## Verdicts
- fanout-a11y-plan.md 1 · applied · .andromeda/a11y-plan.md:104#1
- fanout-a11y-plan--cascade.md 5 · folded into fanout-a11y-plan.md 1
- fanout-a11y-plan.md 2 · applied · .andromeda/a11y-plan.md:273#1
- fanout-a11y-plan--cascade.md 15 · folded into fanout-a11y-plan.md 2
- fanout-a11y-plan--cascade.md 1 · applied · .andromeda/a11y-plan-detail/a11y-scope-summary.md:8#1
- fanout-a11y-plan--cascade.md 2 · applied · .andromeda/a11y-plan-detail/a11y-scope-summary.md:12+#1
- fanout-a11y-plan--cascade.md 3 · applied · .andromeda/a11y-plan-detail/a11y-scope-summary.md:59+#1
- fanout-a11y-plan--cascade.md 4 · applied · .andromeda/a11y-plan-detail/a11y-scope-summary.md:76#1
- fanout-a11y-plan--cascade.md 6 · applied · .andromeda/a11y-plan-detail/aria-patterns-roles.md:10#1, .andromeda/a11y-plan-detail/aria-patterns-roles.md:10#2
- fanout-a11y-plan--cascade.md 7 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:19#1
- fanout-a11y-plan--cascade.md 8 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:21+#1
- fanout-a11y-plan--cascade.md 9 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:21+#2
- fanout-a11y-plan--cascade.md 10 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:21+#3
- fanout-a11y-plan--cascade.md 11 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:26#1
- fanout-a11y-plan--cascade.md 12 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:29+#1
- fanout-a11y-plan--cascade.md 13 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:38+#1
- fanout-a11y-plan--cascade.md 14 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:38+#2

## Edits
- line: .andromeda/a11y-plan.md:104
  old: «the `--help` group labels, whose `handoff:` row is owed to "Explicit session links".»
  new: «the `--help` group labels.»
- line: .andromeda/a11y-plan.md:273
  old: «for `viola list`, `viola --help`, `viola plugin install` and the server-verify hint, by literal,»
  new: «for `viola list`, `viola --help`, `viola plugin install`, the server-verify hint and the `viola link` / `viola unlink` result lines, by literal,»
- line: .andromeda/a11y-plan-detail/a11y-scope-summary.md:8
  old: «`send`, `wait`, `last`, `answer`, `pause` and `release` each refuse, before any frame,»
  new: «`send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` each refuse, before any frame,»
- after: .andromeda/a11y-plan-detail/a11y-scope-summary.md:12
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · scope-entities -->»
  add: «»
  add: «- **Entity:** `viola link <driver> <driven> [--json]` and `viola unlink <driver> <driven> [--json]`, the verb's instance the driven one, in the human TTY, plain and `--json` modes: each prints one plain-ASCII result line in one write with no SGR in any mode (the lines: Cognitive Accessibility › Plain language), or under `--json` one `{"v":1,"ok":<ok>}` document; each refuses with exit 21 a driven instance that fails the strict-modes check, before any log opens, a stopped driven instance and a server that is not the recorded wrapper (the entity above), and a pair the wrapper refuses as invalid params reaches it as a fault, exit 20, no hint (the refusals' text: Cognitive Accessibility › Error recovery)»
  add: «  - **Source:** `src/cmd/link.rs` (`link_call`), `src/cmd/unlink.rs`, `src/human/verbs.rs` (`write_linked`, `write_already_linked`, `write_unlinked`); `tests/cli_links.rs`.»
  add: «  - **Assertability:** assertable. This covers output-discipline assertions on text and streams only. No automated a11y tool reaches a terminal.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/a11y-plan-detail/a11y-scope-summary.md:59
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · scope-surfaces -->»
  add: «»
  add: «- **Surface:** cli, `viola link`'s result line — a `since` that is no RFC 3339 instant (the restore's `unknown`, or a value from a hand-altered log) prints through `escape_message`, which keeps `\n`, `\t` and non-ASCII; the wrapper's own `since` is a link line's `ts` or `unknown`, so such a byte reaches the line only from a hand-altered log.»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/a11y-plan-detail/a11y-scope-summary.md:76
  old: «board / traffic / wheel / setup over the served verbs; the `handoff:` row joins with "Explicit session links"»
  new: «board / traffic / wheel / handoff / setup over the served verbs, the `handoff:` row `handoff:  link, unlink` between `wheel:` and `setup:`»
- line: .andromeda/a11y-plan-detail/aria-patterns-roles.md:10
  old: «`wheel:` · `setup:` over the served verbs only»
  new: «`wheel:` · `handoff:` · `setup:` over the served verbs only»
- line: .andromeda/a11y-plan-detail/aria-patterns-roles.md:10
  old: «; the `handoff:` row is owned by "Explicit session links" |»
  new: «; the `handoff:` row is `handoff:  link, unlink`, between `wheel:` and `setup:`, held by the same two cases |»
- line: .andromeda/a11y-plan-detail/cognitive-accessibility.md:19
  old: «CLI `send`, `wait`, `last`, `answer`, `pause` and `release` refuse an endpoint»
  new: «CLI `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` refuse an endpoint»
- after: .andromeda/a11y-plan-detail/cognitive-accessibility.md:21
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · cognitive-errors -->»
  add: «»
  add: «- **CLI, `viola link` / `viola unlink`, strict-modes:** a driven instance that fails the strict-modes check is refused with exit 21 before any log opens, no log line written: the fixed instance-state `unable` / `hint:` pair on stderr, `hint:` last; under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout with stderr empty and no hint.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/a11y-plan-detail/cognitive-accessibility.md:21
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · cognitive-errors -->»
  add: «»
  add: «- **CLI, `viola link` / `viola unlink`, stopped instance:** a driven instance with no live endpoint is refused with exit 21, the not-running `hint:` as the last stderr line.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/a11y-plan-detail/cognitive-accessibility.md:21
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · cognitive-errors -->»
  add: «»
  add: «- **CLI, `viola link` / `viola unlink`, wrapper refusal:** a pair the wrapper refuses as invalid params (`-32602`, e.g. `driver` equal to `driven`) reaches the verb as a fault, exit 20, no hint; the verbs never refuse it client-side.»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/a11y-plan-detail/cognitive-accessibility.md:26
  old: «the six frame verbs' exit-21 server-verify refusal, by the `HINT` literal in `tests/security_negatives_server_identity.rs`, whose `security_negative_cli_verb_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper` reads, over `send`, `wait`, `last`, `answer`, `pause` and `release` in both modes»
  new: «the eight frame verbs' exit-21 server-verify refusal, by the `HINT` literal in `tests/security_negatives_server_identity.rs`, whose `security_negative_cli_verb_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper` reads, over `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` in both modes»
- after: .andromeda/a11y-plan-detail/cognitive-accessibility.md:29
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · cognitive-errors -->»
  add: «»
  add: «- **Verification, `viola link` / `viola unlink`:** in `tests/cli_links.rs`, `link_verbs_against_a_stopped_instance_exit_21` reads the stopped-instance refusal (exit 21, the not-running hint last), and `link_verbs_refuse_a_widened_instance_before_any_log` the strict-modes refusal (a group-writable driven instance refused before any log, its mode read back as found; `cfg(unix)`, so on ubuntu-latest and macos-latest only).»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/a11y-plan-detail/cognitive-accessibility.md:38
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · cognitive-plain-language -->»
  add: «»
  add: «- **Verification, `viola link` / `viola unlink`:** their three result lines (`write_linked`, `write_already_linked`, `write_unlinked`; the lines' text: design-system §Surface: cli), each one write with no SGR in any mode, are pinned by literal in the unit `link_verb_lines_are_the_design_lines_in_one_write` (`src/human/verbs.rs`) and the root case `link_verbs_print_the_design_lines` (`tests/cli_links.rs`).»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/a11y-plan-detail/cognitive-accessibility.md:38
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · cognitive-plain-language -->»
  add: «»
  add: «- **`viola link`'s time and names:** the `link` line's time is the reply's `since` as UTC clock time in whole seconds with `Z`, pinned by the unit `link_since_is_the_utc_time_alone` (`src/human/verbs.rs`); the names each line prints are the verb's own parsed arguments, never names read back from the reply or the log.»
  add: «»
  add: «<!-- /detail -->»

## Patterns
[[pattern]]
id = "a11y-01"
fixed = 'row is owed to "Explicit session links"'

[[pattern]]
id = "a11y-02"
regex = 'board / traffic / wheel / setup|row joins with "Explicit session links"'

[[pattern]]
id = "a11y-03"
fixed = '`wheel:` · `setup:`'

[[pattern]]
id = "a11y-04"
fixed = 'row is owned by "Explicit session links"'

[[pattern]]
id = "a11y-05"
fixed = '`answer`, `pause` and `release` each refuse'

[[pattern]]
id = "a11y-06"
fixed = '`answer`, `pause` and `release` refuse an endpoint'

[[pattern]]
id = "a11y-07"
fixed = 'the six frame verbs'

[[pattern]]
id = "a11y-08"
fixed = '`viola plugin install` and the server-verify hint, by literal'
