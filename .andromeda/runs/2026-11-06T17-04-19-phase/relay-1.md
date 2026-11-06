# P4 dialog — the live-session cap and the verify (2026-11-06)

## The question, as asked (one question, the recommendation first)

[Founder, new] The dialog hold near the 3600 s ceiling (your 7/7-1) can only be read on a verified CLI: unverified,
the wrapper replies null at once (src/run/dialog.rs:674). No verify-stamped home exists on the host, so the rig home
needs one `viola verify` (six real claude starts) before the wrapped start. How does that count against your cap of 2
live sessions?

1. **Verify = the spare (Recommended).** Session 1 is one `viola verify` stamping the rig home on 2.1.287. Session 2
   is the one wrapped `viola run`, covering v1-53, v1-27, the near-ceiling hold and the killed-wrapper leg. This is how
   the 2026-10-19 hold round counted it («Verify + 1 старт»). Cost: six claude starts inside the verify, about 11
   minutes, plus about an hour of the wrapped start for the hold.
2. **One unverified start only.** No verify. The one wrapped start runs unverified, which covers v1-53, v1-27 and the
   killed-wrapper leg. The 3600 s ceiling stays PROVISIONAL into 0.2.0, reversing 7/7-1. Cost: one start, but the hold
   is not read.
3. **Verify on top of the 2.** Grant one verify beyond the cap, leaving the spare unspent. The cost is the same as the
   first option, with his cap read as two wrapped starts.

## The answer (verbatim, as it came back)

Chosen: «Verify = the spare (Recommended)». Notes: The founder's own word, live, by dialog in the overseer's session,
2026-11-06 19:38Z: «Verify = запасная». Session 1 is one `viola verify` stamping the rig home; session 2 is the one
wrapped `viola run`.
