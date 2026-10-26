# tests · Integration Test Strategy — category read

## Carried
- (.andromeda/test-plan-detail/integration-test-strategy.md:42) The two channel methods, once served, must be covered by the crate channel suite and a root `tests/channel_*.rs` file (the scope's `tests/channel_links.rs`), beside the other methods.
  > `send`/`wait`/`last`/`answer`/`pause`/`release`/`link`/`unlink`/`hook.dialog`/`hook.event` via `crates/viola-channel/tests/` and root `tests/channel_*.rs`
- (.andromeda/test-plan-detail/integration-test-strategy.md:43) The `link` result `{since, already_linked}`, the `unlink` empty-object result and their `-32602` errors must be schema-validated, not compared ad hoc.
  > result, refusal and error shapes schema-validated
- (.andromeda/test-plan-detail/integration-test-strategy.md:45) A `link` or `unlink` frame with a higher `params.v` must be answered `-32602`, held by a case like every other method.
  > higher `params.v` gives -32602
- (.andromeda/test-plan-detail/integration-test-strategy.md:47) The two verbs write a frame, so a server that is not the recorded wrapper must give them CLI exit 21 on Linux and Windows, with the case in the server-identity negatives file; macOS remains the recorded gap.
  > a server pid / start-time mismatch versus `snapshot.json` gives CLI exit 21 and the hook exits 0 with no body, on Linux and Windows: the CLI half in `tests/security_negatives_server_identity.rs`, the hook half the impostor rows of `tests/hook_fail_open.rs` (§6 Security sweep). macOS reports no peer pid, so there the check passes with no comparison, the recorded gap;
- (.andromeda/test-plan-detail/integration-test-strategy.md:52) Re-linking an existing pair must append no `link` line (the served `already_linked` arm), asserted at the channel level.
  > linking an existing pair adds no log line
- (.andromeda/test-plan-detail/integration-test-strategy.md:54) The `link` and `unlink` kinds are already among the 16 round-tripped kinds with `data` keys `driver` and `driven`; the served `unlink` method must write that same shape and add no kind.
  > `state_events.rs`: the 16 kinds (the length of `EventKind::ALL`; 13 until `link` and `unlink` joined at chunk 2026-10-10-session-links, each with the `data` keys `driver` and `driven`, 15 until `dialog-expired` joined at chunk 2026-10-19-dialog-hold-for-the-driver with the `data` keys `dialog_id` and `dialog_kind`) round-trip through the crate's writer and reader with zero on all three counts
- (.andromeda/test-plan-detail/integration-test-strategy.md:54) The replay's link cases already cover a pair keeping its `since` until an `unlink`. The `unlink` product path must append lines the replay reads that way.
  > since chunk 2026-10-10-session-links two link cases: a pair keeps its first `since` until an `unlink` and takes a later one after it, and a snapshot cut short is replayed with the logged link.
- (.andromeda/test-plan-detail/integration-test-strategy.md:57) If the founder picks the instance-check form, a refused strict-modes check on `link` / `unlink` must exit CLI 21, the code for every verb other than `run` / `mcp` / `revive`.
  > hook 0 with no output (the statusline arm reads and opens nothing under the home after a refused check and writes no line, however often it is invoked), CLI 21 for every other verb
- (.andromeda/test-plan-detail/integration-test-strategy.md:18) The channel cases must run over a real endpoint bound in a temp home, with frames validated by jsonschema.
  > Wrapper channel: real endpoint bound by `viola-channel` server in a temp home, with frames validated by jsonschema.
- (.andromeda/test-plan-detail/integration-test-strategy.md:32) Each link / unlink test must start from its own fresh tempfile home.
  > there is a fresh tempfile home per test (the rstest `home` fixture).
- (.andromeda/test-plan-detail/integration-test-strategy.md:33) Under the instance-check form, a case that drives the check-guarded verb path on every OS must start from `stamped_home`, because a seeded home fails the instance check on Windows. The booted `overseer` / `builder` wrappers must sit in the kill-and-wait `Drop` guard.
  > Such a home fails `hook statusline`'s instance check on Windows (as measured at chunk 2026-10-10-statusline-pass-through, CI run 38080061631, three cases refused; the home's DACL not read), so a case that drives a check-guarded path on every OS starts from `stamped_home` (§7 Seed strategies).
