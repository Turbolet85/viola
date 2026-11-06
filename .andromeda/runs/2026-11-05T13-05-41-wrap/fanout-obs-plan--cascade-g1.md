@proposal n=1 detector=D-new-detail severity=warning file=obs-scope-summary anchor=fct-qat4n66a fact=F1 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 1. Obs Scope Summary
change: add after this value: «- an OS target of Linux only: the core on any current distribution, CI on `ubuntu-latest`; the Windows and macOS code removed, recoverable at the tag `multi-os-final` (`ecbe7ab3`);»
sidecar: obs-scope-summary tier values gain the Linux-only OS target (founder's rulings 2026-11-04, inputs#I1, inputs#I5 item 6)
rationale: the report's Retired bullet replaces «Windows first, macOS and Linux» with Linux only, CI on ubuntu-latest, the removed code recoverable at multi-os-final
@proposal n=2 detector=D-retired severity=warning file=obs-scope-summary anchor=fct-drmiyyaa basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 1. Obs Scope Summary
change: - 18 telemetry triggers, including cross-surface propagation through `corr`, perf budgets, chaos fault telemetry, Linux platform compat (CI on `ubuntu-latest`), and binding Founder Directions.
sidecar: obs-scope-summary tier: «multi-OS compat» reworded to Linux platform compat; the trigger count stands
rationale: the Retired bullet retires the three-OS target and the windows-2025 and macos-latest legs; Counts moves the CI matrix to [ubuntu-latest]
@proposal n=3 detector=D-retired severity=warning file=obs-scope-summary anchor=fct-jzzdh6ip basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 1. Obs Scope Summary
change: - **Reason if not fully instrumentable:** N/A for the seam. The openpty OS API behind it is boundary-only (see the OS PTY entry). The PTY byte stream is user content and is never logged (security Data Classifications, user content; brief §3.2 "Nobody parses the rendered screen for content").
sidecar: obs-scope-summary viola-pty: ConPTY leaves the boundary-only OS API behind the seam; openpty alone
rationale: Symbols/APIs removes the conpty-sideload and conpty words of pty_backend (now always openpty) and the ConPTY arms of viola-pty
@proposal n=4 detector=D-retired severity=warning file=body anchor=fct-m24p7svx dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 1. Obs Scope Summary
change: - **Instrumentability:** instrumentable at the seam; the openpty OS API behind it is boundary-only, and the PTY byte stream is user content, never logged.
sidecar: obs-plan §1 viola-pty broad line: ConPTY dropped, openpty alone
rationale: the body's broad line carries the same ConPTY claim as obs-scope-summary#5.3; pty_backend is always openpty per Symbols/APIs
@proposal n=5 detector=D-retired severity=warning file=obs-scope-summary anchor=fct-f3waaflx basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 1. Obs Scope Summary
change: - **Reason if not fully instrumentable:** Instrument viola's call sites (bind, connect, lock, rename, kill-fallback use, and the server check's peer-euid, peer-pid and start-time reads, whose failed check the CLI and the hook log as `server-verify-failed` and MCP logs by no line), not the OS internals. The home's strict-modes checks (`check_instance`, `check_home_tree`, `check_diagnostics`: viola's stat reads) are such call sites; which of their refusals write a line is the strict-modes refusal-telemetry trigger's (below). The socket directory's check (`check_socket_dir`: viola's one `symlink_metadata`) is such a call site, its refusal written under `socket-dir-refused` (MCP by no line), and the wrapper's read of a client's euid in `accept_loop` writes no line.
sidecar: obs-scope-summary OS APIs: the Windows owner/DACL reads, the «(Unix)» euid qualifier and the off-Unix socket-dir arm dropped
rationale: Symbols/APIs removes strict.rs's Windows owner/DACL arm and windows_read_verdict and the off-Unix Ok(()) arm of check_socket_dir (its Unix body unconditional)
@proposal n=6 detector=D-retired severity=warning file=obs-scope-summary anchor=fct-oiibon7o basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 1. Obs Scope Summary
change: the fourth note's last sentence reads «A refused socket directory has its own code, `socket-dir-refused`, and writes its line (the Founder Direction 4 trigger below states it).» — the macOS no-peer-pid carve-out clause dropped; the other notes unchanged
sidecar: obs-scope-summary cli surface notes: the macOS no-peer-pid carve-out retired
rationale: the Retired bullet retires the macOS no-peer-pid carve-out; Symbols/APIs removes server_verdict's macOS Matches arm and SERVER_PID_REPORTED
@proposal n=7 detector=D-retired severity=warning file=obs-scope-summary anchor=fct-kv6frzyp basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 1. Obs Scope Summary
change: in the second note the instance check's parenthesis reads «(`canonical` + `trusted`: stat reads)» — the «on Windows, owner and DACL reads» clause dropped; the measured bands and the other notes unchanged
sidecar: obs-scope-summary hook surface notes: the instance check's Windows owner/DACL reads dropped
rationale: Symbols/APIs removes strict.rs's Windows owner/DACL arm and windows_read_verdict; the Retired bullet retires the Windows home owner/DACL strict-modes arm
@proposal n=8 detector=D-retired severity=warning file=obs-scope-summary anchor=fct-khwnpdov basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 1. Obs Scope Summary
change: in the last paragraph «Every file is 0600 inside a 0700 directory, and the tests secret-scan test enforces it.» and «— a symlink refused, a folder held to the read rule (no group or other bit), a file to the read rule — and a refusal is a failed open» — the «(Windows: home DACL)», «Windows: no read grant to another SID» and «the write rule on Windows» clauses dropped; the rest unchanged
sidecar: obs-scope-summary log file location: the Windows home-DACL and Windows read/write-rule clauses retired; 0600/0700 stands
rationale: the Retired bullet retires the Windows home owner/DACL strict-modes arm; Symbols/APIs removes the DACL arm of create_private_dir and strict.rs's Windows arm
@proposal n=9 detector=D-retired severity=warning file=obs-scope-summary anchor=fct-h2wm2goy basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 1. Obs Scope Summary
change: the exit-1 cause list reads «already-live (a live holder or a fresh replayed beat), stale-heartbeat (a stale holder), squatted-name, sha-256 mismatch, `socket-dir-refused` (…), and for a start made by `viola revive` also no-session, session-live and cwd-missing» — the «`.cmd`/`.bat` child» cause dropped; the rest unchanged
sidecar: obs-scope-summary run path: the .cmd/.bat child exit-1 cause retired with batch-script-child
rationale: the Retired bullet retires the .cmd/.bat refusal (batch-script-child, AgentError::BatchScriptChild); Schema/config drops batch-script-child from the process-exit detail enum
@proposal n=10 detector=D-retired severity=warning file=obs-scope-summary anchor=fct-ycxx4ij4 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 1. Obs Scope Summary
change: the `refusal` / `detail` list reads «… `unverified-cli` exit 12, `server-verify-failed` and `socket-dir-refused`, each exit 21 on CLI `answer` …» — the «on Linux, Windows and macOS (on macOS an euid mismatch alone)» qualifier dropped; the rest unchanged
sidecar: obs-scope-summary dialog-answer path: the three-OS server-verify qualifier and the macOS euid-only case retired
rationale: the Retired bullet retires the macOS no-peer-pid carve-out and the Windows endpoint; Symbols/APIs folds PEER_EUID_REPORTED and SERVER_PID_REPORTED to their Linux value
@proposal n=11 detector=D-retired severity=warning file=obs-scope-summary anchor=fct-mxwnyc6t basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 1. Obs Scope Summary
change: the second item reads «A server-verify mismatch is one `hook-decision{detail:"server-verify-failed"}` at WARN with no frame sent — on a dialog hook `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, detail:"server-verify-failed"}` — and no pid, start time, path or endpoint in the line; a refused socket directory takes the same form with `detail` `socket-dir-refused`.» — the three-OS qualifier and the macOS carve-out clause dropped
sidecar: obs-scope-summary Vector 4 trigger: the three-OS server-verify qualifier and the macOS carve-out retired
rationale: the Retired bullet retires the macOS no-peer-pid carve-out; Symbols/APIs removes server_verdict's macOS Matches arm and the peer_pid_reported parameter
@proposal n=12 detector=D-retired severity=warning file=obs-scope-summary anchor=fct-5ckzuzgq basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 1. Obs Scope Summary
change: the first item reads «`diagnostics/` files are 0600 in 0700 directories, enforced by the tests secret-scan test.»; the second's landed count reads «As landed a strict-modes failure has a line in two places: the `ledger-stamps` read in `run`'s version gate, and a refused `update_stamps` in `viola verify` (`StampsError::StrictModes`), which leaves only verify's own `process-exit` (with an instance; exit 1, `detail` `strict-modes-failed`).» — the Windows home-DACL clause and the Windows x64 `conpty/` companion line dropped; the no-line list and the other items unchanged
sidecar: obs-scope-summary Vector 7 trigger: the Windows home DACL and the conpty companion strict-modes line (PinError::StrictModes, sideload_fallback) retired; three places become two
rationale: the Retired bullet retires the Windows home owner/DACL arm and the ConPTY sideload with sideload_fallback; Symbols/APIs removes pin_companions and PinError::StrictModes
@proposal n=13 detector=D-retired severity=warning file=body anchor=fct-wrd7y3z2 dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 1. Obs Scope Summary
change: - **Required telemetry signal:** `diagnostics/` files are 0600 in 0700 directories, plus refusal telemetry for pinned-exe hash mismatches and ignored symlinks; a strict-modes failure has a line only on the `ledger-stamps` read in `run`'s version gate and on `verify`'s own `process-exit` for a refused stamp write (`detail` `strict-modes-failed`, exit 1); a refused `diagnostics/` folder or file is a failed open, its line dropped; a refused socket directory, read by the strict-modes read verdict, is no strict-modes failure: its line, where it has one, carries its own `detail`, `socket-dir-refused`.
sidecar: obs-plan §1 Vector 7 broad line: the Windows home DACL and the conpty companion strict-modes line retired
rationale: the body's broad line carries the same retired claims as obs-scope-summary#38.1 (Retired: Windows owner/DACL arm, ConPTY sideload, sideload_fallback)
@proposal n=14 detector=D-cascade severity=warning file=obs-scope-summary anchor=fct-3og7ujbn basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-wrfywvkp
section: ## 1. Obs Scope Summary
change: - **Required telemetry signal:** The logger dependency is tracing-subscriber `>=0.3.20` (0.3.23 pinned by tests). Obs dependencies stay Tokio-free in the sync crates and add no C-building crates, and they pass `cargo deny` on its one target, `x86_64-unknown-linux-gnu`.
sidecar: obs-scope-summary Vector 9 trigger: cargo deny's three targets become x86_64-unknown-linux-gnu alone
rationale: Schema/config sets deny.toml / deny-sync.toml targets to ["x86_64-unknown-linux-gnu"] (was three triples); Counts records deny targets 3 → 1
@proposal n=15 detector=D-retired severity=warning file=obs-scope-summary anchor=fct-yfoyoh37 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 1. Obs Scope Summary
change: the third fault class reads «child exit detected on the process handle without EOF;» — the «(ConPTY)» qualifier dropped; the other classes and the gap-b line unchanged
sidecar: obs-scope-summary chaos trigger: the ConPTY qualifier on handle-detected exit dropped
rationale: Symbols/APIs removes the ConPTY arms (pty_backend always openpty) and keeps child exit detected on the handle wait
@proposal n=16 detector=D-retired severity=warning file=obs-scope-summary anchor=fct-7bdi4wod basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 1. Obs Scope Summary
change: - **Required telemetry signal:** One file sink and line schema on Linux (any current distribution; CI on `ubuntu-latest`), each field fixed: permission application by Unix 0600/0700; append atomicity with concurrent hook writers by POSIX `O_APPEND`; the Unix socket in channel logs; openpty in `pty` spans; the server check's reads, the peer pid by `SO_PEERCRED` and the peer euid by `peer_creds` (a mismatch of either reads `server-verify-failed`; an unreadable euid refused). No OS-specific exporter.
sidecar: obs-scope-summary multi-platform-exporter-compat trigger: the Windows and macOS arms, the macOS carve-out and the SERVER_PID_REPORTED / PEER_EUID_REPORTED constants retired; the signal stated for Linux
rationale: the Retired bullet retires the named pipe, the macOS carve-out and the Windows DACL arm; Symbols/APIs removes SERVER_PID_REPORTED and PEER_EUID_REPORTED (peer_euid_admitted is observed == Some(own)) and the ConPTY arms
@proposal n=17 detector=D-retired severity=warning file=body anchor=fct-dao4nifd dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 1. Obs Scope Summary
change: - **Required telemetry signal:** one file sink and line schema on Linux, its fields fixed, and no OS-specific exporter.
sidecar: obs-plan §1 multi-platform broad line: «on every OS» reworded to the Linux statement
rationale: the body's broad line carries the same every-OS claim as obs-scope-summary#44.1; the Retired bullet makes the target Linux only
@proposal n=18 detector=D-retired severity=warning file=obs-scope-summary anchor=fct-n7xftqkr basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 1. Obs Scope Summary
change: the second item's exit-1 list reads «(`already-live`, `stale-heartbeat`, `squatted-name`, `pinned-hash-mismatch`, `socket-dir-refused`, and `viola revive`'s `no-session`, `session-live` and `cwd-missing`)» — `batch-script-child` dropped; the third item reads «The server-verify fail is written as `process-exit{…}` in `cli-<name>.ndjson` by … (for `link` / `unlink` the driven instance's file), distinct from `instance-dead`, with no pid, start time, path or endpoint.» — the three-OS qualifier and the macOS carve-out clause dropped; the fourth item ends at «… reads that one word.» — its macOS carve-out sentence dropped
sidecar: obs-scope-summary Founder Direction 4 trigger: batch-script-child, the three-OS server-verify qualifier and the macOS carve-out retired
rationale: the Retired bullet retires batch-script-child and the macOS no-peer-pid carve-out; Schema/config drops batch-script-child from the process-exit detail enum
@remark
outside the picked entities, §1 body broad lines still name retired OS arms (fct-t2iihkln windows-sys TerminateProcess, fct-zce67g4u ConPTY/named pipe/TerminateProcess, fct-p4bwzizt named pipe, fct-mjdbrsjn windows-2025/macos-latest legs, fct-q2zh72zo agent-run .ps1); §3 fct-26ufyipy, §4 fct-itztmsse/fct-q67chfay, §7 fct-pqz5mgd5, §9 fct-xyexk37w/fct-rzzg2wyr/fct-cstqzpm2 and §10 fct-ydzhack5 are their own categories' sites; F2 is log-coverage's; no scoped detector (instrumentation, stack, PII) fires: the chunk adds no hot path, dependency or logging
