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
