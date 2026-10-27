# Classified — security-plan · dependency-security.b1

## Answers
dependency-security#1.1 · yes · stale · the change moved pin_bytes in pin.rs, which is reached whole and also holds pin_companions' run-time SHA-256 re-hash, and the report does not show that re-hash still holds
dependency-security#2.1 · no · the session adds or bumps no dependency (tempfile 3.27.0 already provides persist_noclobber) and the deny and deny-sync bans on viola-state stay green
dependency-security#3.1 · no · fuzz/ and its lockfile are not reached and the change adds no link from fuzz into a shipped binary
dependency-security#4.1 · no · src/conpty.rs, the one home of the four ConPTY pins, is not reached and no ConPTY pin is added or moved
dependency-security#5.1 · no · no workflow file changed, so the windows-mutants matrix, its permissions and its pins stand
dependency-security#6.1 · no · the G2 exemption in scripts/g2-zero-panics.sh is not reached and the session adds no seam or exemption (the placed hook is a test parameter with no env read)
dependency-security#7.1 · no · no workflow file changed, so the four full-SHA action pins stand
