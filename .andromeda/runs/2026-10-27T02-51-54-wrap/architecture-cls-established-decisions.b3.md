# Classified — architecture · established-decisions.b3

## Answers
established-decisions#22.1 · no · the ledger rows, the relied set and the fake agent's --version answer are unchanged, and the fake agent's only change is its hook receipt, which this row does not describe
established-decisions#23.1 · no · verify's check_home_tree and check_stamps calls and their refusal shape are unchanged and no src/cmd/ file changed, the reached strict.rs change being the plain-home test oracle whose product verdict was already right
established-decisions#24.1 · no · the start order (pinned copy and plugin before the sideload) and the liveness rules are re-read unchanged, and run still pins the exe bytes it checked
established-decisions#24.2 · no · the strict-modes instance check that every frame client runs first and the server identity check are unchanged, and strict.rs's product verdict stands
established-decisions#24.3 · no · run's collision check and its heartbeat-only decision on a replayed snapshot are unchanged, and the copy placement comes after that check
established-decisions#24.4 · no · revive's single snapshot read and its preflight are unchanged, and no revive or replay path is touched
established-decisions#24.5 · no · the home-tree check and its refusal shape are re-read unchanged, run still pins the exe bytes it checked, and the copy is placed only after the check
established-decisions#25.1 · no · no error variant is added, place_private_new returns the existing StateError, PinError::HashMismatch is reused, and Placed is a success value rather than an error enum
established-decisions#26.1 · no · no crate, dependency or feature is added or removed, no Tokio edge moves and viola-state stays sync
established-decisions#27.1 · yes · add-to · the absent-copy write is now the no-clobber placement place_private_new (F1), because a concurrent exec of a file open for write fails ETXTBSY (F2), and measurement B (F5) shows one tree can build two files, so one tree can key two folders
established-decisions#28.1 · no · pin_bytes' signature, its pins-bytes-already-read contract and its callers pin_exe and pin_and_plugin are unchanged, and the copy's placement belongs to the copy rule, not this row
established-decisions#28.2 · no · pin_companions' check_present before the held open and its strict-modes fallback are unchanged, and its companions still write through replace_private_shared
