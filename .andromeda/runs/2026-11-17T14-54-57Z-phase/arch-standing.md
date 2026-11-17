# arch · standing conventions

- S1 · each new or changed test passes under its CI kill-line bound and leaves no process behind its test · occupied-resources
  decision/e2e-no-rmcp-child-transport · E2E owns the viola mcp child — The test-only E2E crate takes rmcp client alone, never transport-child-process, and owns the viola mcp child and awaits its exit. · why: transport-child-process kills the server 3 s after a close
  decision/root-tests-freeze-wrappers-with-one-guard · Root tests freeze a wrapper through one SIGSTOP guard — Root tests make a wrapper read stale by stopping it with SIGSTOP through one shared guard, which waits until the process state reads stopped and sends SIGCONT on drop. · why: The list-row and instance-state tests rely on it for a stale wrapper whose pid and start time still match while its heartbeat ages.
- S2 · each rig arm or reading writes closed words only in a record, never raw upstream text or secrets · cross-cutting-patterns
  contract/mcp-stdin-oversize/parse-rejected · MCP stdin oversize process log · parse-rejected — A session ended by an oversize line writes exactly one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` line at WARN, holding no byte of the line.
  decision/codes-only-role-logs · Codes-only role logs — Home-level role logs hold codes only, with no row text, prompt, id, path or endpoint; content-bearing detail goes only to the instance's owner-only detail files.
- S3 · each new code line passes formatting and clippy with warnings denied, no print macros, no orphan modules · build-system
  decision/clippy-print-and-macro-bans · clippy deny warnings with print and dbg bans — clippy runs with warnings denied, the workspace bans on print_stdout, print_stderr and dbg_macro, and disallowed tracing level macros.
  decision/fmt-and-clippy-gate · Formatting and clippy with warnings denied — Code passes the cargo fmt check and clippy over the whole workspace, all targets and the fake-agent feature, with every warning denied.
  decision/no-orphan-modules · No orphan modules — No lib or bin target holds an orphan module; the check is a CI gate and is proven to fire on planted orphans. · guard: scripts/orphans-check.sh
  decision/no-print-macros · No print macros — The workspace lints deny `print_stdout`, `print_stderr` and `dbg_macro`, and every product member inherits them. · guard: tests/contract_lints.rs · except: viola-e2e denies only `dbg_macro`, because the harness prints its JSON document; the fake agent carries a crate-level print allow.
  decision/no-tracing-level-macros · Tracing level macros banned — The tracing info, warn, error, debug and trace macros are disallowed in the code. · guard: clippy disallowed-macros, checked by the lint probes
