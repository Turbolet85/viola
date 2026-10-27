# Classified — architecture · established-decisions.b3

## Answers
established-decisions#21.1 · yes · add-to · carries F11 (run.start recovers the snapshot before run.collision_check, and collision_check decides a replayed snapshot from the beat alone) and F12 (revive's preflight reads the snapshot once and hands that read to its collision check and its recorded-directory reading)
established-decisions#21.2 · yes · stale · live_endpoint reads the replay, not read_snapshot, and proceeds only on a real snapshot (F11), and every frame-writing client now runs the strict-modes check before the liveness pre-check
established-decisions#22.1 · yes · stale · viola-state's refusal and error surface moved (Refused gains Readable and Symlink; TreeRefused, StampsError and PinError::StrictModes are returned), so the one-enum-per-crate reading of it is stale, and an unstattable home on Unix is now a strict-modes refusal where the cli catch-site line stood
established-decisions#23.1 · no · the session changed no dependency, crate, Cargo manifest or workspace member, and the crate-layout and tokio-gating rules live in manifests it did not reach
established-decisions#24.1 · yes · add-to · carries F13 (run's start pins the bytes it already checked through pin_bytes, with pin_key naming the key once) and F14 (on Windows pin_companions runs check_present on bin/<key>/conpty/ and each companion before its held open, a refusal running the child on the inbox ConPTY with the exit unchanged)
established-decisions#25.1 · no · the plugin scope, the marketplace install and verify's five interactive runs and print-mode probe keep their loads and dirs, and verify's new home check refuses before its cli log
established-decisions#26.1 · no · revive's closed --json codes, pairs and exit codes hold on the changed revive.rs, and the new home-tree refusal reuses exit 1 and strict-modes-failed
