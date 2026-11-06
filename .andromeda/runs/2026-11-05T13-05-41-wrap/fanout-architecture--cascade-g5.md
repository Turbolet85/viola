@proposal n=1 detector=D-retired severity=warning file=occupied-resources anchor=fct-nm6raycf basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Occupied Resources
change: The head reads «**Linux:**» (was «Linux / macOS»); the socket dir is `$XDG_RUNTIME_DIR/viola/`, and `/tmp/viola-<uid>/` only as a fallback, never the `/tmp` root — the «`$TMPDIR/viola/` on macOS» arm and the «or `$TMPDIR`» root are dropped; every other clause (the dir check, chmod 0600, the `.lock` arbiter, the recorded `endpoint`, the replay rules, no abstract namespace) stays.
sidecar: occupied-resources IPC endpoints — the socket entry is Linux only; the macOS `$TMPDIR/viola/` socket-dir arm retired (founder rulings 2026-11-04, tag `multi-os-final`).
rationale: The report's Symbols names the macOS `$TMPDIR/viola/` socket-directory arm removed from viola-channel (forms now `$XDG_RUNTIME_DIR/viola/` and `/tmp/viola-<uid>/`) and its Retired bullet retires the macOS `$TMPDIR/viola/` socket dir.
@proposal n=2 detector=D-retired severity=warning file=body anchor=fct-wular7qm dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Occupied Resources
change: The broad line's head reads «**Linux:**» (was «Linux / macOS») and «never the `/tmp` or `$TMPDIR` root» reads «never the `/tmp` root»; the rest stands.
sidecar: body Occupied Resources IPC endpoints — the Unix socket line is Linux only, the `$TMPDIR` root mention dropped with the macOS arm.
rationale: The broad line the retired macOS socket-dir detail serves; the report's Retired bullet retires the macOS `$TMPDIR/viola/` socket dir.
@proposal n=3 detector=D-retired severity=warning file=occupied-resources anchor=fct-uqyfijk4 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Occupied Resources
change: «and then, on Unix, the connected peer's euid» reads «and then the connected peer's euid»; the closing sentence «macOS reports no peer pid and stays on the recorded gap, owned by no route entry: there the pid and start-time comparison is absent and `server-verify-failed` means an euid mismatch alone.» is dropped — the pid and start-time comparison is made on every connect.
sidecar: occupied-resources server check — the macOS no-peer-pid recorded gap retired; `SERVER_PID_REPORTED` / `PEER_EUID_REPORTED` folded, the euid check unconditional.
rationale: The report's Retired bullet retires the macOS no-peer-pid carve-out; its Symbols folds `SERVER_PID_REPORTED` and `PEER_EUID_REPORTED` (`peer_euid_admitted(observed, own)` is `observed == Some(own)`, an unreadable euid still refused) and drops the macOS `Matches` arm of `liveness::server_verdict`.
@proposal n=4 detector=D-retired severity=warning file=occupied-resources anchor=fct-bzckmn3n basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: ## Occupied Resources
change: «`viola-harness` (in `viola-e2e`, behind `scripts/agent-run.{sh,ps1}`; …)» reads «`viola-harness` (in `viola-e2e`, behind `scripts/agent-run.sh`; …)»; the rest of the entry stands.
sidecar: occupied-resources test-only binaries — the harness shim is `scripts/agent-run.sh` alone (`agent-run.ps1` deleted).
rationale: The report's Harness / gate surface: `scripts/agent-run.ps1` deleted, the harness is `scripts/agent-run.sh` alone; its Retired bullet retires the `{sh,ps1}` shim pair.
@proposal n=5 detector=D-cascade severity=warning file=occupied-resources anchor=fct-mnor3ks3 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu
section: ## Occupied Resources
change: In the `revive` sentence «(one of the eleven codes, §Standard Contracts)» reads «(one of the ten codes, §Standard Contracts)»; the rest stands.
sidecar: occupied-resources subcommands — revive's closed `--json` refusal codes eleven → ten (`batch-script-child` removed).
rationale: The report's Counts: revive's closed `--json` refusal codes 11 → 10 (`batch-script-child` gone); its Outcome pins revive's ten codes in `tests/cross_exit_causes.rs`.
@proposal n=6 detector=D-cascade severity=warning file=occupied-resources anchor=fct-l6ysfimx basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu
section: ## Occupied Resources
change: The closing pointer «How a home under a regular file reads on each OS: Filesystem, `instances/<ViolaName>/`, below.» reads «How a home under a regular file reads: Filesystem, `instances/<ViolaName>/`, below.»; the rest stands.
sidecar: occupied-resources exit lines — the «on each OS» wording dropped on the Linux-only product.
rationale: The report's Counts `os-target` class: an «on every OS» clause is stale wording on a Linux-only product, re-worded to the Linux statement or dropped, the claim kept.
@proposal n=7 detector=D-retired severity=warning file=occupied-resources anchor=fct-p6lni4fy basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Occupied Resources
change: «when the start wrote a settings override (none on Windows, and none for a pinned path the override rule refuses; see the next bullet)» reads «when the start wrote a settings override (none for a pinned path the override rule refuses; see the next bullet)»; the rest stands.
sidecar: occupied-resources child flags — the «none on Windows» settings-override clause retired with the Windows target.
rationale: The report's Retired bullet retires the OS target «Windows first, macOS and Linux» for «Linux only»; the Windows arm of the override no longer exists.
@proposal n=8 detector=D-retired severity=warning file=occupied-resources anchor=fct-nbf2wdvs basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Occupied Resources
change: Run B reads «and, for shell-plain paths, `--settings <probe dir>/typed/statusline.json` after its `--plugin-dir` pair» — «on Unix» and «(no flag on Windows)» dropped; the rest stands.
sidecar: occupied-resources verify probe — Run B's `--settings` flag no longer Unix-qualified; its Windows clause retired.
rationale: The report's Retired bullet retires the Windows target; with `WRAPPED_HERE` folded (Symbols) the statusline path is the same on every build.
@proposal n=9 detector=D-retired severity=warning file=occupied-resources anchor=fct-ehpssyzz basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Occupied Resources
change: «It is written only on Unix, and only when every character …» reads «It is written only when every character …»; «for any other path, and on Windows, no override is written» reads «for any other path no override is written»; «relied on Unix only, since chunk 2026-10-13-owed-ledger-rows» reads «relied on every build (`LedgerRow::relied()` all twenty-four rows), since chunk 2026-10-13-owed-ledger-rows»; the closing sentence «The Windows override is owed to the working-route entry "Windows-only live measurements".» is dropped.
sidecar: occupied-resources settings override — no longer Unix-only; `statusline-override` relied on every build; the Windows owner sentence retired with the Windows claim it owed.
rationale: The report's Symbols folds `WRAPPED_HERE` (`LedgerRow::relied()` is all twenty-four rows on every build) and its Retired bullet retires the Windows target and records the «Windows-only live measurements» entry retired, its sites retired with the Windows claim they owe.
@proposal n=10 detector=D-retired severity=warning file=body anchor=fct-klpb7auf dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Occupied Resources
change: «written only on Unix, and only for a pinned path of plain characters» reads «written only for a pinned path of plain characters»; the rest stands.
sidecar: body Occupied Resources settings override — the «only on Unix» qualifier dropped on the Linux-only product.
rationale: The broad line the settings-override detail serves; the report retires the Windows target, so the override's Unix-only qualifier no longer distinguishes anything.
@proposal n=11 detector=D-retired severity=warning file=occupied-resources anchor=fct-lb4pv4em basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Occupied Resources
change: «every inherited `CLAUDE*` name (ASCII case-insensitive on Windows, exact on Unix) except a name the user defines persistently — on Windows a value NAME under `HKCU\Environment` or `HKLM\…\Environment` (read through windows-sys Registry, names only), on Unix a name in `config.json` `claude_env_keep`» reads «every inherited `CLAUDE*` name, matched exactly, except a name in `config.json` `claude_env_keep`»; «Names only: the strip, the registry reader and the logs never read a value» reads «Names only: the strip and the logs never read a value»; the floor's measurement history, the Linux dev-host readings and the floor ruling stand.
sidecar: occupied-resources R8 strip — the Windows registry reader and the case-insensitive match removed; the exact strip and `claude_env_keep` the one persistent set.
rationale: The report's Symbols: `src/run/env.rs` loses its registry reader (`HKCU\Environment`) and case-insensitive match (the exact strip, `claude_env_keep` and the eleven-name identity floor stay); Retired names the Windows registry environment reader and the `CASE_INSENSITIVE` R8 fold.
@proposal n=12 detector=D-retired severity=warning file=body anchor=fct-jujxlicd dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Occupied Resources
change: «every inherited `CLAUDE*` name except a name the user defines persistently» reads «every inherited `CLAUDE*` name, matched exactly, except a name in `config.json` `claude_env_keep`»; «the strip, the registry reader and the logs never read a value» reads «the strip and the logs never read a value»; the identity floor clause stands.
sidecar: body Occupied Resources R8 — the registry reader dropped; the persistent set is `claude_env_keep`.
rationale: The broad line the R8 detail serves; the report removes the registry reader of `src/run/env.rs`.
@proposal n=13 detector=D-retired severity=warning file=occupied-resources anchor=fct-5lqu6sil basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: ## Occupied Resources
change: The `AGENT_RUN_CHUNK_BASE` item's last sentence reads «No CI workflow sets it: `ci.yml` and `nightly.yml` run no mutation job.» — the clause «and the dispatch-only `windows-mutants.yml` runs the `--package <member> --file …` arm, which reads no base» is dropped; the other three variables stand.
sidecar: occupied-resources test-harness env — the deleted `windows-mutants.yml` dropped from `AGENT_RUN_CHUNK_BASE`'s CI note.
rationale: The report's Harness / gate surface: `windows-mutants.yml` deleted, `nightly.yml` unchanged; Retired names `windows-mutants.yml` (the boundary audit's Windows leg); workflows 3 → 2.
@proposal n=14 detector=D-retired severity=warning file=occupied-resources anchor=fct-3jytrhfq basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Occupied Resources
change: «`claude_env_keep`: the Unix R8 pass-list» reads «`claude_env_keep`: the R8 pass-list»; the rest stands.
sidecar: occupied-resources config.json — `claude_env_keep` no longer Unix-qualified (the registry reader removed).
rationale: The report's Symbols removes the Windows registry reader of the R8 strip, leaving `claude_env_keep` the one persistent pass-list.
@proposal n=15 detector=D-retired severity=warning file=body anchor=fct-i4wacyrp dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Occupied Resources
change: «`claude_env_keep` (the Unix R8 pass-list)» reads «`claude_env_keep` (the R8 pass-list)»; the rest stands.
sidecar: body Occupied Resources config.json — the Unix qualifier on `claude_env_keep` dropped.
rationale: The broad line the config.json detail serves; the registry reader is removed.
@proposal n=16 detector=D-cascade severity=warning file=occupied-resources anchor=fct-6u55z4cp basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Occupied Resources
change: The entry's head reads «`bin/<version>-<hash>/viola`, the pinned copy (0700)» (was «`viola(.exe)` … (0700 on Unix)»); the placement, `Present` re-hash and `pinned-hash-mismatch` clauses stand.
sidecar: occupied-resources pinned copy — the `.exe` name and the Unix qualifier dropped on the Linux-only product.
rationale: The report's Retired bullet retires the Windows target; `pin_bytes` and the pinned copy's re-hash stay (Symbols).
@proposal n=17 detector=D-cascade severity=warning file=body anchor=fct-hnee7xdr dependent_of=D-cascade basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Occupied Resources
change: «`bin/<version>-<hash>/viola(.exe)`» reads «`bin/<version>-<hash>/viola`» and «(0700 on Unix)» reads «(0700)»; the rest stands.
sidecar: body Occupied Resources pinned copy — the `.exe` name and Unix qualifier dropped.
rationale: The broad line the pinned-copy detail serves; the Windows target is retired.
@proposal n=18 detector=D-retired severity=warning file=occupied-resources anchor=fct-if66544y basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu
section: ## Occupied Resources
change: The code list reads «`program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed` or `unclean-fixture`, beside `interrupted`» — `batch-script-child` dropped; the rest stands.
sidecar: occupied-resources verify refusal codes — `batch-script-child` removed, six → five.
rationale: The report's Counts: verify's refusal codes 6 → 5; Symbols drops `src/cmd/verify.rs`'s `BatchScriptChild` arm; Retired names `batch-script-child`.
@proposal n=19 detector=D-retired severity=warning file=occupied-resources anchor=fct-wdtionxy basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Occupied Resources
change: «a folder takes the read rule, a file takes the read rule on Unix and the write rule on Windows» reads «a folder and a file each take the read rule»; the rest stands.
sidecar: occupied-resources diagnostics open check — the Windows write-rule arm for a file retired.
rationale: The report's Retired bullet retires the Windows home owner/DACL strict-modes arm; Symbols removes `strict.rs`'s Windows owner/DACL arm.
@proposal n=20 detector=D-retired severity=warning file=body anchor=fct-5jecrew3 dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Occupied Resources
change: «dir 0700, files 0600 on Unix» reads «dir 0700, files 0600» and «the file to the read rule on Unix and the write rule on Windows» reads «the file to the read rule»; the rest stands.
sidecar: body Occupied Resources diagnostics/ — the Windows write rule and the Unix qualifier dropped.
rationale: The broad line the diagnostics-check detail serves; the Windows strict-modes arm is retired.
@proposal n=21 detector=D-retired severity=warning file=occupied-resources anchor=fct-hn66ngvt basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Occupied Resources
change: «`settings.json` (the per-session settings override that wraps the statusline; Unix only, see …)» drops «Unix only»; «files 0600 on Unix» reads «files 0600»; the board scan's «on every OS» is dropped; revive's «its `events.ndjson` (the write rule on Unix, the read rule on Windows)» reads «its `events.ndjson` (the write rule)»; the last paragraph reads «The check is proven by five widened modes.» — the Windows `cfg(windows)` proof sentence (as measured at ci#40439188053, `windows-2025`) is dropped.
sidecar: occupied-resources instances/ checks — the Unix/Windows split, the «on every OS» wording and the `windows-2025` cfg(windows) proof retired.
rationale: The report's Retired bullet retires the Windows home owner/DACL strict-modes arm and the `windows-2025` CI leg; Counts `win-ci` class: a dated `windows-2025` measurement recording history is re-worded or retired, never left as current.
@proposal n=22 detector=D-retired severity=warning file=body anchor=fct-cicfj3bq dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Occupied Resources
change: «to the read rule (Unix no group or other bit; Windows no read right granted to another SID)» reads «to the read rule (no group or other bit)» and «`events.ndjson` to the write rule on Unix and the read rule on Windows» reads «`events.ndjson` to the write rule»; the rest stands.
sidecar: body Occupied Resources instances/ — the Windows read and write rules dropped.
rationale: The broad line the instances/ check detail serves; the Windows strict-modes arm is retired.
@proposal n=23 detector=D-retired severity=warning file=occupied-resources anchor=fct-kekl3p46 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Occupied Resources
change: «The read rule is the write rule plus a refusal of any group or other bit (`mode & 0o077`) (`unix_read_verdict`, `check_read`).» — the «on Unix» qualifier, the Windows allow-ACE clause, `READ_RIGHTS` and `windows_read_verdict` dropped; the `Refused` sentence stands.
sidecar: occupied-resources strict read rule — `windows_read_verdict` and `READ_RIGHTS` removed; the mode check alone.
rationale: The report's Symbols removes `strict.rs`'s Windows owner/DACL arm, `windows_read_verdict` and `READ_RIGHTS` (`NotADirectory` · `Symlink` · `Unreadable` unchanged).
@proposal n=24 detector=D-cascade severity=warning file=occupied-resources anchor=fct-vs5zumck basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Occupied Resources
change: «the pinned `viola` (`viola.exe` on Windows)» reads «the pinned `viola`»; the rest stands.
sidecar: occupied-resources check_home_tree — the Windows `viola.exe` name dropped.
rationale: The report's Retired bullet retires the Windows target; the pinned copy has one name.
@remark
Body lines of Occupied Resources with retired claims and no picked entity here, for the body/other passes: fct-vjccfh2l and fct-s2vyyjop (Windows named pipe, SQOS test pipe, deleted tests), fct-lvi3drxp and fct-7qo4h5jo (`viola.exe`), fct-ekftm72e and fct-rryrskwd (ConPTY companions, vendor/conpty), fct-qe46hlhe and fct-swegdoif (conpty-seed, seed_conpty), fct-5q3bjmyd and fct-4u2vf5lk (Windows rationales), fct-7wuxnmfc (mutants scratch), fct-ziwerfk3 (junit-<os>), fct-4whhx2hw (test_support SDDL helpers); Standard Contracts fct-ld365mzq, fct-54jxzo44 and fct-horlp4ay are another section's.
