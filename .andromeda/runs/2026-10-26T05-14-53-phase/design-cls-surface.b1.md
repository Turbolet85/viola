# Classified — design · surface.b1

## Answers
surface#1.1 · no · a CLI colour token; no reached file or unit prints colour, so it is a re-check every change owes
surface#1.2 · no · a CLI colour token for a stale row; no reached file or unit prints colour, so it is a re-check every change owes
surface#1.3 · no · a CLI colour token for the NAME column; no reached file or unit prints colour, so it is a re-check every change owes
surface#1.4 · no · the CLI's SGR reset after styled spans; no reached file or unit prints colour, so it is a re-check every change owes
surface#1.5 · no · a CLI colour rule (no blue SGR); no reached file or unit prints colour, so it is a re-check every change owes
surface#1.6 · no · a CLI colour rule (no background SGR); no reached file or unit prints colour, so it is a re-check every change owes
surface#2.1 · no · the CLI's stderr line layout for context, refusal and error lines; no reached file or unit prints stderr, so it is a re-check every change owes
surface#3.1 · no · the CLI table's width truncation read by viola_pty::host_size(); the reached files name no viola-pty or list unit, so it is a re-check every change owes
surface#4.1 · no · the `viola list` board rendering (STATUS, WHEEL, DIALOG, unwrapped rows); list reads no channel and no reached unit renders the board, so it is a re-check every change owes
surface#4.2 · no · list's exit-21 strict-modes refusal of the whole board, a CLI-only path with no channel code; no reached unit renders it, so it is a re-check every change owes
surface#5.1 · yes · its refusal keys (`human-typing`, `budget-paused`, `unverified-cli`, the `not-delivered` details, the `instance-unreachable` causes) are refusal codes the Tokio client carries to `viola mcp`, which the scope keeps unchanged; its hint lines and start and revive refusal lines are CLI-only
surface#6.1 · no · `viola release`'s stdout line, a human verb with no MCP tool; no reached unit prints it, so it is a re-check every change owes
surface#7.1 · no · `viola verify`'s step counter and summary line, a CLI run over PTY children with no channel path; no reached unit prints it, so it is a re-check every change owes
surface#8.1 · no · `viola revive`'s silent start, its `--list` lines and its CLI-only `--json` codes; no reached unit prints them, so it is a re-check every change owes
surface#9.1 · no · exit 0's result words and `--json` ok envelope, a CLI print form that carries no refusal code; no reached unit prints them, so it is a re-check every change owes
surface#9.2 · no · exit 1's start, revive and verify refusals and their `--json` codes are CLI-only and none is a channel refusal code; no reached unit prints them, so it is a re-check every change owes
surface#9.3 · no · clap's usage text, a CLI parse path no channel frame carries; no reached unit prints it, so it is a re-check every change owes
surface#9.4 · yes · exit 10's `human-typing` refusal is a refusal code the Tokio client carries to `viola mcp`, which the scope keeps unchanged; the exit number is CLI-only
surface#9.5 · yes · exit 11's `budget-paused` refusal and its `five-hour` / `seven-day` details are refusal codes the Tokio client carries to `viola mcp`, which the scope keeps unchanged; the exit number is CLI-only
surface#9.6 · yes · exit 12's `unverified-cli` refusal is a refusal code the Tokio client carries to `viola mcp`, which the scope keeps unchanged; the exit number is CLI-only
surface#9.7 · yes · exit 13's `not-delivered` refusal and its detail codes are refusal codes the Tokio client carries to `viola mcp`, which the scope keeps unchanged; the exit number is CLI-only
surface#9.8 · yes · exit 14's `unknown` refusal is a refusal code the Tokio client carries to `viola mcp`, which the scope keeps unchanged; the exit number is CLI-only
surface#9.9 · yes · exit 20's `wrapper-fault` code is one the `viola mcp` tools return from the channel, which the scope keeps unchanged; the exit number is CLI-only
surface#9.10 · yes · exit 21's `instance-unreachable` code is among the refusal codes the `viola mcp` tools return, which the scope keeps unchanged; the exit number and `list`'s refusal are CLI-only
surface#10.1 · no · the `viola --help` verb groups, a CLI help text; no reached unit prints it, so it is a re-check every change owes
