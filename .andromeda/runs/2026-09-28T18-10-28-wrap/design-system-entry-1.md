
## 2026-09-28-capability-ledger-and-viola-verify — `viola verify`'s six-row step counter and its exit-1 phraseology
**Section:** §Surface: cli Component Patterns (`verify`); Exit-code phraseology (exit 1 row)
**Change:**
- `verify`'s step counter prints one static stdout line per ledger row, `[01/06] shim-resolution claude resolves to a real executable  pass` … (six rows today; the count grows as owning chunks land rows), then the last stdout line `stamped 2.1.283  6 pass  0 fail` (was the 14-row `[03/14] S3 …` / `stamped 2.1.280  14 pass` sketch).
- Exit 1 also carries verify's refusals `unable: the claude CLI was not found` / `the claude CLI is a .cmd or .bat script` / `the CLI version could not be read` / `a recorded payload still holds a path or a username`, each with its own hint; a failing row prints no stderr word.
**Why:** the chunk landed the ledger's six rows and the verb's refusals (report Symbols; `tests/cli_verify.rs`).
**Ref:** .andromeda/runs/2026-09-28T18-10-28-wrap/
