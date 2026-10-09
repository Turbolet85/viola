## 2026-10-09-epoch-3-cleanup — the self-healing route entry cited by title
**Section:** §Established Decisions › [Database / State Store]; §Infrastructure Patterns › Project directory structure (key file)
**Change:** both sites name the working-route entry "Self-healing state" by its title. Was: `working-route.md:109` in the body and "route :93" in the key file's tree comment.
**Why:** the operator's direction that every stale bare route number found is cited by title. `:93` was stale; `:109` was current and this wrap's own insertion above the entry moves it. A bare route number goes stale at every insertion ahead of its entry, and the citation sweep reads no bare number.
**Ref:** .andromeda/runs/2026-10-09T17-10-00-wrap/
