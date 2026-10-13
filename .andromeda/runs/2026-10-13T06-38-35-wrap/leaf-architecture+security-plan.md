group: architecture+security-plan
read: .claude/rules/api.md · 45 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/rules/api.md:21   class: stale
  leaf-text: «`list` is not a channel method — it reads the disk + `claude agents --json` caller-side.»
  source: .andromeda/architecture.md:53 «`list` is a caller-side read of the disk and `claude agents --json`, not a channel request (as landed it reads the disk only: the `claude agents --json` read is owed to "Unwrapped sessions on the board")»
  replace-with: «`list` is not a channel method — it reads the disk (and, by contract, `claude agents --json`) caller-side and needs no live wrapper; its `ok` payload is `{items, budget, skipped}` in the `/api/sessions` shapes. As landed CLI `list` reads `<home>/instances/` and `budget.json` only and spawns no `claude agents --json`: every item is `wrapped: true` with `status: "unknown"` and no unwrapped row is listed (owed to "Unwrapped sessions on the board").»

- leaf: .claude/rules/api.md:29   class: omission
  leaf-text: «Human text by default, `--json` for agents: one JSON document»
  source: .andromeda/architecture.md:101 «One verb has no machine form yet: `viola revive` takes no `--json`, so its `--list` rows and its refusals are the human lines and the typed exit only»
  replace-with: «Human text by default, `--json` for agents (`viola revive` takes none yet: human lines and the typed exit only, owed to "CLI machine contract"): one JSON document»

- leaf: .claude/rules/api.md:30   class: omission
  leaf-text: «1 internal / `run` start refusal»
  source: .andromeda/architecture.md:139 «`viola revive` refusing its strict-modes check, which runs before the wrapper log opens and outside the preflight (`unable: <name>'s state files can be written by another user` and its `hint:` on stderr, exit 1»
  replace-with: «1 internal / a `run` start refusal, a `revive` strict-modes or preflight refusal, a `verify` failing row or refusal, `list`'s `instances/` that cannot be listed or failed log start»

- leaf: .claude/rules/api.md:30   class: omission
  leaf-text: «21 instance unreachable.»
  source: .andromeda/architecture.md:147 «also `viola list`'s refused strict-modes check of the home, `instances/` or any one instance, which refuses the whole board (`--json` `{"v":1,"error":"instance-unreachable","detail":null}`, no log opened)»
  replace-with: «21 instance unreachable, and `viola list`'s refused strict-modes check of the home, `instances/` or any one instance, which refuses the whole board (`--json` `{"v":1,"error":"instance-unreachable","detail":null}`, no log opened).»

- leaf: .claude/rules/api.md:31   class: stale
  leaf-text: «`--home`/`VIOLA_DIR` canonicalised and strict-modes-checked before any read (interim ratified exceptions until Epoch 6: `viola hook` shape-checks `VIOLA_DIR` only, and `verify` reads `ledger/stamps.json` before strict-modes — security-plan Decisions Log `2026-09-28`; `run`'s version gate reads it through `read_stamps_strict`).»
  source: .andromeda/security-plan.md:236 «`viola list` makes the home absolute with `std::path::absolute` and does not canonicalise it, as every `cli` verb resolves `--home` today (the canonicalisation is owed to the route entry "CLI machine contract"»
  replace-with: «`--home`/`VIOLA_DIR` canonicalised and strict-modes-checked before any snapshot, `statusline_command` or ledger read — as landed every `cli` verb makes `--home` absolute (`std::path::absolute`) without canonicalising it (owed to "CLI machine contract"), and `list`'s `board::scan` runs `check_path` on the home and `instances/` and `check_instance` per instance before any file under the home is opened. Interim ratified gaps until Epoch 6: `viola hook <event>` shape-checks `VIOLA_DIR` only (`hook statusline` canonicalises its instance directory and runs `check_instance` before its log); `verify` reads `ledger/stamps.json` before strict-modes — security-plan Decisions Log `2026-09-28`; `run`'s version gate reads it through `read_stamps_strict`; `send`, `wait`, `last`, `answer`, `pause` and `release` read `snapshot.json` without strict-modes after a liveness-only pre-check.»
