@row_add n=1 row=decision/run-lineage-flags-checked area=input-validation fact=F1 detector=D-security-input
name: viola run's lineage flags checked before anything is read
rule: viola run's --parent goes through the viola name check and --role must be a code word; a --parent equal to the instance's own name is a usage error, exit 2, before anything is read.
state: provisional
basis: fct-yurmbmhi, fct-evi6pgaa
because: The chunk adds two CLI input flags, and the plan's input validation must name the checks that bound them.
@row_add n=2 row=decision/lineage-parent-authenticates-nothing area=authentication-authorization fact=F2 detector=D-security-auth
name: Self-reported parent authenticates nothing
rule: An instance's parent is read as the parent only — never the instance, never an identity, never joined into a path — taken from --parent or else the inherited VIOLA_NAME and dropped when it is the instance's own name; it authenticates nothing.
guard: lineage_is_read_by_no_wheel_link_send_or_answer_path
basis: fct-yurmbmhi, fct-ao5bhrj4
because: The inherited VIOLA_NAME gains a second use as a self-reported parent, and the plan must say it grants no authority.
@row_add n=3 row=decision/board-lineage-and-links-name-checked area=input-validation fact=F3 detector=D-security-input
name: Board lineage and links shown only when they re-parse
rule: The board shows a recorded parent only when it re-parses as a viola name other than the row's own, a role only when it is a code word, and a link pair only when both names parse; a gone instance gives no links.
state: provisional
basis: fct-yurmbmhi
because: The board now reads parent, role and link names back from state files, and the plan must name the check that bounds what it shows.
@row_add n=4 row=decision/lineage-kept-to-snapshot-and-event-log area=data-protection fact=F4 detector=none
name: Lineage kept to the snapshot and the event log
rule: An instance's lineage (parent and role names) is kept only in the snapshot and one lineage line of the event log at start, with no process-log line, span or terminal byte.
basis: fct-yurmbmhi
because: The chunk stores a new pair of names, and the plan must say where they may stand.
@row_add n=5 row=decision/list-carries-lineage-and-links area=data-protection fact=F5 detector=none
name: List output carries lineage and link pairs
rule: viola list --json and the MCP list tool carry each item's parent and role and the listed instances' link pairs, so logged instance names reach the MCP list result.
state: provisional
basis: fct-uoqmd72r, fct-j3icgnqw, fct-vxm4o5ea
because: Logged instance names now cross into the MCP list result, a boundary widening landed on a delegate's word that the plan must record.
@row_add n=6 row=decision/lineage-from-real-snapshot-only area=input-validation fact=F6 detector=D-security-input
name: Lineage revived only from a real snapshot
rule: The replay takes the last lineage line as the lineage, reading a non-string member as absent, and revive keeps the parent and role only from a real snapshot, never from a replayed one.
state: provisional
basis: fct-yurmbmhi
because: The lineage line is a new value read back from the event log, and the plan must say how far a replayed value is trusted.
@row_none n=1 fact=F7 type=4
relied: the harness lifecycle test asserts no parent on harness items; the test harness's spawn code and test-plan's test-started-viola rows hold the VIOLA_* removal.
