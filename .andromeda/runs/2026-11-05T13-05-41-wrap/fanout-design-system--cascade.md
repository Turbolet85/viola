@proposal n=1 detector=D-retired severity=warning file=typography anchor=fct-64jvg77y basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Typography
change: the per-OS table head reads `| Role token | Windows | macOS | Linux (the target) | Last resort |` (was `Windows (live target)` · `macOS (CI-tested)` · `Linux (CI-tested)`); the `--font-field` stack line above it and every row unchanged
sidecar: Typography per-OS table head: Linux marked the target, Windows no longer the live target and macOS no longer CI-tested (OS target Linux only, chunk 2026-11-05-linux-only-code-and-ci)
rationale: the report's Retired bullet retires «Windows first, macOS and Linux» for «Linux only», and its Counts os-target class marks the per-OS target wording stale
@proposal n=2 detector=D-new-detail severity=warning file=typography anchor=fct-yxpd6jki fact=F2 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Typography
change: add after this item `- A 0.2.0 frontend inherits a Linux-only target, its window features on Hyprland: the Linux font resolution ("DejaVu Sans Condensed" + "DejaVu Sans Mono" where the named faces are absent) is the render its typography must hold.`
sidecar: Typography: a 0.2.0 frontend's typography holds on the Linux font resolution, the target Linux only with its window features on Hyprland (F2)
rationale: the report's Retired bullet sets the target to Linux only, with the window features of 0.2.0 on Hyprland, so the Linux render beside this item becomes the one render a frontend must hold
@proposal n=3 detector=D-retired severity=warning file=body anchor=fct-prvstcem basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Typography
change: the Bahnschrift item drops «, and Windows is the first target» and reads «It ships with Windows 10 1709+.»; its DIN, strip-form and semi-condensed justification unchanged
sidecar: Typography rationale: «Windows is the first target» dropped from the Bahnschrift item (OS target Linux only)
rationale: the report's Retired bullet retires the OS target «Windows first» per the founder's rulings of 2026-11-04
@proposal n=4 detector=D-retired severity=warning file=body anchor=fct-ithdbbjj basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Typography
change: the item reads «**Both faces are Windows-only** and the target is Linux only, so each role carries an explicit per-OS fallback stack (below), and the Linux fallback is the target's own rendering, not a degraded one. The stacks end at the generic `sans-serif` / `monospace`, never `system-ui`.» (the «D3 makes macOS and Linux supported» clause gone)
sidecar: Typography rationale: the D3 macOS-and-Linux-supported clause replaced by the Linux-only target; the Linux fallback is the target's rendering
rationale: the report's Retired bullet replaces «Windows first, macOS and Linux» with «Linux only», so D3's supported-OS clause no longer holds
@proposal n=5 detector=D-cascade severity=warning file=surface anchor=fct-kn56sdzi basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Surface: cli › ### Tokens (platform-specific)
change: the width clause reads «read by `viola_pty::host_size()` (a `TIOCGWINSZ` ioctl)», dropping «Windows `GetConsoleScreenBufferInfo`, … elsewhere»; the NAME-only truncation, `COLUMNS` not read and the piped no-width rule unchanged
sidecar: cli width: host_size() reads TIOCGWINSZ alone, its Windows GetConsoleScreenBufferInfo arm removed
rationale: the report's Symbols / APIs lists host_size()'s GetConsoleScreenBufferInfo arm as removed, the TIOCGWINSZ arm now unconditional
@proposal n=6 detector=D-cascade severity=warning file=surface anchor=fct-nmng4ymy basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Surface: cli › ### Component Patterns
change: «holds the home to the read rule on every OS» reads «holds the home to the read rule»; the rest of the item unchanged
sidecar: cli list: the board scan's «on every OS» clause dropped (Linux only); the read rule stands
rationale: the report's Symbols / APIs removes board.rs's OS arms while board_scan's read rule stays, and its Counts os-target class marks an «on every OS» clause as stale wording
@proposal n=7 detector=D-cascade severity=warning file=surface anchor=fct-ypei347w basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-iol4iyl6
section: ## Surface: cli › ### Component Patterns
change: the lead reads «The stale row's colour layer is witnessed end to end on `ubuntu-latest`, the one CI runner, by `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal` (`tests/cross_list_rows.rs`, ungated), measured passing at ci#41918694243:» (the `macos-latest` runner and «on both» gone); the witnessed escape sequence and the no-SGR clause unchanged
sidecar: cli list: the stale-row witness runs ungated on ubuntu-latest alone, its macos-latest leg removed
rationale: the report's deviation 11 makes the witness unconditional, and its Counts win-ci class shrinks the CI matrix to ubuntu-latest
@proposal n=8 detector=D-design-cli-wording severity=warning file=surface anchor=fct-45kqhafb basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Surface: cli › ### Component Patterns
change: drop from `viola run`'s exit-1 start refusals the sub-item «`unable: <name>'s command is a .cmd or .bat script` (Windows) → `hint: pass the real executable, not a .cmd or .bat shim`»; every other cause and hint unchanged
sidecar: cli send/run hints: the .cmd/.bat start refusal and its hint removed (batch-script-child retired)
rationale: the report's Retired bullet names the .cmd/.bat refusal and its human pair, and Symbols / APIs removes the BatchScriptChild start refusal of src/cmd/run.rs
@proposal n=9 detector=D-retired severity=warning file=surface anchor=fct-45kqhafb basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Surface: cli › ### Component Patterns
change: the obs-plan D-20 log detail code list drops `batch-script-child`, reading «… `squatted-name`, `pinned-hash-mismatch`, and for `viola revive` `no-session` and `cwd-missing` …»
sidecar: cli hints: batch-script-child removed from the restated log detail codes
rationale: the report's Retired bullet removes the closed word `batch-script-child` from the schema along with its code (inputs#I2)
@proposal n=10 detector=D-retired severity=warning file=surface anchor=fct-45kqhafb dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Surface: cli › ### Component Patterns
change: drop the sentence «On macOS `server-verify-failed` means an euid mismatch alone: no peer pid is compared, a named gap no route entry owns.»; the eight-verb and socket-dir sentences before it unchanged
sidecar: cli hints: the macOS euid-alone reading of server-verify-failed removed
rationale: the report's Retired bullet retires the macOS no-peer-pid carve-out, and Symbols / APIs drops server_verdict's macOS Matches arm
@proposal n=11 detector=D-retired severity=warning file=surface anchor=fct-a4ss54r5 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Surface: cli › ### Component Patterns
change: retire the item whole (the macOS no-peer-pid server check), because the server check on Linux compares the peer pid and euid
sidecar: cli send: the macOS no-peer-pid carve-out item retired (Linux only)
rationale: the report's Retired bullet names «the macOS no-peer-pid carve-out», and Symbols / APIs removes PEER_EUID_REPORTED, SERVER_PID_REPORTED and the macOS Matches arm
@proposal n=12 detector=D-retired severity=warning file=surface anchor=fct-sx3u7aez basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu
section: ## Surface: cli › ### Component Patterns
change: «`LedgerRow::relied()`: 24 on Unix, 21 on Windows, where the three statusline rows are not relied because the status line is not wrapped there» reads «`LedgerRow::relied()`: 24 on every build»; «on Unix the statusline rows» reads «the statusline rows»; «all three relied on every OS» reads «all three relied»; the summary sample drops its trailing «on Unix»
sidecar: cli verify: relied count 24 on every build, the Windows relied set of twenty-one retired
rationale: the report's Counts names the relied set as Windows twenty-one to none, with LedgerRow::relied() all twenty-four on every build, and Retired names the Windows relied set
@proposal n=13 detector=D-cascade severity=warning file=surface anchor=fct-sx3u7aez basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-ncm6iksr
section: ## Surface: cli › ### Component Patterns
change: after the `[01/24] shim-resolution claude resolves to a real executable  pass` / `... fail` sample add «(the `shim-resolution` row's verdict is `claude --version` answering alone; the printed row unchanged)»
sidecar: cli verify: the shim-resolution row now checks claude --version answering alone (ProbeRun.program_is_script removed)
rationale: the report's deviation 1 has the shim-resolution row read run.version_answered alone and leaves the row's master wording to this wrap
@proposal n=14 detector=D-retired severity=warning file=surface anchor=fct-bctn63fi basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Surface: cli › ### Component Patterns
change: the item reads «A home under a regular file cannot be read and is a strict-modes refusal, not the catch-site line: `verify` prints the home-state pair (exit 1) and the frame verbs the instance-state pair (exit 21).» (the «On Unix» lead and the Windows `NotFound` sentence gone)
sidecar: cli verify: the Windows NotFound reading of a home under a regular file removed
rationale: the report's Retired bullet names the Windows home owner/DACL strict-modes arm, and Symbols / APIs makes check_socket_dir's Unix body unconditional, removing strict.rs's Windows arm
@proposal n=15 detector=D-retired severity=warning file=surface anchor=fct-mep5dn6i basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-25zti4qi
section: ## Surface: cli › ### Component Patterns
change: the closed codes read `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error` (ten; `batch-script-child` gone)
sidecar: cli revive --json: ten closed codes, batch-script-child removed (was eleven)
rationale: the report's Counts moves revive's closed --json refusal codes from 11 to 10, and its Outcome pins revive's ten codes in tests/cross_exit_causes.rs
@proposal n=16 detector=D-retired severity=warning file=body anchor=fct-2t54hjhs dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu
section: ## Surface: cli › ### Component Patterns
change: the revive line's code list drops `batch-script-child`, reading the same ten codes as the `revive --json` item
sidecar: cli revive body line: batch-script-child removed from its code list
rationale: the report's Counts moves revive's closed codes from 11 to 10, the same change as the revive --json item
@proposal n=17 detector=D-design-cli-wording severity=warning file=surface anchor=fct-sedcqxww dependent_of=D-design-cli-wording basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Surface: cli › ### Component Patterns
change: exit 1's `viola run` start refusal list drops «/ `<name>'s command is a .cmd or .bat script`»; its other lines and the `--json` column unchanged
sidecar: cli exit codes row 1: the run .cmd/.bat refusal line removed
rationale: the report's Symbols / APIs removes src/cmd/run.rs's BatchScriptChild start refusal, and the Retired bullet names the .cmd/.bat refusal
@proposal n=18 detector=D-design-cli-wording severity=warning file=surface anchor=fct-sedcqxww basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Surface: cli › ### Component Patterns
change: exit 1's `viola verify` refusal list drops «/ `the claude CLI is a .cmd or .bat script`», reading `unable: the claude CLI was not found` / `the CLI version could not be read` / …; its other lines unchanged
sidecar: cli exit codes row 1: verify's .cmd/.bat refusal line removed (five verify refusals)
rationale: the report's Retired bullet names the human pair «the claude CLI is a .cmd or .bat script», and Counts moves verify's refusal codes from 6 to 5
@proposal n=19 detector=D-retired severity=warning file=surface anchor=fct-pxdkcmfq dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Surface: cli › ### Component Patterns
change: the parenthetical reads «(its pid or start time differs, or the check cannot be made; also a peer whose euid is not this user's or cannot be read)», dropping «on Linux and macOS» and «, on macOS the euid alone»
sidecar: cli exit codes: the server-verification cause loses its macOS euid-alone clause
rationale: the report's Symbols / APIs folds peer_euid_admitted to observed == Some(own), with an unreadable euid still refused, and removes the macOS Matches arm
@remark
Body sites that state the retired target but sit outside any picked group are not proposed here: the Platform line fct-7z5o76y5, the Toolkit VT line fct-2wyyc34q, rung 7 fct-jijjnoyy, the Windows depth branch fct-vzhczpnh, the conhost note fct-jphpatxv and the Git Bash on Windows framing fct-7u3hdiu2 (the warning stays). F1 (slug=surface, the Linux-only target) has no picked entity to carry it and is left for the new-detail pass.
