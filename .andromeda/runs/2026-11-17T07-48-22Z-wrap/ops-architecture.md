@row_add n=1 row=name/binary/viola-live-rig area=occupied-resources fact=F1 detector=D-arch-resources
class: binary
name: viola-live-rig
basis: fct-yubkirys
because: The chunk lands the test-only binary viola-live-rig in viola-e2e, and no row of the register names it.
@row_add n=2 row=name/other/agent-run-rig area=occupied-resources fact=F2 detector=D-arch-resources
class: other
name: AGENT_RUN_RIG
basis: fct-yubkirys
because: The chunk sets the new env name AGENT_RUN_RIG on the live rig's child as its census marker, and no row of the register names it.
@row_add n=3 row=decision/standing-live-rig area=ci-cd-approach fact=F3 detector=none
name: One standing live rig for every live round
rule: One committed test-only live rig serves every live round: its rehearsals against the fake agent run in CI, its live starts against the real claude run only by hand under a per-chunk start cap, and its live verb refuses under CI.
why: Ratified.
guard: rig_live_arm_refuses_under_ci (rig_rehearsal)
basis: fct-sjq6de27
because: The chunk makes viola-live-rig the standing rig every later live round reuses, its rehearsals a CI test and its live verb refused under CI, a decision no row of the CI approach states.
@row_add n=4 row=decision/live-rig-private-claude-config area=occupied-resources fact=F6 detector=none
name: Live rig child isolated from the user's claude config
rule: The live rig's child runs under its own private CLAUDE_CONFIG_DIR in a scratch the rig removes on every exit path; the user's own claude config is read only as a hash and mtime, before and after.
guard: rig_reads_the_user_config_before_and_after (rig_rehearsal)
basis: fct-nwy7zwl7
because: The chunk gives the rig's child a private config dir in a removed scratch and reads the user's config only as a hash and mtime, an isolation rule no row states.
@row_add n=5 row=contract/viola-live-rig-exit-codes/0 area=occupied-resources detector=D-arch-resources
boundary: viola-live-rig exit codes
item: 0
promise: A whole start.
basis: fct-yubkirys
because: The new test-only binary promises a closed set of exit codes later live rounds read, and no row holds them.
@row_add n=6 row=contract/viola-live-rig-exit-codes/1 area=occupied-resources detector=D-arch-resources
boundary: viola-live-rig exit codes
item: 1
promise: A start read red.
basis: fct-yubkirys
because: The new test-only binary promises a closed set of exit codes later live rounds read, and no row holds them.
@row_add n=7 row=contract/viola-live-rig-exit-codes/2 area=occupied-resources detector=D-arch-resources
boundary: viola-live-rig exit codes
item: 2
promise: A usage error.
basis: fct-yubkirys
because: The new test-only binary promises a closed set of exit codes later live rounds read, and no row holds them.
@row_add n=8 row=contract/viola-live-rig-exit-codes/3 area=occupied-resources detector=D-arch-resources
boundary: viola-live-rig exit codes
item: 3
promise: A start refused before its ledger line is written.
basis: fct-yubkirys
because: The new test-only binary promises a closed set of exit codes later live rounds read, and no row holds them.
@row_set n=1 row=decision/test-binaries-never-released cell=rule seen=1 detector=D-arch-resources
value: viola-fake-agent, viola-harness, viola-mcp-tap and viola-live-rig are test-only binaries and never ship in a release build; the fake agent is built only behind the fake-agent feature.
basis: fct-sjq6de27
because: The chunk adds viola-live-rig as a test-only binary kept out of the release build, and the row's list of test-only binaries must name it.
@row_set n=2 row=decision/harness-env-unread-by-product cell=rule seen=1 detector=D-arch-resources
value: AGENT_RUN_CHUNK_BASE, AGENT_RUN_KEEP_HOMES, AGENT_RUN_KEEP_FAILED, AGENT_RUN_RIG and CI are test-harness variables that viola itself never reads.
basis: fct-yubkirys
because: The chunk adds AGENT_RUN_RIG, set only on the rig's child and never read by viola, so the row's list of harness variables must name it.
@row_set n=3 row=decision/no-typing-into-trust-dialogs cell=except seen=1 fact=F4 detector=none
value: The test-only live rig, outside the product, types only its argv-granted keys (at most two down-arrows and Enter on a read trust row, one x, one DEL) and Ctrl-C only from its stop.
basis: fct-wv4f6rkv
because: The rig's key grant into the trust dialog lands ratified as the one exception outside the product, so the rule must carry it as its exception.
@row_set n=4 row=decision/submitted-form-prompt-match cell=why seen=2 fact=F5 detector=none
value: The CLI trims only a trailing run at submit: it keeps a leading run of spaces, removes a trailing run of spaces and TAB, and keeps an inner run of spaces and an inner LF.
basis: fct-seoizoiu
because: R2 read the CLI's trim live, so the reason that called the trim set an unread assumption with either end a candidate is now false.
@row_set n=5 row=decision/vt100-screen-model cell=rule seen=1 detector=D-arch-decisions
value: vt100 gives pre-send readiness and modal detection on run's feed thread, the screen of verify's interactive probe runs, and the screen the test-only live rig reads of its child.
basis: fct-yalkolrl
because: vt100 now also serves the test-only viola-e2e crate as the live rig's screen model, a role the row does not name.
@row_set n=6 row=decision/vt100-readiness-gate cell=except seen=1 detector=D-arch-decisions
value: The test-only live rig in viola-e2e also uses vt100 as the screen model it reads its child's trust dialog, selector and input box from.
basis: fct-yalkolrl
because: The row says vt100 serves only as a pre-send readiness gate, while the chunk adds it to viola-e2e as the rig's screen model, so the rule needs that exception.
