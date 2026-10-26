# Classified — design-system · surface.b1

## Answers
surface#1.1 · no · the new link and unlink lines carry no DIALOG word and no amber, so the DIALOG-only rule holds unchanged
surface#1.2 · no · no stale row or dim span is added, since the new result lines are plain text and not board rows
surface#1.3 · no · bold stays on NAME column values and the new result lines carry no SGR, so the callsign rule holds unchanged
surface#1.4 · no · no styled span is added, so the reset rule holds unchanged
surface#1.5 · no · the built link and unlink lines carry no SGR at any colour depth, so the claim that link output is plain holds on the changed files
surface#1.6 · no · the new result lines are plain with no SGR and set no background
surface#2.1 · yes · contradict · link and unlink's exit-21 strict-modes refusal now takes the fixed unable: form, so the "only" list of fixed-form refusals no longer holds
surface#3.1 · no · the new result lines are not tables and the session adds no width or length rule, as the report adds no length
surface#4.1 · no · the board, its columns, the BAY line and the unwrapped section are unchanged and the board shows no pairs
surface#4.2 · yes · add-to · link and unlink's exit-21 strict-modes refusal prints this same instance pair as list's (F5), so this exit-21 group gains two verbs
surface#5.1 · yes · stale · the verbs that write server-verify-failed and take the server-verify hint grow from six to eight with link and unlink
surface#6.1 · yes · stale · the six frame verbs that take the server-verification hint become eight with link and unlink
surface#7.1 · no · the macOS server-check gap is stated for the frame verbs generically, so link and unlink fall under it unchanged and its owner entry is unchanged
surface#8.1 · yes · add-to · link and unlink add one plain stdout line in this verb-line form, a since as UTC HH:MM:SSZ or the escaper when it is no RFC 3339 instant, and the already-linked and unlink-of-unheld forms (F1, F2, F4)
surface#9.1 · no · the report leaves viola verify's output unchanged, so its step lines, relied count and summary hold
surface#10.1 · no · the report leaves viola revive's output and its --json refusal documents unchanged
surface#11.1 · yes · add-to · link and unlink print their one plain result line on stdout at exit 0, and their --json success documents join the {"v":1,"ok":…} shape (F1, F3)
surface#11.2 · yes · add-to · link and unlink add a reply of the wrong shape as a cause of the fixed error: internal error at exit 1, which has no --json document
surface#11.3 · no · clap's exit 2 usage text is generic to every verb's arguments, the two new verbs take it like the others, and the hook is untouched
surface#11.4 · no · human-typing is a send refusal, the report leaves send's lines unchanged, and link and unlink add no wheel refusal
surface#11.5 · no · budget-paused is a send refusal, the report leaves send's lines unchanged, and link and unlink take no budget gate
surface#11.6 · no · unverified-cli is a send refusal, the report leaves send's lines unchanged, and link and unlink add no stamp check
surface#11.7 · no · not-delivered is a send refusal, the report leaves send's lines unchanged, and link and unlink add no such case (their wrapper refusals are exit 20, F6)
surface#11.8 · no · unknown is send's opaque refusal, and the session adds no exit 14 refusal
surface#11.9 · yes · add-to · link and unlink reach error: wrapper fault at exit 20 when the wrapper refuses the pair, such as driver equal to driven, which is never refused client-side (F6)
