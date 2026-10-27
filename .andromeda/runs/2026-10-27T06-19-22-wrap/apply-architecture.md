# Apply — architecture

## Verdicts
- fanout-architecture.md 1 · applied · .andromeda/architecture.md:43#1
- fanout-architecture.md 3 · applied · .andromeda/architecture.md:421+#1
- fanout-architecture--cascade-new-cross-cutting-patterns.md 1 · applied · .andromeda/architecture-detail/cross-cutting-patterns.md:23+#1, .andromeda/architecture.md:461#1
- fanout-architecture--cascade.md 1 · applied · .andromeda/architecture-detail/established-decisions.md:23#1
- fanout-architecture--cascade.md 2 · applied · .andromeda/architecture-detail/established-decisions.md:108+#1
- fanout-architecture--cascade.md 3 · applied · .andromeda/architecture-detail/established-decisions.md:114+#1
- fanout-architecture--cascade.md 4 · applied · .andromeda/architecture-detail/established-decisions.md:115+#1
- fanout-architecture--cascade.md 6 · applied · .andromeda/architecture-detail/established-decisions.md:19+#1
- fanout-architecture--cascade.md 5 · applied · .andromeda/architecture-detail/stack-and-technologies.md:20#1
- fanout-architecture--cascade.md 7 · applied · .andromeda/architecture-detail/occupied-resources.md:29#1, .andromeda/architecture-detail/occupied-resources.md:29+#1
- fanout-architecture--cascade.md 8 · applied · .andromeda/architecture-detail/occupied-resources.md:123#1
- fanout-architecture.md 2 · applied · .andromeda/registries/contracts/architecture/build-system.md:8#1, .andromeda/registries/contracts/architecture/build-system.md:8#2

## Edits
- line: .andromeda/architecture.md:43
  old: «as human controls over the wheel and the link topology. Rationale:»
  new: «as human controls over the wheel and the link topology. Its stdin is read under a `MAX_FRAME` line cap, each line's newline counted: a longer line is refused whole, never truncated or skipped, and ends the session as a closed stdin does, with one `parse-rejected{parser:"mcp-stdin"}` line holding no byte of it; the bound sits in the reader viola hands rmcp, because rmcp 3.4.1's stdio transport reads each line unbounded. Rationale:»
- after: .andromeda/architecture.md:421
  anchor: «- **Untrusted upstream text.** A driven session's `Stop` text and dialog payloads are content, never commands to viola. viola forwards them as escaped strings and never interprets them (R1).»
  add: «- **Bounded readers.** Every product reader of external bytes is bounded in its own statement (`take`) or named as an exception (the own-executable reads); the contract binary `tests/contract_bounded_readers.rs` enumerates the boundaries and censuses the product sources.»
- line: .andromeda/architecture.md:461
  old: «crosscut-capability-ledger · infra-crate-dependency-direction -->»
  new: «crosscut-capability-ledger · crosscut-bounded-readers · infra-crate-dependency-direction -->»
- after: .andromeda/architecture-detail/cross-cutting-patterns.md:23
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · crosscut-bounded-readers -->»
  add: «»
  add: «- **Bounded readers at every boundary.** Every product reader of external bytes is bounded in the same statement (`take`) or named in an `EXCEPTIONS` row; none reads an external byte source to its end unbounded. The E2E contract binary `tests/contract_bounded_readers.rs` (a `contract_` binary, run under `run --e2e`) holds it: its `BOUNDARIES` table, one `Boundary {` block per bounded read, names each read and its witness test, and `boundaries_each_name_a_bounded_read_that_stands` checks that each named read still stands; its `EXCEPTIONS` table names the own-executable reads, which are no external input. A census, `product_readers_are_each_bounded_or_named`, walks the product sources and judges each reader call bounded over its statement up to the call, matching an `EXCEPTIONS` needle against the call's whole source line, because the executable argument the needle names lies past the call's `(`; its self-check, `census_finds_an_unbounded_read_and_passes_a_bounded_one`, reads a planted unbounded read as a hit and passes a bounded one, and `serde_json_unbounded_depth_is_banned_in_deny_toml` reads the `deny.toml` feature ban. A new reader therefore lands bounded in its statement or as a named exception, or the census goes red.»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/architecture-detail/established-decisions.md:23
  old: «one `parse-rejected{parser:"vt100-feed", detail:"oversize"}` per poisoning episode»
  new: «one `parse-rejected{parser:"vt100-feed"}` per poisoning episode, its `detail` `panicked` for a caught panic and `oversize` for a dropped message»
