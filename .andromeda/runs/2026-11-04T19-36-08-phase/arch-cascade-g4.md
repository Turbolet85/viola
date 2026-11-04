@carried n=1 label=architecture/established-decisions fact=fct-44n3w6o5
why: The v1-40 stale-row witness and the Unix kill-and-revive read must judge liveness exactly as this rule states, heartbeat primary and pid plus start time deciding gone, on ubuntu-latest and macos-latest as on Windows.
quote: **[Session Liveness] Heartbeat file primary; pid + process start time decide `gone`, and `claude agents --json` only enriches it.** Rationale: the prototype's heartbeat proved reliable. A bare pid is unsafe because pids get reused, which is why start time is matched too: a snapshot whose pid is dead, or alive with another start time, is `gone` whatever the heartbeat says (Session liveness).
@carried n=1 run=2 label=architecture/established-decisions fact=fct-44n3w6o5
quote: Between the first snapshot and the spawn `run` also rewrites the per-session settings override (Unix only; §Occupied Resources, Claude Code integration names).
@carried n=1 run=3 label=architecture fact=fct-aos27ydg
quote: **[Session Liveness] Heartbeat file primary; pid + process start time decide `gone`, and `claude agents --json` only enriches it.** A `viola run` or `viola revive` whose `ViolaName` is `live` or `stale` refuses to start; a revive resumes the `gone` instance in place through the same start as `run`, its `events.ndjson` appended to, never truncated.
@carried n=2 label=architecture/established-decisions fact=fct-2iyz7ner
why: The collision check's printed, logged and span words are closed words of tests/run_cli.rs behaviours the Unix suite must read identically on every OS.
quote: `collision_check` prints and logs `stale-heartbeat` for a `stale` holder and `already-live` for a live holder or a fresh replayed beat. The `run.collision_check` span's `outcome` reads `already-live` for a live or a stale holder, its closed set saying a holder stands. Stale heartbeat is exit 1's cause alone: a frame verb against a stale instance reads `instance-dead`, `live_endpoint` giving none for a stale wrapper.
@carried n=2 run=2 label=architecture/established-decisions fact=fct-zgxijmsq
quote: `run`'s collision check takes its caller's one snapshot read (`replay::read_snapshot_or_replay`): over a replayed snapshot, which holds no `pid` or `started_at`, it decides from the heartbeat alone, a beat at most `STALE_AFTER` old refusing `already-live` and an older one free; a failed read or no snapshot is free.
@carried n=3 label=architecture/established-decisions fact=fct-uzcv7hop
why: The scope keeps macOS's recorded no-peer-pid gap, so the server check must compare pid and start time on Linux and Windows only and no Unix test may read macOS as a comparison.
quote: then connects and reads `peer_admitted(client.server_euid())`, then, on Linux and Windows, compares the connected peer's pid and its OS start time with them before its first frame (`verify_server`; §Standard Contracts, Session liveness)
@carried n=4 label=architecture/established-decisions fact=fct-wwpgfv5r
why: The start's home-tree strict-modes check is an OS-differing rule a parity test must read per OS, with the same refusal pair and the same strict-modes-failed document on Unix as on Windows.
quote: Before its wrapper log opens, every start (`run`'s and a revived one) checks the home tree: `checked_exe` reads the exe's bytes once and takes their pin key, then `viola_state::strict::check_home_tree` checks the home, the instance's set, `ledger/`, `bin/`, `plugin/`, `bin/<key>/`, the pinned copy, `plugin/<key>/` and its `hooks/` and `.claude-plugin/`, each that exists; a refusal is exit 1 with the home pair, or the instance pair for an instance-level refusal, or under `viola revive --json` the `strict-modes-failed` document in place of either pair, no log opened, no line and no child.
@carried n=5 label=architecture/established-decisions fact=fct-mefdhndu
why: The Unix socket directory check before any snapshot, child or bind is the Unix endpoint's start refusal the parity suite reads on both Unix runners.
quote: Before any snapshot, child or `Server::bind`, `viola run` and `viola revive`'s start run `socket_dir_ready`: `check_endpoint_dir`, create the parent only when absent, check again, so a race's loser reads the same; a failed create and a refused check read one code, `socket-dir-refused`, exit 1, and a `Server::bind` failure does not read it.
@carried n=6 label=architecture/established-decisions fact=fct-xqkfyxga
why: The openpty wrapper runs behind the same viola-pty seam, so any error it adds must be a fixed-message PtyError and its normal dependencies must stay exactly this set.
quote: one ratified exception: `viola-pty`'s `PtyError` has a hand-written `Display`/`Error` with fixed messages, because the seam's normal dependencies are exactly portable-pty, tracing, windows-sys and libc; portable-pty's anyhow error is kept only as a boxed source.
@carried n=6 run=2 label=architecture fact=fct-omwrdnlc
quote: **[PTY] portable-pty `=0.8.1` pinned behind viola's own `pty` seam (spawn · read · write · resize · wait · kill).** Child exit is detected on the process handle, never on EOF; on Windows x64 the ConPTY is sideloaded, failing open to the inbox one, and every `viola` process restricts its DLL search to System32; viola never holds, queues or re-orders a human key behind a resize and never answers DA1 itself.
@carried n=7 label=architecture/established-decisions fact=fct-yjcz4wzl
why: Any new code on the openpty or run path must keep viola-pty a sync crate whose normal graph never reaches tokio.
quote: No other product member's normal graph reaches tokio (the sole-root loop and `scripts/channel-tokio-free.sh`). Rationale: the compiler enforces that `viola-pty` cannot see `viola-agent-claude`.
@carried n=7 run=2 label=architecture fact=fct-l5jpezvw
quote: **[Concurrency / Backend Framework] Hybrid sync core + Tokio only in `mcp` (KEYSTONE).** CI enforces it with a sole-root `cargo deny` `tokio` ban and a `cargo check` of the sync crates on all three OSes.
@carried n=8 label=architecture/established-decisions fact=fct-onw4pywk
why: The one-schema rule requires every --json document, revive's closed refusal codes among them, to carry the same fields and words on every OS.
quote: every refusal of its start, a refused strict-modes check and a preflight refusal among them, prints `{"v":1,"error":"<code>","detail":null}` (`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`, a `claude` not on `PATH` or an anyhow error of the start being `internal-error`) in place of its `unable:` / `hint:` pair where it has one, exit 1
@binding n=1
**Session Liveness decision ↔ the body's Session liveness contract**: the v1-40 witness's stale wrapper must be one whose snapshot pid and start time still name a running process while its beat is older than 5 s, since a dead or reused pid reads `gone`, not `stale` (architecture/established-decisions fct-44n3w6o5) (architecture fct-arftgre4).
@binding n=2
**Session Liveness server check ↔ the instance snapshot contract**: the Unix suite must keep the pid and start-time comparison to Linux, with macOS comparing nothing and no test reading macOS as a comparison (architecture/established-decisions fct-uzcv7hop) (architecture fct-ld365mzq).
@binding n=3
**Start refusals ↔ the Unix IPC endpoint and the CLI --json contract**: `socket-dir-refused`, `strict-modes-failed`, `stale-heartbeat` and `already-live` must exit 1 and print the same closed codes on ubuntu-latest and macos-latest as on windows-2025, and the per-user 0700 socket directory is the check `socket_dir_ready` runs (architecture/established-decisions fct-mefdhndu) (architecture/established-decisions fct-onw4pywk) (architecture fct-wular7qm) (architecture fct-54jxzo44).
@binding n=4
**PTY error type ↔ Module Boundaries**: the openpty backend lives behind viola-pty, so its new code must use only portable-pty, tracing, windows-sys and libc as normal dependencies, report a fixed-message PtyError, and use std threads and blocking I/O under the sole-root tokio ban (architecture/established-decisions fct-xqkfyxga) (architecture/established-decisions fct-yjcz4wzl) (architecture fct-me2myqzf).
