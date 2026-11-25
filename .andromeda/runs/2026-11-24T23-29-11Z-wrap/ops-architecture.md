@row_set n=1 row=contract/cli-json-output/adopt-refusal-codes cell=promise seen=1 detector=D-arch-resources
value: `<code>` is one of `strict-modes-failed` · `session-live` · `not-resumed` · `already-live` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`; only `strict-modes-failed`, `session-live` and `not-resumed` are adopt's own closed details.
basis: fct-p367ntgk
because: viola adopt --json gains its own third refusal code not-resumed, so the closed list and the count of adopt's own details must name it.
@row_set n=2 row=decision/no-document-after-child-held-terminal cell=except seen=1 fact=F2 detector=D-arch-decisions
value: viola adopt --json's not-resumed end prints its error document after the child released the terminal; nothing of run or revive.
basis: fct-x4brbvla
because: adopt's not-resumed end writes its document after the child held the terminal, the one named exception scoped to adopt, so the decision must name it as its exception.
@row_set n=3 row=contract/instance-snapshot/agent-session-id cell=promise seen=1 detector=none
value: `agent_session_id` is the one from the newest `session-start` event; it joins `claude agents --json` rows to this instance, and viola adopt reads a real snapshot holding none as a session that did not resume.
basis: fct-x4brbvla
because: adopt now reads the real snapshot's agent_session_id to decide its not-resumed end, so the field no longer exists only for the agents join.
@row_set n=4 row=decision/claude-flag-literals-in-agent-crate cell=except seen=3 detector=D-arch-decisions
value: viola adopt's fixed not-resumed hint names claude --resume <id> as words for the human to type, never as an argument viola passes.
basis: fct-x4brbvla
because: adopt's fixed hint now carries the --resume literal as human text outside the agent crate, so the decision must name that case as its exception.
@row_add n=1 row=decision/adopt-not-resumed-end area=established-decisions fact=F1 detector=none
name: Adopt ends not-resumed when no session started
rule: viola adopt ends not-resumed, exit 1, when its child exits and the instance's real snapshot holds no agent_session_id, logging the child's process-exit, then a self one of detail not-resumed, then its fixed pair or --json document; a replayed, absent or unreadable snapshot reads as resumed, exit 0.
basis: fct-x4brbvla
because: adopt gained a not-resumed end after its child exits, a decision future work on adopt must respect.
@row_add n=2 row=contract/process-exit-detail/not-resumed area=occupied-resources fact=F3 detector=D-arch-resources
boundary: process-exit detail enum
item: not-resumed
promise: A word of the process-exit detail enum in schemas/diag-line.v1.json, occupied by viola adopt: its exit 1 on the self subject, logged after the claude-child process-exit.
basis: fct-uwoaubez
because: the closed process-exit detail enum gained the word not-resumed, an occupied name the registry must hold.
@row_add n=3 row=contract/adopt-refusal-pairs/not-resumed area=standard-contracts fact=F4 detector=none
boundary: viola adopt human refusal pairs
item: not-resumed
promise: The fixed pair is `unable: <name>'s session did not resume`, then `hint: check the session id - claude --resume <id> must open it by hand, under the same claude config`, with `<id>` the literal word; the session id stands in no line.
basis: fct-x4brbvla
because: adopt's new not-resumed refusal has a fixed human pair, a contract at the CLI boundary.
@row_add n=4 row=decision/adopt-spawn-folder-not-session-folder area=established-decisions fact=F5 detector=none
name: Adopt keeps revive's spawn folder
rule: viola adopt does not start its child in the folder the session was made in; its spawn folder stays as revive's start path sets it.
why: The claude CLI resumes a session by id from an unrelated folder.
basis: fct-v3bqsqjp
because: the chunk's premise that adopt must start in the session's folder was found false and the spawn folder was deliberately left unchanged, a decision future work must respect.