- (.andromeda/test-plan-detail/integration-test-strategy.md:33) No wrapper or state may be shared between the link cases.
  > Wrappers started in a test are held in a `Drop` guard that calls `Child::kill()` then `wait()`. Nothing is shared between tests.
- (.andromeda/test-plan-detail/integration-test-strategy.md:64) Each exit code of `viola link` / `viola unlink --json` must have its `--json` document covered by a root `tests/cli_*.rs` case.
  > root `tests/cli_*.rs` covers the `--json` document per exit code
- (.andromeda/test-plan-detail/integration-test-strategy.md:65) The `handoff:  link, unlink` help row (the third CARRY) must land by moving the `HELP` literal, not through a trycmd case.
  > `viola --help` (the grouped verb table) is pinned by the `HELP` literal in `tests/cli_output_plain.rs` (`help_is_the_grouped_verb_table`), not by a trycmd case (chunk 2026-10-17-cli-output-discipline).
- (.andromeda/test-plan-detail/integration-test-strategy.md:72) `help_lists_only_served_verbs` currently asserts that `link` and `unlink` are absent from the table. Serving the verbs must change it so that each answers `--help` exit 0 and appears in the table; until the verbs are served, the absence must hold.
  > `help_is_the_grouped_verb_table` (the whole stdout as the `HELP` literal, exit 0, stderr empty) and `help_lists_only_served_verbs` (each served verb path answers `--help` exit 0, `plugin install --help` included, its usage program read as `viola` or `viola.exe` followed by the verb's words; the table's words read from the group rows alone, `plugin install` one entry of `setup:`; no `hook`, `help`, `link`, `unlink` or bare `plugin` in the table).
- (.andromeda/test-plan-detail/integration-test-strategy.md:71) The three human result lines must carry no escape sequence in plain modes.
  > no `\x1b[` under non-TTY / `NO_COLOR` / `TERM=dumb`
- (.andromeda/test-plan-detail/integration-test-strategy.md:81) As driver-facing verbs, `link` / `unlink` must join the machine-contract tables: one document and a literal exit over a wrapper's life, over a home with no instance, and over a home another user could write.
  > `tests/cli_machine_contract.rs` reads each driver-facing verb's `--json` document and literal exit over one live wrapper's life (`every_verb_prints_one_document_and_its_literal_exit_over_a_wrapper_s_life`, a labelled case table per world), over a home with no instance (`verbs_over_a_home_with_no_instance_print_their_documents`) and over a home another user could write (`verbs_over_a_home_another_user_could_write_print_their_documents`),
