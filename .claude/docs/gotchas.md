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
**How to avoid:** approve a plan only through PreToolUse `permissionDecision: allow` with `updatedInput` set to the tool's own input, unchanged — a bare `allow` left the plan dialog up on live 2.1.288 (the founder's STOP 7 ruling, 2026-10-05); revise via PermissionRequest `deny` + a message saying what to change. Two identical revise messages in a row made the model stop re-presenting the plan.
**References:** brief §4.1 S7; arch ledger rows (S7 is gated by the `plan-approve-revise` row since chunk 2026-10-05-dialog-rows-and-re-probe); requirements v1-15.

## CLI-native modals bypass every hook
**What breaks:** a paste typed while a CLI-native modal is up ("Teach auto mode…") is swallowed; the screen also lags the hooks.
**How to avoid:** the vt100 readiness gate (quiet period + input-box signature + no modal signature) before any send — the full gate on the compiled `SIGNATURES` only on a verified CLI version, partial otherwise (a poisoned model or a screen not quiet within 5 s refuses; no row is read); every send confirmed after the fact; otherwise `not-delivered` / `input-not-ready`.
**References:** arch [Screen Model], [Delivery Confirmation].

## Local commands fire no UserPromptSubmit
**What breaks:** `/remote-control` (and other built-in local commands) never produce `prompt-submitted`, so a naive confirmation reports `not-delivered`.
**How to avoid:** the ledger lists local commands with their post-condition (`/clear` → SessionStart `clear` + new `session_id`) or "none" → `ok` with `confirmed:false, detail:"unconfirmable"`. The list and the `/clear` row are compiled (`ledger::LOCAL_COMMANDS`, row `local-command-clear`), and `send` consumes them by exact text equality (no trim, no case folding, never a leading-slash test). A listed command with no post-condition, or any listed command on an unverified CLI version, is typed and answered `unconfirmable` at once, with no window. `/clear` on a verified version waits the window for a `session-start` with cause `clear` and a new session id, and no `prompt-submitted` claims it; a missed post-condition is `not-delivered` / `no-prompt-submitted`. The `/clear` confirmation is proven on the recorded 2.1.287 variants replayed by the fake agent; the live proof is owed to "First live test and self-drive". Skills are not local commands.
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

## A resize before the pump's first look was lost
**What breaks:** a host resize landing between the PTY's spawn sizing and the pump's first size read never reached the child, because the pump took a second `host_size()` read as its baseline (as measured at chunk 2026-09-26-local-linux-pre-push-gate: `spawn` 80×24 → resize +23.7 ms → pump baseline 100×30 at +990 ms; 6/6 red under a forced window).
**How to avoid:** `viola_pty::pump(…, spawned, host_size)` takes the size the PTY was spawned with as its baseline. The forced-window test reaches the window through the `fake-agent`-only `FAKE_AGENT_PUMP_DELAY_MS` seam rather than sampling the race.
**References:** test-plan §5 Module ↔ PTY, §12 `2026-09-27`; arch §Occupied Resources → Environment variables.

