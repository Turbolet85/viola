@answer n=1 entity=obs-scope-summary#8.3 v=yes
why: the secret scan on every OS witnesses that R8-stripped CLAUDE values never reach any diagnostics file, and drift reports stay in diagnostics/ (need 15, need 20)
@answer n=2 entity=obs-scope-summary#9.1 v=no
why: source citation only (arch Stack and Workspace); it states no rule the chunk must honour
@answer n=3 entity=obs-scope-summary#9.2 v=no
why: an instrumentability label for the MCP surface that prescribes no code or test; its stdout rule is entity 9.3
@answer n=4 entity=obs-scope-summary#9.3 v=yes
why: no telemetry may go to stdout, which the hook contract (stdout only the decision body) and the clippy print_stdout lint enforce (need 17, need 19)
@answer n=5 entity=obs-scope-summary#10.1 v=no
why: source citation only (portable-pty, Project Intent); it states no rule
@answer n=6 entity=obs-scope-summary#10.2 v=yes
why: the openpty OS API is boundary-only behind the pty seam the chunk builds (need 1)
@answer n=7 entity=obs-scope-summary#10.3 v=yes
why: process-start and process-exit for the claude child come from the handle-wait thread, never from EOF, and paste logs text_bytes only (need 2)
@answer n=8 entity=obs-scope-summary#11.1 v=no
why: source citation only (Hook contract, MCP tools); it states no rule
@answer n=9 entity=obs-scope-summary#11.2 v=yes
why: the hook and MCP surfaces the chunk touches (MCP instance-unreachable with no line, hook contract under the Unix suite) are instrumented only at the boundary (need 6, need 19)
@answer n=10 entity=obs-scope-summary#11.3 v=yes
why: the viola mcp stdin oversize parse-rejected line and its closed words fall under the one diagnostics schema the chunk re-asserts on every OS (need 5)
@answer n=11 entity=obs-scope-summary#12.1 v=no
why: source citation only (sysinfo, claude agents --json); it states no rule
@answer n=12 entity=obs-scope-summary#12.2 v=no
why: the external claude agents CLI is outside the chunk's needs and no live claude session runs (task prose)
@answer n=13 entity=obs-scope-summary#12.3 v=no
why: the claude agents spawn, exit and parse-outcome instrumentation is untouched by the chunk's needs
@answer n=14 entity=obs-scope-summary#13.1 v=no
why: source citation only (Standard Contracts, viola hook statusline); it states no rule
@answer n=15 entity=obs-scope-summary#13.2 v=yes
why: the statusline output is part of the hook contract under the Unix suite, and its shell-out is instrumented only at the boundary (need 19)
@answer n=16 entity=obs-scope-summary#13.3 v=yes
why: the statusline command string must stay out of logs outside diagnostics, a content-bearing rule the log location and detail rule rules over (need 20)
@answer n=17 entity=obs-scope-summary#14.1 v=no
why: source citation only (Stack); it states no rule
@answer n=18 entity=obs-scope-summary#14.2 v=yes
why: the Unix peer-euid, socket-directory and strict-modes checks the chunk adds are instrumented only at viola's call sites (need 7, need 8)
@answer n=19 entity=obs-scope-summary#14.3 v=yes
why: its refusal lines (server-verify-failed, socket-dir-refused, and the accept_loop euid drop with no line) are the Unix telemetry the chunk adds (need 6, need 7, need 8)
@answer n=20 entity=obs-scope-summary#15.1 v=no
why: source citation only (ndjson, snapshots, heartbeat); it states no rule
@answer n=21 entity=obs-scope-summary#15.2 v=no
why: an instrumentability label through viola-state that names no check; the scrub rules that bind are entity 15.3
@answer n=22 entity=obs-scope-summary#15.3 v=yes
why: the scrub rules bind the chunk's diagnostics: no path or pid in detail, absolute paths scrubbed from external output, and state-recovered lines keep the same detail words (need 5, need 21)
@answer n=23 entity=obs-scope-summary#16.1 v=no
why: source citation only (Observability Hints, Config management); it states no rule
@answer n=24 entity=obs-scope-summary#16.2 v=no
why: a not-instrumentable label for static config that the chunk does not change
@answer n=25 entity=obs-scope-summary#16.3 v=no
why: only the existing load-time parse of config is logged, and the chunk adds no config logging or skipped-key detail
