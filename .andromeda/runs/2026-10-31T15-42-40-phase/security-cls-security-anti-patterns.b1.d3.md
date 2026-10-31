# Classified — security · security-anti-patterns.b1

## Answers
security-anti-patterns#1.1 · yes · the frame ban binds the strict-modes check and server verification before the first frame, which the chunk edits in `refuse_untrusted`, the server check and the client's order on every method
security-anti-patterns#2.1 · no · the `run --mutants` wipe guard is another job of `src/cmd/run.rs`; mutation tooling is not touched (Not needed)
security-anti-patterns#3.1 · no · standing S6 · a test-side removal under `target/e2e-home` takes only a dir the test side itself created
security-anti-patterns#4.1 · no · the `hook.dialog` stamp gate is not a unit the chunk edits; dialog answering is not a surface it changes (Not needed)
security-anti-patterns#4.2 · yes · `viola verify` edits its refused home tree, so its strict-modes check must run before any stamps read and before any log opens, and a refusal reads and writes nothing
security-anti-patterns#5.1 · yes · the chunk edits the strict-modes check and the server check, and the colour-fact rule reaches the `viola list` refusal it edits; no config, env or flag may switch those controls off
