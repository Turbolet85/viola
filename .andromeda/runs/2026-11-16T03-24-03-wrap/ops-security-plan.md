@row_set n=1 row=decision/stop-background-tasks-count-only cell=rule seen=1 fact=F1 detector=none
value: Stop's background_tasks is carried only as its length, no element read, and is the only count a turn end carries, no scheduled-wake count crossing; null when absent, null or not an array, and a non-array also writes a drift entry.
basis: fct-j7joxnr6 (viola carries no scheduled-wake count) · fct-tq4ym3ir (session_crons as a count a boundary widening, the founder's to give)
because: The chunk pins that a turn end carries background_tasks as its one count and no scheduled-wake count, so the row stating what crosses must say it is the only count.
@row_set n=2 row=decision/tests-use-own-home cell=rule seen=1 fact=F2 detector=none
value: Development handles secrets as production does, and tests use their own --home directory, each a copy of one viola verify stamp per built binary, never a link, sharing no file with the stamp or another test's home.
basis: fct-xueqgrc6 (stamped_home copies one stamp per built binary, taken under an exclusive lock) · fct-j7joxnr6 (each stamped home a copy, never a link) · fct-777k7lv6 (one stamp per binary, no shared file, tamper isolation)
because: Each test's own home is now a copy of one shared stamp, and the isolation it promises holds only while no file is shared, so the row must say so.
@row_set n=3 row=decision/e2e-home-removal-own-dirs-only cell=rule seen=1 fact=F3 detector=none
value: A test-side removal under target/e2e-home never takes anything but a dir the test side itself created, and takes another build's stamp dir only when it takes that stamp's lock with try_lock, never a stamp a running build holds.
basis: fct-xueqgrc6 (the stamp dir has no owner record; other builds' stamps are swept only when their lock is taken with try_lock)
because: The test fixture now sweeps other builds' stamp dirs, which carry no owner record, so the ban must name the lock condition that keeps a running build's stamp safe.
@row_set n=4 row=decision/test-removal-own-dirs-only cell=rule seen=1 detector=none
value: A test-side removal under the e2e home takes only a directory the test side itself created, by its owner record, its own temp dir or its session name prefix, or another build's stamp dir whose lock it takes with try_lock.
basis: fct-xueqgrc6 (the stamp dir has no owner record; other builds' stamps are swept only when their lock is taken with try_lock)
because: The row lists every way a test-side removal knows its own dir, and the stamp sweep's lock is a new way not among them, so the list is now incomplete.
