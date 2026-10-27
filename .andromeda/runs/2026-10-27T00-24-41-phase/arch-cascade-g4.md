# arch · cascade read · group 4 of 4

## Carried
- (.andromeda/architecture-detail/conventions.md:23) The hash-before-reuse refusal the chunk re-asserts must stay the exit-1 `pinned-hash-mismatch` start refusal with its fixed `unable:` line, on `run`'s path and on verify's, and any refusal the absent-copy fix adds must stay an exit-1 start or verify refusal, no code outside the table.
  > `viola run` refusing a pinned copy that fails its re-hash (`detail:"pinned-hash-mismatch"`, `unable: the pinned viola copy failed its integrity check`)
  (.andromeda/architecture-detail/conventions.md:25)
  > plus run's pinned-copy re-hash refusal
  (.andromeda/architecture.md:100)
  > `1` internal error, and the start, revive-preflight and verify refusals (a refused home-tree check of `run`, `viola revive`'s start or `verify` among them)
  (.andromeda/architecture.md:319)
  > an existing copy is re-hashed on every start, and a mismatch refuses the start (exit 1, `pinned-hash-mismatch`) and leaves the file as found.
  (.andromeda/architecture.md:321)
  > written by `viola run` if absent (`pin_companions` through `replace_private_shared`) and never overwritten;
- (.andromeda/architecture-detail/conventions.md:40) A second first start's home-tree check runs over the `bin/<key>/`, pinned `viola` and `plugin/<key>/` entries the first start is creating, so every entry the race fix creates or leaves there (a `.lock` sibling, a temp file) must pass that check, never turning a racing start into this exit-1 refusal.
  > Exit `1` also covers `viola run`'s start and `viola revive`'s start arm refusing the home tree, checked before the wrapper log opens (`checked_exe`, then `refuse_tree`, in `src/cmd/run.rs`): the home, the instance's `check_instance` set, `ledger/`, `bin/` and `plugin/`, then for the running binary's pin key `bin/<key>/`, the pinned `viola`, `plugin/<key>/` and its `hooks/` and `.claude-plugin/`, each that exists.
- (.andromeda/architecture-detail/occupied-resources.md:149) The fix must keep `run`'s start keying the check by the `pin_key` of the exe bytes read once and pinning those same bytes, keep the check ahead of the wrapper log, and leave `bin/<key>/`, the pinned copy and `plugin/<key>/` with `hooks/` and `.claude-plugin/` passing the write rule at every instant a concurrent start may check them.
  > `viola_state::strict::check_home_tree(home, name, key)` checks the home (the read rule); with a name, `check_instance`'s set; then `ledger/`, `bin/` and `plugin/`; with a key, `bin/<key>/`, the pinned `viola` (`viola.exe` on Windows), `plugin/<key>/`, its `hooks/` and its `.claude-plugin/` (the write rule). It checks only entries that exist and refuses as `TreeRefused::HomeLevel` or `TreeRefused::InstanceLevel`. `run`'s start and `viola revive`'s start call it through `checked_exe` (the instance's name and the `pin_key` of the exe bytes read once), and `verify` calls it with no name and no key.
  (.andromeda/architecture-detail/occupied-resources.md:155)
  > and `viola run`'s and `viola revive`'s starts through `check_home_tree` before `open_wrapper_log` (the home pair, or the instance pair for an instance-level refusal, and no child).
  (.andromeda/architecture.md:320)
  > and `run` pins the very exe bytes it checked.
- (.andromeda/architecture-detail/occupied-resources.md:41) `viola verify`, the second writer racing a `run`'s start whose `pin_exe` call the chunk edits, must keep its home-tree check over `bin/` and `plugin/` ahead of its log and its pin, and a home a concurrent `run` is populating must pass it.
  > `verify` exits 1 with the home-state pair, before its `cli` log and any step line, when `check_home_tree` refuses its home, `ledger/`, `bin/` or `plugin/`, or `update_stamps` refuses `ledger/stamps.json`. `run` and `revive`'s start exit 1 before any log when the home-tree check refuses (`revive --json` keeps its `strict-modes-failed` document).
- (.andromeda/architecture-detail/occupied-resources.md:65) The per-start settings override is a caller of the `replace_private` the chunk edits, so the edit must keep it replaced whole at 0600 after the first snapshot and before the spawn, carrying the absolute pinned path the copy's key names.
  > Per-session settings override: `instances/<ViolaName>/settings.json` holds `{"statusLine":{"type":"command","command":"<pinned path> hook statusline"}}`, the path absolute, so the child's status line is viola's wrapper. `viola run` rewrites it whole at every start, a revived start included, through `viola_state::fs::replace_private` at 0600, after the first snapshot and before the spawn.
