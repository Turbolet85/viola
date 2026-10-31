# Classified — arch · standard-contracts.b1

## Answers
standard-contracts#1.1 · yes · the `wrapped: false` row is never a valid `target` and a send to it exits 21, the unwrapped-name cause and its hint
standard-contracts#2.0 · yes · a replayed snapshot carries no `endpoint` or `pid`, so the instance reads as unreachable; heartbeat-alone collision judgement and `cwd-missing` on revive
standard-contracts#3.0 · no · the stamps envelope and merge rules are the ledger file's own; run's gate records `cli_verified:false` and is no exit cause, and the chunk reads or writes no stamps field
standard-contracts#4.1 · no · `update_stamps`' strict check lives in viola-state, which the reached files do not include; verify's refused home tree is `check_home_tree`, not this check
standard-contracts#5.1 · no · the links set restore and its log-following rule are the wrapper's own; the chunk adds no link state and the link/unlink refusal sits before any log
standard-contracts#5.2 · no · the points where the set changes (implicit link, served link/unlink) are unchanged by the chunk
standard-contracts#5.3 · no · the success answer for an already-held pair (`already_linked: true`) is unchanged; the chunk touches only the refused path
standard-contracts#5.4 · no · unlink's `{}` answer is unchanged; the chunk touches only the refused path
standard-contracts#5.5 · no · the -32602 and -32603 answers of link and unlink write no line and are neither changed nor mapped to a code by the chunk
standard-contracts#6.1 · yes · the frame-writing client's peer pid and OS start-time check against snapshot `pid` and `started_at`, a mismatch writing no frame; the server-verification cause with the macOS gap kept
standard-contracts#7.1 · yes · the strict-modes check runs before the peer comparison and a refusal writes no frame and no line; `client::refuse_untrusted` is edited and its order ahead of any log binds
standard-contracts#8.1 · no · send's success shape and its budget-gate and link rungs are unchanged; the unwrapped-target refusal is not stated by this entity
standard-contracts#9.1 · no · wait's argument rules and wake set are unchanged by the chunk, and this entity does not state wait's strict refusal
standard-contracts#10.1 · no · link and unlink ignoring `from` is unchanged by the chunk
standard-contracts#11.1 · yes · the endpoint-lost read in viola-channel's client must not become a `ProtocolError` carrying one of the three `-32602` causes; it stays apart from the exit-20 protocol fault
standard-contracts#12.1 · yes · hook.event sends nothing and the hook fails open on a failed server check, the shared server check the chunk edits; the hook contract stays a bound
standard-contracts#13.1 · yes · `list`'s board refusal is `instance-unreachable` on MCP, and CLI list keeps its order of scan, log, read, document, so its strict refusal writes no line
standard-contracts#14.1 · no · `prompt-submitted` event data is not touched and the chunk adds no event field
standard-contracts#15.1 · yes · the exact exit-21 document and empty stderr of the `viola link` and `viola unlink` refusals, which the chunk gives a code
standard-contracts#16.1 · yes · the exit-21 document and empty stderr of the send, wait, last, answer, pause and release refusals under `--json`
standard-contracts#16.2 · yes · `sanitised_fault` is the one fault reader the scope names, and the code words crossing a client read go through it
standard-contracts#16.3 · yes · the out-of-five fallback of `sanitised_fault` is the reader rule that the vanished-read and new codes crossing a client read must keep
standard-contracts#17.1 · yes · the hook contract as a bound: statusline's refused check exits 0 with nothing printed and no stderr, and no chunk code reaches it
standard-contracts#18.1 · yes · `live_endpoint`, the liveness pre-check and `verify_server` are the server identity the chunk edits, with the macOS peer-pid gap kept
standard-contracts#19.1 · yes · revive's `claude`-not-on-PATH internal-error and the agents read's join in `list` (never an error) both rest on this resolution and read rule
