# a11y · Cognitive Accessibility — category read

## Carried
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:19) Sanitising must keep this split: `unknown` the one hintless refusal, faults hintless and not refusals, and every `--json` error one document with no hint.
  > The one refusal with no hint is `unknown` (exit 14): its detail is opaque, so design prints no hint rather than a guess. Faults (`error: wrapper fault`, `error: internal error`) are not refusals and carry no hint. `--json` gives one document with no hint.
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:19) The `viola revive --json` refusal documents are CLI `--json` error bodies the enumeration must name, each in this code-and-null form with no hint and empty stderr.
  > under `--json` its strict-modes and preflight refusals give one `{"v":1,"error":"<code>","detail":null}` document with no hint and empty stderr (`revive_json_*` in `tests/cli_revive.rs`; chunk 2026-10-17-cli-machine-contract).
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:19) The server-verify refusal must stay exit 21 with this human pair, and its `--json` body must match this document exactly.
  > CLI `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` refuse an endpoint whose serving process is not the recorded wrapper with exit 21: `unable  <name>  instance-unreachable` on stderr, then `hint: the process on <name>'s endpoint is not its recorded wrapper; it was not contacted` as the last stderr line; under `--json` the `{"v":1,"error":"instance-unreachable","detail":null}` document with stderr empty and no hint.
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:25) The `link` / `unlink` strict-modes refusal must keep this exact `--json` document, with stderr empty and no hint.
  > a driven instance that fails the strict-modes check is refused with exit 21 before any log opens, no log line written: the fixed instance-state `unable` / `hint:` pair on stderr, `hint:` last; under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout with stderr empty and no hint.
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:31) The six frame verbs' strict-modes refusal must keep this exact `--json` document and the fixed human pair.
  > an instance that fails the strict-modes check is refused with exit 21 before any frame or log (`send` after its text read), no log line written: the same fixed instance-state `unable` / `hint:` pair as `viola link` / `viola unlink` on stderr, `hint:` last, stdout empty; under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout with stderr empty and no hint.
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:37) The enumeration must name `revive`'s `strict-modes-failed` document as a `--json` error body.
  > under `--json`, `revive` gives its `strict-modes-failed` document (above).
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:49) A `thiserror` message reaching the `cli` catch site must be a fixed fault line with no hint, since this path reaches it on Windows.
  > on Windows such a path stats `NotFound`, the check skips it as absent, and the `cli` catch site's fault line stands, with no hint.
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:61) Telling the `-32602` causes apart must leave this verb outcome as it stands: a fault, exit 20, no hint, and no client-side refusal.
  > a pair the wrapper refuses as invalid params (`-32602`, e.g. `driver` equal to `driven`) reaches the verb as a fault, exit 20, no hint; the verbs never refuse it client-side.
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:8) A surface's error body must carry this code pair for a late `answer`, with its exit code unchanged.
  > a later `answer` for it is refused `not-delivered` / `unknown-dialog` (exit 13)
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:8) The enumeration must carry the readiness-gate and wheel refusals as these codes.
  > and is then refused `not-delivered` / `input-not-ready` with nothing typed. That bound falls on the driver only as well: a key pressed during the wait goes to the child and takes the wheel at once, and the waiting send is then refused `human-typing`.
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:9) The enumeration must carry the unconfirmed-send refusal as this code pair.
  > a send not confirmed inside its window is `not-delivered` / `no-prompt-submitted`.
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:9) A code-shaped `detail` must hold for the budget refusal: `budget-paused` with `five-hour` or `seven-day`, exit 11.
  > an automated `viola send` is refused `budget-paused` with detail `five-hour` or `seven-day` (exit 11) and nothing is typed
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:9) The folded cap-driving case for the status-line command's stdout must read this outcome: exit 0 with empty stderr.
  > `viola hook statusline` runs the user's own status line command under `STATUSLINE_DEADLINE` (5 s, PROVISIONAL, `src/cmd/hook.rs`) and exits 0 with empty stderr in every case.
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:9) The folded cap-driving case for the `claude agents --json` drain must read this fail-open outcome on an oversize read.
  > `viola list` and `viola revive` each make one `claude agents --json` read (a direct spawn, stdout through `take(MAX_FRAME + 1)`, killed at 5 s; chunk 2026-10-16-unwrapped-sessions-on-the-board): a failed, oversize, malformed or timed-out read leaves `list`'s board as `board::read` gave it and passes `revive`'s preflight (fail open).
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:8) The folded cap-driving case for the dialog-hold reader must match these bounds and this fallback.
  > `dialog_hold_s`, whole seconds 60–3600, default 60, read once at the wrapper's start into `DialogHold`, its bounds `DIALOG_HOLD_MIN` / `DIALOG_HOLD_MAX`, each PROVISIONAL; any other value gives 60 and one `parse-rejected`
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:9) The folded cap-driving case for the thresholds reader must match these defaults and leave the gate unable to be switched off.
  > (`config.json` `five_hour`, default 90, and `seven_day`, default 85; no value of either switches the gate off)
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:68) The `viola list` and `viola plugin install` exit-21 refusals must still pass these literal pins, the fixed pair with its hint last and the exact `--json` document.
  > `viola list`'s exit-21 state pairs — the hint last, stdout empty; under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty — are pinned by literal in the `cfg(unix)` root cases `list_refuses_*` and the unit case `instance_and_home_state_pairs_are_the_fixed_texts`, chunk 2026-10-11-the-board-viola-list; `viola plugin install`'s exit-21 refusal, the same home pair, and its `--json` document, by `plugin_install_refuses_a_widened_home`, chunk 2026-10-17-optional-plugin-install;
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:68) The CLI surface's proof must hold this existing no-leak read of pid, home path and endpoint in both modes, and must not weaken it.
  > reads, over `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` in both modes, the hint as the last stderr line with no ESC byte and stdout empty, or under `--json` the exact document with stderr empty, and no output holding the recorded pid, the home's path or the endpoint,
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:68) The chunk must pass the typed exit-code and hint-free `--json` checks, and a new CLI witness must state its OS legs against this gating.
  > assert_cmd checks the typed exit codes (`10`–`14`, `20`, `21`) and that `--json` carries no hint text. These run on all three OS legs, except `tests/security_negatives_server_identity.rs`, gated `not(target_os = "macos")`, which runs on windows-2025 and ubuntu-latest only.
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:69) Carve-out: the macOS server-verify witness stays with its named owner and is no claim of this chunk.
  > the server-verify refusal cannot occur: no peer pid is read and the check reads as passed (the recorded gap), so its cases compile out there; its macOS witness is owed to "Unix endpoint and home hardening".
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:81) This strict-modes witness must stay green, with its exact `--json` document and pair literals unchanged unless changed on purpose.
  > reads, over `send`, `wait`, `last`, `answer`, `pause` and `release` in both modes against a live wrapper whose instance directory another user may write, exit 21 with the `INSTANCE_PAIR` literal on stderr and stdout empty, or under `--json` the exact `instance-unreachable` document with stderr empty, no line added and no frame sent, the widening left as found;
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:90) Any CLI text this chunk changes must have its pinned string updated on purpose in the same change.
  > Any copy change therefore fails CI until the pinned string is updated on purpose.
