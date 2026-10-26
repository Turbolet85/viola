# Classified — security-plan · data-protection.b1

## Answers
data-protection#1.1 · no · the lock delta is tokio's entry alone and tokio 1.53.3 keeps its dependency set, so the sha2 =0.11.0 pin and its cc-free graph still hold (cargo deny exits 0), and no mode, DACL, persist helper or probe dir moved
data-protection#2.1 · no · the fixture recording, scrub and refusal rules live in no reached file, and the test-only tap's take(MAX_FRAME) and 0600 capture are unchanged
data-protection#3.0 · no · the reached files are Cargo.toml and Cargo.lock only, and the session brings no live CLI reading, stamp or plugin install, so the accepted risks and residuals still hold
