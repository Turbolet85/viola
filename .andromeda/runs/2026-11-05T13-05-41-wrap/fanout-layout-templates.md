@proposal n=1 detector=D-design-cli-wording severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: Component — Primary content block 2: refusal lines and the `unable` column
change: In the list of `viola verify`'s exit-1 `unable:`/`hint:` pairs (fct-7zli33yx), drop the pair `the claude CLI is a .cmd or .bat script` / `pass the real executable, not a .cmd or .bat shim`; the remaining pairs (`the claude CLI was not found`, `the CLI version could not be read`, the fixture, stop and home pairs) stand unchanged.
sidecar: 2026-11-05-linux-only-code-and-ci — the `.cmd`/`.bat` refusal pair leaves verify's exit-1 refusals with `AgentError::BatchScriptChild` (Linux-only, founder rulings 2026-11-04, inputs#I1, inputs#I5 item 6).
rationale: The report's Retired bullet (fct-tulramlm) retires the `.cmd`/`.bat` refusal and its human pair «the claude CLI is a .cmd or .bat script» / «pass the real executable, not a .cmd or .bat shim»; Symbols / APIs (fct-xxutheix) removes `AgentError::BatchScriptChild` and `src/cmd/verify.rs`'s `BatchScriptChild` arm, so the master still states an output wording no build prints.
@proposal n=2 detector=D-design-cli-wording severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu dependent_of=D-design-cli-wording
section: Output structure — `viola revive`
change: In the `--json` start-refusal sentence (fct-i322ygxm), `<code>` is `pinned-hash-mismatch`, `squatted-name`, `socket-dir-refused` (without `--json` the socket-directory pair of Primary content block 2), or `internal-error` for a program not found and for an error of the start itself — `batch-script-child` removed from the list.
sidecar: 2026-11-05-linux-only-code-and-ci — `batch-script-child` leaves revive's `--json` start-refusal codes with its emitter (closed codes 11 -> 10; inputs#I2).
rationale: Report Retired (fct-tulramlm) names `batch-script-child` among the closed words that left with their code (inputs#I2); Counts (fct-tjt6dmfu) moves revive's closed `--json` refusal codes 11 -> 10 with `batch-script-child` gone; Symbols / APIs (fct-xxutheix) removes `src/cmd/run.rs`'s `BatchScriptChild` start refusal that `revive` shares.
@proposal n=3 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: Surface: cli
change: In the Tooling context (fct-vnzdqtob), drop the sentence «Windows VT output is enabled with windows-sys 0.61.2 `SetConsoleMode`, and colour is off if that fails.»; the rest (clap, std `IsTerminal`, the hand-written SGR module, fixed-width padding) stands.
sidecar: 2026-11-05-linux-only-code-and-ci — the Windows VT enable and the windows-sys dependency leave the cli tooling context (Linux-only; `src/human/style.rs` loses the `vt` fact).
rationale: Report Retired (fct-tulramlm) retires the Windows VT enable (decision rung 7) and the `windows-sys` dependency; Symbols / APIs (fct-xxutheix) removes `src/human/style.rs`'s VT enable and `vt` fact; Dependencies (fct-www7rgz5) removes `windows-sys`.
@proposal n=4 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix dependent_of=D-retired
section: Output structure — `viola list` (piped / `NO_COLOR` / `TERM=dumb`)
change: In the As built paragraph (fct-mdoqfj5o), the uncoloured conditions read «under `--json`, a piped stdout, `NO_COLOR` (an empty value included) or `TERM=dumb`» — «or a failed Windows VT enable» removed.
sidecar: 2026-11-05-linux-only-code-and-ci — the failed-Windows-VT-enable colour-off condition leaves `viola list`'s As built (Linux-only).
rationale: Report Retired (fct-tulramlm) retires the Windows VT enable; Symbols / APIs (fct-xxutheix) removes the `vt` fact and the `vt_enable_failed` row of `src/human/style.rs`, rungs 1–6 unchanged; Outcome (fct-35ftxr5n) keeps Linux human output byte-identical.
@proposal n=5 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: Output structure — `viola verify`
change: In the Usage paragraph (fct-pf5uzbca), `MM` is the relied count, `LedgerRow::relied()`'s length: all 24 rows on every build, ending at `local-command-compact`; it grows as owning chunks land their rows — the «24 on Unix, 21 on Windows, where the relied set skips the three statusline rows because the status line is not wrapped there» clause removed.
sidecar: 2026-11-05-linux-only-code-and-ci — the Windows relied set of twenty-one leaves; `LedgerRow::relied()` is all twenty-four rows on every build.
rationale: Report Retired (fct-tulramlm) retires the Windows relied set of twenty-one; Symbols / APIs (fct-xxutheix) folds `WRAPPED_HERE` so `LedgerRow::relied()` is all twenty-four rows on every build; Counts (fct-tjt6dmfu) relied set Windows twenty-one -> none; Outcome (fct-2n5dws2q) `[NN/24]` on Linux.
@proposal n=6 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu dependent_of=D-retired
section: Output structure — `viola verify`
change: In the wireframe (fct-hk2gtsxu), the first command's annotation reads «(24 relied rows)» in place of «(a Unix host: 24 relied rows)»; the step lines and summary stand.
sidecar: 2026-11-05-linux-only-code-and-ci — the verify wireframe no longer qualifies its 24 relied rows by host OS.
rationale: Report Counts (fct-tjt6dmfu): `LedgerRow::relied()` all twenty-four on every build, the Windows relied set gone; a per-host qualifier on a Linux-only product implies a host where the count differs (Retired, fct-tulramlm).
@proposal n=7 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: Output structure — `viola verify`
change: In the Usage paragraph (fct-pf5uzbca), a run stopped by Ctrl-C or TERM (`SIGINT` / `SIGTERM`) before its rows are checked prints only that plain ASCII pair — «Unix» and «Windows Ctrl-C / Ctrl-Break» removed.
sidecar: 2026-11-05-linux-only-code-and-ci — verify's console-control stop arm leaves; `SIGINT` / `SIGTERM` the one stop path.
rationale: Report Retired (fct-tulramlm) retires the console-control stop arm; Symbols / APIs (fct-xxutheix) removes `src/cmd/verify/stop.rs`'s console-control arm (`CTRL_C_EVENT` / `CTRL_BREAK_EVENT`), `SIGINT` / `SIGTERM` unconditional.
@proposal n=8 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: Component — Primary content block 2: refusal lines and the `unable` column
change: In the Hint line bullet (fct-qhojlk2s), the server-check sentence reads «A frame verb (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`) whose server check finds that the process on the endpoint is not the recorded wrapper, or cannot make the check, …» — «On Linux, Windows and macOS (on macOS an euid mismatch alone, the pid and start-time comparison absent, the named carve-out, owned by no route entry)» removed; the exit 21, hint and `server-verify-failed` document stand.
sidecar: 2026-11-05-linux-only-code-and-ci — the server-check hint drops its three-OS scope and the macOS no-peer-pid carve-out (Linux-only).
rationale: Report Retired (fct-tulramlm) retires the OS target «Windows first, macOS and Linux» and the macOS no-peer-pid carve-out; Symbols / APIs (fct-xxutheix) removes `SERVER_PID_REPORTED` / `PEER_EUID_REPORTED` and the macOS `Matches` arm of `liveness::server_verdict`.
@proposal n=9 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: Output structure — `viola --help`
change: In the As built paragraph (fct-4rbi2th6), the sentence reads «The usage line is the override literal.» — «on every OS» dropped.
sidecar: 2026-11-05-linux-only-code-and-ci — an «on every OS» clause re-worded on the Linux-only product, the claim kept.
rationale: Report Retired (fct-tulramlm) makes the OS target Linux only; Counts (fct-tjt6dmfu) `os-target` names an «on every OS» clause stale wording on a Linux-only product, re-worded or dropped with the claim kept.
@proposal n=10 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: Output structure — `viola --help`
change: In fct-33x2qanb, `viola <verb> --help` gives clap's per-verb usage, unchanged; its usage line names the program by its executable — «, `viola.exe` on Windows» removed; the stdin / `--file` sentence stands.
sidecar: 2026-11-05-linux-only-code-and-ci — the Windows `viola.exe` usage-line note leaves (Linux-only).
rationale: Report Retired (fct-tulramlm): the OS target is Linux only, the Windows code removed (recoverable at `multi-os-final`); no Windows executable name is printed by any build.
@proposal n=11 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: Output structure — `viola plugin install`
change: In fct-bievnfhg, `<dir>` is printed bare unless the shell would split it: single-quoted unless every character is an ASCII letter, a digit or one of `_ - . / :` — the «Unix:» label and the «Windows: double-quoted when it holds a space» arm removed.
sidecar: 2026-11-05-linux-only-code-and-ci — the Windows quoting arm of plugin install's printed path leaves (Linux-only).
rationale: Report Retired (fct-tulramlm): the OS target is Linux only and the Windows code removed; Expected amendments (fct-pdxuthnr) list layout-templates §Output structure `viola plugin install` among the os-target sites.
@remark
The `on Unix` qualifiers of the socket-directory refusals in fct-7zli33yx and fct-qhojlk2s remain true on Linux and were left unproposed; the orchestrator may drop them as wording polish.
