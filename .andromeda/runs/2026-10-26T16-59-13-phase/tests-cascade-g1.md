# tests · cascade read · group 1 of 6

## Carried
- (.andromeda/test-plan-detail/test-scope-summary.md:121) Path 1's start sequence must hold with the chunk's checks added ahead of it: the pinned copy re-hashed before reuse, `plugin/` rewritten on every start, the Windows ConPTY companions joining the strict-modes set while a failing one still leaves the child on the inbox ConPTY.
  > **Path:** `run` start sequence. Collision check, pinned copy and plugin folder, the ConPTY sideload (Windows x64, fail-open: any failure leaves the child on the inbox ConPTY), version gate, exclusive endpoint bind, first snapshot and heartbeat, start `wheel` and `budget-gate` events, and only then the child spawn. A second `run` of the same name, or a squatted endpoint, exits 1.
  (.andromeda/test-plan-detail/test-scope-summary.md:125)
  > `snapshot.json` has `pinned_bin`, `pid`, `started_at`, `child_pid` (`endpoint` from "Wrapper channel" on); `plugin/` is rewritten, and on Unix the instance's `settings.json` (the statusline override: 0600, the absolute pinned path, only for a pinned path of plain characters); on Windows no `settings.json` is written.
  (.andromeda/test-plan-detail/test-scope-summary.md:127)
  > A duplicate `run` exits 1. A tampered pinned exe (hash mismatch) exits 1.
  (.andromeda/test-plan.md:135)
  > **Path:** `run` start sequence — the collision check, pinned copy and plugin folder, the fail-open ConPTY sideload, version gate, exclusive endpoint bind, first snapshot and heartbeat, and the start `wheel` and `budget-gate` events all precede the child spawn; a second `run` of the same name, a squatted endpoint or a tampered pinned exe exits 1.
  (.andromeda/test-plan.md:67)
  > **Entity:** config and code-bearing state: `snapshot.json`, `config.json`, `settings.json`, `budget.json`, `ledger/stamps.json`, the `plugin/` files (`hooks.json` / `.mcp.json`), the pinned exe (SHA-256 16-hex re-hash), the `heartbeat` file
  (.andromeda/test-plan.md:70)
  > **Reason if not fully testable:** same Windows DACL limitation as `viola-state`. Everything else is testable in a temp home.
  (.andromeda/test-plan.md:210)
  > pinned exe hash mismatch → exit 1
  >     - `plugin/` rewritten on start, and on Unix the instance's `settings.json` (none is written on Windows)
