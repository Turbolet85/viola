# layouts · standing conventions

- S1 · Human output is a contract: every new or changed output line of a verb is pinned by a test of its exact text, plain modes included. · surface-cli
- S2 · --json on every data verb: a new field or error code joins the one typed --json document of the verb and is pinned by a test. · surface-cli
- S3 · Every refusal cause has a hint naming the next verb, last on stderr, quoting no text and naming no path. · surface-cli
  contract/viola-revive-refusals/directory-missing · viola revive refusals · recorded directory missing — `unable: <name>'s recorded directory is missing` / `hint: resume it by hand from a directory you choose: viola run <name> -- claude --resume <id>, ids from viola revive <name> --list`; no paths, no pids, no logged id.
  contract/viola-run/already-live · viola run start refusals · already live — Exit 1 with `unable: <name> is already live` on stderr and `hint: viola list` as the last stderr line; no paths, no pids.
