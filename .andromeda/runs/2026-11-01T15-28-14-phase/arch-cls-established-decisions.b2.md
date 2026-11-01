# Classified — arch · established-decisions.b2

## Answers
established-decisions#19.1 · yes · the hook's `Client::connect_by` connect gains the client euid check before the first frame and must stay inside the hook's 750 ms CONNECT_DEADLINE; the exec-form tiers are untouched
established-decisions#20.1 · yes · the event and dialog connect (replay read, `Client::connect_by`, `verify_server`) gains the client euid check between connect and the first frame, and a refusal there must write no frame
established-decisions#21.1 · yes · hooks fail open: a directory or euid refusal reaching `viola hook` exits 0 with no body and empty stderr, exit 2 never; the log home it names is standing S4 and not carried
established-decisions#22.1 · no · the wrapper's `dialog-expired` line at the hold's end is a wrapper line that the chunk's connect and accept checks do not reach
established-decisions#22.2 · no · a live measurement of a 671 s dialog hold; the chunk re-measures only takeover, the root kill probe and cleanup
established-decisions#22.3 · yes · a dialog hook whose euid or server check fails sends no hook.dialog and exits 0 at once with no body, the rule the new hook-side euid refusal must follow
established-decisions#22.4 · yes · the hook's instance check (`check_instance`) runs before obs init, config, stdin and any log, the order the chunk's strict-mode checks on the hook path must keep
established-decisions#23.1 · no · the live reading of the long dialog hold and its Windows leg; the chunk does not change the hold or its ceiling
established-decisions#24.1 · no · the wheel and stdin classifier act on bytes and frames after the socket checks; the chunk's checks act at the socket ahead of any typed byte, and the PTY seam is not reached
established-decisions#25.1 · yes · `viola link` and `viola unlink` gain the client euid check before their first frame, and a send or answer from a refused client records no link
established-decisions#25.2 · yes · the explicit verbs' order (check_instance via refuse_untrusted, exit 21 on refusal, then live_endpoint, server identity check, one request) is where the chunk's link-verb euid check sits
established-decisions#25.3 · no · the wrapper writes no span or line for link/unlink, and the chunk adds neither a link span nor a wrapper line
established-decisions#25.4 · no · `LinkSet::line` stamps the link `ts` with `Utc::now()`, which the chunk does not touch
established-decisions#26.1 · no · the budget governor and budget.json are stated unchanged by the chunk's Not needed, and budget.json is not a reached file
