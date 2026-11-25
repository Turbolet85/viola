@row_add n=1 row=decision/r7-place-script-closes-own-terminal area=authentication-authorization detector=none
name: R7 placing script acts only on its own terminal
rule: The test-only R7 placing script opens one terminal window of its own through viola open, starts no claude but an owner-only stand-in answering its version, unsets every inherited VIOLA_ name, refuses on a locked desktop and closes only its own terminal.
state: provisional
basis: fct-u6ko7uul; fct-ykqoxywq
because: A new test-only live script opens and closes a terminal window on the real compositor, and its bounds stand on the plan's step 15 confirmed on a delegate's card, so the row lands provisional until the operator's own word.
