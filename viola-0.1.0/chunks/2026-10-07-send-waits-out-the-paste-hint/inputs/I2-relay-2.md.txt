The answers to this run's P4 fork round (AskUserQuestion, four questions, 2026-10-07, after research), verbatim, as they arrived in the session:

Question 1: "A human key pressed while the gate waits takes the wheel at once, but send reads the wheel only at its arrival (src/run/send.rs:314), so the waiting send still pastes onto the human's typing once the input box returns. That wait ended 0.63 s in before this build; now it can run 8.5 s after every long paste. The same holds for a turn that starts during the wait (a residual accepted on 2026-10-04 at 5 s). What does the waiting send do after the wait?"
Chosen: "Re-read both rungs"
Notes: "overseer (a technical fork; the founder is away): checked send.rs — the wheel is read once before gate.wait_ready. Re-read the wheel and the running turn after the wait, in the documented order, nothing typed. The human always wins must hold through the wait he ordered; this tightens an accepted residual, it widens nothing. Listed for him as a fact, not as an open question."

Question 2: "The verify_window_ tests wait the gate's maximum four times by design: about 21 s today, about 35 s at 8.5 s. Their kill lines are 45 s on CI (15 s x 3) and 30 s under the mutants profile (15 s x 2), so the mutants kill is crossed and CI's margin falls from 24 s to 10 s. What happens to the two overrides in .config/nextest.toml?"
Chosen: "Move both in proportion"
Notes: "overseer (technical): the class waits the bound four times by design and he moved the bound, which is the designed-floor case of testing.md 2026-10-05. CI 20 s x 3 and mutants 15 s x 3, with the planted-hang control read both ways in evidence. No other kill and not WITHIN."

Question 3: "A hint that outlasts the bound is still refused input-not-ready, now at 8.5 s with nothing typed. At which layer is that refusal proved? test-plan §11 names three deadlines an end-to-end case may block on, and the gate's bound is not one of them."
Chosen: "Unit layer, injected clock"
Notes: "overseer (technical): the refusal at 8 500 ms and Wait at 8 499 ms by labelled clock cases; the cross-process input-not-ready path keeps its end-to-end proof in the trust-dialog case. The fake agent cap rises to 10 000 ms for verify's own hint case only."

Question 4: "The input-not-ready hint line reads '<name> was not ready for input; viola wait <name>, then send again'. The entry leaves its wording to this chunk. After this build it is no longer printed for the paste-hint cause; it stays for a modal, a screen that never goes quiet, a poisoned screen and a hint past the bound. A wording change is the founder's (two masters): answered here it is provisional and listed for him."
Chosen: "Leave the line"
Notes: "overseer: no product string moves. I list for the founder that the line still advises viola wait for its remaining causes; a rewording is his and is not provisional work here."

The options as shown, by label. Question 1: "Re-read both rungs (Recommended)" · "Re-read the wheel only" · "Leave both as they are". Question 2: "Move both in proportion (Recommended)" · "Move the mutants kill only" · "Keep verify's fallback at 5 s". Question 3: "Unit layer, injected clock (Recommended)" · "Also an end-to-end case". Question 4: "Leave the line (Recommended)" · "Reword it, provisionally".
