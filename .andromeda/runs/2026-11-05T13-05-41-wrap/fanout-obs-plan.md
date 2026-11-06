@proposal n=1 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §1 Obs Scope Summary
change: fct-t2iihkln — the `viola-pty` entity reads «PTY seam (spawn, read, write, resize, wait, kill) over portable-pty `=0.8.1`», the clause «plus the windows-sys `TerminateProcess` fallback» dropped.
sidecar: §1 viola-pty entity: the windows-sys TerminateProcess kill fallback retired with the Windows code (founder rulings 2026-11-04, inputs#I1, inputs#I5 item 6; recoverable at tag multi-os-final).
rationale: The report's Retired bullet (fct-tulramlm) retires the `windows-sys` dependency, and Symbols / APIs (fct-xxutheix) removes «the windows-sys `TerminateProcess` kill fallback»; Dependencies (fct-www7rgz5) removes `windows-sys` everywhere.
@proposal n=2 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §1 Obs Scope Summary
change: fct-m24p7svx — reads «instrumentable at the seam; the openpty OS API behind it is boundary-only, and the PTY byte stream is user content, never logged», ConPTY dropped.
sidecar: §1 viola-pty instrumentability: ConPTY retired; openpty is the one backend (founder rulings 2026-11-04, inputs#I1).
rationale: Symbols / APIs (fct-xxutheix): `pty_backend()` is now always `openpty`, the ConPTY sideload and `src/conpty.rs` removed; Counts (fct-tjt6dmfu): `pty_backend` words 3 → 1 (`openpty`).
@proposal n=3 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §1 Obs Scope Summary
change: fct-p4bwzizt — the surface reads «ipc-internal (`viola-channel`: JSON-RPC 2.0 over ndjson on a Unix socket, one endpoint per `viola run`)», «a named pipe or» dropped.
sidecar: §1 ipc-internal surface: the Windows named-pipe endpoint retired; the channel is a Unix socket only.
rationale: Retired (fct-tulramlm): «the Windows named-pipe endpoint (`\\.\pipe\viola-<h12>`), its SDDL / SQOS / `accept_remote(false)` controls»; Symbols / APIs (fct-xxutheix): `server/win.rs` and the named-pipe arms of `client.rs` · `endpoint.rs` · `tokio_client.rs` · `server.rs` removed.
@proposal n=4 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: §1 Obs Scope Summary
change: fct-zce67g4u — the OS-API entity reads «OS APIs: openpty, Unix socket, filesystem locks and atomic rename, sysinfo», ConPTY, named pipe and `TerminateProcess` dropped.
sidecar: §1 OS-API entity: ConPTY, the named pipe and TerminateProcess retired with the Windows code.
rationale: Same retired items as the ConPTY, named-pipe and TerminateProcess proposals (Retired fct-tulramlm; Symbols / APIs fct-xxutheix).
@proposal n=5 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: §1 Obs Scope Summary
change: fct-q2zh72zo — the consumption path reads «the tests harness `logs` command (`scripts/agent-run.sh logs`, via `viola-harness`)», the «/ `.ps1`» twin dropped.
sidecar: §1 snapshot / paste-to-AI: the {sh,ps1} shim pair retired; scripts/agent-run.sh is the one harness shim.
rationale: Retired (fct-tulramlm): «the `{sh,ps1}` shim pair (`scripts/agent-run.ps1`; `scripts/agent-run.sh` alone)»; Harness / gate surface (fct-sy4jxsfp): `scripts/agent-run.ps1` deleted.
@proposal n=6 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp dependent_of=D-retired
section: §3 → snapshot-paste-to-ai-integration
change: fct-v725cwjk — opens «`scripts/agent-run.sh logs` merges `events.ndjson` …», the «/ `scripts/agent-run.ps1 logs`» twin dropped.
sidecar: §3 Snapshot / paste-to-AI: scripts/agent-run.ps1 retired; the sh shim alone.
rationale: Harness / gate surface (fct-sy4jxsfp): `scripts/agent-run.ps1` deleted (the harness is `scripts/agent-run.sh` alone).
@proposal n=7 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: §9 CI Integration
change: fct-xyexk37w — Platform reads «GitHub Actions; the push/PR workflow `.github/workflows/ci.yml` beside the scheduled `nightly.yml` (weekly advisories and the `fuzz` job …), matrix `[ubuntu-latest]` (arch CI/CD Platform)», the `windows-mutants.yml` clause and the `windows-2025` / `macos-latest` legs dropped.
sidecar: §9 Platform: windows-mutants.yml and the windows-2025 / macos-latest legs retired; workflows ci.yml and nightly.yml, matrix ubuntu-latest (founder rulings 2026-11-04).
rationale: Harness / gate surface (fct-sy4jxsfp): `ci.yml` jobs keep `strategy.matrix.os: [ubuntu-latest]`, `windows-mutants.yml` deleted, `nightly.yml` unchanged; Counts (fct-tjt6dmfu): workflows 3 → 2, CI matrix 3 OS legs → `[ubuntu-latest]`.
@proposal n=8 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp dependent_of=D-retired
section: §9 CI Integration
change: fct-rzzg2wyr — reads «Each gate is its own `run:` step with `shell: bash` on the `ubuntu-latest` leg, and is copied verbatim into `ci.yml`», «on all three OSes» dropped.
sidecar: §9 Gate commands: the three-OS wording retired; the gates run on the one ubuntu-latest leg.
rationale: Harness / gate surface (fct-sy4jxsfp): matrix `[ubuntu-latest]`; Retired (fct-tulramlm): the `windows-2025` and `macos-latest` CI legs.
@proposal n=9 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: §1 Obs Scope Summary
change: fct-mjdbrsjn — the multi-platform-exporter-compat trigger's source reads «tests excerpt, Coverage Triggers; arch CI/CD (`ubuntu-latest`)», the `windows-2025` / `macos-latest` legs and «Multi-OS» dropped.
sidecar: §1 multi-platform-exporter-compat source: the Windows and macOS CI legs retired; Linux only.
rationale: Retired (fct-tulramlm): the OS target is «Linux only … CI on `ubuntu-latest`», the `windows-2025` and `macos-latest` CI legs retired.
@proposal n=10 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: §1 Obs Scope Summary
change: fct-dao4nifd — reads «one file sink and one line schema, with no OS-specific exporter», the «on every OS … identical fields» clause dropped (Linux is the one OS).
sidecar: §1 multi-platform-exporter-compat signal: the per-OS wording retired on a Linux-only product; the one-sink, one-schema claim kept.
rationale: Counts (fct-tjt6dmfu) os-target class: «an «on every OS» clause is stale wording on a Linux-only product — re-worded to the Linux statement or dropped, the claim it carries kept».
@proposal n=11 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: §3 → logging-stack
change: fct-i4z6jsd7 — the Windows `append(true)` / `FILE_APPEND_DATA` clause dropped; the concurrent-append check reads «a tests-owned concurrent-append check on `ubuntu-latest`»; the two landed figures read «on CI (then all 3 OS legs; now `ubuntu-latest` alone)».
sidecar: §3 Logging stack: the Windows append arm and the three-OS check wording retired; the 2026-09-28 three-OS measurement kept as history.
rationale: Retired (fct-tulramlm): the `windows-2025` and `macos-latest` CI legs; Counts (fct-tjt6dmfu) win-ci class: a dated measurement on `windows-2025` that records history is re-worded, never left as current.
@proposal n=12 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: §3 → logging-stack
change: fct-brueynpb — the failure-class clause reads «a `pty.spawn` (openpty) spawn failure is an `Err` from the seam …», «/ ConPTY» dropped.
sidecar: §3 Logging stack: the ConPTY spawn path retired; openpty only.
rationale: Symbols / APIs (fct-xxutheix): `pty_backend()` now always `openpty`; Retired (fct-tulramlm): the ConPTY sideload.
@proposal n=13 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §1 Obs Scope Summary
change: fct-wrd7y3z2 — reads «`diagnostics/` files are 0600 in 0700 directories, plus refusal telemetry …; a strict-modes failure has a line only on the `ledger-stamps` read in `run`'s version gate and on `verify`'s own `process-exit` for a refused stamp write …», «(Windows: home DACL)» and the `conpty/` companion / `sideload_fallback:"strict-modes-failed"` clause dropped.
sidecar: §1 Vector 7 signal: the Windows home-DACL arm and the ConPTY companion strict-modes line (sideload_fallback) retired; two strict-modes lines remain.
rationale: Retired (fct-tulramlm): «the Windows home owner/DACL strict-modes arm; the ConPTY sideload (… `sideload_fallback`, `run.conpty_sideload`)»; Schema / config (fct-wrfywvkp): `sideload_fallback` leaves `process-start`; Symbols / APIs (fct-xxutheix): `pin_companions` and `PinError::StrictModes` removed.
@proposal n=14 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: §10 SLO Invariants & Telemetry Budgets
change: fct-ydzhack5 — reads «A secret-scan hit, or a `diagnostics/` file that is not 0600», «(or not DACL-protected on Windows)» dropped.
sidecar: §10 failure conditions: the Windows DACL arm retired.
rationale: Retired (fct-tulramlm): the Windows home owner/DACL strict-modes arm; Symbols / APIs (fct-xxutheix): `strict.rs`'s Windows owner/DACL arm and `fs.rs`'s DACL arm of `create_private_dir` removed.
@proposal n=15 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: §3 → log-file-location
change: fct-r43ltybn — Permissions reads «files 0600 in 0700 directories; the tests secret-scan test enforces it», the Windows home-DACL clause dropped.
sidecar: §3 Log file location: the Windows home DACL (user, SYSTEM, Administrators) retired.
rationale: Retired (fct-tulramlm): the Windows home owner/DACL strict-modes arm; Symbols / APIs (fct-xxutheix): the DACL arm of `create_private_dir` removed.
@proposal n=16 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: §3 → otel-sdk-init
change: fct-6bbewo2y — Init order step 4 keeps its Unix arms unconditional («creates it first at 0700»; «owner, mode, not a symlink»; «create `<home>/diagnostics/` with `DirBuilderExt::mode(0o700)`»; «`lstat` owner == `geteuid()`, `mode & 0o077 == 0`, not a symlink»), every «on Windows» DACL clause and the «/ DACL» alternative dropped.
sidecar: §3 OTel SDK init step 4: the Windows owner/DACL arms of the home and diagnostics checks retired; the Unix checks are the only path.
rationale: Retired (fct-tulramlm): the Windows home owner/DACL strict-modes arm; Symbols / APIs (fct-xxutheix): `strict.rs`'s Windows owner/DACL arm and `check_socket_dir`'s off-Unix arm removed, «its Unix body unconditional».
@proposal n=17 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §4 Span / Trace Coverage
change: fct-itztmsse — Scenario `viola run` start reads «Never the endpoint name, the `pinned_bin` path or env values in a span; the child's `--settings` argument, the statusline source path and the command appear in no line or span; a failure's `detail` carries no path or pid; …», «never the sideload companions' paths or hashes» and «a sideload outcome is never a refusal or an exit change» dropped.
sidecar: §4 Scenario 1: the ConPTY sideload clauses retired with run.conpty_sideload.
rationale: Retired (fct-tulramlm): the ConPTY sideload (`sideload_fallback`, `run.conpty_sideload`); Symbols / APIs (fct-xxutheix): `src/cmd/run.rs`'s `run.conpty_sideload` step, `pin_companions` call and `sideload_fallback` removed.
@proposal n=18 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §4 Span / Trace Coverage
change: fct-q67chfay — Scenario `wait` / `last` reads «a serving process that is not the recorded wrapper or runs under another euid ends a CLI `wait` / `last` before any frame …», the «(Linux and Windows)» / «(Linux and macOS)» qualifiers and the «(on macOS an euid mismatch alone, since macOS compares no peer pid, its named carve-out)» clause dropped.
sidecar: §4 Scenario wait/last: the macOS no-peer-pid carve-out and the per-OS qualifiers retired; the server check compares pid and euid.
rationale: Retired (fct-tulramlm): «the macOS `$TMPDIR/viola/` socket dir and the macOS no-peer-pid carve-out»; Symbols / APIs (fct-xxutheix): `SERVER_PID_REPORTED` / `PEER_EUID_REPORTED` folded, `server_verdict` loses the macOS `Matches` arm.
@proposal n=19 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: §7 Error Capture & Reporting
change: fct-pqz5mgd5 — the MCP `instance-unreachable` list reads «… or a frame tool's refused euid or server-verification mismatch — is logged by no line …», «(Linux, Windows and macOS, on macOS an euid mismatch alone)» dropped.
sidecar: §7 MCP rejects: the macOS carve-out and per-OS qualifier retired.
rationale: Retired (fct-tulramlm): the macOS no-peer-pid carve-out; Symbols / APIs (fct-xxutheix): `server_verdict` / `verify_server` lose the `peer_pid_reported` parameter and the macOS `Matches` arm.
@proposal n=20 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu
section: §3 Observability Harness Contract
change: fct-26ufyipy — the start-refusal list drops «`.cmd`/`.bat` child»; the example pair is dropped (the text keeps «two fixed stderr lines, no path or pid»); the writer counts read «`run` (7 sites), `verify` (5)», re-derived by `grep -rcE 'human::refuse\(' src` at chunk 2026-11-05-linux-only-code-and-ci; «`run`'s six» reads «`run`'s five».
sidecar: §3 run terminal rule: the .cmd/.bat start refusal and its human pair retired; human::refuse sites run 8 → 7, verify 6 → 5 (2026-11-05-linux-only-code-and-ci).
rationale: Retired (fct-tulramlm): «the `.cmd`/`.bat` refusal (`batch-script-child`, `AgentError::BatchScriptChild`, its human pair …)»; Counts (fct-tjt6dmfu): «`human::refuse(` sites `run` 8 → 7, `verify` 6 → 5 … obs-plan fct-26ufyipy «`run` (8 sites), `verify` (6)» stale»; the doc's own enumeration of `run`'s six refusals loses the `.cmd`/`.bat` one.
@proposal n=21 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-wrfywvkp
section: §3 → service-identity
change: fct-rrjwwqcp — the identity fields read «`service_name`, `version`, `os` (`linux`, from `std::env::consts::OS`; the schema's one word), `pid`», «windows|macos|» dropped.
sidecar: §3 Service identity: os closed to linux; windows and macos left the schema with their code (inputs#I2).
rationale: Schema / config (fct-wrfywvkp): `schemas/diag-line.v1.json` `os` reads `["linux"]` (was three words); Retired (fct-tulramlm): `os`'s `windows` · `macos` left the schema (`inputs#I2`); Counts (fct-tjt6dmfu): `os` enum 3 → 1.
@remark
D-obs-instrumentation, D-obs-stack and D-obs-pii fire nothing: the report adds no surface or hot path (Coverage fct-oex26kqr, instrumentation n/a), adds or bumps no dependency (fct-www7rgz5), and adds no logging. Many retired-item occurrences sit in the extracted detail files (obs-scope-summary, span-trace-coverage, log-coverage incl. exit-detail 20 → 19 at fct-dsupowm6, telemetry-strategy, error-capture-reporting incl. the wolm site, ci-integration incl. junit-os-sets and windows-mutants rows, pii-scrubbing-compliance): those go to the cascade. Left unproposed as unclear from the report: fct-3epowryi's `exit_source:"kill-fallback"` (only the windows-sys arm is named removed), fct-cstqzpm2's `junit-<os>` upload (artifact names keep their matrix.os form), the «per-OS perf job» wording in fct-4573rvoy / fct-fruln4ot, and contract trace-context-propagation fct-2geebg2r («PIDs are recycled, quickly on Windows», a general fact).