- (.andromeda/test-plan-detail/integration-test-strategy.md:81) A test-started `viola link` / `unlink` must drop the inherited `VIOLA_*` triple. A case that needs `VIOLA_NAME` (E3 step 2's `send`) must set it per child.
  > A test-started `viola` drops `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN` (`tests/support/home.rs` `UNWRAPPED`) and a path oracle compares `resolved(home)`, never `TestHome::path()`.
- (.andromeda/test-plan.md:406) The raw-channel name negatives and the schema checks must use the named drivers: the raw stream for malformed frames, and validators built from the product's schemars schemas.
  > **Wrapper channel:** the `viola-channel` sync client, a raw interprocess 2.4.4 `local_socket::Stream` for malformed frames, and jsonschema 0.57.0 validators built from the product's schemars 1.2.2 schemas.
- (.andromeda/test-plan.md:408) The three result lines must be pinned either by a trycmd case or by literals, and the help row by the `HELP` literal.
  > **Human-mode layouts:** trycmd 1.2.1 case files, except the `viola list` board, pinned by literal asserts in `tests/cli_list.rs` (chunk 2026-10-11-the-board-viola-list), and `viola --help`, pinned by the `HELP` literal in `tests/cli_output_plain.rs` (chunk 2026-10-17-cli-output-discipline).
- (.andromeda/test-plan.md:435) The two new methods fall under "every method" of the channel pattern, server verification included.
  > **Wrapper channel:** every method through the crate and root channel suites, with schema-validated result, refusal and error shapes, the `cursor` equal to the log length at acceptance, `-32602` on a higher `v`, `-32600` and a close on an oversize frame, server verification, and wrapper-side paste validation in which `control-character` wins over every other refusal.
- (.andromeda/test-plan.md:445) The CLI pattern binds the verbs' `--json` documents, their human layouts and their machine-contract rows.
  > **CLI:** root `tests/cli_*.rs` cover the `--json` document per exit code, the human layouts by trycmd or literal with no `\x1b[` under non-TTY / `NO_COLOR` / `TERM=dumb`, `viola verify`'s step lines and its stop cleanup, `plugin install`, leading-slash argv never delivered as a prompt, no setting that disables a control, and the home resolution order and machine contract; `contract_ledger_probes.rs` holds the ledger-probe contract per stamped fixture set.
- (.andromeda/test-plan.md:868) The method shapes and the `-32602` refusals must be held at the §5 channel level, and E3 must assert only cross-process behaviour.
  > NEVER assert at E2E a case that a lower layer can hold. Parser, refusal-order, threshold and clock cases belong in §4, contract shapes in §5, and §6 asserts only cross-process behaviour (§2 pyramid).
- (.andromeda/test-plan.md:886) Link state must be asserted against real files in a tempfile home, never a mock.
  > NEVER mock the on-disk state (the arch has no DB; this is the DB-equivalent rule). Use a real tempfile home.
- (.andromeda/test-plan.md:889) The channel link tests must take the endpoint only from `snapshot.json`.
  > NEVER use interprocess `try_overwrite`, or reuse an endpoint name across tests. Take the endpoint only from `snapshot.json`.
- (.andromeda/test-plan.md:890) The `VIOLA_NAME=overseer` of E3 step 2 must be set through `Command::env`.
  > NEVER use `std::env::set_var` in tests. It is `unsafe` in edition 2024 and races across threads; use `Command::env` per child.
- (.andromeda/test-plan.md:891) A link test that clears the environment must keep coverage.
  > NEVER call `.env_clear()` without re-adding `LLVM_PROFILE_FILE`, because that run silently drops out of coverage.
- (.andromeda/test-plan.md:106) The chunk adds no MCP tool: `tools/list` must stay exactly five, never `link` / `unlink`.
  > `tools/list` is exactly the five tools, never `release` / `pause` / `link` / `unlink`, and negative inputs must violate the advertised schema, which is advisory.

## Bindings
- **channel `link` / `unlink` suite ↔ §6 Scenario E3 and Security control negatives**: the channel-level shapes, `-32602` and no-line-on-relink cases sit at §5 (.andromeda/test-plan-detail/integration-test-strategy.md:42), (.andromeda/test-plan-detail/integration-test-strategy.md:52). E3's steps 1 and 3 (.andromeda/test-plan.md:556) and the raw-channel `link` / `unlink` name validation (.andromeda/test-plan.md:606) stay at §6. Pyramid placement is (.andromeda/test-plan.md:868).
- **server-identity negative ↔ security-plan §Authentication & Authorization (the server check before any frame) and the scope's crossing (b)**: the verbs take `client::connect`'s check, and their impostor case joins `tests/security_negatives_server_identity.rs` (.andromeda/test-plan-detail/integration-test-strategy.md:47).
- **strict-modes CLI 21 / `stamped_home` ↔ security-plan strict-modes and the founder's P4 form (option two, `check_instance` as `viola revive` takes it)**: (.andromeda/test-plan-detail/integration-test-strategy.md:57), (.andromeda/test-plan-detail/integration-test-strategy.md:33). This binds only if that form is named.
- **help row literal ↔ layout-templates §Output structure — `viola --help` (the third CARRY's `handoff:` row between `wheel:` and `setup:`)**: `HELP` and `help_lists_only_served_verbs` move together (.andromeda/test-plan-detail/integration-test-strategy.md:65), (.andromeda/test-plan-detail/integration-test-strategy.md:72).
- **link / unlink event kinds and replay ↔ architecture [Links] (the standing Part A shapes) and `LinkSet::remove`'s product caller**: (.andromeda/test-plan-detail/integration-test-strategy.md:54).
- **machine contract ↔ `verification-matrix.json#v1-13` (`--json` carrying the same facts as the human line)**: (.andromeda/test-plan-detail/integration-test-strategy.md:81), (.andromeda/test-plan.md:445).
- **MCP tool list ↔ scope Boundaries (no `link` / `unlink` MCP tool)**: (.andromeda/test-plan.md:106).
