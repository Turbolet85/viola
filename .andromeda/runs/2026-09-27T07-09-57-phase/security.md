# security extract

## Relevance
Relevant. This chunk builds the main local trust boundary named in security-plan §Threat Model Summary (Attack surface: "Local IPC, the wrapper channel … highest-impact surface"). That covers the listener, the client open, the framing and the protocol errors.

## Constraints
1. **Frame bounds.** Security-plan §Input Validation ("Channel frames (IPC ndjson)" row, plus Constants) requires these on every channel stream reader:
   - wrap the reader in `Read::take(MAX_FRAME)` before `read_line`, using the single `MAX_FRAME` const (16 MiB) in `viola-core`;
   - answer an over-limit frame with `-32600` and close the connection;
   - keep serde_json's default depth of 128, never `unbounded_depth`;
   - answer a bad `v` with `-32602`.
   Channel ndjson framing at `MAX_FRAME` is a named parser surface for fuzz and property coverage (§Input Validation, "Parser surfaces").
2. **Windows listener hardening.** Security-plan §Authentication & Authorization ("IPC access control (Windows)" row) requires:
   - an explicit protected DACL `D:P(A;;GA;;;<user-SID>)(A;;GA;;;SY)` via `ListenerOptionsExt::security_descriptor`, with the SID from `GetTokenInformation(TokenUser)` and no logon SID;
   - `accept_remote(false)`;
   - `FILE_FLAG_FIRST_PIPE_INSTANCE` behaviour, so a squatted name makes `run` exit 1;
   - `inheritable(false)`.

   The scope defers the SDDL and `accept_remote` to Epoch 6. Binding with the default DACL in the meantime is a boundary widening against §Security Anti-Patterns → Authentication (bullet 1). P3 should measure what interprocess 2.4.4 binds by default. P4 either takes the SDDL now or halts for a live operator ruling.
3. **Windows client open.** Security-plan §Authentication & Authorization ("IPC client-side server verification" row) and Decisions Log `2026-09-25` require:
   - the open to use `SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION | FILE_FLAG_OVERLAPPED`;
   - the handle to be adopted with `local_socket::Stream::try_from(OwnedHandle)`;
   - no fallback to interprocess's default connect, and no default-connect test double.

   The plan row names the windows-sys `CreateFileW` recipe. Picking the measured safe-Rust std `OpenOptions::security_qos_flags` + `custom_flags(FILE_FLAG_OVERLAPPED)` open is allowed by the Decisions Log ("recorded for the `viola-channel` chunk to weigh"), but it would mean amending that row and the matching §Bootstrap phases bullet.
4. **Unix endpoint path.** Security-plan §Authentication & Authorization ("IPC access control (Unix)" row) and Decisions Log amendment 2 require:
   - the socket at `<per-user 0700 dir>/viola-<h12>.sock`: `$XDG_RUNTIME_DIR/viola/` on Linux, `$TMPDIR/viola/` on macOS, `/tmp/viola-<uid>/` only as a fallback;
   - `try_overwrite` off;
   - no Linux abstract namespace.

   §Bootstrap phases (auth-scaffolding-baseline) folds amendment 2 in "before the `viola-channel` chunk". So the path location belongs to this chunk even though the dir-verify and peer-euid checks are Epoch 7's.
5. **Server verification before any frame.** Security-plan §Security Anti-Patterns → Authentication (bullet 5) requires verification before the first frame of any method, including `hook.event` notifications. On Windows that is pid plus start time checked against a strict-modes-checked snapshot. The scope defers this to Epoch 6. Whether this chunk's client writes frames unverified in the meantime, and how that deferral is recorded, is P4's question.
6. **Sanitised errors.** Security-plan §Error Handling requires:
   - `ChannelError` with a fixed `Display` that has no path or payload fields;
   - `-32603` with the fixed message "internal error" and `data: null`;
   - no upstream text, absolute paths, serde_path_to_error values or internal type names in `error.data`.

   The newer-peer `-32602` `data: {supported, wrapper}` must carry version strings only.
7. **Self-reported fields and snapshot writes.** Security-plan §Security Anti-Patterns (Authentication bullet 7; Universal) requires:
   - envelope `sender` and `conn`, like `from`, are self-reported and never an authenticated identity;
   - only the instance's own wrapper writes `snapshot.json` `endpoint`;
   - no `config.json`, `VIOLA_*` variable or CLI flag may switch off the IPC DACL, the socket-directory check or server verification.

## Patterns to follow
- **SQOS recipe:** reuse the one pinned in `tests/channel_sqos_open.rs`, including its no-SQOS control (Decisions Log `2026-09-25`, Conditions). Land the viola-client `security_negatives_*.rs` SQOS case with the crate.
- **`MAX_FRAME`:** use the shared const from `viola-core`. It is one const for every external reader (§Input Validation, Constants).
- **Snapshot write:** write `endpoint` into `snapshot.json` through `viola_state::fs::replace_private`, which sets 0600 on the temp file before the write and then persists it (§Authentication & Authorization, `~/.viola/` access control).
- **Windows listener hardening:** use interprocess 2.4.4 `ListenerOptionsExt::security_descriptor(SecurityDescriptor::deserialize(..))` plus the windows-sys SID lookup (§Bootstrap phases, auth-scaffolding-baseline `viola-channel` bullet).
- **Enum `Display`:** give `ChannelError` a fixed-message `Display`, following the existing `PtyError` / `CoreError` precedent (§Error Handling; §Bootstrap phases error-sanitization-wire).

