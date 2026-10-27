proposals:
  - detector: D-design-cli-wording
    severity: warning
    section: Surface: cli › Exit-code phraseology
    change: The `viola link` / `viola unlink` paragraph widens to the eight frame verbs: a refused strict-modes check of the named instance (exit 21) made by `viola send`, `wait`, `last`, `answer`, `pause`, `release` (before any frame) or by `viola link` / `viola unlink` of the driven instance prints the instance-state `unable:` / `hint:` pair naming that instance, as `viola list`'s does, or under `--json` the `instance-unreachable` document, stderr empty; no frame is written and no line.
    sidecar: Surface: cli — the six frame verbs (send, wait, last, answer, pause, release) join link / unlink in printing the instance-state pair at their exit-21 strict-modes refusal (chunk 2026-10-26-home-and-code-bearing-file-integrity; the founder's own answer, inputs#I3 Q3).
    rationale: Report Changes, Symbols `src/cmd/client.rs` — `refuse_untrusted` prints `human::instance_state_pair(name)` (or `refused_document("instance-unreachable")` under `--json`), `Some(21)`, called first by send, wait, last, answer, pause, release; Count 8 (pattern `instance-pair`): «the six frame verbs join the list»; Decisions: «the six frame verbs print `link`'s instance pair» (inputs#I3 Q3). The body names only link / unlink for this pair.
    basis: src/cmd/client.rs:20-41
  - detector: D-design-cli-wording
    severity: warning
    section: Surface: cli › Streams
    change: The stderr bullet's fixed `unable: <text>` form covers the exit-1 refusals of `run`, `revive` and `verify` and the exit-21 strict-modes refusals of `list`, `plugin install`, `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`.
    sidecar: Surface: cli Streams — the fixed `unable:` form's exit-21 strict-modes list gains the six frame verbs (chunk 2026-10-26-home-and-code-bearing-file-integrity, inputs#I3 Q3).
    rationale: Report Changes, Symbols `src/cmd/client.rs` (`refuse_untrusted` through `human::refuse` of `human::instance_state_pair(name)` at the six verbs) and Count 8; the body's Streams bullet names only `list`, `link` and `unlink` for the exit-21 fixed form.
    dependent-of: D-design-cli-wording
  - detector: D-design-cli-wording
    severity: warning
    section: Surface: cli › Exit-code phraseology
    change: Exit 21 reads `unable  instance-unreachable` then the cause's hint, except a refused strict-modes check, which prints its fixed home-state or instance-state `unable:` / `hint:` pair instead (the `list`, `plugin install` and frame-verb paragraphs below); its `--json` document with stderr empty.
    sidecar: Surface: cli exit-code phraseology — exit 21's strict-modes refusal is the fixed pair, not the `unable  instance-unreachable` line with the strict-modes hint (chunk 2026-10-26-home-and-code-bearing-file-integrity).
    rationale: Report Changes, Symbols `src/cmd/client.rs` and Count 8: every verb that now runs a strict-modes check at exit 21 prints the fixed pair; the body's generic exit-21 clause still reads as the `unable  instance-unreachable` line plus a cause hint for every cause.
    dependent-of: D-design-cli-wording
  - detector: D-design-cli-wording
    severity: warning
    section: Surface: cli › Component Patterns
    change: Pattern 5's `verify` paragraph adds: a home tree that fails the strict-modes check (the home, `ledger/`, `bin/`, `plugin/`, each that exists), checked before any log, prints no step line and no summary, only the home-state pair `unable: the viola home is not private to you` / `hint: the viola home must be readable only by you; viola does not change its permissions`, and exits 1; a stamps write refused by its check prints the same pair.
    sidecar: Surface: cli pattern 5 — `viola verify` prints the home-state pair (exit 1, no step line, no summary) on a refused home tree or stamps check (chunk 2026-10-26-home-and-code-bearing-file-integrity).
    rationale: Report Changes, Symbols `src/cmd/verify.rs`: `check_home_tree(&home, None, None)` before its `cli` log, refused → `refuse_home()` (`human::home_state_pair()`), exit 1, no step line, no summary, no line; `measure`'s `StampsError::StrictModes` maps to the same pair; Count 9 (pattern `home-pair`): «`run` and `verify` print the home pair (exit 1) on a refused home tree». The body's verify paragraph names only the stopped-run pair.
    basis: src/cmd/verify.rs:64-70
note: Detail sites owed by the cascade (category file .andromeda/design-system-detail/surface.md, report Counts 8 and 9): surface.md:21 (cli-output fixed-form list), :37 (the instance pair's printers), :54-58 (exit-21 strict-modes hint cause, now superseded by the pair at every checking verb), :59-65 (run's start refusals gain the instance pair for an instance-level tree refusal), :67 (revive's pair), :122 (exit-1 row: verify's home pair, run's instance pair), :130 (exit-21 row: the six frame verbs). D-design-tokens: every new surface's Coverage reads `tokens n/a`, no drift. D-retired: no body passage states the eight dated gaps or `read_snapshot`.
