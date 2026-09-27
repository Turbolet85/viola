# security extract

## Relevance
Relevant. This chunk creates the code-bearing, integrity-sensitive state that the security plan names: the home tree modes, `snapshot.json`, the pinned `bin/` exe, the `plugin/` files and `settings.json`. It also sets the child spawn env.

## Constraints
1. **Creation modes.** Security-plan §Authentication & Authorization ("`~/.viola/` access control" row) and §Data Protection ("At rest — Files") set these rules for this chunk's writes:
   - Every directory this chunk creates is 0700. That covers the home or `--home`, `instances/<name>/`, `bin/<version>-<hash>/` and `plugin/<version>-<hash>/`, including `hooks/` and `.claude-plugin/`.
   - Every file is 0600: `events.ndjson`, `snapshot.json`, the heartbeat carrier, the plugin files and `settings.json`.
   - The pinned `bin/<version>-<hash>/viola(.exe)` is the only 0700 file. It must never be group- or world-writable.
   - For atomic-write-file, the temp file's mode is set explicitly before `commit()`. Never rely on the umask or on the mode of the file being replaced. If the mode cannot be set, the write is discarded.
   - On Windows, a `--home` outside `%USERPROFILE%` gets an explicit protected user + SYSTEM DACL, or it is refused.
2. **Pinned exe hash.** Security-plan §Data Protection ("Code-bearing artefacts") and the Decisions Log `2026-09-25` entry require:
   - `<hash>` is the first 16 hex characters of a SHA-256 of the exe bytes, computed with `sha2 =0.11.0` and `default-features = false`.
   - Before reusing an existing `bin/<version>-<hash>/viola(.exe)`, `viola run` re-hashes it and compares. A mismatch refuses the start with exit 1.
   - FNV-1a and `DefaultHasher` are banned for `<hash>`.
3. **Plugin and settings rewrite.** Security-plan §Data Protection, §Bootstrap phases (auth-scaffolding-baseline, the `viola run` / `viola-state` bullet) and §Security Anti-Patterns → Universal require:
   - The `plugin/<version>-<hash>/` files and `instances/<name>/settings.json` are rewritten at every start with atomic-write-file at 0600. An existing copy is never reused.
   - Every exec-form `command` in `hooks.json` and `.mcp.json`, and the statusline command in `settings.json`, is the absolute pinned `bin/<version>-<hash>/viola(.exe)` path, never a PATH lookup.
   - Per §Secret Management ("Never in code"), the embedded plugin files hold exec paths only and no credential literal.
4. **Snapshot writer and reference values.** Security-plan §Security Anti-Patterns → Universal says only the instance's own wrapper may write `instances/<name>/snapshot.json`. Per §Authentication & Authorization ("IPC client-side server verification" row), the snapshot's `endpoint`, `pid` and `started_at` are the reference values the later channel client checks against `server_process_id()` and the sysinfo process start time. So `started_at` must be recorded in a form that can be compared with sysinfo's start time. Whether the snapshot shape this chunk lands allows that comparison is a question for research.
5. **Strict-modes on read.** Security-plan §Authentication & Authorization ("`~/.viola/` access control": the strict-modes check, the read rule and the Windows owner + DACL check) requires the check at the start of `run`, in every process that reads `snapshot.json` or `ledger/stamps.json`. That includes the live/stale name-refusal read and the version-gate stamp read. On failure `run` exits 1. On Windows, each existing `events.ndjson` also passes the read-ACE check before it is opened. Security-plan §Input Validation ("Own state files on read" row) also requires snapshot and ndjson reads to be capped at `MAX_FRAME` and to happen only after this check. Two things are research's question: whether the code already does any of this, and whether this chunk or the "Home and code-bearing file integrity" route entry owns it (see amendment history).
6. **Names and env at spawn.** Security-plan §Security Anti-Patterns → Input bans joining `instances/<name>` with any string that did not come out of `ViolaName::try_new`. §Security Anti-Patterns → Universal says that `VIOLA_*` variables, including the `VIOLA_DIR` / `VIOLA_NAME` / `VIOLA_BIN` this chunk sets at spawn, are plumbing and never a configuration channel that can switch off a control. Per §Secret Management → Storage, the R8-stripped `CLAUDE*` values must never be serialised into events, snapshots or diagnostics. Adding `VIOLA_*` must not re-admit an identity-floor name.
7. **Error sanitisation.** Security-plan §Error Handling requires `StateError`'s `Display` to use fixed messages, with fields holding paths or payloads left out. The refusal output for a hash mismatch or a live name carries no absolute path (the OS username is the only PII in scope). Full detail goes only to `instances/<name>/diagnostics/` (0600), per §Bootstrap phases `error-sanitization-wire` and `logging-redaction-wire`.

## Patterns to follow
- **Atomic writes:** atomic-write-file 0.3.1 with an explicit std `set_permissions` on the temp file before `commit()`. Append-created files use `OpenOptionsExt::mode(0o600)` (security-plan §Authentication & Authorization, `~/.viola/` row).
- **Hash recipe:** reuse the SHA-256 and 16-hex truncation recipe pinned by `tests/contract_content_hash.rs`. `sha2` moves from a root dev-dependency to a normal dependency of `viola-state` (Decisions Log `2026-09-25` Conditions).
- **Spawn env:** the existing R8 strip (`src/run/env.rs`, `IDENTITY_FLOOR`, names only) and `resolve_program`'s absolute-path child resolution stay unchanged. `VIOLA_*` and the `PATH` prefix are layered on after the strip (security-plan §Secret Management → Storage; §Input Validation, "Child executable resolution" row).
- **Error types:** thiserror `StateError` with fixed `Display` strings, as the other `<Crate>Error` enums do (security-plan §Error Handling).

