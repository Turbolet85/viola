# security · Data Protection — category read

## Carried
- (.andromeda/security-plan-detail/data-protection.md:8) The `link` / `unlink` lines the wrapper appends and the snapshot `links` it rewrites stay plaintext state under the home, protected by OS permissions only, and the verbs' read of `snapshot.json` (crossing (c)) answers to the strict-modes check this rule names, whichever of the founder's forms is chosen at P4.
  > **Files (`~/.viola/`):** Plaintext by design (Established Decisions: Database / State Store). Protected by OS permissions only:
  >   - Unix: 0700 directories and 0600 files, except the pinned `bin/<version>-<hash>/viola`, which is 0700.
  >   - Windows: the inherited `%USERPROFILE%` DACL (user + SYSTEM + Administrators), or an explicit protected user + SYSTEM DACL for a `--home` outside `%USERPROFILE%`.
  >   - The strict-modes check runs at startup (Authentication & Authorization, `~/.viola/` access control).
- (.andromeda/security-plan.md:29) The verbs read `snapshot.json` for `endpoint`, `pid` and `started_at`; its recorded `cwd` must reach none of the three result lines, the `--json` result, a refusal or an error body.
  > The snapshot's recorded `cwd`, an absolute host path that can carry the OS username, stands in the 0600 snapshot only, on no log line, error body or stdout.
- (.andromeda/security-plan.md:189) The two newly served channel methods (crossing (a)) carry their names over the existing local endpoint only, so their confidentiality in transit rests on the same IPC ACLs; no new transport is added.
  > **TLS:** N/A. The IPC channel is local named pipes or Unix sockets, and MCP is stdio. Confidentiality in transit depends on the IPC ACLs (Authentication & Authorization).
- (.andromeda/security-plan.md:202) A served `link` or `unlink` appends its line to `events.ndjson`, which must stay 0600 inside 0700 directories; an `unlink` of an absent pair appends nothing.
  > `events.ndjson` and the files in `instances/<name>/diagnostics/` are 0600 inside 0700 directories.
- (.andromeda/security-plan.md:204) The `link` / `unlink` line's format and fields are obs's, not this plan's.
  > Logging format and fields belong to obs.
- (.andromeda/security-plan.md:459) Every write the chunk makes under the home (the event append, the snapshot `links` replace) sets 0700 / 0600 explicitly, never the umask; and if the founder picks the instance-check form, the check the verbs run before the snapshot read must not skip the Windows owner + DACL half.
  > NEVER create `~/.viola/`, `--home` or any file under it with the default umask. Directories are 0700 and files 0600 (`OpenOptionsExt::mode`); the only exception is the pinned `bin/<version>-<hash>/viola` at 0700, and NEVER make it group- or world-writable. On Windows, NEVER accept a `--home` outside `%USERPROFILE%` without an explicit protected user + SYSTEM DACL, and NEVER skip the Windows owner + DACL strict-modes check at startup (`#[cfg(windows)]` must not return `Ok(())` unchecked), and NEVER pass a NULL DACL, a volume without persistent ACLs, or an inherit-only `GENERIC_*` grant just because no specific-rights ACE matched.

## Bindings
- **Files strict-modes ↔ Authentication & Authorization (`~/.viola/` access control) and Anti-Patterns § Authentication (the dated gaps' list)**: the verbs' snapshot read is crossing (c); the first P4 form is a dated strict-modes gap of their own (the anti-pattern's gap list names no `link` / `unlink` frame, so it gains them only on the founder's word), the second is `check_instance` ahead of the read; both sides name the same strict-modes check (.andromeda/security-plan-detail/data-protection.md:11)
- **Snapshot `cwd` ↔ Bootstrap phases `logging-redaction-wire` and Error Handling (no absolute path in an external error)**: the result lines, `--json` and the `-32602` refusals carry codes, closed words and `ViolaName`-parsed names only, never the recorded `cwd` (.andromeda/security-plan.md:29)
- **In transit ↔ Authentication & Authorization (IPC access control, server verification) and Anti-Patterns § Universal (a new channel method needs a Security Decisions Log entry)**: the two methods ride the existing ACL'd endpoint, and their serving is the boundary widening the founder rules at P4 (.andromeda/security-plan.md:189)
- **Logs ↔ obs-plan (event catalog) and architecture (the `link` / `unlink` kinds, landed)**: permissions here, line fields there (.andromeda/security-plan.md:204)
- **Umask ban ↔ architecture (`viola_state::fs::replace_private`, the ndjson append)**: the snapshot `links` rewrite goes through the one replace helper (.andromeda/security-plan.md:459)
