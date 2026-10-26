proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 8
    section: '## 1. A11y Scope Summary'
    change: 'In the CLI verbs entity, the server-verify sentence reads "`send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` each refuse, before any frame, an endpoint whose serving process is not the recorded wrapper: …" (eight frame verbs, chunk 2026-10-26-explicit-session-links), the rest of the sentence unchanged.'
    sidecar: 'a11y-scope-summary scope-entities: the server-verify refusal names the eight CLI frame verbs, `link` and `unlink` added (chunk 2026-10-26-explicit-session-links).'
    rationale: 'The report counts the CLI frame verbs six → eight (`link`, `unlink`), each taking the liveness pre-check and the server check through `cmd::client::connect` before its frame, exit 21 with the server-verify hint on a mismatch (report.md:51-55, :77-79, :94).'
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 8
    section: '## 1. A11y Scope Summary'
    change: 'Add to the CLI verbs entity: `viola link <driver> <driven> [--json]` and `viola unlink <driver> <driven> [--json]` (served since chunk 2026-10-26-explicit-session-links; the driven instance is the verb''s) print one plain-ASCII result line in one write with no SGR in any mode, or under `--json` one `{"v":1,"ok":…}` document; they refuse with exit 21 a driven instance that fails the strict-modes check before any log opens (the fixed instance-state `unable` / `hint:` pair, or under `--json` `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty), a stopped instance (the not-running hint last) and a server that is not the recorded wrapper (as above); a wrapper `-32602` (e.g. `driver` equal to `driven`) is a fault, exit 20, no hint.'
    sidecar: 'a11y-scope-summary scope-entities: the link verbs'' output modes and exit-21 refusals recorded as built (chunk 2026-10-26-explicit-session-links).'
    rationale: 'The report builds the two verbs: `check_instance` first (exit 21, the fixed pair or the `--json` document), the liveness pre-check (not-running hint), the server check, the three plain result lines in one write and `{"v":1,"ok":<ok>}` under `--json`, a wrapper error exit 20 (report.md:38-66); the entity names `link` / `unlink` with no mode text.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 56
    section: '## 1. A11y Scope Summary'
    change: 'Beside the C0/C1 escaping rule add: a `viola link` line whose `since` is no RFC 3339 instant (the restore''s `unknown`, or a value from a hand-altered log) prints that `since` through `escape_message`, which keeps `\n`, `\t` and non-ASCII (chunk 2026-10-26-explicit-session-links).'
    sidecar: 'a11y-scope-summary scope-surfaces: the link line''s non-RFC 3339 `since` goes through `escape_message`, keeping `\n` / `\t` / non-ASCII (chunk 2026-10-26-explicit-session-links).'
    rationale: 'The report records `link_since` printing a `since` that is no RFC 3339 instant through `escape_message`, which keeps `\n`, `\t` and non-ASCII, reached only from the restore''s `unknown` or a hand-altered log (report.md:63-65, :287-289).'
    fact: F7
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 76
    section: '## 1. A11y Scope Summary'
    change: 'The path-6 required-roles line reads that the `--help` group labels stand since chunk 2026-10-17-cli-output-discipline as board / traffic / wheel / handoff / setup over the served verbs, the `handoff:` row (`handoff:  link, unlink`, between `wheel:` and `setup:`) built by chunk 2026-10-26-explicit-session-links; the clause "the `handoff:` row joins with «Explicit session links»" is removed.'
    sidecar: 'a11y-scope-summary path-p6-board: the `handoff:` help row recorded built, its owed clause retired (chunk 2026-10-26-explicit-session-links).'
    rationale: 'The report adds `handoff:  link, unlink` between `wheel:` and `setup:` (report.md:67, :313-314) and names a11y-scope-summary.md:76 stale under the owner pattern `title` (report.md:133-137, :227).'
    basis: .andromeda/a11y-plan-detail/a11y-scope-summary.md:76
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan.md
    near: 104
    section: '## 1. A11y Scope Summary'
    change: 'Path 6 Required ARIA roles ends "… and the `--help` group labels (board / traffic / wheel / handoff / setup)", the clause "whose `handoff:` row is owed to «Explicit session links»" removed.'
    sidecar: 'a11y-plan §1 path 6: the `handoff:` help row no longer owed (chunk 2026-10-26-explicit-session-links).'
    rationale: 'The broad line the path-p6-board group details; the report builds the row (report.md:67, :313-314) and names a11y-plan.md:104 stale (report.md:136-137, :227).'
    basis: .andromeda/a11y-plan.md:104
    dependent-of: D-cascade
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/aria-patterns-roles.md
    near: 10
    section: '## 4. ARIA Patterns & Roles'
    change: 'In the cli row the `--help` rows read `board:` · `traffic:` · `wheel:` · `handoff:` · `setup:` over the served verbs only, the `handoff:` row (`handoff:  link, unlink`, between `wheel:` and `setup:`) built by chunk 2026-10-26-explicit-session-links and pinned by `help_is_the_grouped_verb_table` and `help_lists_only_served_verbs` (`tests/cli_output_plain.rs`); the closing clause "the `handoff:` row is owned by «Explicit session links»" is removed.'
    sidecar: 'aria-patterns-roles aria-landmarks: the `handoff:` help row built between `wheel:` and `setup:`, its owner clause retired (chunk 2026-10-26-explicit-session-links).'
    rationale: 'The report adds the row `handoff:  link, unlink` between `wheel:` and `setup:`, met by `help_is_the_grouped_verb_table` and `help_lists_only_served_verbs` (report.md:67, :313-314), and names aria-patterns-roles.md:10 stale (report.md:137, :227).'
    basis: .andromeda/a11y-plan-detail/aria-patterns-roles.md:10
    fact: F8
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 19
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'The server-verify sentence reads "CLI `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` refuse an endpoint whose serving process is not the recorded wrapper with exit 21: …", the rest unchanged.'
    sidecar: 'cognitive-accessibility cognitive-errors: the server-verify refusal names the eight CLI frame verbs (chunk 2026-10-26-explicit-session-links).'
    rationale: 'The CLI frame verbs are now eight, each refusing a server mismatch with exit 21 and the server-verify hint (report.md:51-55, :77-79, :94-97).'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 19
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'Add: `viola link` / `viola unlink` refuse a driven instance that fails the strict-modes check with exit 21 before any log opens: the fixed instance-state `unable` / `hint:` pair, `hint:` last, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout with stderr empty (chunk 2026-10-26-explicit-session-links).'
    sidecar: 'cognitive-accessibility cognitive-errors: the link verbs'' strict-modes refusal, exit 21, the fixed pair or the `--json` document (chunk 2026-10-26-explicit-session-links).'
    rationale: 'Step 1 of `link_call` runs `check_instance` before `client::start`; refused → exit 21, `human::instance_state_pair(driven)` through `human::refuse`, or the `--json` document with stderr empty, no log line (report.md:41-46, :271).'
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 19
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'Add: against a stopped driven instance `viola link` / `viola unlink` exit 21 with the not-running hint as the last stderr line (`link_verbs_against_a_stopped_instance_exit_21`, chunk 2026-10-26-explicit-session-links).'
    sidecar: 'cognitive-accessibility cognitive-errors: the link verbs'' stopped-instance refusal, exit 21, not-running hint last (chunk 2026-10-26-explicit-session-links).'
    rationale: 'The liveness pre-check finds no endpoint → exit 21 through `client::unreachable` with the not-running hint last, met by `link_verbs_against_a_stopped_instance_exit_21` (report.md:48-49, :311-312).'
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 19
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'Beside the faults sentence add: a pair the wrapper refuses as invalid params (`-32602`, e.g. `driver` equal to `driven`) reaches `viola link` / `viola unlink` as a fault, exit 20, no hint; it is never refused client-side (chunk 2026-10-26-explicit-session-links).'
    sidecar: 'cognitive-accessibility cognitive-errors: a wrapper `-32602` on the link verbs is a fault, exit 20, no hint (chunk 2026-10-26-explicit-session-links).'
    rationale: 'A wrapper error, `-32602` included, goes through `client::fault`, exit 20; `driver` equal to `driven` is refused by the wrapper only, never client-side (report.md:56-59, :276-277).'
    fact: F5
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 26
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'The cli verification reads "the eight frame verbs'' exit-21 server-verify refusal, by the `HINT` literal in `tests/security_negatives_server_identity.rs`, whose `security_negative_cli_verb_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper` reads, over `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` in both modes, …", the rest unchanged.'
    sidecar: 'cognitive-accessibility cognitive-errors procedure: the identity table reads the eight frame verbs (chunk 2026-10-26-explicit-session-links).'
    rationale: 'The identity table grows from six verbs × two modes to eight × two plus the controls (19 → 25 cases), and the report names cognitive-accessibility.md:26 stale under `frameverbs` / `idtable` (report.md:94-97, :121-124, :228-229).'
    basis: .andromeda/a11y-plan-detail/cognitive-accessibility.md:26
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 26
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'Add to the cli verification: the link verbs'' exit-21 refusals (chunk 2026-10-26-explicit-session-links) are read by `link_verbs_against_a_stopped_instance_exit_21` (the not-running hint last) and `link_verbs_refuse_a_widened_instance_before_any_log` (a group-writable driven instance refused before any log, its mode read back as found; Unix only, ubuntu-latest and macos-latest), both in `tests/cli_links.rs`.'
    sidecar: 'cognitive-accessibility cognitive-errors procedure: the link verbs'' stopped-instance and strict-modes refusals and their witnesses (chunk 2026-10-26-explicit-session-links).'
    rationale: 'The report lists both cases among `tests/cli_links.rs`''s new ones and their outcomes: stopped instance exit 21 with the not-running hint, widened instance refused before any log with the mode as found on ubuntu and macOS (report.md:273-275, :305-306, :311-312).'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 36
    section: '## 8. Cognitive Accessibility › ### Plain language'
    change: 'Add to the literal-pinned strings: the `viola link` / `viola unlink` result lines `link    <driver> -> <driven>  since <HH:MM:SSZ>`, `link    <driver> -> <driven>  already linked` and `unlink  <driver> -> <driven>`, each one write with no SGR in any mode, pinned by literal in the unit `link_verb_lines_are_the_design_lines_in_one_write` (`src/human/verbs.rs`) and `link_verbs_print_the_design_lines` (`tests/cli_links.rs`) (chunk 2026-10-26-explicit-session-links).'
    sidecar: 'cognitive-accessibility cognitive-plain-language: the three link result lines join the literal-pinned strings (chunk 2026-10-26-explicit-session-links).'
    rationale: 'The report adds `write_linked`, `write_already_linked`, `write_unlinked`, each one `write_all`, plain in every mode, pinned by the verbs.rs unit and `link_verbs_print_the_design_lines` (report.md:60-63, :245-248, :310-311).'
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 36
    section: '## 8. Cognitive Accessibility › ### Plain language'
    change: 'Beside the link lines add: the `link` line''s time is the reply''s `since` as UTC clock time in whole seconds with `Z` (pinned by `link_since_is_the_utc_time_alone`), and the names printed are the verb''s own parsed arguments, never names read back from the reply or the log (chunk 2026-10-26-explicit-session-links).'
    sidecar: 'cognitive-accessibility cognitive-plain-language: the link line''s time shape and name source (chunk 2026-10-26-explicit-session-links).'
    rationale: 'The report states `link_since` renders `since` as UTC clock time in whole seconds with `Z`, the printed names the parsed `ViolaName` arguments, and lists the unit `link_since_is_the_utc_time_alone` (report.md:63-66, :245-246).'
    fact: F2
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan.md
    near: 273
    section: '## 8. Cognitive Accessibility › ### Plain language'
    change: 'The verification reads "every fixed CLI string is pinned, by trycmd or, for `viola list`, `viola --help`, `viola plugin install`, the server-verify hint and the `viola link` / `viola unlink` result lines, by literal, …", the rest unchanged.'
    sidecar: 'a11y-plan §8 Plain language: the link result lines among the literal-pinned strings (chunk 2026-10-26-explicit-session-links).'
    rationale: 'The broad line the cognitive-plain-language group details; the report pins the three result lines by literal (report.md:60-63, :245-248) and expects §8 Plain language amended (report.md:225-226).'
    dependent-of: D-new-detail
note: F6 (the link verbs' `--json` `ok` documents) fits no picked entity — none of the picked cognitive-accessibility groups holds success output — so it is left to the new-detail pass; detectors D-a11y-surface and D-a11y-obs-schema did not fire (no interactive element, no schema change).
