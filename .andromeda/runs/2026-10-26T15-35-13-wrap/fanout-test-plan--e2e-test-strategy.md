proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 226
    section: '## 6. E2E Test Strategy › #### Scenario: E3 — `link` / `unlink`'
    change: '- **As landed (chunks 2026-10-10-session-links and 2026-10-26-explicit-session-links): landed.** The explicit verbs `viola link` / `viola unlink` and the wrapper''s `link` / `unlink` methods are built, in the founder''s form (`inputs#I3` of chunk 2026-10-26-explicit-session-links: the driven instance''s `check_instance` first, then the liveness pre-check and the server check). Steps 1 and 3 landed in `e3_two_links_and_a_send_leave_one_link_record` (`tests/cli_links.rs`, `binary(cli_links)`, from a stamped home), the driver a self-reported name and not booted: re-linking adds no `link` record, and the `unlink` appends one product `unlink` record (`source: wrapper`, `{driver, driven}`); `since` is asserted by shape only (`is_clock_time`), because `LinkSet::line` stamps it with `Utc::now()` outside the clock seam. Beside it: `links_an_unlink_survives_a_wrapper_restart`, `link_verbs_print_the_design_lines`, `link_verbs_against_a_stopped_instance_exit_21`, `link_verbs_log_the_channel_pair_and_no_names` and, on Unix, `link_verbs_refuse_a_widened_instance_before_any_log`, green on the three CI runners (ci#40347680974). The implicit link stays held by `links_a_send_from_another_instance_links_once`, `links_an_answer_from_another_instance_links`, `links_a_refused_send_and_an_own_name_send_record_none` and `links_survive_a_wrapper_restart` (run 38098238731).'
    sidecar: 'E3 as-landed rewritten: the explicit link/unlink hold, the -32601 answer and the no-product-unlink sentence retired; steps 1 and 3 landed in e3_two_links_and_a_send_leave_one_link_record and its neighbour cases named (chunk 2026-10-26-explicit-session-links).'
    rationale: 'The report Retired bullet names the hold, the -32601 answer for link/unlink and LinkSet::remove having no caller; its Counts list e2e-test-strategy.md:226 stale under the title and killline patterns, and its Expected amendments say E3 steps 1 and 3 landed with the driver a self-reported name, not booted.'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:86
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 234
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'Replace "The link leg landed at chunk 2026-10-10-session-links, on the implicit link alone (no `viola link` verb exists; the explicit verbs are not built):" with "The link leg landed at chunk 2026-10-10-session-links, on the implicit link:", and after the closing `links_survive_a_wrapper_restart` sentence add "Since chunk 2026-10-26-explicit-session-links, `links_an_unlink_survives_a_wrapper_restart` (`tests/cli_links.rs`) holds a product `unlink` across a restart through the same start restore."'
    sidecar: 'E5 link leg: the retired "no viola link verb exists / explicit verbs not built" parenthetical removed; the unlink-across-restart case added (chunk 2026-10-26-explicit-session-links).'
    rationale: 'The report Expected amendments flag e2e-test-strategy.md:234 stale ("no `viola link` verb exists"); the verbs are built and the Outcome names links_an_unlink_survives_a_wrapper_restart for restart restore.'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:209
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 305
    section: '## 6. E2E Test Strategy'
    change: 'Replace "runs `send`, `wait`, `last`, `answer`, `pause` and `release`, each under `--json` and in human mode" with "runs the eight CLI frame verbs `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` (the last two since chunk 2026-10-26-explicit-session-links, after their passing `check_instance`), each under `--json` and in human mode"; the rest of the bullet unchanged.'
    sidecar: 'Server identity negative: six frame verbs to eight (link, unlink added) per chunk 2026-10-26-explicit-session-links.'
    rationale: 'The report Counts move CLI frame verbs six to eight and tests/security_negatives_server_identity.rs from six verbs x two modes + control to eight x two + the controls; this bullet still enumerates the six.'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:121
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 306
    section: '## 6. E2E Test Strategy'
    change: 'Replace "runs each of the six verbs, in both modes" with "runs each of the eight verbs, in both modes".'
    sidecar: 'Server identity control: six verbs to eight (chunk 2026-10-26-explicit-session-links).'
    rationale: 'Same moved count as the primary: the control now runs the eight frame verbs (report Counts, idtable 19 to 25 cases).'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:121
    dependent-of: D-tests-count-literals
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 270
    section: '## 6. E2E Test Strategy'
    change: 'Replace "its Linux and Windows case is the six-verb server-identity negative" with "its Linux and Windows case is the eight-verb server-identity negative".'
    sidecar: 'Exit-cause matrix server-verification cause: six-verb negative to eight-verb (chunk 2026-10-26-explicit-session-links).'
    rationale: 'Same moved count: the CLI frame verbs are eight and the identity table runs them all (report Symbols / APIs and Counts).'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:77
    dependent-of: D-tests-count-literals
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 298
    section: '## 6. E2E Test Strategy'
    change: 'Append to the bullet: "As landed (chunk 2026-10-26-explicit-session-links): `tests/channel_links.rs` `channel_link_names_are_validated_by_the_wrapper` (`binary(channel_links)`, on the `ci` profile''s 20 s stamped-home kill line, not a `security_negatives_*.rs` file) writes its raw frames for both methods through `viola_channel::Client::request` (it adds `v` and `sender` and makes no server check, so the frame reaches the wrapper past every CLI check); each bad name, a `driven` that is not the serving wrapper''s own name and a `driver` equal to `driven` get `-32602` with `data: null` and no `link` / `unlink` record, beside a control frame."'
    sidecar: 'Raw-channel link/unlink name negative: as-landed record added (channel_link_names_are_validated_by_the_wrapper, binary(channel_links) on the 20 s kill line).'
    rationale: 'The report Expected amendments name e2e-test-strategy.md:298 as the raw-channel negative now built; Outcome (security) cites channel_link_names_are_validated_by_the_wrapper; deviation 4 records the Client::request route; Harness adds binary(channel_links) to the kill line.'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:301
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 268
    section: '## 6. E2E Test Strategy'
    change: 'Add an Exit 21 cause after this one: "    - the driven instance fails `check_instance` at `viola link` / `viola unlink` (run before `client::start`, so before any role file or log opens): exit 21 with the fixed instance-state `unable:` / `hint:` pair, under `--json` `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty, no line in any log and the directory''s mode as found; as landed (chunk 2026-10-26-explicit-session-links) `tests/cli_links.rs` `link_verbs_refuse_a_widened_instance_before_any_log` (Unix, the mode read after the refusal), not a `cross_exit_causes.rs` row"'
    sidecar: 'Exit-cause matrix: the link/unlink instance strict-modes refusal (exit 21) added with its landed case.'
    rationale: 'The report Symbols / APIs step 1 and Decisions (card lean 5) make the two verbs'' instance-check refusal exit 21, not revive''s exit 1; Outcome (security) cites link_verbs_refuse_a_widened_instance_before_any_log.'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:41
note: 'Lines 305, 306 and 270 enumerate or count the six frame verbs but are not in the report''s idtable stale list (it names only test-scope-summary.md:12 for tests); proposed on the moved count itself.'
