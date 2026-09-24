# layouts extract

## Relevance
Partial. The chunk adds no UI surface, region, focusable element, modal or empty state. Its only contact with the layout plan is the cli surface: the new diagnostics sinks and the panic hook must not show up on `viola run`'s terminal or on any verb's stdout/stderr layout.

## Constraints
- `viola run` prints nothing on success, because the terminal belongs to the wrapped claude TUI until it exits. The only allowed output is the exit-1 start refusal plus its `hint:` line. Per layout-templates §Surface: cli → "Output structure — `viola run`". The per-role JSON sink (`run-<name>`) and the migrated `obs_event!` lines must go only to `<home>/diagnostics/` files, never to the TTY. Whether the current `src/run/mod.rs` tracing setup already writes nothing to stdout/stderr is research's question.
- No version line on normal output: the version appears only in `--version` / `--help`. `viola hook` / `viola mcp` have no human surface. Per layout-templates §Surface: cli → "Component — Header / banner" ("No banner anywhere"). So `process-start`'s `service_name`/`version`/`os`/`pid` belong only in the diag line, never echoed to the terminal.
- Stream discipline: results go to stdout; refusals, hints and errors go to stderr; the two are never mixed. Per layout-templates §Surface: cli → "Component — Primary content block 2: refusal lines and the `unable` column" (Streams). Diagnostics output (including panic output and any subscriber-internal errors) must not add lines to either stream that the verb layout does not define.
- A hint or refusal line never names a path or a pid. Per layout-templates §Surface: cli → "Component — Primary content block 2" (Hint line) and "Output structure — `viola run`" ("no paths, no pids"). If a diagnostics or detail file cannot be opened, or a panic is routed to `detail-run.ndjson`, no user-facing line may mention `<home>/diagnostics/...` or the pid.
- Expression level 0.0 for `viola run` passthrough and non-TTY output: no colour, no non-ASCII glyph, no cursor control. Per layout-templates §Surface: cli → "Expression level (this surface)". This lines up with the sink's `with_ansi(false)`. Terminal styling is owned only by the hand-written SGR module in the `viola` bin (per §Surface: cli → "Tooling context"), not by tracing.

## Patterns to follow
- The terminal terminator is the exit code, and the last printed line says what happened. There is no summary banner (per layout-templates §Surface: cli → "Component — Footer / terminator"). Diagnostics add no closing lines of their own.
- History lives in files, not in scrollback chrome: "Terminal scrollback is the history. The tape lives in `events.ndjson`" (per layout-templates §Surface: cli → "Component — Footer / terminator" and §Surface: web-spa → "Component — Primary navigation", the line-cap notice). Diagnostics follow the same model: a file sink with no on-screen surface.

## Anti-patterns to avoid
- No spinner, progress bar or in-place redraw on the CLI (per layout-templates §Decisions Log → cli motion, "no motion"). Nothing in the obs init or the panic hook may write terminal control sequences.
- The page footer and header never carry a version string (per layout-templates §Surface: web-spa → "Component — Header (`<viola-atis>`)" and "Component — Footer"). The service identity introduced here must not be wired to any display surface.

## Contract bindings
- layouts ↔ obs: obs-plan §3 file location / logging stack (file-only per-role sinks) is what keeps layout-templates §Surface: cli → "Output structure — `viola run`" ("prints nothing") true.
- layouts ↔ security: the ban on printing paths or pids in hint/refusal lines (§Surface: cli → Primary content block 2) complements security-plan §Bootstrap `logging-redaction-wire`. Panic payloads go to the 0600 detail file, not to stderr prose.
- The harness `logs` command (item 7) is a test/agent tool and is not a product surface in layout-templates. It has no layout binding.

## Acceptance criteria contributions
- (layouts) A successful `viola run <name> -- <cmd>` with `diagnostics_level` set to `"info"` and to `"debug"` writes zero bytes of viola-originated output to stdout/stderr. All `obs_event!` lines land only in `<home>/diagnostics/run-<name>.ndjson` (per layout-templates §Surface: cli → "Output structure — `viola run`").
- (layouts) No `process-start` identity field (`version`, `pid`, `os`, `service_name`) and no diagnostics or detail file path appears on the terminal during a normal run or an exit-1 start refusal (per layout-templates §Surface: cli → "Component — Header / banner" and "Component — Primary content block 2" Hint line).
- (layouts) Diagnostics output contains no ANSI/SGR escape bytes (per layout-templates §Surface: cli → "Expression level (this surface)", 0.0 for `viola run` passthrough).

## Relevant amendment history
(none): `D:\dev\projects\viola\.andromeda\layout-templates-amendments.md` does not exist, so the history is empty.
