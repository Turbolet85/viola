# Task — architecture · 2026-10-27-pinned-copy-race-and-reproducible-builds

## The session
The pinned copy's absent-copy write in `viola_state::pin::pin_bytes` changed from `replace_private_shared` to a new no-clobber placement of a closed temp file, `viola_state::fs::place_private_new`, so a concurrent first start can no longer exec the copy while another start holds it open for write (`ETXTBSY`, raw errno 26: 4 of 24 two-instance boots failed before, 0 of 24 after).
A lost placement (a concurrent start placed the copy first) re-hashes the copy that stands: equal bytes are reused, different bytes are `PinError::HashMismatch` with the file left as found; `run.pin_copy` keeps `ok|pinned-hash-mismatch`.
Measurement B: six cold release builds of one tree gave two different files whose loaded sections are all equal (3 of 34 sections unequal: `.note.gnu.build-id`, `.symtab`, `.strtab`), so one tree can key two `bin/<key>/` and `plugin/<key>/` folders; no cause is claimed, and acting on it is the founder's.
Measured: the Windows plain-home refusal's inherited write grant is held by whichever SID the runner's temp dir grants (Authenticated Users `S-1-5-11` on one windows-2025 host, Users on another); the product verdict was right and the test oracle now reads the grant by its right.
Test-only: the fake agent's `hook` receipt gains `spawn_os_error`; Path 7's MCP leg boots the default `overseer` + `builder` pair unstamped, retiring its single-instance race workaround. No config key, env var, flag, refusal code, log line, schema, dependency or `src/cmd/` file changed.

## Reached files
- crates/viola-state/src/fs.rs
- crates/viola-state/src/pin.rs
- crates/viola-state/src/strict.rs
- src/bin/viola-fake-agent.rs
- crates/viola-e2e/tests/mcp_tools.rs
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/preconditions.md
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/repro-unstamped.py
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/race-before.md
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/race-after.md
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/red-green.md
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/repro-builds.sh
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/reproducible-builds.md
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/path1-phases.md
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/operator-pass.md

## Touches
- [Snapshot writer] detail (`.andromeda/architecture-detail/established-decisions.md:53`): it lists the pinned copy among `replace_private_shared`'s files; the pinned copy's absent-copy write is now `fs::place_private_new` (a no-clobber placement, never a replace), and `replace_private_shared` keeps only the plugin files and the ConPTY companions.
- [Snapshot writer] body (`.andromeda/architecture.md:37`) and Inherited Defaults (`.andromeda/architecture.md:445`): the one-shared-helper claim over tempfile `persist`; `viola_state::fs` now holds a second placement helper using `TempPath::persist_noclobber` after `into_temp_path` closes the handle, so whatever these lines say of one helper, one `persist` route and no own temp + rename code must be read against it.
- Occupied Resources → Filesystem, the pinned copy (`.andromeda/architecture.md:319` and its counterpart in the detail's res-filesystem group): the copy that `viola run` creates if it is absent is now placed once and never replaced; a concurrent first start that loses the placement re-hashes the copy that stands (equal reused, different `pinned-hash-mismatch`, the file left as found).
- [Deployment / Distribution] (`.andromeda/architecture.md:69`, `.andromeda/architecture-detail/established-decisions.md:247`): its rationale that the content hash keys the copy; measurement B shows one source tree can build two files (equal loaded sections) and so key two `bin/<key>/` and `plugin/<key>/` folders.
- [Deployment / Distribution] detail on the `pin_bytes` / `pin_exe` delegation (`.andromeda/architecture-detail/established-decisions.md:255`): re-read, unchanged and true; `pin_bytes`' signature and its callers `pin_exe` and `pin_and_plugin` are unchanged, and so is `pin_exe`'s caller in `verify`.
- The ConPTY companions' write (`.andromeda/architecture.md:321`, `.andromeda/architecture-detail/established-decisions.md:16`, `:17`, `:248`, `:249`): re-read, unchanged; `pin_companions` still writes through `replace_private_shared`, now imported under `cfg(windows)` only in `pin.rs`; the companions' sharing-violation retry on `established-decisions.md:53` stays a true claim of another writer.
- The plain-home refusal on Windows (`.andromeda/architecture-detail/occupied-resources.md:137`): it names an inherited Users (`S-1-5-32-545`) write entry as the refused entry; disproved on a windows-2025 runner where Authenticated Users (`S-1-5-11`) held the inherited write and Users read only; the refused entry is the inherited untrusted write grant, whichever SID holds it.
- `VIOLA_BIN` and the child's `PATH` prefix (`.andromeda/architecture.md:301`, `:302`): re-read, the pinned copy's path is unchanged.
- The home-tree check naming the pinned copy (`.andromeda/architecture.md:320`, `.andromeda/architecture-detail/established-decisions.md:223`, `:224`, `:229`, `.andromeda/architecture-detail/cross-cutting-patterns.md:9`): re-read, unchanged; `run` still pins the exe bytes it checked, and the start order is unchanged.
- The tampered-copy refusal (`.andromeda/architecture-detail/conventions.md:23`): its text and exit 1 are unchanged; the same `pinned-hash-mismatch` is now also reached when a concurrent start placed different bytes first.
- The pinned copy's key and mode in Stack (`.andromeda/architecture-detail/stack-and-technologies.md:19`): re-read, unchanged; tempfile stays 3.27.0, its `persist_noclobber` and `into_temp_path` already present, no version moved.
- Crash-safe disk writes (`.andromeda/architecture.md:413`): mutable files replaced atomically; the pinned copy is now placed atomically without replace, so a claim that every viola file write is a replace no longer covers it.
- Test-only binaries and the fake agent's receipt (`.andromeda/architecture.md:269`, `:340`, the detail's res-binaries group): `viola-fake-agent`'s `hook` receipt carries a new field on a failed absolute spawn; the binary list and the receipt file's path are unchanged.
- The e2e-home link and the sideload order (`.andromeda/architecture-detail/occupied-resources.md:226`): re-read, unchanged.
- [Error Handling] and the Rust error types convention: no new error variant; `place_private_new` returns `StateError`, and `PinError::HashMismatch` is reused for the lost placement with different bytes.

