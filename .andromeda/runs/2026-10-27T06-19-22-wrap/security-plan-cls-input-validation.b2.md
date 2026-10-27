# Classified — security-plan · input-validation.b2

## Answers
input-validation#1.20 · no · its rule lives in unreached viola-pty sideload, viola-state pin::pin_companions, src/conpty.rs, src/main.rs and src/cmd/run.rs; the reached fs.rs changed only replace_private_shared's compare read, its callers and write path unchanged
input-validation#1.21 · no · the session adds no upload, webhook or database surface, so the N/A claim holds
input-validation#2.0 · yes · add-to · the list of reads MAX_FRAME bounds gains viola mcp's stdin lines (F1: capped at MAX_FRAME before rmcp sees them; F2: an over-cap line refused whole, ending the session) and replace_private_shared's compare read (F4: take(bytes.len() + 1))
