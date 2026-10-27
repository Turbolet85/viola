# New groups — architecture · conventions

## Verdicts
- fanout-architecture--cascade-g1.md 17 · distinct · line 23 states only `revive`'s instance strict-modes check before its log (the instance-state pair); no standing line holds the start's home-tree check (`checked_exe` / `refuse_tree`) for `run` or `revive`, nor its home-state pair
- fanout-architecture--cascade-g1.md 18 · distinct · line 25's list of `viola verify`'s refusals holds no home-tree check before its `cli` log and no `StampsError::StrictModes` mapping
- fanout-architecture--cascade-g1.md 19 · distinct · lines 34 and 40 give exit 21 for a refused strict-modes check only to `list`, `plugin install` and `link` / `unlink`, and for the six frame verbs only the server-identity mismatch; none states the six verbs' `refuse_untrusted` check
- fanout-architecture--cascade-g1.md 22 · distinct · line 53 states the frame tools' server-identity mismatch only; no standing line holds a `check_instance` refusal before `live_endpoint`
