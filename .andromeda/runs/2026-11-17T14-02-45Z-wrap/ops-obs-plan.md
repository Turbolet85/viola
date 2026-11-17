@row_set n=1 row=decision/direct-chrono-dependency cell=rule seen=1 fact=F1 detector=none
value: The root binary takes chrono directly, without default features, for the millisecond UTC timestamp, and keeps it though unmaintained: RUSTSEC-2026-0543 stands in the cargo deny ignore list on the unmaintained-upstream ground, re-read at each epoch boundary; jiff and time are not adopted.
basis: fct-s6xve3jr
because: The direct chrono dependency now stands under an admitted unmaintained advisory with no patched version, so the row that decides chrono is taken must say it is kept on that ground and its alternatives not adopted.
