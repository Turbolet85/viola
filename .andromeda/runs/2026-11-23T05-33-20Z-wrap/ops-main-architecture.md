@row_set n=1 row=decision/snapshot-replay-log-fields-only cell=rule seen=2 detector=D-arch-resources
value: A snapshot rebuilt by replay recovers only links, agent_session_id, wheel and lineage from the log; a field no line gave is absent, never a default.
basis: fct-yurmbmhi
because: The replay now recovers lineage as a fourth log-derived field, so the row's closed list of three is false; this is the shorter set the fit reader's not_master on the proposer's longer one leaves owed.
@row_set n=2 row=decision/board-read-model-shape cell=rule seen=2 detector=D-arch-decisions
value: The board read model is sync and std only, carries no Claude shape, and none of its types holds endpoint, pinned_bin, cwd, pid or started_at.
basis: fct-yurmbmhi
because: The board now holds a links list, so the row's ban on holding links is false; the ban is cut and nothing is added, the shorter set the fit reader's not_master leaves owed.
@row_add n=1 row=contract/list-item/parent area=standard-contracts detector=D-arch-resources
boundary: list payload item
item: parent
promise: Present only when the recorded parent parses as an instance name other than the item's own.
basis: fct-yurmbmhi
because: A list item gains an optional `parent` kept only under a name check, a promise to list readers no row holds; the add rests on the overseer's F6 and F8, a delegate's word.
@row_add n=2 row=contract/list-item/role area=standard-contracts detector=D-arch-resources
boundary: list payload item
item: role
promise: Present only when the recorded role is a code word.
basis: fct-yurmbmhi
because: A list item gains an optional `role` kept only when it is a code word, a promise to list readers no row holds; the add rests on the overseer's F6, a delegate's word.
