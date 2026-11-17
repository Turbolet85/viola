@row_add n=1 row=decision/verify-writes-own-trust-record area=surface-cli fact=F2 detector=none
name: viola verify writes and removes its own trust record
rule: viola verify writes one trust record, for its own temp folder only, into the Claude config file its child reads, and removes only that record after a pass, a fail or a stop; it never creates an absent config file, and the trust-record row then fails.
why: Ratified as the one bounded exception to viola writing nothing into the Claude config, so the trust-record row can probe whether the CLI honours a record another process wrote.
guard: verify_trust_record_row_reads_config_absent_and_writes_nothing
basis: fct-ytf3ugzg, fct-yjvwqjma, fct-2t56x2kc
because: The chunk adds verify's Run F, which writes and then removes its own trust record in the config its child reads, a ruled widening no row of the cli surface states.
@row_none n=1 fact=F1 type=5
what: the relied row count and the 23rd row's id and words are an inventory the ledger code holds; the step-line, exit and summary rows already state the form with no count.
@row_none n=2 fact=F3 type=0
what: the trust writer's closed code words ride only the stamp file, never a printed line; the stamps' row-value contract is architecture's and the closed set is held by the code.
