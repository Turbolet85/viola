# Apply — architecture · part 2 of 2

## Verdicts
- fanout-architecture--cascade.md 9 · applied · .andromeda/architecture-detail/conventions.md:13#1
- fanout-architecture--cascade.md 10 · applied · .andromeda/architecture-detail/conventions.md:15#1
- fanout-architecture--cascade.md 11 · applied · .andromeda/architecture-detail/conventions.md:17+#1
- fanout-architecture--cascade.md 12 · applied · .andromeda/architecture-detail/conventions.md:18+#1
- fanout-architecture--cascade.md 14 · applied · .andromeda/architecture-detail/conventions.md:64#1
- fanout-architecture--cascade.md 18 · applied · .andromeda/architecture-detail/occupied-resources.md:29#1
- fanout-architecture--cascade.md 19 · applied · .andromeda/architecture-detail/occupied-resources.md:124#1
- fanout-architecture--cascade.md 20 · applied · .andromeda/architecture-detail/occupied-resources.md:126+#1
- fanout-architecture--cascade.md 21 · applied · .andromeda/architecture-detail/occupied-resources.md:207+#1
- fanout-architecture--cascade.md 22 · applied · .andromeda/architecture-detail/occupied-resources.md:236+#1
- fanout-architecture.md 4 · applied · .andromeda/registries/contracts/architecture/crate-dependency-direction.md:6#1

## Edits
- line: .andromeda/architecture-detail/conventions.md:13
  old: «`unknown`: any value»
  new: «`unknown`: a code word (`viola_core::is_code_word`) or `null`»
- line: .andromeda/architecture-detail/conventions.md:15
  old: «Readers treat an unrecognised `detail` as opaque text, never as an error.»
  new: «Readers treat an unrecognised code-word `detail` as opaque text, never as an error.»
- after: .andromeda/architecture-detail/conventions.md:17
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · conv-refusal-reason -->»
  add: «»
  add: «- **Code word:** `viola_core::is_code_word(&str) -> bool`: 1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first; `reply_of` and `of_reply` call it.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/architecture-detail/conventions.md:18
  anchor: «»
  next: «<!-- detail: values · conv-exit-codes -->»
  add: «<!-- detail: values · conv-refusal-reason -->»
  add: «»
  add: «- **A peer's reply:** `reply_of` and `of_reply` read a `refusal` that is no code word as `unknown`, a string `detail` that is no code word as `null`, and a non-string `detail` as no known reply.»
  add: «»
  add: «<!-- /detail -->»
  add: «»
- line: .andromeda/architecture-detail/conventions.md:64
  old: «`wrapper-fault` with the JSON-RPC fault's `{code,message,data}`»
  new: «`wrapper-fault` with the JSON-RPC fault's `{code,message,data}` as `viola_channel::sanitised_fault` reads it: a fixed `message`, `data` a closed object or null»
- line: .andromeda/architecture-detail/occupied-resources.md:29
  old: «(exit 0 once initialized, 11 before `initialize`)»
  new: «(exit 0 once initialized, 1 before `initialize`, both streams empty)»
- line: .andromeda/architecture-detail/occupied-resources.md:124
  old: «, and `process-exit` on stdin EOF or at that cut — none when Claude Code ends it by a signal, as measured live on claude 2.1.287 at that chunk;»
  new: «, and no self `process-exit`: none on stdin EOF or at that cut (`command grep -n 'ObsEvent::ProcessExit' -r src crates --include=*.rs`), and none when Claude Code ends it by a signal, as measured live on claude 2.1.287 at that chunk;»
- after: .andromeda/architecture-detail/occupied-resources.md:126
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · res-filesystem -->»
  add: «»
  add: «- `channel-response`'s `detail` in `schemas/diag-line.v1.json` is the closed enum `unsupported-version` · `invalid-params` · `release-from-driver`, written at WARN on both the server's and the client's line by `ResponseLine`'s `Drop` (`ProtocolError::cause`), the client's `fault_of(&Value)` reading `data` for `-32602`. It is absent on every other `channel-response` line and holds no param value or name.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/architecture-detail/occupied-resources.md:207
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · res-filesystem -->»
  add: «»
  add: «- Test-only: `tests/support/events.rs` holds the error-body check, `error_body_hit` and `assert_error_body_clean`, classes `absolute-path` · `caused-by` · `serde-path` · `planted`, reporting place and class, never bytes. No after-every-test helper is built: the three surface witnesses, the enumeration and the census of writers replace it, reading this check.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/architecture-detail/occupied-resources.md:236
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · res-repository -->»
  add: «»
  add: «- `v1-47`'s after every end-to-end test is after each job's end-to-end run: `AGENT_RUN_KEEP_HOMES` `"1"` on the job and `if: always()` in `.github/workflows/ci.yml`, the `viola-chaos-*` home out. PROVISIONAL, owed the founder's own word.»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:6
  old: «`resolve_program`, `Refusal`, `PLUGIN_DIR_FLAG`»
  new: «`resolve_program` (returning `Result<PathBuf, AgentError>`), `PLUGIN_DIR_FLAG`»

## Patterns
[[pattern]]
id = "arch2-01"
fixed = '`unknown`: any value'

[[pattern]]
id = "arch2-02"
fixed = 'treat an unrecognised `detail` as opaque'

[[pattern]]
id = "arch2-03"
fixed = "JSON-RPC fault's `{code,message,data}`)"

[[pattern]]
id = "arch2-04"
fixed = '11 before `initialize`'

[[pattern]]
id = "arch2-05"
fixed = '`process-exit` on stdin EOF'

[[pattern]]
id = "arch2-06"
fixed = '`resolve_program`, `Refusal`'
