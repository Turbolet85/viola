# /implement question — session 2 after a red verify (2026-11-06)

## The question, as asked (the recommendation first)

Live session 1 (the verify) read RED: `stamped 2.1.287  23 pass  1 fail`, the failing row `statusline-rate-limits`. The
likely cause: the account's status-line payload right now carries no five-hour window. This session's own wrapped
status line, read 2 s before the stop, holds five_hour `unknown` and only the seven-day window. Recorded and not
re-fired. The stamp leaves 2.1.287 unverified, so a wrapped start replies null to any dialog at once and the 3600 s
hold cannot be read. Plan step 9 says session 2 is not made. What now?

1. **Stop the round (Recommended).** As plan step 9 writes it: no session 2. The ledger holds 1 start and 1 red, so
   readers 37 and 39 read red. The live clauses of v1-53 and v1-27, CARRY 2 (the hold) and CARRY 11 (the killed
   wrapper on real claude) stay unproven. They are surfaced for the wrap, and the founder rules them (defer, or a later
   round). Gates and records continue.
2. **Unverified session 2.** Spend the cap's second slot on one unverified wrapped start,
   `run done-check -- claude 2.1.287`, no key. It reads openpty, the 0700 socket dir and 0600 socket, the child's fd
   table and the killed-wrapper leg (v1-53, CARRY 11, CARRY 15's table). v1-27's `cli_verified:true` clause and the
   hold cannot be read, so CARRY 2 stays PROVISIONAL. This departs from step 9's stop.

## The answer (verbatim, as it came back)

Chosen: «Unverified session 2». Notes: The overseer, technical answer (founder-delegated), 2026-11-06 20:16Z. The
founder's cap is two starts and his word is to prove v1-53. One unverified wrapped start, no key, reads every live clause
it can: openpty, the socket dir and socket modes, the child's fd table and the killed-wrapper leg. Record the departure
from step 9 with this reason. v1-27's `cli_verified:true` clause and the 3600 s hold cannot be read, so CARRY 2 stays
PROVISIONAL and v1-27's verified clause goes to the founder at the wrap. Surface the verify's red row
`statusline-rate-limits` as a finding: the account's status-line payload carries no five_hour window now, so the row
reads the account's plan, not the CLI. No third start.