## New facts
- F1 · established-decisions · `viola_state::fs::place_private_new(path, bytes, mode) -> Result<Placed, StateError>` with public enum `Placed { Written, Present }`: a temp file beside the target (`NamedTempFile::new_in`), its mode set before any byte, the bytes written, `sync_all`, the handle closed (`into_temp_path`), then `persist_noclobber`; `AlreadyExists` is `Ok(Placed::Present)` with the target untouched and the temp removed; any other failure leaves the target as it was; no retry.
- F2 · established-decisions · The pinned copy's absent-copy write uses a closed-handle no-clobber placement because tempfile's `NamedTempFile::persist` renames while the write handle is still open, and a concurrent start's agent exec of a file open for write fails `ETXTBSY` (measured 4 of 24 two-instance boots, each raw errno 26; 0 of 24 after).
- F3 · established-decisions · `place_private_new`'s body `place_private_new_with` takes a `placed` hook that runs right after a `Written` placement: a forced-window test parameter, not a seam (no env read, no feature).
- F4 · occupied-resources · The pinned copy `bin/<version>-<hash>/viola(.exe)` is placed through `pin::place_absent`: `Written` is ok; `Present` (a concurrent first start placed it first) re-hashes the standing copy, equal reused, different `PinError::HashMismatch` with the file left as found; a lost placement logs `run.pin_copy` `ok`.
- F5 · established-decisions · Measurement B: six cold release builds of one tree at one target path gave two files (two sizes, three each, in no fixed order); 3 of 34 sections unequal (`.note.gnu.build-id`, `.symtab`, `.strtab`), every loaded section equal; 6 `strings` lines differ (three `anon.<hash>.<n>.llvm.<number>` local symbols); the cause is not explained, one tree can key two pinned-copy folders, and the act is the founder's (a CARRY on Version done-check).
- F6 · occupied-resources · On windows-2025 runners of one image the temp dir's inherited ACL differs by host (Users write on one; Authenticated Users `S-1-5-11` write with Users read on another), so the plain-home refusal is read by its right (inherited, a write right, a SID `trusted_sid` does not trust), never by one named SID.
- F7 · occupied-resources · The test-only fake agent's `hook` receipt carries `spawn_os_error` (the integer raw OS error, or `null` when the OS gives none) when the spawn of an absolute `command` fails; a successful spawn and the non-absolute arm are unchanged.

## Untouched
- Standard Contracts: no channel frame, method, event kind or `data` field, snapshot field, ledger stamps envelope, `list` payload or CLI `--json` document changed.
- Refusal reasons, details, MCP results and CLI exit codes: no new code or detail; `pinned-hash-mismatch` keeps exit 1.
- Environment variables: no env var read or set, and no new test seam (the `placed` hook reads no env and needs no feature).
- IPC endpoints, the Unix socket dir, the named pipe SDDL and the server identity check.
- Claude Code integration names, registered hook events, the flags passed to the child and the per-session settings override.
- [Hook Transport] and [Hook Contract]: fail-open, the dialog hold and the spine are unchanged.
- [Budget Governor], [Links], [Human Takeover / Wheel], [MCP], [Delivery Confirmation] and [Screen Model].
- [CLI Version Compatibility] and the capability ledger: no row added or moved, the relied set and stamped versions unchanged.
- [Session Liveness] heartbeat and the `claude agents --json` join.
- [Concurrency / Backend Framework], [Module Boundaries] and Tokio containment: `viola-state` stays sync, no crate added or removed, the `deny-sync.toml` bans green.
- Stack versions and licences: no dependency added or bumped.
- Repository resources: `schemas/diag-line.v1.json` and every other schema, `fixtures/`, `vendor/conpty/` and the `target/` paths are unchanged.
- Path 1's timing and kill line: red 1 is recorded as runner variance under a WATCH and changes no architecture claim.
