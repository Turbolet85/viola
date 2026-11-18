@row_set n=1 row=decision/unverified-cli-gate cell=except seen=4 detector=none
value: StopFailure is registered with no capability ledger row and no verify probe, owned by the entry A live StopFailure payload.
basis: fct-vmxsvsbs
because: The founder moved StopFailure off the 2.1.325 recorded fixture set to its own entry, A live StopFailure payload (inputs#I2 answer 3), so the exception's named owner must be that entry, named by its title.
@row_set n=2 row=decision/mcp-fixtures-via-test-only-tap cell=rule seen=1 detector=none
value: MCP exchange fixtures are recorded by a test-only tap, started live by the standing rig under its private CLI config, that spawns viola mcp directly with no shell, caps each line, keeps its raw capture 0600 in a private directory never committed, and applies the same scrub and closed refusal codes.
basis: fct-txncvu63
because: The plan's expected amendment: the tap's live start is now the standing rig's R4M under its private CLI config, never the user's real config, and the row must say where the tap runs.
