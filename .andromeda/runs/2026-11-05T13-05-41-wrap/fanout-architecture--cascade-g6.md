@proposal n=1 detector=D-retired severity=warning file=occupied-resources anchor=fct-7uywwkoa basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Occupied Resources
change: in the opening clause «the mode or ACL left as found» reads «the mode left as found»; every other clause of the entity stands.
sidecar: occupied-resources: the instance-check entry points no longer leave an ACL as found — the Windows owner/DACL strict-modes arm is retired (Linux-only ruling, inputs#I1).
rationale: the report's Retired names the Windows home owner/DACL strict-modes arm, and Symbols / APIs removes strict.rs's Windows owner/DACL arm, so no ACL is checked or left.
@proposal n=2 detector=D-retired severity=warning file=occupied-resources anchor=fct-sbmskf35 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Occupied Resources
change: the entity reads the Linux statement alone: «A path under a regular file is a strict-modes refusal (`verify` exits 1 with the home pair, the frame verbs exit 21 with the instance pair).» — the Windows `NotFound` skip, the per-OS oracle and the `windows-2025` measurement dropped.
sidecar: occupied-resources: the path-under-a-regular-file rule keeps its Linux refusal only; the Windows NotFound arm and its per-OS oracle retired (inputs#I1).
rationale: the report retires the OS target «Windows first, macOS and Linux» and the Windows strict-modes arm; its os-enum class makes a per-OS oracle stale.
@proposal n=3 detector=D-retired severity=warning file=occupied-resources anchor=fct-isxdwj6d basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Occupied Resources
change: «`viola list` holds the home to the read rule: `board::scan` reads it through `check_read` with or without an instance, so a 0750 home refuses the whole board (`ScanRefused::Home`).» — «on every OS» and the Windows SID clause dropped.
sidecar: occupied-resources: viola list's home read rule restated for Linux; the Windows SID clause retired with the owner/DACL arm.
rationale: Symbols / APIs: board.rs loses its OS arms and board_scan's read rule stays; the Windows owner/DACL arm is retired; the os-target class makes «on every OS» stale wording.
@proposal n=4 detector=D-retired severity=warning file=occupied-resources anchor=fct-jznqbn54 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Occupied Resources
change: read_stamps_strict's parenthesis reads «(the `viola_state::strict` check first: owner-only modes)», the «owner + DACL on Windows» clause dropped, and «both files 0600 on Unix» reads «both files 0600»; the rest of the entity stands.
sidecar: occupied-resources: ledger/stamps.json's strict read is owner-only modes alone; the Windows owner + DACL clause retired.
rationale: Symbols / APIs removes strict.rs's Windows owner/DACL arm and windows_read_verdict; the OS target is Linux only.
@proposal n=5 detector=D-retired severity=warning file=occupied-resources anchor=fct-i7o6tqmg basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Occupied Resources
change: the leftover clause reads «SIGKILL, a crash or a failed handler install leave it to be removed by hand», «a Windows close, logoff or shutdown» dropped, and «on Unix, when `probe_document` is `Some`» reads «when `probe_document` is `Some`»; the rest stands.
sidecar: occupied-resources: verify's probe dir loses the Windows close / logoff / shutdown leftover case with the console-control stop arm.
rationale: Symbols / APIs: src/cmd/verify/stop.rs's console-control arm is removed, SIGINT / SIGTERM unconditional; the console-control stop arm is in Retired.
@proposal n=6 detector=D-retired severity=warning file=occupied-resources anchor=fct-c3nhxk2n basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Occupied Resources
change: «stopped by Ctrl-C or TERM: `SIGINT` / `SIGTERM` (`cmd::verify::stop`, …)», the Windows `CTRL_C_EVENT` / `CTRL_BREAK_EVENT` arm dropped, and «SIGKILL, a crash and a failed handler install (ignored, fail-open) keep the OS default and leave the dirs to be removed by hand», the Windows close / logoff / shutdown case dropped; the rest stands.
sidecar: occupied-resources: verify's interactive probe dirs are removed on SIGINT / SIGTERM only; the Windows console-control arm retired.
rationale: Symbols / APIs: stop.rs's console-control arm (CTRL_C_EVENT / CTRL_BREAK_EVENT) removed, SIGINT / SIGTERM unconditional; Retired names the console-control stop arm.
@proposal n=7 detector=D-cascade severity=warning file=occupied-resources anchor=fct-jv5gqhbl basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu
section: ## Occupied Resources
change: the count reads «`Instant::now() + WITHIN` reads 31 sites in 23 files under `tests/` (…), as measured at chunk 2026-11-05-linux-only-code-and-ci's wrap»; holder_gone's rule reads «a connect to the path is refused or the path is absent», the «on Windows the pipe is not found» arm dropped, «its socket file stays on Unix» reads «its socket file stays», and «on Unix its `attach` polls» reads «its `attach` polls»; the rest stands.
sidecar: occupied-resources: root WITHIN waits 32 sites in 24 files → 31 in 23 (re-measured at this wrap); the Windows pipe arm of holder_gone retired.
rationale: Counts: within-waits 32 sites in 24 files → 31 in 23 names this fact as stale; the Windows pipe endpoint is retired.
@proposal n=8 detector=D-retired severity=warning file=occupied-resources anchor=fct-w6vpbrjt basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Occupied Resources
change: «holds the `Frozen` guard, `attach(pid)` and its `Drop`, ungated: `attach` sends `kill -STOP` and polls `ps -o stat=` until it reads `T`, and the drop sends `kill -CONT`.» — the one-twin-per-OS framing and the Windows debugger twin dropped; the rest stands.
sidecar: occupied-resources: the Frozen guard is the Unix kill -STOP twin alone, ungated; the Windows debugger twin retired.
rationale: Symbols / APIs: tests/support/frozen.rs keeps the Unix kill -STOP twin ungated.
@proposal n=9 detector=D-retired severity=warning file=occupied-resources anchor=fct-5v4ir6ih basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Occupied Resources
change: «`SOCKET_ROOT_VAR` reading `XDG_RUNTIME_DIR`», the «`TMPDIR` on macOS … elsewhere» arm dropped; the rest stands.
sidecar: occupied-resources: the test socket root reads XDG_RUNTIME_DIR alone; the macOS TMPDIR arm retired.
rationale: Symbols / APIs: tests/support/home.rs's SOCKET_ROOT_VAR reads XDG_RUNTIME_DIR; Retired names the macOS $TMPDIR/viola/ socket dir.
@proposal n=10 detector=D-retired severity=warning file=occupied-resources anchor=fct-3fjm2cf6 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Occupied Resources
change: «stamped at the relied set (`LedgerRow::relied()`: all twenty-four rows on every build) by `contract_ledger_probes` (its `ROW_IDS` the twenty-four ids; `24 pass  0 fail`)», «24 rows on Unix, 21 on Windows» and the trailing «on Unix» dropped; the rest stands.
sidecar: occupied-resources: the 2.1.287 fixture set is stamped at all twenty-four relied rows; the Windows relied set of twenty-one retired.
rationale: Symbols / APIs: WRAPPED_HERE folded, LedgerRow::relied() is all twenty-four rows on every build; Counts: relied-win stale.
@proposal n=11 detector=D-retired severity=warning file=occupied-resources anchor=fct-fs7kfa27 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Occupied Resources
change: «the session's `bin/claude` copy», the Windows `[.exe]` suffix dropped; the `harness-<os>` upload keeps its matrix.os form and the rest stands.
sidecar: occupied-resources: the harness session's claude copy is bin/claude; the Windows .exe form retired.
rationale: Retired: the OS target is Linux only; Harness / gate surface keeps every job and artifact name's matrix.os form, so only the .exe form is stale.
@proposal n=12 detector=D-retired severity=warning file=occupied-resources anchor=fct-xib5fpty basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Occupied Resources
change: «On Windows the keepers are the plain create.» dropped and «On Unix a base that is a link» reads «A base that is a link»; the floor, its statvfs reading, the message and the rest stand.
sidecar: occupied-resources: the e2e-home keepers' Windows plain-create clause retired; the linked-base contract and HEADROOM_FLOOR_MIB unchanged.
rationale: Symbols / APIs: ensure_e2e_home keeps its signature, HEADROOM_FLOOR_MIB, headroom_refusal and the message unchanged; the Windows arm leaves with the Linux-only target.
@proposal n=13 detector=D-retired severity=warning file=occupied-resources anchor=fct-snmldjr6 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Occupied Resources
change: retire the entity — the icacls grant, seed_conpty and target/conpty-seed it describes are gone.
sidecar: occupied-resources: the icacls-through-seed_conpty measurement retired with the ConPTY sideload (inputs#I1).
rationale: Retired names target/conpty-seed/ and the ConPTY sideload; Symbols / APIs: tests/support/home.rs loses seed_conpty and the icacls arms.
@proposal n=14 detector=D-retired severity=warning file=occupied-resources anchor=fct-jyuckxyp basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Occupied Resources
change: retire the entity — scripts/junit-os-sets.py is deleted.
sidecar: occupied-resources: scripts/junit-os-sets.py and its three junit-<os> artifact reads retired.
rationale: Retired names scripts/junit-os-sets.py and the three junit-<os> artifacts; Files lists the script deleted whole.
@proposal n=15 detector=D-retired severity=warning file=occupied-resources anchor=fct-xtqblyzi basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Occupied Resources
change: the first paragraph (the Windows harness arm, HOST_SCRATCH, the guard, the wipe, the DACL, its witness, windows-mutants.yml and scratch_bytes) is retired, the entity reading «`<repo parent>/viola-mutants-scratch/` (outside the repository, a sibling of it): `run --mutants` has no scratch arm and `mutants.out/` stays at the repository root (gitignored).» followed by the Linux dev host paragraph unchanged.
sidecar: occupied-resources: the Windows mutation scratch arm retired with windows-mutants.yml; the dev-host NOCOW TMPDIR fact stays.
rationale: Retired names windows-mutants.yml with viola-mutants-scratch / HOST_SCRATCH / scratch_bytes; Symbols / APIs: scratch.rs deleted whole and mutants.rs loses scratch_bytes and its TMP/TEMP/--output arm.
@proposal n=16 detector=D-retired severity=warning file=occupied-resources anchor=fct-ymvbuev2 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Occupied Resources
change: retire the entity — the windows-2025 owner-only DACL SDDL measurement.
sidecar: occupied-resources: the windows-2025 DACL SDDL oracle note retired with the Windows owner/DACL arm.
rationale: Retired names the Windows home owner/DACL strict-modes arm; Symbols / APIs removes test_support's canonical_sddl · dacl_of · user_sid.
@proposal n=17 detector=D-retired severity=warning file=occupied-resources anchor=fct-znlazsvj basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Occupied Resources
change: retire the entity — the windows-2025 temp-dir ACL oracle and its strict::win test.
sidecar: occupied-resources: the windows-2025 runner-inherited ACL oracle note retired with strict.rs's win module.
rationale: Crates / modules removes the win modules of viola-state's strict.rs; Retired names the Windows owner/DACL arm.
@proposal n=18 detector=D-retired severity=warning file=body anchor=fct-cicfj3bq basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Occupied Resources
change: the read rule's parenthesis reads «(no group or other bit)», the «Windows no read right granted to another SID» clause dropped, and «`events.ndjson` to the write rule on Unix and the read rule on Windows» reads «`events.ndjson` to the write rule»; the rest stands.
sidecar: architecture §Occupied Resources: the instances/ strict-modes rules lose their Windows clauses with the owner/DACL arm.
rationale: Symbols / APIs: strict.rs's Windows owner/DACL arm and windows_read_verdict removed, check_socket_dir's Unix body unconditional.
@proposal n=19 detector=D-retired severity=warning file=body anchor=fct-qe46hlhe basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Occupied Resources
change: retire the line — target/conpty-seed/ and seed_conpty are gone.
sidecar: architecture §Occupied Resources: target/conpty-seed/ retired with the ConPTY sideload.
rationale: Retired names target/conpty-seed/; Symbols / APIs: tests/support/home.rs loses seed_conpty; tests/conpty_sideload.rs deleted.
@proposal n=20 detector=D-retired severity=warning file=body anchor=fct-7wuxnmfc dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Occupied Resources
change: «`<repo parent>/viola-mutants-scratch/` (outside the repository, a sibling of it): on the Linux dev host the operator's NOCOW `TMPDIR` for mutation runs, an environment fact, not harness code; `run --mutants` has no scratch arm.» — the Windows arm, its guard and «Off Windows the harness arm is inert» dropped.
sidecar: architecture §Occupied Resources: the mutation scratch's Windows harness arm retired; the dev-host TMPDIR fact kept.
rationale: Retired names viola-mutants-scratch / HOST_SCRATCH / scratch_bytes with windows-mutants.yml; scratch.rs deleted whole.
@remark
other §Occupied Resources body lines carry retired Windows/macOS claims with no picked entity here (fct-vjccfh2l fct-s2vyyjop fct-wular7qm fct-lvi3drxp fct-7qo4h5jo fct-jujxlicd fct-i4wacyrp fct-hnee7xdr fct-ekftm72e fct-5jecrew3 fct-klpb7auf fct-4whhx2hw fct-swegdoif fct-rryrskwd fct-5q3bjmyd fct-4u2vf5lk); task facts F1-F4 belong to established-decisions, cross-cutting-patterns and stack-and-technologies, not this group.
