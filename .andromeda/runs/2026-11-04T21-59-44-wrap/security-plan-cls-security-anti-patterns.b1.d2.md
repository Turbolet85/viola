@answer n=1 entity=security-anti-patterns#1.1 v=no
why: the carve-out itself, its scope and its lack of a route entry are unchanged, and the census only names it as NO_PEER_PID
@answer n=2 entity=security-anti-patterns#2.1 v=no
why: no run --mutants or scratch code moved, no src file changed, and no reached test file holds the wipe guard
@answer n=3 entity=security-anti-patterns#3.1 v=no
why: the three cli_instance_state.rs tests gained only reason lines and the keepers and removal rule are unchanged
@answer n=4 entity=security-anti-patterns#4.1 v=no
why: no ledger or stamp code moved, no src file changed, and cli_verify.rs gained only a reason line
@answer n=5 entity=security-anti-patterns#4.2 v=no
why: verify's strict-modes ordering lives in unchanged src code and the session only added a reason line to a verify test
@answer n=6 entity=security-anti-patterns#5.1 v=no
why: no colour, fake-agent seam or product env read was added, and the new witness sets TERM only for a child process inside a test
@answer n=7 entity=security-anti-patterns#5.2 v=no
why: no seam or env read was added to force a refusing peer-euid decision, and no new committed test forces one
