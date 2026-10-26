# New groups — obs-plan · log-coverage

## Verdicts
- fanout-obs-plan--cascade-g2.md 18 · distinct · line 28 only lists `server-verify-failed` among exit-21 codes, and no standing group states that this build writes it, nor its `during:"connect"`, WARN level, `cli-<name>.ndjson` sink or six CLI verbs (line 33's "no arm of this build writes the word" concerns `strict-modes-failed`, not this code)
- fanout-obs-plan--cascade-g2.md 19 · distinct · line 30 only lists `server-verify-failed` among hook fail-open codes, and no standing group states that `viola hook` writes it, nor its WARN level, `hook-<name>.ndjson` sink or the dialog path's `hook-invoked` / `hook-decision` order (line 33's no-arm-writes clause concerns `strict-modes-failed`); its fact (the hook arm) is not proposal 18's (the CLI verbs)
