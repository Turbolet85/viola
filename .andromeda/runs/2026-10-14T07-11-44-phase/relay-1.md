# relay-1 — P4 forks of chunk 2026-10-14-resume-and-compact-ledger-rows, and the answers

Asked by the phase session as one AskUserQuestion round (three forks, each recommendation marked first). Answered by
the operator in that dialog, each answer carrying the operator's note that it is the founder's own word, given live in
the overseer's dialog at 2026-10-14T07:31:48Z. The answers and notes below are verbatim.

## Fork 1 — the cut
Question: Fork 1 (the cut; the founder's own word). Research found that `send`'s single confirm window is 10 s
(`CONFIRM_WINDOW_FALLBACK`). The one measured `/compact` post-condition arrived at 18.8 s, on a 2-turn session, and
compaction time grows with the session (unmeasured). So a `/compact` row plus a claim arm still ends exit 13 unless
`/compact` gets a window of its own. Which shapes does THIS chunk build?
Options, in order: Resume row; /compact listed (Recommended) · Resume row; /compact owed · Both rows, /compact window ·
/compact row only.
The recommended option as shown: builds the `--resume` ledger row. `/compact` joins the local-command list with no
post-condition, so a driver's send reads `ok unconfirmable` at once instead of a false exit 13. That needs no row or
probe (the ruled limit: a shape no send relies on needs no probe), no live compaction and no new bound. The `/compact`
post-condition row stays owed. Trade-off: `/compact` is never read back.
Answer: "Resume row; /compact listed (Recommended)"
Note: The founder's own word, live in the overseer's dialog at 2026-10-14T07:31:48Z: «Resume; /compact в список».

## Fork 2 — the resume probe (a boundary widening of the verify child set, and a new id source on a child's argv)
Question: Fork 2 (a boundary widening of the founder-ratified verify child set, his own word; applies if the `--resume`
row is built). How does `viola verify` probe `claude --resume <id>`? The session that holds turns is Run B's FIRST id
(after `/clear` the session is new and empty). That id would be read from Run B's own first SessionStart capture and
put on a child's argv. That is a new id source beside the ratified logged id or `--id`, so it is a crossing too.
Options, in order: Run E: a sixth start (Recommended) · Run C resumes Run B · No widening.
The recommended option as shown: after Run D, a new Run E in its own 0700 dir: `--resume <Run B's first id>`
(`is_session_id`-checked, built by `resume_args`, one argv element of a direct spawn under the R8 strip). It waits for
a SessionStart of source `resume` with that id, settles, and is killed with no key. +1 start per round, a seventh spawn
pair (`verify-pty-probe`), the stop and dir-guard rules as Runs A–D. No ratified run changes shape.
Answer: "Run E: a sixth start (Recommended)"
Note: The founder's own word, live in the overseer's dialog at 2026-10-14T07:31:48Z: «Run E, шестой старт» — the
widening ratified by him.

## Fork 3 — the live-start cap
Question: Fork 3 (the founder's own word): the cap of live `claude` starts for this chunk. One `viola verify` round is
five starts, six with a Run E. Recording the new variant(s) and re-stamping the tmpfs `--local-live` home are two rounds
when both are wanted (the previous chunk: two rounds, ten starts).
Options, in order: Twelve starts (Recommended) · Six starts · Fifteen starts · None.
Answer: "Twelve starts (Recommended)"
Note: The founder's own word, live in the overseer's dialog at 2026-10-14T07:31:48Z: «Двенадцать» — cap 12 live
starts for this chunk.
