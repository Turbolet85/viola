@row_retire n=1 row=decision/send-confirmation-exact-match seen=1 heir=decision/send-confirmation-trailing-trim-match detector=none
why: The send claim is no longer an exact equality: a submitted prompt equal to the typed text less a trailing run of ASCII space, TAB, CR and LF now claims the send.
basis: fct-ynhvzkwk, fct-hlqa3jtq
because: is_submitted_form replaced the exact compare in the claim, so the row's name and rule both state a match the code no longer holds.
@row_add n=1 row=decision/send-confirmation-trailing-trim-match area=unit-test-strategy fact=F1 detector=none
name: Send confirmation tolerates a trailing trim
rule: A send is confirmed only when the submitted prompt equals the typed text, or is non-empty and equals the typed text with its trailing run of ASCII space, TAB, CR and LF removed; any other difference, a leading trim included, claims nothing.
why: The CLI trims whitespace from a prompt at submit, so a driver's own send ending in it went unclaimed; the trim set is an assumption, not a live reading.
guard: submitted_form_takes_a_trailing_run_the_cli_trims, submitted_form_refuses_every_other_difference
state: provisional
basis: fct-ynhvzkwk, fct-kbepn7pe, fct-imkps2py
because: The chunk widened the confirmation match to a trailing-run tolerance held PROVISIONAL with the founder's word owed, and the test-plan row must state the match the guards now pin.
@row_set n=1 row=decision/send-refusal-order cell=rule seen=2 fact=F2 detector=none
value: send refuses in the order control-character, empty-text, human-typing, turn-running, input-not-ready, no-prompt-submitted; after the gate it rereads human-typing and turn-running under the flight lock the hook's turn and claim decisions share, held through the link, send-issued and the paste.
basis: fct-ynhvzkwk, fct-hlqa3jtq
because: The post-gate reads now run under one flight lock held across the link, send-issued and the paste and shared with the hook event's decisions, closing the turn race, with the order unchanged.
@row_set n=2 row=contract/viola-send/records cell=promise seen=1 detector=none
value: The records after the send's cursor are send-issued{cursor, from} then prompt-submitted{origin:"driver"} holding the submitted text: the typed text, or the typed text less the trailing run the CLI trimmed.
basis: fct-ynhvzkwk
because: The relabel keeps the submitted text and the claim now takes a trailing-trimmed prompt, so the driver line no longer always holds the typed text.
@row_set n=3 row=contract/fake-agent-input/bracketed-paste cell=promise seen=1 detector=none
value: Bracketed pastes plus Enter are accepted as one prompt and fire UserPromptSubmit (variant default) with that text, or under --trim-prompt-end with its trailing run of space, TAB, CR and LF removed.
basis: fct-ynhvzkwk, fct-lf7rpcrh
because: The fake agent gained the argv flag --trim-prompt-end, which changes the prompt a default UserPromptSubmit fires with.
@row_none n=1 fact=F3 type=9
what: The unreached hook-latency window is a known limit owned by the carry on the standing live rig; the route holds it.
