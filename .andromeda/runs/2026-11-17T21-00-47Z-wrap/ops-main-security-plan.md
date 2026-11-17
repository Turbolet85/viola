@row_set n=1 row=decision/verify-probe-dirs-cleanup cell=except seen=1 detector=none
value: SIGKILL, a crash or a failed handler install leave the directories, and verify's own trust record in the Claude config, to be removed by hand.
basis: fct-ytf3ugzg, fct-2t56x2kc
because: verify's Run F now writes its own trust record and its guard removes it on the same exits as the probe directories, so what a SIGKILL leaves now includes that record.
@row_set n=2 row=decision/probe-artifacts-cleanup cell=except seen=1 detector=none
value: a kill or a crash leaves them, and verify's own trust record in the Claude config, to be removed by hand
basis: fct-ytf3ugzg, fct-2t56x2kc
because: verify's own trust record is now a probe artifact removed on pass, fail and stop, and a kill leaves it like the rest.
