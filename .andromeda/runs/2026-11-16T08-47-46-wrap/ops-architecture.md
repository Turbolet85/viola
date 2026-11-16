@row_set n=1 row=decision/local-commands-by-ledger cell=rule seen=1 detector=D-arch-decisions
value: A send is a local command only when its typed text is exactly a name on the ledger's local-command list, or that name, one space and a non-empty rest not starting with a space, never by a leading slash; any other slash text, skills included, is an ordinary send.
basis: fct-ywi7ddhz
because: Classification moved from exact equality of the whole typed text to by-name matching, a ratified widening, so the row's rule is now false.
@row_set n=2 row=decision/send-confirmed-after-the-fact cell=except seen=1 detector=none
value: A listed local command whose post-condition is none or unmeasured, a stdout-echo one with no readable transcript, or any on an unverified CLI returns `ok`, `confirmed: false`, `unconfirmable`.
basis: fct-ywi7ddhz
because: A stdout-echo command whose session transcript cannot be read is now answered unconfirmable, a case the exception does not name.
@row_set n=3 row=decision/unconfirmed-delivery-not-delivered cell=except seen=1 detector=none
value: A ledger local command whose post-condition is none or unmeasured, a stdout-echo one with no readable transcript, or any on an unverified CLI is reported ok, confirmed false, unconfirmable.
basis: fct-ywi7ddhz
because: A stdout-echo command whose session transcript cannot be read is now answered unconfirmable, a case the exception does not name.
@row_set n=4 row=decision/every-send-confirmed cell=except seen=1 detector=none
value: A ledger-listed local command is confirmed by its own measured post-condition, such as a stdout echo in the session transcript, or answered ok unconfirmable where none can be read.
basis: fct-ywi7ddhz
because: A local command echoing to stdout is now confirmed by a transcript tag with no prompt-submitted, so the row's unqualified rule no longer holds for every send.
@row_set n=5 row=contract/send-results/unconfirmable cell=promise seen=1 detector=none
value: A listed local command whose post-condition is none, a stdout-echo command with no readable session transcript, or any listed command on an unverified CLI, is typed and answered ok with {"confirmed":false,"detail":"unconfirmable"} at once; nothing else is ever unconfirmable.
basis: fct-ywi7ddhz
because: The send now makes a stdout-echo command unconfirmable when no transcript or no readable end is remembered, which the closed promise excluded.
@row_set n=6 row=contract/ok-payload/unconfirmable cell=promise seen=1 detector=none
value: unconfirmable is no refusal detail; it appears only as confirmed false with detail unconfirmable, for a ledger-listed local command whose post-condition is none or not yet measured, or is a stdout echo with no readable session transcript.
basis: fct-ywi7ddhz
because: A stdout-echo command with no readable transcript is now answered unconfirmable, a case the promise does not list.
@row_set n=7 row=contract/events-ndjson/send-confirmed-order cell=promise seen=1 detector=none
value: The line that confirms a send (the driver prompt-submitted, or the session-start for /clear and /compact) always precedes send-confirmed, whose submitted_at is that line's ts; a send confirmed by a stdout echo has no such line and its submitted_at is the instant the echo was read.
basis: fct-ywi7ddhz
because: An echo-confirmed send is settled by a transcript tag count with the read instant as submitted_at, so no event line precedes it as the promise said.
@row_set n=8 row=contract/channel-methods/hook-event cell=promise seen=1 detector=D-arch-resources
value: A notification sent id-less by `hook`, no reply, `{ts, event:{kind, data}, transcript?}`; the wrapper re-validates the event and appends it to `events.ndjson` with `source:"hook"`, an invalid event not appended.
basis: fct-ywi7ddhz
because: hook.event gained the optional sibling param `transcript`, so the stated params shape is incomplete.
@row_set n=9 row=contract/hook-event/notification cell=promise seen=1 detector=D-arch-resources
value: Sent id-less with no reply only after the instance, socket-directory and server checks pass; otherwise nothing is sent and the hook fails open. Shape {ts, event:{kind, data}, transcript?}.
basis: fct-ywi7ddhz
because: hook.event gained the optional sibling param `transcript`, so the stated shape is incomplete.
@row_add n=1 row=decision/local-command-echo-confirmation area=established-decisions fact=F1 detector=none
name: Local command confirmed by its stdout echo
rule: A local command whose post-condition is a stdout echo is confirmed by counting `<local-command-stdout>` tags in the session transcript past its end recorded before the paste; only a count crosses, never content, and the read is bounded by MAX_FRAME.
why: Ratified; the read opens no symlink, only a regular file owned by the own euid.
basis: fct-ywi7ddhz
because: Send now confirms a stdout-echo local command by a transcript tag count, a ratified decision no row states.
@row_add n=2 row=contract/hook-event/transcript area=standard-contracts fact=F2 detector=D-arch-resources
boundary: channel hook.event
item: transcript
promise: An optional string param beside `event`, never inside it, sent only for a SessionStart whose transcript path passes the shape check against its own session id; the wrapper re-validates it for a session-start line only, drops any other, and it never enters event data.
basis: fct-ywi7ddhz
because: hook.event gained the optional `transcript` param, a new channel contract item that must be registered in Standard Contracts.
@row_add n=3 row=decision/local-command-echo-probe area=established-decisions fact=F3 detector=none
name: Local command echo ledger row and its probe
rule: The local-command-echo relied ledger row is probed by Run B's seventh paste, /mcp disable viola-probe-absent, and passes on a new stdout tag with no UserPromptSubmit within the confirmation window; with no readable transcript the paste is skipped and the row fails.
why: Ratified as a widening of verify's pastes.
basis: fct-lel7lpiz
because: A new relied ledger row with its own verify probe now stands, a ratified decision no row states.
@row_none n=1 fact=F4 type=9
what: Unmeasured residuals of the echo probe and the owed live re-stamp: the route's carry and the security plan's residuals hold them.
