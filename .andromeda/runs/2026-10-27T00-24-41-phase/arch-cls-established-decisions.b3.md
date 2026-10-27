# Classified — arch · established-decisions.b3

## Answers
established-decisions#22.1 · yes · verify's Run B statusLine spawns the pinned viola path, so the pin_exe call the chunk edits must finish before verify's first probe
established-decisions#23.1 · yes · verify's check_home_tree over home, ledger/, bin/ and plugin/ runs before its cli log, so the pin_exe write the chunk edits must stay behind that check
established-decisions#24.1 · yes · run's start order puts the pinned-copy and plugin step (the edited pin_bytes call and per-start plugin rewrite) after the collision check and before the ConPTY sideload, first snapshot and spawn
established-decisions#24.2 · no · the race fix sends no frame and changes no method, and the server identity check is on the task's Not needed list
established-decisions#24.3 · yes · with no snapshot the collision check reads free, so the scope's two first runs on a fresh home both reach the pin the chunk guards
established-decisions#24.4 · no · revive's preflight read is untouched and not on the chunk's path; a revived start reaches the pin through run's start, which 24.1 covers
established-decisions#24.5 · yes · every start's home-tree check, before its log, covers bin/<key>/, the pinned copy and plugin/<key>/ on the exe bytes run then pins, so the fix pins those same bytes and passes its new entries through that check
established-decisions#25.1 · no · standing S7 · a new error type follows the per-crate <Crate>Error thiserror enum with fixed messages, a convention the task lists
established-decisions#26.1 · yes · viola-state stays a sync crate with no tokio in its normal graph, so the fix's code uses std threads and blocking I/O only (the Needs' Tokio containment)
established-decisions#27.1 · yes · the write-if-absent pinned copy, its re-hash, the per-start plugin rewrite and the content-hash key are what the edited start and the reproducible-build measurement rest on
established-decisions#28.1 · yes · pin_bytes and pin_exe are the edited absent-copy units, and run's start pins the same bytes it home-checked under pin_key
established-decisions#28.2 · yes · pin_companions is an edited unit of run's start, and its Windows owner/DACL check must stay ahead of the held open
