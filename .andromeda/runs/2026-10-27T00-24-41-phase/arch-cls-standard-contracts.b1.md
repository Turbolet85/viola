# Classified — arch · standard-contracts.b1

## Answers
standard-contracts#1.1 · no · the list payload is Not-needed (no board field changes) and its items carry no pinned path or hash
standard-contracts#2.0 · yes · it names `pinned_bin`, absent on a replayed snapshot, and a `writer` that is the binary version, both read by P3 for hash keying; its atomic replacement runs through `replace_private`, which the fix edits
standard-contracts#3.0 · yes · the stamps envelope keyed per CLI version (`data.versions`, writer `verify`) is read by P3 to see whether the stamps readers key on the hash; its atomic replacement runs through `replace_private`, which the fix edits
standard-contracts#4.1 · no · the stamps write's strict check and lock run after verify's `pin_exe` call, the only verify edit, and the stamps write is untouched
standard-contracts#5.1 · no · links are Not-needed, and the restore neither appends a line nor touches the pin write
standard-contracts#5.2 · no · links are Not-needed, and the set's changes on implicit and served link calls are untouched
standard-contracts#5.3 · no · links are Not-needed, and the `already_linked` answer is untouched
standard-contracts#5.4 · no · links are Not-needed, and the `unlink` answer is untouched
standard-contracts#5.5 · no · links are Not-needed, and the `-32602` and `-32603` cases of `link` and `unlink` are untouched
standard-contracts#6.1 · no · the wrapper channel's per-client server identity check is Not-needed, since the race fix sends no frame
standard-contracts#7.1 · no · the frame clients' instance check is the wrapper channel's and Not-needed, while the chunk's home check is `check_home_tree`
standard-contracts#8.1 · no · channel method `send` is Not-needed, so delivery, the budget gate and the cursor are untouched
standard-contracts#9.1 · no · channel method `wait` is Not-needed, so the wake rule and the dialog-first rule are untouched
standard-contracts#10.1 · no · channel methods `link` and `unlink` are Not-needed, so their `from` rule is untouched
standard-contracts#11.1 · no · the `hook.event` frame and its append are Not-needed, and the race fails the spawn before any frame is sent
standard-contracts#12.1 · no · `list` is Not-needed (no board field changes, the agents join untouched) and its items carry no pinned path or hash
standard-contracts#13.1 · no · event data per kind is Not-needed, since the fix appends no event kind and the send-text matching is untouched
standard-contracts#14.1 · no · the `link` and `unlink` `--json` documents belong to links, which are Not-needed
standard-contracts#15.1 · no · the `instance-unreachable` `--json` documents of the frame verbs are Not-needed, since no refusal code changes
standard-contracts#16.1 · no · the hook statusline contract is Not-needed and the reached files hold no statusline code
standard-contracts#17.1 · no · session liveness and the server identity read are Not-needed, with heartbeat, pid and start time untouched
standard-contracts#18.1 · no · the `claude agents --json` join is Not-needed and no probe run changes
