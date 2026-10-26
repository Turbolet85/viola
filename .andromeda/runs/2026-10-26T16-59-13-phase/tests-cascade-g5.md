# tests · cascade read · group 5 of 6

## Carried
- (.andromeda/test-plan-detail/e2e-test-strategy.md:165) The statusline arm over a group-writable home is a `viola hook` entry point whose strict-modes check this chunk makes whole on every OS, so Path 6 step 6 must stay a driven case of it.
  > 6. Chmod the home to 0770 (Unix) and pipe statusline stdin again.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:174) The chunk must keep `statusline_command` from running whenever the home fails strict-modes, with exit 0 and empty stdout, at the check it places before any home open.
  > Step 6: exit 0 with empty stdout, and the marker file is unchanged. `statusline_command` never runs when strict-modes fails.
  (.andromeda/test-plan.md:528)
  > `statusline_command` never runs when strict-modes fails, and the refusal hint never names `release`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:182) The step-6 witness is `cfg(unix)` only, so the chunk's Windows proof of the statusline arm's check (and the three cases a seeded home failed on `windows-2025`) is new work beside it in `tests/hook_statusline.rs`, keeping its no-role-line rule.
  > Step 6 is `hook_statusline_a_home_another_user_can_write_runs_nothing_however_often` in `tests/hook_statusline.rs` (`cfg(unix)`, two invocations, no role line added).
- (.andromeda/test-plan-detail/e2e-test-strategy.md:193) The unstamped Path 7 boot is the harness's one-instance workaround of the `pin_exe` race this chunk owns, so any fix to the home copy of the binary must keep it booting and may let a two-instance unstamped boot be measured again.
  > 1. `agent-run boot --session p7 --unstamped --instance builder` (no stamps at all).
  (.andromeda/test-plan-detail/e2e-test-strategy.md:196)
  > 4. In a fresh home, run `viola verify --home H3` against the fake agent at a fixture-stamped version.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:199) The hook's added strict-modes check must leave the hook inside the spine deadline with empty stdout, and `verify` over a fresh home must still end in its stamped line once `update_stamps` reads `ledger/stamps.json` under strict-modes.
  > The `question` event is logged once. Hook stdout is empty, with exit 0 and returning within the spine deadline.
  (.andromeda/test-plan-detail/e2e-test-strategy.md:201)
  > `verify` exits 0 with a last line matching `^stamped \S+  \d+ pass  0 fail$`.
  (.andromeda/test-plan.md:536)
  > `viola verify` at a fixture-stamped version exits 0 with its `stamped <ver>  <n> pass  0 fail` line.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:237) E5 steps 1–3 and the snapshot readers' move onto `read_snapshot_or_replay` are this entry's CARRY, built only if the founder's P4 answer moves the readers; if built, the two signals set against what landed must be restated and the classified read taken only after `check_instance`.
  > Steps 1–3 stay unbuilt while `run`'s collision check calls `read_snapshot`; the three other snapshot readers (`read_snapshot(` under the root `src/` outside test modules: `run`'s collision check, `src/cmd/hook.rs` once, `src/cmd/hook/statusline.rs`) stay on `read_snapshot`, and their move onto the replay (`read_snapshot_or_replay`), with what a client does with an unreadable or newer snapshot, is one CARRY on the route entry "Home and code-bearing file integrity". A replayed snapshot carries no `cwd`, so a revive over one is refused `cwd-missing`, and a revived start's first record is `wheel{holder:"driver", cause:"start"}` too. Two of its signals are set against what landed, for the entry that builds it to restate: every start appends `wheel{holder:"driver", cause:"start"}` (as measured at `tests/chaos_torn_append.rs`, the second start's first record), so `wheel:"human"` does not hold after step 3; and the replay writes no file, so no snapshot is rebuilt from it.
  (.andromeda/test-plan.md:590)
  > every snapshot reader stays on `read_snapshot` (option 1), and their move onto `read_snapshot_or_replay` stays a CARRY owed to the route entry "Home and code-bearing file integrity".
