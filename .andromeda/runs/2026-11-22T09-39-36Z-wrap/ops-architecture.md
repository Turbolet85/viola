@row_set n=1 row=decision/human-key-never-held-behind-resize cell=why seen=1 fact=F1 detector=D-arch-decisions
value: On Linux, real claude kept a key typed 0, 20 or 100 ms after a resize of its own pty in every measured round, so viola adds no wait; the key-loss reading is a Windows-ConPTY measurement.
basis: fct-rdondmwf
because: The H2 question is now measured on Linux with no key lost after a resize, which is the reason the standing no-hold decision lacked and the fact F1 records.
@row_set n=2 row=decision/no-key-held-behind-resize cell=except seen=1 detector=none
value: The test-only live rig's KR arm and its rehearsals type a key right after a resize of their own pty, to measure whether the key is kept.
basis: fct-qulhmgcs
because: The KR arm and rehearsals deliberately write a key 0 to 100 ms after a resize without waiting for the child to observe the new size, so the row's test rule now has a named exception.
