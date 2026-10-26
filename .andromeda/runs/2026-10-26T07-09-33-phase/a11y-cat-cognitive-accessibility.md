# a11y · Cognitive Accessibility — category read

## Carried
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:19) The CLI verbs' refusal on a failed or impossible server verification (send, wait, last, answer, pause, release; exit 21, instance unreachable) must keep the refusal layout: the word `unable`, reason and detail on stderr, a final `hint:` line, a typed exit code, and under `--json` one document with no hint.
  > **CLI:** `unable  <reason>  <detail>` on stderr, then `hint:` as the last stderr line, with a typed exit code. The one refusal with no hint is `unknown` (exit 14): its detail is opaque, so design prints no hint rather than a guess. Faults (`error: wrapper fault`, `error: internal error`) are not refusals and carry no hint. `--json` gives one document with no hint.
- (.andromeda/a11y-plan.md:257) A verification mismatch is a refusal with a recovery hint, never a fault form: a check that cannot be made must surface as the instance-unreachable refusal (with its `hint:`), not as `error: internal error` or `error: wrapper fault`, which carry no hint.
  > - **Pattern:** every error is a printed word followed by plain detail, and a recovery instruction where recovery exists; on the CLI, `unable  <reason>  <detail>` on stderr (`viola revive`'s exit-1 refusals and `viola verify`'s interrupted pair: one `unable: <text>` line), then `hint:` as the last stderr line, with a typed exit code; the one refusal with no hint is `unknown` (exit 14), whose detail is opaque; faults are not refusals and carry no hint; `--json` gives one document with no hint.
- (.andromeda/a11y-plan.md:259) The new refusal path is held to these two criteria (as output-discipline stand-ins, no conformance claim).
  > - **WCAG SC:** SC 3.3.1 Error Identification (A) + SC 3.3.3 Error Suggestion (AA).
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:26) Each verb's exit-21 verification refusal must be pinned: the `unable … hint:` layout by trycmd or literal, the typed exit code 21 by assert_cmd, and its `--json` form shown to carry no hint text, on all three OS legs.
  > - **cli:** trycmd cases pin the `unable <reason> <detail>` + final `hint:` layout per refusal
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:26) The exit-code set the CI checks includes 21, and the `--json` hint-free assertion covers the changed verbs.
  > and assert_cmd checks the typed exit codes (`10`–`14`, `20`, `21`) and that `--json` carries no hint text. These run on all three OS legs.
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:26) The instance-unreachable `--json` document shape already pinned for `viola list` is the shape the changed verbs' unreachable document must stay consistent with (stdout one document, stderr empty).
  > under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:35) If the chunk changes any fixed refusal or hint string of these verbs, the pinned string must be updated on purpose in the same change, or CI fails.
  > trycmd pins every fixed CLI string, except `viola list`'s, which literals pin
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:35) The pin is the gate for copy drift.
  > Any copy change therefore fails CI until the pinned string is updated on purpose.
- (.andromeda/a11y-plan.md:270) Any reason or detail code the verification refusal prints (`instance-unreachable` or a new one) must sit beside the plain word `unable`.
  > - Every kebab-case reason/detail code (`not-delivered`, `human-typing`, `unverified-cli`) always sits beside the plain word `unable`.
- (.andromeda/a11y-plan.md:271) A verification failure (a sysinfo or peer-credential read error included) must never reach the human as a stack trace or a bare status number alone.
  > - The CLI never shows a stack trace or bare status number as the only error text.
- (.andromeda/a11y-plan.md:345) Every human-mode refusal the chunk adds or reroutes ends with a `hint:` line.
  > - NEVER show error without recovery suggestion (per SC 3.3.3). Every human-mode CLI refusal except `unknown` (exit 14, opaque detail, design cli) ends with a `hint:` line.
- (.andromeda/a11y-plan.md:346) Ban on a code shown without its plain word, applied to the verification refusal.
  > a kebab-case reason/detail code (`not-delivered`, `human-typing`, `unverified-cli`) never appears without the plain word `unable`.
- (.andromeda/a11y-plan.md:347) Ban on a stack trace as the only error text, applied to the verification path.
  > - NEVER render a stack trace as the only error text in the CLI.
- (.andromeda/a11y-plan.md:251) The verification adds no time bound that falls on the human: a check made before the first frame bounds only the driver's verb, and no key is blocked, refused or delayed by it.
  > every time bound viola holds — the `viola wait` timeout the caller passes, the MCP `wait` cap, the dialog hold, the `viola send` readiness gate, the confirmation windows, the status-line deadline, the `claude agents --json` read, the budget pause — falls on the driver, the user's own command or viola's own child, never on the human: no key is blocked, refused or delayed by it.
- (.andromeda/a11y-plan.md:344) No harsh timeout without warning is introduced by the check.
  > - NEVER set harsh timeouts without warning (no warn-extend pattern per SC 2.2.1). v1 has none.
- (.andromeda/a11y-plan.md:29) The changed CLI verbs are a11y entities in human TTY, plain and `--json` modes: their new refusal is held by output-discipline assertions only.
  > - **Entity:** CLI verbs (`list`, `send`, `wait`, `last`, `answer`, `verify`, `pause`, `release`, `link`, `unlink`, `revive`, `plugin install`, `--help`) in human TTY, plain (non-TTY / `NO_COLOR` / `TERM=dumb`) and `--json` modes — assertable: output-discipline assertions on text and streams only; no automated a11y tool reaches a terminal.
- (.andromeda/a11y-plan.md:37) The hook client's fail-open and the MCP tools' `instance-unreachable` result fall on a not-assertable surface: no a11y assertion is owed for them.
  > - **Entity:** Claude Code plugin: hooks (`viola hook`, `hook statusline`) and the stdio MCP server; wrapper IPC channel
- (.andromeda/a11y-plan.md:40) Reason the hook and MCP paths carry no a11y assertion.
  > - **Reason:** ipc-internal with no UI surface. Machine-to-machine traffic only.

## Bindings
- **Error recovery refusal layout ↔ design-system cli pattern / security.md §Error handling**: the exit-21 verification refusal takes design's `unable` + `hint:` form and the security plan's fixed-message, code-only external error (no path, pid or start time in the text or `--json` document) — (.andromeda/a11y-plan-detail/cognitive-accessibility.md:19)
- **Error recovery verification ↔ test-plan §3 / §9 harness**: the trycmd / assert_cmd cases for exit 21 and the hint-free `--json` run inside the per-OS `coverage` suite, no new suite value — (.andromeda/a11y-plan-detail/cognitive-accessibility.md:26)
- **Plain language pins ↔ design-system fixed copy**: a changed refusal or hint string is a design copy change, re-pinned on purpose — (.andromeda/a11y-plan-detail/cognitive-accessibility.md:35)
- **Timeout bounds ↔ the universal invariant "the human always wins"**: the pre-frame check bounds the driver's verb only; the hook's fail-open (exit 0, no body) leaves the human path untouched — (.andromeda/a11y-plan.md:251)
- **Not-assertable plugin/IPC surface ↔ security plan (hook fail-open, MCP `instance-unreachable`)**: those outcomes are governed by security and tests, not a11y — (.andromeda/a11y-plan.md:37)
