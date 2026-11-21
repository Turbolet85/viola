# layouts · standing conventions

- S1 · each new viola verify step line, failing row or summary figure follows the viola verify output contract: plain ASCII, results on stdout and the rest on stderr, the exit code by the verify contract. · surface-cli
  contract/viola-revive/list-row · viola revive · --list row — `viola revive <name> --list` prints one stdout row per logged session in log order, `<ts>  <cause>  <id>` with two spaces between fields, no header, no colour, plain ASCII, with an empty stderr.
  decision/stdout-results-stderr-rest · Results on stdout, the rest on stderr — Results go to stdout; `waiting:`, the issue line, refusals, hints and errors go to stderr, and the two are never mixed.
- S2 · each new refusal or failure line the probe can print ends with a hint naming the next verb, never quoting text or naming paths. · surface-cli
  contract/viola-revive-refusals/directory-missing · viola revive refusals · recorded directory missing — `unable: <name>'s recorded directory is missing` / `hint: resume it by hand from a directory you choose: viola run <name> -- claude --resume <id>, ids from viola revive <name> --list`; no paths, no pids, no logged id.
  contract/viola-revive/refusal-shape · viola revive refusals · refusal shape — Each `viola revive` refusal is exit 1 with one `unable:` line and one `hint:` line last on stderr and an empty stdout.
  contract/viola-run/already-live · viola run start refusals · already live — Exit 1 with `unable: <name> is already live` on stderr and `hint: viola list` as the last stderr line; no paths, no pids.
  decision/hint-last-on-stderr · Hint is the last stderr line — On a refusal, the `hint:` line is the last line on stderr.
  decision/hints-name-next-verb · Hints name the next verb — `hint:` lines point to the next verb by name, as the CLI's breadcrumbs.
