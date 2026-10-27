# Apply — test-plan

## Verdicts
- fanout-test-plan.md 2 · applied · .andromeda/test-plan.md:634#1
- fanout-test-plan--cascade.md 11 · folded into fanout-test-plan.md 2
- fanout-test-plan.md 3 · applied · .andromeda/test-plan.md:382#1
- fanout-test-plan--cascade.md 5 · folded into fanout-test-plan.md 3
- fanout-test-plan--cascade.md 1 · applied · .andromeda/test-plan-detail/test-scope-summary.md:117#1
- fanout-test-plan--cascade.md 2 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:26#1
- fanout-test-plan--cascade.md 3 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:26+#1
- fanout-test-plan--cascade.md 4 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:28+#1
- fanout-test-plan--cascade.md 6 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:29+#1
- fanout-test-plan--cascade.md 7 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:34#1
- fanout-test-plan--cascade.md 8 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:36+#1
- fanout-test-plan--cascade.md 9 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:202+#1
- fanout-test-plan--cascade.md 10 · applied · .andromeda/test-plan-detail/test-data-fixtures.md:10#1
- fanout-test-plan--cascade.md 12 · applied · .andromeda/test-plan-detail/test-data-fixtures.md:59#1
- fanout-test-plan--cascade.md 13 · applied · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:20+#1
- fanout-test-plan.md 1 · applied · .andromeda/registries/contracts/test-plan/5-command-implementation.md:26#1

## Edits
- line: .andromeda/test-plan.md:634
  old: «its refused entry an inherited `S-1-5-32-545` write ACE, as measured on `windows-2025` by»
  new: «its refused entry an inherited write grant to a SID strict-modes does not trust, read by its right whichever SID holds it, as measured on `windows-2025` by»
- line: .andromeda/test-plan.md:382
  old: «, and the stamps and pin strict-modes refusals.»
  new: «, the stamps and pin strict-modes refusals, and the pinned copy's no-clobber placement (`fs::place_private_new`) and the re-hash of a copy another start placed (`pin::place_absent`).»
- line: .andromeda/test-plan-detail/test-scope-summary.md:117
  old: «Windows: its inherited Users write ACE)»
  new: «Windows: an inherited write grant to a SID strict-modes does not trust, whichever SID holds it)»
