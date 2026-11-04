@answer n=1 entity=span-trace-coverage#15.1 v=no
why: the session moved no product code, and the torn-append heal rule lives in src/ with no reached file holding it
@answer n=2 entity=span-trace-coverage#16.1 v=no
why: the capability-ledger read and parse-rejected rule live in run's version gate in src/, which the session left unchanged
@answer n=3 entity=span-trace-coverage#17.1 v=yes mode=stale
why: the session reached tests/cli_verify.rs and tests/support/mod.rs, where the CI verify-against-fake-agent harness and stamped_home fixture live, and the fake-agent references moved
