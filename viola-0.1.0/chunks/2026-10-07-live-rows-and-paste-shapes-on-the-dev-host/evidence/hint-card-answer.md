# The answer to the hint card (R-L4)

The founder's own live answer, relayed by the overseer, to the one question raised from `hint-card.md` through
AskUserQuestion at implement. Recorded as his. Snapshot: inputs#I6.

## The answer, verbatim, as it arrived in the session
Chosen: "A longer bound"

Notes: "The founder's own live answer at 2026-10-07T10:29Z, through the overseer's AskUserQuestion, with the nine
timings and every option priced in front of him. His choice, in the card's terms: options 1 and 2 TOGETHER — the
gate waits for the input box on a quiet literal-less verified screen, and the bound rises to 8.5 s so all nine
measured windows are covered. He was told the price: one by-path re-verify (5 live starts), send's longest block
18.5 s, the fake agent's 8 000 ms hold cap to raise, no human keystroke delayed. Relayed by the overseer; record it
as his. Nothing is built from it here: it is a route entry at the wrap."

## The question as asked
"Founder's card (R-L4): what should `viola send` do while the CLI's paste hint stands? Measured on claude 2.1.287,
3 live starts: after a long paste the footer shows `paste again to expand` in place of the input-box literal for
8.0 s from the paste (9 timings, 8.000-8.023 s); 3.8 to 7.0 s of that falls after the turn's Stop. On a verified
home a send issued in that window is refused `input-not-ready` after 0.63 s with nothing typed, and the hint line's
advice (`viola wait <name>, then send again`) does not work: a no-cursor wait woke on nothing and ran its full
20 s. On an unstamped home the same short send was typed under the hint and the CLI submitted and confirmed it,
twice. Nothing is built from your answer in this chunk; the remedy becomes a route entry at the wrap. Which
remedy?"

## The options as shown
1. **Wait for the input box** — "On a verified CLI a quiet screen with no input-box literal and no modal keeps the
   gate waiting up to the existing 5 s maximum instead of refusing at once (viola verify's Run B already waits this
   way). Small: Screen::verdict and its cases, two fake-agent cases. At 5 s it covers only 2 of the 9 measured
   windows; the other seven are still refused, 5 s later than today. send's longest block stays 15 s. No human
   keystroke is delayed or refused."
2. **A longer bound** (chosen, together with 1) — "Raise GATE_MAX_WAIT above the window. Alone it changes nothing
   measured (the refusal comes at the first quiet instant, never at the bound); it only works together with the
   wait above. Together at 8.5 s it covers all 9 windows. Extra price: the constant is what the stamped quiet-period
   row is checked against, so one by-path re-verify (5 live starts); send's longest block becomes 18.5 s; an 8.5 s
   bound is over the fake agent's 8 000 ms hold cap. No human keystroke is delayed or refused."
3. **Type under the hint** — "Treat the hint screen as ready and paste under it. Measured to work for a short text
   (confirmed in 0.64 s, twice); unmeasured for a long text or a repeated text, and the footer's own words suggest
   a second paste can expand the first. Largest build: the hint literal becomes a relied-on CLI behaviour, so an
   eighteenth ledger row with a verify probe, a recorded variant, a fake-agent replay, 75 count-literal lines and a
   5-start record round. No wait added; send's longest block stays 15 s."
4. **Stay as it is** — "No code. A send in the window is refused fast (0.63 s) with nothing typed and the driver
   retries; what ends the window is time, at most 8.0 s from the paste. The printed hint keeps naming `viola wait`,
   which the live run showed leads nowhere for this cause, unless you also ask for the hint line's wording to change
   (a small build of its own, two masters)."

## What follows from it
- **Nothing is built from the answer in this chunk.** The gate, `GATE_MAX_WAIT`, `QUIET_PERIOD` and `send_hint`
  are where the base commit left them; the plan's preservation guard reads so.
- The build the answer asks for is the route's at the wrap: on a verified CLI the gate waits for the input box on a
  quiet literal-less screen, and the bound rises to 8.5 s; with it one by-path re-verify of 2.1.287 (5 live
  starts), `send`'s longest block at 18.5 s, and the fake agent's 8 000 ms hold cap raised.
