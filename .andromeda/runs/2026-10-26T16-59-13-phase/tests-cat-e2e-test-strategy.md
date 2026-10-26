# tests · E2E Test Strategy — category read

## Carried
- (.andromeda/test-plan-detail/e2e-test-strategy.md:28) Path 1's tamper-and-rewrite steps are the end-to-end witness of the pinned-binary re-hash and the per-start plugin rewrite this chunk owns; any change to the start's home copy (the `pin_exe` race) must keep them green.
  > 5. Tamper and rewrite: stop the first wrapper (Ctrl-C into the outer PTY, `child.wait()`). Overwrite one byte of the file at snapshot `pinned_bin`, and write a sentinel into the instance's `plugin/` hooks.json and `settings.json`. Then run `viola run builder --home H` again.
  >   6. Restore the pinned exe by deleting it. Run again, and read the plugin files.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:34) The tampered pinned exe must exit 1 and a restart must rewrite `plugin/` with absolute pinned-path hook commands (M6).
  > The duplicate `run` exits 1. The squatted endpoint exits 1. The tampered pinned exe exits 1.
  >   - After restart, `plugin/` hooks.json no longer contains the sentinel, and its hook commands are the absolute `pinned_bin` path, not a PATH lookup (M6).
- (.andromeda/test-plan-detail/e2e-test-strategy.md:36) The per-start rewrite covers the Unix `settings.json` override, whole, 0600, shell-plain path only; none on Windows.
  > On Unix the restart rewrites the instance's `settings.json` whole at 0600 as `{"statusLine":{"type":"command","command":"<pinned path> hook statusline"}}` with the absolute pinned path, and only when every character of that path is an ASCII letter, a digit or one of `_ - . / :`. On Windows no `settings.json` is written.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:37) The landed Path 1 flow is the binary a start-order change in this chunk must keep green, on its 20 s kill line.
  > steps 1–6 run as one ordered flow over one stamped home in the root path binary `tests/path1_run_start.rs`, `path1_run_start_one_home_in_order(stamped_home)` (`[[test]]` `required-features = ["fake-agent"]`; in the E2E layer, `nextest-e2e`; on the `ci` profile's verify-driven 20 s kill line), every signal above asserted
- (.andromeda/test-plan-detail/e2e-test-strategy.md:174) The statusline arm's strict-modes refusal must run no user command and leave the marker file unchanged.
  > Step 6: exit 0 with empty stdout, and the marker file is unchanged. `statusline_command` never runs when strict-modes fails.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:182) The landed witness of Path 6 step 6 stands as the pattern (and the regression guard) for a check-before-open on a widened home.
  > Step 6 is `hook_statusline_a_home_another_user_can_write_runs_nothing_however_often` in `tests/hook_statusline.rs` (`cfg(unix)`, two invocations, no role line added).
- (.andromeda/test-plan-detail/e2e-test-strategy.md:193) Path 7's unstamped single-instance boot is the test-side workaround CARRY 16 names for the `pin_exe` race; the chunk's home-copy fix must not depend on it.
  > `agent-run boot --session p7 --unstamped --instance builder` (no stamps at all).
- (.andromeda/test-plan-detail/e2e-test-strategy.md:237) The readers' move onto the replay is owed to this route entry (CARRY 19, the founder's P4 fork); if ruled in, E5 steps 1–3 become buildable.
  > Steps 1–3 stay unbuilt while `run`'s collision check calls `read_snapshot`; the three other snapshot readers (`read_snapshot(` under the root `src/` outside test modules: `run`'s collision check, `src/cmd/hook.rs` once, `src/cmd/hook/statusline.rs`) stay on `read_snapshot`, and their move onto the replay (`read_snapshot_or_replay`), with what a client does with an unreadable or newer snapshot, is one CARRY on the route entry "Home and code-bearing file integrity".
- (.andromeda/test-plan-detail/e2e-test-strategy.md:237) If the move is built, the entry must restate E5's two signals set against what landed.
  > Two of its signals are set against what landed, for the entry that builds it to restate: every start appends `wheel{holder:"driver", cause:"start"}` (as measured at `tests/chaos_torn_append.rs`, the second start's first record), so `wheel:"human"` does not hold after step 3; and the replay writes no file, so no snapshot is rebuilt from it.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:246) The security sweep must cover the `--home` strict-modes refusals at every entry point this chunk adds.
  > The sweep also covers the `--home` strict-modes refusals.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:248) A strict-modes check added before the hook's snapshot read must keep the impostor rows' `server-verify-failed` verdict reachable over their test-built snapshot.
  > driving `stop`, `pre-tool-use` and `permission-request` against a same-user server on the recorded endpoint while the snapshot records another live process (the fake agent's `--hold-stdout-internal` holder): each exits 0 with empty streams under the 1.0 s `Instant` bound, no frame reaches the server, and one `hook-decision` line carries `detail:"server-verify-failed"`
- (.andromeda/test-plan-detail/e2e-test-strategy.md:260) Every `diagnostics/` file stays 0600 on Unix after every E2E test.
  > `diagnostics/` files are 0600 on Unix
- (.andromeda/test-plan-detail/e2e-test-strategy.md:268) New strict-modes refusal cases go in the binary of their verb, not a `cross_exit_causes.rs` row.
  > As landed, `cross_exit_causes.rs` does not exist and no rstest table joins the causes: each built cause is a case of the binary its row names.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:270) The strict-modes home failure is an exit-21 cause for the CLI frame verbs, Windows by a synthetic-descriptor stub.
  > the home fails strict-modes (Unix 0770 home; Windows synthetic-descriptor stub)
