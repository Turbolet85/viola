@proposal n=1 detector=D-retired severity=warning file=span-trace-coverage anchor=fct-2tnvi4o3 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 4. Span / Trace Coverage
change: replace the item's two closing sentences with «The peer pid and start-time comparison runs on Linux, the one target; `server-verify-failed` covers a refused euid, a mismatch and a check that cannot be made, with no OS carve-out.»; the rest of the item stands verbatim
sidecar: span-trace-coverage — the cli server check's Linux-and-Windows comparison and macOS euid-only carve-out retired with the Windows and macOS code (Linux-only ruling)
rationale: the report's Retired bullet retires the macOS no-peer-pid carve-out with the Windows and macOS code, and Symbols / APIs drops verify_server's peer_pid_reported parameter and the macOS Matches arm
@proposal n=2 detector=D-retired severity=warning file=span-trace-coverage anchor=fct-jvpve25b basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 4. Span / Trace Coverage
change: end the item with «`peer_admitted` refuses a peer of another euid and a peer whose euid it cannot read (`peer_euid_admitted(observed, own)` is `observed == Some(own)`).» — the qualifier «where euids are reported (`PEER_EUID_REPORTED`, Unix)» and the closing Windows sentence dropped
sidecar: span-trace-coverage — the client euid check reads without PEER_EUID_REPORTED and its Windows no-euid sentence
rationale: Symbols / APIs removes the constant PEER_EUID_REPORTED, folds its Linux value into peer_euid_admitted (observed == Some(own), an unreadable euid still refused) and retires the Windows named-pipe controls
@proposal n=3 detector=D-retired severity=warning file=span-trace-coverage anchor=fct-vivmhmma basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: ## 4. Span / Trace Coverage
change: end the item at «Their span recorder `Seen` stands in the test module `refused_seen` of `src/cmd/client.rs`.» — the clause «out of the macOS-gated `refused_inside`» dropped
sidecar: span-trace-coverage — the socket-directory captured-span item drops its macOS-gated module reference
rationale: the report's Harness / gate surface makes any gate whose predicate names target_os macos a census fault and its Outcome reads the carry pattern clean on every .rs line, so no macOS-gated module stands in src/cmd/client.rs
@proposal n=4 detector=D-retired severity=warning file=span-trace-coverage anchor=fct-kohas2np dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 4. Span / Trace Coverage
change: replace the item's closing sentence with «The peer pid and start-time comparison runs on Linux, the one target; `server-verify-failed` covers a refused euid, a mismatch and a check that cannot be made, with no OS carve-out.»; the rest stands verbatim
sidecar: span-trace-coverage — the hook server check's Linux-and-Windows comparison and macOS carve-out retired
rationale: same retirement as the cli server check: the macOS no-peer-pid carve-out and the Windows arm leave with their code (Retired; Symbols / APIs viola-state liveness)
@proposal n=5 detector=D-retired severity=warning file=span-trace-coverage anchor=fct-ts6ky5ry dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 4. Span / Trace Coverage
change: replace the item's closing sentence with «The peer pid and start-time comparison runs on Linux, the one target; `server-verify-failed` covers a refused euid, a mismatch and a check that cannot be made, with no OS carve-out.»; the rest stands verbatim
sidecar: span-trace-coverage — the MCP server check's Linux-and-Windows comparison and macOS carve-out retired
rationale: same retirement as the cli server check; viola-mcp tools.rs loses its SERVER_PID_REPORTED argument (Symbols / APIs root bin)
@proposal n=6 detector=D-retired severity=warning file=span-trace-coverage anchor=fct-e6vtcesf basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 4. Span / Trace Coverage › #### Scenario: `viola run` start sequence to child spawn
change: the span chain reads `run.pin_copy` › `run.version_gate`, the «› `run.conpty_sideload` (Windows)» link dropped, and the settings override write loses its «, Unix only» qualifier; the rest stands verbatim
sidecar: span-trace-coverage — run start chain loses run.conpty_sideload (ConPTY sideload retired)
rationale: the Retired bullet retires the ConPTY sideload with run.conpty_sideload, and Symbols / APIs removes src/cmd/run.rs's run.conpty_sideload step; the Linux-only target leaves no non-Unix start to qualify
@proposal n=7 detector=D-retired severity=warning file=span-trace-coverage anchor=fct-smfiay44 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 4. Span / Trace Coverage › #### Scenario: `viola run` start sequence to child spawn
change: `run.pin_copy` keeps `outcome` (`ok|pinned-hash-mismatch`) and drops its `.cmd`/`.bat` (`batch-script-child`) sentence; the `run.conpty_sideload` (Windows) row is dropped; `channel.bind`: `endpoint_kind` (`unix-socket`); `pty.spawn`: `pty_backend` (`openpty`, from `viola_pty::pty_backend()`), `env_stripped_count`; the other rows stand
sidecar: span-trace-coverage — run start span attributes lose batch-script-child, run.conpty_sideload, named-pipe and the two conpty backends
rationale: Symbols / APIs removes AgentError::BatchScriptChild, the run.conpty_sideload step and the conpty words of pty_backend() (now always openpty); the Retired bullet retires the named-pipe endpoint
@proposal n=8 detector=D-retired severity=warning file=span-trace-coverage anchor=fct-4g2at3oh basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-wrfywvkp
section: ## 4. Span / Trace Coverage › #### Scenario: `viola run` start sequence to child spawn
change: the failure `detail` set reads `already-live|stale-heartbeat|squatted-name|pinned-hash-mismatch|socket-dir-refused`; the success line ends at `env_kept}` (D-34), its Windows `sideload_fallback` clause and the degraded-sideload sentence dropped; the schema item reads «In `schemas/diag-line.v1.json`, `pty_backend` is `{ "type": "string" }` and `os` is closed to `linux`.», its sideload sentences dropped; the rest stands verbatim
sidecar: span-trace-coverage — run start log fields lose batch-script-child and sideload_fallback; os reads linux (inputs#I2)
rationale: Schema / config: os reads ["linux"], sideload_fallback leaves process-start and batch-script-child leaves the process-exit detail enum, closed words leaving with their emitter by the overseer's answer inputs#I2
@proposal n=9 detector=D-retired severity=warning file=span-trace-coverage anchor=fct-lohdixpq dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 4. Span / Trace Coverage › #### Scenario: Confirmed `send` (CL-1) from driver to readback
change: the server-check item's closing parenthetical reads «(the peer pid and its start time, compared on Linux, the one target, with no OS carve-out)»; the rest stands verbatim
sidecar: span-trace-coverage — send server check drops the Linux/Windows/macOS split and the macOS carve-out
rationale: the Retired bullet retires the macOS no-peer-pid carve-out and the Windows arm with their code
@proposal n=10 detector=D-retired severity=warning file=span-trace-coverage anchor=fct-gghzntp4 dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 4. Span / Trace Coverage › #### Scenario: Confirmed `send` (CL-1) from driver to readback
change: the client-side server-verification refusal item drops «(Linux, Windows and macOS, on macOS an euid mismatch alone)»; the rest stands verbatim
sidecar: span-trace-coverage — send's server-verification refusal line drops its OS list and macOS reading
rationale: the Retired bullet retires the macOS carve-out and the Windows code; the refusal itself stands on Linux
@proposal n=11 detector=D-retired severity=warning file=span-trace-coverage anchor=fct-u2zbdxyd dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 4. Span / Trace Coverage › #### Scenario: Confirmed `send` (CL-1) from driver to readback
change: the exit-21 `server-verify-failed` clause of the Cleanup item drops «Linux, Windows and macOS, on macOS an euid mismatch alone», keeping «no frame sent»; the rest stands verbatim
sidecar: span-trace-coverage — send cleanup's server-verify-failed cause drops its OS list and macOS reading
rationale: the Retired bullet retires the macOS carve-out and the Windows code; exit 21's six causes stand (Outcome exit codes)
@proposal n=12 detector=D-retired severity=warning file=span-trace-coverage anchor=fct-l4ierws2 dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 4. Span / Trace Coverage › #### Scenario: `wait` / `last` event-driven readback
change: the server-check sub-item drops its closing parenthetical «(the peer pid and its start time on Linux and Windows; … its named carve-out)»; the rest stands verbatim
sidecar: span-trace-coverage — wait/last server check drops the Linux/Windows/macOS split and the macOS carve-out
rationale: the Retired bullet retires the macOS no-peer-pid carve-out and the Windows arm with their code
@proposal n=13 detector=D-retired severity=warning file=span-trace-coverage anchor=fct-hluu43pb dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 4. Span / Trace Coverage › #### Scenario: `wait` / `last` event-driven readback
change: the client `process-exit` item drops «(Linux, Windows and macOS, on macOS an euid mismatch alone)» from its `server-verify-failed` clause; the rest stands verbatim
sidecar: span-trace-coverage — wait/last process-exit server-verify-failed drops its OS list and macOS reading
rationale: the Retired bullet retires the macOS carve-out and the Windows code
@proposal n=14 detector=D-retired severity=warning file=span-trace-coverage anchor=fct-u5crvqku dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 4. Span / Trace Coverage › #### Scenario: Dialog → `answer` (question / permission / plan), including the unverified-CLI branch
change: the instance-and-server-check sub-item ends «… and `answer` sends no frame.», its parenthetical «(the peer pid and its start time on Linux and Windows; … its named carve-out)» dropped; the rest stands verbatim
sidecar: span-trace-coverage — dialog/answer server check drops the Linux/Windows/macOS split and the macOS carve-out
rationale: the Retired bullet retires the macOS no-peer-pid carve-out and the Windows arm with their code
@proposal n=15 detector=D-retired severity=warning file=span-trace-coverage anchor=fct-a7vqqg3h dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 4. Span / Trace Coverage › #### Scenario: Human takes the wheel, automation is refused, `release` returns it
change: the pause/release sub-item ends «… opens no `channel.request` and sends no frame.», its parenthetical «(the peer pid and its start time on Linux and Windows; … its named carve-out)» dropped; the rest stands verbatim
sidecar: span-trace-coverage — pause/release server check drops the Linux/Windows/macOS split and the macOS carve-out
rationale: the Retired bullet retires the macOS no-peer-pid carve-out and the Windows arm with their code
@proposal n=16 detector=D-retired severity=warning file=span-trace-coverage anchor=fct-gqw4imf3 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 4. Span / Trace Coverage › #### Scenario: Budget governor
change: the statusline-shell item reads «`process-start` / `process-exit{subject:"statusline-shell", shell_exit_status, duration_ms}`, only when the snapshot records a `statusline_command`;» — the clause «and a shell is built (never on Windows, where `shell_argv` is `None`)» dropped; the rest stands verbatim
sidecar: span-trace-coverage — statusline-shell pair no longer conditioned on shell_argv being None on Windows
rationale: Symbols / APIs: statusline::shell_argv returns [&str; 3] (was Option), WRAPPED_HERE folded, so a shell is built on every build
@proposal n=17 detector=D-retired severity=warning file=span-trace-coverage anchor=fct-6xa7lf3f basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 4. Span / Trace Coverage › #### Scenario: Budget governor
change: the strip sub-item reads «The strip removes every inherited `CLAUDE*` name, matched exactly, except the persistent set (`config.json` `claude_env_keep`), and always removes the 11 names of the identity floor `IDENTITY_FLOOR`, first measured on a Windows host, whatever the persistent set says (…as now…); … no value is read by the strip or any log, and no env map is passed to a span or event (D-34).» — the Windows case-insensitive fold, the `HKCU`/`HKLM` `Environment` persistent set and «the registry reader» dropped
sidecar: span-trace-coverage — E2 strip matches names exactly; the Windows registry persistent set and case fold retired
rationale: Symbols / APIs removes src/run/env.rs's registry reader (HKCU\Environment) and case-insensitive match and CASE_INSENSITIVE; the exact strip, claude_env_keep and the eleven-name identity floor stay
@proposal n=18 detector=D-retired severity=warning file=span-trace-coverage anchor=fct-f3gi2vhc basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 4. Span / Trace Coverage › #### Scenario: Budget governor
change: the stop clause reads «(`SIGINT` / `SIGTERM`)»; `MM` reads «the relied count (`LedgerRow::relied()`, all 24 rows of `ALL` in its order on every build)», the `WRAPPED_HERE` rule and «24 on Unix, 21 on Windows, … relied on every OS» dropped; «with one probe `--settings` on Unix» → «with one probe `--settings`»; «SIGKILL, a crash and a Windows close, logoff or shutdown leave it» → «SIGKILL and a crash leave it»; the statusline capture arm drops «, Unix only»; the rest stands verbatim
sidecar: span-trace-coverage — verify item: console-control stop and the Windows relied set of 21 retired; relied() is all 24 rows
rationale: Symbols / APIs removes verify/stop.rs's console-control arm (SIGINT / SIGTERM unconditional), folds WRAPPED_HERE (LedgerRow::relied() all twenty-four rows on every build) and removes the Windows relied count; Retired lists the Windows relied set of twenty-one
@proposal n=19 detector=D-retired severity=warning file=metric-coverage anchor=fct-l3f7hrdu dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 5. Metric Coverage
change: the CLI's server-verification refusal clause drops «(Linux, Windows and macOS, where it means an euid mismatch alone)»; the rest stands verbatim
sidecar: metric-coverage — viola.wait.outcome item's server-verification refusal drops its OS list and macOS reading
rationale: the Retired bullet retires the macOS carve-out and the Windows code; the refusal and its Counter grouping stand on Linux
@proposal n=20 detector=D-retired severity=warning file=body anchor=fct-q67chfay dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 4. Span / Trace Coverage › #### Scenario: `wait` / `last` event-driven readback
change: «a serving process that is not the recorded wrapper or runs under another euid ends a CLI `wait` / `last` before any frame, …» — «(Linux and Windows)», «(Linux and macOS)» and «(on macOS an euid mismatch alone, since macOS compares no peer pid, its named carve-out)» dropped; the rest stands verbatim
sidecar: obs-plan §4 wait/last broad line drops the per-OS comparison split and the macOS carve-out
rationale: the Retired bullet retires the macOS no-peer-pid carve-out and the Windows arm; liveness::server_verdict loses peer_pid_reported and the macOS Matches arm (Symbols / APIs)
@proposal n=21 detector=D-retired severity=warning file=body anchor=fct-itztmsse basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 4. Span / Trace Coverage › #### Scenario: `viola run` start sequence to child spawn
change: «Never the endpoint name, the `pinned_bin` path or env values in a span; the child's `--settings` argument, the statusline source path and the command appear in no line or span; a failure's `detail` carries no path or pid; the child's `process-exit` comes from the handle-wait thread, never on reader EOF.» — the sideload companions' clause and «a sideload outcome is never a refusal or an exit change» dropped
sidecar: obs-plan §4 run start broad line drops the retired ConPTY sideload rules
rationale: the Retired bullet retires the ConPTY sideload (sideload_fallback, run.conpty_sideload, the vendored companions); the P3 smoke reads process-start with no sideload_fallback (Outcome obs)
@remark
not mine: §1 body fct-t2iihkln/fct-m24p7svx/fct-zce67g4u/fct-p4bwzizt (obs-scope-summary, task F1) and §6 body fct-3epowryi plus span-trace#4.4's exit_source kill-fallback — the report removes the TerminateProcess kill fallback but states no exit_source word change, so log-coverage owns whether kill-fallback still has an emitter; endpoint_kind's schema words are not stated by the report
