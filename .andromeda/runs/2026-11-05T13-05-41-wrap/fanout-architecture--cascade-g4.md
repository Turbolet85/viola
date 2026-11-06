@proposal n=1 detector=D-retired severity=warning file=conventions anchor=fct-vuikun3q basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Conventions
change: In exit `1`, drop the clause «`viola run` refusing a `.cmd`/`.bat` child program before any spawn (`process-exit{subject:"self", exit_code:1, detail:"batch-script-child"}`, the two fixed `unable:`/`hint:` stderr lines)». Drop `batch-script-child` from revive's `--json` code list, which leaves ten codes: `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`. Drop verify's refusal pair «`the claude CLI is a .cmd or .bat script` / `pass the real executable, not a .cmd or .bat shim`». Every other clause stays as it is.
sidecar: conventions conv-exit-codes: the `.cmd`/`.bat` refusal `batch-script-child` and its human pair are retired with the Windows code (Linux-only ruling 2026-11-04, inputs#I1/I2); revive's `--json` codes 11 to 10.
rationale: The report's Retired bullet retires the `.cmd`/`.bat` refusal (`batch-script-child`, `AgentError::BatchScriptChild`) and its human pair. Its Counts record revive's closed `--json` refusal codes going from 11 to 10.
@proposal n=2 detector=D-retired severity=warning file=conventions anchor=fct-tg3zks37 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu
section: ## Conventions
change: verify's refusal codes read `program-not-found` · `pinned-hash-mismatch` · `version-unreadable` · `strict-modes-failed` · `unclean-fixture` (five), beside `interrupted`. `batch-script-child` is dropped and the rest of the entity is unchanged.
sidecar: conventions conv-exit-codes: verify's refusal codes 6 to 5, `batch-script-child` removed (Linux-only ruling).
rationale: The report's Counts record verify's refusal codes going from 6 to 5. Its Symbols record that `src/cmd/verify.rs` lost its `BatchScriptChild` arm.
@proposal n=3 detector=D-retired severity=warning file=conventions anchor=fct-7s455w3m basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Conventions
change: The parenthetical reads «(a connected peer whose euid is not this user's or cannot be read, `peer_admitted`; or the connected peer's pid or OS start time not the snapshot's, or that check not made)». The «on Unix» and «on Linux and Windows» qualifiers are dropped and the rest is unchanged.
sidecar: conventions conv-exit-codes: the server-verify cause is stated without OS qualifiers (Linux only; `PEER_EUID_REPORTED` / `SERVER_PID_REPORTED` folded).
rationale: The report's Symbols fold `SERVER_PID_REPORTED` and `PEER_EUID_REPORTED`: `peer_euid_admitted` is `observed == Some(own)` and an unreadable euid is still refused. Retired removes the Windows and macOS arms.
@proposal n=4 detector=D-retired severity=warning file=conventions anchor=fct-fw4invxz basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Conventions
change: Retire the entity whole. `server-verify-failed` has no macOS exception now: the peer pid and its OS start time are compared on every connect, and the `v1-18` macOS carve-out leaves with macOS.
sidecar: conventions conv-exit-codes: the macOS no-peer-pid exception to `server-verify-failed` is retired (Linux-only ruling 2026-11-04).
rationale: The report's Retired bullet names «the macOS no-peer-pid carve-out». Its Symbols record that `server_verdict` lost the macOS `Matches` arm.
@proposal n=5 detector=D-retired severity=warning file=conventions anchor=fct-gvna3tj2 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Conventions
change: The parenthetical reads «(its peer euid not this user's or unreadable, `peer_admitted`; or its pid or OS start time not the snapshot's, or the check cannot be made; either reads `UnreachableCause::ServerVerify`; the start time read on `tokio::task::spawn_blocking`)». «on Unix» and «else» are dropped and the rest is unchanged.
sidecar: conventions conv-mcp-results: the MCP frame tool's server check is stated without the Unix/else OS split (Linux only).
rationale: The report's Symbols fold `PEER_EUID_REPORTED`, so the euid check is unconditional. They also record that `viola-mcp`'s `tools.rs` lost the `SERVER_PID_REPORTED` argument.
@proposal n=6 detector=D-retired severity=warning file=standard-contracts anchor=fct-62qgonvf basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Standard Contracts
change: Replace «(all twenty-four on Unix; on Windows twenty-one, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`)» with «(all twenty-four, on every build)». The sentence after it reads «so an all-`pass` stamp of the eighteen pre-2026-10-13-owed-ledger-rows ids, of the twenty-one pre-2026-10-14-resume-and-compact-ledger-rows ids, of the twenty-two pre-2026-10-16-unwrapped-sessions-on-the-board ids or of the twenty-three pre-2026-10-18-compact-post-condition-ledger-row ids reads unverified», and its Windows-only «verified» clauses are dropped. Everything else stays.
sidecar: standard-contracts contract-ledger-stamps: the Windows relied set of twenty-one is retired; `LedgerRow::relied()` is all twenty-four rows on every build (`WRAPPED_HERE` folded).
rationale: The report's Symbols state that `WRAPPED_HERE`'s `true` is folded and `LedgerRow::relied()` is all twenty-four rows on every build. Its Counts move the relied set from Windows twenty-one to none.
@proposal n=7 detector=D-retired severity=warning file=standard-contracts anchor=fct-2tmeycxu basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Standard Contracts
change: The client «reads the connected stream's peer euid (`Client::server_euid`, `TokioClient::server_euid`) and refuses another euid or one it cannot read (`peer_admitted`)», with «where it is reported (`PEER_EUID_REPORTED`, Unix),» dropped. Drop the last sentence, «macOS reports no peer pid and holds the euid check alone, the named carve-out; Windows reads no euid, its DACL and the pid comparison the controls.» The pid and start-time comparison and the no-frame rule are unchanged.
sidecar: standard-contracts contract-channel-methods: `PEER_EUID_REPORTED` is folded; the macOS carve-out and the Windows DACL control are retired (Linux-only ruling).
rationale: The report's Symbols remove `PEER_EUID_REPORTED` and keep the roles of `peer_admitted` / `serve_admitting`. Retired names the macOS no-peer-pid carve-out and the Windows SDDL/SQOS controls.
@proposal n=8 detector=D-retired severity=warning file=standard-contracts anchor=fct-zxepdepr basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Standard Contracts
change: `refused_with` prints `refused_document` in place of the human pair for `pinned-hash-mismatch`, `squatted-name` and `socket-dir-refused`, and for a program not found it prints `internal-error`. `batch-script-child` is dropped and the rest is unchanged.
sidecar: standard-contracts contract-cli-json-output: `batch-script-child` leaves `refused_with`'s document list with the `BatchScriptChild` start refusal.
rationale: The report's Symbols record that `src/cmd/run.rs` lost the `BatchScriptChild` start refusal and that `AgentError::BatchScriptChild` is removed.
@proposal n=9 detector=D-retired severity=warning file=standard-contracts anchor=fct-p4s6aosk basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Standard Contracts
change: In the shell-out sub-bullet, the shell's argv is built by `statusline::shell_argv`, which returns the fixed three-element argv `[&str; 3]`. The ledger row `statusline-shell` reads «probed through Run B's `--shell "$0"` word, relied on every build: `LedgerRow::relied()` holds all twenty-four rows; §Cross-cutting Patterns, Capability ledger». Drop the sentence «On Windows `shell_argv` returns `None` for every input and no command runs; the Windows shell-out is owed to the working-route entry "Windows-only live measurements" (the founder's word on the cut, same day and relay).»
sidecar: standard-contracts contract-hook-statusline: `shell_argv` returns `[&str; 3]`; the Windows no-shell-out clause and its Windows-only owner are retired; `statusline-shell` is relied on every build.
rationale: The report's Symbols state that `statusline::shell_argv` returns `[&str; 3]` (it was `Option`) and that `LedgerRow::relied()` is all twenty-four rows. The report's deviation 3 states that with `WRAPPED_HERE` folded it was always `Some`.
@proposal n=10 detector=D-retired severity=warning file=standard-contracts anchor=fct-a6y4blve basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Standard Contracts
change: The pure `server_verdict(recorded_pid, started_at, observed_pid, observed_start)` compares the snapshot's `pid` and `started_at` with the observed peer pid and that process's OS start time. The joined `verify_server(recorded_pid, started_at, observed_pid)` reads the observed start time through `process_start_time` (a zombie reads none) and returns `server_verdict`'s verdict. Drop «with `peer_pid_reported` false, as on macOS, which reports no peer pid, it returns `Matches` with no comparison: the recorded gap». The rest is unchanged.
sidecar: standard-contracts contract-session-liveness: `server_verdict` / `verify_server` lose `peer_pid_reported` and the macOS `Matches` arm (Linux-only ruling).
rationale: The report's Symbols state that `liveness::server_verdict` / `verify_server` lose the `peer_pid_reported` parameter and the macOS `Matches` arm.
@proposal n=11 detector=D-retired severity=warning file=standard-contracts anchor=fct-ikwybol4 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Standard Contracts
change: «The peer euid decision is `viola_channel`'s: `peer_euid_admitted(observed, own)` is `observed == Some(own)`, so a peer it cannot read is refused; `peer_admitted(observed)` is that decision over this process's `own_euid()`. `Client::server_euid` and `TokioClient::server_euid` read the connected peer through `peer_euid_of` (`peer_creds().ok()?.euid()`), writing and logging nothing.» `PEER_EUID_REPORTED` and «none on Windows» are dropped.
sidecar: standard-contracts contract-session-liveness: `peer_euid_admitted` loses its `reported` parameter; `PEER_EUID_REPORTED` is folded; there is no Windows arm.
rationale: The report's Symbols state that `peer_euid_admitted(observed, own)` is `observed == Some(own)`, that an unreadable euid is still refused, and that `PEER_EUID_REPORTED` is removed with no seam added.
@proposal n=12 detector=D-retired severity=warning file=standard-contracts anchor=fct-yj2flshm basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Standard Contracts
change: «`list` and `revive` resolve `claude` from their current directory with the same resolution `run` uses (`run::resolve_program`: a given path as given, a bare name on `PATH`)», replacing «npm-shim → `claude.exe`». «the join is ledger row `agents-join`, relied on every OS» becomes «relied on every build». The rest is unchanged.
sidecar: standard-contracts contract-session-liveness: the npm-shim to `claude.exe` resolution is retired from the agents read; `agents-join` is relied on every build.
rationale: The report's Symbols state that `resolve_program` lost `Search::windows`, `pathext`, `is_script` and `SHIM_TARGET`, so it takes a given path as given and a bare name on `PATH`. Counts classes «on every OS» as stale wording.
@proposal n=13 detector=D-retired severity=warning file=body anchor=fct-horlp4ay dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Standard Contracts
change: Drop «; none on Windows» from the shell-out parenthetical, which then reads «(the only shell-out in viola)». The rest is unchanged.
sidecar: body Standard Contracts Hook contract: the Windows no-shell-out clause is retired.
rationale: This body line is the broad line of the contract-hook-statusline group: the same claim as proposal 9. `statusline::shell_argv` now returns `[&str; 3]`.
@proposal n=14 detector=D-retired severity=warning file=body anchor=fct-54jxzo44 dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu
section: ## Standard Contracts
change: Drop `batch-script-child` from revive's `--json` `<code>` list, which leaves ten codes: `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`. The rest is unchanged.
sidecar: body Standard Contracts CLI --json output: revive's closed refusal codes 11 to 10.
rationale: This body line is the broad line of the contract-cli-json-output group: the same claim as proposals 1 and 8. The report's Counts move revive's codes from 11 to 10.
@remark
Some body facts outside group 4's picked groups carry the same retired claims and are not proposed here: fct-ld365mzq (Instance snapshot «on Linux and Windows», the macOS gap) and fct-vjccfh2l, fct-s2vyyjop and fct-wular7qm (Occupied Resources IPC). F1-F4 belong to established-decisions, cross-cutting-patterns and stack-and-technologies, so no group 4 entity carries them.
