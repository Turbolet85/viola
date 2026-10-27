# arch · cascade read · group 1 of 5

## Carried
- (.andromeda/architecture-detail/stack-and-technologies.md:10) Any fuzz target or corpus the proof adds or extends must build only in the separate `fuzz/` workspace on its own pinned nightly, never on the root toolchain.
  > The separate `fuzz/` workspace alone builds on `nightly-2026-09-20`, pinned by `fuzz/rust-toolchain.toml`
- (.andromeda/architecture-detail/stack-and-technologies.md:32) The fuzz targets the chunk may extend must keep cargo-fuzz CI-only and the fuzz lockfile's own pins outside the root `cargo deny` graph.
  > cargo-fuzz 0.13.2 (CI-only, `cargo install --locked` in the fuzz jobs), whose `fuzz/` workspace pins libfuzzer-sys `=0.4.13` and arbitrary `=1.4.2` in its own lockfile, outside the root `cargo deny` graph (a test-only exemption, see Build system)
- (.andromeda/architecture-detail/stack-and-technologies.md:11) Every reader the chunk bounds on the `run`, `hook`, `send` and channel-server paths must stay on std threads and blocking I/O, with a Tokio runtime built only inside `mcp`.
  > Hybrid: std threads for `run`, `hook`, `send`, `wait` and every other CLI verb; Tokio 1.53.3 built only inside `mcp` | No async runtime on the hot `hook` path.
  (.andromeda/architecture.md:444)
  > A sync std-thread core; Tokio 1.53.3 only in `mcp`;
- (.andromeda/architecture-detail/stack-and-technologies.md:15) The frame-read bound must hold on both client flavours, with the Tokio flavour confined to `viola-channel`'s feature-gated `TokioClient` used by `viola-mcp` alone.
  > (sync API; the Tokio flavour, `interprocess/tokio`, only in `viola-channel`'s feature-gated `tokio_client::TokioClient`, used by `viola-mcp` alone,
- (.andromeda/architecture-detail/stack-and-technologies.md:25) The channel frame cap must be measured over one ndjson line of a JSON-RPC 2.0 frame, the framing every channel side reads.
  > None. Per-wrapper local sockets (JSON-RPC 2.0 over ndjson) for request/response
- (.andromeda/architecture-detail/stack-and-technologies.md:14) The re-read panic degrade must keep the feed thread a consumer of a copy of the pump's output handed after the human's write, so a screen-model panic never stalls the passthrough.
  > Pre-send readiness and modal detection on `run`'s feed thread, handed a copy of the pump's output after the human's write, and the screen of `viola verify`'s five interactive probe runs.
- (.andromeda/architecture.md:4) A screen model poisoned by a feed panic must still refuse a send rather than read any signature, which is the `input-not-ready` degrade the chunk re-verifies.
  > a poisoned screen model or a screen not quiet within the maximum wait still refuses, and no signature is read)
- (.andromeda/architecture-detail/stack-and-technologies.md:20) Every serde_json parse of external bytes the chunk proves must run on this parser at its default depth and parse external payloads tolerantly.
  > serde 1.0.229, serde_json 1.0.151 (feature `preserve_order`, which pulls indexmap), serde_path_to_error 0.1.20 | Channel frames, log events, snapshots, tolerant parsing of external payloads with path-precise drift reports
  (.andromeda/architecture.md:449)
  > - Validation: tolerant serde for external payloads, versioned-tolerant own formats, nutype 0.8.0 newtypes.
- (.andromeda/architecture-detail/stack-and-technologies.md:17) The own-format readers the chunk bounds (`snapshot.json`, `events.ndjson`, the replay) must keep `v`, skip unknown kinds and fields, and rebuild an unsupported-`v` snapshot by replay.
  > Every record carries `v`, readers skip unknown kinds and fields, and a snapshot with an unsupported `v` is rebuilt by replaying the log.
