# Classified — obs · obs-anti-patterns.b1

## Answers
obs-anti-patterns#1.1 · yes · the chunk's edited `refuse_squatted` (src/cmd/run.rs) and budget log-capture helper sit on the `run` path, so the squatted refusal writes through `src/human.rs` `refuse` and no print macro, `dbg!` or local `#[allow]` may enter
