@answer n=1 entity=surface#1.1 v=no
why: the amber SGR bytes of the DIALOG word; the chunk adds no colour dependency and asserts no styled byte, which item 7 keeps in the SGR module's own code.
@answer n=2 entity=surface#1.2 v=no
why: the stale dim SGR value; the chunk's code and tests neither emit nor assert it.
@answer n=3 entity=surface#1.3 v=no
why: the callsign bold SGR value; the chunk's code and tests do not reach it.
@answer n=4 entity=surface#1.4 v=no
why: the reset SGR value; the chunk's code and tests do not reach it.
@answer n=5 entity=surface#1.5 v=no
why: the no-blue ban on link output colour; the link line's plainness is held by surface#11.1, and this colour rule is not reached.
@answer n=6 entity=surface#1.6 v=no
why: the background-never-set rule; the chunk's code and tests set no background.
@answer n=7 entity=surface#2.1 v=yes
why: the human stderr forms of refusals and errors, including the fixed unable form, which item 9's enum folds must keep.
@answer n=8 entity=surface#2.2 v=yes
why: the code-word screen of a peer's refusal and detail in both clients, the closed detail words item 9 keeps through the Refused fold.
@answer n=9 entity=surface#2.3 v=yes
why: the unknown and no-detail readings of a peer's refusal and detail, which set the --json refusal and detail documents item 9 keeps.
@answer n=10 entity=surface#2.4 v=yes
why: exit 14 for a code word outside the closed refusal list, a typed exit code item 9 keeps.
@answer n=11 entity=surface#3.1 v=yes
why: the TIOCGWINSZ width read lives in crates/viola-pty, which the pty move reaches whole.
@answer n=12 entity=surface#4.1 v=yes
why: the as-built board read rule that item 11 rules on, and the six fixed list columns item 2 keeps.
@answer n=13 entity=surface#4.2 v=yes
why: the exit-21 strict-modes refusal with its fixed unable and hint pair and its --json document, which item 9 keeps.
@answer n=14 entity=surface#5.1 v=no
why: the board's strict-modes scan order is board code no reached file holds; its refusal is bound by surface#4.2.
@answer n=15 entity=surface#6.1 v=yes
why: names the ungated ubuntu-latest witness of the stale-row colour layer, a check the Linux-only green CI reads.
@answer n=16 entity=surface#7.1 v=yes
why: the per-reason refusal hints and the fixed human forms item 9 keeps and item 1 pins, with no hint naming release (item 3).
@answer n=17 entity=surface#8.1 v=yes
why: the byte-exact server-verification hint of eight verbs, a fixed human form that items 1 and 9 reach.
@answer n=18 entity=surface#8.2 v=yes
why: the socket-directory refusal at eight verbs, exit 21 with its socket-dir-refused document, which items 9 and 10 reach.
@answer n=19 entity=surface#9.1 v=yes
why: states the socket-directory pair's wording as an open planning form, the status item 10 asks the chunk to honour.
@answer n=20 entity=surface#10.1 v=yes
why: the viola release --budget line and its date form, the phraseology item 3 re-words or defers for v1-13.
@answer n=21 entity=surface#11.1 v=yes
why: the link and unlink result line, plain ASCII with exit 0, the output phraseology item 3 reaches.
@answer n=22 entity=surface#11.2 v=yes
why: the since clock form of the link line, the phraseology item 3 reaches.
@answer n=23 entity=surface#11.3 v=yes
why: the already-linked form and the unlink of an unheld pair, the phraseology item 3 reaches.
@answer n=24 entity=surface#12.1 v=yes
why: the verify stop message is a fixed exit-1 stderr form that items 1 and 9 reach; the step counter and summary are excluded by not-needed item 8.
@answer n=25 entity=surface#13.1 v=yes
why: the verify home-state pair and the stamps.json strict-modes refusal at exit 1, reached through the StampsError fold (item 9).
