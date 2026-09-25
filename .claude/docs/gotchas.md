# Gotchas

_Known issues, non-obvious constraints, and things that have broken before. Extracted from `.andromeda/architecture.md` and the specialist plans (measured facts and research findings) by `/andromeda-setup-project`._

Rendered by `/andromeda-setup-project` on the first run and kept current by wrap's cascade; a setup re-run preserves it. Learnings discovered during work sessions go to `session-learnings.md` (curated by `/andromeda-wrap-session`) — this file is for **documented architectural traps**, not runtime discoveries.

## Format

```
## {Short title}
**What breaks:** … **How to avoid:** … **Fix if broken:** … **References:** …
```

## Gotchas

## ConPTY never closes the output stream
**What breaks:** waiting for reader EOF to detect child exit hangs forever on Windows — ConPTY does not close the output stream when the child exits.
**How to avoid:** detect exit only by `wait()` on the process handle (a dedicated handle-wait thread); tests use `child.wait()`, never master EOF.
**Fix if broken:** move exit detection to the handle; log `process-exit{exit_source:"handle-wait"}`.
**References:** arch [PTY]; brief §4.1 Windows details; test-plan §11 E2E.

## portable-pty 0.9.0 garbage reads on Windows
**What breaks:** 0.9.0 has the Windows garbage-read bug (wezterm#6783).
**How to avoid:** keep `=0.8.1` behind the `pty` seam; the named swap is portable-pty-psmux 0.9.7 (must keep `bInheritHandles = 0`). The PTY chunk records the pin decision on current evidence (requirements v1-25).
**References:** arch [PTY]; security §Data Protection (handle inheritance).

## `claude` on PATH is an npm shim
**What breaks:** spawning `claude` by name on Windows hits the npm `.cmd` shim; portable-pty builds its own `CreateProcessW` line without std's BatBadBut escaping.
**How to avoid:** viola resolves the program itself (PATHEXT only, never the bare name — portable-pty's own search hits the extensionless sh shim first) and resolves the shim to its sibling `…\@anthropic-ai\claude-code\bin\claude.exe` without reading it; `viola run` refuses (exit 1) any other `.cmd` / `.bat` PTY child.
**References:** brief §4.1; security §Input Validation (child executable resolution).

## Append + exclusive lock fails on Windows
**What breaks:** locking the file you append to fails on Windows (rust-lang/rust#54118).
**How to avoid:** every guarded file has its own `<name>.lock` sibling; lock that, append to the data file with one `write` per line.
**References:** arch [Database / State Store].

## Git Bash rewrites leading-slash arguments
**What breaks:** `/andromeda-new-session` passed as an argument from Git Bash arrives as `C:/Program Files/Git/andromeda-new-session`.
**How to avoid:** prompt text only from stdin or `--file`; warn on a rewritten-path prefix; hooks and MCP use exec-form commands (no shell).
**References:** brief §6; arch [CLI Conventions]; design cli Platform notes.

## `DefaultHasher` is not stable across Rust releases
**What breaks:** two viola binaries built by different toolchains compute different endpoint names and cannot find each other.
**How to avoid:** the endpoint hash is hand-written FNV-1a 64 in `viola-channel`; the pinned-binary `<hash>` is truncated SHA-256 (never FNV or `DefaultHasher`).
**References:** arch Data model conventions; security Decisions Log amendment 8.

## macOS caps Unix socket paths at ~104 bytes
**What breaks:** long socket paths fail to bind on macOS.
**How to avoid:** short hashed names in a per-user 0700 dir; `run` records the resolved `endpoint` in the snapshot and every other process connects to the recorded path (a child may see a different `TMPDIR`).
**References:** arch Occupied Resources (IPC endpoints); security amendment 2.

## tracing-subscriber writes to stdout and `eprintln`s its own errors by default
**What breaks:** the default writer is stdout (breaks `--json`, hook decision bodies, MCP frames, the child's screen) and `log_internal_errors(true)` falls back to `eprintln` (breaks the hook's no-stderr contract).
**How to avoid:** always `.with_writer(..)` explicitly and `.log_internal_errors(false)`; also `.with_span_list(false)`, `.with_ansi(false)`, the `MillisUtc` timer (the default writes microseconds; `ChronoUtc` writes `+00:00`).
**References:** obs-plan §3 Logging stack (research findings 1–4).

## `panic = "abort"` silently defeats `catch_unwind`
**What breaks:** with an abort strategy a hook panic dies by SIGABRT instead of exiting 0, and the Claude session sees a hook failure.
**How to avoid:** every profile keeps `panic = "unwind"`; gate G3 greps Cargo.toml / cargo config / workflows for every spelling.
**References:** obs-plan §7, §9 G3.

## `jq -s` aborts on a torn line
**What breaks:** slurping ndjson with a torn last line exits 2 and a gate reads garbage as a pass/fail.
**How to avoid:** `awk 1 files… | jq -R -n '[inputs|fromjson?|…]'` (skip torn lines, count them).
**References:** obs-plan §5, §9 G2, D-30.

## `std::env::set_var` is `unsafe` in edition 2024
**What breaks:** tests that mutate the process environment race across threads and fail to compile safely.
**How to avoid:** pass env per child with `Command::env`; `.env_clear()` must re-add `LLVM_PROFILE_FILE` or that run drops out of coverage.
**References:** test-plan §7, §11 Integration.

## ExitPlanMode ignores PermissionRequest `allow`
**What breaks:** approving a plan through a PermissionRequest `allow` leaves the dialog rendered (measured twice).
**How to avoid:** approve a plan only through PreToolUse `permissionDecision: allow`; revise via PermissionRequest `deny` + a message saying what to change. Two identical revise messages in a row made the model stop re-presenting the plan.
**References:** brief §4.1 S7; arch ledger rows; requirements v1-15.

## CLI-native modals bypass every hook
**What breaks:** a paste typed while a CLI-native modal is up ("Teach auto mode…") is swallowed; the screen also lags the hooks.
**How to avoid:** the vt100 readiness gate (quiet period + input-box signature + no modal signature) before any send; every send confirmed after the fact; otherwise `not-delivered` / `input-not-ready`.
**References:** arch [Screen Model], [Delivery Confirmation].

## Local commands fire no UserPromptSubmit
**What breaks:** `/remote-control` (and other built-in local commands) never produce `prompt-submitted`, so a naive confirmation reports `not-delivered`.
**How to avoid:** the ledger lists local commands with their post-condition (`/clear` → SessionStart `clear` + new `session_id`) or "none" → `ok` with `confirmed:false, detail:"unconfirmable"`. Skills are not local commands.
**References:** arch [Delivery Confirmation], [CLI Version Compatibility].

## Long pastes arrive wrapped and escaped
**What breaks:** exact-match delivery confirmation fails on long pastes (wrapped in `<pasted_content id=…>` above ~981 bytes on 2.1.280) and on tag-like text (`<` becomes `<\`).
**How to avoid:** `viola-agent-claude` removes the wrapper, then reverses tag escaping, before emitting `prompt-submitted`.
**References:** arch Ledger rows; test-plan Path 2 step 3.

## The parent session's identity leaks into the child
**What breaks:** started from inside a Claude session, the child inherits `CLAUDECODE`, `CLAUDE_CODE_SESSION_ID`, the Remote Control bridge and the messaging socket/token (11 names measured on the Windows host).
**How to avoid:** the R8 rule: `viola-agent-claude::plan_strip` removes every inherited `CLAUDE*` name except persistent-environment names (Windows registry `Environment`, Unix `config.json` `claude_env_keep`), and always the 11-name `IDENTITY_FLOOR`; log names only (`env_stripped_count` / `env_stripped_known` / `env_kept`), never values.
**References:** brief §4.1 S6; arch Occupied Resources → Environment variables; security Decisions Log 2026-09-25; obs D-34.

## The docs lag the installed CLI
**What breaks:** designing against docs misses real flags (`stream-json` input, `claude attach`) and real behaviours.
**How to avoid:** design against the installed CLI, measured; every relied-on behaviour is a ledger row with a `viola verify` probe.
**References:** brief §4; arch [CLI Version Compatibility].

## Related

- For runtime-discovered learnings, see `.claude/docs/session-learnings.md` (curated by /andromeda-wrap-session)
- For path-specific rules (loaded when touching specific files), see `.claude/rules/*.md`
- For architectural decisions and their rationale, see `.andromeda/architecture.md` Established Decisions section
