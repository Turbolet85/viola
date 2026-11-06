@answer n=1 entity=span-trace-coverage#15.1 v=no
why: the torn-append heal line is written by the state-store appender, and the chunk touches no appender or reader (not-needed item 5); the reader neither heals nor logs.
@answer n=2 entity=span-trace-coverage#16.1 v=yes
why: the live `viola run` (v1-53) passes the version gate that reads ledger/stamps.json, and its cli_verified on process-start{subject:"claude-child"} and any parse-rejected line land in run-<name>.ndjson, the evidence the chunk reads through jq (needs items 6 and 10).
@answer n=3 entity=span-trace-coverage#17.1 v=yes
why: the portable-pty bump reaches verify's PTY runs and signalled-child stop codes, and the chunk's test, G2, G4 and secret-scan gates re-run green over tests/cli_verify.rs, the boot step 4 harness and the lines verify writes under VIOLA_NAME.