- (.andromeda/architecture-detail/occupied-resources.md:173) Any guard the race fix adds to the absent-copy write must take the landed shape, an exclusive lock on a separate `<name>.lock` sibling around a `replace_private` write, never a lock on the guarded file itself, and the two landed lock-guarded writers, callers of the edited helper, must keep their whole-file atomic replace.
  > It is replaced whole through `replace_private` at 0600 under an exclusive lock on `budget.json.lock` (`viola_state::budget::write_budget`).
  (.andromeda/architecture-detail/occupied-resources.md:180)
  > it holds the `.lock` for the read-modify-write, reads the current bytes under the `MAX_FRAME` cap and replaces through `replace_private`.
  (.andromeda/architecture.md:124)
  > **On-disk files:** lower-case with `.ndjson` for logs, `.json` for snapshots and `.lock` for lock files, one lock file per guarded file (`<name>.lock`).
  (.andromeda/architecture.md:413)
  > **Crash-safe disk writes.** Append-only logs use one `write` per line. Mutable files are replaced atomically. Every guarded file has its own `.lock` sibling.
- (.andromeda/architecture-detail/standard-contracts.md:35) The chunk's reading of which readers key on the binary's hash must take the stamps as written by `viola verify` alone, per CLI version, with `writer` no binary version, and record no keying it has not read.
  > `writer` is the literal `"verify"`, not a binary version.
  (.andromeda/architecture.md:155)
  > per CLI version, each ledger row's `pass` or `fail` and the probe's measured values, of which `run` reads none
- (.andromeda/architecture-detail/standard-contracts.md:24) The same reading must take `pinned_bin` as a field only the instance snapshot carries, absent on a replay, so a rebuild keyed apart is read through each instance's recorded pinned path.
  > Fields no event carries (`endpoint`, `pid`, `started_at`, `pinned_bin`, `statusline_command`, `cli_version`, `cli_verified`, `child_pid`, `pending_dialog`, `cwd`) are treated as absent.
  (.andromeda/architecture.md:157)
  > `data` is `{endpoint?, pid, started_at, pinned_bin, statusline_command?, cli_version?, cli_verified, wheel, budget_paused, budget_override_until?, pending_dialog?, links, child_pid?, agent_session_id?, cwd?}`.
  (.andromeda/architecture.md:319)
  > Each instance snapshot records the pinned path it uses, so any later cleanup (an open item) can skip copies that a live or `stale` instance still calls.
- (.andromeda/architecture-detail/occupied-resources.md:53) The per-start plugin rewrite the chunk reads and may edit writes the folder the child loads first by this flag, so a concurrent start's rewrite must leave that folder's `hooks.json` and `.mcp.json` whole for the child that reads them, and a rebuild keyed apart loads another folder.
  > Flags passed to the child: `--plugin-dir <viola home>/plugin/<version>-<hash>`, first on every start;
  (.andromeda/architecture.md:322)
  > `plugin/<version>-<hash>/.claude-plugin/plugin.json`, `plugin/<version>-<hash>/hooks/hooks.json`, `plugin/<version>-<hash>/.mcp.json`.
- (.andromeda/architecture-detail/cross-cutting-patterns.md:9) The pin must stay on the canonical home after the wrapper log and before the endpoint bind, so two first starts of one home meet on one `bin/<key>/` and `plugin/<key>/` whatever spelling each was given, and the fix adds no environment variable or flag that steers the pin.
  > `run`'s start canonicalises the home after its wrapper log has created it, before the collision check, the pin and the endpoint bind. `viola run --home <dir>` passes the home to the child only through `VIOLA_DIR`, so the pinned copy, plugin folder, endpoint hash, logs, `budget.json` and ledger stamps all follow it.
  (.andromeda/architecture-detail/cross-cutting-patterns.md:13)
  > There are no secrets in v1, and environment variables are not a configuration channel.
  (.andromeda/architecture.md:403)
  > The viola home resolves `--home`, then the grandparent of `VIOLA_DIR`, then `<user home>/.viola/`;

## Bindings
- **Conventions exit 1 ↔ Occupied Resources home tree**: a racing start's home-tree check over the entries the absent-copy write creates is the exit-1 refusal, so the fix's entries under `bin/<key>/` and `plugin/<key>/` must pass the write rule the tree check applies, (.andromeda/architecture-detail/conventions.md:40) ↔ (.andromeda/architecture-detail/occupied-resources.md:149) and (.andromeda/architecture.md:320)
- **Occupied Resources lock siblings ↔ Conventions on-disk naming ↔ Cross-cutting crash-safe writes**: a guard the fix adds is a `<name>.lock` sibling with an exclusive lock around one `replace_private` write, as `budget.json` and the stamps hold it, (.andromeda/architecture-detail/occupied-resources.md:173) and (.andromeda/architecture-detail/occupied-resources.md:180) ↔ (.andromeda/architecture.md:124) ↔ (.andromeda/architecture.md:413)
- **Standard Contracts stamps and snapshot ↔ Occupied Resources pinned copy key**: the reproducible-build impact reading takes the stamps keyed per CLI version and `pinned_bin` snapshot-only against the copy keyed by content hash, (.andromeda/architecture-detail/standard-contracts.md:35) and (.andromeda/architecture-detail/standard-contracts.md:24) ↔ (.andromeda/architecture.md:319)
- **Cross-cutting home resolution ↔ Occupied Resources plugin flag**: the canonical home names the one `plugin/<version>-<hash>` folder racing starts rewrite and the child loads, (.andromeda/architecture-detail/cross-cutting-patterns.md:9) ↔ (.andromeda/architecture-detail/occupied-resources.md:53)
