# security extract

## Relevance
relevant: the chunk changes the dialog-answering gate (a stamp is what switches on a non-`null` decision, including PermissionRequest `allow`), records content-bearing dialog payloads into committed fixtures, and its W3c card may widen the `--capture` arm or the no-key PTY probe boundary.

## Constraints
- **R2's dated gap closes in the plan text.** Security-plan §Security Anti-Patterns › Universal (the stamp-gate ban and its dated gap) requires that a non-`null` `hook.dialog` decision has a `viola verify` stamp for the child's CLI version. Today the dated gap lets the ten-row stamp carry it. Once this entry lands, the requirement becomes that the dialog-tier rows (S3 / S7 / S8 / dialog concurrency) pass in that version's stamp. The gap text and its residual are a wrap amendment.
  - Two questions are research's: whether the wrapper gate reads row-level pass/fail at all today, or only stamp presence; and which keying P4 picks (all four rows, or per kind).
  - The decision must still come only through `read_stamps_strict` (the next Universal ban), and a refusal must still read `cli_verified:false`.
- **`ledger/stamps.json` keeps a single writer.** Per §Data Protection › At rest (code-bearing artefacts) and §Anti-Patterns › Universal, only `viola verify` writes it, under its `.lock`, at 0600.
  - The `/10` → `/14` growth and the re-stamps of 2.1.288 and 2.1.287 go through verify alone.
  - The `update_stamps` read stays inside its dated stamps-read exception (§Input Validation › CLI arguments / stdin, Own state files on read). It must not widen.
- **The re-probe stays inside the founder's interactive-probe boundary as ratified.** Per §Threat Model Summary › Attack surface (child process spawning and PATH resolution) and §Data Protection › At rest (interactive probe dirs):
  - The rules: direct spawns under the R8 strip; output through `MAX_FRAME` into a `Screen` fed under `catch_unwind`; Run B only in `<cwd>/.viola-verify-<pid>/`, 0700 and removed on every exit path; never a byte into a CLI-native dialog (trust, external imports); a modal start is killed with no key; viola writes nothing under `~/.claude`.
  - Both W3c mechanisms that make "the decision take effect" cross this boundary as worded:
    - a probe hook that answers. Today the capture arm emits empty stdout (§Input Validation › Hook stdin, capture arm; §Error Handling › Internal logging);
    - the probe keying a rendered dialog.
  - Either one is a widening that needs the founder's ruling plus a Decisions Log entry before it is planned (§Anti-Patterns › Universal, the "new channel method / listener" rule by analogy; the capture arm was itself a ratified widening).
  - Whether `hook --capture` at HEAD still writes nothing to stdout and always exits 0 is research's question.
