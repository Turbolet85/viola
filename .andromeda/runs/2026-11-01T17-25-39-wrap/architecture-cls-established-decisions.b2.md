# Classified — architecture · established-decisions.b2

## Answers
established-decisions#19.1 · no · the tier map, SPINE_DEADLINE and the 1.5 s SessionEnd budget do not move; the two new checks sit in the timed path, and the entity states no measurement of it
established-decisions#20.1 · yes · add-to · the shared hook connect gains check_endpoint_dir before Client::connect_by and a peer_admitted check beside verify_server, with an unreadable euid refused where reported (F3, F4) and the checks sitting at the caller, not inside Client::connect (F9)
established-decisions#21.1 · yes · add-to · a hook meeting a refused socket directory still exits 0 with empty streams and no frame, and writes one hook-decision line at WARN with detail socket-dir-refused (F6)
established-decisions#22.1 · no · the dialog-expired append is the wrapper's own expiry at the hold's end, which the session does not reach
established-decisions#22.2 · no · the measured 671 s held dialog is evidence of the hold, which neither new check alters
established-decisions#22.3 · yes · add-to · a refused euid and a refused socket directory also send no hook.dialog and exit 0 at once with no body, and an unreadable euid is a check that cannot be made (F4, F6)
established-decisions#22.4 · no · the instance check still runs first, before the diagnostics level and obs init, and a refused instance stays silent; the socket-directory check runs later, inside connect
established-decisions#23.1 · no · the hold's ceiling and its pending live read are not touched by the session
established-decisions#24.1 · no · the wheel's classifier, stdin observer, send refusals and running-turn rules live in src/run files the session does not reach, and viola run still starts the wheel as driver
established-decisions#25.1 · no · the links decision (recording, set restore, the wrapper serving link and unlink) holds; the new checks refuse before a link is sent and change no recorded line
established-decisions#25.2 · yes · add-to · the explicit verbs' server identity check through cmd::client::connect gains the endpoint directory check and the peer euid check before the request (F3), and a refused socket directory is exit 21
established-decisions#25.3 · no · the wrapper's own span and process-log silence for link and unlink still hold, and a refused socket directory never reaches the wrapper
established-decisions#25.4 · no · LinkSet::line's Utc::now stamp of the pair's since is untouched
established-decisions#26.1 · no · the budget gate, its evaluation points, order and refusal path are wrapper-side and unchanged; the new client-side check refuses before a send reaches the gate
