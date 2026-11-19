@row_set n=1 row=decision/capability-ledger cell=except seen=4 detector=none
value: StopFailure is registered with no ledger row and no verify probe, ratified on one contained live StopFailure reading on claude 2.1.328, error word server_error.
basis: fct-vj6tsu22
because: The StopFailure exception is settled as outcome B on the contained live reading and is no longer provisional, so the row's exception must cite that reading.
@row_set n=2 row=decision/cli-capability-ledger cell=except seen=4 detector=none
value: StopFailure is registered with no ledger row and no verify probe, ratified on one contained live StopFailure reading on claude 2.1.328, error word server_error.
basis: fct-vj6tsu22
because: The StopFailure exception is settled as outcome B on the contained live reading and is no longer provisional, so the row's exception must cite that reading.
@row_set n=3 row=decision/capability-ledger-gate cell=except seen=3 detector=none
value: A CLI shape no send relies on needs no probe, as the long-paste frame beside typed text and between two pairs; StopFailure has no ledger row or verify probe, ratified on one contained live reading.
basis: fct-vj6tsu22
because: The StopFailure exception is settled as outcome B on the contained live reading and is no longer provisional, so the row's exception must cite that reading.
@row_set n=4 row=decision/claude-first-no-adapter-trait cell=except seen=6 detector=none
value: Test-only: rig private config; UserPromptSubmit/StopFailure keys, source word, error word (13, else other), 2 lengths, cachedChromeExtensionInstalled; VH keys, needle; fake agent, rig: trust records.
basis: fct-usi3wfxj
because: The rig's StopFailure capture now keeps key names, the error word of the CLI's 13 (else other) and two text lengths outside the agent crate, the one boundary widening the Claude-shape exception gains.
@row_set n=5 row=decision/claude-shapes-in-one-crate cell=except seen=6 detector=none
value: Test-only: rig private config; UserPromptSubmit/StopFailure keys, source word, error word (13, else other), 2 lengths, cachedChromeExtensionInstalled; VH keys, needle; fake agent, rig: trust records.
basis: fct-usi3wfxj
because: The rig's StopFailure capture now keeps key names, the error word of the CLI's 13 (else other) and two text lengths outside the agent crate, the one boundary widening the Claude-shape exception gains.
@row_set n=6 row=decision/claude-shapes-isolated cell=except seen=5 detector=none
value: Test-only: rig private config; UserPromptSubmit/StopFailure keys, source word, error word (13, else other), 2 lengths, cachedChromeExtensionInstalled; VH keys, needle; fake agent, rig: trust records.
basis: fct-usi3wfxj
because: The rig's StopFailure capture now keeps key names, the error word of the CLI's 13 (else other) and two text lengths outside the agent crate, the one boundary widening the Claude-shape exception gains.
@row_add n=1 row=decision/bwrap-test-only-rig-dependency area=build-system fact=F1 detector=D-arch-decisions
name: bwrap a test-only host dependency of the live rig
rule: bwrap is a host dependency of the test-only live rig alone, found by path on the rig's own PATH and never installed; a contained rig start without it is refused before any credentials read or ledger line.
basis: fct-rt54wgyw
because: The chunk brings bwrap in as a new host tool for the rig's contained arm, a runtime choice no row of the build system names.
@row_add n=2 row=decision/live-rig-contained-arm-sandbox area=occupied-resources fact=F3 detector=D-arch-resources
name: Live rig's contained arm in a network-closed sandbox
rule: The live rig's contained arm runs the wrapped CLI in a network-closed bwrap sandbox with its own network namespace, and starts only where XDG_RUNTIME_DIR is absolute and outside the temp root.
why: The sandbox's private temp root would hide a socket directory under it from viola send outside.
basis: fct-zrtvvb6q
because: The contained arm occupies its own network namespace and a runtime directory outside the temp root, a resource condition no row states.
@row_none n=1 fact=F2 type=4
what: The rig's kept record dir keyed by CLI version, refusing a version already kept, is how one test-rig mechanism is built, with no reason stated; the code holds it.
