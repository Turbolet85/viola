# design-system — amendments

## 2026-09-27-browser-verdict-reachability — render and contrast judged on the ubuntu leg while the browser runs on three OSes
**Section:** §Typography (the per-OS fallback rationale; "Assertions hold on the Linux fallback") · §Surface: web-spa → Platform-Specific Notes (Fonts: "Linux is the CI render")
**Change:** the headless GUI checks' render and contrast verdicts are judged on the ubuntu leg (was "the headless GUI checks run on ubuntu"); the browser suite itself runs on all three CI OSes, where those assertions are not the verdict. Linux stays the CI render, with DejaVu Sans Condensed + DejaVu Sans Mono resolved.
**Why:** founder ruling W125 put the browser pipe on all three CI OSes (chunk 2026-09-27-browser-verdict-reachability); the font stacks and the DejaVu requirement are unchanged.
**Kept:** the "the CI render" labels in the font table and the token comment, and the 2026-09-24 Decisions Log line, stand as written.
**Ref:** .andromeda/runs/2026-09-27T19-50-23-wrap/

## 2026-09-28-cli-output-tokens — clap built without `color`
**Section:** §Surface: cli → Toolkit / Framework
**Change:**
- clap 4.6.7 (derive) is built without its `color` feature: its help and usage output is plain in every mode and no dependency reads a colour or terminal variable, so every styled byte of human output is viola's own SGR module's (was "clap 4.6.7 (derive), Rust stable").
**Why:** clap's `color` put bold/underline headers into `--help` and let `CLICOLOR_FORCE` force SGR into a pipe (research F1), against the cli colour ban; the chunk turned it off (report Dependencies; `cli_output_plain` green on 3 OSes).
**Ref:** .andromeda/runs/2026-09-28T09-46-16-wrap/

## 2026-09-28-capability-ledger-and-viola-verify — `viola verify`'s six-row step counter and its exit-1 phraseology
**Section:** §Surface: cli Component Patterns (`verify`); Exit-code phraseology (exit 1 row)
**Change:**
- `verify`'s step counter prints one static stdout line per ledger row, `[01/06] shim-resolution claude resolves to a real executable  pass` … (six rows today; the count grows as owning chunks land rows), then the last stdout line `stamped 2.1.283  6 pass  0 fail` (was the 14-row `[03/14] S3 …` / `stamped 2.1.280  14 pass` sketch).
- Exit 1 also carries verify's refusals `unable: the claude CLI was not found` / `the claude CLI is a .cmd or .bat script` / `the CLI version could not be read` / `a recorded payload still holds a path or a username`, each with its own hint; a failing row prints no stderr word.
**Why:** the chunk landed the ledger's six rows and the verb's refusals (report Symbols; `tests/cli_verify.rs`).
**Ref:** .andromeda/runs/2026-09-28T18-10-28-wrap/
