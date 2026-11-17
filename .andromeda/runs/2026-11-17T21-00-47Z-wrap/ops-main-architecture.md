@row_set n=1 row=decision/env-var-viola-prefix cell=except seen=1 detector=none
value: Test seams `FAKE_AGENT_PUMP_DELAY_MS`, `FAKE_AGENT_HOOK_PANIC`; `NO_COLOR`, `TERM`, `COLORTERM` for SGR; `PATH`, read and set for the child; `XDG_RUNTIME_DIR`; verify's kept `CLAUDE_CONFIG_DIR`
basis: fct-v6anf47h
because: verify now reads CLAUDE_CONFIG_DIR's value when claude_env_keep keeps it, a variable without the VIOLA_ prefix, and the proposer's shorter value dropped PATH being read.
