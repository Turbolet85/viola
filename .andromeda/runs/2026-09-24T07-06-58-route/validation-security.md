# Security validation — route draft

## Reorder
- Move `Bounded inputs at every boundary` before `PTY wrapper on Windows` (Epoch 2, right after `Security prerequisites`)
  Reason: per security-plan §Bootstrap phases input-validation-library-install and amendments 3/6, the shared validators (MAX_FRAME, ViolaName, paste rule, vt100 panic degrade) must exist before the channel, hook, send and MCP chunks that call them, and those chunks sit in Epochs 2–5.
- Move `Home and code-bearing file integrity` to directly after `Instance state and start order` (before `Wrapper channel`)
  Reason: per security-plan §Authentication (`~/.viola/` access control) and amendment 8, owner-only creation, strict modes and the pinned-binary re-hash must land with the first chunk that writes `bin/`, `plugin/` and `snapshot.json`, and must come before `Statusline pass-through` runs `statusline_command`.
- Move `Windows endpoint admission` and `Server verification before any frame` to directly after `Wrapper channel` (before `Hooks to normalised events`)
  Reason: per security-plan §Anti-Patterns Authentication, no Windows listener may be created without the explicit DACL and no frame may be written before server verification, yet the draft ships the channel, hooks and `First live test and self-drive` (real prompts and permission `allow`) four epochs before these controls.
- Move `Sanitised errors and never-log floor` to directly after `Diagnostics plane` (Epoch 1)
  Reason: per security-plan §Bootstrap phases error-sanitization-wire / logging-redaction-wire, the NEVER-log floor (CLAUDE* tokens, upstream text, paths) must be in force before the first sink and before the channel, CLI `--json` and MCP error surfaces in Epochs 2–5 emit anything.
- Move `Unix endpoint and home hardening` before `Linux live confirmation`
  Reason: per security-plan §Anti-Patterns Authentication (never a socket at the `/tmp` root) and the Unix strict-modes row, a live Linux run with real session content must not happen on an unhardened endpoint and home.

## Rewrite
- `Wrapper channel`: "over the per-instance named pipe" → "over the per-instance named pipe (Unix: socket inside a per-user 0700 directory)"
  Reason: per security-plan §Bootstrap phases, amendment 2 (Unix endpoint move) must be folded in before the `viola-channel` chunk, and three-OS CI runs the channel on Unix from Epoch 2 on.
- `Diagnostics plane`: "per-instance detail files" → "owner-only per-instance detail files"
  Reason: per security-plan §Data Protection Logs and logging-redaction-wire, `diagnostics/` files are 0600 (Windows: home DACL) from the moment they are created, and they hold user content and drift reports.
- `viola ui loopback server`: "token-to-cookie exchange, CSP" → "token-to-cookie exchange gating /api and SSE, no CORS headers, CSP"
  Reason: per security-plan §Authentication (GUI principal check, v1 reads) and amendment 1, the cookie check on `/api/*` and SSE is the control that keeps other local users from reading the feed, and the current wording leaves it implicit.
- `Strip-bay live page`: "cell-for-cell parity with viola list" → "cell-for-cell parity with viola list, event fields rendered as text only"
  Reason: per security-plan §API Security (Frontend output-encoding constraints), prompts, assistant output, tool `input` and unwrapped names must never be rendered as HTML or Markdown on the page that first displays them.
- `Statusline pass-through`: "named read-only statusline source" → "named read-only statusline source, per-start settings.json rewrite with absolute pinned path"
  Reason: per security-plan §Data Protection (code-bearing artefacts) and §Anti-Patterns Universal, Claude Code runs `instances/<name>/settings.json`, so it must be rewritten at each start and must never resolve `viola` through PATH.
