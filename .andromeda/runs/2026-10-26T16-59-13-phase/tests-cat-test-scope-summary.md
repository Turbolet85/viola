# tests · Test Scope Summary — category read

## Carried
- (.andromeda/test-plan.md:24) The chunk's test breadth is the Comprehensive tier: every new strict-modes, re-hash and single-writer control gets its negative test, on every OS whose branch it touches.
  > **Test tier:** `Comprehensive (2)`
- (.andromeda/test-plan.md:48) `viola-state`'s strict-modes and Unix modes are a named coverage entity; the instance check this chunk adds before every role-file open must be covered as that entity's behaviour.
  > **Entity:** `viola-state`: ndjson logs, atomic snapshots, `File::lock` on `.lock` siblings, torn-line healing, tailing, pid + start-time liveness, rebuild by replay, Unix modes and strict-modes
- (.andromeda/test-plan.md:51) Unix strict-modes and pinned-exe hash mismatch must be proven in a temp `--home`; the Windows foreign-owner-SID and non-persistent-ACL cases (CARRY 12 / CARRY 14's "a home another user owns") must be stubbed at the descriptor-check layer, never claimed as runner-measured.
  > **Reason if not fully testable:** Unix modes and strict-modes, torn lines, snapshot replay, heartbeat staleness and pinned-exe hash mismatch are testable in a temp `--home`. Windows strict-modes cases that need a foreign-owner SID or a volume without persistent ACLs (FAT/exFAT) cannot be provisioned on windows-2025 hosted runners. They get stubs at the descriptor-check layer, per the security excerpt's Config classification ("partially testable with stubs for the Windows DACL cases").
- (.andromeda/test-plan.md:67) The code-bearing state this chunk hardens (snapshot, stamps, `plugin/` files, the pinned exe's 16-hex SHA-256 re-hash) is a named coverage entity whose integrity behaviour must be tested.
  > **Entity:** config and code-bearing state: `snapshot.json`, `config.json`, `settings.json`, `budget.json`, `ledger/stamps.json`, the `plugin/` files (`hooks.json` / `.mcp.json`), the pinned exe (SHA-256 16-hex re-hash), the `heartbeat` file
- (.andromeda/test-plan.md:70) Everything about that state except the Windows DACL limitation must be tested in a temp home.
  > **Reason if not fully testable:** same Windows DACL limitation as `viola-state`. Everything else is testable in a temp home.
- (.andromeda/test-plan.md:88) A home owned by another OS user (CARRY 14) cannot be driven on hosted runners; the chunk must not claim a CI proof of it.
  > Cross-OS-user attacks (a second local user squatting or connecting to the endpoint, reading the home). Hosted runners provide one account (security excerpt, Vectors 1 and 7).
- (.andromeda/test-plan.md:89) Non-persistent-ACL volumes and foreign-owner SIDs stay an untestable zone on hosted runners; their cases go only through stubs.
  > Windows volumes without persistent ACLs and foreign-owner SIDs on hosted runners (security excerpt, Vector 7 and Data Classifications).
- (.andromeda/test-plan.md:131) Test homes must be self-bootstrapped with stamps only from `viola verify` and no developer-seeded state, which binds the seeded-home questions of CARRY 6 and CARRY 13 (a home a test makes before viola must not be the product's proof).
  > **Test data strategy:** self-bootstrapping per test in a fresh `--home`: the fake agent is the child and replays recorded fixtures, `budget.json` is seeded through `viola hook statusline`, stamps come from running `viola verify` against the fake agent, and property / fuzz generators cover the parser surfaces; fixtures never come from real prompts and are checked for absolute paths and usernames, and no state is developer-seeded.
- (.andromeda/test-plan.md:184) The server-identity negative must keep its three outcomes once the strict-modes check runs ahead of it (CARRY 18).
  > server pid / start-time mismatch vs `snapshot.json` → CLI exit 21 / MCP `instance-unreachable` / hook exit 0 with no body
- (.andromeda/test-plan.md:193) The hook statusline arm's fail-open coverage must keep proving the user command never runs on a home that fails strict-modes.
  > `statusline_command` must not run when the home fails strict-modes.
- (.andromeda/test-plan.md:201) Every entry point this chunk adds to the strict-modes set must have a `--home` / `VIOLA_DIR` strict-modes negative with this exit split (CARRYs 3, 7–10, 15).
  > `--home` / `VIOLA_DIR` failing strict-modes → exit 1 / 21 / hook 0
- (.andromeda/test-plan.md:204) CARRY 4's completeness case for `tests/cli_controls_not_disableable.rs` must prove no env var, flag or config key disables a control.
  > no env / flag / config disables a control
- (.andromeda/test-plan.md:208) The filesystem vector's negatives bind this chunk directly: a group/world-writable home or instance dir refused, a pinned-exe hash mismatch exit 1, and `plugin/` rewritten on start.
  >   - **Required test type:** negative tests covering:
  >     - group/world-writable home or instance dir → refused
  >     - pinned exe hash mismatch → exit 1
  >     - `plugin/` rewritten on start, and on Unix the instance's `settings.json` (none is written on Windows)
- (.andromeda/test-plan.md:214) The Windows DACL owner / ACE cases (CARRY 12's Windows proof of `check_instance`, CARRY 5's `conpty/` set) must be run on the runner where it allows and stubbed otherwise.
  > Windows DACL owner / ACE cases where the runner allows, stubbed otherwise (section 1)
- (.andromeda/test-plan.md:250) The Windows DACL / strict-modes branch must be tested on its own `windows-2025` runner, not inferred from Unix.
  > OS-branch tests cover named pipe vs Unix socket, ConPTY vs openpty, Windows DACL / strict-modes vs Unix modes, and macOS euid + dir-only server verification.
- (.andromeda/test-plan.md:265) The diagnostics file this chunk guards with the DACL check must remain 0600 under the sanitization sweep.
  > `diagnostics/` must be 0600.
- (.andromeda/test-plan-detail/test-scope-summary.md:12) The landed server-verification negatives (CLI frame verbs, hook impostor rows, MCP handlers) must keep passing with the strict-modes check placed before each snapshot read; their impostor and test-wrapper homes must pass that check.
  > **As landed (server verification):** the pid/start-time server verification is asserted on Linux and Windows: the eight CLI frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`) by `tests/security_negatives_server_identity.rs` (an impostor server gets no frame and the verb exits 21, beside a control that reaches the recorded wrapper; §6 Security control negatives), the hook by `tests/hook_fail_open.rs`'s impostor rows (exit 0, no body, no frame), the MCP frame tools by `crates/viola-mcp/tests/handlers.rs` (`instance-unreachable`, no frame)
- (.andromeda/test-plan-detail/test-scope-summary.md:33) A strict-modes refusal on a widened verb must stay inside the closed exit set; no new exit-cause code is added (that is "Exit-cause code catalogue"'s).
  > global `--home`, exits 0/1/2/10–14/20/21
- (.andromeda/test-plan-detail/test-scope-summary.md:36) The as-landed note that only `link` / `unlink` run `check_instance` before any role file or log opens must be restated once the other six frame verbs run it in the same order.
  > **As landed (`link` / `unlink`):** of the eight CLI frame verbs only `link` / `unlink` run `viola_state::strict::check_instance` on the driven instance, before any role file or log opens (their refusal: §5 On-disk)
- (.andromeda/test-plan-detail/test-scope-summary.md:44) The widened verbs' refusals must be asserted on the CLI signal: the exit code plus one JSON document.
  > **Signal:** exit code (0, 1, 2, 10–14, 20, 21) plus one JSON document on stdout (`{"v":1,"ok":…}` / `{"v":1,"refusal":…,"detail":…}` / `{"v":1,"error":…}`).
- (.andromeda/test-plan-detail/test-scope-summary.md:80) Harness `boot` must surface a `run` start refusal (a tampered pinned copy included) verbatim, so a strict-modes or re-hash refusal at start reaches the harness unaltered.
  > On readiness failure it exits non-zero with a JSON reason on stdout. A `run` refusal to start (exit 1: a `live` or `stale` name, a tampered pinned copy, or a `.cmd`/`.bat` child) is surfaced verbatim.
- (.andromeda/test-plan-detail/test-scope-summary.md:114) Stamps must come only from `viola verify` (the single stamp writer); no test or harness writes `stamps.json`.
  > Stamps must therefore be produced by running `viola verify` against the fake agent, or the conflict must be resolved upstream. Resolved in §12 (initial entry, ratified by User review 1): the first option is used.
- (.andromeda/test-plan-detail/test-scope-summary.md:121) `run`'s start order must hold with the new checks inside it, and the ConPTY sideload stays fail-open, so a `conpty/` strict-modes failure (CARRY 5) leaves the child on the inbox ConPTY.
  > Collision check, pinned copy and plugin folder, the ConPTY sideload (Windows x64, fail-open: any failure leaves the child on the inbox ConPTY), version gate, exclusive endpoint bind, first snapshot and heartbeat, start `wheel` and `budget-gate` events, and only then the child spawn.
- (.andromeda/test-plan-detail/test-scope-summary.md:125) The per-start plugin rewrite must be verified: `plugin/` rewritten on every start, and on Unix only the 0600 `settings.json` override with the absolute pinned path.
  > `snapshot.json` has `pinned_bin`, `pid`, `started_at`, `child_pid` (`endpoint` from "Wrapper channel" on); `plugin/` is rewritten, and on Unix the instance's `settings.json` (the statusline override: 0600, the absolute pinned path, only for a pinned path of plain characters); on Windows no `settings.json` is written.
- (.andromeda/test-plan-detail/test-scope-summary.md:127) A tampered pinned exe must exit 1 at start (the pinned-binary re-hash, CARRY 16's home copy included).
  > A duplicate `run` exits 1. A tampered pinned exe (hash mismatch) exits 1.
- (.andromeda/test-plan-detail/test-scope-summary.md:173) The replay's recovered field set and its no-file rule bind the readers' move onto `read_snapshot_or_replay` (CARRY 19), which is held for the founder's P4 ruling and neither planned in nor out until then.
  > corrupt `snapshot.json` or give it an unsupported `v`: the replay recovers only `links`, `agent_session_id`, `wheel`, `budget_paused`, `budget_override_until`, with `dialog_pending:false`. As landed, the replay recovers `links` from the logged `link` / `unlink` lines as `{driver, driven, since}` items (chunk 2026-10-10-session-links), the replay writes no file, and `viola revive` is its first product reader (chunk 2026-10-10-viola-revive); the replay gives no `cwd`, a field no event carries

## Bindings
- **strict-modes exit split ↔ body §5 On-disk**: the §1 negative's `exit 1 / 21 / hook 0` is the same split §5's On-disk pattern states (exit 1 for `run` / `mcp` / `viola revive`, hook 0, CLI 21); both sides must agree for every verb this chunk widens (.andromeda/test-plan.md:201)
- **server-verification negatives ↔ body §6 Security control negatives and Exit-cause matrix; security.md §Local trust boundary's dated exceptions**: the landed impostor negatives must keep passing while the strict-modes check that closes each dated exception (CARRY 18) runs before them (.andromeda/test-plan-detail/test-scope-summary.md:12)
- **`link` / `unlink` as-landed note ↔ security.md §Local trust boundary**: the note names the founder's instance-check-first order that the six other frame verbs now take; the note is restated when they do (.andromeda/test-plan-detail/test-scope-summary.md:36)
- **replay recovery ↔ body §6 E5 (as landed: in part) and the founder's P4 fork**: E5 steps 1–3 stay owed to this entry while the readers' move is unruled (.andromeda/test-plan-detail/test-scope-summary.md:173)
- **Windows DACL stubs ↔ body §4 viola-state (Windows strict-modes over synthetic SDDL descriptors) and §8 Filesystem stubbing**: the cases hosted runners cannot provision are stubbed at the descriptor-check layer only (.andromeda/test-plan.md:51)
- **no developer-seeded state ↔ body §5 setup carve-out (Windows x64 `seed_conpty`) and §7 seed strategies**: CARRY 6 / CARRY 13 decide whether the seeded-home carve-out survives a home viola must create first (.andromeda/test-plan.md:131)
- **single stamp writer ↔ body §6 Security control negatives (single writers of `stamps.json` and `snapshot.json`) and §11 Test Data ban on hand-written state files** (.andromeda/test-plan-detail/test-scope-summary.md:114)
- **tampered pinned exe ↔ body §6 Path 1 and Contract suite (pinned-exe content hash pinned by literal vectors)** (.andromeda/test-plan-detail/test-scope-summary.md:127)
- **plugin rewrite on start ↔ body §6 Path 1 (a restart rewrites `plugin/` with absolute hook commands)** (.andromeda/test-plan-detail/test-scope-summary.md:125)
- **no control disableable ↔ body §5 CLI pattern ("no setting that disables a control") and CARRY 4's `tests/cli_controls_not_disableable.rs`** (.andromeda/test-plan.md:204)
- **statusline under strict-modes ↔ body §6 Path 6 (`statusline_command` never runs when strict-modes fails)** (.andromeda/test-plan.md:193)
