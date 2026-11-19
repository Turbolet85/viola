@row_set n=1 row=decision/hook-payloads-recorded-by-verify cell=except seen=3 detector=none
value: StopFailure has no recorded payload, ledger row or verify probe, kept on one live reading (2.1.328, SF, server_error), none claimed beyond it; its tests use synthetic payloads.
basis: fct-vj6tsu22
because: The StopFailure exception is settled as outcome B on the green live reading of one connection-lost payload, so the except cites that reading and claims nothing for the other error words or versions.
