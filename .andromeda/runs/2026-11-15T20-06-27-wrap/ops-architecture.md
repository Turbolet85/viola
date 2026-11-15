@row_set n=1 row=decision/root-waits-share-one-bound cell=rule seen=1 detector=none
value: Root test waits on a child share the one 12 s WITHIN bound of the watch support.
basis: fct-w5d65k72
because: The shared root wait bound WITHIN moved from 7 s to 12 s, and the rule cell holds that bound's value.
@row_add n=1 row=decision/test-sockets-under-short-tmp-root area=occupied-resources fact=F1 detector=D-arch-resources
name: Test channel sockets under a short /tmp root
rule: Test channel sockets bind under a test-owned short root directory made under /tmp, never under TMPDIR; the product endpoint socket is not under that root.
why: So the socket path length bound holds whatever the length of TMPDIR.
basis: fct-w5d65k72
because: Every viola-test-chan- socket now binds under the short /tmp test root instead of under TMPDIR, a location no architecture row registers.
