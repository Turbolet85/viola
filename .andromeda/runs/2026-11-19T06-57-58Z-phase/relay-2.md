# Relay 2 — the P4 fork on entry 18, and its answer (verbatim)

## The question put (AskUserQuestion, P4)
Implement re-enters at L2 with `gate.py run --live-legs --entry 17`. That call fires the census before L2 (15), the debug build (16) and L2 (17), but not entry 18, the rig census after L2. The tool fires a round step after the last leg only in the whole round, and the whole round would re-fire L1 (gate-contract §Live round). How should the after-L2 census be read?

1. Re-key 18 operator (Recommended): entry 18's leg becomes 'operator'; the span is then 9–17, and implement fires it once with `--operator 18` right after the round; `run` and `expect` unchanged; a second block edit beyond the relay's «entry 14's expect alone», named on the card.
2. Leave 18, not fired: the after-L2 reading rests on the tool's census of entry 17's tree and implement P4's `gate.py census`; the rig's own census never runs after L2.

## The answer
Chosen: «Re-key 18 operator (Recommended)», with these notes:

Overseer's technical answer, 2026-11-19T07:00:09Z: re-key 18 to the operator leg, named on the card. Two more things for the card, both on entry 14 and its carry: (1) the mended expect must still be able to FAIL — say in the plan which reading turns it red: either needle word reading `unread` (the read not taken), or the needle `present` after the run in either file where it was absent before; write the atoms so each of those fails the entry. (2) «why no new projects entry appeared for the probe folder» is a named CARRY proposal for the wrap with its next reader (the next live start that reads a real config by key names), not only a sentence.

Authority: the overseer's technical answer (delegate overseer), given through the operator's dialog.
