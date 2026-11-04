@answer n=1 entity=bootstrap-phases#1.1 v=yes
why: its Unix socket directory, 0600 chmod after bind, peer euid check and 0700/0600 home strict-modes with the Unix settings override are the chunk's items 3, 4, 5, 7 and 8.
@answer n=2 entity=bootstrap-phases#2.1 v=yes
why: the artifact canary scan's gating of scan-gated uploads binds the chunk's Unix-leg scan and the new perf-<os>, diag-perf-<os> and test-home uploads it adds to that list.
