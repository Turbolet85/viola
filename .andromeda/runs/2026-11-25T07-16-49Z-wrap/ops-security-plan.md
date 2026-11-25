@row_set n=1 row=decision/window-step-moves-own-terminal-window cell=rule seen=1 fact=F3 detector=none
value: The wrapper's window step moves only the window of its own terminal, found by its parent pid within a 5 s wait, at most once, and never a window it did not open; it switches back once to the workspace active before it only when the move changed it; a failed read or restore changes no placement.
basis: fct-yf2imaoq; fct-py27nrcw
because: The window step now reads the monitors before and after its move and may send one workspace switch back to the previously active workspace, a boundary widening the row must name.
@row_set n=2 row=decision/verify-never-answers-import-dialog cell=rule seen=1 fact=F1 detector=none
value: viola verify against the real claude runs from a folder the user trusts whose external CLAUDE.md import is already answered; verify never answers that dialog, and a run that first meets the trust or import dialog is refused untrusted-folder, exit 1, with no stamp written.
basis: fct-yf2imaoq; fct-froyousk
because: A verify whose Runs B to E first read the trust or imports dialog is now refused untrusted-folder before any row is checked or stamp written, which the row requiring a pre-trusted folder must state.
@row_add n=1 row=contract/verify-refusal-codes/untrusted-folder area=data-protection fact=F2 detector=none
boundary: verify's refusal codes
item: untrusted-folder
promise: One of the closed codes verify writes on its process-exit when refused: a run that first read the trust or imports dialog, refused before any row is checked or stamp written, its unable and hint pair fixed and holding no path.
basis: fct-vmrlleio; fct-wnswac6i; fct-yf2imaoq
because: verify's closed refusal codes gain untrusted-folder, and the closed list holds one row per code.
@row_add n=2 row=decision/r7-place-script-closes-own-terminal area=authentication-authorization fact=F4 detector=none
name: R7 placing script acts only on its own terminal
rule: The test-only R7 placing script opens one terminal window of its own through viola open, starts no claude but an owner-only stand-in answering its version, unsets every inherited VIOLA_ name, refuses on a locked desktop and closes only its own terminal.
basis: fct-u6ko7uul; fct-ykqoxywq
because: A new test-only live script opens and closes a terminal window on the real compositor, and what it may touch is a boundary future work must respect.