- line: .andromeda/test-plan-detail/unit-test-strategy.md:26
  old: «its refused entry an inherited Users (`S-1-5-32-545`) write ACE.»
  new: «its refused entry an inherited write grant to a SID strict-modes does not trust, whichever SID holds it (its oracle below).»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:26
  anchor: «  - The instance check `strict::check_instance(home, instance_dir)` reads, each that exists and a folder before what it holds, the home (read rule), `instances/` (write rule), the instance directory (read rule), its `diagnostics/` (read rule), its `snapshot.json` (write rule) and its `events.ndjson` (the write rule on Unix, the read rule on Windows); its call sites (`check_instance(` under `src/`) are three, `src/cmd/hook.rs` `trusted`, `src/cmd/client.rs` `refuse_untrusted` (shared by the eight frame verbs) and `src/cmd/revive.rs`. Unit cases `strict_instance_*`: five widened modes (`cfg(unix)`), a tree viola wrote passes, a home with no instance directory passes, and a path that cannot be statted is unreadable. Three `cfg(unix)` root cases in `tests/cli_revive.rs` hold the refusal: two on `viola revive` (a widened instance tree; and, since chunk 2026-10-10-budget-governor, a group-writable home, refused and left as found) and one on `viola revive --list`; one in `tests/cli_links.rs`, `link_verbs_refuse_a_widened_instance_before_any_log`, holds it on `viola link` / `viola unlink` (§5 On-disk). On Windows the instance tree is covered by `cfg(windows)` cases green on `windows-2025` (as measured at ci#40403389629): a tree granted Users write is refused `Writable` beside a `create_private_dir` tree that passes, a `diagnostics/` granted Users read is refused, and `strict_instance_refuses_a_plain_home_made_before_viola_and_names_the_entry` refuses a home made with `create_dir_all` before viola, its refused entry an inherited Users (`S-1-5-32-545`) write ACE.»
  add: «  - The plain-home case's Windows oracle is the `cfg(windows)` test helper `strict::win::tests::inherited_untrusted_writes(home)`: the allow ACEs that are inherited, carry a `WRITE_RIGHTS` bit and name a SID `trusted_sid` does not trust, so it reads the grant by its right whichever SID holds it; its control, a home `instance_tree()` makes, reads none, so the oracle cannot pass vacuously (green on `windows-2025`, as measured at ci#40439188053).»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:28
  anchor: «  - Stamps and pin: `update_stamps_refuses_a_widened_ledger_and_leaves_it` (`StampsError::StrictModes`, checked before `create_private_dir(ledger/)` and again under the lock, nothing read or written), `stamps_error_messages_are_fixed`, `pin_key_is_the_version_and_the_content_key` and `pin_companions_refuses_a_path_another_user_may_write` (`PinError::StrictModes`, nothing written).»
  add: «  - The pinned copy's placement (`fs::place_private_new`): `place_private_new_places_the_bytes_once_at_the_mode_and_leaves_no_temp` (0700 / 0600 on Unix, the mode set before any byte), `place_private_new_over_a_present_name_is_present_and_leaves_it` (`Placed::Present`, the target byte-identical, no temp left), `place_private_new_under_a_missing_dir_fails`, and the forced-window cases `place_private_new_placed_copy_execs_at_once` (Unix) and `place_private_new_placed_copy_runs_at_once` (Windows) (§5); the lost placement (`pin::place_absent`): `place_absent_over_a_copy_another_start_placed` (the same bytes reused, other bytes `PinError::HashMismatch` with the file byte-identical) and `place_absent_on_a_free_path_writes_the_copy`.»
- after: .andromeda/test-plan-detail/integration-test-strategy.md:29
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · integration -->»
  add: «»
  add: «- **Forced-window witness, filesystem placement** (no PTY spawn; beside the PTY ones above): `place_private_new_placed_copy_execs_at_once` (`crates/viola-state/src/fs.rs`, Unix) execs the placed copy right after a `Placed::Written` placement, through `place_private_new_with`'s `placed` hook; with the guard removed (the copy persisted while its write handle is still open) it is red on the Linux dev host with `ExecutableFileBusy` (raw 26, `ETXTBSY`) and green with the guard (as measured at `viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/red-green.md`), and green on `ubuntu-latest` and `macos-latest`; its Windows form `place_private_new_placed_copy_runs_at_once` is green-only on `windows-2025` (as measured at ci#40439188053), the red leg read on the Linux dev host only.»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/test-plan-detail/integration-test-strategy.md:34
  old: «the refused entry an inherited `S-1-5-32-545` (Users) write ACE (as measured at chunk 2026-10-26-home-and-code-bearing-file-integrity on `windows-2025`: ci#40398601594 for the refusal,»
  new: «the refused entry an inherited write grant to a SID strict-modes does not trust, read by its right whichever SID holds it (as measured on `windows-2025`: ci#40439188053 for the refusal,»
- after: .andromeda/test-plan-detail/integration-test-strategy.md:36
  anchor: «**DA1 answer:** under the sideloaded ConPTY the piped test driver (`tests/support/piped.rs` `Piped`) answers a DA1 query `ESC[c` with `ESC[?1;0c`, as a terminal would; viola sends no DA1 answer of its own.»
  add: «**Runner-inherited grants:** on one image (`windows-2025-vs2026` 20261021.278.1) `windows-2025` runners differ by host in a temp dir's inherited ACL: on one (westus3) Users holds the write, on another (northcentralus) Authenticated Users (`S-1-5-11`, mask `0x001301BF`) holds it and Users read only (as measured at ci#40403389629 and ci#40425866096). An oracle for a runner-inherited grant therefore names no SID: it reads the grant by its right, beside a control that cannot pass vacuously (§4 viola-state, the plain-home oracle).»
- after: .andromeda/test-plan-detail/e2e-test-strategy.md:202
  anchor: «  - `verify` exits 0 with a last line matching `^stamped \S+  \d+ pass  0 fail$`.»
  add: «- **As landed:** the MCP leg is `mcp_tools_path7_unverified_cli_answer` (`crates/viola-e2e/tests/mcp_tools.rs`), which boots the default `overseer` + `builder` pair unstamped (`SessionOptions { unstamped: true, ..SessionOptions::default() }`), so it also witnesses two first starts on one fresh unstamped home both landing (green on all three CI OSes, as measured at ci#40439188053).»
- line: .andromeda/test-plan-detail/test-data-fixtures.md:10
  old: «the refused entry an inherited `S-1-5-32-545` (Users) write ACE (`strict_instance_refuses_a_plain_home_made_before_viola_and_names_the_entry`, as measured at ci#40398601594)»
  new: «the refused entry an inherited write grant to a SID strict-modes does not trust, read by its right whichever SID holds it (`strict_instance_refuses_a_plain_home_made_before_viola_and_names_the_entry`, as measured at ci#40439188053)»
- line: .andromeda/test-plan-detail/test-data-fixtures.md:59
  old: «when it ran}»
  new: «when it ran, and `spawn_os_error` when the spawn of an absolute `command` failed: the raw OS error as an integer, `null` when the OS gives none}»
- after: .andromeda/test-plan-detail/mocking-stubbing-discipline.md:20
  anchor: «| Filesystem faults | none (a real tempfile home, real modes and ACLs) | A widened or read-granted home tree is made on the real file by the §7 widened-test-home helpers and refused by the product's own check, never a stubbed one: on windows-2025 an instance tree granted Users write, or a `diagnostics/` granted Users read, is refused beside a `create_private_dir` tree that passes; the pure verdicts (`windows_read_verdict` among them) take synthetic descriptors at the unit tier; a test that widens a home tree above `bin/<key>/conpty/` starts its home unseeded (§7, a widened seeded home) |»
  add: «| Forced-window test parameters | none (a parameter of the private function under test) | `place_private_new_with`'s `placed` hook (`crates/viola-state/src/fs.rs`), run right after a `Placed::Written` placement, is a test parameter, not a seam: no env read, no feature, no mockall double, and nothing added to the test-seam list |»
- line: .andromeda/registries/contracts/test-plan/5-command-implementation.md:26
  old: «whose Path 7 MCP leg boots an unstamped `builder` alone»
  new: «whose Path 7 MCP leg boots the default `overseer` + `builder` pair unstamped, `SessionOptions { unstamped: true, ..SessionOptions::default() }`»

## Patterns
[[pattern]]
id = "tp-01"
fixed = 'inherited `S-1-5-32-545` write ACE'

[[pattern]]
id = "tp-02"
fixed = 'its inherited Users write ACE'

[[pattern]]
id = "tp-03"
fixed = 'inherited Users (`S-1-5-32-545`) write ACE'

[[pattern]]
id = "tp-04"
fixed = 'an inherited `S-1-5-32-545` (Users) write ACE'

[[pattern]]
id = "tp-05"
fixed = 'the refused entry an inherited `S-1-5-32-545` (Users) write ACE'

[[pattern]]
id = "tp-06"
fixed = 'boots an unstamped `builder` alone'