- (.andromeda/test-plan-detail/e2e-test-strategy.md:271) The landed `link`/`unlink` case is the shape every newly checked verb must match: widened instance, exit 21 both modes, no log line, no frame, mode as found, with a remove-the-guard control proving the check precedes the role-file open.
  > `link_verbs_refuse_a_widened_instance_before_any_log` (`cfg(unix)`) makes the driven instance's directory group-writable while its wrapper runs; both verbs, in both modes, exit 21 with the fixed instance-state `unable:` / `hint:` pair or the `instance-unreachable` document with stderr empty, the verb's `cli` log gains no line, the wrapper gets no frame, and the directory's mode reads back as found. A check placed after the role file opens would pass it, because opening a role file narrows an existing home to 0700; remove-the-guard control A, the check moved, read red 1 of 1
- (.andromeda/test-plan-detail/e2e-test-strategy.md:274) The `--json` document of a strict-modes exit 21 is every exit-21 cause's one document.
  > its `--json` document is every exit-21 cause's `{"v":1,"error":"instance-unreachable","detail":null}`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:281) A pinned exe failing its re-hash is an exit-1 cause of `run`.
  > the pinned exe fails its SHA-256 re-hash (Path 1 step 5)
- (.andromeda/test-plan-detail/e2e-test-strategy.md:283) A `--home` failing strict-modes is an exit-1 cause of `run`.
  > the `--home` fails strict-modes at `run`
- (.andromeda/test-plan-detail/e2e-test-strategy.md:284) The revive strict-modes cases fix the assertion shape a run-side check must also hold: no wrapper-log line, no child, the home's mode unchanged, read while the process still runs.
  > The strict-modes cases assert exit 1, the `unable:` / `hint:` pair and no line added to `run-<name>.ndjson` (chunk 2026-10-10-budget-governor); the case over a group-writable home also asserts no child and the home's mode unchanged, read while the revive still runs, because a revive whose check wrongly passes runs on as a wrapper and never exits.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:303) The Windows `--home` outside `%USERPROFILE%` negative must create the home with a protected user + SYSTEM DACL, read back.
  > `viola run` with a not-yet-existing `--home` under `RUNNER_TEMP` (outside `%USERPROFILE%`) creates it with a protected DACL whose ACEs are exactly the user and SYSTEM (read back with `GetNamedSecurityInfoW`).
- (.andromeda/test-plan-detail/e2e-test-strategy.md:303) The `RUNNER_TEMP` negative case is owed to this route entry, its home under `target/e2e-home`, and a pre-existing inheriting directory must be refused exit 1.
  > the `RUNNER_TEMP` negative case as worded here stays owed to that route entry. A Windows DACL test creates its home where the CI homes live (`target/e2e-home`), never under `%TEMP%`, which lies inside the profile. A pre-existing directory there that inherits a broader ACL (for example `BUILTIN\Users` read) is refused with exit 1.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:304) Owner-only modes must hold under `umask 000`, the pinned binary 0700.
  > **Umask-independent modes** (`#[cfg(unix)]`). Start `viola run` under `umask 000` (via `sh -c 'umask 000; exec viola run …'`). `events.ndjson`, `snapshot.json` and the other home files are still 0600, directories 0700, and the pinned `bin/<version>-<hash>/viola` is 0700 and never group- or world-writable.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:305) The single-writers negative is the proof of "only the wrapper writes `snapshot.json`, only `viola verify` writes `ledger/stamps.json`".
  > **Single writers** (security anti-patterns). Record the bytes and mtime of `ledger/stamps.json` and `snapshot.json`. With the wrapper stopped (SIGSTOP on Unix; on Windows the wrapper is suspended with the fake agent idle), run every other writer-capable entry point: every `hook <event>` including `hook statusline`, `list`, `wait` / `last` / `send` (which then fail with exit 21) and `mcp` tool calls. Neither file changes. `stamps.json` changes only when `viola verify` runs.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:309) The eight-verb server-identity negative runs over a test-built snapshot; the strict-modes check added ahead of each verb's snapshot read must keep its exit-21 verdict the server-verification cause's, and its control (line 310) reaching the recorded wrapper.
  > runs `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`, each under `--json` and in human mode, against a same-user server bound on the recorded endpoint while `snapshot.json` records another live process
