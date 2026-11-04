@answer n=1 entity=input-validation#1.1 v=no
why: the session changed no validate_paste_text or send path code, and the report lists the paste-text validation as untouched
@answer n=2 entity=input-validation#1.2 v=no
why: the free-text rule shares validate_paste_text, which the session did not change, and the answer handler is not in the change
@answer n=3 entity=input-validation#1.3 v=no
why: the report lists channel framing, MAX_FRAME and the frame envelope as untouched, and the session moved no frame code
@answer n=4 entity=input-validation#1.4 v=no
why: the hook stdin cap, fail-open paths and capture arm are untouched, and the session changed no hook code, only test gate reasons
@answer n=5 entity=input-validation#1.5 v=no
why: the report lists the MCP inputs as untouched, and the session added no MCP handler or schema change
@answer n=6 entity=input-validation#1.6 v=no
why: the report lists the MCP stdin as untouched, and the session changed no viola-mcp line-cap code
@answer n=7 entity=input-validation#1.7 v=no
why: the clap parsers, home resolution and strict-modes gates are unchanged, and the session only adds gate-reason lines to test files
@answer n=8 entity=input-validation#1.8 v=no
why: the report lists config.json as untouched, and no config reader or threshold changed
@answer n=9 entity=input-validation#1.9 v=no
why: the registry name read in src/run/env.rs is unchanged, and the session touched no product environment code
@answer n=10 entity=input-validation#1.10 v=no
why: the task states no seam or env read was added, the seam rule in src/cmd/run.rs is unchanged, and only a test reader in tests/cli_fake_agent.rs moved
@answer n=11 entity=input-validation#1.11 v=no
why: the FAKE_AGENT_HOOK_PANIC seam and its panic_on rule in src/cmd/hook/seam.rs are unchanged, and the session added no seam
@answer n=12 entity=input-validation#1.12 v=no
why: the session adds an integration witness in tests/cross_list_rows.rs, but no colour read or escaping changed and the row's claim still holds
@answer n=13 entity=input-validation#1.13 v=no
why: the bounded readers, strict-modes read gates and budget read are unchanged, and the session added only gate-reason lines to test files
@answer n=14 entity=input-validation#1.14 v=no
why: the report lists the tailed paths as untouched, and no notify or tailing code changed
@answer n=15 entity=input-validation#1.15 v=no
why: the claude agents read and the --version probe are unchanged, and the session touched no spawn or parse code
@answer n=16 entity=input-validation#1.16 v=no
why: the report lists the vt100 feed as untouched, and the session added gate reasons and a different witness, not this feed or its catch_unwind rule
@answer n=17 entity=input-validation#1.17 v=no
why: the .cmd/.bat refusal is unchanged, verify_a_batch_script_refuses only gained a reason line and a census row, and resolve_program is not touched
@answer n=18 entity=input-validation#1.18 v=no
why: the report lists the session-id argv as untouched, and no revive or verify resume code moved
@answer n=19 entity=input-validation#1.19 v=no
why: the statusline source read in src/cmd/run.rs is unchanged, and the session touched no product statusline code
@answer n=20 entity=input-validation#1.20 v=no
why: the Unix-only shell-out rule and shell_argv are unchanged, and UNIX_STATUSLINE only names its tests in the census
@answer n=21 entity=input-validation#1.21 v=no
why: the sideload and pin code in viola-pty, src/conpty.rs and viola-state is unchanged, and the session touched no Windows loader code
@answer n=22 entity=input-validation#1.22 v=no
why: the session adds no product surface, listener or data class, so the no upload, webhook or database claim still holds
@answer n=23 entity=input-validation#2.0 v=no
why: the report lists MAX_FRAME and its consumers as untouched, and the session changed no cap or reader
