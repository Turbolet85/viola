# Classified — arch · established-decisions.b1

## Answers
established-decisions#1.1 · yes · Tokio containment binds: the hook and CLI verification stays std and blocking, so the check adds no tokio to the sync crates or to viola-channel's tokio-free graph
established-decisions#2.1 · no · the chunk reaches no PTY code: the scope lists the PTY seam and ConPTY sideload as not needed, and run.rs is re-read only for its collision-check snapshot read
established-decisions#3.1 · no · the vt100 screen model and readiness gate are wrapper-side, the chunk's clients read no screen, and the scope keeps wrapper-side send behaviour unchanged
established-decisions#4.1 · yes · the client send path keeps refusal_of's empty-text refusal (exit 13) ahead of the new check and any frame, sends the typed text unchanged, and fits the MCP tool-call bound with the check's time added
established-decisions#5.1 · yes · the SessionEnd direct-append fallback that a refused hook.event must still reach is try_append_event, whose heal and held-lock rules hold, and the check's snapshot.json read relies on atomic snapshots
established-decisions#6.1 · yes · on Windows a reader holding snapshot.json open is the case replace_private's bounded persist retry meets, and the hook snapshot reads the chunk edits and the exiting-wrapper window it re-reads at P3 are such readers
established-decisions#7.1 · yes · the Windows clients keep the SQOS Identification open for the pre-check's connect, the connected endpoint must be the snapshot's endpoint, and a failed check reads as the instance-unreachable outcome
established-decisions#8.1 · yes · the MCP send, wait, last and answer tools gain the pre-check and return instance-unreachable as an isError result with codes only, while release stays CLI-only and its -32602 from refusal is unchanged
established-decisions#9.1 · yes · the hook.event path must fit the spine deadline and the connect deadline, and SessionEnd's 1.5 s budget keeps its direct-append fallback for a refused or failed hook.event
established-decisions#10.1 · yes · viola hook must fail open on a mismatch (exit 0, no body, nothing on stdout or stderr, no frame), and the check's time before the dialog reply read must stay inside the 3615 s timeout margin
established-decisions#11.1 · no · the dialog-expired line is the wrapper's append at the hold's end, and a refused hook.dialog writes no frame, so the wrapper's hold, wake and expiry are unchanged
established-decisions#11.2 · no · a recorded live measurement of a 671 s hold that the chunk neither re-measures nor changes, since its pre-check runs before the hold
established-decisions#12.1 · no · a pending live reading of the hold near the 3600 s ceiling, owned by the Version done-check and Windows-only live measurements entries, and the chunk changes no ceiling
