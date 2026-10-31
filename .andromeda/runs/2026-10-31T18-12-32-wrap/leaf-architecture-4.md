group: architecture-4
read: .claude/rules/events.md · 40 · «- 2026-10-16: A snapshot field that a reader expects once an»
- leaf: .claude/rules/events.md:31   class: omission
  leaf-text: «(`LiveEndpoint`, only on a real snapshot: a replayed or absent one, or a failed read, gives none)»
  source: .andromeda/architecture-detail/standard-contracts.md:140 «`live_endpoint` gives none for a stale wrapper, so a frame verb against a stale instance reads `instance-dead`.»
  replace-with: «(`LiveEndpoint`, only on a real snapshot: a replayed or absent one, a failed read or a stale wrapper gives none; where it gives none, `viola_state::liveness::without_endpoint`, behind the strict-modes check at every caller, reads the cause through `symlink_metadata`, opening nothing and writing no line: a directory reads `InstanceDead`, so a frame verb against a stale instance reads `instance-dead`, and anything else `UnwrappedName`, a reading still PROVISIONAL)»
- leaf: .claude/rules/events.md:20   class: omission
  leaf-text: «SessionEnd → `session-end`»
  source: .andromeda/architecture-detail/established-decisions.md:147 «`Source::Hook`, which writes nothing while `events.ndjson.lock` is held), else nothing; `hooks.json` gives it no `timeout` key. `run` writes `session-end` from the process exit if no hook arrived.»
  replace-with: «SessionEnd → `session-end` (over the channel, else, once the instance check has passed and the endpoint is not reached or its server check fails, the hook's direct `try_lock` append, `viola_state::try_append_event` with `Source::Hook`, which writes nothing while `events.ndjson.lock` is held, else nothing; `run` writes `session-end` from the process exit if no hook arrived)»
- leaf: .claude/rules/events.md:20   class: omission
  leaf-text: «long-paste wrapper removed»
  source: .andromeda/architecture.md:205 «`text` is the submitted prompt as it was typed, the long-paste wrapper and its frame removed and tag escaping reversed»
  replace-with: «long-paste wrapper and the frame the CLI writes around it removed»
- leaf: .claude/rules/events.md:30   class: omission
  leaf-text: «The replay writes no file, gives no `cwd`»
  source: .andromeda/architecture-detail/standard-contracts.md:25 «`replay::read_snapshot_or_replay` replays the log for an unreadable or unsupported snapshot, in one pass, and logs one `state-recovered` line; an absent snapshot is neither replayed nor logged»
  replace-with: «The replay logs one `state-recovered` line into the role file of the process that read it (an absent snapshot is neither replayed nor logged), writes no file, gives no `cwd`»
