P4 fork (phase of 2026-11-16-granular-api-error-cause), asked by AskUserQuestion:
"Granular API-error cause: when a later CLI sends a StopFailure `error` value outside the 13 the CLI declares today, it will read `api-error`, the same as the CLI's own `unknown`. Should viola leave a trace that such a value appeared?"

Answer selected: "One closed log word (Recommended)"

Notes, verbatim: The overseer's technical answer, 2026-11-16T17:40:41Z (not the founder's word): it applies obs's standing rule parse-failures-unknown-counter and widens no boundary — a code and a count, never the value. If the plan finds it needs more than the one enum member, its emitter and its test, say so on the card.
