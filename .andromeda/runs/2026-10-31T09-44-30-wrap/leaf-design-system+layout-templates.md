group: design-system+layout-templates
read: .claude/docs/design-summary.md · 73 · «**Full plans:** `.andromeda/design-system.md` + `.andromeda/»
- leaf: .claude/docs/design-summary.md:50   class: omission
  leaf-text: «the `viola send` readback mirror»
  source: .andromeda/design-system.md:263 «except an oversize `viola answer` and an unusable `viola send`, which exit 2 with their fixed stderr line and no document»
  replace-with: «the `viola send` readback mirror (an unusable `viola send` and an oversize `viola answer` instead exit 2 with their fixed stderr line, `UNUSABLE_TEXT` / `UNUSABLE_RESPONSE`, whatever `--json` says, and no document)»
- leaf: .claude/docs/design-summary.md:50   class: omission
  leaf-text: «with exit 1 and no hint)»
  source: .andromeda/design-system-detail/surface.md:23 «A peer's `refusal` that is no code word reads `unknown`; a string `detail` that is no code word reads as no detail, null under `--json`; a non-string `detail` is no known reply.»
  replace-with: «with exit 1 and no hint), refusal lines (both clients screen a peer's `refusal` and `detail` through the code word, 1 to 64 bytes of lower-case letters, digits and hyphens, a lower-case letter first, `viola_core::is_code_word`: a `refusal` that is no code word reads `unknown`, a string `detail` that is no code word reads as no detail, null under `--json`, a non-string `detail` is no known reply, and a code word outside the closed refusal list stays visible and exits 14 as `unknown` does; `unknown` has no hint)»
- leaf: .claude/docs/design-summary.md:50   class: omission
  leaf-text: «`viola verify` step counter»
  source: .andromeda/design-system-detail/surface.md:160 «Exit 20's `--json` `detail` is closed for the five known codes (`viola_channel::sanitised_fault`): `message` is each code's fixed text;»
  replace-with: «`error: wrapper fault  <code>` (exit 20, no hint, the human line holds the code only; under `--json` the `wrapper-fault` document, its `detail` closed for the five known codes by `viola_channel::sanitised_fault` and `{"code":<code>,"message":"wrapper fault","data":null}` for any other code), `viola verify` step counter»
