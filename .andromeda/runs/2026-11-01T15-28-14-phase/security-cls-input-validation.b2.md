# Classified — security · input-validation.b2

## Answers
input-validation#1.19 · yes · the `<home>/statusline-source.json` read at `run` start is a read under the home, so the home strict-modes check must run before it; the default-home and `VIOLA_DIR` conditions bound that check's scope
input-validation#1.20 · yes · the `hook statusline` arm's order (instance check, then snapshot read, then shell-out) and its silent output sit where the home check and the fail-open refusal reach `hook`
input-validation#1.21 · no · Windows DLL search order and the ConPTY companion pin are excluded by the scope; the Windows strict-modes reading does not move
input-validation#1.22 · no · N/A, no upload, webhook or database surface
input-validation#2.0 · no · the `MAX_FRAME` cap and frame shape are excluded by the scope, and the chunk adds no unbounded product reader for the census to record
