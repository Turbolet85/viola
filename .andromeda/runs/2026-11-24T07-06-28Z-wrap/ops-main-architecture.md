@row_add n=1 row=decision/up-selection-rule area=established-decisions fact=F3 detector=none
name: up re-opens only instances gone before the boot
rule: viola up skips a live or stale instance and takes a gone one only when its heartbeat is older than the host boot time and, unless it was named, its events log has no session-end after its last session-start; an unnamed instance ended on purpose stays down.
basis: fct-orhmfpak, fct-urp643y2
because: The chunk fixes which instances up re-opens on the operator's own answer to fork U1 (arm 1, inputs#I18), and a named instance skips the session-end reading, which the proposer's rule left out.
@row_add n=2 row=decision/up-parents-first area=established-decisions fact=F4 detector=none
name: up re-opens parents before children
rule: viola up re-opens its candidates one at a time, parents before children; a candidate whose parent names no other candidate is a root, and a parent cycle is taken once.
state: provisional
basis: fct-o5x32fkq
because: The chunk orders up's re-opens by declared parent on technical fork T2, a delegate's word, so the decision lands provisional.
@row_add n=3 row=decision/up-writes-no-state area=standard-contracts fact=F8 detector=none
name: up writes no state of its own
rule: viola up writes no snapshot, event line or stamp, leaving each to the revive it spawns; its lines, hint and --json document hold instance names and closed words only, never a window address, pid or workspace; under --dry-run it spawns nothing.
state: provisional
basis: fct-lwvzzjno, fct-ykd4mawd, fct-v57w6gal
because: The chunk fixes that up leaves every state file to the revives it spawns and withholds window identities from its output, a P5 answer given by a delegate.
