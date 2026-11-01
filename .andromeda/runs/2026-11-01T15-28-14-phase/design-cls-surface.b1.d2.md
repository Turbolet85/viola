# Classified — design · surface.b1

## Answers
surface#1.1 · no · standing S3 · each new line is plain ASCII with no colour, glyph or cursor control
surface#1.2 · no · standing S3 · each new line is plain ASCII with no colour, glyph or cursor control
surface#1.3 · no · standing S3 · each new line is plain ASCII with no colour, glyph or cursor control
surface#1.4 · no · standing S3 · each new line is plain ASCII with no colour, glyph or cursor control
surface#1.5 · no · standing S3 · each new line is plain ASCII with no colour, glyph or cursor control
surface#1.6 · no · standing S3 · each new line is plain ASCII with no colour, glyph or cursor control
surface#2.1 · yes · the stderr form of each new refusal, `unable  <name>  <reason>  <detail>` or the fixed `unable: <text>` form, with its `hint:` line directly after; the chunk's socket-directory and peer euid refusals choose one of these forms
surface#2.2 · yes · each new closed code is a code word (1 to 64 bytes, lower-case first), or a client's screen reads it as `unknown`
surface#2.3 · yes · a new closed code carried as a peer's refusal must be a code word, else it reads `unknown`; a non-code-word detail reads as no detail
surface#2.4 · yes · each new refusal code must join the closed refusal list, else it stays visible and exits 14 as `unknown` does
surface#3.1 · no · the NAME truncation of human table rows at the terminal width; the chunk changes board::scan's home check, not the table, and prints no name column
surface#4.1 · no · the board's rows, WHEEL, UNWRAPPED section and widths in healthy output; the chunk changes board::scan's home check, not the table
surface#4.2 · yes · the board's strict-modes refusal (whole board refused, fixed home or instance pair, `--json` `strict-modes-failed` document); the chunk's one home rule in board::scan decides which pair prints
surface#5.1 · yes · the board's two-rule home check (write rule on the home, read rule on each instance set) that the chunk makes one rule; which pair a 0750 home reads depends on it
surface#6.1 · yes · the per-cause instance-unreachable hints of send and the frame verbs, and the strict-modes pair before any frame; the chunk's client connect refusals and euid or directory refusals print under these hints
surface#7.1 · yes · the server-verification hint for the eight frame verbs, the line a client prints when the server's peer euid check before its first frame refuses
surface#8.1 · yes · on macOS the server check passes with no comparison and the server-verification hint cannot print; the chunk records this pid gap as a named carve-out
surface#9.1 · no · the `viola release --budget` success line; the chunk touches only refusals before their frames
surface#10.1 · no · the link and unlink success line and exit 0; the chunk adds refusals before the first frame and changes no success output
surface#10.2 · no · the `since` field of the link success line; not reached
surface#10.3 · no · the already-linked and not-held unlink success lines; not reached
surface#11.1 · no · verify's step lines, summary and stop message; the chunk changes no ledger row or stamp
surface#12.1 · yes · verify's home-tree check before its log and the stamps.json strict-modes refusal print the home-state pair and exit 1; the chunk applies Unix strict modes on these paths
surface#13.1 · yes · on Unix a home under a regular file is a strict-modes refusal reaching the home-state pair and the frame verbs' instance-state pair; the chunk's strict-modes checks must produce it
surface#14.1 · yes · revive's start refusal (exit 1, fixed pair and one hint per cause) and its silence once the child starts; the start shares the bind and directory check the chunk adds, and a wrapper's euid refusal prints nothing while the child holds the terminal
