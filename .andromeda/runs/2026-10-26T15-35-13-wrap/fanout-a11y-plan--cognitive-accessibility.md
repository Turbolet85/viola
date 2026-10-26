proposals:
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 19
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'In the server-verify sentence, the verb list becomes the eight CLI frame verbs: "CLI `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` refuse an endpoint whose serving process is not the recorded wrapper with exit 21: `unable  <name>  instance-unreachable` on stderr, then `hint: the process on <name>''s endpoint is not its recorded wrapper; it was not contacted` as the last stderr line (for `link` / `unlink`, `<name>` is the driven instance; chunk 2026-10-26-explicit-session-links); under `--json` the `{"v":1,"error":"instance-unreachable","detail":null}` document with stderr empty and no hint."'
    sidecar: 'cognitive-accessibility.md Error recovery values: the server-verify exit-21 refusal names `link` / `unlink` beside the six earlier frame verbs (chunk 2026-10-26-explicit-session-links).'
    rationale: 'The report''s Counts moved: CLI frame verbs six -> eight; each of `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink` takes the server check through `cmd::client::connect` and exits 21 `instance-unreachable` with the server-verify hint on a mismatch (Symbols / APIs step 5); line 19 still enumerates six.'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:77
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 19
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'Append to the line-19 item: "`viola link` / `viola unlink <driver> <driven>` (chunk 2026-10-26-explicit-session-links) run the strict-modes check on the driven instance (`check_instance`) before any log opens or any snapshot value is read; a refused check is exit 21 with the fixed instance-state `unable` / `hint:` pair (`human::instance_state_pair`, the pair `viola list` prints), the hint last, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout with stderr empty; a driven instance that is not running is exit 21 with the not-running hint last; a wrapper error (a `driver` equal to `driven`, refused `-32602` by the wrapper only) is a fault, exit 20, with no hint."'
    sidecar: 'cognitive-accessibility.md Error recovery values: the `link` / `unlink` refusals (strict-modes and not-running exit 21 with the hint last, the `--json` document, the self-pair fault exit 20) recorded (chunk 2026-10-26-explicit-session-links).'
    rationale: 'The report''s Symbols / APIs (new verbs, `link_call` steps 1, 3 and 6) and Expected amendments (a11y-plan §8 Error recovery: the verbs'' server-verify and strict-modes refusals) bring two new refusal paths and one fault path to the CLI error-recovery pattern, none of which line 19 states.'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:41
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 26
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'In the server-verify verification clause, "the six frame verbs'' exit-21 server-verify refusal" becomes "the eight frame verbs'' exit-21 server-verify refusal", and its read list "over `send`, `wait`, `last`, `answer`, `pause` and `release` in both modes" becomes "over `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` in both modes"; the rest of the clause (hint last, no ESC byte, stdout empty, the `--json` document, no pid / path / endpoint, the control, the macOS gate) stands.'
    sidecar: 'cognitive-accessibility.md Error recovery verification: the identity table reads eight frame verbs, `link` / `unlink` added (chunk 2026-10-26-explicit-session-links).'
    rationale: 'The report''s Counts moved names `a11y-plan-detail/cognitive-accessibility.md:26` stale under `frameverbs` / `idtable` ("the six" verbs): `tests/security_negatives_server_identity.rs` gained the `link` / `unlink` rows in both modes.'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:121
    dependent-of: D-a11y-surface
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 26
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'Add to the cli verification list: "`viola link` / `viola unlink`''s own refusals, by the root cases in `tests/cli_links.rs`: `link_verbs_refuse_a_widened_instance_before_any_log` (`cfg(unix)`: a group-writable driven instance refused exit 21 before any log, the mode read back as found after the refusal) and `link_verbs_against_a_stopped_instance_exit_21` (exit 21 with the not-running hint), chunk 2026-10-26-explicit-session-links."'
    sidecar: 'cognitive-accessibility.md Error recovery verification: the `link` / `unlink` strict-modes and not-running refusals'' root cases named (chunk 2026-10-26-explicit-session-links).'
    rationale: 'The report''s Outcome: (security) group-writable driven instance (Unix) refused before any log, mode as found, met by `link_verbs_refuse_a_widened_instance_before_any_log` (ubuntu, macOS); (design) stopped instance exit 21 not-running hint, met by `link_verbs_against_a_stopped_instance_exit_21`.'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:305
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 36
    section: '## 8. Cognitive Accessibility › ### Plain language'
    change: 'Add to the literal-pinned exceptions: "and `viola link` / `viola unlink`''s three result lines (`link    <driver> -> <driven>  since <HH:MM:SSZ>` · `link    <driver> -> <driven>  already linked` · `unlink  <driver> -> <driven>`, plain in every mode, each one write), pinned by literal in the units `link_verb_lines_are_the_design_lines_in_one_write` and `link_since_is_the_utc_time_alone` (`src/human/verbs.rs`) and the root case `link_verbs_print_the_design_lines` (`tests/cli_links.rs`), chunk 2026-10-26-explicit-session-links."'
    sidecar: 'cognitive-accessibility.md Plain language verification: the three `link` / `unlink` result lines join the literal-pinned fixed strings (chunk 2026-10-26-explicit-session-links).'
    rationale: 'The report''s Symbols / APIs (three result lines, `src/human/verbs.rs:57-89`), Coverage of new surfaces (the result lines'' unit and integ cases) and Expected amendments (a11y-plan §8 Plain language: the result lines among the literal-pinned strings).'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:60
note: 'D-a11y-obs-schema not fired (no a11y violation or obs log schema change; the product `unlink` line is a kind already in the closed set); no D-retired hit in this file (it never stated the link hold); the body''s Plain language verification (a11y-plan.md:273) names the literal-pinned exceptions too and would gain the link result lines for the body detector to judge.'
