@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - For warnings and gotchas, see `.claude/docs/gotchas.md`
@leaf_read leaf=.claude/docs/gotchas.md lines=149
last: - For architectural decisions and their rationale, see `doc:a
@leaf_read leaf=.claude/docs/services/viola-agent-claude.md lines=47
last: - `doc:architecture` [CLI Version Compatibility] · `.claude/
@leaf_edit n=1 leaf=.claude/docs/conventions.md line=20 class=stale
leaf-text: `ts` on every event/frame, `*_at` for other instants
source: dec-u4y5h6gf «Every instance event, and the `hook.event` frame, carries its own instant in the field `ts`; request frames carry no instant, process-log and detail lines carry `timestamp`»
replace-with: `ts` on every instance event and the `hook.event` frame (request frames carry no instant), `timestamp` on process-log and detail lines, `*_at` for other instants (a link record's start instant excepted)
@leaf_edit n=2 leaf=.claude/docs/conventions.md line=13 class=omission
leaf-text: Two of the ratified exceptions are the test seams
source: dec-u6qgqlqe «`NO_COLOR`, `TERM`, `COLORTERM`, read only for SGR bytes; `PATH`, read and set for the child; `XDG_RUNTIME_DIR`, read only.»
replace-with: Besides `PATH` (read and set for the child) and `XDG_RUNTIME_DIR` (read only, for the socket directory), two of the ratified exceptions are the test seams
@leaf_edit n=3 leaf=.claude/docs/conventions.md line=33 class=omission
leaf-text: a failing row is no refusal and carries no code, and `verify` takes no `--json`
source: ctr-sxvauhzs «A relied ledger row failed; the failing row's stamp is still written.»
replace-with: a failing row is no refusal and carries no code, yet `viola verify` still exits 1 on a failed relied row with that row's stamp written, and `verify` takes no `--json`
@leaf_edit n=4 leaf=.claude/docs/conventions.md line=33 class=omission
leaf-text: · 2 · 10 human-typing
source: ctr-tfupf2nf «Usage error, and viola send text over 16 MiB or not UTF-8; never produced by viola hook.»
replace-with: · 2 (usage error, and `viola send` text over 16 MiB or not UTF-8) · 10 human-typing
@leaf_edit n=5 leaf=.claude/docs/conventions.md line=45 class=omission
leaf-text: `claude_env_keep` (the R8 pass-list of `CLAUDE*` names, default empty)
source: ctr-szg56kho «The R8 pass-list of CLAUDE* names, an array of at most 32 names each matching ^CLAUDE[A-Z0-9_]*$, default empty; a bad value is rejected whole, and an identity-floor name in it has no effect.»
replace-with: `claude_env_keep` (the R8 pass-list of `CLAUDE*` names, an array of at most 32 names each matching `^CLAUDE[A-Z0-9_]*$`, default empty; a bad value is rejected whole, and an identity-floor name in it has no effect)
@leaf_edit n=6 leaf=.claude/docs/conventions.md line=45 class=omission
leaf-text: `diagnostics_level` (`info` by default, or `debug`; obs D-15)
source: ctr-szpb6rrw «"info" by default, or "debug"; diagnostics_level is the only source of the process-log level, and RUST_LOG has no effect.»
replace-with: `diagnostics_level` (`info` by default, or `debug`; the only source of the process-log level, `RUST_LOG` having no effect; obs D-15)
@leaf_edit n=7 leaf=.claude/docs/gotchas.md line=89 class=omission
leaf-text: `viola-agent-claude` removes the wrapper, then reverses tag escaping, before emitting `prompt-submitted`.
source: dec-x4r4ngxl «Prompt normalisation classifies harness on the raw prefix first, then removes any long-paste wrapper with its frame, then un-escapes a backslash after < before an ASCII letter or /.»
replace-with: `viola-agent-claude` classifies `harness` on the raw prefix first, then removes the wrapper with its frame, then reverses tag escaping, before emitting `prompt-submitted`.
@leaf_edit n=8 leaf=.claude/docs/services/viola-agent-claude.md line=6 class=omission
leaf-text: No adapter trait yet: a second agent arrives as a sibling `viola-agent-<name>` crate.
source: dec-yarxguwe «A second driven agent arrives as a sibling viola-agent crate, at which point the adapter trait is extracted into viola-core.»
replace-with: No adapter trait yet: a second agent arrives as a sibling `viola-agent-<name>` crate, at which point the adapter trait is extracted into `viola-core`.
