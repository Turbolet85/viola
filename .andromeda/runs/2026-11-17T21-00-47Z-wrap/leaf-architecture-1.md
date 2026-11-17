@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - For warnings and gotchas, see `.claude/docs/gotchas.md`
@leaf_read leaf=.claude/docs/services/viola-agent-claude.md lines=47
last: - `doc:architecture` [CLI Version Compatibility] · `.claude/
@leaf_read leaf=.claude/docs/services/viola-channel.md lines=45
last: - `doc:architecture` §Standard Contracts (Wrapper channel fr
@leaf_read leaf=.claude/docs/services/viola-core.md lines=43
last: - `.claude/docs/stack.md` · `.claude/docs/conventions.md` · `
@leaf_read leaf=.claude/docs/services/viola-mcp.md lines=39
last: - `doc:architecture` [MCP] · `.claude/rules/api.md`
@leaf_edit n=1 leaf=.claude/docs/conventions.md line=13 class=omission
leaf-text: `XDG_RUNTIME_DIR` (read only, for the socket directory)
source: dec-u6qgqlqe «Test seams `FAKE_AGENT_PUMP_DELAY_MS`, `FAKE_AGENT_HOOK_PANIC`; `NO_COLOR`, `TERM`, `COLORTERM` for SGR; `PATH`, read and set for the child; `XDG_RUNTIME_DIR`; verify's kept `CLAUDE_CONFIG_DIR`»
replace-with: `XDG_RUNTIME_DIR` (read only, for the socket directory) and `CLAUDE_CONFIG_DIR` (its value read by `viola verify` alone, only when `claude_env_keep` keeps that name, to locate the Claude config file)
@leaf_edit n=2 leaf=.claude/docs/conventions.md line=45 class=omission
leaf-text: and an identity-floor name in it has no effect), `dialog_hold_s`
source: ctr-szg56kho «Keeping CLAUDE_CONFIG_DIR makes viola verify read its value to locate the Claude config file.»
replace-with: and an identity-floor name in it has no effect; keeping `CLAUDE_CONFIG_DIR` makes `viola verify` read its value to locate the Claude config file), `dialog_hold_s`
@leaf_edit n=3 leaf=.claude/docs/conventions.md line=54 class=omission
leaf-text: (except the test-only live rig in `viola-e2e`, which reads the key names and the source word of its own captured UserPromptSubmit payload, never a value)
source: dec-uun4jqhm «Test-only: the live rig reads the key names and source word of its own captured UserPromptSubmit payload, never a value; the fake agent and the rig read a Claude config file's trust record.»
replace-with: (except, test-only, the live rig in `viola-e2e`, which reads the key names and the source word of its own captured UserPromptSubmit payload, never a value, and the fake agent and the rig, which read a Claude config file's trust record)
@leaf_edit n=4 leaf=.claude/docs/services/viola-agent-claude.md line=6 class=omission
leaf-text: (one test-only exception: the live rig in `viola-e2e` reads the key names and the source word of its own captured UserPromptSubmit payload, never a value)
source: dec-zypgxbrh «Test-only: the live rig reads the key names and source word of its own captured UserPromptSubmit payload, never a value; the fake agent and the rig read a Claude config file's trust record.»
replace-with: (test-only exceptions: the live rig in `viola-e2e` reads the key names and the source word of its own captured UserPromptSubmit payload, never a value; the fake agent and the rig read a Claude config file's trust record)
@leaf_edit n=5 leaf=.claude/docs/services/viola-agent-claude.md line=11 class=stale
leaf-text: `viola verify`'s five interactive runs feed their own `Screen` the same way
source: dec-uczlwirj «viola verify makes one print-mode probe, six interactive PTY runs under the R8 strip at a fixed 80×24, and one agents --json read inside Run B, pasting only compiled texts.»
replace-with: `viola verify`'s six interactive runs feed their own `Screen` the same way
@leaf_edit n=6 leaf=.claude/docs/services/viola-agent-claude.md line=26 class=stale
leaf-text: twenty-two closed rows
source: dec-f67jf527 «The trust-record relied ledger row asks, per CLI version, whether a trust record written for an untrusted folder opens it with no modal; verify's Run F, after Run E, passes it only when the record was written, Run A showed the trust modal, and Run F's start shows no modal and the input box.»
replace-with: twenty-three closed rows
@leaf_edit n=7 leaf=.claude/docs/services/viola-agent-claude.md line=26 class=omission
leaf-text: `local-command-compact` · `local-command-echo`; the relied set `LedgerRow::relied()` (every row of `ALL`, in its order, all twenty-two)
source: dec-f67jf527 «The trust-record relied ledger row asks, per CLI version, whether a trust record written for an untrusted folder opens it with no modal; verify's Run F, after Run E, passes it only when the record was written, Run A showed the trust modal, and Run F's start shows no modal and the input box.»
replace-with: `local-command-compact` · `local-command-echo` · `trust-record` (probed by `viola verify`'s Run F, after Run E: passes only when the trust record was written, Run A showed the trust modal, and Run F's start shows no modal and the input box); the relied set `LedgerRow::relied()` (every row of `ALL`, in its order, all twenty-three)
@leaf_edit n=8 leaf=.claude/docs/services/viola-agent-claude.md line=26 class=stale
leaf-text: all twenty-two on every build (`local-command-echo` among them
source: dec-f67jf527 «The trust-record relied ledger row asks, per CLI version, whether a trust record written for an untrusted folder opens it with no modal; verify's Run F, after Run E, passes it only when the record was written, Run A showed the trust modal, and Run F's start shows no modal and the input box.»
replace-with: all twenty-three on every build (`local-command-echo` and `trust-record` among them
@leaf_edit n=9 leaf=.claude/docs/services/viola-agent-claude.md line=28 class=omission
leaf-text: Logged as `env_stripped_count` + `env_stripped_known` + `env_kept` — never values.
source: dec-wjypwipg «viola verify reads the value of CLAUDE_CONFIG_DIR, only when claude_env_keep keeps that name, to locate the Claude config file.»
replace-with: Logged as `env_stripped_count` + `env_stripped_known` + `env_kept` — never values; only `viola verify` reads the value of `CLAUDE_CONFIG_DIR`, only when `claude_env_keep` keeps that name, to locate the Claude config file.