- **Recorded dialog payloads go through the `--record` scrub/refusal before any write.** Per §Data Protection › Repository fixtures:
  - The scrub covers every string and key, `tool_input` included (it can carry plan text, Run B's absolute cwd and file bodies).
  - It maps the home prefix to `~` and the user word to `<user>`, and it refuses the whole recording when a drive path, `/home/`, `/Users/`, `\Users\`, the user word or an email-shaped token survives.
  - The refusal names only the fixture file and a closed code (`home-path` · `absolute-path` · `username` · `email`).
  - Probe prompts must be synthetic and compiled, never runtime or upstream text (§Anti-Patterns › Data Protection, the fixture-commit ban; § Code Patterns, the compiled-literals ban).
  - Whether the current scrub walks nested `tool_input` objects and keys is research's question.
- **Probe captures stay transient and private.** Per §Data Protection › At rest (probe captures): the dialog-hook captures (`PreToolUse.<k>.json`, `PermissionRequest.<k>.json`) land under `ledger/probes/<pid>/` (0700 dir, 0600 files through `replace_private`). A drop guard removes them on every exit path of `verify`, and only a scrubbed `--record` copy leaves.
  - Hook stdin for the dialog events stays under `take(MAX_FRAME + 1)` (§Input Validation › Hook stdin; Constants).
  - The largest-hook-payload row's per-kind maximum now also covers the dialog events, and a value within 4× of `MAX_FRAME` is the revisit signal (§Input Validation › Constants).
- **Any decision the re-probe or the `permission` E2E case sends goes through the closed enums and the paste-text validator.** Per §Input Validation › Dialog free text and Channel frames (`hook.dialog` and `answer` params):
  - `behavior` is a closed enum, `dialog_id` is a `u64`, and `validate_paste_text` runs on `message` and `answers.*` on both sides.
  - The `hook.dialog` / `answer` frames stay inside their sixth dated pre-check gap, with no new borrowing (§Anti-Patterns › Authentication, frame-before-verification ban; scope Boundaries).
- **Hook fail-open is preserved on every dialog path the probe exercises.** Per §Error Handling › Internal logging and §Anti-Patterns › Logging, `viola hook` (event or capture arm) never writes stderr and never exits non-zero, including when a capture write or parse fails.

## Patterns to follow
- The capture-arm discipline: stdin raw and unparsed through `take(MAX_FRAME + 1)`, written to the first free `<Event>.<k>.json` via `replace_private`, no `VIOLA_*` read, no obs init, no channel, silent exit 0 (§Input Validation › Hook stdin, capture arm). A widened arm keeps everything here except what the ruling changes.
- Drop-guard cleanup of both probe dirs and `ledger/probes/<pid>/` on every exit path, passing or failing (§Data Protection › At rest).
- The named `--record` refusal (file + closed code, never content) and the screen-fixture rule that keeps only signature rows (§Data Protection › Repository fixtures). Dialog payload fixtures reuse the same scrub/`is_clean` path.
- The version gate reads stamps through `read_stamps_strict`: no lock, `take(MAX_FRAME + 1)`, a refusal or malformed file is one `parse-rejected{parser:"ledger-stamps"}` WARN and `cli_verified:false` (§Input Validation › Own state files on read).
- Closed-enum mapping of dialog kind and decision body in `viola_agent_claude::dialog::classify`, with a malformed dialog payload failing open (§Input Validation › Hook stdin).

## Anti-patterns to avoid
- NEVER answer a `hook.dialog` with a non-`null` decision on a stamp whose dialog rows did not pass for the child's version once this entry lands. Also NEVER let any process but `viola verify` write `ledger/stamps.json` (§Anti-Patterns › Universal).
- NEVER type a byte into a CLI-native dialog or answer one from the probe hook without the founder's ruling on the M7 widening and its Decisions Log entry. NEVER build probe prompts or screen signatures from runtime or upstream text (§Threat Model Summary › Attack surface, child process spawning; §Anti-Patterns › Code Patterns).
- NEVER commit a dialog fixture that holds a home path, username, email-shaped token or non-synthetic content. NEVER let the refusal message quote the offending content (§Anti-Patterns › Data Protection; §Data Protection › Repository fixtures).

## Contract bindings
- **security ↔ arch** (architecture [CLI Version Compatibility]): the dialog-tier ledger rows and the stamp shape (`/14`) define what "verified" means for the decision gate. CLAUDE.md's and architecture's matching ten-row dated exceptions close together with security-plan's §Anti-Patterns › Universal residual (wrap amendments, one fact in three masters).
- **security ↔ tests** (test-plan §6 Path 4, §10; `verification-matrix.json#v1-15`, `#v1-30`):
  - CI has no Claude credential (§Threat Model Summary › Attack surface, supply chain: real-CLI probe and `--record` local only), so CI stamps the new rows only against the fake agent replaying the scrubbed fixtures.
  - The CI artifact canary scan (§Bootstrap phases, `secret-scanning-ci-gate`) covers the new fixtures.
- **security ↔ obs** (obs-plan §6): the NEVER-log floor (§Bootstrap phases, `logging-redaction-wire`) keeps dialog payload content, plan text and tool `input` out of every log, stderr and verify output line, in `diagnostics/detail-*` only. Verify's per-row failure output carries codes, not payload text (§Error Handling › External responses).
- **security ↔ operator/founder**:
  - The Run B accepted residual (§Data Protection, Accepted risk: verify's trusted run) grows by every new live re-probe session's transcript under `~/.claude/projects/`. The plan should restate the new count against the accepted class.
  - A PermissionRequest `allow` in a live re-probe runs a real tool as the user, so its tool and arguments must be synthetic and confined to Run B's dir.

## Acceptance criteria contributions
- The wrapper's dialog gate refuses a non-`null` decision (the human gets the `null` / `unverified-cli` path) when the child version's stamp lacks a passing dialog row, and allows it when the rows pass. Tested with a ten-row-only stamp and with a full `/14` stamp (per security-plan §Security Anti-Patterns › Universal, the stamp-gate ban).
- `verify --record` over a dialog payload whose `tool_input` holds the home path and the username writes the scrubbed form or refuses with `unable: a recorded fixture is not clean: <file> <code>` and writes nothing. Every committed `fixtures/claude/<version>/` dialog file passes `is_clean` (per security-plan §Data Protection › Repository fixtures).
- After a `viola verify` re-probe run, passing or failing (and on an injected mid-run failure), neither `ledger/probes/<pid>/` nor `<cwd>/.viola-verify-<pid>/` nor Run A's temp dir remains, and nothing new appears under `~/.claude` other than the CLI's own transcript residual (per security-plan §Data Protection › At rest, probe captures and interactive probe dirs).
- `viola hook <PreToolUse|PermissionRequest> --capture <DIR>` exits 0 with empty stderr on every input, oversize and bad `<DIR>` included. Its stdout stays empty unless the founder's M7 ruling, recorded in the Decisions Log, widens it to an answering body (per security-plan §Input Validation › Hook stdin, capture arm; §Error Handling › Internal logging).
