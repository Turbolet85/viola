# Classified — security · input-validation.b1

## Answers
input-validation#1.1 · no · paste-text validation of channel, MCP and CLI send text; the chunk changes no send path (the task's Not needed names paste-text validation untouched)
input-validation#1.2 · no · dialog free-text validation in answer; no answer path changes (the task's Not needed names dialog free-text validation untouched)
input-validation#1.3 · no · channel-frame size, depth, version and param rules; no frame path changes (the task's Not needed names channel-frame validation untouched)
input-validation#1.4 · no · hook stdin, instance and capture-arm rules live in src/cmd/hook.rs, which the chunk does not reach; the Needs' fail-open line forbids only a hook-side refusal, which the fix does not add
input-validation#1.5 · no · MCP tool input validation; the viola mcp tools are untouched (the task's Not needed names them)
input-validation#1.6 · yes · check_home_tree (the home, ledger/, bin/, plugin/ and the pinned copy's dirs) must run before the pin and before any log for run's and revive's start (through checked_exe) and for verify, the placement the Needs fix for the edited pin calls
input-validation#1.7 · no · config.json reads at start (thresholds, dialog hold, claude_env_keep, diagnostics_level) are untouched (the task's Not needed names the budget gate and dialog hold) and are other units of run.rs that must only keep holding
input-validation#1.8 · no · Windows registry value-name reads for the R8 pass-list in src/run/env.rs, which the chunk does not reach (the task's Not needed names R8 untouched)
input-validation#1.9 · no · FAKE_AGENT_PUMP_DELAY_MS is a fake-agent-only read in run.rs's hold_pump_start, another unit of a file the chunk edits by units; the witness names no pump delay
input-validation#1.10 · no · FAKE_AGENT_HOOK_PANIC is a fake-agent-only read in src/cmd/hook/seam.rs, which the chunk does not reach; the witness names no hook panic
input-validation#1.11 · no · terminal colour facts are read only by viola list's human mode, which the task's Not needed names untouched
input-validation#1.12 · yes · the Needs keep ledger/stamps.json read and written strictly (verify's pin_exe sits between its home-tree check and its stamps write) and snapshot.json's pinned_bin written only by the instance's own wrapper, both over the edited pin calls
input-validation#1.13 · no · notify tailing of events.ndjson is not reached (the chunk tails no path)
input-validation#1.14 · no · claude agents --json parse, the --version probe and verify's PTY output are untouched (the task's Not needed names them, and only verify's pin_exe call is reached)
input-validation#1.15 · no · the vt100 tee, feed thread and screen model in src/run/gate.rs and viola-agent-claude are not reached
input-validation#1.16 · no · child program resolution (resolve_program, the claude cwd, the .cmd and .bat refusal) is untouched; the pin writes the viola copy and resolves no child, and the start's cwd write is another unit that must only keep holding
input-validation#1.17 · no · the session-id argv of revive's --id and of verify's Run E is untouched (the task's Not needed names revive's session-id argv)
input-validation#1.18 · no · the statusline source read at start is untouched (the task's Not needed names it), and statusline_command's record is another unit of run.rs
input-validation#1.19 · no · the statusline shell-out is untouched (the task's Not needed names it) and src/cmd/hook/statusline.rs is not reached
