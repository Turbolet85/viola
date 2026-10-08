# Fan-out results — 2026-10-08-first-live-test-and-self-drive

Seven doc-agents, one batch. Each doc: its verdict and dispositions (Validate), then the list as the agent returned it, taken from its transcript by script.

## architecture

Verdict: 11 proposals (nothing stripped; entities 0).
Dispositions, in the list's order:
1. [Human Takeover / Wheel], the seven shapes — APPLY. Check 1: the "Boundary widening" class by its subject; ratified by the founder himself on 2026-10-08 after the exact grammars and the narrower option were shown, relayed verbatim by the overseer (inputs#I17), an answer given after the widening was shown, so it resolves without a new halt and is recorded in the sidecar. Check 5: the plan's expected amendment.
2. [Delivery Confirmation], the three live readings — APPLY. Check 1: "Accurate this-chunk addition". Check 6: disposes the trailing-CR and only-newlines claims. The values were re-read from `evidence/live-readings.ndjson` by the orchestrator, which wrote those rows.
3. [Delivery Confirmation], the `/clear` live proof — APPLY (the same rule; check 5).
4. [Delivery Confirmation], `/clear` and a newline — APPLY (dependent of 3).
5. [CLI Version Compatibility], a `send` under the real hint — APPLY (the same rule; check 5).
6. [CLI Version Compatibility], the hint wording — APPLY (the same rule; check 5).
7. [CLI Version Compatibility], the sentence judging the old advice — APPLY (dependent of 6).
8. §Design Philosophy, "the live-supported target" — APPLY. Check 1: no rule; the operator's recorded direction settles it (the founder's ruling R-L1, 2026-10-07, and the P5-approved expected amendment that names the change). The Windows-first design rule is kept.
9. §Project Intent, Scale path — APPLY (dependent of 8).
10. §Cross-cutting Patterns, Config management, proposed `escalate` — APPLY IN PART, no halt. Check 1: "Sequencing deferral": the `VIOLA_DIR` step for CLI verbs is owed, and its owner is a route annotation that stands, the CARRY on "CLI machine contract" (`working-route.md:118`: `--home` as given, else the default, "without the `VIOLA_DIR`-grandparent step"). That answers the detector's question (owed, not dropped), so the design sentence stays and the body gains its as-landed state and the owner. The proposal's removal of the step is not applied.
11. §Occupied Resources, `VIOLA_DIR` readers — APPLY IN PART (dependent of 10; the same as-landed clause, the design kept).

### The return

