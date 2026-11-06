@proposal n=1 detector=D-retired severity=warning file=log-coverage anchor=fct-yb5jpaot basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-wrfywvkp
section: ## 6. Log Coverage
change: in the `process-start` row `pty_backend` reads (`openpty`) and the whole `sideload_fallback` (claude-child, Windows, … D-36) entry is dropped; every other field stays
sidecar: log-coverage process-start row: pty_backend openpty alone, sideload_fallback removed with the ConPTY sideload (Linux-only chunk, inputs#I1, inputs#I2)
rationale: the report's Schema/config has sideload_fallback leaving process-start and pty_backend always openpty; Retired names sideload_fallback and the ConPTY sideload
@proposal n=2 detector=D-retired severity=warning file=log-coverage anchor=fct-dsupowm6 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu
section: ## 6. Log Coverage
change: drop `batch-script-child` from the exit-1 list and from verify's refusal codes (now `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed`, `unclean-fixture`); `interrupted` reads «stopped by Ctrl-C or TERM (`SIGINT` / `SIGTERM`)»; the schema closes the `process-exit` `detail` at 19 values, as measured at chunk 2026-11-05-linux-only-code-and-ci
sidecar: log-coverage process-exit catalog: batch-script-child retired, Windows Ctrl-C/Ctrl-Break arm retired, detail enum 20 -> 19 (Linux-only chunk)
rationale: the report's Retired names batch-script-child and the console-control stop arm; Counts moves the process-exit detail enum 20 -> 19 and verify's codes 6 -> 5
@proposal n=3 detector=D-new-detail severity=warning file=log-coverage anchor=fct-dsupowm6 fact=F2 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-emurktt5
section: ## 6. Log Coverage
change: new bullet after it: «- A closed diag-line word no build can emit leaves `schemas/diag-line.v1.json` with its emitter (the overseer's ruling `inputs#I2`, 2026-11-05): on that rule `batch-script-child`, `sideload_fallback` with its five words and `os`'s `windows` · `macos` left at chunk 2026-11-05-linux-only-code-and-ci.»
sidecar: log-coverage: the closed-word rule of inputs#I2 recorded (a word no build emits leaves with its emitter)
rationale: the report's Decisions records the overseer's answer inputs#I2 and Retired lists the closed words that left the schema with their code
@proposal n=4 detector=D-retired severity=warning file=log-coverage anchor=fct-qzbqiq2k basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 6. Log Coverage
change: the frame-verb `server-verify-failed` sentence reads «on a server of another euid, on a server that is not the recorded wrapper, or on a check that cannot be made:» — the «(Unix)» and «(Linux, Windows and macOS, where it means an euid mismatch alone)» qualifiers dropped; the line shape stays
sidecar: log-coverage exit-21 server-verify-failed: the Windows/macOS qualifier and the macOS euid-only carve-out retired (Linux-only chunk)
rationale: the report's Retired names the macOS no-peer-pid carve-out; Symbols/APIs fold peer_euid_admitted to observed == Some(own) and drop the macOS Matches arm
@proposal n=5 detector=D-retired severity=warning file=log-coverage anchor=fct-qiql34nd basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 6. Log Coverage
change: the hook `server-verify-failed` sentence reads «written by `viola hook` on a server of another euid, on a server that is not the recorded wrapper, or on a check that cannot be made:» — «(Unix)» and «(Linux, Windows and macOS, where it means an euid mismatch alone)» dropped; the rest stays
sidecar: log-coverage hook-decision server-verify-failed: the Windows/macOS qualifier and macOS carve-out retired (Linux-only chunk)
rationale: the report's Retired names the macOS no-peer-pid carve-out; liveness::server_verdict loses its macOS Matches arm (Symbols/APIs)
@proposal n=6 detector=D-cascade severity=warning file=error-capture-reporting anchor=fct-c5mu6ru4 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 7. Error Capture & Reporting
change: the backtrace is «raw, never-symbolised frames from `src/panic_frames.rs` — `libc::backtrace` + `dladdr` — up to 62 strings …», the Windows `RtlCaptureStackBackTrace` + `GetModuleHandleExW`/`GetModuleFileNameW` arm dropped; the symbolising-retired history stays as dated, «on the `windows-2025` runner of that chunk»
sidecar: error-capture panic backtrace: the Unix capture the one path, the Windows capture arm removed (Linux-only chunk)
rationale: the report's Symbols/APIs remove the OS arms of src/panic_frames.rs, the Unix capture the one path with the same line shape; Counts says a dated windows-2025 measurement is re-worded as history
@proposal n=7 detector=D-retired severity=warning file=error-capture-reporting anchor=fct-bprhan45 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 7. Error Capture & Reporting
change: «a symlink is refused, a folder and a file are held to the read rule, and an absent path passes» — «on Unix and the write rule on Windows» dropped
sidecar: error-capture check_diagnostics: the Windows write-rule arm retired with the owner/DACL strict-modes arm (Linux-only chunk)
rationale: the report's Retired names the Windows home owner/DACL strict-modes arm; Symbols/APIs remove strict.rs's Windows owner/DACL arm and windows_read_verdict
@proposal n=8 detector=D-retired severity=warning file=error-capture-reporting anchor=fct-vfnyc5f5 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 7. Error Capture & Reporting
change: Catch-site reach reads «a home under a regular file is a strict-modes refusal with no line — `viola verify` exits 1 with the home pair, the frame verbs exit 21 with the instance pair — so it never reaches the `cli` catch site» (the Windows `NotFound` clause dropped); Start refusals under `--json` names `pinned-hash-mismatch`, `squatted-name` and `socket-dir-refused` (`batch-script-child` dropped)
sidecar: error-capture catch sites: the Windows catch-site reach arm and batch-script-child's --json document retired (Linux-only chunk)
rationale: the report's Retired names batch-script-child and AgentError::BatchScriptChild; Symbols/APIs remove src/cmd/run.rs's BatchScriptChild start refusal and the Windows strict-modes arm
@proposal n=9 detector=D-retired severity=warning file=error-capture-reporting anchor=fct-l5yuzjd3 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 7. Error Capture & Reporting
change: «none of `panic`, `parse-rejected`, `vt100`, `unable`, `hint:` reaches the outer terminal, and the child's bytes `e4 b8 ad` reach it» — «on any OS», «on Unix» and the Windows byte reading owed to «Windows-only live measurements» dropped with the ConPTY claim
sidecar: error-capture forced feed panic witness: the Windows byte reading retired with the ConPTY (Linux-only chunk; the wolm entry retired)
rationale: the report's Retired removes ConPTY and records the working-route entry «Windows-only live measurements» retired, its sites re-pointed or retired with the Windows claim
@proposal n=10 detector=D-retired severity=warning file=error-capture-reporting anchor=fct-j5jj3gux basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 7. Error Capture & Reporting
change: the last sentence reads «`AgentError`'s variants include `ProgramNotFound`, its message fixed.» (`BatchScriptChild` dropped)
sidecar: error-capture typed-error census: AgentError::BatchScriptChild removed (Linux-only chunk)
rationale: the report's Symbols/APIs remove AgentError::BatchScriptChild with every caller, ProgramNotFound unchanged; Retired names it
@proposal n=11 detector=D-retired severity=warning file=pii-scrubbing-compliance anchor=fct-ydc7ueix basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 8. PII Scrubbing & Compliance › ### PII Scrubbing
change: under Unscanned uploads drop «On a Windows host it sits in the harness's host mutation scratch `<repo parent>/viola-mutants-scratch/` (`HOST_SCRATCH = cfg!(windows)`, inert elsewhere), outside the repository, whose path is never printed (documents carry only the byte count `scratch_bytes`).»; the next sentence reads «On the Linux dev host a directory named `viola-mutants-scratch/` is only the operator's NOCOW `TMPDIR` …»
sidecar: pii unscanned uploads: the Windows mutation scratch, HOST_SCRATCH and scratch_bytes retired (Linux-only chunk)
rationale: the report's Retired names windows-mutants.yml with viola-mutants-scratch, HOST_SCRATCH and scratch_bytes; Symbols/APIs delete scratch.rs and the harness document's scratch_bytes key
@proposal n=12 detector=D-retired severity=warning file=pii-scrubbing-compliance anchor=fct-ydc7ueix basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 8. PII Scrubbing & Compliance › ### PII Scrubbing
change: the content-sink item reads «A symlink, a folder another user may read (a group or other bit) or a file another user may read (a group or other bit) refuses the open …» — both «Windows: …» arms dropped
sidecar: pii content sink: the Windows SID and write-rule arms of check_diagnostics retired (Linux-only chunk)
rationale: the report's Retired names the Windows home owner/DACL strict-modes arm; Symbols/APIs remove strict.rs's Windows owner/DACL arm and windows_read_verdict
@proposal n=13 detector=D-cascade severity=warning file=pii-scrubbing-compliance anchor=fct-ydc7ueix basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-iol4iyl6
section: ## 8. PII Scrubbing & Compliance › ### PII Scrubbing
change: the three witness items end «on Linux»: server-verification («… with `digit_runs` …, on Linux», the macOS compile-out dropped), strict-modes («… finds no line, on Linux») and socket-directory («… `assert_names_nothing` and `digit_runs`, on Linux», the Windows/macOS clause dropped)
sidecar: pii refusal witnesses: OS lists reduced to Linux (Linux-only chunk)
rationale: the report's Retired makes the target Linux only and deviation 11 makes security_negatives_server_identity.rs ungated; the macOS and Windows legs are gone
@proposal n=14 detector=D-retired severity=warning file=ci-integration anchor=fct-y3ozahco basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 9. CI Integration
change: the row is removed: `scripts/junit-os-sets.py` and the three `junit-<os>` artifacts are retired, recoverable at the tag `multi-os-final`
sidecar: ci-integration: the junit-os-sets.py cross-OS reader row retired with the three junit-os artifacts (Linux-only chunk)
rationale: the report's Retired names scripts/junit-os-sets.py and the three junit-<os> artifacts; Changes lists the script deleted whole
@proposal n=15 detector=D-cascade severity=warning file=ci-integration anchor=fct-xpak76xj basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: ## 9. CI Integration
change: the Lint row reads «… the fail-closed raw-`event!` grep and both-ways ban probes in `scripts/lint-probes.sh` (mechanism in §3 logger-stack-install) …» — «, Linux leg» dropped; the rest stays
sidecar: ci-integration lint stage: lint-probes no longer named a Linux leg, the only leg left (Linux-only chunk)
rationale: the report's Harness/gate surface has `Lint probes` lose its `if: runner.os` condition and the matrix reduced to ubuntu-latest
@proposal n=16 detector=D-cascade severity=warning file=ci-integration anchor=fct-t2ak7i7a basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: ## 9. CI Integration
change: the census row reads «… fails on an unnamed gate, a row that no longer stands, or a gate whose predicate names `windows` or `target_os = "macos"` (`another OS: …`, a fault even when a row names it); its self-check reads its planted cases …»
sidecar: ci-integration OS-gated census: the another-OS fault class added (Linux-only chunk)
rationale: the report's Harness/gate surface adds the census's another-OS fault and re-plants its self-check (new test census_reads_a_gate_naming_another_os_as_a_fault_even_when_named)
@proposal n=17 detector=D-retired severity=warning file=ci-integration anchor=fct-r4tkrl6z basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 9. CI Integration
change: the Mutation row's third cell drops the sentence «The audit's Windows form is the dispatch-only, report-only `windows-mutants.yml` … test-plan §10 Mutation gate).»; the epoch-boundary `run --mutants` rule and the red conditions stay
sidecar: ci-integration Mutation row: the windows-mutants.yml audit leg retired (Linux-only chunk; recoverable at multi-os-final)
rationale: the report's Retired names windows-mutants.yml, the boundary audit's Windows leg; Changes deletes .github/workflows/windows-mutants.yml
@proposal n=18 detector=D-retired severity=warning file=slo-invariants-telemetry-budgets anchor=fct-4od5l5b2 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 10. SLO Invariants & Telemetry Budgets
change: the timed path reads «… the peer euid decision, then the server check (the connected endpoint's peer pid and its OS start time against the one `snapshot.json` read) …» — «on Unix» and «; macOS the recorded gap, no pid comparison» dropped; the measured max stays
sidecar: slo spine-hook budget: the macOS no-pid carve-out retired from the timed path (Linux-only chunk)
rationale: the report's Retired names the macOS no-peer-pid carve-out; liveness::server_verdict loses its macOS Matches arm
@proposal n=19 detector=D-cascade severity=warning file=obs-anti-patterns anchor=fct-hfdtlj6j basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu
section: ## 11. Obs Anti-Patterns (NEVER do these) › ### Logs
change: the caller counts read «callers `run` 7 · `verify` 5 · `list` 2 · `revive` 1 · `plugin install` 1 · `client` 1»
sidecar: obs-anti-patterns print-macro ban: human::refuse sites run 8 -> 7, verify 6 -> 5 (Linux-only chunk)
rationale: the report's Counts re-measures `grep -rcE 'human::refuse\(' src` at this wrap: list 2, plugin 1, revive 1, client 1, run 7, verify 5
@proposal n=20 detector=D-retired severity=warning file=body anchor=fct-pqz5mgd5 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 7. Error Capture & Reporting
change: «… or a frame tool's refused euid or server-verification mismatch — is logged by no line …» — «(Linux, Windows and macOS, on macOS an euid mismatch alone)» dropped
sidecar: obs-plan §7 MCP rejects: the Windows/macOS qualifier and the macOS carve-out retired (Linux-only chunk)
rationale: the report's Retired names the macOS no-peer-pid carve-out; this broad line is the one the MCP error-capture group details
@proposal n=21 detector=D-retired severity=warning file=body anchor=fct-xyexk37w basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: ## 9. CI Integration
change: «… uploading `fuzz/artifacts/` on `failure()` only, see §8 item 5), matrix `ubuntu-latest` (arch CI/CD Platform). No OTLP collector job.» — the dispatch-only `windows-mutants.yml` clause and the `windows-2025` / `macos-latest` legs dropped
sidecar: obs-plan §9 platform: workflows 3 -> 2 (ci.yml, nightly.yml), matrix ubuntu-latest alone (Linux-only chunk)
rationale: the report's Retired names windows-mutants.yml and the windows-2025 / macos-latest legs; Counts moves workflows 3 -> 2 and the matrix to [ubuntu-latest]
@proposal n=22 detector=D-cascade severity=warning file=body anchor=fct-rzzg2wyr basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: ## 9. CI Integration
change: «Each gate is its own `run:` step with `shell: bash`, and is copied verbatim into `ci.yml`:» — «on all three OSes» dropped
sidecar: obs-plan §9 gate commands: the three-OS clause dropped (Linux-only chunk)
rationale: the report's Harness/gate surface keeps ci.yml's matrix at [ubuntu-latest] and removes the pwsh steps and OS conditions; Counts names «three OSes» stale wording
@proposal n=23 detector=D-retired severity=warning file=body anchor=fct-ydzhack5 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 10. SLO Invariants & Telemetry Budgets
change: «- A secret-scan hit, or a `diagnostics/` file that is not 0600.» — «(or not DACL-protected on Windows)» dropped
sidecar: obs-plan §10 failure conditions: the Windows DACL clause retired, the 0600 rule stands (Linux-only chunk)
rationale: the report's Retired names the Windows home owner/DACL strict-modes arm; the DACL arm of create_private_dir is removed (Symbols/APIs)
@remark
Outside group 4: §1 fct-wrd7y3z2 (home DACL, sideload strict-modes line), fct-mjdbrsjn / fct-dao4nifd, fct-q2zh72zo (.ps1), §3 fct-26ufyipy (.cmd/.bat pair, run 8 / verify 6) and §4 fct-itztmsse / fct-q67chfay still carry retired items; error-capture #6.3/#6.4's derive count «9» reads 10 by the report's grep at HEAD and after (fct-pws3g656), the tenth name unstated, left to the orchestrator.
