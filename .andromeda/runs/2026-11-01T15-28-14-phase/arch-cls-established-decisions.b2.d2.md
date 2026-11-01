# Classified — arch · established-decisions.b2

## Answers
established-decisions#19.1 · yes · the hook's sync connect under CONNECT_DEADLINE gains the new directory and euid checks, and a refusal there fails open with a diagnostics record and exit 0
established-decisions#20.1 · yes · the hook's shared connect (snapshot read, Client::connect_by, verify_server) gains the client-side euid check before the first frame, and a refusal there is a server-verify failure with no frame
established-decisions#21.1 · yes · every new refusal reaching viola hook must exit 0 with no body and no stderr, never exit 2
established-decisions#22.1 · no · the dialog-expired append at a hold's end is wrapper-side dialog behaviour that the connect and accept checks do not reach
established-decisions#22.2 · no · a measured 671 s live dialog hold; the chunk re-measures no hold, only socket takeover, the root connect probe and the cleanup reading
established-decisions#22.3 · yes · a dialog hook whose server check fails sends no hook.dialog and exits 0 with no body; the chunk's euid refusal on the hook's connect must follow the same fail-open
established-decisions#22.4 · yes · the hook's instance check runs before any diagnostics, obs or config read, so the chunk's new hook-side directory and euid checks must sit after it and write no line on refusal
established-decisions#23.1 · no · a pending live measurement of the dialog hold ceiling, which the chunk does not change
established-decisions#24.1 · no · wheel byte classification and human-typing refusals; the chunk's checks act at the socket before any typed byte
established-decisions#25.1 · yes · the explicit link and unlink verbs gain the client euid check before their first frame, so a refusal there records no link or unlink line
established-decisions#25.2 · yes · explicit viola link and unlink run check_instance through client::refuse_untrusted before client::start with refusal as exit 21; the new euid check joins the server identity check in cmd::client::connect
established-decisions#25.3 · no · the wrapper's span and log silence for link and unlink, which the chunk's refusals do not change
established-decisions#25.4 · no · the link line's ts is stamped by Utc::now outside the Clock seam, which the chunk does not touch
established-decisions#26.1 · no · the budget gate reads at send and turn points, which a connection refused before the first frame never reaches; the task's Not needed lists the governor as unchanged
