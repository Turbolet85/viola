proposals:
  - detector: D-design-cli-wording
    severity: warning
    section: 'Surface: cli → Output structure — `viola --help` (the As built paragraph)'
    change: 'As built, `viola --help` prints the usage line, a blank line, then five rows: `board:    list` · `traffic:  send, wait, last, answer` · `wheel:    pause, release` · `handoff:  link, unlink` · `setup:    run, revive, verify, plugin install`. The handoff row sits between `wheel:` and `setup:` and was built by chunk 2026-10-26-explicit-session-links. Drop the sentence "the wireframe''s `handoff:  link, unlink` row joins with "Explicit session links"", because the wireframe and the As built rows now match. The rest of the paragraph (served verbs only, `help_lists_only_served_verbs`, the hidden `mcp`, the `HELP` literal pin) stays.'
    sidecar: 'layout-templates §Output structure — `viola --help` As built: four rows → five, the `handoff:  link, unlink` row built between `wheel:` and `setup:` (chunk 2026-10-26-explicit-session-links); the pending "joins with Explicit session links" sentence retired.'
    rationale: 'In the report, Changes → Symbols / APIs says "`viola --help` gains the row `handoff:  link, unlink` between `wheel:` and `setup:`", and Retired names the hold on the verbs. Outcome (layouts) shows it met by `help_is_the_grouped_verb_table`. The report flags `layout-templates.md:219` as stale under the owner pattern `title` and under `handoff`.'
    basis: 'src/cmd/mod.rs:180-189'
  - detector: D-retired
    severity: warning
    section: 'Surface: cli → Primary screens (commands)'
    change: 'Rewrite the `viola --help` bullet: it is the grouped verb table with the served verbs only, holding the board, traffic, wheel, handoff (`link, unlink`; chunk 2026-10-26-explicit-session-links) and setup rows, with `setup:` holding `plugin install`. Remove "the `handoff:` row joins when "Explicit session links" serves its verbs".'
    sidecar: 'layout-templates §Primary screens: the `--help` bullet names the built `handoff:` row; the pending clause on "Explicit session links" retired (chunk 2026-10-26-explicit-session-links).'
    rationale: 'Retired: the hold on `viola link` / `viola unlink` and every master line stating it. The report lists `layout-templates.md:61` as stale under the owner pattern `title` and under `handoff`. Symbols / APIs: the help row is built.'
    dependent-of: D-design-cli-wording
  - detector: D-retired
    severity: warning
    section: 'Surface: cli → Component — Primary navigation (verb structure)'
    change: 'Discoverability bullet: `--help` groups verbs as board / traffic / wheel / handoff / setup, and as built prints all five groups over the served verbs, with `setup:` holding `plugin install` and `handoff:` holding `link, unlink` (chunk 2026-10-26-explicit-session-links). Remove "prints board / traffic / wheel / setup" and "the handoff group joining when its route entry serves its verbs". The hint and `release` sentences stay.'
    sidecar: 'layout-templates §Primary navigation Discoverability: the as-built help groups gain `handoff:` and the pending join clause is retired (chunk 2026-10-26-explicit-session-links).'
    rationale: 'Retired names the hold on the explicit verbs, and Symbols / APIs reports the `handoff:  link, unlink` row as built. This bullet repeats the retired pending-handoff claim in other words, so it is a duplicate of the `--help` As built occurrence.'
    dependent-of: D-design-cli-wording
  - detector: D-design-cli-wording
    severity: warning
    section: 'Surface: cli → Output structure — wheel, handoff and dialog verbs'
    change: 'Add an As built paragraph (chunk 2026-10-26-explicit-session-links). `viola link <driver> <driven> [--json]` and `viola unlink <driver> <driven> [--json]` act on the driven instance. Each prints one stdout line in one write, plain in every mode, exit 0. The lines are `link    <driver> -> <driven>  since <HH:MM:SSZ>` (the reply''s `since` as UTC clock time in whole seconds; a `since` that is no RFC 3339 instant, such as the restore''s `unknown`, prints escaped), `link    <driver> -> <driven>  already linked` for a pair already held, and `unlink  <driver> -> <driven>` whether or not a pair was removed. The printed names are the verb''s own arguments, never names read back from the reply. Under `--json` the verbs print `{"v":1,"ok":{"since":<ts>,"already_linked":<bool>}}` and `{"v":1,"ok":{}}`. A wrapper error, including a `driver` equal to `driven` (`-32602`), is the wrapper-fault line with exit 20.'
    sidecar: 'layout-templates §Output structure — wheel, handoff and dialog verbs: As built paragraph for the `link` / `unlink` result lines, the `since` time form, the `--json` ok documents and the exit-20 self-pair (chunk 2026-10-26-explicit-session-links).'
    rationale: 'Changes → Symbols / APIs, "Three result lines": `write_linked` / `write_already_linked` / `write_unlinked`, plain in every mode, `since` as UTC `HH:MM:SSZ`, and a non-RFC 3339 `since` through `escape_message`. Step 6 covers the `--json` `{"v":1,"ok":<ok>}` document and the exit-20 wrapper error. The wireframe shows the lines, but no master states their as-built clause.'
    basis: 'src/human/verbs.rs:57-89'
  - detector: D-design-cli-wording
    severity: warning
    section: 'Surface: cli → Component — Primary content block 2: refusal lines and the `unable` column (Hint line bullet)'
    change: 'The server-check verb list becomes the eight frame verbs: "a frame verb (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`) whose server check finds that the process on the endpoint is not the recorded wrapper…". The `unable  <name>  instance-unreachable` line, the server-verify hint and the `--json` exit-21 document stay as written, with `<name>` the driven instance for `link` / `unlink`.'
    sidecar: 'layout-templates §Primary content block 2 Hint line: the server-verify refusal''s frame-verb list grows from six to eight (`link`, `unlink`; chunk 2026-10-26-explicit-session-links).'
    rationale: 'Changes → Symbols / APIs step 5: a server that is not the recorded wrapper gets no frame and the verb exits 21 `instance-unreachable` with the server-verify hint. Changes → Counts: the CLI frame verbs go from six to eight (pattern `frameverbs`), and Expected amendments ask for the refusal block''s verb lists.'
    basis: 'src/cmd/link.rs:85-136'
  - detector: D-design-cli-wording
    severity: warning
    section: 'Surface: cli → Component — Primary content block 2: refusal lines and the `unable` column (Hint line bullet)'
    change: 'The not-running exit-21 sentence names `viola link` / `viola unlink` beside `wait` / `last`. With no live driven instance, they print `unable  <driven>  instance-unreachable` and then `hint: <driven> is not running; viola list shows the live instances` as the last stderr line, exit 21.'
    sidecar: 'layout-templates §Primary content block 2 Hint line: `link` / `unlink` join `wait` / `last` under the not-running exit-21 pair (chunk 2026-10-26-explicit-session-links).'
    rationale: 'Changes → Symbols / APIs step 3 says that when the liveness pre-check finds no live endpoint, the verb exits 21 through `client::unreachable` / `client::unable_unreachable` (the not-running hint last). Outcome (design) shows "stopped instance → exit 21 not-running hint" as met by `link_verbs_against_a_stopped_instance_exit_21`.'
    basis: 'src/cmd/link.rs:113-115'
  - detector: D-design-cli-wording
    severity: warning
    section: 'Surface: cli → Component — Primary content block 2: refusal lines and the `unable` column (Line form bullet, the fixed-message exceptions)'
    change: 'Add `viola link` / `viola unlink`''s exit-21 strict-modes refusal of the driven instance to the fixed-message exceptions. It is checked before any role file or log opens, and stdout is empty. It prints the fixed instance-state pair (`revive`''s pair, also `list`''s per-instance pair): `unable: <driven>''s state files can be written by another user` / `hint: viola will not read them; make the viola home and its files owner-only`. No line is written to any log and the directory''s mode is left as found. Under `--json` it prints `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty.'
    sidecar: 'layout-templates §Primary content block 2 Line form: the `link` / `unlink` exit-21 strict-modes refusal joins the fixed-message exceptions with the instance-state pair (chunk 2026-10-26-explicit-session-links; the founder''s word `inputs#I3`).'
    rationale: 'Changes → Symbols / APIs step 1: `check_instance` runs before `client::start`, and a refusal is exit 21 with human `human::instance_state_pair(driven)` through `human::refuse` (the fixed instance-state pair) or `--json` `{"v":1,"error":"instance-unreachable","detail":null}`, with no line in any log. Decisions: exit 21 as `list` / `plugin install`, not `revive`''s exit 1.'
    basis: 'src/cmd/link.rs:98-106'
  - detector: D-design-cli-wording
    severity: warning
    section: 'Surface: cli → Component — Primary content block 2: refusal lines and the `unable` column (Line form bullet, the catch-site verb list)'
    change: 'The list "A failed or panicked `cli` verb (`send`, `wait`, `last`, `list`, `answer`, `pause`, `release`, `verify`, `revive --list`, `plugin install`; …)" gains `link` and `unlink`. For these two, `error: internal error` with exit 1 also covers a `Refused` reply or an `ok` of another shape.'
    sidecar: 'layout-templates §Primary content block 2: the `error: internal error` catch-site verb list gains `link` / `unlink` (chunk 2026-10-26-explicit-session-links).'
    rationale: 'Changes → Symbols / APIs step 6: a `Refused` or an `ok` of another shape → internal error, exit 1. The `Command` gains `Link` / `Unlink` dispatched with `cli_sink`.'
    basis: 'src/cmd/mod.rs:79-82'
  - detector: D-design-cli-wording
    severity: warning
    section: 'Surface: cli → Component — Primary content block 2: refusal lines and the `unable` column (Exit codes bullet)'
    change: 'Exit 21: `unable  instance-unreachable` (for `viola list`, `viola plugin install` and the strict-modes refusal of `viola link` / `viola unlink`, the fixed state pair above instead).'
    sidecar: 'layout-templates §Primary content block 2 Exit codes: exit 21''s fixed-state-pair exceptions gain the `link` / `unlink` strict-modes refusal (chunk 2026-10-26-explicit-session-links).'
    rationale: 'Changes → Symbols / APIs step 1: the verbs'' refused `check_instance` is exit 21 with the fixed instance-state pair. Decisions: exit 21 as `list` / `plugin install`. This bullet''s exception list repeats the Line form claim.'
    dependent-of: D-design-cli-wording
  - detector: D-design-cli-wording
    severity: warning
    section: 'Surface: cli → Signature placement'
    change: 'The paragraph after the two components says the word half of the box is the first-word column of every other verb''s refusal line. Its verb list becomes `viola answer`, `viola pause`, `viola release`, and `viola wait` / `viola last` / `viola link` / `viola unlink` for exit 21.'
    sidecar: 'layout-templates §Signature placement: `link` / `unlink`''s exit-21 `unable` lines join the shared word-column list (chunk 2026-10-26-explicit-session-links).'
    rationale: 'Changes → Symbols / APIs steps 3 and 5: both verbs refuse with exit 21 `instance-unreachable` through the shared `unable` refusal line. This paragraph lists by name the verbs whose refusal line uses the `unable` column, so it is a duplicate of the Hint line occurrence.'
    dependent-of: D-design-cli-wording
note: 'D-layout-surface did not fire. The new verbs'' output (`link` / `unlink` lines) and the `handoff:` help row were already wireframed in §Output structure — wheel, handoff and dialog verbs and §Output structure — `viola --help`, and `layout-templates.md:215` and `:322` read true per the report. The footer example and §Primary navigation''s argument order also read true. Neither the `-32601` nor the `dead_code` retired items appear in this doc.'
