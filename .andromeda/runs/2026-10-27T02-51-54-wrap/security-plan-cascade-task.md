# Task — security-plan · 2026-10-27-pinned-copy-race-and-reproducible-builds

## The session
The pinned copy's absent-copy write in `viola_state::pin::pin_bytes` changed from a `replace_private_shared` replace to a new no-clobber placement of a closed temp file, `viola_state::fs::place_private_new`, with its result `Placed { Written, Present }`.
A concurrent first start that loses the placement now re-hashes the copy that stands. Equal bytes are reused; different bytes are `PinError::HashMismatch`, with the file left as found.
This closes the measured first-start race. A concurrent start's agent could exec the copy while another start still held it open for write (`ETXTBSY`, raw 26 on Linux): 4 of 24 two-instance boots failed before the change and 0 of 24 after.
Measurement B built one tree cleanly six times and got two different release files. Every loaded section was equal; only `.note.gnu.build-id`, `.symtab` and `.strtab` differed. No cause is claimed, and acting on it is the founder's call.
The Windows plain-home oracle now finds the inherited untrusted write grant by its right, whichever SID holds it (a runner gave Authenticated Users `S-1-5-11` the write, not Users). This is a test-only `cfg(windows)` module of `strict.rs`, and the product verdict is unchanged.
Test-only: the fake agent's `hook` receipt gains `spawn_os_error`. Path 7's MCP leg now boots the default unstamped pair, which retires the race workaround.

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
- Data Protection's owed line, which says the `pin_exe` first-start race and reproducible builds are owed to the route entry "Pinned copy race and reproducible builds". It is now closed. The race is fixed by the no-clobber placement. Measurement B's reading is that one tree can build two files with equal loaded sections, so one tree can key two `bin/<key>/` and `plugin/<key>/` folders; the cause is not explained and the act is the founder's.
- Data Protection's statement of the Windows plain-home refusal, which says the refused entry is an inherited Users (`S-1-5-32-545`) write ACE. Measurement disproved it. The refused entry is an inherited write grant to an untrusted SID, read by its right, and on one windows-2025 runner that SID was Authenticated Users (`S-1-5-11`, mask `0x001301BF`) while Users held read only.
- Lines that name the plain-home refusal but no SID, or that give Users, Authenticated Users and Everyone only as examples (Authentication & Authorization's plain-home lines): re-read, unmoved.
- Data Protection's rows on how the plugin files and the ConPTY companions are written through `replace_private_shared`: still true. That helper's users are now exactly the plugin files and the companions, and the pinned copy is no longer one of them.
- The `~/.viola/` access-control rule that every replaced file goes through `replace_private` with its mode set before any byte. The pinned copy's first write is now a placement, not a replace, through `place_private_new`, which also sets the mode (`restrict`) before any byte. The 0700 mode of the pinned `viola` is unchanged.
- The re-hash rule for the pinned binary before reuse (Data Protection Files, the Data Protection anti-pattern on reusing `bin/<version>-<hash>/viola(.exe)`, Bootstrap's `viola-state` re-hash). It now also covers a copy that a concurrent first start placed: that copy is re-hashed, never replaced. The rule's text is unmoved.
- The truncated SHA-256 key `<hash>` in `bin/<version>-<hash>/`: measurement B shows it keys a built file, not a source tree. The cryptographic-hash ban is unmoved.
- The Filesystem boundary (Threat Model, `boundary-filesystem`): the pinned binary is placed once and never replaced under concurrent starts. Its strict-modes check before any log opens is unchanged.
- The Threat Model lines that name the pinned binary's integrity, path, mode or re-hash: re-read, unmoved.
- The hash-mismatch outcome on a lost placement leaves the file as found, the same left-as-found posture the companions already have. `run.pin_copy` keeps `ok|pinned-hash-mismatch`, a lost placement reads `ok`, and no new error variant or detail code was added.
- The ban on new seams and environment reads (Universal anti-pattern; the Input Validation test-seam rows): `place_private_new_with`'s private `placed` hook is a test parameter with no env read and no feature, so the seam set stays the two `FAKE_AGENT_*` seams plus the terminal colour facts.
- The Windows owner and DACL strict-modes check: the product code in `strict.rs` is unchanged. Only its `cfg(windows)` test module gained `inherited_untrusted_writes` and a control, a viola-made home that reads no such grant.
- The fake agent (a test-only binary, `fake-agent` feature), which `scripts/release-check.sh` keeps out of the release build: its new receipt field `spawn_os_error` (an integer or `null`) adds no product surface.
- Dependency rows: no dependency was added or bumped. `tempfile 3.27.0` already provides `persist_noclobber` and `into_temp_path`, and `cargo deny check` and the `deny-sync.toml` bans on `viola-state` are green.
- Release build gate: measurement B used `scripts/release-check.sh`'s build at one target path; the script and its `--probe` (5/5 refused, control clean) are unchanged.
- The two product callers of `replace_private_shared` (`pin_and_plugin` for the plugin files, `pin_companions` for the companions under `cfg(windows)`) and `pin_bytes`' two callers (`pin_exe`, `pin_and_plugin`) are unchanged in count. Any count of `replace_private_shared`'s users drops the pinned copy.

## New facts
- F1 · data-protection · The pinned copy's absent-copy write is a no-clobber placement through `viola_state::fs::place_private_new`, never a replace. It writes a temp file beside the target, sets its mode before any byte, runs `sync_all`, closes the write handle (`into_temp_path`), then calls `persist_noclobber`.
- F2 · data-protection · A present name is never replaced: `AlreadyExists` returns `Placed::Present`, leaves the target untouched and removes the temp file. Any other failure leaves the target as it was, with no retry.
- F3 · data-protection · On a lost placement (`Present`), `pin_bytes` re-hashes the copy that stands. Equal bytes are reused; different bytes are `PinError::HashMismatch`, with the file left as found, never overwritten or deleted.
- F4 · threat-model-summary · The concurrent-first-start exec race on the pinned copy was `ETXTBSY` (raw 26 on Linux), read on the product run. The cause is a write handle still open at exec, and closing the handle before placement removes it: 4 of 24 two-instance boots failed before the change and 0 of 24 after.
- F5 · data-protection · Measurement B: clean release builds of one tree can produce two different files whose loaded sections are all equal. They differed only in `.note.gnu.build-id`, `.symtab` and `.strtab`, from three `anon.<hash>.<n>.llvm.<number>` local symbols. No cause is claimed, so one tree can key two `bin/<key>/` and `plugin/<key>/` folders. This is a founder item, carried on "Version done-check".
- F6 · authentication-authorization · The inherited ACL of a windows-2025 runner's temp dir differs by host: Users held the write on one runner, Authenticated Users held it on another while Users held read only. The plain-home refusal is therefore checked by the write right whichever SID holds it, and a test that names one fixed SID depends on the runner.

## Untouched
- IPC access control and server verification (the Windows SDDL listener, the SQOS client open, the Unix 0700 socket directory, peer euid, pid and start-time verification): no change in `viola-channel` or the client paths.
- Input Validation's paste text, dialog free text, channel frames, hook stdin, MCP tool inputs, `config.json` and the `MAX_FRAME` constant: none of them read by this change.
- The NEVER-log floor and `logging-redaction-wire`: no new log line, field, span or detail code. `schemas/diag-line.v1.json` is unchanged, and `spawn_os_error` is a receipt field of a test-only binary, not a product log line.
- Error Handling's external response shapes (channel `-32603`, MCP `isError` codes, CLI `--json` errors): no new external error and no new error variant.
- The statusline source, the shell-out and the budget gate: `src/cmd/run.rs` and the `hook statusline` arm are unchanged.
- `viola verify`'s interactive children, probe dirs and `--record` fixture scrub: `src/cmd/verify.rs` is unchanged.
- Dependency Security's licence list, advisory ignore, the vendored ConPTY audit (`scripts/conpty-vendor.sh`), the `fuzz/` exemption and the CI workflow rules: no workflow, lockfile or vendored file changed.
- Secret Management, rotation and the CI artifact canary scan: there are no secrets, and the scan is unchanged.
- The Windows DLL search restriction and the sideload pre-load order in `main`: unchanged.
- The tier, data classification, compliance triggers and the API Security rmcp feature table: unmoved.
