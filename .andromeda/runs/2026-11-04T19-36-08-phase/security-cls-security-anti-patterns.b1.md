@answer n=1 entity=security-anti-patterns#1.1 v=yes
why: the Unix client-side server verification (need 5) and the macOS euid-only carve-out (need 6) under the peer and server checks (need 4) that every channel frame must pass
@answer n=2 entity=security-anti-patterns#2.1 v=no
why: run --mutants scratch wipe is the not_needed mutation item (need 5) and its guard is Windows-only, so the chunk reaches no scratch
@answer n=3 entity=security-anti-patterns#3.1 v=yes
why: the test-side home keepers prepare_home_base and the target/e2e-home removal rules are the chunk's own need 18, which the Unix fake-agent tests now hit
@answer n=4 entity=security-anti-patterns#4.1 v=yes
why: the hook contract on every OS (need 11) and the Unix statusline row (need 8) run under the stamp gate, so the Unix suite's hook dialog tests must hold it
@answer n=5 entity=security-anti-patterns#4.2 v=yes
why: the Unix home strict-modes check before the first read in every process (need 7) covers verify's ordering over ledger/stamps.json
@answer n=6 entity=security-anti-patterns#5.1 v=yes
why: the scope names the seams FAKE_AGENT_PUMP_DELAY_MS and FAKE_AGENT_HOOK_PANIC (need 14) and the NO_COLOR, TERM and COLORTERM reads (need 13), and none may switch off a control
@answer n=7 entity=security-anti-patterns#5.2 v=yes
why: no env, flag or feature seam may force a refusing peer-euid decision (need 4), which the Unix suite's peer_admitted wiring must honour
