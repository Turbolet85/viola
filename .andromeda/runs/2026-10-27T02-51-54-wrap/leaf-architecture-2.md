group: architecture-2
read: .claude/docs/services/viola-core.md · 43 · «- `.claude/docs/stack.md` · `.claude/docs/conventions.md` · »
read: .claude/docs/services/viola-mcp.md · 39 · «- `.andromeda/architecture.md` [MCP] · `.claude/rules/api.md»
read: .claude/docs/services/viola-pty.md · 47 · «- `.andromeda/architecture.md` [PTY] · `.claude/docs/gotchas»
read: .claude/docs/services/viola-state.md · 49 · «- `.andromeda/architecture.md` §Standard Contracts (snapshot»

- leaf: .claude/docs/services/viola-state.md:11   class: omission
  leaf-text: «tempfile 3.27.0 (the `persist` helper)»
  source: .andromeda/architecture-detail/stack-and-technologies.md:18 «and tempfile 3.27.0 `TempPath::persist_noclobber` after `into_temp_path` closes the write handle (`viola_state::fs::place_private_new`) for the pinned copy's absent-copy write»
  replace-with: «tempfile 3.27.0 (the `persist` helper, and `TempPath::persist_noclobber` for `fs::place_private_new`, the pinned copy's absent-copy write)»

- leaf: .claude/docs/services/viola-state.md:20   class: stale
  leaf-text: «the pinned `bin/<version>-<hash>/viola` is 0700; the temp file's mode is set before any byte is written, then `persist`; if the mode cannot be set, discard the write.»
  source: .andromeda/architecture-detail/established-decisions.md:60 «it creates a temp file beside the target (`NamedTempFile::new_in`), sets its mode before any byte (the pinned `viola` at `DIR_MODE`), writes the bytes, `sync_all`s, closes the write handle (`into_temp_path`)»
  replace-with: «the pinned `bin/<version>-<hash>/viola` is 0700; the temp file's mode is set before any byte is written, then `persist` (`replace_private`), or for the pinned copy's absent-copy write the write handle closed (`into_temp_path`) and `TempPath::persist_noclobber` (`fs::place_private_new` → `Placed { Written, Present }`, `AlreadyExists` being `Ok(Placed::Present)` with the target untouched; no retry); if the mode cannot be set, discard the write.»

- leaf: .claude/docs/services/viola-state.md:24   class: omission
  leaf-text: «`pin_bytes(home, bytes)` pins bytes already read (`run`'s start pins the very bytes its home-tree check keyed)»
  source: .andromeda/architecture-detail/occupied-resources.md:114 «`Present` (a concurrent first start placed the copy first) re-hashes the copy that stands: equal bytes are reused, and different bytes are `PinError::HashMismatch`, the file left as found»
  replace-with: «`pin_bytes(home, bytes)` pins bytes already read (`run`'s start pins the very bytes its home-tree check keyed), an absent copy placed once and never replaced through `pin::place_absent` (`fs::place_private_new` at `DIR_MODE`, no clobber): `Written` is ok, `Present` (a concurrent first start placed it first) re-hashes the copy that stands, equal bytes reused and different bytes `PinError::HashMismatch` with the file left as found»
