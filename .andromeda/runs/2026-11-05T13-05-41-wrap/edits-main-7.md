@fact id=fct-y2h27pfd expect_rev=1
old: plus `cargo deny check` on all three targets.
new: plus `cargo deny check` on its one target (`x86_64-unknown-linux-gnu`).

@fact id=fct-ad44nn2e expect_rev=2
old: Child exit is detected on the process handle, never on EOF, because ConPTY does not close the output stream on exit.
new: Child exit is detected on the process handle, never on EOF.

@fact id=fct-4u2vf5lk expect_rev=1
old: It sits under the current dir, not the OS temp dir, because a temp dir on another Windows drive (`windows-2025`: the checkout on `D:`, the temp dir on `C:`) has no relative path from the current dir.
new: It sits under the current dir, not the OS temp dir.

@fact id=fct-mw7zqeom expect_rev=1
old: never on reader EOF (ConPTY chaos class).
new: never on reader EOF.

@fact id=fct-z43sni6d expect_rev=1
old: (runner images read at chunk 2026-09-24-workspace-tree-and-code-graph-planes: jq 1.8.1 on windows-2025, 1.8.2 on macos-15 arm64, 1.7 on ubuntu-24.04)
new: (runner image read at chunk 2026-09-24-workspace-tree-and-code-graph-planes: jq 1.7 on ubuntu-24.04)

@fact id=fct-lzdm6e7w expect_rev=3
old: `plain_drive_*` (the plain drive form on Windows);
new: `plain_drive_*`;

@fact id=fct-mos7j36w expect_rev=1
old: In the per-OS `test` job, every test artifact
new: In the `test` job, every test artifact

@fact id=fct-47aec6c3 expect_rev=2
old: in the per-OS `test` job)
new: in the `test` job)
old: In the per-OS `test` job, `target/agent-run/`
new: In the `test` job, `target/agent-run/`

@fact id=fct-jv5gqhbl expect_rev=3
old: the kill-and-revive case green on the three CI runners
new: the kill-and-revive case green on a run of the retired three-OS matrix

@fact id=fct-hgewdmj2 expect_rev=3
old: the kill-and-revive case green on the three CI runners
new: the kill-and-revive case green on a run of the retired three-OS matrix

@fact id=fct-nuclnamd expect_rev=1
old: (the Unix R8 pass-list)
new: (the R8 pass-list)