````yaml
proposals:
  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions → [Human Takeover / Wheel]"
    change: >-
      In the closed non-editing list sentence, extend the terminal replies after the existing seven (DA1, DA2, CPR, DECRPM, kitty flags, OSC, DCS) with the seven shapes added at chunk 2026-10-08-first-live-test-and-self-drive, each after `CSI`, by exact grammar (fields compared as bytes, no intermediate byte, exact field count): DSR status `CSI 0 n`; colour scheme `CSI ? 997;1 n` and `CSI ? 997;2 n`; window size `CSI 4;n;n t`; cell size `CSI 6;n;n t`; text area `CSI 8;n;n t`; in-band resize `CSI 48;n;n;n;n t`; modifyOtherKeys reply `CSI > 4;n m`. State that the widening was ratified by the founder himself on 2026-10-08 after the exact grammars and the narrower option were shown (inputs#I17), as measured on foot 1.28.0 (before: seven replies of the terminal were outside the list and took the wheel from the driver 237 ms after a live CLI started; after: 0 of 23 probe queries read as typing, and the nearest human keys `n`, `t`, `m` still take the wheel), and that a reply this terminal does not write is not added on a guess. Every other sentence of the entry (F-W2's original ruling, "every other byte takes the wheel", the 64-byte bound, the carry states) stands unchanged.
    sidecar: >-
      2026-10-08 · architecture §Established Decisions [Human Takeover / Wheel] · the closed non-editing list (F-W2) gains seven terminal reply shapes (DSR status, two colour-scheme, window/cell/text-area size, in-band resize, modifyOtherKeys), founder-ratified inputs#I17, measured on foot 1.28.0 · chunk 2026-10-08-first-live-test-and-self-drive.
    rationale: >-
      Report Changes → Symbols / APIs (`is_reply`): four new match arms, the list "gains seven terminal reply shapes after `CSI`", ratified by the founder (inputs#I17); Spec claims disproved → "The closed list at HEAD, on a real terminal"; Expected amendments names this exact site. The entry enumerates the list as closed, so the enumeration is now short by seven shapes. Sweep: the list is enumerated only in this entry (report grep: closed/non-editing list 1 line, F-W2 1 line, terminal replies 1 line in architecture; my read of the rest of the body and of the five key files found no second enumeration; `project-directory-structure` names `wheel.rs` only as "the stdin observer + classifier", which stands).
    basis: "src/run/wheel.rs:506-511 (the new arms; 499 the `first` binding; 474-477 the doc comment; 528 `REPLIES` 15 → 23), evidence/terminal-replies.md, evidence/reply-probe-fixed.ndjson"
  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions → [Delivery Confirmation]"
    change: >-
      Replace the sentence "This is proved under the fake agent, at the unit and cross-process layers on the three CI OSes; unmeasured on a live CLI, and carried on the working route, are a send ending in newlines, a text of only newlines and a trailing CR." with the fake-agent proof kept and the three live readings as measured on 2.1.287 on the Linux dev host at chunk 2026-10-08-first-live-test-and-self-drive (`evidence/live-readings.ndjson`): (1) a send ending in newlines (start 7, `ends-in-newlines`) is confirmed as designed — typed without its trailing LF, its prompt relabelled `driver`, `ok`, no `wheel` record; (2) a text of only newlines (start 8, `only-newlines`) is issued with `text_bytes` 0, no `prompt-submitted` follows, and the `send` ends `not-delivered` / `no-prompt-submitted`, exit 13, after 10.0 s, with no `wheel` record and the snapshot's wheel still `driver`; (3) a driver text ending in one CR (start 8, `trailing-cr`) is typed with the CR (`typed_text` strips LF only, `text_bytes` 32), the CLI submits the 31-byte text without the CR, the exact match fails, the prompt is filed `origin: human`, a `wheel` record `{human, human-input}` follows 33 ms after `send-issued`, the turn runs and ends, and the `send` ends `not-delivered` / `no-prompt-submitted`, exit 13, 10.0 s after it began — so a delivered and answered driver text ending in CR is reported not delivered and takes the wheel from the driver. No fix landed in that chunk; the CR case is an open product finding.
    sidecar: >-
      2026-10-08 · architecture §Established Decisions [Delivery Confirmation] · the three carried live readings are measured on 2.1.287: ends-in-newlines confirmed; only-newlines ends not-delivered/no-prompt-submitted with no wheel move; a trailing CR is dropped by the CLI, so the send ends not-delivered and the wheel moves to human (open, no fix) · chunk 2026-10-08-first-live-test-and-self-drive.
    rationale: >-
      Report Changes → Spec claims disproved → "A driver text ending in one CR" and "A text of only newlines" (both name this entry as the carrier of the unmeasured claim); Cross-project claims ("it submits a pasted text ending in one CR WITHOUT the CR"); Expected amendments ("[Delivery Confirmation] … the sentences carrying the unmeasured live readings … as measured"); Outcome ("`ends-in-newlines` … measured in start 7; `only-newlines` and `trailing-cr` measured in start 8"). The report states the CR and only-newlines values itself; the ends-in-newlines values (exit 0, confirmed, 2 trailing LF, prompt origin driver, 0 wheel records) are read from the evidence file the report cites, for the orchestrator to re-read. Sweep: the "unmeasured on a live CLI … carried on the working route" wording occurs once in architecture. The mechanism sentences in [Human Takeover / Wheel] ("a prompt that is not the in-flight send's typed text claims nothing and is appended as the hook filed it, so one filed `human` still moves the wheel") and in §Standard Contracts (typed text = sent text without trailing LF) describe exactly what was measured and are not retired; no proposal for them.
    basis: "viola-0.1.0/chunks/2026-10-08-first-live-test-and-self-drive/evidence/live-readings.ndjson (rows `ends-in-newlines`, `only-newlines`, `trailing-cr`)"
  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions → [Delivery Confirmation]"
    change: >-
      Replace "The `/clear` confirmation is proven on the recorded 2.1.287 `clear-1` variants replayed by the fake agent; its proof against a live CLI is owed to \"First live test and self-drive\"." with: proven on the recorded 2.1.287 `clear-1` variants replayed by the fake agent, and against the live CLI 2.1.287 on the Linux dev host at chunk 2026-10-08-first-live-test-and-self-drive (`evidence/live-run.ndjson`, `evidence/live-readings.ndjson`): a driver `/clear` was confirmed by a `session-start` of cause `clear` with a new `agent_session_id`, no `prompt-submitted` and no `wheel` record, and the next skill was sent into the cleared session. Nothing is owed to that entry any more.
    sidecar: >-
      2026-10-08 · architecture §Established Decisions [Delivery Confirmation] · the `/clear` confirmation's live proof is no longer owed: measured on live 2.1.287 (new session, cause `clear`) · chunk 2026-10-08-first-live-test-and-self-drive.
    rationale: >-
      Report Outcome, carried into Changes by Expected amendments ("carried: … Coverage and Outcome (the readings measured)"): "(arch) the live run reader green: met — … `/clear` confirmed by a new session, the skill sent again into the cleared session". The entry still says the live proof is owed to this very chunk. Sweep: "First live test and self-drive" is named once in architecture; the unit-layer-only clause of the same entry is the second occurrence (next proposal).
    basis: "viola-0.1.0/chunks/2026-10-08-first-live-test-and-self-drive/evidence/live-run.ndjson; evidence/live-readings.ndjson (row `clear-then-newline`)"
  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions → [Delivery Confirmation]"
    change: >-
      In "`/clear` and a newline is `/clear` (the founder was shown this consequence with the ruling above; proved at the unit layer only)", replace "proved at the unit layer only" with: proved at the unit layer and as measured live on 2.1.287 at chunk 2026-10-08-first-live-test-and-self-drive (start 7, `clear-then-newline`: `/clear` plus one LF sent, typed as the 6-byte `/clear`, confirmed by a new session of cause `clear`, no `prompt-submitted`, no `wheel` record).
    sidecar: >-
      2026-10-08 · architecture §Established Decisions [Delivery Confirmation] · "`/clear` and a newline is `/clear`" is no longer unit-only: measured live on 2.1.287 (`clear-then-newline`) · chunk 2026-10-08-first-live-test-and-self-drive.
    rationale: >-
      Second occurrence of the claim the previous proposal retires (that `/clear`'s confirmation has no live proof). Report Outcome: "`send-under-hint`, `ends-in-newlines`, `clear-then-newline` measured in start 7". The values are read from the evidence file the report cites.
    basis: "viola-0.1.0/chunks/2026-10-08-first-live-test-and-self-drive/evidence/live-readings.ndjson (row `clear-then-newline`)"
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions → [CLI Version Compatibility]"
    change: >-
      Replace "A `send` under the real CLI's hint is unmeasured since the build." with: as measured live on 2.1.287 at chunk 2026-10-08-first-live-test-and-self-drive (start 7, `send-under-hint`, `evidence/live-readings.ndjson`), a 32-byte `send` begun 4 248 ms after a 2 564-byte long paste was issued waited out the hint inside the gate, was issued once the input box returned, and was confirmed (`ok`, 4 081 ms, inside the 8.5 s bound, no `wheel` record, the turn ended). The two neighbouring "unmeasured" clauses stay as they are: a long or a repeated text pasted under the hint was not measured by that chunk.
    sidecar: >-
      2026-10-08 · architecture §Established Decisions [CLI Version Compatibility] · a `send` under the real CLI's paste hint is measured on 2.1.287: delivered and confirmed inside the 8.5 s bound · chunk 2026-10-08-first-live-test-and-self-drive.
    rationale: >-
      Report Expected amendments ("[Delivery Confirmation] and [CLI Version Compatibility], the sentences carrying the unmeasured live readings … as measured") and Outcome ("`send-under-hint` … measured in start 7"). The report states that the reading was taken but not its values; the values above are read from the evidence file it cites, for the orchestrator to re-read. Sweep: "unmeasured" under the hint occurs three times in this entry; only this sentence (a `send` under the hint) is retired. "a long or a repeated text under the hint is unmeasured" and the long-paste-wrapper row's "Unmeasured: a long or a repeated text pasted while the paste hint stands" are a different claim the reading (a 32-byte text) does not cover.
    basis: "viola-0.1.0/chunks/2026-10-08-first-live-test-and-self-drive/evidence/live-readings.ndjson (row `send-under-hint`)"
  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions → [CLI Version Compatibility]"
    change: >-
      Replace "The `input-not-ready` hint line is unchanged: it is no longer printed for the paste-hint cause, and its wording for the remaining causes (a modal, a screen that never goes quiet, a poisoned screen, a hint past the bound) is open for the founder." with: the `input-not-ready` hint line is no longer printed for the paste-hint cause; its wording for the remaining causes (a modal, a screen that never goes quiet, a poisoned screen, a hint past the bound) was settled by the founder (inputs#I7) and landed at chunk 2026-10-08-first-live-test-and-self-drive: `{name} was not ready for input; send again, and if it repeats a human must look at the session` (`send_hint`, `src/human.rs`), in place of `{name} was not ready for input; viola wait {name}, then send again`. Exit code, refusal, detail and `--json` are unchanged.
    sidecar: >-
      2026-10-08 · architecture §Established Decisions [CLI Version Compatibility] · the `input-not-ready` hint wording is no longer open: the founder's text (inputs#I7) landed in `send_hint`, the `viola wait` advice is gone · chunk 2026-10-08-first-live-test-and-self-drive.
    rationale: >-
      Report Changes → Symbols / APIs (`send_hint`, added lines 216-218: the hint text changes to the founder's wording, inputs#I7; exit code, refusal, detail and `--json` unchanged); Expected amendments ("[CLI Version Compatibility], … the open hint wording"). The entry says the line is unchanged and its wording open. Sweep: architecture quotes neither hint text (`grep 'was not ready for input'`: 0 lines); the one other statement about the hint's advice is in the same entry (next proposal).
    basis: "src/human.rs:216-218"
  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions → [CLI Version Compatibility]"
    change: >-
      In "A `viola wait` with no cursor issued in the window woke on nothing and ran to its deadline, and `wait --after` the earlier cursor returns the logged turn end at once, inside the window, so the `input-not-ready` hint line's advice does not lead out of it.", put the last clause in the past and name the text it judged: "so the advice of the `input-not-ready` hint line of that time (`viola wait {name}, then send again`) did not lead out of it; since chunk 2026-10-08-first-live-test-and-self-drive the line no longer advises `viola wait` (below)."
    sidecar: >-
      2026-10-08 · architecture §Established Decisions [CLI Version Compatibility] · the sentence judging the hint's `viola wait` advice now reads as history of the old text · chunk 2026-10-08-first-live-test-and-self-drive.
    rationale: >-
      Second occurrence of the claim the previous proposal retires (that the hint line still carries its old `viola wait` advice): a present-tense statement about advice the line no longer gives. Report Changes → Symbols / APIs (`send_hint`): the old text `…; viola wait {name}, then send again` is replaced.
    basis: "src/human.rs:216-218"
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: "§Design Philosophy → Cross-platform from the first commit, Windows first"
    change: >-
      Replace "Windows is the live-supported target. The macOS and Linux paths build and pass CI on every commit against the fake agent, …" with wording that keeps the Windows-first design rule and the "no tmux, shell or POSIX-only behaviour" consequence but states the live fact as ruled and measured: the first live run was on the Linux dev host (the founder's ruling R-L1, 2026-10-07T05:43Z, relayed by the overseer; run at chunk 2026-10-08-first-live-test-and-self-drive on `claude` 2.1.287 in foot 1.28.0 under Hyprland 0.56.2); Windows live behaviour stays proven on the CI runner under the fake agent until an interactive Windows host exists; macOS builds and passes CI against the fake agent.
    sidecar: >-
      2026-10-08 · architecture §Design Philosophy (Windows first) · "Windows is the live-supported target" is replaced by the ruled and measured fact: the first live run is on the Linux dev host (R-L1), Windows live stays fake-agent-proven on CI until an interactive Windows host exists · chunk 2026-10-08-first-live-test-and-self-drive.
    rationale: >-
      Report Expected amendments: "§Design Philosophy and §Project Intent, \"Windows live\" against a first live run on the Linux host (R-L1)"; Changes → Dev-tool versions (first readings on the dev host: foot 1.28.0, Hyprland 0.56.2, `wtype`) and Cross-project claims (the installed `claude` 2.1.287 measured live on 2026-10-08). The only live host the product has run on is Linux; no Windows live run exists ([PTY] already says its Windows live measurements are "blocked on an interactive Windows host", which stands). The R-L1 wording is the chunk's `scope.md` §Authority. The Windows-first design priority itself is not touched by the report and is kept.
    basis: "viola-0.1.0/chunks/2026-10-08-first-live-test-and-self-drive/scope.md (§Authority, R-L1)"
  - detector: D-arch-decisions
    severity: warning
    section: "§Project Intent → Scale path"
    change: >-
      Replace "v1 is personal (Windows live, macOS and Linux CI-tested)." with: v1 is personal (first run live on the Linux dev host, R-L1; Windows, macOS and Linux CI-tested against the fake agent; Windows live awaits an interactive Windows host).
    sidecar: >-
      2026-10-08 · architecture §Project Intent → Scale path · "Windows live" replaced: first live on the Linux dev host (R-L1), Windows live awaits an interactive Windows host · chunk 2026-10-08-first-live-test-and-self-drive.
    rationale: >-
      Second occurrence of the claim the §Design Philosophy proposal retires (Windows is the live host). The report's own search (`grep -c -E 'Windows live|live on Windows|founder.s Windows'`: architecture 1 line) hits this line; the §Design Philosophy sentence says the same thing in other words ("live-supported target"). No third occurrence: [Scale / Product], [CI/CD] and §Inherited Defaults speak of CI on three OSes only.
    dependent-of: D-arch-decisions
  - detector: D-arch-resources
    severity: escalate
    section: "§Cross-cutting Patterns → Config management (Viola home)"
    change: >-
      Replace "Resolution order: `--home`, then the grandparent of `VIOLA_DIR` when it is set, then the default." with the as-built order and its limit: a CLI verb takes the home from `--home`, else the default `<user home>/.viola/` (`resolve_home`, `src/cmd/mod.rs:140-148`); only `hook` reads `VIOLA_DIR` and derives the home as its grandparent (`src/cmd/hook.rs:103`). So `viola run --home <dir>` carries the home to the child's hooks through `VIOLA_DIR`, and a CLI verb run inside that session without `--home` resolves the default home, not the session's (read from the code at chunk 2026-10-08-first-live-test-and-self-drive, research M2; every verb of that chunk's live run named `--home`). Whether the `VIOLA_DIR` step for CLI verbs is owed (a build item) or dropped from the design is the founder's call — hence escalate: the body must not keep stating it as built.
    sidecar: >-
      2026-10-08 · architecture §Cross-cutting Patterns → Config management · home resolution corrected to as-built: CLI verbs read `--home` else the default; only `hook` reads `VIOLA_DIR` (the CLI-verb `VIOLA_DIR` step is not built) · chunk 2026-10-08-first-live-test-and-self-drive.
    rationale: >-
      Report Changes → Spec claims disproved → "Architecture §Cross-cutting Patterns \"Config management\", the `VIOLA_DIR` step of home resolution": the extract states the step for every verb; read from the code, the CLI verbs take the home from `--home`, else `~/.viola`, and only `hook` reads `VIOLA_DIR`. Expected amendments names this site. No new env var lands (Symbols / APIs: none), so the detector's "new resource registered" half is clean; the drift is the registered reader set of an existing env var. Escalated because the report measures the divergence but rules nothing on which side (doc or code) is right. Sweep of the four `VIOLA_DIR` lines in architecture: [Naming] (the name only) and Hook contract's `hook statusline` (a `hook` read) stand; the §Occupied Resources entry restates the claim (next proposal).
    basis: "src/cmd/mod.rs:140-148; src/cmd/hook.rs:103"
  - detector: D-arch-resources
    severity: escalate
    section: "§Occupied Resources → Environment variables (`VIOLA_DIR`)"
    change: >-
      Replace "It is read by `hook` (…) and by `mcp` and every CLI verb run inside the session. Each derives the viola home as the grandparent of `VIOLA_DIR`, so a session started with `--home` keeps its hooks, MCP server and nested CLI calls in that home." with: it is read by `hook` alone (for the recorded `endpoint`, the SessionEnd direct-append fallback, the statusline wrapper and the diagnostics directory; `src/cmd/hook.rs:103`), which derives the viola home as its grandparent, so a session started with `--home` keeps its hooks in that home. No CLI verb reads it as built (`resolve_home`, `src/cmd/mod.rs:140-148`: `--home`, else the default), so a nested CLI call must name `--home` itself; the `mcp` verb is not built yet. Same founder question as the Config management proposal.
    sidecar: >-
      2026-10-08 · architecture §Occupied Resources → Environment variables · `VIOLA_DIR` readers corrected to as-built: `hook` alone; nested CLI verbs do not read it · chunk 2026-10-08-first-live-test-and-self-drive.
    rationale: >-
      Second occurrence of the claim the Config management proposal retires (every verb run inside the session resolves its home through `VIOLA_DIR`), here in the env-var registry: "and by `mcp` and every CLI verb run inside the session … nested CLI calls in that home". Report Changes → Spec claims disproved: "only `hook` reads `VIOLA_DIR` (`src/cmd/hook.rs:103`)". That `mcp` is not built is architecture's own baseline ([Deployment / Distribution]: "Until the `mcp` verb exists"), not a change of this chunk.
    basis: "src/cmd/hook.rs:103; src/cmd/mod.rs:140-148"
    dependent-of: D-arch-resources
````

## security-plan

Verdict: 4 proposals (nothing stripped; entities 0).
1. Threat Model Summary, the human-terminal stdin vector and the widened list — APPLY. Check 1: "Boundary widening", ratified as architecture 1; "Verbatim upstream copy kept current" (judged like any body amendment, the label stays). Check 5.
2. Threat Model Summary, the agent-typed takeover key — APPLY. Check 1: "Boundary widening" (a key an agent types crosses the human's own input); ratified by the founder (inputs#I3, his ruling that set the method; inputs#I12 and inputs#I13, given after the compositor form and its price were shown, relayed verbatim). Check 5.
3. Threat Model Summary, "CLI arguments, flags and environment" — REJECT. Check 1: "Sequencing deferral": the route owns the `VIOLA_DIR` step (`working-route.md:118`, whose CARRY names this plan's Input Validation CLI row); the spec stays. The as-landed state is stated once, in architecture.
4. Input Validation, the Home path sentence — REJECT (dependent of 3).

### The return

````yaml
# D-security-deps: no drift. The report's Dependencies bullet reads "none" (Cargo.toml, Cargo.lock unchanged, the preservation guard); foot, Hyprland and wtype are first readings of dev-host tools, none installed or upgraded.
# D-security-auth: no drift. The report touches no auth library, token, cookie, key or secret source.
# D-security-input: the detector's escalate case (an unvalidated new boundary) is NOT met. The report adds no external-input surface, and the one widened surface (`is_reply`) is a closed exact grammar with 27 negative controls. The four proposals below are documentation drift on input surfaces, so each is filed `warning`:
#   1-2 are the two amendments the report's "Expected amendments (from plan)" names for security-plan §Threat Model Summary. security-plan holds no text on the wheel's stdin classifier, the closed list, a compositor or wtype (its two `compositor|wtype` grep hits are the word "newtype").
#   3-4 are security-plan's two restatements of the `VIOLA_DIR` home-resolution claim the report lists under "Spec claims disproved by measurement". No detector invariant covers them strictly; D-security-input is the nearest. The other four `VIOLA_DIR` lines in security-plan name `hook` as the reader and agree with the measurement.
# Considered, not proposed:
#   - `plansDirectory` (three lines, all about `viola verify` Run D): the report says this chunk did not measure Run D's plan file, so the Run D claims stand as last measured.
#   - trailing CR and only-newlines: security-plan's four lines state the typed-text rule and that CR is allowed, and carry no forecast the readings falsify.
#   - the `input-not-ready` hint text: no site in security-plan.
proposals:
  - detector: D-security-input
    severity: warning
    section: >-
      Threat Model Summary → Attack surface (a new vector, beside "CLI input, the `send` text written into the PTY")
    change: >-
      Add a vector for the human's terminal on `viola run`'s stdin: bytes there that fall outside the wheel's closed non-editing list (F-W2: focus reports, mouse reports, terminal replies) read as human typing and take the wheel from the driver (a `wheel` record `{human, human-input}`); as of chunk 2026-10-08-first-live-test-and-self-drive the list also holds seven terminal reply shapes after `CSI` (eight literal strings), each by exact grammar, fields compared as bytes, no intermediate byte, field count exact: `0 n` (DSR status), `? 997;1 n` and `? 997;2 n` (colour scheme), `4;n;n t` (window size), `6;n;n t` (cell size), `8;n;n t` (text area), `48;n;n;n;n t` (in-band resize), `> 4;n m` (modifyOtherKeys reply); this is a boundary widening ratified by the founder himself on 2026-10-08 after the exact grammars and the narrower option of two were shown (inputs#I17), measured on foot 1.28.0; the nearest human keys `n`, `t`, `m`, their Alt forms and kitty key events stay typing, and a reply this terminal does not write is never added on a guess.
    sidecar: >-
      2026-10-08 (chunk 2026-10-08-first-live-test-and-self-drive): Threat Model Summary, Attack surface gains the human-terminal stdin vector and records the closed non-editing list (F-W2) widened by seven terminal reply shapes, the founder's ratification of 2026-10-08 (inputs#I17).
    rationale: >-
      Report Changes, Symbols / APIs (`is_reply`): the closed list gains seven reply shapes, "Ratified by the founder himself after the exact grammars and the narrower option were shown (inputs#I17)"; Deviations calls it "a boundary widening ratified by the founder"; Expected amendments names "security-plan §Threat Model Summary, the same widening with its ratification" and notes security-plan has 0 grep hits, so the site is the section by name. security-plan records every other founder-ratified boundary widening in its body and has no text on this surface, so the detector's check has no §Input Validation mandate to confirm `is_reply` against. Filed as warning, not the detector's escalate: the boundary is validated (Coverage: "a closed, exact grammar over bytes; nothing is parsed into a value"; Outcome, security: "the closed list holds the seven shapes and nothing wider: met", all 27 negative controls typing).
    basis: >-
      src/run/wheel.rs:506-511 (and :499; doc comment 474-477)
  - detector: D-security-input
    severity: warning
    section: >-
      Threat Model Summary → Attack surface (Trust boundary of the human-terminal stdin vector the proposal above adds; a standalone bullet there if that proposal is declined)
    change: >-
      Record the dev-host measurement that crossed the human's own input: at chunk 2026-10-08-first-live-test-and-self-drive an agent typed five keys into live sessions' terminals, which viola reads as the human's (`wheel` `{human, human-input}`), each by `wtype` on the socket of a nested compositor the chunk started itself, after its guard read; there was one compositor start for all live work (kept up across the second plan revision under a 3 hour bound on the operator's word, inputs#I15), the desktop's own lock was untouched and read `true` after the start, before each live step and twice after the end, and the one call naming the desktop instance was `hyprctl --instance <desktop signature> locked`; this is the founder's ruling (inputs#I12, inputs#I13); no product code took part (a live reading of the existing human-input path).
    sidecar: >-
      2026-10-08 (chunk 2026-10-08-first-live-test-and-self-drive): Threat Model Summary records the dev-host takeover-key measurement, an agent-typed key through the chunk's own nested compositor on a locked desktop, on the founder's ruling (inputs#I12, inputs#I13).
    rationale: >-
      Report Expected amendments: "security-plan §Threat Model Summary, a key an agent types into a session's terminal through a compositor crosses the human's own input, and the compositor is one the chunk starts itself, nested on a locked desktop"; Decisions & corrections (the founder's ruling I12/I13, the operator's 3 hour bound I15); Outcome, security criterion "the desktop lock was not touched and read locked: met" (one start line and one end line, every lock reading `true`, five keys by `wtype` on the own socket); Coverage, "the takeover by a compositor-typed key (no code ...)". security-plan already records the earlier dev-host boundary crossings (Data classification, credential bullet) and holds nothing on this one. Warning: no product surface changed and no boundary is unvalidated.
    basis: >-
      report.md, Outcome "(security) the desktop lock was not touched and read locked"; evidence/compositor.md (no source line: no code)
  - detector: D-security-input
    severity: warning
    section: >-
      Threat Model Summary → Attack surface → Vector "CLI arguments, flags and environment" (Entry point)
    change: >-
      Say which process reads which: the CLI verbs take the home from `--home <dir>`, else `~/.viola`, and that home chooses the state tree, snapshot, `endpoint` and ledger stamps they use; `VIOLA_DIR` is read by `viola hook` alone (the path to the snapshot whose `statusline_command` `hook statusline` runs) and is no step of any other verb's home resolution.
    sidecar: >-
      2026-10-08 (chunk 2026-10-08-first-live-test-and-self-drive): Threat Model Summary, "CLI arguments, flags and environment" now reads that the CLI verbs resolve the home from `--home`, else `~/.viola`, and only `hook` reads `VIOLA_DIR` (read from the code, research M2).
    rationale: >-
      Report Changes, Spec claims disproved by measurement: "the CLI verbs take the home from `--home`, else `~/.viola` (`resolve_home`, `src/cmd/mod.rs:140-148`); only `hook` reads `VIOLA_DIR` (`src/cmd/hook.rs:103`)", with "`grep -c VIOLA_DIR`: ... security-plan 6". This Entry point lists "All subcommands" and then "`--home <dir>` and `VIOLA_DIR` choose which state tree ... are used (Cross-cutting Patterns: Config management)", restating the architecture extract the report disproves. The documented surface is wider than the code's, so this is warning. D-security-input is the nearest detector (an external-input surface's description), not a strict violation of its invariant.
    basis: >-
      src/cmd/mod.rs:140-148; src/cmd/hook.rs:103
  - detector: D-security-input
    severity: warning
    section: >-
      Input Validation → row "CLI arguments / stdin" (the **Home path** sentence)
    change: >-
      The Home path sentence should name each source's reader and change nothing else of the mandate or its interim gaps: `--home` (the CLI verbs' home source, else `~/.viola`) and `VIOLA_DIR` (read by `viola hook` alone) are canonicalised (`std::fs::canonicalize`) and then pass the `~/.viola/` strict-modes check before any snapshot, `statusline_command` or ledger read.
    sidecar: >-
      2026-10-08 (chunk 2026-10-08-first-live-test-and-self-drive): Input Validation, "CLI arguments / stdin" Home path names its readers: `--home` for the CLI verbs (else `~/.viola`), `VIOLA_DIR` for `hook` alone (research M2).
    rationale: >-
      Second occurrence of the claim the proposal above retires (a `VIOLA_DIR` step in the CLI verbs' home resolution). The row states "`--home` and `VIOLA_DIR` are canonicalised" as the general rule and `viola hook` as its interim exception, but by the report (Spec claims disproved, research M2) `hook` is the only reader of `VIOLA_DIR`, so for `VIOLA_DIR` the exception is the whole case. The report does not say whether canonicalisation has landed for `--home`, so the mandate's wording is kept and only the readers are named. Dependent of the D-security-input proposal on Threat Model Summary → "CLI arguments, flags and environment".
    basis: >-
      src/cmd/mod.rs:140-148; src/cmd/hook.rs:103
    dependent-of: D-security-input
````

## design-system

Verdict: 1 proposal (nothing stripped; entities 0).
1. Component Patterns → 2, the `input-not-ready` hint — APPLY. Check 1: "Accurate this-chunk addition". Check 5. Out of D-design-tokens' own invariant, as the detector says; raised under check 5 all the same.

### The return

````yaml
proposals:
  - detector: D-design-tokens
    severity: warning
    section: '§Surface: cli → Component Patterns → 2 (`viola send`: the readback mirror)'
    change: 'The `not-delivered · input-not-ready` hint bullet reads: `<name> was not ready for input; send again, and if it repeats a human must look at the session` (replacing `<name> was not ready for input; viola wait <name>, then send again`); the report reads the site at design-system.md:764.'
    sidecar: '2026-10-08 (chunk 2026-10-08-first-live-test-and-self-drive): cli Component Patterns 2, the `input-not-ready` hint text changed to "send again, and if it repeats a human must look at the session" (founder wording, inputs#I7); exit code, refusal, detail and `--json` unchanged.'
    rationale: 'The detector''s own check reads clean: all three Coverage rows carry `tokens n/a`, none `hardcoded✗`, so no token drift against §Color Palette / §Spacing / §Typography. This proposal is raised on the same Coverage row instead: the `input-not-ready` hint line is flagged `tokens n/a (cli text, design-system §Surface: cli → Component Patterns → 2)`, i.e. its governing value is that section''s hint specimen, and the report''s Changes → Symbols / APIs (`send_hint`) states the rendered text changed from `{name} was not ready for input; viola wait {name}, then send again` to `{name} was not ready for input; send again, and if it repeats a human must look at the session`. Changes → Expected amendments lists this exact site (`grep -c ''was not ready for input''`: design-system 1 line). Sweep of design-system for other occurrences of the retired claim (grep `was not ready`, `viola wait`, `input-not-ready`, `send again`, `hint:`; and a read of both surfaces, the exit-code table, Streams and the cli bans): none. The `turn-running` hint (`viola wait <name> first`) is a different hint and unchanged; the `RefusalDetail::input-not-ready` status row and the exit-13 row name the detail code only, not the hint text; the "no hint ever names `viola release`" claim (T3) still holds for the new wording. No dependent proposals. If the orchestrator holds that an Expected-amendment site with a clean `tokens` flag is outside D-design-tokens, drop this proposal as out of detector scope rather than as wrong on the facts.'
    basis: 'src/human.rs:216-218'
````

## layout-templates

Verdict: 1 proposal (nothing stripped; entities 0).
1. Output structure — `viola send`, the hint specimen — APPLY. Check 1: "Accurate this-chunk addition". Check 5.

### The return

````yaml
proposals:
  - detector: D-layout-surface
    severity: warning
    section: "§Surface: cli › Output structure — `viola send`"
    change: >-
      In the `viola send` specimen block, the hint line under the `[/ ] unable  builder  not-delivered  input-not-ready` row should read
      `hint: builder was not ready for input; send again, and if it repeats a human must look at the session   <- stderr, the last line`
      (replacing `hint: builder was not ready for input; viola wait builder, then send again`); nothing else in the block moves.
    sidecar: >-
      2026-10-08 (chunk 2026-10-08-first-live-test-and-self-drive): `viola send` output specimen, the `input-not-ready` hint line re-worded to the shipped text
      "{name} was not ready for input; send again, and if it repeats a human must look at the session" (founder's wording, inputs#I7); exit code, refusal, detail and `--json` unchanged.
    rationale: >-
      The report adds no new surface or region ("Coverage of new surfaces (no new external surface; two changed ones)"), so no wireframe entry is missing.
      One of the two changed surfaces is the `input-not-ready` hint line on stderr, and its wireframe entry in this doc is now wrong:
      report Changes › Symbols / APIs (`send_hint`, `src/human.rs`, added lines 216-218) changes the hint text from
      "{name} was not ready for input; viola wait {name}, then send again" to
      "{name} was not ready for input; send again, and if it repeats a human must look at the session".
      The report's Expected amendments names this exact site ("layout-templates §Surface: cli › Output structure — `viola send`, the hint specimen", 1 line, research M10 reads it at `:415`),
      and the gate `grep -c 'was not ready for input; viola wait' src/human.rs` reads 0.
      Sweep for other occurrences of the retired claim in layout-templates: the old wording appears once only (the specimen line).
      The one neighbouring site, §Surface: cli › Component — Primary navigation (verb structure) › Discoverability, which lists `viola wait builder` as an example of a hint naming the next verb,
      is NOT retired by this change: design-system §Surface: cli → Component Patterns → 2 still carries the `not-delivered · turn-running` hint "a turn is running; viola wait <name> first", which the report does not touch,
      so that example stays true and gets no dependent proposal. The same sentence's "No hint names `viola release`" is confirmed by the report's Coverage line (the new hint "does not name `release`").
      The §Component — Primary content block 2 "Hint line" bullet quotes no `input-not-ready` wording and needs no edit.
    basis: ".andromeda/layout-templates.md:415 (the report's research M10 reading); src/human.rs:216-218 (report, Symbols / APIs — `send_hint`)"
````

## test-plan

Verdict: 9 proposals (nothing stripped; entities 0).
1. §3 → 5-command implementation, `--local-live` alone — APPLY. Check 1: "Accurate this-chunk addition" (wording reconciled to the implementation as fired). Checks 5 and 6.
2. The same key, "The default is `--all`" — APPLY (dependent of 1).
3. §6 Path 2, the owed live readings — APPLY (checks 5 and 6; the values are the orchestrator's own rows).
4. §6 Path 2, the `/clear` live proof — APPLY (dependent of 3).
5. §3 → 5-command implementation, the seventeen-row firing owed — APPLY (dependent of 3).
6. §6 Path 5, the classifier's tables and the reply probe — APPLY. Check 5.
7. §6, the untestable list, the real CLI's local routes — APPLY. Check 1: no rule; accurate, this chunk's (three live sessions outside `viola verify`), and the sentence says "only".
8. §11, the real-CLI ban's second sentence — APPLY (dependent of 7; the ban is kept).
9. §1, Untestable zones — APPLY (dependent of 7; "Verbatim upstream copy kept current").

### The return

````yaml
proposals:
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 → 5-command implementation (label `run`, the `--local-live` bullet)"
    change: >-
      State that `--local-live` passed alone runs no other suite (the `--all` default does not apply to it; with other selectors it runs after them), and that because it names no program it stamps whichever `claude` is first on `PATH`, so a firing heads `PATH` with the install directory of the version to stamp.
    sidecar: >-
      2026-10-08 first-live-test-and-self-drive: §3 `run` `--local-live` reworded as read from the code and seen in the firing: a lone `--local-live` runs no other suite, and the stamped version is the first `claude` on `PATH`.
    rationale: >-
      Report Changes, "Spec claims disproved by measurement: Test-plan §3 `run`, `--local-live`" and "Expected amendments: test-plan §3 `run`". The key file reads "`--local-live` appends `viola verify` ... Otherwise, after the selected suites" beside "The default is `--all`", which reads as a lone `--local-live` running the default suites first; the firing (`evidence/round-072838Z.txt`, `evidence/local-live.md`) ran suite `local-live` alone. "Harness / gate surface: none", so the code did not move: the wording is the drift. Checked for a one-sided change: obs-plan's single `local-live` line says only that the real-`claude` verify runs through the local `run --local-live`, which refuses under `CI`; it still agrees and needs no edit. Test-plan §1 "Local-only mode adds `viola verify` against the real CLI" was read and stands (it asserts no ordering against the default).
    basis: "crates/viola-e2e/src/harness/run.rs:58-65 (`Selection::from_flags`) and :379-385 (no program named)"
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 → 5-command implementation (label `run`, the Command body sentence \"The default is `--all`, which runs steps 1 and 2 in order\")"
    change: >-
      Qualify the default: with no selector and no `--local-live` the selection is `--all` (steps 1 and 2 in order); `--local-live` alone is a selection of its own and runs neither step.
    sidecar: >-
      2026-10-08 first-live-test-and-self-drive: §3 `run` Command body, the `--all` default scoped to a call with no selector and no `--local-live`.
    rationale: >-
      Second occurrence, in the same key file, of the claim the primary retires: an unqualified "The default is `--all`" is what makes a lone `--local-live` read as running the default suites. Report Changes, "Spec claims disproved by measurement: Test-plan §3 `run`, `--local-live`" quotes the pairing "after the selected suites (the default selection is `--all`)".
    basis: "crates/viola-e2e/src/harness/run.rs:58-65"
    dependent-of: D-tests-obs-harness
  - detector: D-tests-coverage
    severity: warning
    section: "§6 E2E Test Strategy → Scenario: Path 2 — confirmed `send` with CL-1 events (Verification signal, the bullet \"A text ending in newlines\", its last sentence)"
    change: >-
      Replace "Owed to \"First live test and self-drive\": such a send on a live CLI, a text of only newlines, and a trailing CR." with the readings as measured live on `claude` 2.1.287 at chunk 2026-10-08-first-live-test-and-self-drive: `ends-in-newlines` and `clear-then-newline` read in start 7; a text of only newlines (two LF) is issued with `text_bytes` 0, draws no `prompt-submitted` and no `wheel` record, and ends `not-delivered`/`no-prompt-submitted`, exit 13, after 10.0 s; a text ending in one CR is issued whole (`text_bytes` 32), the CLI submits it without the CR, the exact match fails, the prompt is filed `origin: human`, a `wheel {human, human-input}` record follows and the `send` ends `not-delivered`/`no-prompt-submitted`, exit 13, although the turn ran: no fix and no test at any tier pins the trailing-CR outcome yet.
    sidecar: >-
      2026-10-08 first-live-test-and-self-drive: §6 Path 2, the three live readings owed to this chunk recorded as measured; the trailing-CR and only-newlines outcomes stated as read, the trailing-CR defect left open.
    rationale: >-
      Report Changes, "Spec claims disproved by measurement" (the driver text ending in one CR; the text of only newlines) and "Expected amendments: test-plan §6 Path 2, the owed live readings as measured or still owed"; Outcome, "the readings reader green" (start 7: `send-under-hint`, `ends-in-newlines`, `clear-then-newline`; start 8: `only-newlines`, `trailing-cr`). The "owed" sentence is now false. The report states no outcome for `ends-in-newlines` and `clear-then-newline` beyond "measured" and that only two readings falsify or sharpen an expectation, so the orchestrator should read `evidence/live-readings.ndjson` before writing a result for those two. The neighbouring "refusal at the 8.5 s bound is unit-tier only" sentence is left alone: the report gives no result for `send-under-hint`.
    basis: "evidence/live-readings.ndjson (start 7 and start 8); evidence/live-run.ndjson"
  - detector: D-tests-coverage
    severity: warning
    section: "§6 E2E Test Strategy → Scenario: Path 2 — confirmed `send` with CL-1 events (Verification signal, the `local` bullet, the parenthesis ending \"the live proof is owed to \"First live test and self-drive\"\")"
    change: >-
      Replace "the live proof is owed to \"First live test and self-drive\"" with the proof as taken: on live `claude` 2.1.287 a `/clear` was confirmed by a new session and the next send landed in the cleared session (chunk 2026-10-08-first-live-test-and-self-drive).
    sidecar: >-
      2026-10-08 first-live-test-and-self-drive: §6 Path 2 `local` bullet, the live proof of `/clear` confirmed by its new session recorded as taken.
    rationale: >-
      Second occurrence of the retired "owed to this chunk" claim. Report Outcome, "(arch) the live run reader green: met — ... `/clear` confirmed by a new session, the skill sent again into the cleared session".
    basis: "evidence/live-run.ndjson"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → 5-command implementation (label `run`, the `--local-live` bullet, its last sentence)"
    change: >-
      Replace "the live firing at seventeen rows is owed to \"First live test and self-drive\"" with the firing as read: fired once at chunk 2026-10-08-first-live-test-and-self-drive, suite `local-live` 1 passed 0 failed, `stamped 2.1.287  17 pass  0 fail`.
    sidecar: >-
      2026-10-08 first-live-test-and-self-drive: §3 `run` `--local-live`, the seventeen-row live firing recorded as done (2.1.287, 17 pass, 0 fail).
    rationale: >-
      Third occurrence of the retired "owed to this chunk" claim, in the keyed contract. Report Changes, "Counts / qualifiers moved" (`run --local-live` stamped `2.1.287  17 pass  0 fail`, the ledger's row count stays seventeen) and Outcome, "(arch) the live round green, fired once: met" and the Gates line for `run --local-live` (`round: COMPLETE · legs fired 1/1`).
    basis: "evidence/local-live.md; evidence/round-072838Z.txt"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§6 E2E Test Strategy → Scenario: Path 5 — human takes the wheel, refusal, `release` (Surfaces involved, the \"As landed\" record)"
    change: >-
      Add the as-landed record of chunk 2026-10-08-first-live-test-and-self-drive: the stdin classifier's closed non-editing list (F-W2) gained seven terminal reply shapes, pinned at unit tier in `src/run/wheel.rs` by the rstest table `classifier_terminal_reply_is_not_editing` (8 reply cases), the whole-and-split test over the 23 `REPLIES` strings and 27 negative controls that stay typing; the list's reading on a real terminal is a local live measurement, not a suite (the reply probe on foot 1.28.0: seven replies read as typing before the fix, 0 of 23 after, keys `n`, `t`, `m` still take the wheel), since the outer-PTY tier writes no terminal reply of its own.
    sidecar: >-
      2026-10-08 first-live-test-and-self-drive: §6 Path 5, the classifier's reply tables (unit) and the reply probe on a real terminal (live reading of the closed list) recorded as landed.
    rationale: >-
      No tier is missed: both changed surfaces carry unit tests (report "Coverage of new surfaces": `is_reply` unit✓ and live✓, `send_hint` unit✓), which is the tier §4 root bin sets for the wheel state machine. The drift is the record: the report's "Expected amendments" names test-plan §6 Path 5 for "the classifier's unit tables as extended and the reply probe on a real terminal as the live reading of the list", and test-plan holds 0 lines for `closed list`, `terminal repl(y|ies)` or `F-W2` (the report's own grep), so Path 5's as-landed list names neither the new tables nor the one tier where the defect was visible (seven replies took the wheel 237 ms after a live CLI started while the CI tiers were green). Counts are the report's: unit 1370 → 1405 (8 reply cases, 27 negative controls), `REPLIES` 15 → 23.
    basis: "src/run/wheel.rs:617 (the reply table, row 608-619) and :528 (`REPLIES`); evidence/classifier-pins-first.md; evidence/reply-probe-fixed.ndjson"
  - detector: D-tests-framework
    severity: warning
    section: "§6 E2E Test Strategy → the list \"Skipped as \"untestable\" per test-scope Sec 1\", its first bullet (\"The real `claude` CLI and Anthropic backend. Local-only via `agent-run run --local-live` (`viola verify`, Haiku).\")"
    change: >-
      Say the suites reach the real `claude` only through `agent-run run --local-live` (`viola verify`), and that a route entry's live readings of a real session are a chunk-owned local measurement kept in that chunk's `evidence/` (its own launcher, driver and readers; a key typed with `wtype` in a compositor the chunk starts itself, by the founder's ruling): not a test layer, not harness surface, never in CI.
    sidecar: >-
      2026-10-08 first-live-test-and-self-drive: §6 untestable list, the real CLI's two local routes told apart: the harness suite `run --local-live`, and a chunk-owned live measurement outside the harness.
    rationale: >-
      The chunk's own tests are on-spec (rstest `#[case]` at `src/run/wheel.rs:617`, run through `scripts/agent-run.sh run --unit`, `run`, `pre-push`: nextest, coverage, Playwright, as §2 and §4 set). The off-spec runner is the live half: report "Counts / qualifiers moved" (eight live `claude` starts: five by the round, then starts 6, 7, 8), "Harness / gate surface: none" (the measurement scripts `live-compositor.sh`, `live-start.sh`, `live-drive.py`, `key-probe-own.sh`, `reply-probe.sh`, `reply-probe-child.py` "are not harness surface"), and "Coverage of new surfaces", which counts them as a tier (`tests e2e-live✓`, `live✓`) with `jq` readers of `evidence/*.ndjson` as gates. Starts 6 to 8 ran real sessions under `viola run`, raised and answered real dialogs, and did not go through `run --local-live` or `viola verify`, so "Local-only via `agent-run run --local-live`" no longer describes every way the real CLI is run. The method is the founder's (Decisions & corrections, inputs#I12, inputs#I13), so the doc is what moves. Warning only: the orchestrator may judge the sentence was always about suites.
    basis: "evidence/live-sessions.ndjson (eight `start` lines); evidence/live-start.sh; evidence/live-drive.py"
  - detector: D-tests-framework
    severity: warning
    section: "§11 Test Anti-Patterns → Test Strategy (the ban \"NEVER run the real `claude` CLI or `viola verify` against the real CLI in CI\", its second sentence \"The real CLI is local-only through `agent-run run --local-live`, which refuses when `CI` is set.\")"
    change: >-
      Keep the ban; reword the second sentence so it covers both local routes: the suites reach the real CLI only through `agent-run run --local-live`, which refuses when `CI` is set, and a route entry's chunk-owned live measurement is local too and never a CI step.
    sidecar: >-
      2026-10-08 first-live-test-and-self-drive: §11 Test Strategy, the real-CLI ban's local-only sentence widened to name the chunk-owned live measurement beside `run --local-live`.
    rationale: >-
      Same claim as the primary, restated in a ban: "local-only through `agent-run run --local-live`" as the single local route. Report "Counts / qualifiers moved" (starts 6, 7, 8 outside the round) and "Harness / gate surface" (the measurement scripts are not harness surface).
    basis: "evidence/live-sessions.ndjson"
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → Untestable zones, the first bullet (\"Real `claude` CLI and Anthropic backend in CI ... Local only, through `viola verify`.\")"
    change: >-
      Replace "Local only, through `viola verify`." with: local only, through `viola verify` for the suites, and through a route entry's own live measurement for a real session's dialogs and readings (as at chunk 2026-10-08-first-live-test-and-self-drive).
    sidecar: >-
      2026-10-08 first-live-test-and-self-drive: §1 Untestable zones, the real CLI's local reach no longer stated as `viola verify` alone.
    rationale: >-
      Same claim as the primary, restated in the scope table without the `--local-live` token: the bullet lists "live sessions ... real CLI-native modals" as reachable locally through `viola verify` only, and the report's Outcome (`verification-matrix.json#v1-31`: a question, a permission answered `deny` and a plan, each raised by the live CLI and decided by its id) shows them reached in live sessions outside `viola verify`. The §1 viola-agent-claude entity line ("Locally it is agent-runnable through `viola verify`") and the inherited-credentials line ("local `viola verify` / live tests") were read and stand.
    basis: "evidence/live-run.ndjson"
    dependent-of: D-tests-framework
````

## obs-plan

Verdict: 3 proposals (nothing stripped; entities 0).
1. §3 → OTel SDK init, step 3 — REJECT. Check 1: "Sequencing deferral", the route owner as for architecture 10; the contract keeps the designed resolution.
2. §3 → Log file location, Path — REJECT (dependent of 1).
3. §1, the copy of init step 3 — REJECT (dependent of 1).
Raised by the orchestrator, on the operator's word at this wrap (the first citation sweep): §9 CI Integration, the G2 probe sentence reworded so that its other-file case carries no line citation — APPLIED before the sweep's write.

### The return

````yaml
proposals:
  - detector: D-obs-stack
    severity: warning
    section: '§3 → OTel SDK init'
    change: 'Init order step 3 reads: resolve the home from `--home`, else `~/.viola/` (`resolve_home`); that resolution has no `VIOLA_DIR` step, and of all verbs only `hook` reads `VIOLA_DIR`. Was: `--home` → grandparent of `VIOLA_DIR` → `~/.viola/`, stated for every role. The per-role instance sources, the role list and steps 1, 2, 4, 5 and 6 are unchanged.'
    sidecar: '2026-10-08-first-live-test-and-self-drive — init step 3: the home is `--home`, else `~/.viola/`; the `VIOLA_DIR` step is not a step of every role (only `hook` reads `VIOLA_DIR`). Read from the code at this chunk; no code changed.'
    rationale: 'Report, Changes → Spec claims disproved by measurement, the `VIOLA_DIR` bullet: "the CLI verbs take the home from `--home`, else `~/.viola` (`resolve_home`, `src/cmd/mod.rs:140-148`); only `hook` reads `VIOLA_DIR` (`src/cmd/hook.rs:103`)", with `grep -c VIOLA_DIR` reading obs-plan 5 lines and registries 2 files. The init order is the obs harness setup §3 specifies, and its step 3 states the disproved three-step resolution for every role, so the setup as written is off the code as read. The claim is disproved by reading, not by a code change of this chunk (the preservation guard holds `src/cmd` unchanged). Of the five body lines the grep counts, one is this claim (the §1 copy, proposed below); the other four state that the `mcp` sink depends on `VIOLA_DIR` being set, a different claim (sink selection) that §1''s closing note already places under D-09 and that the report did not measure, so they are not proposed here.'
    basis: 'src/cmd/mod.rs:140-148 (`resolve_home`); src/cmd/hook.rs:103'
  - detector: D-obs-stack
    severity: warning
    section: '§3 → Log file location'
    change: 'The Path sub-bullet reads: `<home>` is resolved from `--home`, else `~/.viola/`; only `hook` reads `VIOLA_DIR` (§3 → OTel SDK init, step 3). Was: `<home>` is resolved `--home` → grandparent of `VIOLA_DIR` → `~/.viola/`. The path, basenames, detail-file rule, permissions and Rotation are unchanged.'
    sidecar: '2026-10-08-first-live-test-and-self-drive — Log file location, Path: `<home>` is `--home`, else `~/.viola/`; the `VIOLA_DIR` step dropped from the general resolution (only `hook` reads `VIOLA_DIR`), the same reading as init step 3.'
    rationale: 'Second occurrence of the claim the primary retires: this key restates the three-step home resolution in its own words. Same source, the report''s Spec claims disproved `VIOLA_DIR` bullet; this key file is one of the two registry files that hold the token.'
    basis: 'src/cmd/mod.rs:140-148 (`resolve_home`); src/cmd/hook.rs:103'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§1 Obs Scope Summary → Observability harness specification → OTel SDK init (Init order per process, step 3)'
    change: 'Step 3 reads: resolve the home (`--home`, then the default; only `hook` reads `VIOLA_DIR`) and the instance (`VIOLA_NAME` or an argument). Was: resolve the home (`--home`, then the grandparent of `VIOLA_DIR`, then the default). The other five steps and the per-role sentence after them are unchanged.'
    sidecar: '2026-10-08-first-live-test-and-self-drive — §1 copy of init step 3 brought current: the home is `--home`, then the default; the `VIOLA_DIR` step is not general (only `hook` reads `VIOLA_DIR`).'
    rationale: 'Third occurrence of the retired claim, in the §1 copy of the init order. §1 is kept current by the founder''s ruling of 2026-10-04 stated in its closing note (a proposal inside it is judged like any body amendment, and its label stays), so the copy is amended with §3 rather than left to the "§3 wins" clause. Same source: the report''s Spec claims disproved `VIOLA_DIR` bullet.'
    basis: 'src/cmd/mod.rs:140-148 (`resolve_home`); src/cmd/hook.rs:103'
    dependent-of: D-obs-stack
# D-obs-instrumentation: no drift. The report adds no operation: `is_reply` gains match arms inside the stdin classifier (Coverage: no line per byte in the stdin hot loop, by rule; the effect is the existing `wheel` record) and `send_hint` changes the stderr text of an existing refusal (no new event). The measured `text_bytes` readings (32 for the CR-ended text, 0 for two LF) and the dialog / wheel lines agree with §4 and §6 as written.
# D-obs-pii: no drift. No logging is added; the changed hint names the instance only (no path, pid or sent text), and obs-plan carries no copy of the hint text. The report's Dependencies bullet is "none", so the library half of D-obs-stack holds.
````

## a11y-plan

Verdict: 2 proposals (nothing stripped; entities 0).
1. §3 → Keyboard test harness, the closed list as extended — APPLY. Check 1: "Boundary widening", ratified as architecture 1. Check 5. The plan's entry also names §1: the detector read §1's clause as true as written (it names the class by reference), so §1 takes no edit.
2. §3 → Keyboard test harness, the live reading of the takeover — APPLY. Check 1: ratified as security-plan 2. Check 5.

### The return

````yaml
proposals:
  - detector: D-a11y-surface
    severity: warning
    section: "§3 → Keyboard test harness"
    change: >-
      In the Tooling label's tui sentence, the stdin classifier's closed non-editing list (F-W2) reads: focus reports, mouse reports (X10, SGR, urxvt) and terminal replies — DA1, DA2, CPR, DECRPM, kitty flags, OSC and DCS replies, and, after `CSI`, seven further reply shapes matched by exact grammar (fields compared as bytes, no intermediate byte, field count exact), ratified by the founder on 2026-10-08 (inputs#I17) after measurement on foot 1.28.0: `0 n` (DSR status), `? 997;1 n` and `? 997;2 n` (colour scheme), `4;n;n t` (window size), `6;n;n t` (cell size), `8;n;n t` (text area), `48;n;n;n;n t` (in-band resize), `> 4;n m` (modifyOtherKeys reply); every other byte still takes the wheel, the nearest human keys `n`, `t`, `m`, their Alt forms and kitty key events included; the replies are proven by the classifier's unit table and read on a real terminal by the reply probe (foot 1.28.0, 23 queries, none read as typing).
    sidecar: >-
      §3 → Keyboard test harness (key file, Tooling, the tui sentence): the closed non-editing list (F-W2) gains seven terminal reply shapes after CSI, by exact grammar, on the founder's ratification of 2026-10-08 (inputs#I17), measured on foot 1.28.0; proof is the classifier's unit table plus the reply probe on a real terminal. Chunk 2026-10-08-first-live-test-and-self-drive.
    rationale: >-
      Report Changes → Symbols / APIs (`is_reply`, private, `src/run/wheel.rs`): four new match arms add seven reply shapes to the closed list, `REPLIES` 15 → 23 strings; Coverage of new surfaces lists the classifier's closed list as a changed surface with a11y kbd coverage (unit: 8 reply cases, 27 negative controls; live: the reply probe on foot before and after). The key file enumerates the list's terminal replies as a closed set (DA1, DA2, CPR, DECRPM, kitty flags, OSC and DCS replies), dates the list to the 2026-10-04 ruling alone, and names the unit table as the replies' only proof — all three are now behind the report. The report's Expected amendments name this site. Sweep for the retired claim (the reply enumeration as complete): it occurs only in this key file. The other mentions in a11y-plan — the §1 tui boundary clause ("terminal replies (F-W2's closed list)"), §1 Critical path 4 focus order, §4 P4 and the §11 Keyboard ban — name the class by reference without enumerating it and stay true as written, so no dependent proposal is raised for them, §1 included although the report's Expected amendments mention it.
    basis: "src/run/wheel.rs:499, 506-511 (the new arms and the `first` binding), 474-477 (the doc comment), 528 (`REPLIES`), per the report's Symbols / APIs bullet and New text, by line"
  - detector: D-a11y-surface
    severity: warning
    section: "§3 → Keyboard test harness"
    change: >-
      In the Tooling label's tui sentence, after the boundary cases, add the live reading of the takeover clause on Linux (supplemental, never gating): on 2026-10-08 a key typed with `wtype` into a live `claude` session's own window, inside a nested compositor the test started itself with the desktop lock untouched, wrote the `wheel` record `{human, human-input}`; the window's focus wrote none; the next driver `send` exited 10 `human-typing` with one `hint:` line that does not name `release` and no `send-issued`. No stand-in for the key.
    sidecar: >-
      §3 → Keyboard test harness (key file, Tooling, the tui sentence): records the first live reading of the human-takeover clause on Linux, a `wtype` key in a compositor of the test's own, as supplemental evidence beside the three nextest boundary cases. Chunk 2026-10-08-first-live-test-and-self-drive.
    rationale: >-
      Report Coverage of new surfaces: "the takeover by a compositor-typed key (no code: a live reading of the existing `Observed::read` → `human_input` path)" with instrumentation by the existing `wheel` record, tests e2e-live and a11y kbd; Outcome → Affordance: met, with the readings quoted above; Decisions & corrections: the founder ruled the key is typed in a compositor the chunk starts itself, nested, one start, the desktop lock untouched (inputs#I12, inputs#I13). The key file records where each live reading of the tui keyboard boundary sits (it already carries one for a Windows mouse report) and is silent on this one (`compositor|wtype`: 0 lines in a11y-plan, per the report's own search). This is an omission, not a falsified sentence: no existing claim in the body is contradicted, so the orchestrator may treat it as the report's Expected amendment ("a11y-plan §3 Keyboard test harness, the live reading of the takeover clause on Linux, typed in a compositor of the test's own") rather than as a violated invariant. No other occurrence to sweep.
    basis: "report Outcome → (arch) Affordance; `evidence/key-probe-own.md`, `evidence/live-run.ndjson` as the report names them (no source line stated)"

# Detector notes (no proposal):
# D-a11y-surface — the report adds no new interactive UI element ("no new external surface; two changed ones"). The third changed surface, the `input-not-ready` hint text (`send_hint`, `src/human.rs` 216-218), needs no a11y-plan edit: the plan never quotes the hint wording (0 hits for "was not ready for input"), and its claims about it still hold per the report — `hint:` is one line, last on stderr, ASCII; exit code, refusal, detail and `--json` unchanged (§8 Error recovery, §8 Timeout extensions).
# D-a11y-obs-schema — no drift. Report Schema / config: none ("No event kind, snapshot field, diag-line field, config key or fixture changed"; `schemas`, `fixtures` unchanged under the preservation guard). The one obs-plan edit at this wrap is a form-only rewording in §9 CI Integration, not the log schema; the a11y violation row shape (§3 → Structured violation JSON schema) is untouched.
````
