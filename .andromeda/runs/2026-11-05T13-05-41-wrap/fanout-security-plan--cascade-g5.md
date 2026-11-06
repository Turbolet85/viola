@proposal n=1 detector=D-retired severity=warning file=error-handling anchor=fct-io4zn2wb basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Error Handling
change: replace «and, on Unix, the socket-directory and peer euid checks and, on Linux and Windows, the server verification (macOS the recorded gap),» with «and the socket-directory, peer euid and server verification checks,»; the rest of the entity unchanged.
sidecar: error-handling — the hook's landed checks lose their per-OS split (Windows server verification, macOS gap) with the Linux-only ruling.
rationale: the report's Retired bullet removes the Windows named-pipe endpoint and the macOS no-peer-pid carve-out; Symbols / APIs keep the Linux socket-dir, peer euid and server checks.
@proposal n=2 detector=D-retired severity=warning file=error-handling anchor=fct-2qzjtoe6 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Error Handling
change: drop «, on macOS the whole of the check» so that the parenthesis reads «(the connected peer's euid is refused; the peer is not the recorded wrapper; or the check cannot be made)».
sidecar: error-handling — the mcp frame tool's server-check causes lose the macOS euid-only case.
rationale: the report's Retired bullet removes the macOS no-peer-pid carve-out, so server-verify-failed no longer means an euid mismatch alone on any build.
@proposal n=3 detector=D-retired severity=warning file=error-handling anchor=fct-j463drna basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Error Handling
change: replace the last sentence with «A home under a regular file (unstattable) is such a refusal (the frame verbs: exit 21, the instance pair).», dropping the Windows NotFound clause.
sidecar: error-handling — the home-under-a-file refusal reads as the one Linux rule; the Windows NotFound path leaves with the Windows arm.
rationale: the report retires the Windows home owner/DACL strict-modes arm (Retired; Symbols / APIs: strict.rs's Windows owner/DACL arm removed).
@proposal n=4 detector=D-retired severity=warning file=error-handling anchor=fct-g4i32qdx basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Error Handling
change: `viola_state::stamps::StampsError` (`StrictModes` · `State`, the error `update_stamps` returns) carries fixed `Display` messages with no path or payload: `the capability stamps failed the strict-modes check`, and a write failure its own fixed message.
sidecar: error-handling — PinError::StrictModes and its sideload_fallback mapping removed with the ConPTY companions.
rationale: Symbols / APIs lists pin.rs's PinError::StrictModes removed (pin_bytes and the re-hash stay) and run.rs's sideload_fallback gone; Retired names sideload_fallback.
@proposal n=5 detector=D-retired severity=warning file=error-handling anchor=fct-zu3fn6nz basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Error Handling
change: `resolve_program` returns `Result<PathBuf, AgentError>` in `viola_agent_claude` and in the root wrapper `src/run/mod.rs`, its one refusal `AgentError::ProgramNotFound`, a fixed message (a given path taken as given, a bare name searched on `PATH`); `src/cmd/revive.rs`'s private `Refusal` is another type.
sidecar: error-handling — resolve_program's BatchScriptChild refusal removed with the .cmd/.bat rule.
rationale: Symbols / APIs: viola-agent-claude loses AgentError::BatchScriptChild (a given path as given, a bare name on PATH; ProgramNotFound unchanged); Retired names the .cmd/.bat refusal.
@proposal n=6 detector=D-retired severity=warning file=error-handling anchor=fct-wyrvqwn5 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Error Handling
change: replace «(a symbolic link, or a mode or grant another user may use)» with «(a symbolic link, or a mode another user may use)»; the rest unchanged.
sidecar: error-handling — check_diagnostics reads owner and mode only; the Windows grant test leaves with the DACL arm.
rationale: Symbols / APIs: strict.rs loses its Windows owner/DACL arm, windows_read_verdict and READ_RIGHTS; Retired names the Windows home owner/DACL strict-modes arm.
@proposal n=7 detector=D-retired severity=warning file=security-anti-patterns anchor=fct-izf47ccx basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Security Anti-Patterns (NEVER do these) › ### Authentication
change: NEVER write any channel frame (`send`, `answer`, `hook.dialog`, `wait`/`last`, `pause`/`release`, `link`/`unlink`, and `hook.event` notifications, which carry prompt and reply text) before verifying the server's euid + pid (`peer_creds()`; the client-side euid check, `peer_admitted(client.server_euid())` at the three client seams, before the server verification, an unreadable euid refused) plus sysinfo start time against `snapshot.json`, and NEVER take those reference values (`endpoint`, `pid`, `started_at`) from a `snapshot.json` that has not passed the strict-modes check in the same process. Every frame meets this ban in full: CLI `viola send` · `wait` · `last` · `answer` · `pause` · `release` and `viola link` / `viola unlink` take `endpoint`, `pid` and `started_at` from a `snapshot.json` read after `cmd::client::refuse_untrusted`'s `check_instance` passed in the same process, `viola hook`'s `hook.event` and `hook.dialog` frames after its instance check (`trusted`), and `viola mcp`'s tool frames after `check_instance`, each verifying the server (the CLI verbs through `cmd::client::connect`); no carve-out stands.
sidecar: security-anti-patterns — the frame-before-verification ban loses its Windows pid read and the macOS carve-out with the Linux-only ruling.
rationale: the report's Retired bullet removes the Windows named-pipe endpoint with its SQOS controls and the macOS no-peer-pid carve-out; Symbols / APIs keep peer_admitted and an unreadable euid refused.
@proposal n=8 detector=D-retired severity=warning file=body anchor=fct-6cauxi6n dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Security Anti-Patterns (NEVER do these) › ### Authentication
change: - NEVER write any channel frame before verifying the server (euid, pid and start time) against a `snapshot.json` that passed the strict-modes check in the same process; no carve-out stands.
sidecar: security-plan body — the frame-before-verification broad line loses the macOS carve-out.
rationale: the broad line security-anti-patterns#1.1 details; the report's Retired bullet removes the macOS no-peer-pid carve-out.
@proposal n=9 detector=D-retired severity=warning file=security-anti-patterns anchor=fct-f5tdilgi basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Security Anti-Patterns (NEVER do these) › ### Code Patterns
change: drop the ban whole: the Windows host mutation scratch it guards, its guard (`scratch-refused`, `scratch-wipe-failed`), `HOST_SCRATCH` and the scratch wipe left with `scratch.rs`.
sidecar: security-anti-patterns — the run --mutants Windows scratch-wipe ban retired with windows-mutants.yml and scratch.rs.
rationale: Retired names windows-mutants.yml with viola-mutants-scratch / HOST_SCRATCH / scratch_bytes; Symbols / APIs: scratch.rs deleted whole (HOST_SCRATCH, the Windows scratch wipe, prepare).
@proposal n=10 detector=D-retired severity=warning file=body anchor=fct-z5qs3bqk dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Security Anti-Patterns (NEVER do these) › ### Code Patterns
change: drop the broad line whole (the Windows host mutation scratch ban), retired with the scratch.
sidecar: security-plan body — the run --mutants scratch-wipe broad line retired.
rationale: the broad line security-anti-patterns#2.1 details; the report retires viola-mutants-scratch and HOST_SCRATCH with scratch.rs.
@proposal n=11 detector=D-retired severity=warning file=security-anti-patterns anchor=fct-zvkglgpg basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Security Anti-Patterns (NEVER do these) › ### Universal
change: replace «(`LedgerRow::relied()`, counter `/24` on Unix; on Windows, where the status line is not wrapped, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`, counter `/21`)» with «(`LedgerRow::relied()`, all twenty-four rows on every build, counter `/24`)», «`20 pass  4 fail` on Unix (`17 pass  4 fail` on Windows)» with «`20 pass  4 fail`», and drop «on every OS,»; the rest unchanged.
sidecar: security-anti-patterns — the stamp rule's relied set is all twenty-four rows; the Windows relied set of twenty-one retired.
rationale: Retired names the Windows relied set of twenty-one; Symbols / APIs: WRAPPED_HERE folded, LedgerRow::relied() all twenty-four rows on every build; Counts: relied-win stale.
@proposal n=12 detector=D-retired severity=warning file=security-anti-patterns anchor=fct-bg5w767t basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Security Anti-Patterns (NEVER do these) › ### Universal
change: replace «the IPC DACL or socket-directory check» with «the socket-directory check»; the rest unchanged.
sidecar: security-anti-patterns — the no-switch-off ban loses the IPC DACL control of the retired Windows pipe.
rationale: the report's Retired bullet removes the Windows named-pipe endpoint with its SDDL / SQOS controls, so no IPC DACL stands.
@proposal n=13 detector=D-retired severity=warning file=body anchor=fct-7ackeyph dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Security Anti-Patterns (NEVER do these) › ### Universal
change: replace «(the IPC DACL or socket-directory check,» with «(the socket-directory check,»; the rest unchanged.
sidecar: security-plan body — the no-switch-off broad line loses the IPC DACL control.
rationale: the broad line security-anti-patterns#5.1 details; the report retires the Windows named-pipe endpoint and its SDDL controls.
@proposal n=14 detector=D-cascade severity=warning file=security-anti-patterns anchor=fct-lvhmwli7 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Security Anti-Patterns (NEVER do these) › ### Universal
change: replace «`peer_euid_admitted(observed, own, reported)`» with «`peer_euid_admitted(observed, own)`»; the rest unchanged.
sidecar: security-anti-patterns — peer_euid_admitted loses its reported parameter with PEER_EUID_REPORTED.
rationale: Symbols / APIs: PEER_EUID_REPORTED removed, peer_euid_admitted(observed, own) is observed == Some(own), peer_admitted / serve_admitting keep their roles, no seam added.
@remark
F1-F4 are dependency-security / threat-model-summary facts, not this group's; body bans of these categories that detail no picked entity of this group still name Windows or macOS (fct-4vfalbv2, fct-gqlrkxte, fct-zyth7okk macOS reason, fct-fmxdaj54, fct-gmz2hnmb, fct-tpzheerr, fct-iftwwapx, fct-my3j53so, fct-enn2bam2 viola(.exe)) - left to the body pass.
