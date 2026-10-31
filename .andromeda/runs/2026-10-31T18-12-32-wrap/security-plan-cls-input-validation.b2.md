# Classified — security-plan · input-validation.b2

## Answers
input-validation#1.20 · no · the statusline shell-out's rule lives in src/cmd/hook.rs and src/cmd/hook/statusline.rs, neither reached nor in the diff, and hook stays silent at the catch site as before
input-validation#1.21 · yes · stale · src/main.rs and src/cmd/run.rs are reached whole, so the second-statement-of-main restrict_dll_search and run's conpty_sideload step order can have moved, and the report does not show them unchanged
input-validation#1.22 · no · N/A boundary with no upload, webhook or database surface, and the session reaches none; F10 and F11 belong to revive's rows, not this one
input-validation#2.0 · yes · stale · MAX_FRAME's file crates/viola-core/src/lib.rs and the reader files (send, answer, list, revive, verify, viola-mcp) are reached whole, and the report does not show their caps and claude-agents reads unchanged, with `oversize` absent from revive's new code list
