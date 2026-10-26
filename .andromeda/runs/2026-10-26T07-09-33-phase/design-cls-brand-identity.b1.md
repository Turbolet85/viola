# Classified — design · brand-identity.b1

## Answers
brand-identity#1.1 · yes · static and zero animation bind: the Needs keep the waiting line static, with no spinner or counter in any case, on the cli human TTY row at 0.2
brand-identity#1.2 · no · no cli surface sits at the 0.3-0.4 band (hover, fade, focus rings; the cli rows are 0.2 and 0.0) and the chunk renders no page
brand-identity#1.3 · no · no cli surface sits at the 0.5-0.6 band (transitions, micro-interactions, loading skeletons); the wait verb's loading state stays the static waiting line, with no skeleton added
brand-identity#1.4 · no · no cli surface sits at the 0.7-0.8 band (staggered entrances, parallax, spring physics) and the chunk adds no such motion
brand-identity#1.5 · no · no cli surface sits at the 0.9-1.0 band (scroll-driven, 3D, particles, canvas/WebGL) and the chunk adds no such motion
brand-identity#2.1 · yes · the cli human TTY row (0.2) binds: colour stays only on DIALOG and stale rows, so the new unable refusal carries none; nothing redraws in place; the waiting line prints no spinner or counter (Needs)
brand-identity#2.2 · yes · the cli --json, piped, NO_COLOR and TERM=dumb row (0.0) binds: each new refusal there carries no colour, glyph, cursor control or spinner (Needs: no SGR, no glyph or check-mark prefix); the viola run and revive passthrough is not reached
