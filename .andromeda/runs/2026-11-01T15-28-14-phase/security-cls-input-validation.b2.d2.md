# Classified — security · input-validation.b2

## Answers
input-validation#1.19 · yes · the home strict-modes check must run before run's first read under the home, which is this run-start read of `<home>/statusline-source.json`; its take(MAX_FRAME) bound is untouched
input-validation#1.20 · yes · the hook statusline arm reads stdin and the snapshot and writes budget.json under the home, so the new home check must precede them, and a refusal there is a fail-open that runs no shell-out and prints no body
input-validation#1.21 · yes · the scope names the Windows strict-modes reading as not moving, and pin_companions' check_present on the conpty directory is that reading, which the chunk leaves unchanged
input-validation#1.22 · no · the N/A row for upload, webhook and SQL, which the task's Not needed list drops as no such surface
input-validation#2.0 · no · the MAX_FRAME cap is in Not needed: the new checks run before any frame, add no read to bound, and the bounded-reader census gains no product reader
