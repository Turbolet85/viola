proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 76
    section: '## 1. A11y Scope Summary'
    change: 'Line 76 states the `--help` group labels as built: they stand since chunk 2026-10-17-cli-output-discipline (board / traffic / wheel / setup over the served verbs), and the `handoff:` row (`handoff:  link, unlink`, between `wheel:` and `setup:`) stands since chunk 2026-10-26-explicit-session-links; the owed clause "the `handoff:` row joins with "Explicit session links"" is removed.'
    sidecar: 'a11y-scope-summary path-p6-board: the `handoff:  link, unlink` help row built by chunk 2026-10-26-explicit-session-links; its owed-to-"Explicit session links" clause retired.'
    rationale: 'The report Retired bullet retires the hold on the explicit `viola link` / `viola unlink` and every master line stating it; Symbols / APIs: `viola --help` gains `handoff:  link, unlink` between `wheel:` and `setup:`; the owner pattern `title` lists this line stale (Counts, "Explicit session links" row); Outcome (layouts) met by `help_is_the_grouped_verb_table`, `help_lists_only_served_verbs`.'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:67
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 8
    section: '## 1. A11y Scope Summary'
    change: 'In line 8 (piece @1900) the server-verify refusal names the eight CLI frame verbs: "`send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` each refuse, before any frame, an endpoint whose serving process is not the recorded wrapper"; the rest of that sentence (exit 21, stdout empty, the server-verify `hint:` line last, the `--json` `instance-unreachable` document, the macOS gap, codes unchanged) stays as written.'
    sidecar: 'a11y-scope-summary scope-entities: the CLI verbs entity server-verify refusal list grows from six to eight verbs (`link`, `unlink`), chunk 2026-10-26-explicit-session-links.'
    rationale: 'Report Counts: CLI frame verbs six to eight (pattern `frameverbs`); Symbols / APIs: "The CLI frame verbs are now eight" and each takes the server check through `cmd::client::connect` before its frame, a mismatch exit 21 `instance-unreachable` with the server-verify hint (step 5); the line enumerates only six.'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:77
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 10
    section: '## 1. A11y Scope Summary'
    change: 'After line 10, one item under the CLI verbs entity: `  - **As built (chunk 2026-10-26-explicit-session-links):** `viola link <driver> <driven> [--json]` and `viola unlink <driver> <driven> [--json]` run `check_instance` on the driven instance before any log or snapshot read; a refused check is exit 21 with the fixed instance-state `unable` / `hint:` pair on stderr (hint last) and stdout empty, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout with stderr empty; a stopped instance is exit 21 with the not-running hint last; a wrapper error (`-32602` included, `driver` equal to `driven`) is the fault path, exit 20. A served call prints one line in one write, plain in every mode (no SGR): `link    <driver> -> <driven>  since <HH:MM:SSZ>`, `link    <driver> -> <driven>  already linked` or `unlink  <driver> -> <driven>`, the names the verb''s own arguments; under `--json` `{"v":1,"ok":<ok>}`, exit 0.`'
    sidecar: 'a11y-scope-summary scope-entities: `viola link` / `viola unlink` output and refusals as built (strict-modes refusal exit 21 with the instance-state pair, not-running exit 21, fault exit 20, the three plain result lines, the `--json` documents), chunk 2026-10-26-explicit-session-links.'
    rationale: 'Report Symbols / APIs: `link_call` steps 1, 3, 5 and 6 (refusal pair, `--json` document, not-running hint, exit 20 fault) and the three result lines `write_linked` / `write_already_linked` / `write_unlinked`, each one `write_all`, plain in every mode; Coverage of new surfaces: a11y plain ASCII, no SGR, hint last; Outcome (design) met by `link_verbs_print_the_design_lines`, `link_verbs_against_a_stopped_instance_exit_21`. The entity names the verbs but holds none of their output.'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:38
note: 'Line 8 already runs past the 2 000-char entity-shape bound (it prints in two pieces), so the new link / unlink detail goes in its own item after line 10; the body twin of line 76, .andromeda/a11y-plan.md:104 ("whose `handoff:` row is owed to "Explicit session links"", report Counts `title`), is the body''s to retire. D-a11y-surface and D-a11y-obs-schema read clean: the chunk adds no interactive element and changes no a11y or obs log schema.'