@fact id=fct-2j6rht5l expect_rev=3
old: 8.7–9.9 s on three OSes under `ci` (as measured at ci#39383206773)
new: 8.7–9.9 s under `ci` (as measured at ci#39383206773, a run of the retired three-OS matrix)

@fact id=fct-73ei4s3r expect_rev=1
old: npm-shim resolution
new: child program resolution

@fact id=fct-enn2bam2 expect_rev=1
old: `bin/<version>-<hash>/viola(.exe)`
new: `bin/<version>-<hash>/viola`

@fact id=fct-5akis2u7 expect_rev=2
old: ("Every OS-specific branch compiles and is tested on its CI runner")
new: ("The product builds and is tested on Linux (CI on `ubuntu-latest`)")

@fact id=fct-zxaumenb expect_rev=2
old: as `<bin dir>/viola-fake-agent[.exe] statusline-echo
new: as `<bin dir>/viola-fake-agent statusline-echo
old: beside the session's `bin/claude[.exe]` copy
new: beside the session's `bin/claude` copy
old: Copy `<bin dir>/viola-fake-agent[.exe]` (see step 5) to `target/agent-run/<session>/bin/claude[.exe]`,
new: Copy `<bin dir>/viola-fake-agent` (see step 5) to `target/agent-run/<session>/bin/claude`,
old: -- <bin dir>/viola-fake-agent[.exe] --cli-version
new: -- <bin dir>/viola-fake-agent --cli-version

@fact after=fct-smfiay44 as=s1
- **Required span attributes:**
  - `run.start`: `instance`;
  - `run.collision_check`: `outcome` (`free|already-live|squatted-name`) — a `live` or `stale` holder is `already-live`; a `gone` one is `free` (taken over, its `events.ndjson` appended to); over a replayed snapshot, which carries no pid or start time, the heartbeat alone decides (a beat no older than `STALE_AFTER` is `already-live`, an older one `free`), and an absent snapshot or a failed read is `free`;
  - `run.pin_copy`: `outcome` (`ok|pinned-hash-mismatch`);
  - `run.version_gate`: `cli_version`, `cli_verified`;
  - `channel.bind`: `endpoint_kind` (`unix-socket`);
  - `state.snapshot_write`: `v`;
  - `pty.spawn`: `pty_backend` (`openpty`, from `viola_pty::pty_backend()`), `env_stripped_count`.
  - Never the endpoint name, `pinned_bin` path or env values.

@fact after=fct-jzj7djty as=s2
- **Files (`~/.viola/`):** Plaintext by design (Established Decisions: Database / State Store). Protected by OS permissions only:
  - 0700 directories and 0600 files, except the pinned `bin/<version>-<hash>/viola`, which is 0700.
  - The strict-modes check runs at startup (Authentication & Authorization, `~/.viola/` access control).
  - **Code-bearing artefacts:**
    - `viola run` always rewrites `plugin/<version>-<hash>/` files with the shared tempfile `persist` helper (`replace_private_shared`: a replace that fails counts as done only when the file already holds exactly the rendered bytes, which a concurrent start of the same version writes) rather than trusting an existing copy.
    - `instances/<name>/settings.json` (the per-session override whose statusline command Claude Code executes) is `{"statusLine":{"type":"command","command":"<pinned path> hook statusline"}}`. `viola run` rewrites it whole at every start, a revived start included, with the shared tempfile `persist` helper (`viola_state::fs::replace_private`, 0600), after the first snapshot and before the spawn, and never reuses an existing copy. Its command uses the absolute pinned `bin/<version>-<hash>/viola` path (§ Universal). It is written only when every character of the pinned path is an ASCII letter, a digit or one of `_ - . / :` (`viola_agent_claude::statusline::override_document`), since a shell runs the string unquoted; for any other path no override is written. The child gets `--settings <instance dir>/settings.json` after `--plugin-dir <dir>` and before the caller's arguments, only when an override was written. The flag is the agent crate's constant (`statusline::SETTINGS_FLAG`), which `viola verify`'s Runs C and D read too (moved at chunk 2026-10-13-interrupted-verify-cleanup). **Ratified:** a code-bearing file written at every start and handed to the CLI is a boundary widening the founder ratified (live, 2026-10-10, relayed verbatim by the operator).
    - `<hash>` is a truncated SHA-256 of the exe bytes (the first 16 hex characters of the digest). Before reusing an existing `bin/<version>-<hash>/viola`, `viola run` re-hashes the file with SHA-256 and compares the truncated result with `<hash>`. On a mismatch it refuses to start (exit 1). The SHA-256 implementation is `sha2 =0.11.0` with `default-features = false` (Decisions Log `2026-09-25`): pure Rust, no `cc` on the one `[graph]` triple (`x86_64-unknown-linux-gnu`), passing the cargo-deny C-build ban; `tests/contract_content_hash.rs` pins it against published FIPS 180-2 vectors and the 16-hex truncation. A non-cryptographic fallback is banned (§ Data Protection). Because the hash is cryptographic, this catches substitution as well as accidental corruption. Tampering by the same OS user (who could also rewrite the folder name) stays outside the v1 boundary.
    - `viola run`'s start pins the bytes it has already checked (`viola_state::pin::pin_bytes`, under the key `pin::pin_key`: `<VERSION>-<16 hex>`); `pin_exe`, which `viola verify` calls, delegates to `pin_bytes`. An absent copy is placed, never replaced, a copy another start placed first is re-hashed before reuse, and one tree's release builds can key two folders (each in the groups below).
    - `ledger/stamps.json` is written only by `viola verify`, under its `.lock`, with 0600.
    - `marketplace/` is written only by `viola plugin install`: two manifests and `viola/.mcp.json`, whose `mcpServers.viola` `command` is the canonical exe that ran the install and whose `args` are `["--home", <the canonical home>, "mcp"]`, built by `serde_json`, no hooks in any document; each file rewritten whole at 0600 on every run (`replace_private`), each dir 0700, any other entry left as found. Claude Code runs that command for MCP in a session that installed the plugin. viola never reads the tree back nor re-hashes the command, spawns no `claude` and writes nothing under `~/.claude`: the user runs the two printed `claude plugin` commands.
  - **Probe captures:** `viola verify` writes its probe plugin and the hook-payload captures under `ledger/probes/<pid>/` (0700). The captures, `captures/<Event>.<k>.json` claimed exclusively by the hidden `hook --capture` arm, are transient, content-bearing 0600 files, and so are Run B's status line captures under `typed/statusline/` (0700; `StatusLine.<k>.json` and its `StatusLine.<k>.shell` verdict, claimed by the hidden statusline capture arm) beside the 0600 probe settings `typed/statusline.json`; the `questions/` (Run C) and `plan/` (Run D) roots each hold their own `plugin/`, `captures/` and an `answers/` dir of compiled `ProbeAnswer` ids; the `resume/` root (Run E) holds the same capture plugin as `typed/` and its own `captures/`, read by the `resume-session` check and `ledger::resume_variants`; a drop guard removes the whole probe dir when `verify` passes, fails or is stopped by Ctrl-C or TERM (only this run's `<pid>` dir), and only a scrubbed `--record` copy leaves it; SIGKILL, a crash or a failed handler install leave it to be removed by hand.
  - **Interactive probe dirs (outside the viola home):** Run A's fresh `viola-verify-*` dir under the OS temp root, and in the user's current folder Run B's `<cwd>/.viola-verify-<pid>/`, Run C's `<cwd>/.viola-verify-<pid>-dialogs/`, Run D's `<cwd>/.viola-verify-<pid>-plan/`, which holds a 0700 `plans/` (its `plansDirectory`), and Run E's `<cwd>/.viola-verify-<pid>-resume/`. All are 0700 and removed when verify passes, fails, or is stopped by Ctrl-C or TERM (a drop guard each; the repo's `.gitignore` carries `/.viola-verify-*/` for a leftover). The stop is `SIGINT` / `SIGTERM` (`cmd::verify::stop`, installed by `viola verify` alone; its handler only sets one flag every wait reads). A stopped run kills and reaps its child with no key, and its guards remove only what that run created, never by a pattern or a listing; it adds no owner record and no remover of an earlier run's or another process's dirs. SIGKILL, a crash or a failed handler install (ignored, fail-open) keep the OS default and leave the dirs to be removed by hand: the founder's narrow entry (relayed by the overseer, 2026-10-10), and the event set the operator's P4 answer (chunk 2026-10-13-interrupted-verify-cleanup). Run C's one `allow` runs `touch viola-probe-permission` inside its own dir; Run D's plan file lands inside its own dir, never under `~/.claude/plans` (as measured on 2.1.288 and 2.1.287: `~/.claude/plans/` 17 → 17 files across the re-run). These are the founder's live rulings on the dirs (2026-10-05 08:25Z) and on M7 = A and STOP 7 (2026-10-05), relayed by the overseer.
