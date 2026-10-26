# Classified — a11y · a11y-scope-summary.b1

## Answers
a11y-scope-summary#1.1 · yes · the no-behaviour-change boundary keeps every CLI verb's human, plain and --json output, refusal words, hint lines and typed exit codes byte-identical, re-run through the existing output-discipline cases
a11y-scope-summary#2.1 · yes · the scope keeps the viola run TUI boundary as it is (zero viola bytes to the terminal, keystrokes never blocked or delayed) and re-runs the existing outer-PTY boundary cases
a11y-scope-summary#3.1 · yes · the existing CLI output-discipline drivers (assert_cmd, trycmd, SGR checks) ride inside the re-run gates, and the required CLI output rules (no SGR under --json, a pipe, NO_COLOR or TERM=dumb; static lines; C0/C1 escaped) keep holding as built
a11y-scope-summary#4.1 · yes · the viola run TUI boundary (zero viola bytes to the terminal, keystrokes never blocked or delayed), re-run through the existing outer-PTY cases on all three OS legs; the manual NVDA pass is founder-owned and not carried
a11y-scope-summary#5.1 · yes · the as-built caption line, UNWRAPPED separator and --help group labels are CLI output the byte-identical boundary keeps, though the chunk changes none of them