## Anti-patterns to avoid
- NEVER create an interprocess Windows listener without `security_descriptor`, since the default DACL grants read to Everyone and anonymous. NEVER open a Windows client pipe without the SQOS Identification flags (§Security Anti-Patterns → Authentication).
- NEVER place a Unix socket directly in `/tmp` or at the `$TMPDIR` root. NEVER rely on `mode(0o600)` as the only Unix control. NEVER use the abstract namespace, and NEVER enable `try_overwrite` (§Security Anti-Patterns → Authentication; Code Patterns).
- NEVER call `read_line` on a channel stream without `Read::take(MAX_FRAME)`. NEVER enable `unbounded_depth`. NEVER make channel handles inheritable (§Security Anti-Patterns → Input; Data Protection).

## Contract bindings
- **obs ↔ security:** channel spans (`#[instrument(skip_all, …)]`), `conn`/`corr` fields and veil `#[derive(Redact)]` payloads must meet the NEVER-log floor (§Bootstrap phases, logging-redaction-wire). Frame params carry prompt and reply text, so they stay out of log lines, stderr and `error.data`. `diagnostics/` files are 0600.
- **tests ↔ security:** test-plan §6 Security control negatives covers the Windows client SQOS case. The framing proptest and fuzz target at `MAX_FRAME` ± 1 covers a security-plan parser surface, and its corpus must be synthetic only (Decisions Log `2026-09-24`). The E2 Unix `fds` receipt witnesses channel-handle non-inheritance (§Data Protection, Handle inheritance).
- **CI / supply chain ↔ security:**
  - `viola-channel` joins the `deny-sync.toml` sole-root tokio ban.
  - The new deps interprocess `=2.4.4` and windows-sys `=0.61.2` are exact-pinned in `[workspace.dependencies]`.
  - `cargo deny check` stays green, relying on the existing 0BSD per-crate exceptions for `doctest-file` / `recvmsg` (§Dependency Security).
- **arch ↔ security:** amendment 2 (Unix endpoint path) and the "Wrapper channel frames" contract; snapshot `endpoint` is the reference value for later client verification (§Security Anti-Patterns → Universal).

## Acceptance criteria contributions
- A frame of `MAX_FRAME` + 1 bytes gets `-32600` and the connection is closed without parsing. A frame of exactly `MAX_FRAME` is accepted. A grep finds no `read_line` on a channel stream without `take(MAX_FRAME)` (per security-plan §Input Validation).
- The Windows client open carries `SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION | FILE_FLAG_OVERLAPPED`. A grep finds no interprocess default connect in `viola-channel`. The `security_negatives_*.rs` SQOS case passes on the windows-2025 `test` leg (per security-plan §Authentication & Authorization).
- The Windows listener is built with `security_descriptor` (user SID + SYSTEM), `accept_remote(false)` and `inheritable(false)`. Otherwise the default-DACL interim is recorded as an operator-ratified boundary widening in the Security Decisions Log before merge. In either case the Unix socket path is inside a per-user dir, never at the `/tmp` or `$TMPDIR` root (per security-plan §Security Anti-Patterns → Authentication).
- `cargo deny check` and the sole-root `deny-sync.toml` tokio ban both pass with `viola-channel` listed. `ChannelError`'s `Display` has fixed messages, and `-32603` emits `"internal error"` with `data: null` (per security-plan §Dependency Security; §Error Handling).

## Relevant amendment history
- **2026-09-25-security-prerequisites:**
  - The SQOS spike passed; `FILE_FLAG_OVERLAPPED` is required because a non-overlapped adopted handle hangs.
  - The default connect reads Impersonation and stays banned.
  - The std `OpenOptions` alternative was left for this chunk to weigh.
  - interprocess's transitive `doctest-file` / `recvmsg` were admitted as per-crate 0BSD exceptions. This was an operator-ratified boundary widening, and any further exception needs its own entry.
  - Why it matters here: this is the client recipe and licence posture this chunk inherits.
- **2026-09-27-instance-state-and-start-order:**
  - Every replaced file now goes through `viola_state::fs::replace_private`.
  - The sweep marked the snapshot `endpoint` "as a verification reference once bound" as true with no change.
  - Why it matters here: it governs how this chunk writes `endpoint`.
- **2026-09-24-supply-chain-and-workflow-gates:** the tokio ban moved into `deny-sync.toml` per sync crate as sole root, and `scripts/deny-probes.sh` proves each ban live. Why it matters here: it governs how `viola-channel` joins the sync-crate gate.
- **2026-09-24-three-os-ci-headless-harness-skeleton (rejected entry):**
  - Known gaps in the interim skeleton were treated as sequencing deferrals owned by route entries with `CARRY:` pins, not as Decisions Log entries.
  - This is the precedent for the Epoch 6/7 listener and verification deferrals.
  - The 2026-09-27 overseer directive in scope overrides it for any boundary widening: halt for a live answer.
- **2026-09-24-quality-gates:** the fuzz workspace exemption requires a synthetic corpus only, and a non-synthetic seed drops the fuzz upload. Why it matters here: it applies to the new framing fuzz target.
