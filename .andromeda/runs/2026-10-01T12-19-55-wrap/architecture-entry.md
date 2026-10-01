
## 2026-10-01-t12-19-55-wrap — GUI reads gated by the per-launch cookie; the page's toolkit is React + TypeScript
**Section:** Established Decisions → [GUI Control Scope]; Conventions → GUI HTTP errors; Occupied Resources → Filesystem; Occupied Resources → Repository (the `e2e-web/test-results/lint/` row); §Infrastructure Patterns → Build system; §Infrastructure Patterns → Project directory structure
**Change:**
- [GUI Control Scope] was "no token or CSRF"; now v1 reads are gated: a per-launch token exchanged once at `/?t=` for the `viola_<port>` cookie (`HttpOnly; SameSite=Strict; Path=/`, 303 to `/`) gates `/api/info`, `/api/sessions`, `/api/links` and SSE `/api/events` (401 without it); `/`, `/assets/*`, `/health`, `/ready` stay ungated. View-only and the v1.x brake contract are unchanged.
- GUI HTTP errors add `urn:viola:problem:unauthorized` (401) and `urn:viola:problem:cross-origin-forbidden` (403, reserved for the v1.x brake, not served in v1).
- Filesystem adds `ui/<port>.url`: the launch URL, written through `replace_private` (0600, dir 0700 on Unix), overwritten at the next launch on that port, removed on graceful shutdown, never read back as a credential, never logged.
- The lint report row names eslint-plugin-jsx-a11y (was eslint-plugin-lit-a11y).
- Build system keeps its guard (no tsconfig, package manifest or bundler config under `crates/viola-ui/`) and now names the founder ruling: the route's frontend-toolchain entry replaces the guard and brings the frontend sources into the ts plane.
- The directory tree's `assets/` line no longer names vendored Lit 3.3.3 ESM; the bundle layout is OPEN. `e2e-web/eslint.config.js` runs eslint-plugin-jsx-a11y over the frontend sources.
**Why:** security-plan's arch amendments 1 and 5 and amendment 3's two URNs were ratified and never folded, which left arch contradicting security on GUI reads; the overseer (founder-delegated) directed the fold at this wrap. The toolkit move is the founder's ruling of 2026-09-30, relayed by the overseer. Standing rule: retiring the no-bundler guard is a boundary widening the founder rules live at the frontend-toolchain entry, not before.
**Kept:** amendment 3's `control-character` detail, unfolded until the route's "Confirmed send" entry, which names it; the guard text itself.
**Ref:** .andromeda/runs/2026-10-01T12-19-55-wrap/
