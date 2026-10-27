# New groups — architecture · established-decisions

## Verdicts
- fanout-architecture--cascade.md 4 · contradiction · lines 53–53 · «`replace_private_shared` is the variant for files every concurrent start of one version writes identically (the pinned copy, the plugin files» · the standing line places the pinned copy through `replace_private_shared` (a `persist` replace), the proposal through the no-clobber `place_private_new` at `DIR_MODE`
- fanout-architecture--cascade.md 5 · distinct · no standing group names `place_private_new_with` or its `placed` test hook
- fanout-architecture--cascade.md 6 · contradiction · lines 53–53 · «`replace_private_shared` is the variant for files every concurrent start of one version writes identically (the pinned copy, the plugin files» · the standing line has the pinned copy written by the `persist` replace whose open-handle rename the proposal's rationale says the pinned copy no longer uses; the `ETXTBSY` measurement itself stands nowhere
- fanout-architecture--cascade.md 7 · distinct · no standing group states that one tree builds two files keying two `bin/<key>/` folders; line 249 states only why the hash is in the key
