@carried n=1 label=security-plan/error-handling fact=fct-pfehx2bz
why: The Unix server verification the chunk exercises on Linux and macOS must refuse a foreign peer euid with `server-verify-failed` at all three clients, on macOS meaning an euid mismatch alone.
quote: A peer euid that `peer_admitted` refuses has no word of its own: it is `CallError::ServerUnverified` in `src/cmd/client.rs`, `NotSent::ServerUnverified` in `src/cmd/hook.rs` and `UnreachableCause::ServerVerify` in `crates/viola-mcp/src/tools.rs`, each reading `server-verify-failed`.
@carried n=2 label=security-plan/error-handling fact=fct-excpcefl
why: Bringing every `--json` document into line across OSes must keep the two clients as the only writers of the cause words, with no serde path on `UnreachableCause`.
quote: `viola_core::UnreachableCause` has no serde path; every cause word is written by the two clients: `client::unreachable_document` is the one writer of the CLI's exit-21 document (`refuse_untrusted` and `src/cmd/plugin.rs` through it; `src/cmd/list.rs`'s `refused_document` builds the board's `strict-modes-failed` as a `Value` for `board_document`), and `unreachable` in `crates/viola-mcp/src/result.rs` is MCP's.
@carried n=2 run=2 label=security-plan fact=fct-kulo4ws7
quote: `instance-unreachable` with `detail` its closed cause, `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused`
@carried n=3 label=security-plan/error-handling fact=fct-iaqygteq
why: The Unix-only socket-directory cause the Unix suite now reads must keep its exit 21, its `connect` phase, its hint and its single WARN `process-exit` on both Unix OSes.
quote: `viola_core::UnreachableCause::SocketDir`, its word `socket-dir-refused`, has no `RefusalReason`; on a CLI frame verb it is `CallError::SocketDirRefused` to `Unreachable::SocketDir`: exit 21, `during` `connect`, hint cause `socket-dir`, one `process-exit` at WARN and no `channel-request`.
@carried n=4 label=security-plan/error-handling fact=fct-2iushqjk
why: The socket-directory refusal exercised on Linux and macOS must name only the parsed name, never a Unix path, pid or uid.
quote: The socket-directory refusal names only the parsed `ViolaName`, and its lines hold no path, pid or uid.
@carried n=4 run=2 label=security-plan fact=fct-zhbrnvo2
quote: No anyhow context chains, no absolute paths, no upstream text (prompts, tool `input`, assistant output), no serde_path_to_error paths or values, and no internal type names.
@carried n=5 label=security-plan/error-handling fact=fct-6qpqztvz
why: The cross-OS `--json` parity reading must keep the exit-2 paths of `answer` and `send` a fixed stderr line with no document on every OS.
quote: Exception: the exit-2 paths of `viola answer` and `viola send` write the fixed `UNUSABLE_RESPONSE` / `UNUSABLE_TEXT` line to stderr whatever `--json` says, and no document (`src/cmd/answer.rs`, `src/cmd/send.rs`); `NOT_ERROR_BODIES` names them.
@carried n=6 label=security-plan/error-handling fact=fct-j463drna
why: The start's home strict-modes refusal must open no log and start no child on every OS, and its stated per-OS divergence over an unstattable home is the kind of case a test left OS-gated must name its reason for.
quote: `viola run` and `viola revive`'s start run the home tree's strict-modes check (`check_home_tree`, over the instance and the pinned copy's key) before the wrapper log opens: a refusal exits 1 with the home pair for a home-level refusal or the instance pair for an instance-level one, and opens no log, writes no line and starts no child (`revive --json` prints `strict-modes-failed`). `viola verify` runs the check on the home, `ledger/`, `bin/` and `plugin/` before its `cli` log, and a refusal there exits 1 with the home pair, no step line, no summary and no line; `update_stamps`'s `StampsError::StrictModes` exits 1 with the home pair, no step line and no summary, and its `process-exit`, under a parsed `VIOLA_NAME`, carries `strict-modes-failed`. On Unix a home under a regular file (unstattable) is such a refusal (the frame verbs: exit 21, the instance pair); on Windows that path stats `NotFound` and the `cli` catch-site line stands.
@carried n=7 label=security-plan/error-handling fact=fct-oolatrnj
why: The Unix wrapper's start must refuse a bad socket directory before any snapshot, child or bind, with exit 1 and one `socket-dir-refused` `process-exit`, on Linux and macOS alike.
quote: `viola run` and `viola revive`'s start, before any snapshot, child or `Server::bind`, refuse a socket directory through `refused_with` and `refuse_socket_dir`: exit 1 and one `process-exit` of `detail` `socket-dir-refused`, a step with no span.
@carried n=8 label=security-plan/error-handling fact=fct-vr46nktj
why: The internal-error `--json` document and the silence of `run`, `revive`'s start, `mcp` and `hook` at the catch site must read the same on every OS, keeping the hook contract.
quote: Where the `cli` catch site prints its internal-error line (`prints_internal_error`) and `json_asked` (`src/main.rs`) reads `--json` among the arguments before a `--`, `{"v":1,"error":"internal-error","detail":null}` prints on stdout in place of `error: internal error`, stderr empty; `run`, `revive`'s start, `mcp` and `hook` stay silent there.
@carried n=8 run=2 label=security-plan fact=fct-r24nqlwe
quote: `hook` never writes to stderr and never exits non-zero: a security check failing in `hook` still ends in exit 0 with no body, so the dialog renders for the human; its hidden capture arms print nothing on every failure, their stderr always empty, their exit always 0.
@carried n=9 label=security-plan/error-handling fact=fct-g4i32qdx
why: The pinned copy's strict-modes failure must keep its fixed message and map to the closed `strict-modes-failed` fallback outcome, a word the one diagnostics schema carries the same on every OS.
quote: `viola_state::stamps::StampsError` (`StrictModes` · `State`, the error `update_stamps` returns) and `viola_state::pin::PinError::StrictModes` carry fixed `Display` messages with no path or payload: `the capability stamps failed the strict-modes check`, a write failure its own fixed message, and `the pinned copy failed the strict-modes check`; `run` maps `PinError::StrictModes` to the `sideload_fallback` outcome `strict-modes-failed`.
@carried n=9 run=2 label=security-plan fact=fct-scytqusw
quote: The `Display` impls on `PtyError` (hand-written) and on `AgentError`, `ChannelError`, `StateError`, `PinError`, `StampsError`, `StampsReadError`, `Refused`, `McpError` and `ServeError` (`thiserror`) use fixed messages. Variant fields that hold paths or payloads are excluded from `Display`.
@carried n=10 label=security-plan/error-handling fact=fct-ttxl4mzv
why: The Unix socket-directory check exercised on both Unix OSes must refuse a non-directory, a symlink and an unstattable path with these fixed-message variants.
quote: `Refused::NotADirectory` carries the fixed `Display` message `a trusted state path is not a directory`; `check_socket_dir` returns it for a path that is no directory, `Refused::Symlink` for a symlink and `Refused::Unreadable` for an unstattable path.
@carried n=11 label=security-plan/error-handling fact=fct-zu3fn6nz
why: The Unix openpty wrapper must resolve its child through `resolve_program`, refusing only with its fixed-message `AgentError` variants.
quote: `resolve_program` returns `Result<PathBuf, AgentError>` in `viola_agent_claude` and in the root wrapper `src/run/mod.rs`, its refusals `AgentError::BatchScriptChild` and `AgentError::ProgramNotFound`, each a fixed message; `src/cmd/revive.rs`'s private `Refusal` is another type.
@carried n=12 label=security-plan/error-handling fact=fct-syp3b7yx
why: Any error writer the cross-OS schema alignment adds or moves must be named in this enumeration with its witness, and its two checks must pass on every OS.
quote: `tests/contract_error_bodies.rs`, carrying `andromeda:walks-tree`, enumerates every error body with its writer and witness (`CLI_DOCUMENTS`, `MCP_RESULTS`, `CHANNEL_FAULTS`, with `NOT_ERROR_BODIES` and `PLUMBING` the named exclusions); `product_error_writers_are_each_named` fails on an unnamed product error writer and `thiserror_messages_interpolate_nothing` on an interpolating `thiserror` message.
@carried n=13 label=security-plan/error-handling fact=fct-ticjysfg
why: This `fake-agent` cause table runs in the Unix suite and must hold the exit-21 and exit-1 causes, the Unix-only ones included, inside the unchanged typed exit set.
quote: `tests/cross_exit_causes.rs`, a root `cross_` binary with `required-features = ["fake-agent"]`, holds the exit-21 and exit-1 cause table, with `every_code_is_a_code_word`, `no_two_codes_share_a_hint`, its control `shared_hints_reads_a_planted_duplicate`, and `no_hint_names_release`.
@carried n=14 label=security-plan/error-handling fact=fct-msfbrx4e
why: The error-body check the Unix suite reads must catch Unix absolute paths under its `absolute-path` class and report place and class only.
quote: `tests/support/events.rs` holds the error-body check, `error_body_hit` and `assert_error_body_clean`, with the classes `absolute-path` · `caused-by` · `serde-path` · `planted`; it reports place and class, never the bytes it read.
@carried n=14 run=2 label=security-plan/error-handling fact=fct-4kx565zx
quote: Absence: no check runs after every E2E test; the three surface witnesses, the enumeration and the census of writers stand in its place, reading the check in `tests/support/events.rs`.
@carried n=15 label=security-plan/error-handling fact=fct-wyrvqwn5
why: A `diagnostics/` that fails the Unix strict-modes check, as the Unix suite can now exercise, must be a failed open, the role writer at `std::io::sink` and the detail line dropped.
quote: A `diagnostics/` folder, role file or detail file that fails `strict::check_diagnostics` (a symbolic link, or a mode or grant another user may use) is a failed open, checked before anything is narrowed: the role writer ends at `std::io::sink` and the detail line is dropped.
@binding n=1
**error-handling ↔ authentication-authorization**: the words `socket-dir-refused` and `server-verify-failed` (security-plan/error-handling fct-iaqygteq) (security-plan/error-handling fct-pfehx2bz) are the error side of the client-side server verification on Linux and macOS (security-plan fct-phspeqis): each refusal sends no frame, and a parity test reads macOS's `server-verify-failed` as an euid mismatch alone, never as a pid or start-time comparison (security-plan fct-6cauxi6n).
@binding n=2
**error-handling ↔ authentication-authorization (home strict-modes)**: the start's and diagnostics' strict-modes refusals (security-plan/error-handling fct-j463drna) (security-plan/error-handling fct-wyrvqwn5) are the error side of the owner-only home (security-plan fct-jya3vgt6): on Unix any group or other bit, or a symlinked `diagnostics/`, is refused before the first read, the start opening no log and starting no child, a failed diagnostics open ending at sink; where Unix and Windows diverge (an unstattable home) the OS-gated test names its reason and no Windows pin moves.
@binding n=3
**error-handling ↔ bootstrap-phases (NEVER-log floor)**: the socket-directory refusal's content and the error-body check's `absolute-path` class (security-plan/error-handling fct-2iushqjk) (security-plan/error-handling fct-msfbrx4e) hold the consolidated floor on every OS's output (security-plan fct-auxwpgtg): no absolute path, Unix-shaped included, in any CLI `--json` error, MCP `isError` result or channel `error.data`.
@binding n=4
**error-handling ↔ input-validation (child resolution)**: `resolve_program`'s fixed refusals (security-plan/error-handling fct-zu3fn6nz) carry the Unix wrapper's own resolution, which hands portable-pty an absolute path (security-plan fct-sgmmo3ki).
@binding n=5
**error-handling ↔ the body (CI matrix)**: the cause table and the error-body enumeration (security-plan/error-handling fct-ticjysfg) (security-plan/error-handling fct-syp3b7yx) must read green in the CI `test` job on `ubuntu-latest` and `macos-latest` beside `windows-2025` (security-plan fct-ju2d5wlo), the typed exit set unchanged.
