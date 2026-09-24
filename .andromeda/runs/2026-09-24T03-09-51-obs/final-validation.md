## Issues Found

- **[CONTRADICTION]** Section 11's PII subsection bans `statusline_command`, send text, tool `input` and `last_assistant_message` "at any level, in any file". Section 8 allows exactly this content in the instance-scoped `detail-<process>.ndjson` files:
  - its `statusline_command` row says "detail file only if part of an error chain";
  - its drift-report row sends reports to the detail file only;
  - its verification step says the canary "may appear only in `detail-*.ndjson`";
  - the detail-file upload is justified only because test inputs are synthetic.

  The Section 8 prompt-text and tool-input rows also say "never in any process-log line", which conflicts with that same canary rule. Upstream security agrees with the detail-file allowance: `statusline_command` goes to "`diagnostics/` only". The Critical-class values (`CLAUDE*`, the GUI token) are a separate case: those really are never logged, detail files included.
  Section: §8 Data classification table, §11 PII Scrubbing
  Severity: medium

- **[STALE_ARTIFACT]** A Section 11 CI line still shows bare `if: always()` / `failure()` conditions. D-27 and Section 9 replaced those with conditions gated on `steps.secret-scan.outcome`, and Section 9 warns that a bare `always()` would upload after a failed scan. As written, the ban tells readers to use the pattern the plan rejects.
  Section: §11 CI vs §9 (artifact table and step order), D-27
  Severity: medium

- **[STALE_ARTIFACT]** The Section 2 panic invariant lists only three bounded exemptions and says they "are listed in Section 10". D-32 added a fourth one to Section 10: a failed role-file open at init step 4. Section 2 was not updated.
  Section: §2 Agent-readable invariants vs §10 Bounded exemptions, D-32
  Severity: low

- **[CONTRADICTION]** Section 3's list of third-party targets set to OFF has 6 entries and omits `sysinfo` and `interprocess`. The Section 6 per-module table and D-11 both list 8. Behaviour is unaffected because Section 6 also says "everything else OFF", but the Section 3 list is written as the full set.
  Section: §3 Logging stack (Agent-mode flag) vs §6 Per-module log levels, D-11
  Severity: low

- **[INCOMPLETE]** The Section 4 "Span kinds" list promises a kind for every span, but it leaves out two spans the scenarios use:
  - `hook.statusline`, which Scenario 6 labels SERVER;
  - `last.client`, used in Scenario 3, which is a client span like `wait.block`.

  `channel.bind`, `cli.<verb>` and `cli.verify_step` also have no kind anywhere in the plan. I did not patch those three, because no source says what their kind should be.
  Section: §4 Span kinds vs §4 Scenarios 3 and 6
  Severity: low

- **[STALE_ARTIFACT]** Section 8's default-deny paragraph calls an unknown field "a review failure". The mechanism is machine-enforced: Section 10 counts it as a build failure, and gate G4 in Section 9 enforces it. "Review failure" reads like a human gate.
  Section: §8 Default-deny posture vs §9 G4, §10 Build / deploy failure conditions
  Severity: low

- **[DANGLING_REF]** D-22's Impact line points to "§4 Heartbeat (`liveness-changed.pid_alive`)". Heartbeat ticks is a subsection of Section 3, and Section 4 has no Heartbeat subsection. I made no patch: the Decisions Log says "do not modify historical entries", so this belongs in a new appended entry if it is fixed at all.
  Section: §12 D-22
  Severity: low

## Patches

### Patch 1: Split the Section 11 PII never-log line into Critical (never, any file) and High (never in home-level files)
**Reason:** [CONTRADICTION] Section 11 bans in every file what Section 8 and upstream security allow in instance detail files.
**Old:**
```
- NEVER log `CLAUDE*` values, the GUI token, `Cookie`, `?t=`, `ui/<port>.url` contents, `statusline_command`, send text, tool `input` or `last_assistant_message`, at any level, in any file.
```
**New:**
```
- NEVER log `CLAUDE*` values, the GUI token, `Cookie`, `?t=` or `ui/<port>.url` contents, at any level, in any file, detail files included. NEVER log `statusline_command`, send text, tool `input` or `last_assistant_message` in any home-level file. They may reach only the instance-scoped `detail-<process>.ndjson`, and only inside a chain or drift report (§8).
```

