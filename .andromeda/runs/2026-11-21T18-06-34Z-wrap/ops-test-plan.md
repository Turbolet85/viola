@row_set n=1 row=contract/fake-agent-receipt/start cell=promise seen=1 detector=none
value: Carries cli_version, pid and the print-mode prompt; in interactive mode written after the fake agent has taken raw mode on its own terminal, so tests send no key before it; print mode takes no raw mode and writes it before its four fixture events.
basis: fct-to766trg, fct-zjeenbpz
because: The fake agent's start receipt now records pid and the print prompt, so the receipt contract that lists only cli_version is incomplete.
@row_set n=2 row=contract/fake-agent-cli/settings cell=promise seen=1 detector=none
value: --settings <file> is accepted; the fake agent reads its cross-session inbox only when the value holds the accept settings, and otherwise ignores it.
basis: fct-zjeenbpz, fct-mxkdncaf
because: The fake receiver now reads its inbox only under the accept settings, so --settings is no longer consumed and ignored.
@row_set n=3 row=decision/fixture-drift-byte-compare cell=except seen=1 fact=F3 detector=none
value: A hook payload that names the send tool is left out of the dialog comparison, so a side sender's hooks never shift recorded dialog positions.
basis: fct-tkwnjker, fct-lwwhvdyk
because: The drift comparison now leaves out send-tool hook payloads, an exception to the byte-for-byte rule the row states without one.
@row_set n=4 row=decision/fake-agent-matches-recordings cell=except seen=1 detector=none
value: A hook payload that names the send tool is left out of the dialog comparison, so a side sender's hooks never shift recorded dialog positions.
basis: fct-tkwnjker, fct-lwwhvdyk
because: The fake agent's hook sequence is no longer compared whole: send-tool hook payloads are left out, which the row does not say.
@row_set n=5 row=decision/fixture-hygiene-walk cell=except seen=3 fact=F4 detector=none
value: The recorded cross-session arrival keeps the recording host's runtime-dir path, the sender pid and a numeric from-name inside the CLI's own prefix; the walk flags only home-shaped paths, so it passes.
basis: fct-wn36g3ew
because: The committed cross-session fixture holds an absolute runtime-dir path that the walk, flagging only home-shaped paths, lets pass, an exception the row does not state.
@row_set n=6 row=decision/fake-agent-replays-recorded-bytes cell=except seen=3 detector=none
value: The cross-session sender's PreToolUse and PostToolUse hooks carry synthetic payloads naming the send tool, never recorded bytes.
basis: fct-tkwnjker, fct-lwwhvdyk
because: The fake sender now runs two synthetic send-tool hooks, so the row's no-payload-built rule holds an exception it does not name.
@row_set n=7 row=contract/fake-agent-hooks/stdin-payload cell=promise seen=3 detector=none
value: The payload on a hook's stdin is the recorded fixture's bytes, save the fields a prompt, a resume, a compact replay, the launch SessionStart under --transcript or a Stop under --stop-crons sets, and the cross-session sender's synthetic send-tool hooks.
basis: fct-tkwnjker, fct-lwwhvdyk
because: The fake sender's two hooks carry synthetic payloads, so the promise that every hook stdin is recorded bytes save the listed fields is incomplete.
@row_add n=1 row=decision/stamping-verify-test-owned-runtime-dir area=test-data-bootstrap fact=F1 detector=none
name: Stamping verifies get a test-owned runtime dir
rule: Every test or harness viola verify that stamps runs with a short owner-only runtime dir the test or harness owns for the stamp's life.
why: verify reads XDG_RUNTIME_DIR once to compose the cross-session address, and the clean pre-push environment leaves it unset.
except: The rig rehearsal sets its own runtime dir only where the host sets none.
basis: fct-ty2bg6tq, fct-m4yamtxm, fct-whtuymhr
because: A new env read in verify makes every 29-row stamp depend on a runtime dir, so the test-data bootstrap now gives each stamping verify one.
@row_add n=2 row=decision/fake-agent-cross-session-by-uds-address area=test-data-fixtures fact=F2 detector=none
name: Fake agent cross-session by uds address
rule: The fake agent's cross-session sender resolves its target by the uds socket address alone, delivers only to its sibling and refuses an address not of the uds shape; its receiver reads its inbox only under the accept settings.
why: The real CLI's send tool resolves no local session by a bare session id.
guard: sender_refuses_an_address_not_of_the_uds_shape, receiver_reads_no_inbox_without_the_accept_settings
basis: fct-zjeenbpz, fct-vgvx3eex
because: The fake agent gained a cross-session sender and receiver held to the real CLI's address form, a test-only contract no row states.
@row_none n=1 fact=F5 type=4
relied: A rig rehearsal command rebuilt from its program and args keeps the original command's env, which the rehearsals' runtime dir under the clean pre-push environment relies on.
@row_none n=2 fact=F6 type=5
what: The canary negative is a test in the inventory; the names-only capture arm's decision is the security-plan's and the architecture's, and the code holds the test.