- (.andromeda/a11y-plan.md:31) Carve-out: `run`'s post-preflight start refusals under `viola revive --json` stay with "Exit-cause code catalogue", and this chunk must not take them.
  > **Owed:** under `viola revive --json`, a refusal `run`'s start makes after a passed preflight prints no document yet — owed to "Exit-cause code catalogue".
- (.andromeda/a11y-plan.md:37) The chunk's MCP and channel surfaces fall in this entity, so their witnesses are security proofs and no a11y assertion.
  > **Entity:** Claude Code plugin: hooks (`viola hook`, `hook statusline`) and the stdio MCP server; wrapper IPC channel
- (.andromeda/a11y-plan.md:40) The chunk must claim no a11y coverage for the MCP or channel error bodies, on this stated reason.
  > **Reason:** ipc-internal with no UI surface. Machine-to-machine traffic only.
- (.andromeda/a11y-plan.md:107) Sanitising the machine surfaces must leave the human refusal's reason and detail in text, as this stand-in requires.
  > Output-discipline assertions stand in for SC 1.3.2, 1.4.1 and 3.3.1: meaningful linear order, colour never the sole cue, and the refusal reason and detail given in text.
- (.andromeda/a11y-plan.md:257) The chunk must hold the error pattern on the CLI, its human lines and its hint-free `--json` alike.
  > **Pattern:** every error is a printed word followed by plain detail, and a recovery instruction where recovery exists; on the CLI, `unable  <reason>  <detail>` on stderr (`viola revive`'s exit-1 refusals and `viola verify`'s interrupted pair: one `unable: <text>` line), then `hint:` as the last stderr line, with a typed exit code; the one refusal with no hint is `unknown` (exit 14), whose detail is opaque; faults are not refusals and carry no hint; `--json` gives one document with no hint.
