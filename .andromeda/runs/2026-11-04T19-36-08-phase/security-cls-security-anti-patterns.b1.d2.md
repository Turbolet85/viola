@answer n=1 entity=security-anti-patterns#1.1 v=yes
why: the Unix server and client peer checks (items 4 to 6) must verify the server by euid, pid and start time, or euid alone on macOS, before any channel frame is written
@answer n=2 entity=security-anti-patterns#2.1 v=no
why: the run --mutants Windows host scratch wipe is Windows-only and out of scope (not_needed n=5), and no Unix code or test the chunk touches reads it
@answer n=3 entity=security-anti-patterns#3.1 v=yes
why: the test-side home keepers and the target/e2e-home removal rule are named by item 18 and hit by the fake-agent tests on Linux and macOS
@answer n=4 entity=security-anti-patterns#4.1 v=yes
why: item 11 requires viola hook to print nothing on every OS when a security check fails, and this stamp rule sets the unverified-cli outcome of the hook and answer on Unix
@answer n=5 entity=security-anti-patterns#4.2 v=yes
why: item 7 requires the Unix home strict-modes check before the first read in every process, and this rule orders verify's read and write of ledger/stamps.json after that check
@answer n=6 entity=security-anti-patterns#5.1 v=yes
why: the scope names the FAKE_AGENT_PUMP_DELAY_MS and FAKE_AGENT_HOOK_PANIC seams (item 14) and NO_COLOR, TERM and COLORTERM (item 13), and this rule rules over what each may switch off or reach
@answer n=7 entity=security-anti-patterns#5.2 v=yes
why: item 4 names the Unix server-side peer-euid check, and this rule forbids any seam that forces a refusing peer-euid decision in it
