# layouts extract

## No domain coverage
This chunk is a behaviour-preserving cleanup of Rust files, harness, mutation tests and scripts. It creates or changes no web-spa region, component placement, focus order, breakpoint or modal. It also creates or changes no cli output structure, since scope Boundaries rule out any product behaviour change. The only cli-adjacent file is the `src/cmd/run.rs` tracing-capture dedupe. The `viola run` passthrough prints nothing while the child runs (per layout-templates §Surface: cli › Output structure — `viola run`). The dedupe keeps that true only if no output reaches the terminal, and whether the refactor keeps it that way is a question for research and obs, not a layout mandate.
