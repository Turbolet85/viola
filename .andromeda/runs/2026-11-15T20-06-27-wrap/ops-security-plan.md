@row_add n=1 row=decision/test-sockets-in-short-tmp-dir area=security-anti-patterns-never-do-these fact=F1 detector=none
name: Test channel sockets in a short test-owned directory
rule: Test channel sockets bind inside a fresh test-owned short directory under /tmp, made per test, never under TMPDIR and never at the /tmp root; the product socket rules are unchanged.
why: A socket path under a long TMPDIR overflows sun_path and the bind fails.
basis: fct-w5d65k72
because: The chunk moved every test channel socket to a fresh short /tmp directory after a sun_path bind failed under a long TMPDIR, a test-side socket convention no row of the area states.
@row_add n=2 row=decision/record-test-home-no-home-component area=data-protection fact=F2 detector=none
name: Record test homes hold no component named home
rule: A home a test gives to viola verify --record comes from the record-home helper and holds no path component named home.
guard: record_home_names_no_path_component_home
basis: fct-iq6zeo4z
because: The chunk moved every verify --record test caller to the record-home helper whose home has no path component named home, a test convention the fixture scrub rows do not state.