- (.andromeda/a11y-plan.md:261) The chunk must pass this verification, on these OS legs, for every refusal it touches.
  > **Verification:** trycmd pins the `unable <reason> <detail>` + final `hint:` layout per refusal and assert_cmd the typed exit codes and a hint-free `--json`, on all three OS legs (the frame verbs' server-verify refusal on windows-2025 and ubuntu-latest only: macOS reports no peer pid).
- (.andromeda/a11y-plan.md:271) A sanitised CLI failure must not reduce to a bare number or a trace as its only text.
  > The CLI never shows a stack trace or bare status number as the only error text.
- (.andromeda/a11y-plan.md:273) Every fixed CLI string the chunk adds or changes must be pinned by trycmd or by literal.
  > **Verification:** every fixed CLI string is pinned, by trycmd or, for `viola list`, `viola --help`, `viola plugin install`, the server-verify hint and the `viola link` / `viola unlink` result lines, by literal, so any copy change fails CI until the pinned string is updated on purpose.
- (.andromeda/a11y-plan.md:330) The proof must claim no WCAG conformance for the CLI; its witnesses stand in as output discipline only.
  > NEVER claim WCAG conformance for the cli or the `viola run` TUI (the wrapper-hosted TUI, whether `viola run` or a passed `viola revive` started it). assert_cmd / trycmd / portable-pty results are output-discipline stand-ins for SC 1.3.2 / 1.4.1 / 3.3.1, not a11y-tool evidence.
- (.andromeda/a11y-plan.md:345) A human-mode refusal the chunk touches must end with its `hint:`, `unknown` excepted.
  > NEVER show error without recovery suggestion (per SC 3.3.3). Every human-mode CLI refusal except `unknown` (exit 14, opaque detail, design cli) ends with a `hint:` line.
- (.andromeda/a11y-plan.md:346) A kebab-case code in human output must sit beside the word `unable`; the rule leaves the `--json` code-only form to its own pattern.
  > NEVER use jargon without a plain alternative. SC 3.1.5 AAA is not claimed, but a kebab-case reason/detail code (`not-delivered`, `human-typing`, `unverified-cli`) never appears without the plain word `unable`.
- (.andromeda/a11y-plan.md:347) A `thiserror` message or anyhow chain must not render as a stack trace standing as the only CLI error text.
  > NEVER render a stack trace as the only error text in the CLI.
- (.andromeda/a11y-plan.md:359) The per-surface witnesses and the enumeration must run inside the existing harness, with no new suite value or driver.
  > NEVER add a `suite` value (such as `a11y`) or a separate a11y driver outside `scripts/agent-run.* run`.

## Bindings
- **`--json` error document ↔ body §8 Error recovery**: the detail's exact documents and the body's pattern state one rule, a hint-free `--json` document under a typed exit, (.andromeda/a11y-plan-detail/cognitive-accessibility.md:19) and (.andromeda/a11y-plan.md:257).
- **`revive` exit-1 fixed-message form ↔ design-system cli pattern 2**: `viola revive`'s human refusals follow the design pattern the detail names, so a copy change binds both, (.andromeda/a11y-plan-detail/cognitive-accessibility.md:19).
- **`-32602` as a fault ↔ obs-plan §5**: the verb's fault outcome stands here while the `viola.compat.v_rejected` count named in the scope's carry is read against obs-plan, (.andromeda/a11y-plan-detail/cognitive-accessibility.md:61).
- **Cap-driving cases ↔ test-plan §4 and §6**: the scope names test-plan the owner of the owed cases, and this file holds the outcomes they must read, (.andromeda/a11y-plan-detail/cognitive-accessibility.md:9) and (.andromeda/a11y-plan-detail/cognitive-accessibility.md:8).
- **Owed revive start refusals ↔ route entry "Exit-cause code catalogue"**: the body's owed line and the scope's boundary both leave them to that entry, (.andromeda/a11y-plan.md:31).
- **macOS server-verify witness ↔ route entry "Unix endpoint and home hardening"**: the detail's owed line and the scope's boundary both leave it to that entry, (.andromeda/a11y-plan-detail/cognitive-accessibility.md:69).
- **Fixed-string pins ↔ tests' harness contract**: pinned literals fail in the existing suites, with no new suite value, (.andromeda/a11y-plan-detail/cognitive-accessibility.md:90) and (.andromeda/a11y-plan.md:359).
