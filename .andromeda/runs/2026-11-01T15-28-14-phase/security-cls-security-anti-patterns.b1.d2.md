# Classified — security · security-anti-patterns.b1

## Answers
security-anti-patterns#1.1 · yes · the frame ban before server verification: the chunk's client euid check runs before the first frame of every Unix client, and the macOS pid gap is the named carve-out this entry owes
security-anti-patterns#2.1 · no · the `run --mutants` scratch wipe guard is Windows-host only and the chunk's scope does not reach it; the Windows arms do not move
security-anti-patterns#3.1 · no · standing S3 · each new test-side removal under `target/e2e-home` takes only a directory the test side itself created
security-anti-patterns#4.1 · no · the stamp rule governs non-null hook decisions and the ledger stamp; the chunk changes neither, and its hook refusal only fails open with no body
security-anti-patterns#4.2 · no · the ordering rule for `viola verify` and `update_stamps` reads and writes of `ledger/stamps.json`; neither verify nor `strict::check_stamps` is a reached file, and the chunk's home check is `board::scan`'s
security-anti-patterns#5.1 · yes · no `config.json` key, `VIOLA_*` variable or CLI flag may reach the socket-directory check, the peer check or the home check, which the chunk's scope names
