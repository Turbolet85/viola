Check 4 (9) must-pass control for gate entry 24 (`cargo tree --manifest-path fuzz/Cargo.toml --locked -e normal
--prefix none -i veil`), fired AS THE ENTRY through `gate.py run --chunk 2026-11-14-hyprland-seam --docs-root .
--entry 24 --project-root $TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/bc8a9f8b-5b9c-455a-8afa-3506b0730cd5/scratchpad/clone-lock`
over a scratch clone (`git clone --shared` at HEAD 4255b3ed), 2026-11-14:

- unmodified: `24 probe red · exit 0 ✗ (exit 101) · exit 101` — the same reading as the tree's baseline.
- input: the clone's `fuzz/Cargo.lock` refreshed by `cargo tree --manifest-path fuzz/Cargo.toml -e normal --prefix
  none -i veil` without `--locked` (it lists `veil v0.3.0` under `viola-agent-claude`), the file then reading
  ` M fuzz/Cargo.lock` in the clone.
- with the input: `24 probe green · exit 0 · 0.08s` — the literal `veil v` is the form the run's own output prints.

The input lived in the clone only; the tree's `fuzz/Cargo.lock` stands as committed (restored with `git checkout`
after the baseline run of `agent-run.sh run --fuzz-replay` had rewritten it).