- (.andromeda/test-plan-detail/e2e-test-strategy.md:362) The pinned-exe content hash stays pinned to `sha2` by literal vectors.
  > `tests/contract_content_hash.rs` pins the pinned-exe content hash (§1 "SHA-256 16-hex re-hash") to its library, `sha2` (a root dev-dependency): `sha256_published_vector_matches_digest` over the three FIPS 180-2 vectors and `sha256_truncated_to_eight_bytes_yields_sixteen_hex` over the 8-byte / 16-hex truncation, literal oracles, on all three OSes.
- (.andromeda/test-plan.md:427) The `seed_conpty` carve-out is the one CARRY 6 / CARRY 13 must resolve: a seeded home would skip viola's creation-time DACL.
  > Two carve-outs: Windows x64 starts seed the ConPTY companions before viola runs, and `tests/chaos_feed_panic.rs` alone boots outside the scans.
- (.andromeda/test-plan.md:477) Foreign-owner and non-persistent-ACL Windows cases stay synthetic-descriptor stubs, not E2E cases.
  > Non-persistent-ACL volumes and foreign-owner SIDs. Covered by synthetic SDDL descriptor stubs (§4).
- (.andromeda/test-plan.md:488) Path 1's broad signal binds the re-hash and the rewrite.
  > a duplicate run, a squatted endpoint and a tampered pinned exe each exit 1; a restart rewrites `plugin/` with absolute hook commands, never a PATH lookup, and on Unix the `settings.json` override.
- (.andromeda/test-plan.md:528) Path 6's broad signal binds the statusline strict-modes refusal.
  > `statusline_command` never runs when strict-modes fails, and the refusal hint never names `release`.
- (.andromeda/test-plan.md:590) E5 steps 1–3 stay unbuilt under option 1; the move is owed to this route entry.
  > steps 1–3 stay unbuilt while `run`'s collision check calls `read_snapshot`: every snapshot reader stays on `read_snapshot` (option 1), and their move onto `read_snapshot_or_replay` stays a CARRY owed to the route entry "Home and code-bearing file integrity".
- (.andromeda/test-plan.md:606) The security control negatives list names this chunk's three (the Windows `--home` DACL, umask-independent modes, single writers).
  > a Windows `--home` outside the profile given a protected DACL (its test home never under `%TEMP%`), umask-independent modes, single writers of `stamps.json` and `snapshot.json`
- (.andromeda/test-plan.md:631) The seed exceptions (the `seed_conpty` companions, the impostors' test-built `snapshot.json`) meet the strict-modes check this chunk adds; each must be one the check reads as intended.
  > except the chaos tests' corruption of files the product already wrote, the Windows x64 `seed_conpty` companions and the server-identity impostors' test-built `snapshot.json`; a case that needs a home viola created starts from `stamped_home`.

## Bindings
- **Path 1 tamper/rewrite ↔ security.md §Local trust boundary (re-hash before reuse, `plugin/` rewritten atomically on every start)**: the E2E steps are the test side of the security rule the working entry names, (.andromeda/test-plan-detail/e2e-test-strategy.md:28), (.andromeda/test-plan-detail/e2e-test-strategy.md:34)
- **E5 replay move ↔ the scope's Boundaries (CARRY 19, the founder's P4 fork)**: neither planned in nor out until his answer; if in, E5 steps 1–3 and the two restated signals land with it, (.andromeda/test-plan-detail/e2e-test-strategy.md:237), (.andromeda/test-plan.md:590)
- **Exit-21 strict-modes cause ↔ obs-plan §10 (a refused check writes no line) and the Exit-cause catalogue entry**: the case asserts no `cli` log line and the one `instance-unreachable` document; a new hint or code is "Exit-cause code catalogue"'s, (.andromeda/test-plan-detail/e2e-test-strategy.md:271), (.andromeda/test-plan-detail/e2e-test-strategy.md:274)
- **Windows `RUNNER_TEMP` negative ↔ §5/§7 `seed_conpty` carve-out (CARRY 6, CARRY 13)**: a home a test seeds first meets the creation-time DACL the negative asserts and the inheriting-ACL refusal, (.andromeda/test-plan-detail/e2e-test-strategy.md:303), (.andromeda/test-plan.md:427), (.andromeda/test-plan.md:631)
- **Server-identity negatives ↔ §7 seed exception (the impostors' test-built `snapshot.json`)**: the strict-modes check now ahead of the server check must pass that snapshot or the server-verify verdict becomes unreachable, (.andromeda/test-plan-detail/e2e-test-strategy.md:248), (.andromeda/test-plan-detail/e2e-test-strategy.md:309)
- **Single writers ↔ §11 Test Data ban on hand-written `stamps.json` / `snapshot.json`**: the negative proves the product side of the same writer rule the ban holds for tests, (.andromeda/test-plan-detail/e2e-test-strategy.md:305)
- **Umask-independent modes ↔ security.md home rules (0700 dirs, 0600 files, pinned exe 0700)**: (.andromeda/test-plan-detail/e2e-test-strategy.md:304)
- **Path 6 step 6 ↔ security.md statusline arm (check before anything under the home is opened)**: (.andromeda/test-plan-detail/e2e-test-strategy.md:174), (.andromeda/test-plan-detail/e2e-test-strategy.md:182)
