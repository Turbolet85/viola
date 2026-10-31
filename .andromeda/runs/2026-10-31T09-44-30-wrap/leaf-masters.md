group: masters
read: CLAUDE.md · 129 · «<!-- USER:session-learnings end -->»
- leaf: CLAUDE.md:36   class: omission
  leaf-text: «no absolute paths, no upstream text, no anyhow chain holding a serde source.»
  source: .andromeda/architecture-detail/cross-cutting-patterns.md:33 «a peer's `message`, `data`, `refusal` or `detail` reaches no document or text but as a closed value, and a code word outside the closed list stays visible by design.»
  replace-with: «no absolute paths, no upstream text, no anyhow chain holding a serde source; a client (`reply_of`, `of_reply`) reads a peer's `refusal` and string `detail` only as a code word (`viola_core::is_code_word`) and its fault only through `viola_channel::sanitised_fault`, a code word outside the closed list staying visible by design; `tests/contract_error_bodies.rs` names every error body and product error writer, and a `thiserror` message interpolates nothing.»
- leaf: CLAUDE.md:20   class: omission
  leaf-text: «`validate_paste_text`, `MAX_FRAME`, `SPINE_DEADLINE`»
  source: .andromeda/architecture-detail/conventions.md:21 «- **Code word:** `viola_core::is_code_word(&str) -> bool`: 1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first; `reply_of` and `of_reply` call it.»
  replace-with: «`validate_paste_text`, `is_code_word`, `MAX_FRAME`, `SPINE_DEADLINE`»
- leaf: CLAUDE.md:22   class: omission
  leaf-text: «sync client/server, Tokio client behind a feature;»
  source: .andromeda/architecture-detail/standard-contracts.md:118 «`viola_channel::sanitised_fault(&Value) -> Option<Value>` is the one fault reader, of `reply_of` and `of_reply`»
  replace-with: «sync client/server, Tokio client behind a feature; `sanitised_fault` (the one reader of a peer's fault, for `reply_of` and `of_reply`) and `ProtocolError::cause` (`channel-response`'s closed `detail` for the three `-32602` faults);»