- after: .andromeda/architecture-detail/established-decisions.md:108
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · dec-mcp -->»
  add: «»
  add: «- `viola mcp` hands rmcp its stdin through the crate-private `viola_mcp::stdin::CappedLines` at `MAX_FRAME` bytes per line, the `\n` counted, as the reader of rmcp's `(reader, writer)` stdio transport (the path `transport-io` already enables; rmcp's features stay `server` + `transport-io`). A line of exactly the cap passes whole, and at the end of stdin a held unterminated line within the cap is released; a longer line releases no byte to rmcp, sets the reader's cut flag and then reads as end of stream, so the line is refused whole and the session ends as at a closed stdin. It is never truncated, because a padded valid request would be dispatched, and never skipped, because the peer's request would go unanswered with no signal; the wrapper channel's close-on-oversize rule is its precedent (§Standard Contracts, Wrapper channel frames).»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/architecture-detail/established-decisions.md:114
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · dec-mcp -->»
  add: «»
  add: «- After a session a cut ended, `viola_mcp::serve` writes exactly one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` at WARN through `obs_event!`, holding no byte of the line, then returns what the session gave: `Ok(())` once the session initialized (exit 0, as at a closed stdin), or `ServeError::Initialize` (exit 11) when the cut lands before `initialize` completes.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/architecture-detail/established-decisions.md:115
  anchor: «»
  next: «<!-- detail: values · dec-hook-transport -->»
  add: «<!-- detail: values · dec-mcp -->»
  add: «»
  add: «- The bound sits in the reader viola hands rmcp, never in rmcp's codec: rmcp 3.4.1's stdio `AsyncRwTransport::receive` reads each line with an unbounded `read_until`, and `JsonRpcMessageCodec::new_with_max_length` bounds only the writer side (as read in rmcp 3.4.1's crates.io source, `src/transport/async_rw.rs`). `CappedLines` signals a cut as end of stream, never as an I/O error, so rmcp writes nothing of its own.»
  add: «»
  add: «<!-- /detail -->»
  add: «»
- after: .andromeda/architecture-detail/established-decisions.md:19
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: pending · dec-pty -->»
  add: «»
  add: «- OPEN — on a Windows host, how the child's panicking wide character (`e4 b8 ad`) reaches the outer terminal after a contained vt100 feed panic under a 1-column ConPTY is not read: ConPTY re-renders the child's output, and a wide character on a 1-column screen has no guaranteed rendering, so `tests/chaos_feed_panic.rs` asserts those bytes on Unix and, on Windows, only that no viola literal appears on the outer terminal; the reading and a Windows byte oracle from it · owner: "Windows-only live measurements" (PROVISIONAL).»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/architecture-detail/stack-and-technologies.md:20
  old: «declared key order (e.g. `{"v":1,"cmd":…,"ok":…}`) |»
  new: «declared key order (e.g. `{"v":1,"cmd":…,"ok":…}`); serde_json's `unbounded_depth` feature is banned in `deny.toml` (Build system), so every parse of external bytes keeps serde_json's default recursion limit |»
- line: .andromeda/architecture-detail/occupied-resources.md:29
  old: «until stdin closes, and exits 1 with nothing on stdout or stderr and no role file when its home or `diagnostics/` (each that exists) fails `viola_state::strict::check_path`. `verify` takes `[--record <DIR>] [-- <program> [args…]]` (program default `claude`). `list` takes `[--json]` and nothing else; it exits 0 with a board (one stdout write), 21 when its strict-modes check refuses the home, `instances/` or any one instance (the whole board refused), 1 for an internal error through the `cli` catch site and 2 for a clap usage error (chunk 2026-10-11-the-board-viola-list). `plugin install` takes `[--json]` and nothing else (`viola plugin` alone is a clap usage error; clap's own `viola plugin help` is served); it exits 0 with its result (one stdout write), 21 when its strict-modes check refuses the home or any existing `marketplace/` tree dir (nothing created, no log opened), 1 for an internal error through the `cli` catch site and 2 for a clap usage error. `revive` takes `<name> [--id <ID>] [--fork] [-- <child args>]`, or `<name> --list`, each with `[--json]`; under `--json` a refused strict-modes check or a preflight refusal is the exit-1 `{"v":1,"error":"<code>","detail":null}` document (one of the five codes, §Standard Contracts) with stderr empty, and `--list --json` prints the `ok.sessions` document, or the `no-session` document with exit 1; it exits 0 when the revived child's wrapper ends as `run`'s does, 1 for a refused strict-modes check or a preflight refusal and 2 for a clap usage error (a malformed `--id` included).»
  new: «until stdin closes or a stdin line passes `MAX_FRAME`, which ends the session as a close does (exit 0 once initialized, 11 before `initialize`), and exits 1 with nothing on stdout or stderr and no role file when its home or `diagnostics/` (each that exists) fails `viola_state::strict::check_path`.»
- after: .andromeda/architecture-detail/occupied-resources.md:29
  anchor: «- Subcommands: `run`, `revive`, `send`, `wait`, `last`, `list`, `answer`, `hook <event>`, `mcp`, `verify`, `pause`, `release` (incl. `--budget`), `link`, `unlink`, `plugin install`. `mcp` (chunk 2026-10-17-mcp-server-for-drivers) is hidden from `viola --help` (`#[command(hide = true)]`: run by the plugin, never by a person) and takes no argument but the global `--home`; it serves the five tools over stdio until stdin closes, and exits 1 with nothing on stdout or stderr and no role file when its home or `diagnostics/` (each that exists) fails `viola_state::strict::check_path`. `verify` takes `[--record <DIR>] [-- <program> [args…]]` (program default `claude`). `list` takes `[--json]` and nothing else; it exits 0 with a board (one stdout write), 21 when its strict-modes check refuses the home, `instances/` or any one instance (the whole board refused), 1 for an internal error through the `cli` catch site and 2 for a clap usage error (chunk 2026-10-11-the-board-viola-list). `plugin install` takes `[--json]` and nothing else (`viola plugin` alone is a clap usage error; clap's own `viola plugin help` is served); it exits 0 with its result (one stdout write), 21 when its strict-modes check refuses the home or any existing `marketplace/` tree dir (nothing created, no log opened), 1 for an internal error through the `cli` catch site and 2 for a clap usage error. `revive` takes `<name> [--id <ID>] [--fork] [-- <child args>]`, or `<name> --list`, each with `[--json]`; under `--json` a refused strict-modes check or a preflight refusal is the exit-1 `{"v":1,"error":"<code>","detail":null}` document (one of the five codes, §Standard Contracts) with stderr empty, and `--list --json` prints the `ok.sessions` document, or the `no-session` document with exit 1; it exits 0 when the revived child's wrapper ends as `run`'s does, 1 for a refused strict-modes check or a preflight refusal and 2 for a clap usage error (a malformed `--id` included).»
  add: « `verify` takes `[--record <DIR>] [-- <program> [args…]]` (program default `claude`). `list` takes `[--json]` and nothing else; it exits 0 with a board (one stdout write), 21 when its strict-modes check refuses the home, `instances/` or any one instance (the whole board refused), 1 for an internal error through the `cli` catch site and 2 for a clap usage error (chunk 2026-10-11-the-board-viola-list). `plugin install` takes `[--json]` and nothing else (`viola plugin` alone is a clap usage error; clap's own `viola plugin help` is served); it exits 0 with its result (one stdout write), 21 when its strict-modes check refuses the home or any existing `marketplace/` tree dir (nothing created, no log opened), 1 for an internal error through the `cli` catch site and 2 for a clap usage error. `revive` takes `<name> [--id <ID>] [--fork] [-- <child args>]`, or `<name> --list`, each with `[--json]`; under `--json` a refused strict-modes check or a preflight refusal is the exit-1 `{"v":1,"error":"<code>","detail":null}` document (one of the five codes, §Standard Contracts) with stderr empty, and `--list --json` prints the `ok.sessions` document, or the `no-session` document with exit 1; it exits 0 when the revived child's wrapper ends as `run`'s does, 1 for a refused strict-modes check or a preflight refusal and 2 for a clap usage error (a malformed `--id` included).»
- line: .andromeda/architecture-detail/occupied-resources.md:123
  old: «(the `config-json` rejections included), and `process-exit` on stdin EOF»
  new: «(the `config-json` rejections included), one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` at WARN after a session a stdin line past `MAX_FRAME` ended (no byte of the line), and `process-exit` on stdin EOF or at that cut»
- line: .andromeda/registries/contracts/architecture/build-system.md:8
  old: «`tracing-subscriber/env-filter`)»
  new: «`tracing-subscriber/env-filter`, `serde_json/unbounded_depth`, so every parse of external bytes keeps serde_json's default recursion limit, its probe row `serde_json-unbounded-depth`)»
- line: .andromeda/registries/contracts/architecture/build-system.md:8
  old: «(`deny-probes: 13/13 banned, control clean`)»
  new: «(`deny-probes: 14/14 banned, control clean`, the count one per `probes=(` row of `scripts/deny-probes.sh` plus the planted `channel-tokio-edge`)»

## Patterns
[[pattern]]
id = "arch-01"
fixed = 'detail:"oversize"}` per poisoning episode'

[[pattern]]
id = "arch-02"
fixed = 'over stdio until stdin closes, and exits'

[[pattern]]
id = "arch-03"
fixed = '`process-exit` on stdin EOF —'

[[pattern]]
id = "arch-04"
fixed = '`tracing-subscriber/env-filter`)'

[[pattern]]
id = "arch-05"
fixed = '13/13 banned, control clean'