- (.andromeda/test-plan-detail/e2e-test-strategy.md:246) The hook fail-open matrix must gain a row whose `VIOLA_DIR` home fails strict-modes (exit 0, no body, empty streams), and the sweep's `--home` strict-modes refusals must cover every entry point the chunk adds.
  > The hook fail-open matrix cases are oversize stdin, malformed JSON, clap error, channel failure, forced panic, missing `VIOLA_NAME` and, on Linux and Windows, a server that is not the recorded wrapper (`server-verify-failed`). The sweep also covers the `--home` strict-modes refusals.
  (.andromeda/test-plan-detail/e2e-test-strategy.md:248)
  > each exits 0 with empty streams under the 1.0 s `Instant` bound, no frame reaches the server, and one `hook-decision` line carries `detail:"server-verify-failed"`
  (.andromeda/test-plan-detail/e2e-test-strategy.md:244)
  > its name is a row in the `cli_controls_not_disableable.rs` table.
  (.andromeda/test-plan.md:598)
  > plus the `--home` strict-modes refusals; the panic seam fires only under the `fake-agent` feature and never reaches the release build.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:257) Every strict-modes refusal the chunk adds must carry no absolute path in its `--json`, MCP or channel error, and the owner-only home must keep every `diagnostics/` file 0600 on Unix.
  > CLI `--json`, MCP `isError` and channel `error.data` contain no absolute path (drive letter, `/home/`, `/Users/`, the temp home prefix), no serde path and no `Caused by`
  (.andromeda/test-plan-detail/e2e-test-strategy.md:260)
  > `diagnostics/` files are 0600 on Unix
  (.andromeda/test-plan.md:600)
  > `diagnostics/` files 0600 on Unix
- (.andromeda/test-plan-detail/e2e-test-strategy.md:270) Each frame verb, `run` and the pinned-exe re-hash gain or keep their strict-modes and hash causes (exit 21 / exit 1), each case beside the remove-the-guard control showing a check after the role-file open would pass a 0770 home, as the `link` / `unlink` case already proves.
  > the home fails strict-modes (Unix 0770 home; Windows synthetic-descriptor stub)
  (.andromeda/test-plan-detail/e2e-test-strategy.md:271)
  > A check placed after the role file opens would pass it, because opening a role file narrows an existing home to 0700; remove-the-guard control A, the check moved, read red 1 of 1 (as measured at `viola-0.1.0/chunks/2026-10-26-explicit-session-links/evidence/remove-the-guard.md`).
  (.andromeda/test-plan-detail/e2e-test-strategy.md:281)
  > the pinned exe fails its SHA-256 re-hash (Path 1 step 5)
  (.andromeda/test-plan-detail/e2e-test-strategy.md:283)
  > the `--home` fails strict-modes at `run`
  (.andromeda/test-plan.md:488)
  > a duplicate run, a squatted endpoint and a tampered pinned exe each exit 1; a restart rewrites `plugin/` with absolute hook commands, never a PATH lookup, and on Unix the `settings.json` override.
  (.andromeda/test-plan.md:477)
  > Non-persistent-ACL volumes and foreign-owner SIDs. Covered by synthetic SDDL descriptor stubs (§4).
