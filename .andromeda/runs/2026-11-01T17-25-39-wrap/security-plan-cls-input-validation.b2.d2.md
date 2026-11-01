# Classified — security-plan · input-validation.b2

## Answers
input-validation#1.19 · yes · stale · the session put the socket-dir check at run's start before any snapshot, and the statusline source read sits in reached src/cmd/run.rs with its place in that start order not shown to hold
input-validation#1.20 · yes · stale · the directory check added at `viola hook` lands in reached src/cmd/hook.rs, which holds this arm's step order and STATUSLINE_DEADLINE, and the report does not show that order holds
input-validation#1.21 · yes · stale · run.rs gains the socket-dir check before any snapshot or child, and the conpty_sideload step lives in that reached file with its place in run's order not shown to hold
input-validation#1.22 · no · the session adds no upload, webhook or database surface, and no reached file holds this row
input-validation#2.0 · yes · stale · the session reached crates/viola-core/src/lib.rs, where MAX_FRAME lives, and the report does not show its claim holds on the changed file
