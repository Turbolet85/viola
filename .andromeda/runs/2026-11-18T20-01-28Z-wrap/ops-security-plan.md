@row_set n=1 row=decision/unverified-cli-gate cell=except seen=4 detector=none
value: StopFailure is registered with no capability ledger row and no verify probe, owned by the live StopFailure payload entry.
basis: fct-vmxsvsbs
because: The founder moved StopFailure's exception off the 2.1.325 recorded fixture set to its own entry, so the row's named owner of the exception no longer holds.
@row_add n=1 row=decision/live-rig-record-kept-owner-only area=data-protection fact=F1 detector=none
name: Live rig's kept record owner-only and local
rule: The live rig's recording arm keeps the CLI's recorded set locally at owner-only modes, never overwriting an existing copy and leaving first-screen captures out, and refuses before any start a scratch base whose path or canonical form names a component exactly home.
guard: keep_record_copies_the_set_byte_for_byte_and_never_overwrites
basis: fct-yu26cnvz
because: The chunk adds the rig's VR arm that keeps the recorded set and its record-home refusal, a data-protection rule no standing row states.
@row_add n=2 row=decision/live-rig-mcp-arm-keeps-no-output area=data-protection fact=F2 detector=none
name: Live rig's MCP arm keeps no CLI output
rule: The live rig's MCP recording arm keeps no CLI output, stdout drained and dropped and stderr kept only as a byte count and a SHA, and its MCP config is a private 0600 file inside the scratch, removed with it.
basis: fct-xacnmfep
because: The chunk adds the rig's MT arm whose handling of the CLI's output and its private MCP config no standing row states.
