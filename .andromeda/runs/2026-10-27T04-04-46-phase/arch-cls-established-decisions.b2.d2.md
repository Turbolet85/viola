# Classified — arch · established-decisions.b2

## Answers
established-decisions#17.1 · yes · the hook fail-open rule (oversize or malformed stdin exits 0 with no body and no stderr, exit 2 never) and the capture arms' bounds (stdin through take(MAX_FRAME + 1), --answers through take(64) into ProbeAnswer, the statusline ShellVerdict) bind the chunk's hook reads
established-decisions#18.1 · yes · the hold-end dialog-expired line lives in src/run/dialog.rs, reached whole, and its dialog_kind is the kind the chunk's per-kind answer normalisation keys on
established-decisions#18.2 · no · a record of one live 671 s hold, not a rule over reached code, and the chunk re-measures no hold
established-decisions#18.3 · no · the server identity check it states is Not needed (owned by Unix endpoint and home hardening), and its fail-open exit is already carried by the general hook rule
established-decisions#18.4 · yes · the chunk's capped stdin read on the event and dialog paths must stay behind the instance check, so a refused or absent instance does no stdin read (the check's own mechanics are owned by Unix endpoint and home hardening)
established-decisions#19.1 · no · the live read of a hold near the 3600 s ceiling is owned by Version done-check and its Windows leg by Windows-only live measurements, and the dialog hold's ceiling is Not needed
established-decisions#20.1 · yes · its typed-text rule (every CR LF and lone CR one LF, trailing CR and LF removed, so the in-flight send's exact match holds) is the paste rule re-verified in src/run/send.rs, while the byte-source claim, human-typing and the closed non-editing key list are Not needed
established-decisions#21.1 · yes · the from field (absent, null or a valid ViolaName) and the driver and driven checks (-32602) bind, while the link set's replay, restore and implicit-link timing are Not needed
established-decisions#21.2 · no · the home strict-modes check on the explicit verbs is Not needed (owned by Unix endpoint and home hardening), and the chunk adds no role, log or snapshot read ahead of it
established-decisions#21.3 · yes · its rule lives in src/run/links.rs, reached whole, so the chunk's link and unlink checks must add no span or process-log line and no driver or driven name may reach a log
established-decisions#21.4 · yes · LinkSet::line lives in src/run/links.rs, reached whole, so the chunk's edits there keep its since stamp on Utc::now outside the Clock seam
established-decisions#22.1 · yes · the thresholds read once at start through the cap, budget.json read only through read_budget (cap, tolerant parse, every failure no reading) and send's refusal order (control-character and empty-text before the gate) bind, while the soft-stop pause and lift semantics are Not needed
