## 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host — the paste hint measured on the live CLI, and the founder's decision
**Section:** §Established Decisions → [CLI Version Compatibility] (the `viola verify` paragraph: the paste-hint passage)
**Change:**
- What `send` does with the hint screen is measured on the real CLI as well as under the fake agent: on a verified CLI a `send` issued while the hint stands ends `not-delivered` / `input-not-ready` with nothing typed, 0.63 s after it was issued. Was "measured end to end under the fake agent only" and "no `send` was run in the real CLI's hint window".
- The hint is a timer of 8.0 s from the last long paste (nine timings, 8.000 s to 8.023 s; a second long paste restarts it, a short one does not), 3.8 s to 7.0 s of it after the turn's Stop.
- A no-cursor `viola wait` issued in the window woke on nothing and ran to its deadline; `wait --after` the earlier cursor returns at once, inside the window. The `input-not-ready` hint line's advice does not lead out of it.
- On an unverified CLI the partial gate typed a short text under the hint and the CLI submitted and confirmed it. A long or a repeated text under the hint is unmeasured.
- What `send` should do is decided: on a verified CLI the gate waits for the input box on a quiet screen with no literal, and the bound rises to 8.5 s. Was "the founder's decision and is open".
- It is not built: the gate, `GATE_MAX_WAIT`, the hint line and the fake agent's 8 000 ms hold cap stay as they are until the route entry that builds it lands.
**Why:** two hint runs on `claude` 2.1.287 gave the numbers; the founder chose on a priced card (his own live answer of 2026-10-07T10:29Z, relayed by the overseer), having been told the price: one by-path re-verify of 5 live starts, `send`'s longest block 18.5 s, the fake agent's hold cap to raise, no human keystroke delayed.
**Kept:** Run B's own measured sentence (8.0 s from the paste, 6.5 s after that turn's Stop) and its wait rule.
**Ref:** .andromeda/runs/2026-10-07T10-53-41-wrap/