### Patch 2: Align the Section 8 prompt-text row with the canary rule
**Reason:** [CONTRADICTION] "never in any process-log line" conflicts with Section 8 Verification, where the canary is allowed in `detail-*.ndjson`.
**Old:**
```
| Prompt text, assistant output (`prompt-submitted.data.text`, `last_assistant_message`) | High | never in any process-log line; payloads stay in contract payloads |
```
**New:**
```
| Prompt text, assistant output (`prompt-submitted.data.text`, `last_assistant_message`) | High | never in any home-level process-log line; payloads stay in contract payloads and reach an instance `detail-<process>.ndjson` only inside a chain or drift report |
```

### Patch 3: Align the Section 8 tool-input row with the canary rule
**Reason:** [CONTRADICTION] Same conflict as Patch 2.
**Old:**
```
| Tool `input`, plan text, dialog questions/answers | High | never in any process-log line |
```
**New:**
```
| Tool `input`, plan text, dialog questions/answers | High | never in any home-level process-log line; instance `detail-<process>.ndjson` only inside a chain or drift report |
```

### Patch 4: Replace the bare CI conditions in the Section 11 CI ban
**Reason:** [STALE_ARTIFACT] The ban still shows the bare `always()` / `failure()` conditions that D-27 and Section 9 replaced.
**Old:**
```
- NEVER lose the failing job's `diagnostics/` (`if: always()` / `failure()`). A re-run is not a substitute under `retries = 0`.
```
**New:**
```
- NEVER lose the failing job's `diagnostics/`. G2, G4 and the secret scan run `if: always()`, and the uploads run `if: always()` / `failure()` combined with `steps.secret-scan.outcome == 'success'`, never bare. On a failed scan the hit report is uploaded instead (§9 Step order). A re-run is not a substitute under `retries = 0`.
```

### Patch 5: Add the fourth exemption to the Section 2 panic invariant
**Reason:** [STALE_ARTIFACT] Section 2 lists three exemptions; D-32 added a fourth to Section 10.
**Old:**
```
The bounded exemptions (the pre-init window, `cli` without an instance, `hook` without `VIOLA_NAME`) are listed in Section 10.
```
**New:**
```
The bounded exemptions (the pre-init window, `cli` without an instance, `hook` without `VIOLA_NAME`, a failed step-4 role-file open) are listed in Section 10.
```

### Patch 6: Complete the Section 3 list of third-party targets set to OFF
**Reason:** [CONTRADICTION] Section 3 lists 6 targets; Section 6 and D-11 list 8.
**Old:**
```
  - Third-party targets (`rmcp`, `axum`, `tower_http`, `hyper`, `portable_pty`, `notify`) are `OFF` in every sink,
```
**New:**
```
  - Third-party targets (`rmcp`, `axum`, `tower_http`, `hyper`, `portable_pty`, `notify`, `sysinfo`, `interprocess`) are `OFF` in every sink,
```

### Patch 7: Add the missing spans to the Section 4 span-kind list
**Reason:** [INCOMPLETE] `last.client` (Scenario 3) and `hook.statusline` (Scenario 6, labelled SERVER) are missing from the list.
**Old:**
```
- `CLIENT`: `send.client`, `wait.block`, `answer.client`, `channel.request` (client side), `statusline.shell_out`, `pty.spawn`.
- `SERVER`: `channel.dispatch` (wrapper side of every `channel.request`), `ui.http_request`, `ui.sse_stream`, `mcp.tool_call`, `hook.handle`.
```
**New:**
```
- `CLIENT`: `send.client`, `wait.block`, `last.client`, `answer.client`, `channel.request` (client side), `statusline.shell_out`, `pty.spawn`.
- `SERVER`: `channel.dispatch` (wrapper side of every `channel.request`), `ui.http_request`, `ui.sse_stream`, `mcp.tool_call`, `hook.handle`, `hook.statusline`.
```

### Patch 8: Say that unknown fields fail the build through G4, not a review
**Reason:** [STALE_ARTIFACT] "Review failure" wording for a check that Section 10 and gate G4 in Section 9 enforce automatically.
**Old:**
```
Unknown fields are a review failure, caught by the `schemas/diag-line.v1.json` conformance check (`additionalProperties: false` per event).
```
**New:**
```
Unknown fields are a build failure (§10), caught by gate G4 (§9) against `schemas/diag-line.v1.json` (`additionalProperties: false` per event).
```