## Anti-patterns to avoid
- **Data Protection bans (security-plan §Security Anti-Patterns → Data Protection):**
  - creating anything under the home with the default umask;
  - reusing `bin/<version>-<hash>/viola(.exe)` without a SHA-256 re-hash;
  - deriving `<hash>` from a non-cryptographic hash;
  - reusing existing `plugin/` files without rewriting them;
  - on Windows, letting a `#[cfg(windows)]` strict-modes path return `Ok(())` unchecked.
- **Universal bans (security-plan §Security Anti-Patterns → Universal):**
  - any exec-form `command` or statusline command that resolves `viola` through PATH;
  - any writer of `snapshot.json` other than the instance's wrapper;
  - a `VIOLA_*` variable, flag or config value that turns off the strict-modes check.
- **NEVER-log ban (security-plan §Security Anti-Patterns → Secrets):** R8-stripped `CLAUDE*` values must not reach `events.ndjson`, snapshots or `diagnostics/`, including in the start events this chunk may write.

## Contract bindings
- **security ↔ arch (Wrapper channel chunk):** the snapshot's `endpoint`, `pid` and `started_at` are the reference values for client server verification (security-plan §Authentication & Authorization, IPC client-side server verification row). The endpoint value itself, including the Unix per-user 0700 directory (Decisions Log arch amendment 2), is filled by the channel chunk. This chunk fixes only the snapshot's shape and who may write it.
- **security ↔ obs:** start-sequence spans and process-log events, such as `process-start{subject:"claude-child"}` naming the stripped and kept names, follow the consolidated NEVER-log floor: names only, no values, no absolute paths in external errors (security-plan §Bootstrap phases `logging-redaction-wire`).
- **security ↔ tests:**
  - The test-plan §6 exit-cause row "pinned exe fails re-hash → exit 1" is the witness for the §Data Protection re-hash rule.
  - E2's `VIOLA_NAME` / `VIOLA_DIR` presence assertion sits beside `tests/tui_env_strip.rs`'s identity-floor canaries.
  - Test homes pass the CI `viola-harness secret-scan` canary gate (security-plan §Secret Management → Secret scanning in CI).
- **security ↔ build gates:** `viola-state` is a sync crate, so the sole-root `deny-sync.toml` tokio ban applies, and `sha2` stays inside the `cargo deny` C-build ban (security-plan §Dependency Security, `deny.toml` additions).

## Acceptance criteria contributions
- A pinned `bin/<version>-<hash>/viola(.exe)` whose bytes are changed after the first start makes the next `viola run` exit 1 with no child spawned and no absolute path in its error output (per security-plan §Data Protection).
- On Unix, `stat` after a start shows 0700 on the home, `instances/<name>/`, `bin/<version>-<hash>/` and `plugin/<version>-<hash>/` (with its `hooks/` and `.claude-plugin/`). It shows 0600 on `events.ndjson`, `snapshot.json`, the plugin files and `settings.json`, and exactly 0700 on the pinned `viola` (per security-plan §Authentication & Authorization, `~/.viola/` access control).
- Every exec-form `command` in the written `hooks.json` and `.mcp.json`, and the statusline command in `instances/<name>/settings.json`, parses as the absolute pinned path. A tampered plugin file is overwritten at the next start (per security-plan §Security Anti-Patterns → Universal).
- The secret-scan canary gate passes over the chunk's test homes: no stripped `CLAUDE*` canary value appears in `events.ndjson`, `snapshot.json` or `diagnostics/` (per security-plan §Secret Management).

## Relevant amendment history
- **2026-09-24-three-os-ci-headless-harness-skeleton (rejected proposals).** The walking-skeleton `viola run` does not canonicalise or strict-modes-check `--home`, and does not set the Windows protected DACL. These were recorded as sequencing deferrals owned by the markerless route entries "Home and code-bearing file integrity" and "CLI machine contract — global --home". The plan text stays the target. This bears directly on the scope's P3 premise about which chunk lands strict-modes on read and which lands only the 0700/0600 creation half.
- **2026-09-25-security-prerequisites.** Picked `sha2 =0.11.0` (`default-features = false`) and the 16-hex truncation for `<hash>`, and named the `viola-state` re-hash bootstrap bullet. This is the source of CARRY 1. The re-hash refusal is owed to the chunk that writes `bin/`, which is this one.
- **2026-09-25-pty-wrapper-on-windows.** Built the R8 strip (persistent-name exemption plus the 11-name identity floor, names only) and absolute child resolution with the `BatchScriptChild` refusal. This is the spawn env that CARRY 2's `VIOLA_*` additions build on, and the floor must still hold.
- **2026-09-26-local-linux-pre-push-gate.** Added the `FAKE_AGENT_PUMP_DELAY_MS` seam in `src/cmd/run.rs`, a file this chunk edits, and made the Universal env rule explicit: no other env seam without its own Decisions Log entry. Reordering the start sequence must keep the seam `fake-agent`-only and out of release builds.
