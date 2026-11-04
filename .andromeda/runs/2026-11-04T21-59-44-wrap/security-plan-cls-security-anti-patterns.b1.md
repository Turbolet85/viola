@answer n=1 entity=security-anti-patterns#1.1 v=no
why: the macOS carve-out, its scope and its lack of a route entry are unchanged; the session only names it NO_PEER_PID in the test census, and no channel code moved
@answer n=2 entity=security-anti-patterns#2.1 v=no
why: the run --mutants scratch guard lives in code the change does not reach, and no file in the change touches the wipe or its re-creation
@answer n=3 entity=security-anti-patterns#3.1 v=no
why: the three cli_instance_state tests gained only reason lines; the keepers, the owner-only removal rule and the harness cleanup are unchanged, so the claim still holds
@answer n=4 entity=security-anti-patterns#4.1 v=no
why: the stamp rule and the ledger rows live in product code (LedgerRow, update_stamps) that the session did not change
@answer n=5 entity=security-anti-patterns#4.2 v=no
why: the verify ordering before ledger/stamps.json and the strict-modes refusal are product code the session did not change; its strict-modes tests gained reason lines only
@answer n=6 entity=security-anti-patterns#5.1 v=no
why: no seam or environment read was added and the colour facts and fake-agent seams are unchanged; the new PTY witness only sets TERM on a child process
@answer n=7 entity=security-anti-patterns#5.2 v=no
why: no env, flag or feature seam was added to force a refusing peer-euid decision, and the injected-value proofs are unchanged
