@carried n=1 label=obs-plan/obs-scope-summary fact=fct-rj2a6toh
why: The channel's lines must go to the same per-side role file on ubuntu-latest and macos-latest as on windows-2025, the same sink the chunk re-asserts on every OS.
quote: **Exporter:** The log file of whichever process is on each side (`run-<name>`, `hook-<name>`, `mcp`, or none for short-lived CLI verbs without an instance).
@carried n=1 run=2 label=obs-plan fact=fct-dao4nifd
quote: the same file sink and line schema on every OS, with identical fields and no OS-specific exporter.
@carried n=2 label=obs-plan/obs-scope-summary fact=fct-qe2js5tq
why: Each side of a Unix channel call must log its own `process`, a required field the one-schema re-assertion reads identical on every OS.
quote: Each side logs its own `process`. The peer version is available as `params.sender` (`CARGO_PKG_VERSION`).
@carried n=2 run=2 label=obs-plan fact=fct-23yn4mby
quote: the channel join key (D-25: `(conn, corr)`, each side logs its own `instance`)
@carried n=3 label=obs-plan/obs-scope-summary fact=fct-6xpygltd
why: The Unix channel endpoint must drop a refused-euid peer before any frame with no line and EOF to the client, and the Unix suite must read it so.
quote: A stream whose peer euid is refused is dropped on the accept thread (`accept_loop`, `peer_admitted`), before its worker thread and before any frame: the peer reads end-of-file, and the wrapper writes no line, there being no frame to key a line on and `channel-response`'s `detail` being closed, and nothing on the terminal.
@carried n=4 label=obs-plan/obs-scope-summary fact=fct-g4wb4nmr
why: `mcp` must write its home-level role file the same way on both Unix OSes, one of the role files whose location and modes the chunk re-asserts.
quote: Always the file sink `<home>/diagnostics/mcp.ndjson` (tests excerpt; D-09), the home resolved through `resolved_home` like every verb's and checked with its `diagnostics/` by strict-modes before the sink opens; stderr JSON only when that file cannot be opened (as landed, chunk 2026-10-17-mcp-server-for-drivers).
@carried n=5 label=obs-plan/obs-scope-summary fact=fct-uy5gif5o
why: `mcp` lines must carry `process` and `instance` with the same values on every OS, required fields of the re-asserted schema.
quote: as for the short-lived verbs, with `process="mcp"`. `instance` is the server's own `VIOLA_NAME` when set, which is also the `from` it adds.
@carried n=6 label=obs-plan/obs-scope-summary fact=fct-k56v7yhe
why: MCP's `instance-unreachable` must carry only these closed causes on every OS, including the Unix-reached `server-verify-failed`, `socket-dir-refused` and `strict-modes-failed` the macOS carve-out and endpoint cases read.
quote: `isError` results carry codes only: `refusal` / `detail`, `instance-unreachable` (its `detail` one closed cause: `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`, `socket-dir-refused`; the CLI's document less `v`, its text `refused: instance-unreachable`)
@carried n=7 label=obs-plan/obs-scope-summary fact=fct-foy4up7k
why: On both Unix OSes every role must install the panic hook first and make its home-tree check before any role file opens, so a refusal writes no line, which the Unix suite reads.
quote: Init order per process:
  1. As the first statement in `main`, install a replacement panic hook that writes one JSON line and nothing to stderr. The default hook's stderr output breaks the hook contract and the `run` terminal.
  2. Parse clap.
  3. Resolve the home (`--home`, then the grandparent of `VIOLA_DIR`, then the default) and the instance (`VIOLA_NAME` or an argument). Where the role checks the home before its first open, that strict-modes check runs here — every `viola hook` arm but the two `--capture` arms (`canonical` + `trusted`); `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` (`refuse_untrusted`); `verify` (`check_home_tree`); `run` and `viola revive`'s start (`checked_exe` + `check_home_tree`) — and a refusal ends the process with nothing under the home opened and no line.
  4. Open `diagnostics/<process file>` in append mode with 0600 permissions inside a 0700 directory.
  5. Install the subscriber with an explicit writer (never stdout).
  6. Emit `process-start`.
@carried n=8 label=obs-plan/obs-scope-summary fact=fct-he3oql74
why: Each line must stay one `write` on an append-mode handle on both Unix OSes, where concurrent hooks share one file under the Unix suite.
quote: The writer must emit each formatted line as a single `write` on an append-mode handle (arch Crash-safe disk writes). Concurrent `hook` processes share one file.
@carried n=9 label=obs-plan/obs-scope-summary fact=fct-ogkwmfvl
why: These required fields and the closed `event` enum are the one diagnostics schema the chunk must re-assert on every OS, with no OS-only word.
quote: **Required fields:**
  - For process logs: `timestamp` (RFC 3339 UTC with ms and `Z`), `level` (`DEBUG|INFO|WARN|ERROR`), `target`, and `fields.message` (or the flattened `message`).
  - `event`, a closed kebab-case enum. A new value needs a Decisions Log entry, like the §3 closed enums. Each value fixes what its `corr` holds:
    - `channel-request`, `channel-response`: `corr` = the JSON-RPC request `id`
    - `dialog-raised`, `dialog-answered`: `corr` = `dialog_id`
    - `hook-invoked`, `hook-decision`: `corr` = `dialog_id` for dialog hooks, otherwise null
    - `send-issued`, `send-confirmed`, `send-refused`: `corr` = the send `cursor`
    - `process-start`, `process-exit`, `panic`: `corr` = null
  - `process` (`run|hook|mcp|cli`) and `instance` (a `ViolaName` or null).
  - `corr` is copied unchanged as a JSON number or string. It is never renamed (for example to `correlation_id`).
@carried n=10 label=obs-plan/obs-scope-summary fact=fct-khwnpdov
why: Every diagnostics file must be 0600 inside 0700 on Unix, held at open by the per-OS strict-modes rule, the location and modes the chunk re-asserts on every OS.
quote: Every file is 0600 inside a 0700 directory (Windows: home DACL), and the tests secret-scan test enforces it. The product holds it at open too: `open_role_file` and `write_detail` run `strict::check_diagnostics` on the existing `diagnostics/` folder and on the role or detail file before anything is created or narrowed — a symlink refused, a folder held to the read rule (Unix: no group or other bit; Windows: no read grant to another SID), a file to the read rule on Unix and the write rule on Windows — and a refusal is a failed open (no role file opened; a detail line dropped).
@carried n=10 run=2 label=obs-plan fact=fct-k3pbv5ix
quote: Every role's codes-only process log goes to its home-level role file under `diagnostics/`, and content-bearing detail goes only to the instance's `diagnostics/detail-<role>.ndjson`.
@carried n=11 label=obs-plan/obs-scope-summary fact=fct-jqtgp6hh
why: The Unix wrapper's start must end at `pty.spawn` behind the same seam as ConPTY, logging the run's and the child's `process-start` and `process-exit` on failure as on Windows.
quote: **Must-trace spans:** `run.start` › `state.snapshot_recover` › `run.collision_check` › `run.pin_copy` › `run.version_gate` › `channel.bind` › `run.budget_eval` › `state.snapshot_write` › `state.heartbeat_start` › `pty.spawn`. Log events: `process-start` (run), `process-start` for the child, and `process-exit` on failure. The product event order to verify in `events.ndjson` is `wheel{cause:"start"}` → `budget-gate` → `session-start`.
@carried n=11 run=2 label=obs-plan/obs-scope-summary fact=fct-6yzr723e
quote: **Surfaces involved:** cli (`run`) + ipc-internal (channel bind) + persistent stores (snapshot, heartbeat, `events.ndjson`)
@carried n=11 run=3 label=obs-plan fact=fct-m24p7svx
quote: instrumentable at the seam; the ConPTY and openpty OS APIs behind it are boundary-only, and the PTY byte stream is user content, never logged.
@carried n=11 run=4 label=obs-plan fact=fct-ctz7espz
quote: the child's `process-exit` comes from the handle wait, never from reader EOF.
@carried n=12 label=obs-plan/obs-scope-summary fact=fct-h2wm2goy
why: run's Unix pre-child refusals must use these distinct exit-1 detail codes, `socket-dir-refused` before any snapshot, child or bind, and a home-tree refusal must write no line.
quote: Exit-1 causes use a distinct `detail` code: already-live (a live holder or a fresh replayed beat), stale-heartbeat (a stale holder), squatted-name, sha-256 mismatch, `.cmd`/`.bat` child, `socket-dir-refused` (a refused socket directory, on `run` and on a start made by `viola revive`: one `process-exit{subject:"self", exit_code:1}` before any snapshot, child or bind), and for a start made by `viola revive` also no-session, session-live and cwd-missing. A revive whose instance check refuses writes no line: the check precedes the log (chunk 2026-10-10-budget-governor). A `run` or a revived start whose home-tree check (`check_home_tree`, through `checked_exe`) refuses writes no line either, so that exit-1 cause has no `detail` code: the check precedes the wrapper log, and the output is the home pair (a home-level refusal) or the instance pair (an instance-level one), revive `--json` the `strict-modes-failed` document. No path or pid appears in the detail.
@carried n=12 run=2 label=obs-plan fact=fct-26ufyipy
quote: a socket directory `socket_dir_ready` refuses (Unix, exit 1, before any snapshot, child or `Server::bind`), a home tree another user could write or read: the home pair for a home-level refusal, the instance pair for an instance-level one, before any log opens) writes exactly two fixed stderr lines
@carried n=12 run=3 label=obs-plan fact=fct-y6qmqcu2
quote: each cause behind exit 21 and exit 1 gets a distinct `detail` code with no path or pid, except a strict-modes check refused before any log opens, which writes no line (its cause is the fixed stderr pair or the `--json` document).
@carried n=12 run=4 label=obs-plan fact=fct-wrd7y3z2
quote: a refused socket directory, read by the strict-modes read verdict, is no strict-modes failure: its line, where it has one, carries its own `detail`, `socket-dir-refused`.
@carried n=13 label=obs-plan/obs-scope-summary fact=fct-zrxuhqku
why: The Unix seam's write must carry `pty.paste_write` and the send path must log the same event order under openpty as under ConPTY.
quote: **Must-trace spans:** `send.client` (cli/mcp) › `channel.request` › `run.readiness_gate` (vt100) › `pty.paste_write` › `run.confirm_window` (not opened for an unconfirmable send). Log events: `channel-request`, then `send-issued`, then `hook-invoked` (UserPromptSubmit, or SessionStart for a `/clear` confirmed by its new session or a `/compact` confirmed by its same session (verified CLI); none awaited for an unconfirmable send), then `send-confirmed` or `send-refused`, then `channel-response`.
@carried n=13 run=2 label=obs-plan/obs-scope-summary fact=fct-7bjg2xvs
quote: **Surfaces involved:** cli or ipc-internal (mcp) → ipc-internal (channel) → cli (`run`, PTY paste) → cli (`hook` UserPromptSubmit; `hook` SessionStart for a `/clear` confirmed by its new session or a `/compact` confirmed by its same session (verified CLI); no hook for an unconfirmable send) → `events.ndjson`
@carried n=14 label=obs-plan/obs-scope-summary fact=fct-gpdjinuy
why: Send lines must carry these fields and closed `detail` words identically on every OS, and sent text must never appear under the Unix suite.
quote: **Required log fields:** `event`, `process`, `instance`, `corr` (the JSON-RPC `id` on channel lines; the send `cursor` on send lines), `from` (self-reported, not an identity), `confirmed`, `refusal`, `detail` (`input-not-ready` / `no-prompt-submitted` / `control-character` / `empty-text` / `turn-running`; on a `channel-response` answering `-32602`, `invalid-params` or `unsupported-version`), `duration_ms`. Sent text never appears.
@carried n=15 label=obs-plan/obs-scope-summary fact=fct-cetbw4yv
why: A Unix CLI `wait` must end at exit 21 with `endpoint-lost` during the call when its wrapper is killed, or `socket-dir-refused` during connect with no `channel-request`, the outcomes the kill-and-revive and endpoint cases read.
quote: `exit_code` (21 when the wrapper is killed mid-wait, `detail` `endpoint-lost`, `during:"call"`, or when a CLI `wait` meets a refused socket directory before any frame, `detail` `socket-dir-refused`, `during:"connect"`, no `channel-request`)
@carried n=15 run=2 label=obs-plan/obs-scope-summary fact=fct-leqqhyxz
quote: **Must-trace spans:** `wait.block` (client) › `channel.request` › `run.wait_dispatch`. Log events: `channel-request` and `channel-response` with the outcome.
@carried n=15 run=3 label=obs-plan/obs-scope-summary fact=fct-r2x644q6
quote: **Surfaces involved:** cli or ipc-internal (mcp) → ipc-internal (channel) → cli (`run`) + persistent store (`events.ndjson` tail)
@carried n=16 label=obs-plan/obs-scope-summary fact=fct-3dvlg5hs
why: The dialog path driven over the Unix socket must log these events with the same fields on every OS.
quote: Log events: `hook-invoked`, `dialog-raised`, `dialog-answered`, `hook-decision` (decision null on deadline expiry).
@carried n=16 run=2 label=obs-plan/obs-scope-summary fact=fct-ewxn4524
quote: **Surfaces involved:** cli (`hook` PreToolUse / PermissionRequest) → ipc-internal (`hook.dialog`) → cli (`run`) ← cli or ipc-internal (mcp) `answer`
@carried n=16 run=3 label=obs-plan fact=fct-qlfp3c3h
quote: a failed log write is swallowed and the hook still exits 0; without `VIOLA_NAME`, on an instance the instance check refuses, or on a `VIOLA_DIR` naming an instance directory that does not exist, the hook is a silent no-op (exit 0, no file opened, no line, no frame)
@binding n=1
**obs-scope-summary log-fields ↔ the body's Section 1 currency note**: the one-schema re-assertion on ubuntu-latest and macos-latest reads the required fields and closed `event` enum (obs-plan/obs-scope-summary fct-ogkwmfvl), but where Section 1's verbatim mirror differs, §3, §6 and §12 win — the diagnostics root, the enum extensions and the `(conn, corr)` channel join (obs-plan fct-23yn4mby) — so the re-asserted schema is `schemas/diag-line.v1.json` with the §6 catalog, not the mirror's text.
@binding n=2
**obs-scope-summary paths (run start) ↔ scope-entities (PTY seam)**: the Unix wrapper's start must reach `pty.spawn` and log the child's `process-start` (obs-plan/obs-scope-summary fct-jqtgp6hh) through the seam alone, openpty boundary-only and the byte stream never logged (obs-plan fct-m24p7svx), the child's `process-exit` from the handle wait, never reader EOF (obs-plan fct-ctz7espz).
@binding n=3
**obs-scope-summary paths (exit-1 and exit-21 details) ↔ scope-surfaces (channel and MCP)**: a refused socket directory reads `socket-dir-refused` on run's exit 1 (obs-plan/obs-scope-summary fct-h2wm2goy) and on a CLI `wait`'s exit 21 during connect (obs-plan/obs-scope-summary fct-cetbw4yv), MCP's `instance-unreachable` carries the same closed causes (obs-plan/obs-scope-summary fct-k56v7yhe), and a refused-euid peer writes no line at all (obs-plan/obs-scope-summary fct-6xpygltd): on macOS the server check compares euid alone, so no Unix test may read a pid or start-time comparison there.
@binding n=4
**obs-scope-summary strategy (init order) ↔ logs (file modes)**: a home-tree or strict-modes refusal must end the process before step 4 opens any role file, so it writes no line (obs-plan/obs-scope-summary fct-foy4up7k), while every file that does open is 0600 in 0700 under the Unix strict-modes read rule (obs-plan/obs-scope-summary fct-khwnpdov); the Unix suite must read the refusal by its fixed stderr pair (obs-plan fct-26ufyipy), never by a missing line counted as a schema or G2 failure.
