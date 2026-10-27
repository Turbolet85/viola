# Apply — architecture · part 5 of 5

## Verdicts
- fanout-architecture.md 7 · applied · .andromeda/registries/contracts/architecture/project-directory-structure.md:68#1
- fanout-architecture.md 8 · applied · .andromeda/registries/contracts/architecture/project-directory-structure.md:70#1
- fanout-architecture.md 9 · applied · .andromeda/registries/contracts/architecture/project-directory-structure.md:32#1
- fanout-architecture.md 10 · applied · .andromeda/registries/contracts/architecture/project-directory-structure.md:31#1
- cascade-compare-architecture.md 1 · applied · .andromeda/architecture-detail/cross-cutting-patterns.md:9+#1

## Edits
- line: .andromeda/registries/contracts/architecture/project-directory-structure.md:31
  old: «hook/statusline.rs: the `hook statusline` arm (the instance check,»
  new: «hook.rs: the hook's instance check (`canonical` / `trusted`: the `VIOLA_DIR` instance directory as it exists, then `check_instance`), shared by the event, dialog and statusline arms and taken before anything under the home is read (§Cross-cutting Patterns, Config management), and the event and dialog paths' `connect` (the snapshot through the replay, a replayed one unreachable); hook/statusline.rs: the `hook statusline` arm (the instance check through hook.rs's `trusted`,»
- line: .andromeda/registries/contracts/architecture/project-directory-structure.md:32
  old: «client.rs (a helper, no subcommand): `connect`,»
  new: «client.rs (a helper, no subcommand): `refuse_untrusted`, the instance strict-modes check (`check_instance` on the absolute home and `instances/<name>`) that every CLI frame verb (send, wait, last, answer, pause, release) and `link` / `unlink` run first (refused: exit 21 with the instance pair, or the `instance-unreachable` document under `--json`; nothing opened under the home, no line); `connect`,»
- line: .andromeda/registries/contracts/architecture/project-directory-structure.md:68
  old: «read by `viola revive` and `board::read`»
  new: «read by `viola revive`, `board::read`, `liveness::live_endpoint`, `viola hook`'s `connect`, `hook statusline`'s `serve` and `run`'s collision check»
- line: .andromeda/registries/contracts/architecture/project-directory-structure.md:70
  old: «the instance strict-modes check (`strict::check_instance`)»
  new: «the strict-modes checks (`strict`: the write rule and the read rule (`unix_read_verdict`, `windows_read_verdict`, `READ_RIGHTS`), `check_instance`, `check_home_tree` → `TreeRefused`, `check_diagnostics`, `check_present`, `check_stamps`)»
- after: .andromeda/architecture-detail/cross-cutting-patterns.md:9
  anchor: «  - Viola home: a global `--home <dir>` flag on every subcommand overrides the default `<user home>/.viola/`. Resolution order: `--home`, then the grandparent of `VIOLA_DIR` when it is set, then the default. As landed (chunk 2026-10-17-cli-machine-contract): `resolve_home(flag, name, dir, user_home)` in `src/cmd/mod.rs` resolves `--home` as given, else the `instance_of` home when both `VIOLA_NAME` and `VIOLA_DIR` pass the one shape check (a valid `ViolaName`, an absolute dir ending `instances/<name>`), else `<user home>/.viola`, a failing pair falling through; `resolved_home` reads the two variables and `std::env::home_dir()` once, at every verb arm of `dispatch`; `canonical_home` canonicalises a home that exists (`std::fs::canonicalize`, a Windows `\\?\C:\…` written back as `C:\…`, every other verbatim form kept) and keeps `std::path::absolute`'s spelling for one that does not, creating nothing; `run`'s start canonicalises the home after its wrapper log has created it, before the collision check, the pin and the endpoint bind. `viola run --home <dir>` passes the home to the child only through `VIOLA_DIR`, so the pinned copy, plugin folder, endpoint hash, logs, `budget.json` and ledger stamps all follow it. The statusline source follows the home too: `<home>/statusline-source.json` is the source whenever it exists, and only the default home (`canonical_home(default_home(user_home))`, `default_home` shared by `resolve_home` and the source rule) falls back to the user's `<user home>/.claude/settings.json`, so a home named by `--home` or reached through `VIOLA_DIR` never reads the user's settings. The rule has no `config.json` key and no environment variable; `start` reads `std::env::home_dir()` for it, the read `resolved_home` already made. This is the isolation point for tests: each test runs in its own home, and that home's `ledger/stamps.json` can stamp the fake agent's reported version.»
  add: «  - `viola hook` takes the `instance_of` session only when its `VIOLA_DIR` instance directory exists (`canonical`) and passes `check_instance` (`trusted`; both in `src/cmd/hook.rs`, shared by the event, dialog and statusline arms), before `diagnostics_level` is read from `config.json` or obs starts: a `VIOLA_DIR` naming no existing instance directory, or an instance the check refuses, makes the hook exit 0 at once with no `config.json` read, no role file, no stdin read, no line, no frame and no SessionEnd append. The two `--capture` arms, which read no `VIOLA_*`, stay ahead of it.»

## Patterns
[[pattern]]
id = "arch5-01"
fixed = 'hook/statusline.rs: the `hook statusline` arm (the instance check,'

[[pattern]]
id = "arch5-02"
fixed = 'read by `viola revive` and `board::read`'
