# Operator entry 3 — the measurement-only compile + lint, fired once by hand at /implement

- run: `cargo clippy -p viola-pty --all-targets --features h2-measure -- -D warnings`
- fired: 2026-09-29T10:26:58Z, by /implement (session `2026-09-29T08-52-03-implement`), tree = HEAD `fb78ddc` + this chunk's working changes
- exit: 0 · atoms: `exit 0` held · warnings 0, errors 0
- valid only until the removal commit (plan step 9); the wrap never re-runs it