## A key written right after a ConPTY resize can fail to reach a test child (H2)
**What breaks:** on the Windows runner a key written into ConPTY right after `ResizePseudoConsole` returned was never read by the Rust test child, which was blocked in its console read (`ReadConsoleW` through Rust's stdin): measured 13 losses in 200 isolated iterations (6.5 %) of `spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code` under `cargo llvm-cov nextest` on `windows-2025-vs2026` 20260922.246.2 (ci#36527891850, job 109274838484). Every loss was class K: the child's size watcher saw the new size with no key, viola's writer had written and flushed the key, and ConPTY wrote no cursor-position request (`dsr-cpr 0`). The loss lies between the ConPTY input-pipe write and that test child's read; **where inside that span, and why, is not established** — an overseer read of microsoft/terminal's current source (relayed at the wrap, not verified here) found no input-buffer flush on the resize path and no reported resize input loss, so this is not recorded as a ConPTY defect. This host's conhost 10.0.26100.8875 read 20/20.
**The product window — open and unmeasured for `claude`:** viola forwards resizes and keys as they come and never holds, queues or reorders a human key behind a resize (a11y-plan §1), so nothing in viola closes this window. Whether the real `claude` (Node/libuv, reading with `ReadConsoleInputW`) loses a key typed right after a resize in `viola run` is **unmeasured**; the rate above is the Rust test child's. Owner: the working-route entry "First live test and self-drive" (the harness's `run --local-live` does not claim it). The founder's product question on H2 stays open.
**How to avoid (tests only):** a test that writes a key after a resize waits for the child to observe the new size first — the red test now waits for its watcher's `size 120x40` line, and read 200/200 under the same loop (ci#36529038462). That reshaped test does NOT cover the product window: it proves the key arrives once the resize has landed, not that a key racing a resize arrives, and it reads with the Rust child, never `claude`.
**Fix if broken:** read the kept reports in `<temp dir>/viola-pty-watch/` (`<test>.report`, `<test>.test.report`) by the chunk's localisation rule — class R (no `size` line), K (size seen, key flushed, key lost) or E (key before size) — before changing any wait; never raise `CHILD_WITHIN` or retry the test.
**With the sideloaded ConPTY** (chunk `2026-09-29-sideloaded-conpty`, one windows-2025 run, image `windows-2025-vs2026` 20260828.587, ci#36563868040, the measurement-only race): sideloaded `OpenConsole.exe` 1.24.260710001 lost 0 of 200, the inbox ConPTY 14 of 200 — no rate claimed, and the product window above stays unmeasured for `claude`.
**References:** arch [PTY]; test-plan §5 Module ↔ PTY, §10 Zero-flakiness budget; chunk `2026-09-29-h2-conpty-resize-probe` `evidence/h2-reproduction.md`; chunk `2026-09-29-sideloaded-conpty` `evidence/h2-with-without.md`; microsoft/terminal PR #19535 (post-resize CPR — not seen on either build).

## The sideloaded ConPTY holds the child's start for a DA1 answer
**What breaks:** on Windows x64 the sideloaded `OpenConsole.exe` writes `ESC[1t ESC[c ESC[?1004h ESC[?9001h` at spawn, and the child's start waits until the DA1 query `ESC[c` is answered. A real terminal answers at once; a `viola run` whose stdio is pipes (or any host that never answers) holds the child ~3 s (measured on the dev host: 3.54 s unanswered vs 0.54 s answered, 0.48 s on the inbox host). The answer travels through viola's pump, so the child's start now also waits for the pump.
**How to avoid:** a test that drives `viola run` on pipes answers DA1 the way a terminal would (`tests/support/piped.rs` `Piped` answers `ESC[?1;0c`); viola itself stays silent and never answers. A test timing a window around the child's start keys on the wrapper's `process-start{claude-child}` line, not the child's `start` receipt.
**Open:** the stall of a headless `viola run` is a product finding, owned by the route entry that first runs viola headless.
**References:** arch [PTY]; test-plan §5 Module ↔ PTY; chunk `2026-09-29-sideloaded-conpty` `evidence/da1-stall.md`.

## A stopped wrapper's pipe can still take a hook on Windows
**What breaks:** a `viola hook` fired within milliseconds of the wrapper's exit can still reach the exiting wrapper's own `\\.\pipe\viola-<h12>`: measured at ci#36532038635 (`test (windows-2025)`), where the test's hook connected and wrote its `hook.event` 25 ms after the wrapper's `process-exit` line, after the test had seen the wrapper's exit code. `viola hook` still exits 0 silently (fail-open holds), but the notification reads as delivered, so `SessionEnd`'s direct-append fallback is skipped in that window. **Why the pipe outlives the observed exit is not established** — Microsoft's documented `ExitProcess` order closes handles before the exit status is set, which argues against the obvious reading. A pipe-name collision with another home is ruled out: the endpoint name carries the home.
**How to avoid (tests):** "stopped" means the endpoint is gone — a client connect reads NotFound (the harness `endpoint_gone` rule) — never the exit code alone: the root fixture's `Wrapper::stop` / `stop_keep` wait for it (`wait_endpoint_gone`).
**References:** test-plan §10 Zero-flakiness budget; obs-plan §6 fail-open `detail` codes; chunk `2026-09-29-verify-stamped-test-homes-and-harness` research.md §Item 8.

## A dependency's source is not under `~/.cargo`
**What breaks:** on this host `CARGO_HOME` sits on another drive (the registry is not under `~/.cargo`), so a path to a crate's source built from `~/.cargo/registry/src/…` or a guessed `CARGO_HOME` reads nothing, and a search over it reads as "not found".
**How to avoid:** locate a crate's source through `cargo metadata --format-version 1`: the package's `manifest_path` is its `Cargo.toml`, and its parent is the source root at the exact version the lockfile resolved.
**References:** Epoch 2b evolve diagnosis §L4 (`.andromeda/runs/2026-10-01T09-05-15-evolve-diagnose/proposals.md`).

## Related

- For runtime-discovered learnings, see `.claude/docs/session-learnings.md` (curated by /andromeda-wrap-session)
- For path-specific rules (loaded when touching specific files), see `.claude/rules/*.md`
- For architectural decisions and their rationale, see `.andromeda/architecture.md` Established Decisions section
