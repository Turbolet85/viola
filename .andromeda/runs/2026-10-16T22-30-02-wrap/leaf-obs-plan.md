group: obs-plan
read: .claude/docs/obs-summary.md · 76 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »

- leaf: .claude/docs/obs-summary.md:67   class: stale
  leaf-text: «and from `viola revive`'s preflight `no-session|cwd-missing`,»
  source: .andromeda/obs-plan.md:754 «`session-live` (one `claude agents --json` read lists the logged session id live in another `claude`; skipped under `--fork`; a failed, oversize or malformed read passes, fail open;»
  replace-with: «and from `viola revive`'s preflight `no-session|session-live|cwd-missing` (`session-live` skipped under `--fork`, fail open on a failed read, its reading one `agents-probe` spawn pair in `run-<name>.ndjson`),»

- leaf: .claude/docs/obs-summary.md:71   class: omission
  leaf-text: «`state-recovered` per replayed snapshot and the per-key `config-json` threshold lines into `cli-<name>.ndjson`.»
  source: .andromeda/obs-plan.md:788 «the `agents-probe` pair is written by `run::agents::read_logged` for `viola list` (into `cli-<name>.ndjson`, only under a parsed `VIOLA_NAME`)»
  replace-with: «`state-recovered` per replayed snapshot, the per-key `config-json` threshold lines and the `agents-probe` spawn pair of its `claude agents --json` read (chunk 2026-10-16-unwrapped-sessions-on-the-board) into `cli-<name>.ndjson`.»
