proposals:
  - detector: D-design-cli-wording
    severity: warning
    section: "§Surface: cli → Component Patterns → 2. `viola send`: the readback mirror"
    change: "Replace the clause 'a per-cause `detail` code is pending, owner: the arch amendment naming them' with: exit 21's `--json` document names its cause in `detail`, one of five closed words (`strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`), and `error` stays `instance-unreachable`; the sentence still opens 'Under `--json` no hint prints'."
    sidecar: "Pattern 2's pending per-cause `detail` clause now states the five closed causes exit 21's `--json` document carries (chunk 2026-10-31-exit-cause-code-catalogue)."
    rationale: "Report Changes, Symbols / APIs, 'The exit-21 document moved': every cause now prints `{\"v\":1,\"error\":\"instance-unreachable\",\"detail\":\"<cause>\"}` with five closed causes (`inputs#I2` Q1), and Retired names the document's `detail: null`; the pending clause states as owed what the chunk built."
    basis: "crates/viola-core/src/lib.rs:181-190"
  - detector: D-design-cli-wording
    severity: warning
    section: "§Surface: cli → Exit-code phraseology"
    change: "In the typed-code paragraph, exit 21's closing clause 'its `--json` document with stderr empty' becomes: its `--json` document `{\"v\":1,\"error\":\"instance-unreachable\",\"detail\":\"<cause>\"}`, the cause one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`, with stderr empty."
    sidecar: "Exit 21's phraseology now states the one document per cause in place of a single unnamed document."
    rationale: "Report Changes, Symbols / APIs, 'The exit-21 document moved', and Expected amendments, design-system's exit-code row 21; no exit, `RefusalReason` or exit code moved, only the document's `detail`."
    basis: "design-system.md:338"
    dependent-of: D-design-cli-wording
  - detector: D-design-cli-wording
    severity: warning
    section: "§Surface: cli → Exit-code phraseology"
    change: "The `viola plugin install` paragraph's 'under `--json` the `instance-unreachable` document, stderr empty' becomes: under `--json` the `instance-unreachable` document with `detail` `strict-modes-failed`, stderr empty."
    sidecar: "`viola plugin install`'s refused strict-modes document now names its cause, `strict-modes-failed`."
    rationale: "Report Changes, Symbols / APIs, `src/cmd/client.rs`: `refuse_untrusted` writes `StrictModes` through `unreachable_document`, `src/cmd/plugin.rs` too; exit 21 no longer has one document, so the bare name restates the retired single form."
    basis: "tests/cross_exit_causes.rs:274-292"
    dependent-of: D-design-cli-wording
  - detector: D-design-cli-wording
    severity: warning
    section: "§Surface: cli → Exit-code phraseology"
    change: "The `viola link` / `viola unlink` and six frame verbs paragraph's 'or under `--json` the `instance-unreachable` document, stderr empty' becomes: or under `--json` the `instance-unreachable` document with `detail` `strict-modes-failed`, stderr empty; its 'no frame and no line are written' stays."
    sidecar: "The link, unlink and frame verbs' refused strict-modes document now names its cause, `strict-modes-failed`."
    rationale: "Report Changes, Symbols / APIs, `src/cmd/client.rs`: `refuse_untrusted` writes `StrictModes` through `unreachable_document`; Counts, `doc-null`: a strict-modes site reads `strict-modes-failed`."
    basis: "src/cmd/client.rs:222-226"
    dependent-of: D-design-cli-wording
  - detector: D-design-cli-wording
    severity: warning
    section: "§Surface: cli → Tokens (platform-specific) → Streams"
    change: "The `--json` bullet's 'the typed `refusal` / `error` fields are the whole answer' becomes: the typed `refusal` / `error` fields and their `detail` are the whole answer, exit 21's cause being its `detail`."
    sidecar: "The Streams `--json` bullet now counts `detail` in the machine answer, since exit 21's `error` is shared by five causes."
    rationale: "Report Changes, Symbols / APIs, 'The exit-21 document moved': `error` stays `instance-unreachable` for all five causes, so the `error` field alone no longer answers which one; the cause is in `detail`."
    dependent-of: D-design-cli-wording
  - detector: D-design-cli-wording
    severity: warning
    section: "§Surface: cli → Exit-code phraseology"
    change: "In the typed-code paragraph, exit 1's `error: internal error` gains its `--json` form: a verb that prints that line prints instead `{\"v\":1,\"error\":\"internal-error\",\"detail\":null}` on stdout under `--json`, stderr empty; without `--json` the line stands byte for byte, and `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site."
    sidecar: "Exit 1 now states the catch site's `internal-error` document under `--json`."
    rationale: "Report Changes, Symbols / APIs, 'The `cli` catch site under `--json`': `json_asked` prints the document where `prints_internal_error` is true; Counts, `catch-site`: a site stating the output under `--json` too takes the document."
    basis: "src/main.rs:154-161"
  - detector: D-retired
    severity: warning
    section: "§Surface: cli → Component Patterns → 5. `viola verify` / `viola run`"
    change: "The `revive` paragraph drops the sentence 'A refusal `run`'s start makes after a passed preflight prints no `--json` document: pending, owner: the working-route entry \"Exit-cause code catalogue\"', and its 'under `--json` a refused strict-modes check or preflight prints one stdout document and no hint' becomes: under `--json` every refusal of its start prints one stdout document and no hint, stderr empty, codes `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`; a `pump_child` error prints no document, the child having held the terminal."
    sidecar: "`revive`'s pending post-preflight `--json` gap retired; the paragraph now states the one document per start refusal and its ten codes."
    rationale: "Report Retired names the `revive --json` post-preflight gap; Changes, Symbols / APIs, '`viola revive --json`', lists the codes and says the sentence 'a refusal `run`'s start makes after a passed preflight prints no document' is no longer true."
    basis: "design-system.md:334"
  - detector: D-retired
    severity: warning
    section: "§Surface: cli → Exit-code phraseology"
    change: "In the typed-code paragraph, exit 1's parenthetical 'a `--json` document only for `viola revive`'s strict-modes and preflight refusals' becomes: a `--json` document for every refusal of `viola revive`'s start."
    sidecar: "Exit 1's `--json` scope for `viola revive` widened from its strict-modes and preflight refusals to every refusal of its start."
    rationale: "Report Retired names the `revive --json` post-preflight gap; Changes, Symbols / APIs, '`viola revive --json`': one exit-1 document for every refusal of the start it shares with `run`."
    basis: "design-system.md:338"
    dependent-of: D-retired
note: "D-design-tokens has no drift, every Coverage row reads tokens n/a. No D-new-detail, every moved fact has a category file: the standing groups the report names (surface.md:64, :65, :82, :83, :84, :135, :144, :152, :168) and the new `endpoint-lost` hint are the cascade's, not proposed here."
