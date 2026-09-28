
## 2026-09-28-cli-output-tokens — clap without `color`, the env sentence true again, `src/human.rs` the one refusal writer
**Section:** §Stack and Technologies CLI parser row; §Infrastructure Patterns Build system (lint bullet) and Project directory structure; §Cross-cutting Patterns Config management (environment variables)
**Change:**
- clap 4.6.7 is `default-features = false` with `std`, `derive`, `help`, `usage`, `error-context`, `suggestions` — its default set minus `color` (was "clap 4.6.7 (derive)"); 8 packages left `Cargo.lock`, none added.
- The exhaustive env sentence now says why it holds: clap's `color` route (anstream → anstyle-query, reading `CLICOLOR`, `CLICOLOR_FORCE`, `NO_COLOR`, `TERM`, `COLORTERM`, `CI`) was closed by this chunk, and a dependency's env read breaks the rule like a direct one.
- The root bin's only human-stderr writer is `src/human.rs` (`write_refusal`, one `write_all`; `refuse`, locked stderr, result dropped), called only by `run`'s five start refusals, squatted endpoint included (was "one `refuse` helper with `writeln!`", four kinds); `src/human.rs` joins the directory tree.
**Why:** the sentence was false at c04e332 through clap's `color` (as measured at research F1: `CLICOLOR_FORCE=1 viola --help` into a pipe, 25 ESC bytes); the chunk shipped the feature change and the writer (report Changes; ci#36404931982 green).
**Ref:** .andromeda/runs/2026-09-28T09-46-16-wrap/
