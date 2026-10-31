# Classified — a11y-plan · aria-patterns-roles.b1

## Answers
aria-patterns-roles#1.1 · no · the served-verb help rows and the HELP literal in tests/cli_output_plain.rs (not reached) still hold, since the session adds no verb, help row or group and its src/main.rs and src/cmd edits move exit documents only
aria-patterns-roles#1.2 · no · the tui row is the vendor claude TUI boundary, N/A, and the session touches no TUI surface or viola-channel code
aria-patterns-roles#2.1 · no · the cli claim (send refused with human-typing on stderr, typed exit 10) sits in the untouched human-typing arm of send.rs, and the wheel probe's answer exit codes 13 and 10 are unmoved, the session changing only the no-endpoint exit-21 arm
aria-patterns-roles#3.1 · no · the board, wait and last claims (CAPTION literal, no SGR under non-TTY or --json, no raw C0/C1 on stdout, waiting line once on stderr) still hold, since the new no-endpoint documents carry no ESC or SGR and the non-JSON lines are unchanged
