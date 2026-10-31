group: architecture+design-system+layout-templates
read: .claude/rules/frontend.md · 29 · «_This section is owned by `/wrap-session`. setup-project pre»
- leaf: .claude/rules/frontend.md:25   class: omission
  leaf-text: «exits 1 with nothing on stdout or stderr when its home or `diagnostics/` fails `check_path`»
  source: .andromeda/architecture-detail/established-decisions.md:131 «`viola mcp` on a line past `MAX_FRAME` before `initialize` exits 1 with both streams empty»
  replace-with: «exits 1 with nothing on stdout or stderr when its home or `diagnostics/` fails `check_path` or a stdin line past `MAX_FRAME` lands before `initialize`»
- leaf: .claude/rules/frontend.md:25   class: omission
  leaf-text: «`{"v":1,"error":"<code>","detail":null}` with stderr empty)»
  source: .andromeda/design-system.md:263 «except an oversize `viola answer` and an unusable `viola send`, which exit 2 with their fixed stderr line and no document»
  replace-with: «`{"v":1,"error":"<code>","detail":null}` with stderr empty, and an oversize `viola answer` / an unusable `viola send` exit 2 with their fixed `UNUSABLE_RESPONSE` / `UNUSABLE_TEXT` stderr line and no document)»
- leaf: .claude/rules/frontend.md:25   class: omission
  leaf-text: «, errors. ASCII only»
  source: .andromeda/design-system-detail/surface.md:161 «prints the same human line, `error: wrapper fault  <code>`, and under `--json` the document `{"v":1,"error":"wrapper-fault","detail":{"code":<code>,"message":"wrapper fault","data":null}}`»
  replace-with: «, errors (`error: wrapper fault  <code>` holds the code only; under `--json` the `wrapper-fault` document's `detail` is closed, read through `viola_channel::sanitised_fault`, a code outside the five reading `{"code":<code>,"message":"wrapper fault","data":null}`). ASCII only»
- leaf: .claude/rules/frontend.md:25   class: omission
  leaf-text: «no stack traces, no "done".»
  source: .andromeda/architecture-detail/cross-cutting-patterns.md:33 «CLI `--json` documents, MCP error results and channel error data carry codes and fixed messages only, no path and no upstream text»
  replace-with: «no stack traces, no "done"; a `--json` document carries codes and fixed messages only, no path and no upstream text.»
- leaf: .claude/rules/frontend.md:26   class: omission
  leaf-text: «no hint for `unknown`, `wrapper fault` or `internal error`.»
  source: .andromeda/design-system-detail/surface.md:23 «A peer's `refusal` that is no code word reads `unknown`; a string `detail` that is no code word reads as no detail, null under `--json`; a non-string `detail` is no known reply.»
  replace-with: «no hint for `unknown`, `wrapper fault` or `internal error`. `reply_of` (`src/cmd/client.rs`) screens a peer's `refusal` and `detail` through `viola_core::is_code_word`: a `refusal` that is no code word reads `unknown`, a string `detail` that is no code word reads as no detail (null under `--json`), a non-string `detail` is no known reply, and a code word outside the closed refusal list stays visible and exits 14 as `unknown` does.»
