# Classified — security · input-validation.b2

## Answers
input-validation#1.20 · no · the statusline shell-out is outside the chunk: the scope excludes it ("no exit-21 or exit-1 cause runs through it") and `src/cmd/hook.rs` is not reached
input-validation#1.21 · no · the DLL search order and ConPTY sideload are excluded by the scope ("a sideload failure degrades silently and is no exit cause"); `pin_companions` and the `sideload_fallback` list are neither reached nor named
input-validation#1.22 · no · no upload, webhook or database surface in the chunk's reach
input-validation#2.0 · yes · the `claude agents --json` read's `take(MAX_FRAME + 1)`, 5 s kill and closed `oversize` bind where `list` and `revive`'s preflight read it, since the scope names that read's fail-open rule; the other readers are not reached
