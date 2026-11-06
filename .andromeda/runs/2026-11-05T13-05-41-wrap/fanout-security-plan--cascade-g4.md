@proposal n=1 detector=D-retired severity=warning file=input-validation anchor=fct-l7hy7pzk basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Input Validation
change: In the CLI arguments / stdin row drop the Windows and macOS clauses: the Home path reads «canonicalised (`std::fs::canonicalize`, once the home exists; the resolution as landed is **Home resolution** below)»; «on Unix the peer euid decision `peer_admitted`» reads «the peer euid decision `peer_admitted`» and «then, on Linux and Windows, the server check» reads «then the server check»; the `revive --json` codes are ten — `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error` (`batch-script-child` gone); Home resolution's Windows verbatim-prefix parenthetical leaves, and `board::scan` runs `check_read` on the home without «on every OS»; every other clause stays.
sidecar: input-validation CLI arguments row — Windows/macOS clauses dropped, revive --json codes 11 → 10 (batch-script-child retired with the Linux-only ruling).
rationale: the report retires the .cmd/.bat refusal and batch-script-child, counts revive's codes 11 → 10, and makes the product Linux only with the server check unconditional (fct-tulramlm, fct-tjt6dmfu, fct-xxutheix).
@proposal n=2 detector=D-retired severity=warning file=input-validation anchor=fct-uooa7rml basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Input Validation
change: Retire the Persistent-environment names (Windows registry) row whole: the `HKCU\Environment` / `HKLM` reader in `src/run/env.rs` and its windows-sys `Win32_System_Registry` use are removed; the R8 persistent set is `config.json`'s `claude_env_keep` alone (Configuration values row), the identity floor stripped whatever it holds.
sidecar: input-validation Persistent-environment names (Windows registry) row retired with the registry reader.
rationale: the report retires the Windows registry environment reader and removes `src/run/env.rs`'s `HKCU\Environment` reader, the exact strip, `claude_env_keep` and the eleven-name floor staying (fct-tulramlm, fct-xxutheix).
@proposal n=3 detector=D-retired severity=warning file=body anchor=fct-cpef5o24 dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Input Validation
change: Retire the body line «Persistent-environment names (Windows registry) — …» whole, its reader removed.
sidecar: body Input Validation registry line retired with the Windows registry reader.
rationale: the broad line of the retired input-validation registry row; the report retires the Windows registry environment reader (fct-tulramlm).
@proposal n=4 detector=D-retired severity=warning file=input-validation anchor=fct-lj2i7oks basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Input Validation
change: In the terminal colour facts row the none list ends «… `TERM` exactly `dumb`, or a non-terminal stdout»: the failed Windows VT enable clause (`GetConsoleMode` / `SetConsoleMode(mode | 0x0004)`, the saved mode restored on drop, never viola-pty's `0x000c`) leaves, and How reads «root bin `src/human/style.rs` `fn decide` / `fn read`, called from `src/cmd/list.rs`» (no `mod vt`); every other clause stays.
sidecar: input-validation terminal colour facts row — the Windows VT enable clause retired; rungs 1–6 and the depth rule unchanged.
rationale: the report removes `src/human/style.rs`'s VT enable (the `vt` fact) and retires decision rung 7 «Windows VT enable failed», rungs 1–6 unchanged (fct-xxutheix, fct-tulramlm, fct-kbhpzsoq).
@proposal n=5 detector=D-retired severity=warning file=input-validation anchor=fct-g433mics basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Input Validation
change: The PTY output bytes row's Witness reads «after a caught feed panic, `chaos_feed_panic_refuses_send_and_keeps_passthrough` reads the outer terminal: the child's bytes (`e4 b8 ad`) reach it, and none of `panic`, `parse-rejected`, `vt100`, `unable` or `hint:` appears on it»; «on Unix», «on every OS» and «the Windows byte reading is owed to "Windows-only live measurements"» leave, the Windows claim retired with the OS.
sidecar: input-validation PTY output bytes witness — the Windows byte reading owed to the retired Windows-only live measurements entry dropped with the Windows claim.
rationale: the report makes viola Linux only and records the Windows-only live measurements entry retired at the 2026-11-04T23-11-43-wrap, its sites re-pointed or retired with their Windows claim (fct-tulramlm, fct-pdxuthnr).
@proposal n=6 detector=D-retired severity=warning file=input-validation anchor=fct-q67nxnld basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Input Validation
change: In Child executable resolution «`viola run` resolves its child program itself and hands portable-pty an absolute path» stays, its Windows reason (portable-pty 0.8.1's own Windows search, the extensionless npm sh shim) and the PATHEXT search, the `claude.cmd`/`claude.bat` → `claude.exe` sibling and the whole `AgentError::BatchScriptChild` refusal (its `batch-script-child` exit, pair and `revive --json` document, the BatBadBut reason) leave; it reads «a given path is used as given and a bare name is looked up on `PATH`; a program not found is `ProgramNotFound` (under `viola revive --json` the document `internal-error`, before `spawn_child`)»; the cwd, revive and ratification clauses stay.
sidecar: input-validation Child executable resolution — Windows PATHEXT search and the .cmd/.bat refusal retired; a given path as given, a bare name on PATH.
rationale: the report removes `resolve_program`'s `Search::windows`, `pathext`, `is_script`, `SHIM_TARGET` and `AgentError::BatchScriptChild` (a given path as given, a bare name on PATH, `ProgramNotFound` unchanged) and retires the .cmd/.bat refusal (fct-xxutheix, fct-tulramlm).
@proposal n=7 detector=D-retired severity=warning file=body anchor=fct-sgmmo3ki dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Input Validation
change: The body line reads «… `viola run` resolves its child itself, a given path as given and a bare name on `PATH`, and hands portable-pty an absolute path; `revive`'s recorded `cwd` …», the clause «on Windows a `.cmd`/`.bat` child is refused, except a `claude` shim, resolved to its fixed `claude.exe` sibling when that exists» dropped.
sidecar: body Child executable resolution line — the Windows .cmd/.bat clause retired.
rationale: the broad line of the input-validation child-resolution row; the report removes the .cmd/.bat refusal and its resolver arms (fct-xxutheix).
@proposal n=8 detector=D-cascade severity=warning file=input-validation anchor=fct-m6zdbeoz basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Input Validation
change: The Statusline shell-out row's Boundary reads «Statusline shell-out (`viola hook statusline`)»; «on Windows that function returns `None` for every input and no command runs» leaves, `shell_argv` returning the fixed `[&str; 3]` argv; «relied on Unix only» reads «relied on every build»; «the Windows shell-out is owed to "Windows-only live measurements"» leaves; every other clause stays.
sidecar: input-validation Statusline shell-out row — Unix-only and Windows clauses dropped; shell_argv returns [&str; 3]; relied on every build.
rationale: the report has `statusline::shell_argv` return `[&str; 3]` (was `Option`) and `LedgerRow::relied()` all twenty-four rows on every build, the Windows relied set retired (fct-xxutheix, fct-mjkivgjx, fct-tulramlm).
@proposal n=9 detector=D-cascade severity=warning file=body anchor=fct-2fgozo5r dependent_of=D-cascade basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Input Validation
change: The body line reads «Statusline shell-out (`viola hook statusline`) — the recorded command runs only from `hook statusline`, as `/bin/sh -c` with the command as one argument; …», «, Unix only» and «, never on Windows» dropped.
sidecar: body Statusline shell-out line — the Unix-only and never-on-Windows wording dropped.
rationale: the broad line of the input-validation statusline row; `shell_argv` now always yields the `/bin/sh -c` argv (fct-xxutheix, fct-mjkivgjx).
@proposal n=10 detector=D-retired severity=warning file=input-validation anchor=fct-bboseg7m basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Input Validation
change: Retire the DLL search order + sideloaded ConPTY companions (Windows x64) row whole: `restrict_dll_search`, the sideload pre-load, `run.conpty_sideload`, `sideload_fallback`, `pin_companions`, `bin/<version>-<hash>/conpty/` and the inbox-ConPTY degrade are removed; `pty_backend()` always reads `openpty`.
sidecar: input-validation DLL search order + ConPTY sideload row retired with the ConPTY sideload.
rationale: the report deletes `viola_pty::sideload`, `src/conpty.rs` and `main`'s `restrict_dll_search`, removes `pin_companions` and `run.conpty_sideload`, and retires the ConPTY sideload (fct-xxutheix, fct-tulramlm, fct-yq5vc7zj).
@proposal n=11 detector=D-retired severity=warning file=body anchor=fct-oqqkxxum dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Input Validation
change: Retire the body line «DLL search order + sideloaded ConPTY companions (Windows x64) — …» whole.
sidecar: body DLL search order + ConPTY companions line retired with the sideload.
rationale: the broad line of the retired input-validation sideload row (fct-tulramlm).
@proposal n=12 detector=D-retired severity=warning file=data-protection anchor=fct-jzj7djty basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Data Protection
change: In Files (`~/.viola/`) the permissions are one rule, «0700 directories and 0600 files, except the pinned `bin/<version>-<hash>/viola`, which is 0700» (no «Unix:» label); the two Windows DACL bullets and the windows-2025 plain-home `Writable` sentence leave, «The strict-modes check runs at startup (Authentication & Authorization, `~/.viola/` access control)» staying; settings.json drops «only on Unix, and» and «, and on Windows,» and the sentence «The Windows half is cut … "Windows-only live measurements"»; the `<hash>` bullet reads `bin/<version>-<hash>/viola` and «no `cc` on the one `[graph]` triple (`x86_64-unknown-linux-gnu`)»; the `conpty/{OpenConsole.exe,conpty.dll}` bullet leaves whole; Probe captures drop «(Unix)» and «and a Windows close, logoff or shutdown»; Interactive probe dirs read «The stop is `SIGINT` / `SIGTERM`» and drop «and a Windows close, logoff or shutdown»; every other clause stays.
sidecar: data-protection Files row — Windows DACL arm, ConPTY companions and console-control stop retired; [graph] one triple.
rationale: the report retires the Windows owner/DACL strict-modes arm, the ConPTY sideload and the console-control stop arm, sets deny targets to `x86_64-unknown-linux-gnu` alone and makes SIGINT/SIGTERM unconditional (fct-tulramlm, fct-wrfywvkp, fct-xxutheix).
@proposal n=13 detector=D-retired severity=warning file=body anchor=fct-7aw7emh6 dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Data Protection
change: The body line reads «plaintext by design, protected by OS permissions only (0700 directories and 0600 files, the pinned `viola` 0700); the plugin files and the settings override are rewritten at every start, the override written only for a pinned path of plain characters, since a shell runs it unquoted; the pinned binary is re-hashed before use».
sidecar: body Files line — the Windows DACL clause and the ConPTY companions re-hash dropped.
rationale: the broad line of the data-protection Files row; the Windows DACL arm and ConPTY companions are retired (fct-tulramlm).
@proposal n=14 detector=D-retired severity=warning file=data-protection anchor=fct-l66ara67 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Data Protection
change: The pinned copy's placement row names `bin/<version>-<hash>/viola` (no `(.exe)`) and ends «`replace_private_shared` keeps only the plugin files».
sidecar: data-protection pinned-copy placement — viola(.exe) → viola; the ConPTY companions leave replace_private_shared's users.
rationale: the report removes `pin_companions` and the ConPTY companions with the sideload, `replace_private_shared` staying (fct-xxutheix, fct-tulramlm).
@proposal n=15 detector=D-retired severity=warning file=data-protection anchor=fct-gfsndi6o basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Data Protection
change: The diagnostics opens row reads «… a folder takes the read rule; a file takes the read rule; an absent path passes», «on Unix and the write rule on Windows» dropped.
sidecar: data-protection diagnostics opens — the Windows write rule retired with strict.rs's Windows arm.
rationale: the report removes `strict.rs`'s Windows owner/DACL arm and `windows_read_verdict` (fct-xxutheix).
@proposal n=16 detector=D-cascade severity=warning file=data-protection anchor=fct-cnkeyzag basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Data Protection
change: The Repository fixtures scrub reads «(the home prefix, both separator spellings, matched exactly, → `~`; the user word, whole word and case-insensitive, → `<user>`)», «and case-folded on Windows» dropped; the user-word scrub and every refusal check stay.
sidecar: data-protection Repository fixtures — the home-prefix scrub matches exactly, the Windows case fold gone.
rationale: the report removes `ledger::scrub`'s case-fold parameter, the scrub matching the home exactly (test `scrub_matches_the_home_case_exactly`), the word scrub a true claim sharing the token (fct-xxutheix, fct-mz5qsqph, fct-pdxuthnr).
@proposal n=17 detector=D-retired severity=warning file=data-protection anchor=fct-zaeoull5 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu
section: ## Data Protection
change: The verify refusal codes row reads «`verify` writes `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed` or `unclean-fixture` (`Ended::Refused`) …», `batch-script-child` dropped; the rest stays.
sidecar: data-protection verify refusal codes 6 → 5, batch-script-child retired.
rationale: the report counts verify's refusal codes 6 → 5 and retires `batch-script-child` with `src/cmd/verify.rs`'s `BatchScriptChild` arm (fct-tjt6dmfu, fct-tulramlm, fct-xxutheix).
@proposal n=18 detector=D-cascade severity=warning file=data-protection anchor=fct-uz6mlfv5 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu
section: ## Data Protection
change: The verify trusted-runs accepted risk reads «… with the user's own status line in Runs C and D (Run B's status line is the probe capture its one probe `--settings` names, the founder's own answer to P4 fork 3 of chunk 2026-10-13-owed-ledger-rows)», «and in Run B on Windows» and «on Unix» dropped; every other clause stays.
sidecar: data-protection verify trusted runs — the Run B on Windows status-line clause dropped on the Linux-only product.
rationale: the report's os-target class makes a per-OS clause stale on a Linux-only product, re-worded to the Linux statement with its claim kept (fct-tjt6dmfu, fct-tulramlm).
@proposal n=19 detector=D-retired severity=warning file=dependency-security anchor=fct-gezignrj basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Dependency Security
change: Retire the vendored Microsoft ConPTY audit row whole (`vendor/conpty/`, `scripts/conpty-vendor.sh` `--verify` / `--probe`, the Authenticode signer check): the binaries and the script are deleted, recoverable at the tag `multi-os-final`.
sidecar: dependency-security vendored ConPTY audit row retired with the dep-conpty group.
rationale: the report deletes `vendor/conpty/` and `scripts/conpty-vendor.sh` and retires the `dep-conpty` supply-chain row and the ConPTY vendor verification CI step (fct-yq5vc7zj, fct-tulramlm).
@proposal n=20 detector=D-retired severity=warning file=body anchor=fct-y5ir4euu dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Dependency Security
change: Retire the body line «The vendored Microsoft ConPTY binaries sit outside every cargo graph … An own audit, never an exemption.» whole.
sidecar: body Dependency Security vendored ConPTY audit line retired.
rationale: the broad line of the retired dep-conpty audit row (fct-tulramlm).
@proposal n=21 detector=D-cascade severity=warning file=dependency-security anchor=fct-o63eny7e basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-wrfywvkp
section: ## Dependency Security
change: The arch-bans row reads «… and `tokio` in the sync crates' graph for the one target triple `x86_64-unknown-linux-gnu` in `deny-sync.toml` …», «for three target triples» replaced; every other clause stays.
sidecar: dependency-security arch bans — deny-sync.toml targets three triples → x86_64-unknown-linux-gnu.
rationale: the report sets `deny.toml` / `deny-sync.toml` `targets = ["x86_64-unknown-linux-gnu"]` (was three triples), every ban staying and `deny-probes: 14/14 banned, control clean` re-read (fct-wrfywvkp, fct-5vhfe5cy).
@proposal n=22 detector=D-retired severity=warning file=dependency-security anchor=fct-mqyejp35 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Dependency Security
change: Retire the ConPTY pins row whole: `src/conpty.rs`'s four pins, the `.gitattributes` `vendor/conpty/** binary` rule and the re-vendor path are removed.
sidecar: dependency-security ConPTY pins row retired; src/conpty.rs and vendor/conpty/ deleted.
rationale: the report deletes `src/conpty.rs` (the four ConPTY pins) and `vendor/conpty/`, and `.gitattributes` loses `vendor/conpty/** binary` (fct-yq5vc7zj, fct-xxutheix, fct-wrfywvkp).
@proposal n=23 detector=D-retired severity=warning file=body anchor=fct-hzqtl6ih dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Dependency Security
change: Retire the body line «The vendored ConPTY package is pinned exactly by four pins …» whole.
sidecar: body Dependency Security ConPTY pins line retired.
rationale: the broad line of the retired ConPTY pins row (fct-tulramlm).
@proposal n=24 detector=D-retired severity=warning file=dependency-security anchor=fct-glgoi2kc basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Dependency Security
change: Retire the windows-mutants.yml row whole (dispatch, permissions, pins, `matrix.*` through `env:`, no uploads): the workflow is deleted, the workflows now `ci.yml` and `nightly.yml`.
sidecar: dependency-security windows-mutants.yml row retired with the workflow; workflows 3 → 2.
rationale: the report deletes `.github/workflows/windows-mutants.yml`, retires the boundary audit's Windows leg and counts workflows 3 → 2 (fct-yq5vc7zj, fct-tulramlm, fct-tjt6dmfu).
@proposal n=25 detector=D-retired severity=warning file=body anchor=fct-lpbl5pyp dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Dependency Security
change: Retire the body line «`windows-mutants.yml` runs only on `workflow_dispatch` …» whole.
sidecar: body Dependency Security windows-mutants.yml line retired.
rationale: the broad line of the retired windows-mutants.yml row (fct-tulramlm).
@proposal n=26 detector=D-retired severity=warning file=bootstrap-phases anchor=fct-sawbqqxr basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Bootstrap phases (derive for route / setup-project)
change: In auth-scaffolding-baseline: `viola-channel` reads «Linux socket hardening (0700 per-user socket directory + chmod 0600 after the bind + `peer_creds` euid check). The per-user directory and the 0600 socket landed …» (SDDL / windows-sys gone); the client drops the SQOS open, reads «`peer_creds().ok()?.euid()`; the decision `peer_admitted`, a refusal reading `server-verify-failed`», drops «built on Linux and Windows» and the SQOS spike / `tests/channel_sqos_open.rs` sentence; `viola-state`'s read rule reads «(`mode & 0o077`)», the `check_present` (the `conpty/` companions) item, «off Unix `Ok(())`» and the Windows protected DACL / owner + DACL clause leave; `viola(.exe)` reads `viola`; the override is «rewritten when the pinned path passes the plain-character rule; it is not written for a pinned path a shell could misread»; every other clause stays.
sidecar: bootstrap auth-scaffolding-baseline — per-OS listener hardening → Linux socket hardening; SQOS, Windows DACL, conpty check and off-Unix arm retired.
rationale: the report removes `server/win.rs`, the SQOS and named-pipe arms, `strict.rs`'s Windows arm and `READ_RIGHTS`, `check_socket_dir`'s off-Unix `Ok(())`, `pin_companions` and `tests/channel_sqos_open.rs` (fct-xxutheix, fct-yq5vc7zj, fct-tulramlm).
@proposal n=27 detector=D-retired severity=warning file=body anchor=fct-pz6zubwd dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Bootstrap phases (derive for route / setup-project)
change: The body line reads «… no auth library: Linux socket hardening and client server verification in `viola-channel`, …», «per-OS listener hardening» replaced.
sidecar: body auth-scaffolding-baseline — per-OS listener hardening → Linux socket hardening.
rationale: the broad line of the bootstrap auth-scaffolding-baseline group; the Windows listener and SQOS client are removed (fct-xxutheix).
@proposal n=28 detector=D-retired severity=warning file=secret-management anchor=fct-fdqq3i6o basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Secret Management
change: The R8 clause reads «every inherited `CLAUDE*` name (matched exactly) is removed except a name in the persistent set — `config.json`'s `claude_env_keep` — and the identity floor …», «ASCII case-insensitive on Windows, exact on Unix» and «Windows registry `Environment` value names» dropped; «The strip, the registry reader and the logs handle names only» reads «The strip and the logs handle names only»; the floor's Windows-host origin and the Linux dev-host reading stay as history.
sidecar: secret-management R8 — the Windows case fold and the registry persistent set retired; the strip matches names exactly.
rationale: the report removes `src/run/env.rs`'s registry reader and case-insensitive match and `CASE_INSENSITIVE` (the R8 strip matches names exactly), `claude_env_keep` and the eleven-name floor staying (fct-xxutheix, fct-tulramlm).
@remark
Body lines no picked g4 entity details stay stale for the body/new-detail passes: fct-32d4ty7r (recvmsg licence exception; F3 belongs beside it), fct-4ggr436n (CI header names windows-mutants.yml), fct-pxt7iprv (zizmor 3 → 2, windows-mutants mention), fct-v76r7jfp (ConPTY vendor step), fct-npplilir (windows-mutants --package arm), fct-n3fibuep and fct-reffq76w (bootstrap ConPTY clauses), fct-6ikkebgm (TLS named pipes), fct-p7rzex2i (handle inheritance CreateProcessW); F1 and F2 (RUSTSEC-2026-0441, the =0.8.1 pin's reason) have no carrying picked entity; the Windows bans fct-fmxdaj54, fct-gmz2hnmb, fct-tpzheerr, fct-my3j53so and fct-z5qs3bqk belong to security-anti-patterns.
