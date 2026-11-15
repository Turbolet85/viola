# Relay 2 — the overseer's answer to the rehearsal key-order technical fork (implement P2, 2026-11-15)

Question (asked as a dialog): the fake agent keeps a typed DEL in its prompt buffer (`src/bin/viola-fake-agent/input.rs`
`plain()`), so step 3(d)'s `x`+DEL leaves `x\x7f` ahead of the send's text; the rehearsal read
`send-issued/prompt-submitted/wheel/send-refused` (`not-delivered`/`no-prompt-submitted`) and the question raised with the
human on the wheel, so no hold. How should the rehearsal order it?

Selected: "Keys after the hold (Recommended)"

Notes, verbatim:

The overseer's technical answer (no founder fork): in rehearse-wrapped only, the keys run after the hold; live-wrapped keeps the plan's order. Two bindings, because the cap is 9 and a red step is not re-fired: (1) «the real CLI erases on DEL» is a behaviour statement — say in the outcome whether it is MEASURED (cite the recording or ledger row) or not; (2) live-wrapped checks it instead of trusting it: after x and DEL and before `release` and the send, the rig reads the drained bytes for the erased box and, if the `x` still stands, types nothing more, sends nothing, records the reading and ends the step there. Rehearse that refusal arm under the fake, whose no-erase gives it for free. The fake's DEL fidelity is NOT changed in this chunk: name it as a Carry-forward with its owner entry for the wrap. Record the rehearsal-only order as a deviation.
