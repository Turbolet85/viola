@proposal n=1 detector=D-retired severity=warning file=established-decisions anchor=fct-ikuc6e4y basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Established Decisions
change: replace «As landed (chunk 2026-10-10-statusline-pass-through) the wrap is Unix only: on Windows no override is written and no command is run, and that half is owed to the working-route entry "Windows-only live measurements" (...)» with «As landed (chunk 2026-10-10-statusline-pass-through) the wrap holds on every build (Linux only since chunk 2026-11-05-linux-only-code-and-ci; the Windows half retired with the Windows code, recoverable at tag multi-os-final)»
sidecar: [Budget Governor] statusline wrap no longer Unix-only; Windows half and its Windows-only live measurements owner retired (founder rulings 2026-11-04, Linux only)
rationale: the report retires the OS target «Windows first» and the working-route entry «Windows-only live measurements» (fct-tulramlm), and folds WRAPPED_HERE's true so the wrap no longer branches by OS (fct-xxutheix)
@proposal n=2 detector=D-cascade severity=warning file=established-decisions anchor=fct-qwpklckh basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Established Decisions
change: the relied-set paragraph reads «The relied set is `LedgerRow::relied()`: every row of `ALL` in its order, all twenty-four on every build (`WRAPPED_HERE` folded at chunk 2026-11-05-linux-only-code-and-ci)», dropping `relied_on(row, statusline::WRAPPED_HERE)`, «only where the status line is wrapped (Unix)» and the Windows host's twenty-one
sidecar: [CLI Version Compatibility] relied set is all twenty-four rows on every build; the Windows relied set of twenty-one retired with WRAPPED_HERE
rationale: the report folds WRAPPED_HERE so LedgerRow::relied() is all twenty-four rows on every build (fct-xxutheix) and counts the Windows relied set twenty-one → none (fct-tjt6dmfu)
@proposal n=3 detector=D-cascade severity=warning file=established-decisions anchor=fct-qwpklckh basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix dependent_of=D-cascade
section: ## Established Decisions
change: the statusline ledger-row bullet reads «Statusline `rate_limits.*`, the settings-override mechanism, and the shell Claude Code runs a statusline command through. Rows `statusline-override` · `statusline-shell` · `statusline-rate-limits` since chunk 2026-10-13-owed-ledger-rows, relied on every build: the statusline pass-through and, since chunk 2026-10-10-budget-governor, the budget gate's `budget-paused` refusal rest on stamped rows», dropping «on each OS», `statusline::WRAPPED_HERE`, «Unix» and «the Windows legs are owed to ...»
sidecar: [CLI Version Compatibility] statusline rows relied on every build; their Windows legs retired
rationale: WRAPPED_HERE folded, relied() all twenty-four rows on every build (fct-xxutheix); the Windows-only live measurements owner retired (fct-tulramlm)
@proposal n=4 detector=D-cascade severity=warning file=established-decisions anchor=fct-qwpklckh basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu dependent_of=D-cascade
section: ## Established Decisions
change: verify's step-line clause reads «`MM` being `LedgerRow::relied().len()`, 24, the last `[24/24] local-command-compact …`», dropping «24 on Unix» and «21 on Windows, the last `[21/21] local-command-compact …`»
sidecar: [CLI Version Compatibility] verify step lines [NN/24] alone; the [21/21] Windows form retired
rationale: relied set Windows twenty-one → none, LedgerRow::relied() all twenty-four on every build (fct-tjt6dmfu); the report's outcome holds viola verify [NN/24] on Linux (fct-2n5dws2q)
@proposal n=5 detector=D-cascade severity=warning file=established-decisions anchor=fct-qwpklckh basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix dependent_of=D-cascade
section: ## Established Decisions
change: Run B's settings clause reads «Only for shell-plain paths, it also gets one `--settings <probe dir>/typed/statusline.json` after its `--plugin-dir` pair (...)», dropping «On a wrapped OS (Unix), and» and «Windows passes none»
sidecar: [CLI Version Compatibility] Run B's probe --settings no longer gated on a wrapped OS
rationale: WRAPPED_HERE's true folded and the Windows code removed (fct-xxutheix, fct-tulramlm)
@proposal n=6 detector=D-cascade severity=warning file=established-decisions anchor=fct-qwpklckh basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu dependent_of=D-cascade
section: ## Established Decisions
change: the older-stamp clause drops «, and so do one holding the eighteen and `resume-session` and one holding the eighteen, `resume-session` and `agents-join` on Windows; one holding the eighteen, `resume-session`, `agents-join` and `local-command-compact` reads verified on Windows only» and reads «... all `pass`, reads unverified»
sidecar: [CLI Version Compatibility] the Windows-only stamp verdicts (twenty-one relied rows) retired
rationale: with the Windows relied set gone (twenty-one → none) every build relies on all twenty-four rows, so no stamp reads verified on Windows only (fct-tjt6dmfu, fct-xxutheix)
@proposal n=7 detector=D-cascade severity=warning file=established-decisions anchor=fct-qwpklckh basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-ncm6iksr
section: ## Established Decisions
change: the ledger-row bullet «npm-shim → `claude.exe` resolution.» reads «`shim-resolution`: `claude --version` answering (the row reads `run.version_answered` alone; the npm-shim → `claude.exe` resolution and `ProbeRun.program_is_script` removed at chunk 2026-11-05-linux-only-code-and-ci)»
sidecar: [CLI Version Compatibility] shim-resolution row now checks only that claude --version answered; npm-shim → claude.exe resolution removed
rationale: the report removes Search::windows, pathext, is_script, SHIM_TARGET and ProbeRun.program_is_script, the shim-resolution row now reading run.version_answered alone (fct-xxutheix, deviation 1 fct-ncm6iksr)
@proposal n=8 detector=D-retired severity=warning file=established-decisions anchor=fct-qwpklckh basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Established Decisions
change: the `agents-join` bullet drops «; its Windows reading owed on "Windows-only live measurements"» and reads «The row is relied on every build» in place of «on every OS»
sidecar: [CLI Version Compatibility] agents-join's Windows reading and its Windows-only live measurements owner retired
rationale: the OS target is Linux only and the working-route entry «Windows-only live measurements» was retired, its sites re-pointed or retired with the Windows claim they owe (fct-tulramlm, fct-pdxuthnr)
@proposal n=9 detector=D-retired severity=warning file=established-decisions anchor=fct-qwpklckh basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: ## Established Decisions
change: the landed-rows paragraph drops «not measured on Windows (owed to "Windows-only live measurements")» after the paste-newline row and «relied on every OS and not measured on Windows (owed to "Windows-only live measurements")» after the resume-session and local-command-compact rows
sidecar: [CLI Version Compatibility] Windows-only live measurements owners of paste-newline, resume-session and local-command-compact retired
rationale: the Windows claims these owners carried leave with the Windows code; the entry is retired (fct-tulramlm, fct-pdxuthnr)
@proposal n=10 detector=D-retired severity=warning file=established-decisions anchor=fct-qwpklckh basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: ## Established Decisions
change: the statusline-rows landing sentence drops «, not measured on Windows, where they are not relied (owed to "Windows-only live measurements")»
sidecar: [CLI Version Compatibility] statusline rows' Windows non-relied carve-out and owner retired
rationale: the Windows relied set and the Windows-only live measurements entry are retired (fct-tulramlm, fct-tjt6dmfu)
@proposal n=11 detector=D-retired severity=warning file=established-decisions anchor=fct-jzpbbnum basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu
section: ## Established Decisions
change: the code list reads «`program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed`, `unclean-fixture`, beside `interrupted`», `batch-script-child` removed
sidecar: [CLI Version Compatibility] verify's Ended::Refused codes 6 → 5, batch-script-child retired
rationale: the .cmd/.bat refusal batch-script-child is retired (fct-tulramlm) and verify's refusal codes go 6 → 5 (fct-tjt6dmfu)
@proposal n=12 detector=D-retired severity=warning file=established-decisions anchor=fct-44n3w6o5 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Established Decisions
change: the start order reads «program resolution (a given path as given, a bare name on `PATH`; it writes nothing), collision check, pinned copy and plugin folder, version gate, ...», dropping «the `.cmd`/`.bat` refusal» and «the ConPTY sideload (Windows x64, fail-open)»
sidecar: [Session Liveness] start order loses the .cmd/.bat refusal and the ConPTY sideload step
rationale: the report retires the .cmd/.bat refusal and the ConPTY sideload with run.conpty_sideload (fct-tulramlm) and removes src/cmd/run.rs's sideload step, pin_companions call and BatchScriptChild start refusal (fct-xxutheix)
@proposal n=13 detector=D-retired severity=warning file=established-decisions anchor=fct-44n3w6o5 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix dependent_of=D-retired
section: ## Established Decisions
change: the as-landed step list reads «program resolution → collision → pinned copy and plugin → strip plan → version gate → ...», dropping «(the sideload step added by 2026-09-29-sideloaded-conpty)» and the «ConPTY sideload (`run.conpty_sideload`, Windows: `pin_companions` + the absolute-path pre-load; `outcome` ...; its held handles live until `spawn_child` returns)» step
sidecar: [Session Liveness] the as-landed step list drops run.conpty_sideload
rationale: run.conpty_sideload, pin_companions and the held companions are removed (fct-xxutheix, fct-tulramlm)
@proposal n=14 detector=D-cascade severity=warning file=established-decisions anchor=fct-44n3w6o5 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Established Decisions
change: «rewrites the per-session settings override (Unix only; §Occupied Resources, Claude Code integration names)» reads «rewrites the per-session settings override (§Occupied Resources, Claude Code integration names)»
sidecar: [Session Liveness] the per-session settings override no longer qualified Unix only
rationale: WRAPPED_HERE folded, the wrap no longer branching by OS, and an OS qualifier is stale wording on a Linux-only product (fct-xxutheix, fct-tjt6dmfu os-target class)
@proposal n=15 detector=D-retired severity=warning file=established-decisions anchor=fct-uzcv7hop basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Established Decisions
change: «then, on Linux and Windows, compares the connected peer's pid and its OS start time» reads «then compares the connected peer's pid and its OS start time»
sidecar: [Session Liveness] server check no longer qualified by OS; macOS no-peer-pid carve-out retired, SERVER_PID_REPORTED folded
rationale: the macOS no-peer-pid carve-out is retired (fct-tulramlm) and SERVER_PID_REPORTED / PEER_EUID_REPORTED folded, verify_server losing the peer_pid_reported parameter (fct-xxutheix)
@proposal n=16 detector=D-cascade severity=warning file=established-decisions anchor=fct-xqkfyxga basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-www7rgz5
section: ## Established Decisions
change: «because the seam's normal dependencies are exactly portable-pty, tracing, windows-sys and libc» reads «because the seam's normal dependencies are exactly portable-pty, tracing and libc»
sidecar: [Error Handling] PtyError rationale: windows-sys no longer among viola-pty's dependencies
rationale: the report removes windows-sys from every cfg(windows) table, viola-pty's included, and moves libc to plain dependencies (fct-www7rgz5)
@proposal n=17 detector=D-retired severity=warning file=established-decisions anchor=fct-5gkabtaw basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Established Decisions
change: «`AgentError` holds `resolve_program`'s one refusal, `ProgramNotFound`, with a fixed message: `resolve_program` (a given path as given, a bare name on `PATH`) returns `Result<PathBuf, AgentError>`, in the crate and in the root wrapper `src/run/mod.rs`»
sidecar: [Error Handling] AgentError::BatchScriptChild retired; resolve_program's one refusal is ProgramNotFound
rationale: AgentError::BatchScriptChild is retired with the .cmd/.bat refusal (fct-tulramlm); resolve_program takes a given path as given and a bare name on PATH, ProgramNotFound unchanged (fct-xxutheix)
@proposal n=18 detector=D-cascade severity=escalate file=established-decisions anchor=fct-gctodeyz basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-pws3g656
section: ## Established Decisions
change: the census reads «The product sources hold 10 `thiserror::Error` derives», its name list completed with the tenth derive (the report gives the count, not the name)
sidecar: [Error Handling] thiserror census 9 → 10 by its own grep, as measured at chunk 2026-11-05-linux-only-code-and-ci
rationale: the report re-ran the census's own command and read 10 before and after the chunk (fct-pws3g656, fct-dmsu33ao); escalate because the tenth enum's name is not in the report and must be read before the list is amended
@proposal n=19 detector=D-retired severity=warning file=established-decisions anchor=fct-23yj6ah3 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Established Decisions
change: «copies its own exe to `<viola home>/bin/<CARGO_PKG_VERSION>-<short content hash>/viola(.exe)`» reads «... `/viola`»
sidecar: [Deployment / Distribution] pinned copy named viola alone; the .exe form retired with Windows
rationale: the OS target is Linux only, the Windows code removed (fct-tulramlm)
@proposal n=20 detector=D-retired severity=warning file=established-decisions anchor=fct-23yj6ah3 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Established Decisions
change: drop the Windows x64 ConPTY companions passage, from «On Windows x64 the binary also embeds Microsoft's ConPTY companions (`src/conpty.rs` ...» through «... and no bound was raised.»
sidecar: [Deployment / Distribution] ConPTY companions (src/conpty.rs, vendor/conpty, bin/<version>-<hash>/conpty/, pin_companions, held-open re-hash, test seeding) retired
rationale: the report retires the ConPTY sideload with vendor/conpty/<version>/x64/, bin/<version>-<hash>/conpty/ and target/conpty-seed/ (fct-tulramlm), deleting src/conpty.rs and removing pin_companions and the held-open companions (fct-yq5vc7zj, fct-xxutheix); pin_bytes and the pinned copy's re-hash stay
@proposal n=21 detector=D-retired severity=warning file=established-decisions anchor=fct-23yj6ah3 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-pdxuthnr
section: ## Established Decisions
change: the registered-viola-mcp-exits-1 measurement drops «; its Windows leg owed to "Windows-only live measurements"»
sidecar: [Deployment / Distribution] the mcp exit-1 reading's Windows leg and owner retired
rationale: the working-route entry «Windows-only live measurements» is retired and its sites retired with the Windows claim they owe (fct-tulramlm, fct-pdxuthnr)
@proposal n=22 detector=D-retired severity=warning file=established-decisions anchor=fct-5neshki5 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Established Decisions
change: retire the entity whole: the Windows x64 owner/DACL check of pin_companions, PinError::StrictModes and sideload_fallback strict-modes-failed no longer exist
sidecar: [Deployment / Distribution] pin_companions' owner/DACL check and PinError::StrictModes retired with the ConPTY sideload
rationale: the report retires the Windows home owner/DACL strict-modes arm, the ConPTY sideload and sideload_fallback (fct-tulramlm) and removes pin_companions and PinError::StrictModes (fct-xxutheix, deviation 7 fct-kzmmhcbh)
@proposal n=23 detector=D-retired severity=warning file=established-decisions anchor=fct-4jk25cex basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-pdxuthnr
section: ## Established Decisions
change: «It is no capability-ledger row (no `send` relies on it, the ruled limit); its Windows leg is owed to "Windows-only live measurements".» reads «It is no capability-ledger row (no `send` relies on it, the ruled limit).»
sidecar: [Plugin Scope] plugin-precedence reading's Windows leg and owner retired
rationale: the working-route entry «Windows-only live measurements» is retired, its sites retired with the Windows claim they owe (fct-tulramlm, fct-pdxuthnr)
@proposal n=24 detector=D-cascade severity=warning file=established-decisions anchor=fct-4jk25cex basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Established Decisions
change: «and Runs C and D, and Run B on Windows, the user's own status line (on Unix Run B's status line is the probe capture its one probe `--settings` names, ...)» reads «and Runs C and D the user's own status line (Run B's status line is the probe capture its one probe `--settings` names, [CLI Version Compatibility])»
sidecar: [Plugin Scope] Run B's status line is the probe capture on every build; the Windows case retired
rationale: WRAPPED_HERE folded, so Run B always gets the probe --settings (fct-xxutheix); the Windows code is removed (fct-tulramlm)
@proposal n=25 detector=D-retired severity=warning file=established-decisions anchor=fct-onw4pywk basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu
section: ## Established Decisions
change: the revive --json code list reads «`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`», `batch-script-child` removed
sidecar: [CLI Conventions] revive's closed --json refusal codes 11 → 10, batch-script-child retired
rationale: batch-script-child is retired (fct-tulramlm) and revive's codes go 11 → 10, pinned in tests/cross_exit_causes.rs (fct-tjt6dmfu, fct-25zti4qi)
@remark
F1 (RUSTSEC CARRY) and F2 (Linux-only OS target) fit no picked entity: F1 belongs beside [PTY] body fct-omwrdnlc, F2 beside [Scale / Product] fct-hcu7z4ke; unpicked body lines in Established Decisions are stale as well: fct-hcu7z4ke, fct-l5jpezvw (three OSes), fct-omwrdnlc and OPEN fct-bq65okf4 (sideload, Windows garbage-read rationale, wolm owner), fct-73drmylz (Windows lock rationale), fct-w2igrcse (SQOS, named pipes), fct-4nt6qtyp (wolm Windows leg), fct-bvzxxcy7 (three-OS matrix, PATHEXT, .cmd/.bat, claude.exe), fct-7tdo3mcl (npm-shim resolution); the body Conventions fct-nhsg4k7g says nine thiserror derives where the report reads 10, and that belongs to the conventions category
