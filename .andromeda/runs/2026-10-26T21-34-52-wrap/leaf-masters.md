group: masters
read: CLAUDE.md · 128 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:23   class: omission
  leaf-text: «the instance strict-modes check»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:70 «the strict-modes checks (`strict`: the write rule and the read rule (`unix_read_verdict`, `windows_read_verdict`, `READ_RIGHTS`), `check_instance`, `check_home_tree` → `TreeRefused`»
  replace-with: «the strict-modes checks (`strict`: the write rule and the read rule, `check_instance`, `check_home_tree` → `TreeRefused`, `check_diagnostics`, `check_present`, `check_stamps`)»

- leaf: CLAUDE.md:23   class: omission
  leaf-text: «the log replay, the link rule»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:68 «the log replay and the session chain (`replay.rs`; read by `viola revive`, `board::read`, `liveness::live_endpoint`, `viola hook`'s `connect`, `hook statusline`'s `serve` and `run`'s collision check)»
  replace-with: «the log replay (`read_snapshot_or_replay`, read by `viola revive`, `board::read`, `live_endpoint`, `viola hook`'s `connect`, `hook statusline`'s `serve` and `run`'s collision check; a replayed snapshot gives no endpoint), the link rule»

- leaf: CLAUDE.md:27   class: omission
  leaf-text: «the `run` pump and its vt100 feed thread,»
  source: .andromeda/architecture.md:320 «Home tree: before any log, `viola run` and `viola revive`'s start check the home tree (`viola_state::strict::check_home_tree`)»
  replace-with: «the `run` pump and its vt100 feed thread, the start's home-tree check (`src/cmd/run.rs`'s `checked_exe` / `refuse_tree` → `viola_state::strict::check_home_tree` on the home, the instance's set, `ledger/`, `bin/`, `plugin/` and the pinned copy's `bin/<key>/` and `plugin/<key>/`, before any log, for `run` and `revive`'s start; exit 1 on a refusal, no child; `verify` checks the home, `ledger/`, `bin/` and `plugin/` the same way before its log),»

- leaf: CLAUDE.md:27   class: omission
  leaf-text: «the `hook statusline` arm (the instance check before anything under the home is opened;»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:31 «hook.rs: the hook's instance check (`canonical` / `trusted`: the `VIOLA_DIR` instance directory as it exists, then `check_instance`), shared by the event, dialog and statusline arms»
  replace-with: «the hook's instance check (`src/cmd/hook.rs`'s `canonical` / `trusted`: `check_instance` on the `VIOLA_DIR` instance, shared by the event, dialog and statusline arms before anything under the home is read; a refused or absent instance exits 0 writing nothing; the two `--capture` arms ahead of it), the `hook statusline` arm (that instance check first;»

- leaf: CLAUDE.md:27   class: omission
  leaf-text: «`check_instance` on the home and `instances/<driven>` before any role file or log, exit 21 on a refusal»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:32 «`refuse_untrusted`, the instance strict-modes check (`check_instance` on the absolute home and `instances/<name>`) that every CLI frame verb (send, wait, last, answer, pause, release)»
  replace-with: «`client::refuse_untrusted` (`src/cmd/client.rs`: `check_instance` on the home and `instances/<driven>`, the check the six CLI frame verbs `send` (after its text read), `wait`, `last`, `answer`, `pause` and `release` run first on `instances/<name>`) before any role file or log, exit 21 on a refusal»
