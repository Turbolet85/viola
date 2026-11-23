@row_set n=1 row=decision/snapshot-replay-fields cell=rule seen=3 detector=none
value: A corrupt or unsupported-v snapshot is replayed from the event log, recovering only links, agent_session_id, wheel, lineage and launch with dialog_pending false, the last lineage and launch line winning (a later lineage line with no parent clears it); the replay writes no file and gives no cwd.
basis: fct-ydz7awsz
because: The replay's recovered set gains the launch pair, replaced whole by the last launch line (apply_launch_line), so the fixed set and its last-line rule must name it.
