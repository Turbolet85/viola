# design · standing conventions

- S1 · each new log line or event datum names a cause or detail by a fixed code word from a closed list, never by free text read from a peer or the agent · surface
  contract/log-detail-codes/server-verify-failed · log detail codes · server-verify-failed — Written to the log by `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` at their exit-21 server-verification refusal, whose `--json` document reads the same word.
  contract/log-detail-codes/socket-dir-refused · log detail codes · socket-dir-refused — Written to the log by `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` at their exit-21 socket-directory refusal, and at `run`'s and `revive`'s start.
  contract/peer-reply-screening/code-word · peer reply screening · code word — A code word is 1 to 64 bytes of lower-case letters, digits and hyphens, a lower-case letter first; both clients, the CLI and the MCP server, screen a peer's `refusal` and `detail` through it.
  contract/peer-reply-screening/refusal-not-code-word · peer reply screening · refusal that is no code word — A peer's `refusal` that is no code word reads `unknown`.
  contract/peer-reply-screening/string-detail-not-code-word · peer reply screening · string detail that is no code word — A peer's string `detail` that is no code word reads as no detail, null under `--json`.