- (.andromeda/architecture-detail/stack-and-technologies.md:22) Every name before a path join and every config value the chunk re-asserts must pass through these nutype newtypes.
  > | Domain newtypes | nutype 0.8.0 | `ViolaName` (charset + length cap), `Percent` (0–100), `DialogHold` (whole seconds 60–3600) |
- (.andromeda/architecture-detail/stack-and-technologies.md:19) The pinned key's closed-shape validator must accept exactly the `<version>-<hash>` key whose hash is the 16-hex truncated SHA-256.
  > SHA-256 of the exe bytes, truncated to the first 8 bytes (16 hex), for the `bin/` and `plugin/` `<version>-<hash>` keys
- (.andromeda/architecture-detail/stack-and-technologies.md:21) Any frame type the chunk's frame-bound work touches in `frame.rs` must keep its `params` behind `#[derive(Redact)]`, so no rejected or oversize frame renders them through `Debug`.
  > veil `=0.3.0` (`#[derive(Redact)]`, feature `toggle` banned), so a frame's `params` never reach a `Debug` render
- (.andromeda/architecture-detail/stack-and-technologies.md:27) The `viola mcp` stdin and tool arguments must be bounded and re-validated by the server, the schemars input schemas staying advisory, over exactly rmcp's `server` + `transport-io` features.
  > rmcp 3.4.1 (features `server`, `transport-io`), minor pinned; schemars 1.2.2 (tool input schemas) | stdio MCP server implementing spec 2026-07-28 (compatible with 2025-11-25): tools `send · wait · last · answer · list`.
- (.andromeda/architecture-detail/stack-and-technologies.md:31) The Path 1 WATCH tally must count only this chunk's settled CI runs, never a local pre-push pass.
  > a pre-push filter, never the verdict of record
- (.andromeda/architecture.md:447) Every refusal the chunk's bounds produce must surface as the typed, closed `RefusalReason`, never a free string.
  > - API style: typed `RefusalReason` in `result.refusal`.

## Bindings
- **serde_json default depth ↔ the security plan's bounded readers**: the parser stack (.andromeda/architecture-detail/stack-and-technologies.md:20) is the depth half of the `Read::take(MAX_FRAME)` + default-depth pair each external reader holds, the tolerant regime confirmed by the body (.andromeda/architecture.md:449).
- **nutype newtypes ↔ closed decision types**: `ViolaName` / `Percent` / `DialogHold` (.andromeda/architecture-detail/stack-and-technologies.md:22) and `RefusalReason` (.andromeda/architecture.md:447) together make every outside decision a closed type or a validated newtype.
- **Concurrency model ↔ Cross-cutting Patterns' Tokio containment**: std threads with Tokio only in `mcp` (.andromeda/architecture-detail/stack-and-technologies.md:11) and the feature-gated `TokioClient` (.andromeda/architecture-detail/stack-and-technologies.md:15) are the stack side of the containment rule (.andromeda/architecture.md:412) over the readers the chunk bounds.
- **Own-format `v` reads ↔ Mixed-version tolerance**: the stack's skip-and-replay rule (.andromeda/architecture-detail/stack-and-technologies.md:17) is the cross-cutting rule that every record carries `v` and readers skip unknown kinds and fields (.andromeda/architecture.md:408).
- **vt100 feed ↔ Design Philosophy's poisoned-screen refusal**: the feed thread handed a copy after the human's write (.andromeda/architecture-detail/stack-and-technologies.md:14) and the poisoned model still refusing (.andromeda/architecture.md:4) are the two halves of the panic degrade the chunk re-verifies.
- **Fuzz toolchain ↔ fuzz tooling pins**: the nightly pin (.andromeda/architecture-detail/stack-and-technologies.md:10) and cargo-fuzz with its own lockfile (.andromeda/architecture-detail/stack-and-technologies.md:32) bind every fuzz target the proof extends.
