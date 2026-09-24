# design extract

## Relevance
Partial. The chunk has no web-spa or token work, but it touches the cli surface's error and stderr phraseology: the fixed `Display` messages, the `main` catch site that turns an error into exit 1, and routing content to `diagnostics/` only.

## Constraints
- design-system §Surface: cli → Exit-code phraseology (exit 1 row) requires the only human stderr text for an internal fault at the `main` catch site to be the fixed message `error: internal error`. It carries no chain with serde sources, no paths and no hint. Under `--json`, exit 1 has no document until arch amends it. Research must answer whether the current `Err(_)` → exit 1 path prints anything at all, and what it prints.
- design-system §Surface: cli → Platform-Specific Notes ("Stack traces") requires that stack traces never print. Errors are fixed messages, and full detail goes only to `instances/<name>/diagnostics/`. This matches the chunk's rule that content-bearing records go only to instance detail files.
- design-system §Surface: cli → Streams requires `error: …` lines, refusals and `hint:` lines on stderr, and results on stdout. Under `--json`, errors are one JSON document on stdout with the typed exit code and no `hint:` line. Redacting error displays must not move text between streams.
- design-system §Surface: cli → Component Patterns 2 (the send hints list) requires hints never to quote the sent text or any upstream text. The exit-1 `viola run` start refusals are fixed-message lines, and none shows a path or a pid. `error: wrapper fault` (exit 20) and `error: internal error` (exit 1) get no hint, because "their detail goes only to `diagnostics/`".
- design-system §Surface: cli → Component Patterns 5 (`ui`) requires the token, the URL and the `.url` path never to print on any line from any verb except the single unstyled `viola ui` launch line. This is the cli-side face of the chunk's NEVER-log floor (GUI token, launch URL/`.url`).
- design-system §Surface: cli → Platform-Specific Notes requires `hook` to write nothing to stderr and `mcp` to write only MCP frames on stdout. Neither may gain a human-facing error line when chains are routed to the detail sink.
- design-system §Surface: cli → Component Patterns 1 requires fields of `claude agents --json` origin to have C0/C1 controls escaped as `\x1B`-style hex text in human output, and `\n` / `\t` escaped too in `list` rows. Whether any error display interpolates such fields is research's question.

## Patterns to follow
- Fixed-phrase error vocabulary per §Surface: cli → Exit-code phraseology: `error: internal error`, `error: wrapper fault  <code>`, and `unable  <reason>  <detail>` with two-space separators. The fixed `Display` strings on `<Crate>Error` should land on these words rather than new prose.
- The per-cause fixed-message form of the `viola run` start refusals (§Surface: cli → Component Patterns 2, e.g. `unable: the viola home is not private to you`). It is the model for a fixed `Display` with no path or pid when a `ConfigRejection::Unreadable`-type error reaches a human.
- Detail codes, not prose, in the machine view (§Surface: cli → Component Patterns 2, the "Hints are human-mode only" paragraph). It points at obs-plan D-20's log detail codes (`instance-dead`, `strict-modes-failed`, …) as the typed carriers.

## Anti-patterns to avoid
- Printing upstream text, paths, pids or anyhow chains with serde sources in errors or hints (per design-system §Anti-Patterns → Per-Surface Bans → cli; the plan calls this "the security plan's NEVER-log floor").
- Printing stack traces (per §Anti-Patterns → Per-Surface Bans → cli, "NEVER print stack traces").
- Printing the token or launch URL anywhere but the single `viola ui` launch line (per §Anti-Patterns → Per-Surface Bans → cli).

## Contract bindings
- design ↔ security: the cli bans cite security-plan §logging-redaction-wire / §error-sanitization-wire as the source of the NEVER-log floor. Design owns the visible fixed wording, and security owns what may never appear.
- design ↔ obs: obs-plan D-20 detail codes are the machine-side carrier for causes that the human line states as fixed phrases (§Surface: cli → Component Patterns 2). Full detail goes to the obs detail sink (`diagnostics/`).
- design ↔ architecture: the exit codes and `--json` shapes (`{"v":1,"error":…}`) are architecture's. Design adds no `--json` document for exit 1 pending an arch amendment (§Surface: cli → Exit-code phraseology).
- design ↔ tests: the plan's Design Decisions Log (T4 entry) notes that tests' exit-cause matrix does not list `error: internal error` among the exit-1 causes.

## Acceptance criteria contributions
- A forced internal error reaching the `main` catch site prints exactly `error: internal error` on stderr: no chain text, no path, no serde source, no `hint:` line. The exit code is 1 (per design-system §Surface: cli → Exit-code phraseology).
- No stack trace or anyhow chain appears on stdout or stderr for any error path at HEAD. The full chain appears only in `instances/<name>/diagnostics/` when an instance resolves (per design-system §Surface: cli → Platform-Specific Notes, "Stack traces").
- No stderr, stdout or `--json` output of any verb at HEAD contains the GUI token, a launch URL, `?t=`, or the `.url` path, other than the single `viola ui` launch line (per design-system §Surface: cli → Component Patterns 5).
- `viola hook` still writes nothing to stderr, even on an internal error routed to the detail sink (per design-system §Surface: cli → Platform-Specific Notes).

## Relevant amendment history
(none). The sidecar `D:/dev/projects/viola/.andromeda/design-system-amendments.md` does not exist yet, which is normal on a fresh project. The in-plan Design Decisions Log has two related 2026-09-24 entries: "Exit-1 start refusal hint" and T4. Both keep `error: internal error` hintless because it is a fault whose detail goes only to `diagnostics/`.
