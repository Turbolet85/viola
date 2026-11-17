@row_set n=1 row=decision/permission-request-question-stays-null cell=why seen=3 detector=none
value: A PermissionRequest allow with updatedInput would satisfy it, but that body has no ledger row; adding one needs a further relied row, its viola verify probe and a re-stamp of every stamped version.
basis: fct-skwar3iy
because: The relied ledger rows moved from 22 to 23, so the row's «22nd» names a row that now stands; the cell says a further row, with no count to go stale; the proposer's identical set was read «landed before» at tx 1193 and did not land, the row still reading «22nd».
