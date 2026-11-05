@carried n=1 label=security-plan/input-validation fact=fct-p4cpzbge
why: The hook's instance check must keep running before any config, stdin or log use, and every failure must stay fail-open, when the strict-modes Unix arm loses its gate, as the scope keeps `hook`'s contract.
quote: The event and dialog paths then canonicalise the instance directory and run the instance check (`canonical`, then `trusted`, in `src/cmd/hook.rs`: `viola_state::strict::check_instance`) before the diagnostics level and obs init: a `VIOLA_DIR` naming no existing instance directory, or an instance the check refuses, exits 0 at once with no `config.json` read, no stdin read, no line, no frame and no SessionEnd append.
@carried n=1 run=2 label=security-plan fact=fct-u7v2p5mv
quote: every failure fails open (exit 0, no body); `budget.json` is written by `hook statusline` alone; the hidden capture arms read no `VIOLA_*`, open no channel and always exit 0.
@carried n=2 label=security-plan/input-validation fact=fct-l7hy7pzk
why: The frame verbs' client order must stay whole on Linux (socket-dir check, peer euid decision, server check, each before the frame) once the Windows and macOS arms of that order leave.
quote: A passed check is followed by the liveness pre-check `live_endpoint` (the endpoint, `pid` and `started_at`), `check_endpoint_dir` on the parent of that endpoint (a refusal is exit 21, `socket-dir-refused`, no frame), the connect, on Unix the peer euid decision `peer_admitted` (a refusal reads `server-verify-failed`) and then, on Linux and Windows, the server check before the frame (IPC client-side server verification row).
@carried n=2 run=2 label=security-plan/input-validation fact=fct-l7hy7pzk
quote: then `canonical_home` applies `std::fs::canonicalize` when the path exists (on Windows a `\\?\C:\…` result is written back as `C:\…`, only the verbatim-drive prefix stripped; `\\?\UNC\…`, `\\?\Volume{…}\…` and `\\.\…` stay as given), while a path that does not exist keeps `std::path::absolute`'s spelling and nothing is created.
@carried n=3 label=security-plan/input-validation fact=fct-nuclnamd
why: The Unix R8 pass-list read in `src/run/env.rs`, whose Windows registry sibling leaves, must keep its whole rejection and its identity-floor rule if its gate is lifted.
quote: `claude_env_keep` (the Unix R8 pass-list) is an array of at most 32 names, each matching `^CLAUDE[A-Z0-9_]*$`; a bad value is rejected WHOLE (never partly accepted) and reported as `parse-rejected`, and an identity-floor name in it has no effect (not an error).
@carried n=3 run=2 label=security-plan fact=fct-wsshf6pn
quote: an identity-floor name in `claude_env_keep` has no effect.
@carried n=4 label=security-plan/input-validation fact=fct-uooa7rml
why: The registry read leaves with its Windows arm, and the remaining Linux R8 rule must keep stripping every identity-floor name with no persistent-set source left.
quote: `viola run` reads value NAMES only, never data, under `HKCU\Environment` and `HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Environment`, into a bounded buffer; a missing key gives an empty set. The names only decide which inherited `CLAUDE*` names the R8 rule keeps, and an identity-floor name is stripped whatever the registry holds (operator ruling 2026-09-25, Decisions Log).
@carried n=4 run=2 label=security-plan fact=fct-cpef5o24
quote: `viola run` reads value names only, never data, into a bounded buffer; they only decide which inherited `CLAUDE*` names the R8 rule keeps, and an identity-floor name is stripped whatever the registry holds.
@carried n=5 label=security-plan/input-validation fact=fct-jd7jb3il
why: The pump-delay seam must stay compiled out of every release build and switch off nothing, as the task keeps both fake-agent seams and the Linux release job's feature refusal.
quote: It is compiled out of every release build, is not a `VIOLA_*` variable, configures nothing, disables no control and widens no redaction (operator ruling 2026-09-27, Decisions Log).
@carried n=5 run=2 label=security-plan fact=fct-ykdescly
quote: compiled out of every release build, it configures nothing, disables no control and widens no redaction.
@carried n=6 label=security-plan/input-validation fact=fct-qyyrikqm
why: The hook-panic seam must stay a closed value compiled out of release builds, since G2's one exemption kept on the Linux leg is its panic location.
quote: Named in product source only in its own file; compiled out of every release build; not a `VIOLA_*` variable; configures nothing, disables no control and widens no redaction (the founder's ratification 2026-09-28, Decisions Log).
@carried n=6 run=2 label=security-plan fact=fct-bbnvgu6s
quote: a closed value, panicking only on exactly `1` with a fixed ASCII payload, the hook still failing open; compiled out of every release build, it configures nothing, disables no control and widens no redaction.
@carried n=7 label=security-plan/input-validation fact=fct-lj2i7oks
why: The Windows VT enable arm of `src/human/style.rs` leaves, and the Linux colour decision must keep the facts only compared, never echoed, reaching no log, error or refusal.
quote: decided into the closed `Depth` (`Truecolor` · `Ansi256` · `Ansi16`) or none: none under `--json`, a passthrough role, `NO_COLOR` present (an empty value included), `TERM` exactly `dumb`, a non-terminal stdout, or a failed Windows VT enable (`GetConsoleMode`, then ` SetConsoleMode(mode | 0x0004) ` on stdout, the saved mode restored on drop; never viola-pty's `0x000c`). They decide only whether and how deep SGR bytes are written around rows already escaped; they reach no log line, error or refusal, are never set by viola, change no escaping or check, configure nothing, disable no control and widen no redaction.
@carried n=8 label=security-plan/input-validation fact=fct-wkbf5hxn
why: The strict-modes arm made unconditional must still run before every own-state read at every entry point, as the task requires it to refuse before the first read under the home.
quote: **Integrity:** read only after the home strict-modes check, at every entry point.
@carried n=8 run=2 label=security-plan fact=fct-brqp4paf
quote: read only after the home strict-modes check
@carried n=9 label=security-plan/input-validation fact=fct-g433mics
why: The feed-panic witness's OS-gated readings are re-cut by the removal, and on Linux it must keep reading the child's bytes and the absence of every panic word.
quote: **Witness:** after a caught feed panic, `chaos_feed_panic_refuses_send_and_keeps_passthrough` reads the outer terminal: on Unix the child's bytes (`e4 b8 ad`) reach it, and on every OS none of `panic`, `parse-rejected`, `vt100`, `unable` or `hint:` appears on it; the Windows byte reading is owed to "Windows-only live measurements".
@carried n=10 label=security-plan/input-validation fact=fct-q67nxnld
why: The Windows PATHEXT search and the `.cmd`/`.bat` refusal leave with their arms while `viola run` must keep resolving its child itself and handing portable-pty an absolute path.
quote: `viola run` resolves its child program itself and hands portable-pty an absolute path, because portable-pty 0.8.1's own Windows search tries the exact name first (it would hit the extensionless npm sh shim). On Windows the PATH search tries PATHEXT extensions only, never the bare name. A `claude.cmd`/`claude.bat` (case-insensitive) resolves to the fixed sibling `<dir>/node_modules/@anthropic-ai/claude-code/bin/claude.exe` when that file exists; the shim is never read or parsed.
@carried n=10 run=2 label=security-plan/input-validation fact=fct-q67nxnld
quote: Every other `.cmd`/`.bat`, including a `claude.cmd` without that sibling, is refused with the closed `AgentError::BatchScriptChild`: exit 1, no child spawned, `process-exit{subject:"self", exit_code:1, detail:"batch-script-child"}`, and the two fixed `unable:`/`hint:` stderr lines with no path or pid
@carried n=10 run=3 label=security-plan fact=fct-sgmmo3ki
quote: `viola run` resolves its child itself and hands portable-pty an absolute path; on Windows a `.cmd`/`.bat` child is refused, except a `claude` shim, resolved to its fixed `claude.exe` sibling when that exists
@carried n=11 label=security-plan/input-validation fact=fct-m6zdbeoz
why: The shell-out's Unix gate is lifted, and it must keep exactly its `/bin/sh -c` one-argument form, its order after the instance check and its bounds, staying the only shell-out.
quote: The recorded `statusline_command` is run only by `hook statusline`, as `/bin/sh -c <command>` with the command as one argument (`viola_agent_claude::statusline::shell_argv`); on Windows that function returns `None` for every input and no command runs. The arm's order: canonicalise the instance directory, the instance check, the log, stdin, the `budget.json` write, the snapshot read (`read_snapshot_or_replay`: a replayed snapshot carries no `statusline_command`, so nothing runs and nothing prints), the shell-out.
@carried n=11 run=2 label=security-plan/input-validation fact=fct-m6zdbeoz
quote: No log line and no span carries the command. Not controlled: what the command does; it runs as the user, as it does without viola. It remains the only shell-out.
@carried n=11 run=3 label=security-plan fact=fct-2fgozo5r
quote: the recorded command runs only from `hook statusline`, as `/bin/sh -c` with the command as one argument, never on Windows; its stdout is capped and printed byte for byte, never parsed, its wait bounded; no log line or span carries the command, and it remains the only shell-out.
@carried n=12 label=security-plan/input-validation fact=fct-bboseg7m
why: The DLL search restriction, the second statement of `main`, leaves with `sideload.rs` and `src/conpty.rs`, and no Linux role may keep calling into the removed sideload code.
quote: **Planting:** every `viola` process, in every role, runs `viola_pty::sideload::restrict_dll_search()` (`SetDefaultDllDirectories(LOAD_LIBRARY_SEARCH_SYSTEM32)`, process-wide, one-way) as the second statement of `main`
@carried n=12 run=2 label=security-plan/input-validation fact=fct-bboseg7m
quote: **Companions:** `run`'s `conpty_sideload` step (after the pinned copy and plugin, before the strip plan) writes the embedded `OpenConsole.exe` + `conpty.dll` write-if-absent into `bin/<version>-<hash>/conpty/`
@carried n=12 run=3 label=security-plan fact=fct-oqqkxxum
quote: every `viola` process restricts the DLL search to System32 as the second statement of `main`, so a bare-name load never resolves from the CWD or `PATH`
@carried n=13 label=security-plan/input-validation fact=fct-jy3jkgu2
why: Each reader the removal deletes takes its `BOUNDARIES` or `EXCEPTIONS` row with it, and the bounded-readers census must keep passing on Linux.
quote: **Enumerated and censused:** `tests/contract_bounded_readers.rs` (an E2E `contract_` binary) names each external boundary's bounded read and its witness in `BOUNDARIES`; its census `product_readers_are_each_bounded_or_named` requires every product reader call to be bounded in its statement or named in `EXCEPTIONS`, and a self-check reads a planted unbounded read as a hit.
@carried n=13 run=2 label=security-plan fact=fct-ydf4aeiw
quote: `MAX_FRAME` = 16 MiB, one const in `viola-core`, caps every external read
@carried n=14 label=security-plan/data-protection fact=fct-jzj7djty
why: The Unix-only settings override loses its gate and must keep its plain-character guard on the pinned path, while the companions leave and the Unix modes and the pinned re-hash stay.
quote: It is written only on Unix, and only when every character of the pinned path is an ASCII letter, a digit or one of `_ - . / :` (`viola_agent_claude::statusline::override_document`), since a shell runs the string unquoted; for any other path, and on Windows, no override is written.
@carried n=14 run=2 label=security-plan/data-protection fact=fct-jzj7djty
quote: Unix: 0700 directories and 0600 files, except the pinned `bin/<version>-<hash>/viola`, which is 0700.
@carried n=14 run=3 label=security-plan/data-protection fact=fct-jzj7djty
quote: Before reusing an existing `bin/<version>-<hash>/viola(.exe)`, `viola run` re-hashes the file with SHA-256 and compares the truncated result with `<hash>`. On a mismatch it refuses to start (exit 1).
@carried n=14 run=4 label=security-plan/data-protection fact=fct-jzj7djty
quote: `bin/<version>-<hash>/conpty/{OpenConsole.exe,conpty.dll}` (Windows x64): Microsoft's ConPTY companions, embedded in the exe and written by `viola run` write-if-absent through `replace_private_shared`, never overwritten.
@carried n=14 run=5 label=security-plan fact=fct-7aw7emh6
quote: the override written on Unix only and only for a pinned path of plain characters, since a shell runs it unquoted; the pinned binary and the ConPTY companions are re-hashed before use.
@carried n=15 label=security-plan/data-protection fact=fct-i5etdjvh
why: The pin module loses `pin_companions`, and `pin_bytes`'s re-hash of the pinned binary on a lost placement must stay as it is.
quote: On `Placed::Present` (a concurrent first start placed the copy first) `pin_bytes` re-hashes the copy that stands: equal bytes are reused, and different bytes are `PinError::HashMismatch` with the file left as found, never overwritten or deleted.
@carried n=16 label=security-plan/data-protection fact=fct-gfsndi6o
why: `check_diagnostics` loses its Windows write-rule arm, and on Linux it must keep refusing a symlink and applying the read rule before any diagnostics open.
quote: `open_role_file` and `write_detail` run `viola_state::strict::check_diagnostics` on the existing `diagnostics/` folder and on the role or detail file before `ensure_private_dir` narrows anything: a symlink is refused; a folder takes the read rule; a file takes the read rule on Unix and the write rule on Windows; an absent path passes. A refusal is a failed open: the writer ends at `std::io::sink` and the detail line is dropped.
@carried n=16 run=2 label=security-plan fact=fct-2vvustrg
quote: `events.ndjson` and the files in `instances/<name>/diagnostics/` are 0600 inside 0700 directories.
@carried n=17 label=security-plan/data-protection fact=fct-zaeoull5
why: `batch-script-child` is a closed code whose refusal leaves with the Windows arm, and whether it stays in `verify`'s and `revive --json`'s closed sets is the closed-word fork the scope leaves to P4.
quote: `verify` writes `batch-script-child`, `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed` or `unclean-fixture` (`Ended::Refused`) on the `process-exit` it writes under a parsed `VIOLA_NAME`
@carried n=17 run=2 label=security-plan/input-validation fact=fct-l7hy7pzk
quote: `viola revive --json`'s closed exit-1 codes are `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`.
@binding n=1
**input-validation (statusline shell-out) ↔ data-protection (settings override)**: both Unix gates are lifted together, and the override must stay written only for a pinned path of plain characters because the shell-out runs it through `/bin/sh -c` unquoted; neither may gain reach (security-plan/input-validation fct-m6zdbeoz) (security-plan/data-protection fct-jzj7djty).
@binding n=2
**input-validation (child resolution) ↔ data-protection (verify refusal codes) ↔ input-validation (revive --json codes)**: the `batch-script-child` code must be decided once, the same way, across the runtime refusal, `verify`'s `process-exit` and `revive --json`'s closed set, with no Linux closed word or exit code changing (security-plan/input-validation fct-q67nxnld) (security-plan/data-protection fct-zaeoull5) (security-plan/input-validation fct-l7hy7pzk).
@binding n=3
**input-validation (DLL search + companions) ↔ data-protection (code-bearing artefacts, lost placement)**: the companions, their sideload step and `pin_companions` must leave together, while the pinned binary's SHA-256 re-hash in `pin_bytes` and at reuse stays as it is (security-plan/input-validation fct-bboseg7m) (security-plan/data-protection fct-jzj7djty) (security-plan/data-protection fct-i5etdjvh).
@binding n=4
**input-validation (hook stdin, CLI home, own state files) ↔ data-protection (diagnostics opens) ↔ authentication (strict-modes)**: the Unix strict-modes arm, made unconditional, must refuse exactly what it refused before, before the first read under the home in the hook, the CLI verbs, the state readers and the diagnostics opens alike (security-plan/input-validation fct-p4cpzbge) (security-plan/input-validation fct-wkbf5hxn) (security-plan/data-protection fct-gfsndi6o) (security-plan fct-jya3vgt6).
@binding n=5
**input-validation (registry names) ↔ input-validation (config.json `claude_env_keep`)**: with the registry source gone, the R8 rule must keep its identity floor stripped whatever `claude_env_keep` holds (security-plan/input-validation fct-uooa7rml) (security-plan/input-validation fct-nuclnamd).
@binding n=6
**input-validation (test seams) ↔ dependency-security (release check, G2)**: the two fake-agent seams must stay compiled out of release builds, which the kept Linux `release` job's refusal of test-only features and G2's one exemption read (security-plan/input-validation fct-jd7jb3il) (security-plan/input-validation fct-qyyrikqm) (security-plan fct-h3byb6pq) (security-plan fct-wsxv34pw).
@binding n=7
**input-validation (bounded-readers census) ↔ the removal**: every reader the removal deletes must leave its census row with it, and the census must keep passing on Linux (security-plan/input-validation fct-jy3jkgu2).
