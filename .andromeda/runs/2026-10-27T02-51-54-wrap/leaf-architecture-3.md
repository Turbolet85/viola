group: architecture-3
read: .claude/docs/services/viola.md · 58 · «- `.andromeda/architecture.md` · `.claude/rules/api.md` · `.»
read: .claude/docs/stack.md · 66 · «5. Commit as `chore: bump {tool} to {version}`»
read: .claude/docs/workflow.md · 67 · «This project uses a **reference-based** CLAUDE.md architectu»

- leaf: .claude/docs/stack.md:17   class: stale
  leaf-text: «tempfile 3.27.0 `persist` through one shared helper (`viola_state::fs::replace_private`: the mode is set on the temp file before the write, then `sync_all` and `persist`) for snapshots and every replaced file;»
  source: .andromeda/architecture-detail/stack-and-technologies.md:18 «and tempfile 3.27.0 `TempPath::persist_noclobber` after `into_temp_path` closes the write handle (`viola_state::fs::place_private_new`) for the pinned copy's absent-copy write, placed once and never»
  replace-with: «tempfile 3.27.0 `persist` through one shared helper (`viola_state::fs::replace_private`: the mode is set on the temp file before the write, then `sync_all` and `persist`) for snapshots and every other replaced file, and tempfile 3.27.0 `TempPath::persist_noclobber` after `into_temp_path` closes the write handle (`viola_state::fs::place_private_new`) for the pinned copy's absent-copy write, placed once and never replaced;»

- leaf: .claude/docs/services/viola.md:20   class: omission
  leaf-text: «pinned copy (SHA-256 re-hash on reuse; a mismatch → exit 1 `pinned-hash-mismatch`)»
  source: .andromeda/architecture.md:319 «the pinned copy of the binary, which `viola run` places once when it is absent, with no clobber, and never replaces (0700 on Unix), where `<hash>` is the first 16 hex of the SHA-256 of the exe's content»
  replace-with: «pinned copy (placed once when absent, with no clobber, through `place_private_new`, and never replaced; SHA-256 re-hash on reuse and of a copy a concurrent first start placed first; a mismatch → exit 1 `pinned-hash-mismatch`, the file left as found)»

- leaf: .claude/docs/services/viola.md:37   class: omission
  leaf-text: «the content hash in `bin/`/`plugin/` keys dev rebuilds (a hook fix applies only to sessions started after the rebuild).»
  source: .andromeda/architecture.md:69 «one tree can build two files whose loaded sections are equal and so key two copies, its cause not explained and acting on it owed to the working-route entry "Version done-check".»
  replace-with: «the content hash in `bin/`/`plugin/` keys dev rebuilds (a hook fix applies only to sessions started after the rebuild); one tree can build two files whose loaded sections are equal and so key two copies, its cause not explained and acting on it owed to the working-route entry "Version done-check".»
