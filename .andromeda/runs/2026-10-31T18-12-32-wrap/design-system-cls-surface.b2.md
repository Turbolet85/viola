# Classified — design-system · surface.b2

## Answers
surface#15.1 · no · exit 0's success documents and result lines are outside the session's change; no success path moved
surface#15.2 · yes · contradict · the cell's rule that every other exit 1 has no `--json` document no longer holds: revive `--json` now prints a document for each start refusal (F8, F9) and the catch site prints `internal-error` under `--json` (F11)
surface#15.3 · yes · stale · the session retired a NOT_ERROR_BODIES row in the reached file tests/contract_error_bodies.rs, and the report does not show the exit-2 rows still held there
surface#15.4 · no · exit 10's refusal object; the session reached no exit-10 path
surface#15.5 · no · exit 11's refusal object; the session reached no exit-11 path
surface#15.6 · no · exit 12's refusal object; the session reached no exit-12 path
surface#15.7 · no · exit 13's refusal object; the session reached no exit-13 path
surface#15.8 · no · exit 14's refusal object; the session reached no exit-14 path
surface#15.9 · no · exit 20's wrapper-fault document; the session moved no exit-20 path
surface#15.10 · yes · contradict · its `detail: null` on exit 21 is no longer true: the five causes now sit in `detail` (F1, F2, F3, F7), and the endpoint-lost hint (F5) joins the hint list
surface#16.1 · no · link's and unlink's exit-0 success documents; the session moved their refusal documents, not their success shape
surface#16.2 · no · exit 20 on a refused link or unlink pair; the session moved no exit-20 path
surface#16.3 · no · exit 20's closed `detail` codes from viola_channel; the session reached none of them
surface#16.4 · no · exit 20's out-of-five wrapper-fault body; the session reached it nowhere
surface#16.5 · yes · add-to · the CLI `--json` failures this file drives gain revive's refusal documents (F8) and the catch site's `internal-error` document with stderr empty (F11)
surface#17.1 · yes · contradict · the server-verification cause's `--json` document reads `detail: null`, but a failed server verification now reads `server-verify-failed` (F4)
surface#18.1 · no · the help verb grouping; the session added and moved no verb