- (.andromeda/test-plan-detail/test-scope-summary.md:33) The CLI exit contract must carry the strict-modes refusal at every frame verb the chunk reaches (`send`, `wait`, `last`, `answer`, `pause`, `release`), with `run` and `revive` refusing exit 1 ahead of any log.
  > CLI verbs and the `--json` / exit-code contract (`run · revive · send · wait · last · list · answer · pause · release · link · unlink · verify · plugin install`, global `--home`, exits 0/1/2/10–14/20/21; `revive` takes `--json` on both arms (chunk 2026-10-17-cli-machine-contract: an exit-1 refusal of its strict-modes check or its preflight prints `{"v":1,"error":"<code>","detail":null}` on stdout, `<code>` one of `strict-modes-failed`, `already-live`, `no-session`, `session-live`, `cwd-missing`;
  (.andromeda/test-plan-detail/test-scope-summary.md:36)
  > **As landed (`link` / `unlink`):** of the eight CLI frame verbs only `link` / `unlink` run `viola_state::strict::check_instance` on the driven instance, before any role file or log opens (their refusal: §5 On-disk); a `driver` equal to `driven` is refused by the wrapper alone, never client-side (CLI exit 20).
  (.andromeda/test-plan.md:201)
  > `--home` / `VIOLA_DIR` failing strict-modes → exit 1 / 21 / hook 0
  (.andromeda/test-plan.md:204)
  > no env / flag / config disables a control
  (.andromeda/test-plan.md:209)
  > group/world-writable home or instance dir → refused
  (.andromeda/test-plan.md:214)
  > Windows DACL owner / ACE cases where the runner allows, stubbed otherwise (section 1)
  (.andromeda/test-plan.md:48)
  > **Entity:** `viola-state`: ndjson logs, atomic snapshots, `File::lock` on `.lock` siblings, torn-line healing, tailing, pid + start-time liveness, rebuild by replay, Unix modes and strict-modes
  (.andromeda/test-plan.md:51)
  > **Reason if not fully testable:** Unix modes and strict-modes, torn lines, snapshot replay, heartbeat staleness and pinned-exe hash mismatch are testable in a temp `--home`. Windows strict-modes cases that need a foreign-owner SID or a volume without persistent ACLs (FAT/exFAT) cannot be provisioned on windows-2025 hosted runners. They get stubs at the descriptor-check layer, per the security excerpt's Config classification ("partially testable with stubs for the Windows DACL cases").
  (.andromeda/test-plan.md:88)
  > Cross-OS-user attacks (a second local user squatting or connecting to the endpoint, reading the home). Hosted runners provide one account (security excerpt, Vectors 1 and 7).
  >   - Windows volumes without persistent ACLs and foreign-owner SIDs on hosted runners (security excerpt, Vector 7 and Data Classifications).
  (.andromeda/test-plan.md:250)
  > OS-branch tests cover named pipe vs Unix socket, ConPTY vs openpty, Windows DACL / strict-modes vs Unix modes, and macOS euid + dir-only server verification.
  (.andromeda/test-plan.md:873)
  > NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
- (.andromeda/test-plan-detail/test-scope-summary.md:44) Each CLI strict-modes negative the chunk adds must read its verdict from the exit code (21 for a frame verb, 1 for `run` / `revive`) and the one JSON document on stdout.
  > **Signal:** exit code (0, 1, 2, 10–14, 20, 21) plus one JSON document on stdout (`{"v":1,"ok":…}` / `{"v":1,"refusal":…,"detail":…}` / `{"v":1,"error":…}`).
- (.andromeda/test-plan-detail/test-scope-summary.md:67) `viola mcp`'s frame tools refused by the strict-modes check the chunk adds through `live_endpoint` must return exactly the `instance-unreachable` result with `detail: null`, no frame written.
  > `isError:true` + `{"error":"instance-unreachable","detail":null}` | `{"error":"wrapper-fault","detail":{code,message,data}}` | `{"error":"invalid-params","detail":"<field>"}` (the closed field words `target · text · after · timeout_ms · dialog_id · response`, no frame written), each text `refused: <code>`.
  (.andromeda/test-plan-detail/test-scope-summary.md:66)
  > **Driver:** language-native MCP stdio test client. It spawns `viola mcp --home <tmp>` and speaks JSON-RPC over stdin/stdout, with JSON schema assertion.
- (.andromeda/test-plan-detail/test-scope-summary.md:12) The as-landed server-verification negatives must keep failing at the server check, not at the new strict-modes check: their homes (the impostors' test-built `snapshot.json` included) must pass `check_instance`, on Linux and Windows.
  > **As landed (server verification):** the pid/start-time server verification is asserted on Linux and Windows: the eight CLI frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`) by `tests/security_negatives_server_identity.rs` (an impostor server gets no frame and the verb exits 21, beside a control that reaches the recorded wrapper; §6 Security control negatives), the hook by `tests/hook_fail_open.rs`'s impostor rows (exit 0, no body, no frame), the MCP frame tools by `crates/viola-mcp/tests/handlers.rs` (`instance-unreachable`, no frame), and the decision table by the `viola-state` unit `server_verdict_rows`.
  (.andromeda/test-plan.md:184)
  > server pid / start-time mismatch vs `snapshot.json` → CLI exit 21 / MCP `instance-unreachable` / hook exit 0 with no body
- (.andromeda/test-plan-detail/test-scope-summary.md:77) The harness `boot` must create a home that passes the new checks at every verb, surface a tampered pinned copy's exit 1 verbatim, and is the instrument the `pin_exe` race is measured with (two instances on a fresh home); `cleanup`'s exit 21 must still hold once a deleted home meets the check.
  > Builds `viola` and creates a fresh temp home, passed as `--home` (arch excerpt, Per-test isolation).
  (.andromeda/test-plan-detail/test-scope-summary.md:78)
  > Starts one or more `viola run <name> -- <fake agent> <args>` wrappers.
  (.andromeda/test-plan-detail/test-scope-summary.md:80)
  > On readiness failure it exits non-zero with a JSON reason on stdout. A `run` refusal to start (exit 1: a `live` or `stale` name, a tampered pinned copy, or a `.cmd`/`.bat` child) is surfaced verbatim.
  (.andromeda/test-plan-detail/test-scope-summary.md:95)
  > Deletes the temp home and verifies the endpoint is gone: a CLI verb against the name returns exit 21, and the Unix socket file or pipe no longer exists.
- (.andromeda/test-plan-detail/test-scope-summary.md:108) The single-writer proof and the `seed_conpty` move are this chunk's own obligation: stamps come only from `viola verify` against the fake agent, and no test pre-seeds a home viola must create.
  > Self-bootstrapping per test: a fresh `--home` temp dir.
  (.andromeda/test-plan-detail/test-scope-summary.md:114)
  > Stamps must therefore be produced by running `viola verify` against the fake agent, or the conflict must be resolved upstream. Resolved in §12 (initial entry, ratified by User review 1): the first option is used.
  (.andromeda/test-plan-detail/test-scope-summary.md:115)
  > No developer-seeded state.
  (.andromeda/test-plan.md:131)
  > **Test data strategy:** self-bootstrapping per test in a fresh `--home`: the fake agent is the child and replays recorded fixtures, `budget.json` is seeded through `viola hook statusline`, stamps come from running `viola verify` against the fake agent, and property / fuzz generators cover the parser surfaces; fixtures never come from real prompts and are checked for absolute paths and usernames, and no state is developer-seeded.
  (.andromeda/test-plan.md:905)
  > NEVER hand-write `ledger/stamps.json`, `snapshot.json` or `budget.json`. Stamps come from `viola verify`, snapshots from the wrapper and budget from `viola hook statusline`. The ban binds every booted-wrapper test.
- (.andromeda/test-plan-detail/test-scope-summary.md:155) The three `hook_statusline` cases the chunk re-reads on `windows-2025` assert Path 6's signals, and must pass once `seed_conpty` lets viola create the home first; `statusline_command` must not run when the home fails strict-modes.
  > **Path:** budget governor. `viola hook statusline` writes `rate_limits` to `budget.json` (only when the payload holds a `rate_limits` object) and passes the user's statusline stdout through unchanged (on Unix only: on Windows `run` writes no `settings.json` override and the arm runs no command).
  (.andromeda/test-plan-detail/test-scope-summary.md:158)
  > `budget.json` holds the fed percentages. A malformed `resets_at` becomes `"unknown"`.
  (.andromeda/test-plan-detail/test-scope-summary.md:159)
  > Statusline stdout equals the user command's stdout (empty on failure), exit 0. The equality is Unix only; on Windows the arm runs no command and prints nothing.
  (.andromeda/test-plan.md:193)
  > **Required test type:** fail-open tests on oversize stdin, malformed JSON, clap error, channel failure, a forced panic (on the real binary, through the `fake-agent`-only `FAKE_AGENT_HOOK_PANIC` seam; see §6 Security sweep), and a missing `VIOLA_NAME`. Each must give exit 0 with empty stderr and no body. `statusline_command` must not run when the home fails strict-modes.
- (.andromeda/test-plan-detail/test-scope-summary.md:173) The readers' move onto the replay is the founder's P4 fork: only if his answer moves them does a corrupt or newer `snapshot.json` recover by replay at the readers, the classified read taken after `check_instance`.
  > corrupt `snapshot.json` or give it an unsupported `v`: the replay recovers only `links`, `agent_session_id`, `wheel`, `budget_paused`, `budget_override_until`, with `dialog_pending:false`. As landed, the replay recovers `links` from the logged `link` / `unlink` lines as `{driver, driven, since}` items (chunk 2026-10-10-session-links), the replay writes no file, and `viola revive` is its first product reader (chunk 2026-10-10-viola-revive); the replay gives no `cwd`, a field no event carries
  (.andromeda/test-plan.md:174)
  > Snapshot corruption → replay recovery.
  (.andromeda/test-plan.md:237)
  > a corrupt or unsupported-`v` snapshot recovered by replay

## Bindings
- **scope Path 1 ↔ e2e-test-strategy Path 1 and the contract suite**: the pinned-copy re-hash, the tampered-exe exit 1 and the per-start `plugin/` rewrite of the scope's path are asserted by the Path 1 scenario's signal (.andromeda/test-plan.md:488) and the pinned-exe hash by literal vectors (.andromeda/test-plan.md:617).
- **scope CLI exit contract ↔ integration-test-strategy On-disk**: the strict-modes refusal's per-surface exits (1 for `run` / `mcp` / `revive`, hook 0 with no output, CLI 21) are the integration pattern (.andromeda/test-plan.md:437), and the security sweep's `--home` strict-modes refusals (.andromeda/test-plan.md:598).
- **scope Windows strict-modes ↔ mocking-stubbing-discipline and ci-integration**: the foreign-SID / non-persistent-ACL cases are synthetic SDDL descriptors fed to the check function (.andromeda/test-plan.md:707), and the Windows DACL legs are proved on the `windows-2025` matrix leg (.andromeda/test-plan.md:767).
- **scope test data ↔ test-data-fixtures seed strategies and integration setup**: the single writers and the `seed_conpty` carve-out (.andromeda/test-plan.md:631, .andromeda/test-plan.md:633) and the Windows x64 seeding carve-out of the setup lifecycle (.andromeda/test-plan.md:427).
- **scope server verification ↔ e2e-test-strategy Security control negatives**: the impostor negatives and the single-writer and umask-independent-mode negatives share one suite (.andromeda/test-plan.md:606).
- **scope MCP surface ↔ integration-test-strategy MCP**: the handler tests' `TestWrapper` writes its snapshot through `write_snapshot` and must stand on a home that passes the check (.andromeda/test-plan.md:449).
- **scope chaos replay ↔ e2e-test-strategy E5**: steps 1–3 stay unbuilt and owed to this entry, reached only by the founder's P4 answer (.andromeda/test-plan.md:590).
- **scope budget governor ↔ e2e-test-strategy Path 6**: `statusline_command` never runs when strict-modes fails (.andromeda/test-plan.md:528).
