## 2. Security Plan Excerpt

### Security Tier
- **Tier:** Minimal (stated verbatim as "`Minimal (0), with targeted elevations for the local privilege boundary`")
- **Justification:** "viola is a local-only, single-user tool. It has no accounts, no public network listener, no database, no stored credentials and no regulated data (Sections 1, 3, 4, 5), which rules out Standard's user-account and HTTPS concerns and Hardened's compliance drivers."
  - The elevations cover the local boundary only: IPC endpoint access control, bracketed-paste breakout in `send.text`, GUI output encoding, GUI cross-user and cross-origin readability, v1.x per-launch brake auth, and integrity of `~/.viola/`. They do not raise the tier to Standard.
  - Context: viola is a local CLI binary with a local IPC channel, a stdio MCP server, a Claude Code hook contract and a loopback-only, view-only web GUI (`viola ui` on `127.0.0.1:47319`, crate `viola-ui`). It has no accounts, no login forms, no password reset, no MFA and no captcha. The plan marks account-flow bans N/A.

### Anti-Patterns Rejected (a11y-relevant)
- **Rendering event content as HTML (`innerHTML`, `v-html`, `dangerouslySetInnerHTML`, or any Markdown-to-HTML renderer)**: rejected because the GUI renders untrusted upstream text (prompts, `last_assistant_message`, plan text, tool `input`, question text, unwrapped session names) and no HTML sanitizer was researched. Every event field must be rendered with `textContent` or the framework's text interpolation, and assistant Markdown is never rendered to HTML. A11y impact: assistant output reaches assistive technology as plain text, not as semantic headings, lists or links. Any structure or landmarks around conversation content must come from the GUI's own markup, not from the rendered content.
- **Inline `<script>`, inline event handlers and `eval`**: rejected under the enforced CSP: `default-src 'none'; script-src 'self'; style-src 'self'; connect-src 'self'; img-src 'self'; base-uri 'none'; form-action 'none'; frame-ancestors 'none'; require-trusted-types-for 'script'`. All JS is served from embedded `/assets/*`. A11y impact:
  - Keyboard and ARIA behaviour must live in external JS, with no inline handler attributes.
  - Inline `<style>` blocks and `style` attributes in markup are blocked (`style-src 'self'`).
  - Images and icons must be same-origin assets.
  - `font-src` falls back to `default-src 'none'`.
  - Third-party or CDN scripts are blocked, including runtime a11y overlays and injected checkers in the served page.
  - DOM writes must comply with Trusted Types.
- **Cookie without `HttpOnly`/`SameSite=Strict`, or leaving the token in the address bar after the `?t=` exchange**: rejected. Access goes through a one-time launch URL `http://127.0.0.1:<port>/?t=<token>`, which is printed to stderr and written to a 0600 `ui/<port>.url` file. The exchange sets a browser-session cookie (no `Max-Age`) and redirects 303 to `/`. A11y impact:
  - There is no login form. The only auth UI is the launch URL.
  - A missing or invalid cookie returns 401 `urn:viola:problem:unauthorized` with a fixed `detail`. No route re-issues the token, and error text must never contain the token or the `.url` path. The only recovery is to reuse the stderr line or the `.url` file, or to restart `viola ui`.
  - `/` loads ungated, while `/api/*` and SSE `/api/events` are gated. The page can therefore load and then hit a 401 on its data calls. That unauthorized state and its recovery instructions need to be perceivable.
  - The cookie has no timed expiry, so auth brings no session timeout.
- **Serving `/api/*` or SSE without the cookie check, and state-changing routes without cookie + `Sec-Fetch-Site`/`Origin` check**: rejected. v1 is GET-only (405 for other methods, `form-action 'none'`). The v1.x brake routes (`POST /api/sessions/{name}/pause`, `/unlink`) return 403 `urn:viola:problem:cross-origin-forbidden` on a cross-origin request. A11y impact: v1 has no forms or state-changing controls. The v1.x pause/unlink controls will have to handle 401/403 Problem Details error states (RFC 9457, fixed `detail` strings).
- **Printing `wait`/`last` human-mode output, or `list` rows from `claude agents --json`, to a terminal without escaping C0/C1 controls**: rejected (terminal-injection class). Controls are escaped except `\n` and `\t`. A11y impact: CLI output read by terminal screen readers can contain visible escape sequences where upstream text held control characters.
- **Writing `send.text` or answer free text containing C0 (other than LF/CR/TAB), DEL or C1 controls into the PTY**: rejected with refusal `not-delivered`, detail `control-character`, with no silent stripping. LF, CR and TAB must never be refused. A11y impact: multi-line input is allowed. Automation gets the refusal code, and security refusals never block the human (`NEVER let a security refusal block the human`).

### A11y Compliance Triggers
(No a11y compliance triggers in security plan — Phase 1 will derive a11y tier from creator brief + project intent + surface count.)
- The security plan states "Compliance triggers: None: no compliance-regulated data detected". It names no accessibility regime (no Section 508, ADA, EU Accessibility Act, EN 301 549, AODA or JIS X 8341). It also says the v1.x public distribution adds no compliance trigger, because all data stays local on each user's machine.
