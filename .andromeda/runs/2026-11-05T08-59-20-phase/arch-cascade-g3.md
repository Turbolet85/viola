@carried n=1 label=architecture/established-decisions fact=fct-qoiufq4r
why: The hook's socket-dir and euid refusals must keep this fail-open form on Linux after their Unix gates are made unconditional.
quote: `viola hook` meeting a refused socket directory (`NotSent::SocketDir`, `Asked::SocketDir`) exits 0 with empty streams and no frame and writes one `hook-decision` at WARN with `detail` `socket-dir-refused`, on the dialog path after `hook-invoked{corr:null}` with `decision_emitted:false`; a refused euid is `NotSent::ServerUnverified` in the same fail-open form, reading `server-verify-failed`.
@carried n=1 run=2 label=architecture/established-decisions fact=fct-4qhayrrs
quote: Beside the absent-`VIOLA_NAME` exit, the event and dialog paths run the instance check (`viola_state::strict::check_instance` on the canonical instance directory) before the diagnostics level is read and obs is initialised: a refused instance, or none (a `VIOLA_DIR` naming no existing directory), exits 0 at once with no `config.json` read, no role file, no stdin read, no line, no frame and no SessionEnd append.
@carried n=1 run=3 label=architecture fact=fct-4nt6qtyp
quote: **[Hook Contract] Hooks fail open.** Exit 0 with a JSON body is a decision, with no body no decision; exit 2 is never produced, and `hook` never writes to stderr.
@carried n=2 label=architecture/established-decisions fact=fct-2y2sv475
why: The explicit verbs' socket-dir, peer-euid and server-identity checks must keep their exit 21 and causes once their Unix gates are made unconditional.
quote: Then come the liveness-only pre-check (`live_endpoint`), then through `cmd::client::connect` the socket-directory check (`check_endpoint_dir`, a refusal exit 21, `socket-dir-refused`), the connect, the peer euid check and the server identity check (either refusal exit 21, `server-verify-failed`), and one request.
@carried n=3 label=architecture/established-decisions fact=fct-ikuc6e4y
why: The Unix-only statusline wrap must lose its gate with no Linux behaviour moved, and its Windows half, owed to the retired entry, is an expected amendment the plan names.
quote: The statusline is wrapped only for sessions started by `viola run`, through a per-session settings override that then runs the user's own statusline command unchanged. As landed (chunk 2026-10-10-statusline-pass-through) the wrap is Unix only: on Windows no override is written and no command is run, and that half is owed to the working-route entry "Windows-only live measurements" (the founder's word on the cut, live, 2026-10-10, relayed verbatim by the operator).
@carried n=4 label=architecture/established-decisions fact=fct-qwpklckh
why: The relied set and verify's probe --settings arm are gated on the wrapped OS, so with Windows gone a Linux host must still rely on all twenty-four rows and pass the probe --settings.
quote: The relied set is `LedgerRow::relied()`, `ALL` filtered in its order by `relied_on(row, statusline::WRAPPED_HERE)`: the three statusline rows are relied only where the status line is wrapped (Unix), so a Unix host relies on all twenty-four rows and a Windows host on twenty-one, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`, the last three relied on every OS.
@carried n=4 run=2 label=architecture/established-decisions fact=fct-qwpklckh
quote: On a wrapped OS (Unix), and only for shell-plain paths, it also gets one `--settings <probe dir>/typed/statusline.json` after its `--plugin-dir` pair (`typed::runs::trusted_args`; the document `statusline::probe_document` builds, written 0600; Windows passes none)
@carried n=5 label=architecture/established-decisions fact=fct-jzpbbnum
why: Verify's closed refusal codes must come out unchanged on Linux, and batch-script-child among them is the .cmd/.bat refusal the plan decides to keep or drop as a fork.
quote: `viola verify` writes `Ended::Refused`'s code on the `process-exit` it logs under a parsed `VIOLA_NAME`: `batch-script-child`, `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed`, `unclean-fixture`, beside `interrupted`.
@carried n=5 run=2 label=architecture/established-decisions fact=fct-5gkabtaw
quote: `AgentError` holds `resolve_program`'s two refusals, `BatchScriptChild` and `ProgramNotFound`, each with a fixed message: `resolve_program` returns `Result<PathBuf, AgentError>`, in the crate and in the root wrapper `src/run/mod.rs`.
@carried n=5 run=3 label=architecture fact=fct-bvzxxcy7
quote: viola resolves the child program to an absolute path itself (on Windows a PATH search over PATHEXT only, never the bare name) and spawns that. A `.cmd`/`.bat` child is refused (exit 1, `batch-script-child`), except the `claude` npm shim: `claude.cmd`/`claude.bat` resolves to `<dir>/node_modules/@anthropic-ai/claude-code/bin/claude.exe` when that file exists, and the shim is never read.
@carried n=6 label=architecture/established-decisions fact=fct-44n3w6o5
why: The Windows ConPTY sideload step and its `run.conpty_sideload` span must leave the start order, and the Unix-only settings override must lose its gate, every other step keeping its place.
quote: As landed by 2026-09-28-capability-ledger-and-viola-verify every step has its place (the sideload step added by 2026-09-29-sideloaded-conpty): program resolution → collision → pinned copy and plugin → ConPTY sideload (`run.conpty_sideload`, Windows: `pin_companions` + the absolute-path pre-load; `outcome` `loaded` · `hash-mismatch` · `strict-modes-failed` · `unreadable` · `load-failed` · `not-built`, never a refusal; its held handles live until `spawn_child` returns) → strip plan → version gate
@carried n=6 run=2 label=architecture/established-decisions fact=fct-44n3w6o5
quote: Between the first snapshot and the spawn `run` also rewrites the per-session settings override (Unix only; §Occupied Resources, Claude Code integration names).
@carried n=7 label=architecture/established-decisions fact=fct-uzcv7hop
why: The peer-euid admission and the pid and start-time server check must stay on Linux unchanged while their Windows arms and macOS carve-out leave.
quote: then connects and reads `peer_admitted(client.server_euid())`, then, on Linux and Windows, compares the connected peer's pid and its OS start time with them before its first frame (`verify_server`; §Standard Contracts, Session liveness); the seams are `src/cmd/client.rs` `connect` with its 8 callers, `src/cmd/hook.rs` `connect` and `crates/viola-mcp/src/tools.rs` `frame`.
@carried n=7 run=2 label=architecture fact=fct-w2igrcse
quote: on Windows the client connects only through the SQOS Identification open, never interprocess's default connect.
@carried n=8 label=architecture/established-decisions fact=fct-mefdhndu
why: socket_dir_ready must keep its order and its one exit-1 code once its Unix gate is made unconditional.
quote: Before any snapshot, child or `Server::bind`, `viola run` and `viola revive`'s start run `socket_dir_ready`: `check_endpoint_dir`, create the parent only when absent, check again, so a race's loser reads the same; a failed create and a refused check read one code, `socket-dir-refused`, exit 1, and a `Server::bind` failure does not read it.
@carried n=9 label=architecture/established-decisions fact=fct-xqkfyxga
why: The PtyError exception's rationale names windows-sys, which leaves viola-pty's manifest with the Windows target dependencies, while the hand-written exception itself stays.
quote: `viola-pty`'s `PtyError` has a hand-written `Display`/`Error` with fixed messages, because the seam's normal dependencies are exactly portable-pty, tracing, windows-sys and libc; portable-pty's anyhow error is kept only as a boxed source.
@carried n=10 label=architecture/established-decisions fact=fct-gctodeyz
why: The derive census must be re-measured by its grep after the Windows-only files leave, and its count and list must match what stands.
quote: The product sources hold 9 `thiserror::Error` derives, the census of [Error Handling]: `AgentError`, `ChannelError`, `StateError`, `PinError`, `StampsError`, `StampsReadError`, `Refused`, `McpError`, `ServeError` (by `command grep -rn "thiserror::Error" src crates --include=*.rs`).
@carried n=10 run=2 label=architecture fact=fct-sha7o35z
quote: One ratified exception, `viola-pty`'s hand-written `PtyError`, and two open divergences, `viola-mcp`'s `ServeError` and `viola-state`'s `StampsError`, with `PinError`, `StampsReadError` and `Refused` the three the census adds, the rule standing;
@carried n=11 label=architecture/established-decisions fact=fct-yjcz4wzl
why: The crates' manifests lose their Windows target dependencies with the tokio confinement and its two enforcers unchanged and still run on the Linux CI.
quote: Among the product crates only `viola-mcp` enables `tokio` (`rt`); `viola-channel` lists it optional behind its `tokio` feature (`["dep:tokio", "interprocess/tokio"]`, the `TokioClient`), which `viola-mcp` enables and, in a package-scoped test build, the channel's own test-only `fake-agent = ["tokio"]` (chunk 2026-10-17-mcp-server-for-drivers); the test-only `viola-e2e` takes `rt` + `process`. No other product member's normal graph reaches tokio (the sole-root loop and `scripts/channel-tokio-free.sh`).
@carried n=11 run=2 label=architecture fact=fct-l5jpezvw
quote: CI enforces it with a sole-root `cargo deny` `tokio` ban and a `cargo check` of the sync crates on all three OSes.
@carried n=12 label=architecture/established-decisions fact=fct-23yj6ah3
why: The embedded ConPTY companions, src/conpty.rs, vendor/conpty and pin_companions are the Windows-only code and files the chunk removes, the pinned exe's rules staying.
quote: On Windows x64 the binary also embeds Microsoft's ConPTY companions (`src/conpty.rs`: `include_bytes!` of `vendor/conpty/<version>/x64/{OpenConsole.exe,conpty.dll}`; the package version, the nupkg SHA-256 and the two file SHA-256s have that file's text as their one home) and writes them write-if-absent to `bin/<version>-<hash>/conpty/` (`viola_state::pin::pin_companions`; never overwritten). At every start each companion is opened with `FILE_SHARE_READ` only and re-hashed in full against its pin, and the handles are held until the spawn. Unlike the exe, a mismatched companion never refuses the start: it is left as found, never loaded, and the child runs on the inbox ConPTY ([PTY]).
@carried n=12 run=2 label=architecture/established-decisions fact=fct-5neshki5
quote: On Windows x64, before the held open, `pin_companions` runs the owner/DACL check (`viola_state::strict::check_present`) on `bin/<version>-<hash>/conpty/` and on each companion that exists: a refusal is `PinError::StrictModes`, writes nothing, and the child runs on the inbox ConPTY with `sideload_fallback` `strict-modes-failed`, the exit code unchanged ([PTY]).
@carried n=12 run=3 label=architecture fact=fct-omwrdnlc
quote: Child exit is detected on the process handle, never on EOF; on Windows x64 the ConPTY is sideloaded, failing open to the inbox one, and every `viola` process restricts its DLL search to System32; viola never holds, queues or re-orders a human key behind a resize and never answers DA1 itself. Rationale: 0.8.1 is spike-proven; 0.9.0 has a Windows garbage-read bug.
@carried n=12 run=4 label=architecture fact=fct-bq65okf4
quote: OPEN — whether the real `claude` loses a key typed right after a resize, and the sideloaded ConPTY's DA1 start stall with no terminal on viola's stdio · owner: "Windows-only live measurements"
@carried n=13 label=architecture/established-decisions fact=fct-4jk25cex
why: Its Windows leg names the retired owner, an expected amendment the plan names, and verify's Windows status line arm leaves with the Windows code.
quote: It is no capability-ledger row (no `send` relies on it, the ruled limit); its Windows leg is owed to "Windows-only live measurements".
@carried n=13 run=2 label=architecture/established-decisions fact=fct-4jk25cex
quote: Runs C and D, and Run B on Windows, the user's own status line (on Unix Run B's status line is the probe capture its one probe `--settings` names, [CLI Version Compatibility])
@carried n=13 run=3 label=architecture fact=fct-hcu7z4ke
quote: It is still built and CI-tested on Windows, macOS and Linux from the first commit, and the on-disk state is parsed defensively.
@carried n=13 run=4 label=architecture fact=fct-igbvtfv6
quote: **Cross-platform from the first commit, Windows first.** Windows is the first target.
@carried n=13 run=5 label=architecture fact=fct-4nt6qtyp
quote: its Windows leg, owner: "Windows-only live measurements")
@carried n=14 label=architecture/established-decisions fact=fct-onw4pywk
why: The Git Bash rewritten-path warning and revive's batch-script-child code name Windows without a cfg gate, so the plan lists each against the removal and keeps Linux's closed words unchanged.
quote: viola warns when an argument carries a Git Bash rewritten-path prefix. Rationale: Git Bash rewrites `/skill` arguments into Windows paths (brief §6), and agents need machine-parseable refusals.
@carried n=14 run=2 label=architecture/established-decisions fact=fct-onw4pywk
quote: (`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`, a `claude` not on `PATH` or an anyhow error of the start being `internal-error`)
@binding n=1
**established-decisions socket-dir and peer checks ↔ occupied-resources IPC endpoints**: the hook, the explicit verbs and every start keep `socket-dir-refused` and `server-verify-failed` with their exit codes (architecture/established-decisions fct-qoiufq4r) (architecture/established-decisions fct-2y2sv475) (architecture/established-decisions fct-mefdhndu) because the Linux socket rules they enforce stay as written while the Windows named pipe leaves (architecture fct-wular7qm) (architecture fct-vjccfh2l).
@binding n=2
**established-decisions server identity ↔ standard-contracts instance snapshot**: the pid and start-time check must stay on Linux (architecture/established-decisions fct-uzcv7hop) while the snapshot contract's macOS recorded gap loses its code (architecture fct-ld365mzq).
@binding n=3
**established-decisions statusline wrap ↔ standard-contracts hook statusline and occupied-resources settings override**: the wrap, the relied set and verify's probe --settings become unconditional together (architecture/established-decisions fct-ikuc6e4y) (architecture/established-decisions fct-qwpklckh), matching the /bin/sh shell-out with its Windows exception removed (architecture fct-horlp4ay) and the override written only on Unix (architecture fct-klpb7auf).
@binding n=4
**established-decisions ConPTY companions ↔ occupied-resources filesystem and repository rows**: removing the companions and their start step (architecture/established-decisions fct-23yj6ah3) (architecture/established-decisions fct-44n3w6o5) removes their rows too: `bin/<version>-<hash>/conpty/` (architecture fct-ekftm72e), `vendor/conpty/` and `scripts/conpty-vendor.sh` (architecture fct-rryrskwd), `target/conpty-seed/` (architecture fct-qe46hlhe) and the `seed_conpty` carve-out (architecture fct-swegdoif).
@binding n=5
**established-decisions Error Handling census ↔ conventions Rust error types**: the re-measured derive count (architecture/established-decisions fct-gctodeyz) and PtyError's dependency list (architecture/established-decisions fct-xqkfyxga) must agree with the conventions' nine-derive line (architecture fct-nhsg4k7g).
@binding n=6
**established-decisions Module Boundaries ↔ cross-cutting Tokio containment and the CI KEYSTONE**: the tokio confinement (architecture/established-decisions fct-yjcz4wzl) stays enforced by the sole-root deny ban and the sync-crate check on the Linux CI once the three-OS matrix shrinks (architecture fct-l5jpezvw) (architecture fct-me2myqzf).
@binding n=7
**established-decisions OS-target text ↔ the wrap's master amendments**: every decision line naming Windows or the retired owner (architecture/established-decisions fct-4jk25cex) (architecture/established-decisions fct-ikuc6e4y) is an expected amendment the plan names, beside Design Philosophy's Windows-first truth, the Personal v1 decision, Cross-platform discipline and the Scale path (architecture fct-igbvtfv6) (architecture fct-hcu7z4ke) (architecture fct-fa2klbw5) (architecture fct-qndsom6a).
@binding n=8
**established-decisions batch-script-child and Git Bash warning ↔ conventions exit codes**: whatever the plan decides for the .cmd/.bat refusal and the Git Bash warning (architecture/established-decisions fct-jzpbbnum) (architecture/established-decisions fct-onw4pywk), the typed exit-code set and viola hook's always-0 stay unchanged on Linux (architecture fct-jdhp6aqu) (architecture fct-nppkdemz).
