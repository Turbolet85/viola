@row_set n=1 row=decision/snapshot-replay-fields cell=rule seen=4 detector=none
value: A corrupt or unsupported-v snapshot is replayed from the event log, recovering only links, agent_session_id, wheel, lineage, launch and window with dialog_pending false, the last such line winning (a later lineage line with no parent clears it); the replay writes no file and gives no cwd.
basis: fct-wrqnldxo
because: The replay now recovers the window record too, so the row's closed list of recovered fields reads false without it.
@row_add n=1 row=decision/live-rig-clean-start area=e2e-test-strategy detector=none
name: Live rig runs clean of the firing session
rule: A live rig's stand-in program takes every argument the wrapper puts before a program's own words, the rig unsets every inherited VIOLA_* name before it starts anything, and every board read it makes runs with no claude on its PATH.
why: A stand-in refusing the wrapper's arguments ends at once, an inherited name leaks the firing session into the scratch home, and a board read under PATH runs the real claude under the user's config.
state: provisional
basis: fct-njf4xxzw, fct-spgqytne
because: R2's red was measured to these three rig faults and the rig was mended to hold each, on the overseer's relay, so the rule every later live rig keeps lands provisional.
