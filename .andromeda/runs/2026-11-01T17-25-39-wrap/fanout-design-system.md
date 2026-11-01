proposals:
  - detector: D-design-cli-wording
    severity: warning
    section: "Surface: cli › Component Patterns › 2. `viola send`: the readback mirror"
    change: "The sentence on exit 21 under `--json` now reads: exit 21's document names its cause in `detail`, one of six closed words (Exit-code phraseology below); it read five."
    sidecar: "Pattern 2: was exit 21's document naming one of five closed words; now one of six, the sixth `socket-dir-refused` (chunk 2026-11-01-unix-endpoint-and-home-hardening, `inputs#I2` Q1)."
    rationale: "The report's Counts / qualifiers moved has exit 21's closed causes five to six, a row reading five closed changing; Symbols / APIs adds `UnreachableCause::SocketDir`, and Decisions names Q1 as the founder's own word."
    basis: "crates/viola-core/src/lib.rs:201"
  - detector: D-design-cli-wording
    severity: warning
    section: "Surface: cli › Component Patterns › Exit-code phraseology"
    change: "Row 21's `--json` cause list extends to six: `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused`, stderr still empty."
    sidecar: "Exit-code phraseology, row 21: the closed cause list gains `socket-dir-refused`, five words to six, in the report's order."
    rationale: "The same retired claim as pattern 2's, restated as a list: the report's Counts / qualifiers moved says a row holding a list of the five extends, and gives the six words in this order."
    basis: "crates/viola-core/src/lib.rs:201"
    dependent-of: D-design-cli-wording
  - detector: D-design-cli-wording
    severity: warning
    section: "Surface: cli › Component Patterns › 5. `viola verify` / `viola run`"
    change: "The `revive` paragraph's `--json` code list extends to eleven: `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`."
    sidecar: "Pattern 5, `revive`: the closed `--json` codes of the start it shares with `run` gain `socket-dir-refused`, ten to eleven."
    rationale: "The report's Counts / qualifiers moved has `revive --json`'s exit-1 codes ten to eleven, each list holding all ten extending; Symbols / APIs has `revive`'s start sharing `refuse_socket_dir` and the `socket-dir-refused` document; the report's Outcome reads it MET in `cross_exit_causes`."
    basis: "src/cmd/run.rs:822-829"
note: "D-design-tokens: no drift, every Coverage row reads tokens n/a and the chunk renders no new UI token. D-retired: no body occurrence, the dated owner, the two-rules claim and the owed socket-directory code stand only in standing groups of .andromeda/design-system-detail/surface.md. These are the cascade's and carry no proposal here: the new pair `unable: the socket directory for <name> cannot be trusted` / `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions` with its planning-form record, the founder's wording owed at Epoch 7's boundary batch (`inputs#I3`, `inputs#I4` item 5), which is unstated in both body and detail today; exit-code rows 1 and 21; the cli-send five closed words; the D-20 sentence and its carve-out; the macOS pending record, its gap staying named with no route owner; `server-verify-failed`'s euid reach; the `revive --json` codes item; the cli-list board-rule item (`surface.md:46`, now the read rule on the home)."