- (.andromeda/test-plan-detail/e2e-test-strategy.md:303) The Windows `RUNNER_TEMP` negative is owed to this entry, and umask-independent modes and single writers are the owner-only home and single-writer rules it builds; the server-identity impostors' home must pass the new check so their refusal stays the server-verify cause.
  > **Windows `--home` outside `%USERPROFILE%`** (`#[cfg(windows)]`, `~/.viola/` access control). `viola run` with a not-yet-existing `--home` under `RUNNER_TEMP` (outside `%USERPROFILE%`) creates it with a protected DACL whose ACEs are exactly the user and SYSTEM (read back with `GetNamedSecurityInfoW`). As landed (chunk 2026-10-04-dialog-answers-by-dialog-id, the creation half pulled forward from the route entry "Home and code-bearing file integrity" by the founder's live ruling): `viola_state::fs::create_private_dir` gives the topmost folder it creates outside the profile the protected `D:P(A;OICI;FA;;;<user>)(A;OICI;FA;;;SY)` DACL and fails the creation when it cannot be set, covered by unit tests on both OSes (Windows on CI) and the integration `tests/cli_version_gate.rs`; the `RUNNER_TEMP` negative case as worded here stays owed to that route entry. A Windows DACL test creates its home where the CI homes live (`target/e2e-home`), never under `%TEMP%`, which lies inside the profile. A pre-existing directory there that inherits a broader ACL (for example `BUILTIN\Users` read) is refused with exit 1.
  (.andromeda/test-plan-detail/e2e-test-strategy.md:304)
  > **Umask-independent modes** (`#[cfg(unix)]`). Start `viola run` under `umask 000` (via `sh -c 'umask 000; exec viola run …'`). `events.ndjson`, `snapshot.json` and the other home files are still 0600, directories 0700, and the pinned `bin/<version>-<hash>/viola` is 0700 and never group- or world-writable.
  (.andromeda/test-plan-detail/e2e-test-strategy.md:305)
  > **Single writers** (security anti-patterns). Record the bytes and mtime of `ledger/stamps.json` and `snapshot.json`. With the wrapper stopped (SIGSTOP on Unix; on Windows the wrapper is suspended with the fake agent idle), run every other writer-capable entry point: every `hook <event>` including `hook statusline`, `list`, `wait` / `last` / `send` (which then fail with exit 21) and `mcp` tool calls. Neither file changes. `stamps.json` changes only when `viola verify` runs.
  (.andromeda/test-plan-detail/e2e-test-strategy.md:309)
  > against a same-user server bound on the recorded endpoint while `snapshot.json` records another live process (the fake agent's `--hold-stdout-internal` holder): exit 21 with the server-verification cause's `--json` document (stderr empty), hint and `process-exit` line (Exit-cause matrix), no `channel-request` line, no frame reaching the server, and no pid, start time, path or endpoint in any output or line (`digit_runs`).
  (.andromeda/test-plan.md:606)
  > a Windows `--home` outside the profile given a protected DACL (its test home never under `%TEMP%`), umask-independent modes, single writers of `stamps.json` and `snapshot.json`,
- (.andromeda/test-plan-detail/e2e-test-strategy.md:362) The re-hash before reuse of `bin/<version>-<hash>/viola` rests on the truncated SHA-256 these literal vectors pin, so the case must keep holding on all three OSes over whatever hash path the chunk touches.
  > `tests/contract_content_hash.rs` pins the pinned-exe content hash (§1 "SHA-256 16-hex re-hash") to its library, `sha2` (a root dev-dependency): `sha256_published_vector_matches_digest` over the three FIPS 180-2 vectors and `sha256_truncated_to_eight_bytes_yields_sixteen_hex` over the 8-byte / 16-hex truncation, literal oracles, on all three OSes.
  (.andromeda/test-plan.md:617)
  > and the pinned-exe content hash is pinned to its library by literal vectors.

## Bindings
- **e2e strict-modes refusals ↔ integration on-disk (§5)**: the exit-21 / exit-1 / hook-0 causes carried above are the same per-surface refusal the integration layer states, `run` / `mcp` / `revive` 1, hook 0, every other verb 21 (.andromeda/test-plan.md:437), triggered by `--home` / `VIOLA_DIR` failing strict-modes (.andromeda/test-plan.md:201).
- **e2e single writers and impostor homes ↔ test data (§7)**: the single-writer negative and the server-identity impostors stand on the test-data ban and its carve-outs (chaos corruption, `seed_conpty`, the impostors' test-built `snapshot.json`), a viola-made home starting from `stamped_home` (.andromeda/test-plan.md:631); the seeded-home move the chunk makes must keep that carve-out true.
- **e2e Windows strict-modes cause ↔ mocking (§8)**: the "Windows synthetic-descriptor stub" exit-21 cause and the foreign-SID / non-persistent-ACL cases are synthetic SDDL descriptors fed to the check function, never a mocked filesystem (.andromeda/test-plan.md:707).
- **e2e Windows legs ↔ CI (§9)**: the `RUNNER_TEMP` DACL negative, the statusline arm's Windows proof and `check_instance` over a widened instance tree are proved only on the `windows-2025` runner (.andromeda/test-plan.md:767).
- **e2e hook strict-modes row ↔ the spine deadline**: the fail-open row and Path 7's hook signal keep the hook inside the spine deadline and the 1.0 s `Instant` bound with the added check (.andromeda/test-plan-detail/e2e-test-strategy.md:199), (.andromeda/test-plan-detail/e2e-test-strategy.md:248).
- **E5 replay ↔ the founder's P4 fork**: E5 steps 1–3 are built only on his answer, the classified read after `check_instance` (.andromeda/test-plan-detail/e2e-test-strategy.md:237).
