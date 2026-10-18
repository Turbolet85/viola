
## 2026-10-18-compact-post-condition-ledger-row — the compact send's 40 s confirm window, a bound on the driver only
**Section:** §8 Cognitive Accessibility → Timeout extensions (CLI)
**Change:** once typed, a confirmed `viola send` waits for its confirmation within `CONFIRM_WINDOW_FALLBACK` (10 s), and a `/compact` send on a verified CLI within its own `COMPACT_CONFIRM_WINDOW` (40 s, PROVISIONAL); unconfirmed inside it, `not-delivered` / `no-prompt-submitted`. The bound falls on the driver only: the confirm wait takes no key and blocks, refuses or delays none, and the wheel is untouched. Was: the section named no confirm window.
**Why:** the chunk added the compact window (report Symbols / APIs); the plan's expected amendment 8, raised by the wrap.
**Ref:** .andromeda/runs/2026-10-18T21-59-05-wrap/
