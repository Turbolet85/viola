# Classified — architecture · established-decisions.b2

## Answers
established-decisions#19.1 · no · the tiers, SPINE_DEADLINE (1 s) and CONNECT_DEADLINE (750 ms) hold on the changed src/cmd/hook.rs, and the two new checks run inside the same timed connect that the "within the deadline" rule already bounds
established-decisions#20.1 · yes · add-to · F3, F4, F9: the hook's one connect gains check_endpoint_dir before Client::connect_by and peer_admitted after it, no check sits inside connect_by, and a peer whose euid cannot be read is refused where it is reported
established-decisions#21.1 · no · hooks still fail open with exit 0, empty streams and no body, and the refused-socket-directory line is a codes-only diagnostic record rather than stderr, so "never writes to stderr" holds
established-decisions#22.1 · no · the dialog-expired append at the hold's end is unchanged; the session touches only the connect before the hold
established-decisions#22.2 · no · the 671 s held-dialog measurement is a dated evidence record of the hold, which the session does not change
established-decisions#22.3 · yes · add-to · F6: a refused socket directory on the dialog path exits 0 with empty streams and no frame and writes one hook-decision line at WARN, and a refused euid takes the same fail-open form, both belonging beside this entity's log-line note
established-decisions#22.4 · no · the instance check still runs first, before the diagnostics level, obs and any stdin or config read, and a refused instance stays silent with no line; the new socket-dir check sits later in the connect
established-decisions#23.1 · no · the pending live read of the hold ceiling and its PROVISIONAL status are not moved by the session
established-decisions#24.1 · no · the wheel rules, classifier and human-take list live in src/run/wheel.rs, which is not reached; the wheel still starts with driver at run start, and a refused stream moves no wheel
established-decisions#25.1 · no · the driven wrapper still serves link and unlink and records the implicit link on send and answer; the accept filter drops only foreign-euid streams before any frame, so the five recorded shapes stand
established-decisions#25.2 · yes · add-to · F3: the explicit verbs' check order gains check_endpoint_dir ahead of connect and peer_admitted after it, through the cmd::client::connect seam
established-decisions#25.3 · no · the wrapper's no-span, no-line rule and its channel-request/channel-response pair for link and unlink are unchanged; the new checks sit on the CLI side and add no wrapper line
established-decisions#25.4 · no · src/run/links.rs, which holds LinkSet::line and its line-92 citation, is not among the reached files, so the Utc::now stamp and the citation hold
established-decisions#26.1 · no · the gate's evaluation points (start, send, turn-ended), refusal order and budget.json rules live in src/run/budget.rs, which is not reached; the start evaluation still runs after the socket check
