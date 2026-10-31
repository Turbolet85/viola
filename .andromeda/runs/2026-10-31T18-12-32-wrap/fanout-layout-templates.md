proposals:
  - detector: D-design-cli-wording
    severity: warning
    section: §Output structure — `viola list --json`
    change: 'The failed strict-modes check of the home, `instances/` or any one instance prints `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` with exit 21 and stderr empty, in place of the `"detail":null` form; the rest of the sentence stands.'
    sidecar: 'Exit-cause code catalogue: `viola list --json`''s exit-21 document carries `detail` `strict-modes-failed`, the board''s only refusal.'
    rationale: 'Report Symbols / APIs: the exit-21 document moved from `detail` null to `detail` the cause, and `src/cmd/list.rs` builds `strict-modes-failed`; Retired names the `detail: null`; Expected amendments list this line under the `doc-null` search.'
    basis: .andromeda/layout-templates.md:102
  - detector: D-design-cli-wording
    severity: warning
    section: §Output structure — `viola plugin install`
    change: 'A refused strict-modes check of the home or a `marketplace/` tree dir under `--json` prints `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` with stderr empty, in place of the `"detail":null` form.'
    sidecar: 'Exit-cause code catalogue: `viola plugin install --json`''s exit-21 document carries `detail` `strict-modes-failed`.'
    rationale: 'Report Symbols / APIs: `unreachable_document` is the one writer of the exit-21 document and `src/cmd/plugin.rs` writes `StrictModes` through it; Retired names the `detail: null`.'
    basis: .andromeda/layout-templates.md:288
    dependent-of: D-design-cli-wording
  - detector: D-design-cli-wording
    severity: warning
    section: §Component — Primary content block 2 → Line form
    change: 'The exit-21 strict-modes refusal of the frame verbs under `--json` prints `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` with stderr empty, in place of the `"detail":null` form; the exit-1 `{"v":1,"error":"<code>","detail":null}` document of `viola revive` in the same bullet stands.'
    sidecar: 'Exit-cause code catalogue: the frame verbs'' strict-modes exit-21 document carries `detail` `strict-modes-failed`.'
    rationale: 'Report Symbols / APIs: `refuse_untrusted` writes `StrictModes` through `unreachable_document`, and `refused_document` stays exit 1''s; Counts, lines over 2 000 chars: `layout-templates.md:329` doc-null changed.'
    basis: .andromeda/layout-templates.md:329
    dependent-of: D-design-cli-wording
  - detector: D-design-cli-wording
    severity: warning
    section: §Component — Primary content block 2 → Hint line
    change: 'A failed or unmakeable server check under `--json` prints the exit-21 document `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}` with stderr empty, in place of the `"detail":null` form.'
    sidecar: 'Exit-cause code catalogue: the server check''s exit-21 document carries `detail` `server-verify-failed`.'
    rationale: 'Report Symbols / APIs: the five closed causes hold `server-verify-failed`, and `Unreachable::ServerUnverified` is its form; Retired names the `detail: null`; Expected amendments name this line.'
    basis: .andromeda/layout-templates.md:331
    dependent-of: D-design-cli-wording
  - detector: D-design-cli-wording
    severity: warning
    section: §Component — Primary content block 2 → Exit codes as the typed tail
    change: 'Replace "The machine view carries the cause as a detail code once arch names those codes; until then it keeps arch''s shape (`"detail":null` for exit 21)" with: the machine view carries the cause as the exit-21 document''s `detail`, one of `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`, with `error` staying `instance-unreachable` and `v` staying 1.'
    sidecar: 'Exit-cause code catalogue: exit 21''s machine view carries its cause in `detail`; the pending clause and its `"detail":null` retire.'
    rationale: 'Report Symbols / APIs: the exit-21 document moved, each cause its word; Retired names the `detail: null`. The sentence restates the retired shape as pending; it is outside the lines the report lists for this document, a further occurrence of the same claim.'
    basis: crates/viola-core/src/lib.rs:181-190
    dependent-of: D-design-cli-wording
  - detector: D-design-cli-wording
    severity: warning
    section: §Component — Primary content block 2 → Hint line
    change: 'Beside the not-running hint, state the two other hints of the frame verbs'' exit 21, as `src/human/hints.rs:50-55` words them: `unwrapped-name` (a name whose instance directory does not stand) and `endpoint-lost` (the wrapper went away during the call), each last on stderr in plain ASCII; `instance-dead` keeps `hint: <name> is not running; viola list shows the live instances`.'
    sidecar: 'Exit-cause code catalogue: the unwrapped-name and endpoint-lost hints stated beside the not-running hint.'
    rationale: 'Report Counts `hints`: the unwrapped hint built and the endpoint-lost hint new, with no standing wording changed, so a new wording stays unstated here; `hint_cause()` reads `not-running`, `unwrapped-name`, `endpoint-lost`, `server-verify`; Decisions carries the delegate''s word "went away during the call", PROVISIONAL.'
    basis: src/human/hints.rs:50-55
  - detector: D-design-cli-wording
    severity: warning
    section: §Component — Primary content block 2 → Line form
    change: 'After "prints exactly `error: internal error` once, from the one catch site, with no hint", add: with `--json` among the arguments before a `--`, the catch site prints `{"v":1,"error":"internal-error","detail":null}` on stdout in its place, stderr empty; without `--json` the line stands byte for byte, and a `revive` start stays outside it.'
    sidecar: 'Exit-cause code catalogue: the `cli` catch site prints the `internal-error` document under `--json`.'
    rationale: 'Report Symbols / APIs, the `cli` catch site under `--json`: `json_asked` and `internal_error_document`; Counts `catch-site`: a site stating the output under `--json` too takes the document, naming `layout-templates.md:288` and `:329`.'
    basis: src/main.rs:154-161
  - detector: D-design-cli-wording
    severity: warning
    section: §Output structure — `viola wait` / `viola last`
    change: 'After "A failure or a caught panic prints `error: internal error` once, exit 1, no hint (Primary content block 2)", add that under `--json` it prints `{"v":1,"error":"internal-error","detail":null}` on stdout in its place, stderr empty.'
    sidecar: 'Exit-cause code catalogue: `wait` and `last` under `--json` print the `internal-error` document at the catch site.'
    rationale: 'Report Symbols / APIs, the `cli` catch site under `--json`; the same paragraph says each verb prints exactly one document and nothing on stderr under `--json`, and Expected amendments list this line for this document.'
    basis: .andromeda/layout-templates.md:146
    dependent-of: D-design-cli-wording
  - detector: D-design-cli-wording
    severity: warning
    section: §Output structure — `viola plugin install`
    change: 'After "A failed exe read, dir creation or file write is the `cli` catch site''s `error: internal error`, exit 1", add that under `--json` it is `{"v":1,"error":"internal-error","detail":null}` on stdout with stderr empty.'
    sidecar: 'Exit-cause code catalogue: `viola plugin install --json` prints the `internal-error` document at the catch site.'
    rationale: 'Report Counts `catch-site` names `layout-templates.md:288` as a site that takes the document; `viola plugin install` takes `--json`, and the sentence states its failure in the human form only.'
    basis: .andromeda/layout-templates.md:288
    dependent-of: D-design-cli-wording
  - detector: D-retired
    severity: warning
    section: §Output structure — `viola revive`
    change: 'Replace the sentence "A refusal `run`''s start makes after a passed preflight ... prints no document as landed; its document is owed to the working-route entry" with: a refusal of the start shared with `run` prints one exit-1 document too, `<code>` also `batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, or `internal-error` for a program not found and for an error of the start itself (the catch site still logging its chain), each with stderr empty; a `pump_child` error prints no document, the child having held the terminal.'
    sidecar: 'Exit-cause code catalogue: the `revive --json` post-preflight gap closed; every start refusal prints its document and the owed clause retires.'
    rationale: 'Report Retired names the `revive --json` post-preflight gap; Symbols / APIs, `viola revive --json`, says this sentence is no longer true and gives the codes; Counts `revive-gap`, `start-codes-rev` and `title`.'
    basis: src/cmd/revive.rs:119-134
  - detector: D-retired
    severity: warning
    section: §Output structure — `viola revive`
    change: 'In the code list of the `--json` paragraph add `stale-heartbeat`: the stale sibling of the collision pair prints `{"v":1,"error":"stale-heartbeat","detail":null}`, and `already-live` is the code of a live holder and a fresh replayed beat; the human pair stays byte-identical.'
    sidecar: 'Exit-cause code catalogue: a stale holder under `viola revive --json` reads `stale-heartbeat`, no longer `already-live`.'
    rationale: 'Report Retired names `already-live` as a stale holder''s refusal; Symbols / APIs, a stale holder''s code and the `viola revive --json` code list.'
    basis: .andromeda/layout-templates.md:262
note: 'D-layout-surface found no drift: the report adds no surface or region, only documents and hints on standing output forms. `.andromeda/layout-templates.md:143` takes no proposal, its not-running hint standing per Counts `hints`. The two hint literals are named by `src/human/hints.rs:50-55`, not quoted, since the report gives only the phrase "went away during the call": the orchestrator reads them there. The same `viola revive` paragraph, the Line form bullet, the Hint line bullet and the `viola plugin install` paragraph each take two proposals, on separate sentences.'
