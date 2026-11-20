@row_set n=1 row=decision/stop-session-crons-count-only cell=rule seen=1 fact=F3 detector=D-security-input
value: Stop's session_crons is carried only as its length, scheduled_wakes on turn-ended, which the wheel rule also reads; no element is read or kept; null when absent, null or not an array, a non-array also writing one drift entry to the diagnostics detail file; null on an API-error turn end.
basis: fct-yqdgc3b5
because: The turn-ended count now feeds the told-wake wheel rule while the new stop-session-crons ledger row checks only is_array and reads no element, so the row must say the count also decides the wheel.
@row_set n=2 row=decision/harness-prefix-literals cell=except seen=1 detector=none
value: The wrapper relabels a keyless human-filed prompt told as a CLI-scheduled wake origin harness.
basis: fct-yqdgc3b5
because: The told-wake rule now files a prompt matching no harness prefix as harness, so the row's only-by-literals rule needs that named exception.
@row_set n=3 row=decision/live-rig-kept-stamp-owner-only cell=rule seen=1 detector=none
value: The live rig keeps a stamp from its CS or WS scratch home owner-only and no-clobber, and seeds it only into its own CC or R3W scratch home; CC with no kept stamp refuses before its ledger line and before any scratch, R3W before its ledger line.
basis: fct-yqdgc3b5
because: The rig now keeps a wake stamp from WS and seeds it into R3W, which refuses without it before its ledger line, so the row naming CS and CC alone is incomplete.
@row_set n=4 row=decision/stamps-written-only-by-verify cell=except seen=2 detector=none
value: The test-only live rig seeds a stamp kept from its own verify scratch home into its own CC or R3W scratch home only.
basis: fct-yqdgc3b5
because: The rig's R3W label now seeds the kept wake stamp into its own scratch home, so the exception naming CC alone is incomplete.
@row_add n=1 row=decision/told-wake-keeps-driver-wheel area=threat-model-summary fact=F1 detector=none
name: A told scheduled wake keeps the driver's wheel
rule: A keyless human-filed prompt is told as a CLI-scheduled wake, filed origin harness with the wheel left with the driver, only when no stdin key was typed since the last human-filed prompt, the last turn end counted a scheduled wake and the CLI is verified.
except: A prompt submitted by the mouse alone is the one named false positive, unmeasured.
basis: fct-yqdgc3b5, fct-wncy4gmx
because: The chunk adds the told-wake rule that keeps the wheel from the human on a keyless prompt, and no row of the threat model states when a human-filed prompt no longer takes the wheel.
@row_add n=2 row=decision/live-rig-hb-arm-keeps-attributes-only area=authentication-authorization fact=F5 detector=none
name: Live rig's HB arm keeps cell attributes and counts only
rule: The live rig's HB arm types one x and one DEL into a fresh box in its own unwrapped pty before any send, and records only cell attributes and counts, never screen text.
basis: fct-yqdgc3b5, fct-lau4xcj2
because: The chunk adds the HB arm that types into a fresh box before any send, and no row bounds what that arm keeps of the screen.
@row_add n=3 row=decision/live-rig-wake-arm-reads-own-wheel-lines area=data-protection fact=F6 detector=none
name: Live rig's R3W arm reads only viola's own wheel lines
rule: The live rig's R3W arm reads from the events log in its own scratch home only viola's own wheel lines and the prompt's origin word, and records the captured prompt's key names, never their values.
basis: fct-yqdgc3b5, fct-qqysllak
because: The chunk adds the R3W arm reading the events log after a fired wake, and no row bounds what that arm reads and keeps.
@row_none n=1 fact=F2 type=9
@row_none n=2 fact=F4 type=4
relied: the fake agent's --stop-crons and --wake-after-ms are capped argv-only options under the fake-agent feature, reading no env and writing no fixture, as decision/test-seams-compiled-out already rules
