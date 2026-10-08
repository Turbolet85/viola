Answer to this run's AskUserQuestion on the stop of the live work (header "Live stop"), verbatim as it arrived in the session, 2026-10-08, before 07:32:09Z (the clock read right after it arrived).

The question: The live session lost the wheel at start to a terminal reply (wheel human/human-input 237 ms after the child started, no key typed), so every driver send is refused. How should the live work stop?

The options as shown:
- "Measure cause, then end (Recommended)": Close the live session now. Inside this same compositor start, with no live start and no key, measure which terminal reply the classifier takes for typing (harness viola over a script child that sends one query at a time). Then end the compositor, read the lock (9a) and report. A later live run needs a second compositor start (the founder's word) and has 2 live starts left.
- "End at once": Close the live session, end the compositor, read the lock, report. No cause measurement: which reply it was stays a hypothesis, and measuring it later needs a compositor start of its own.
- "Measure, keep compositor up": Close the live session and measure the cause, but leave the own compositor running with no window while the plan is revised and the classifier fixed, so the next live session needs no second start. It then stands on the locked desktop for as long as the revision takes; the lock is read now and again at its real end. This departs from the plan's step 9a at a stop.

Chosen: "Measure, keep compositor up"

Notes, verbatim: The operator: the founder ruled ONE compositor start for all live work, and a second start is what the measurement tied to the shell exit, so the one start is kept and stretched instead. Close the live session, measure which terminal reply the classifier takes for typing (no live start, no key), read the desktop lock now and record it, and leave the own compositor up with no window. Then report and stop: the classifier fix is a plan revision through the phase door, in this chunk, with its own red-green case, and the live run is retried on the same compositor with the 2 starts left. Bound: if no live start is made within 3 hours of this answer, end the compositor, take the 9a reading and report. Any lock reading that is not locked still stops everything at once (S6).
