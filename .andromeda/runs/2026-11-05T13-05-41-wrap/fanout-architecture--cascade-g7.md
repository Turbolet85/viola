@proposal n=1 detector=D-cascade severity=warning file=cross-cutting-patterns anchor=fct-k6htnrfj basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Cross-cutting Patterns
change: in the Settings line, `claude_env_keep` reads «the R8 pass-list of `CLAUDE*` names, default empty», the «Unix» qualifier dropped; the rest of the entity unchanged
sidecar: crosscut-config — claude_env_keep no longer qualified as the Unix R8 pass-list (Windows registry reader and case fold removed; Linux-only)
rationale: the report's Symbols / APIs removes src/run/env.rs's HKCU registry reader and case-insensitive match, the exact strip and claude_env_keep staying, so the pass-list has no non-Unix counterpart to be distinguished from
@proposal n=2 detector=D-retired severity=warning file=cross-cutting-patterns anchor=fct-wfge3rsz basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Cross-cutting Patterns
change: the relied-set clause reads «(`LedgerRow::relied()`: all 24 rows on every build; [CLI Version Compatibility])»; «relied on Unix only» (the three statusline rows) and each «relied on every OS» (resume-session, local-command-compact, agents-join) read «relied»
sidecar: crosscut-capability-ledger — the Windows relied set of twenty-one retired; LedgerRow::relied() is all twenty-four rows on every build
rationale: the report's Retired bullet names the Windows relied set of twenty-one and its Symbols / APIs folds WRAPPED_HERE so LedgerRow::relied() is all twenty-four rows on every build; Counts records relied-win twenty-one to none
@proposal n=3 detector=D-retired severity=warning file=cross-cutting-patterns anchor=fct-wfge3rsz basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: ## Cross-cutting Patterns
change: the five Windows-leg owners leave with the Windows claim they owe: drop «its Windows reading owed to "Windows-only live measurements"» (paste-newline, resume-session), «its Windows reading owed on "Windows-only live measurements"» (local-command-compact, agents-join) and «the Windows legs stay owed to "Windows-only live measurements" (the operator's direction at the 2026-10-10-statusline-pass-through wrap)»; each Linux stamp and live measurement kept
sidecar: crosscut-capability-ledger — the Windows readings owed to the retired Windows-only live measurements entry retired with the Windows code (tag multi-os-final)
rationale: the report's Retired bullet makes the OS target Linux only, the Windows code recoverable at multi-os-final, and records the Windows-only live measurements entry retired at the 2026-11-04T23-11-43-wrap, its sites re-pointed to origin 2026-11-04T23-11-43-wrap (target another-os) or retired with the Windows claim they owe (Expected amendments, wolm)
@proposal n=4 detector=D-new-detail severity=warning file=cross-cutting-patterns anchor=fct-xxlqtlbc fact=F3 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: ## Cross-cutting Patterns
change: add after the census sentence: «A gate whose predicate names `windows` or `target_os = "macos"` reads as a fault (`another OS: …`) even when a `GATES` row names it; `census_reads_a_gate_naming_another_os_as_a_fault_even_when_named` proves that class fires, its Windows planted cases building the word from a `WINDOWS` const.»
sidecar: crosscut-os-gated-tests — the census's another-OS fault class and its planted witness added (Linux-only)
rationale: the report's Harness / gate surface states the census reads a windows / macos gate as a fault even when a row names it and its self-check re-planted; New text lists the witness at tests/contract_os_gated_tests.rs:626-647; Outcome fct-6hdh6dws records it green with every planted class firing
@proposal n=5 detector=D-retired severity=warning file=body anchor=fct-fa2klbw5 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Cross-cutting Patterns
change: the heading «Cross-platform discipline» is re-worded to a portability discipline on a Linux-only product, and «Every OS-specific branch compiles and is tested on its CI runner» reads «the product builds and is tested on Linux (CI on `ubuntu-latest`); a gate naming another OS is a census fault»; the forward-slash, no-shell, direct-spawn, hashed-endpoint and census-reason sub-lines kept
sidecar: body Cross-cutting Patterns — the cross-platform discipline line re-worded to the Linux-only target; the per-OS-runner clause retired with the windows-2025 and macos-latest legs
rationale: the report's Retired bullet replaces the OS target with Linux only and retires the windows-2025 and macos-latest CI legs; Harness / gate surface keeps ci.yml's matrix at [ubuntu-latest] and makes a windows / macos gate a census fault
@remark
body fct-ao5mmzh3 (Local endpoint trust boundary: «whether a named pipe or a Unix socket file», «per OS») is stale under the retired Windows named pipe but no picked entity details it; crosscut-config's canonical_home `\\?\C:\` clause describes src/cmd/mod.rs, which the report does not list as changed, so it is left to the os-target sweep; the census self-check's «eight planted cases» count is not re-measured by the report
