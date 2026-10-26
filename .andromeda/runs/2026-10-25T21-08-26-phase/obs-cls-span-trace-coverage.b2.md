# Classified — obs · span-trace-coverage.b2

## Answers
span-trace-coverage#7.2 · no · the run.wheel_transition span of the wheel scenario, which the Not-needed list drops with the wheel and release paths the chunk does not change
span-trace-coverage#7.3 · no · the human-typing send-refused and release-from-driver lines of the wheel scenario, which the Not-needed list drops
span-trace-coverage#7.4 · no · the run.wheel_transition cleanup and the wheel product event, both on the wheel scenario the Not-needed list drops
span-trace-coverage#8.1 · no · the statusline and budget-write spans are dropped by the Not-needed list, and run.budget_eval is a span the scope never names since only the send-side refusal line is reached
span-trace-coverage#8.2 · yes · read_budget reads a refused, unreadable or over-cap budget.json as no reading with no line, a path to the missing send-refused line that the CARRY cause re-check must rule in or out
span-trace-coverage#8.3 · yes · the budget-paused send-refused line with its refusal word, window word, paused flag and exit 11, which CARRY case_4 reads
span-trace-coverage#8.4 · no · the statusline shell-out deadline and oversize-stdin cleanup, on the statusline hook arm the scope does not reach
span-trace-coverage#9.1 · no · the R8 env-strip counts on the claude-child process-start, which the Not-needed list drops with the child spawn
span-trace-coverage#10.1 · no · the implicit link line and the E3 channel pairs, which a refused send never writes and the Not-needed list drops
span-trace-coverage#11.1 · no · the E5 state-recovered replay line, an edge flow the Not-needed list drops, and run's collision check reads the snapshot without replay
span-trace-coverage#12.1 · no · torn-line healing on the events.ndjson append, an edge flow the Not-needed list drops
span-trace-coverage#13.1 · no · the capability ledger read in run.version_gate and its parse-rejected line, which the Not-needed list drops
span-trace-coverage#14.1 · no · the verify and plugin install dispatch spans and child-spawn pairs, which the Not-needed list drops
