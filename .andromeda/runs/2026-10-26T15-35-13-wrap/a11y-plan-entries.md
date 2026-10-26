## 2026-10-26-explicit-session-links — the eight CLI frame verbs' server-verify refusal
**Section:** §1 A11y Scope Summary (a11y-scope-summary.md: the CLI verbs entity); §8 Cognitive Accessibility › Error recovery (cognitive-accessibility.md: the cognitive-errors CLI item and its cli verification item)
**Change:**
- The CLI verbs entity's server-verify sentence: was `send`, `wait`, `last`, `answer`, `pause` and `release` (per 2026-10-26-server-verification-before-any-frame — the six CLI frame verbs' server-verify refusal); now `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` each refuse, before any frame, an endpoint whose serving process is not the recorded wrapper, the rest of the sentence unchanged.
- The cognitive-errors CLI item: the same eight verbs now refuse it with exit 21, the `unable  <name>  instance-unreachable` line and the server-verify `hint:` last, or the `--json` document with stderr empty and no hint.
- The cli verification item: was "the six frame verbs' exit-21 server-verify refusal", read over the six verbs (per 2026-10-26-server-verification-before-any-frame — the server-verify refusal's literal pin and its two-leg reach); now "the eight frame verbs'", `security_negative_cli_verb_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper` reading `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` in both modes; its macOS gating unchanged.
**Why:** the chunk made the CLI frame verbs eight: `link` and `unlink` take the liveness pre-check and the server check through `cmd::client::connect` before their frame, and a server that is not the recorded wrapper gets no frame, exit 21 `instance-unreachable` with the server-verify hint; the identity table in `tests/security_negatives_server_identity.rs` grew to eight verbs in both modes plus the controls.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the link verbs' entity and the link line's escaped since
**Section:** §1 A11y Scope Summary (a11y-scope-summary.md: a new scope-entities group after the CLI verbs entity; a new scope-surfaces group before the tui surface)
**Change:**
- New entity: `viola link <driver> <driven> [--json]` and `viola unlink <driver> <driven> [--json]`, the verb's instance the driven one, in the human TTY, plain and `--json` modes: each prints one plain-ASCII result line in one write with no SGR in any mode, or under `--json` one `{"v":1,"ok":<ok>}` document; each refuses with exit 21 a driven instance that fails the strict-modes check (before any log opens), a stopped driven instance and a server that is not the recorded wrapper; a pair the wrapper refuses as invalid params reaches it as a fault, exit 20, no hint. Source: `src/cmd/link.rs` (`link_call`), `src/cmd/unlink.rs`, `src/human/verbs.rs` (`write_linked`, `write_already_linked`, `write_unlinked`); `tests/cli_links.rs`. Assertability: assertable, output-discipline assertions on text and streams only.
- New cli surface detail: a `viola link` line whose `since` is no RFC 3339 instant (the restore's `unknown`, or a value from a hand-altered log) prints it through `escape_message`, which keeps `\n`, `\t` and non-ASCII; the wrapper's own `since` is a link line's `ts` or `unknown`, so such a byte reaches the line only from a hand-altered log.
**Why:** the chunk built the two verbs and the wrapper's `link` / `unlink` methods, in the form the founder named himself in his own answer to this chunk's P4 fork, live by dialog, relayed verbatim by the operator (inputs#I3): the instance check first, then the liveness pre-check and the server check like `send`; `link_since` prints a `since` that is no RFC 3339 instant through `escape_message`.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the `handoff:` help row built
**Section:** §1 A11y Scope Summary (a11y-plan.md: critical path 6, Required ARIA roles; a11y-scope-summary.md: path-p6-board); §4 ARIA Patterns & Roles (aria-patterns-roles.md: the cli landmark row)
**Change:**
- a11y-plan.md path 6: was "the `--help` group labels, whose `handoff:` row is owed to "Explicit session links""; now "the `--help` group labels", the owed clause gone.
- path-p6-board: was "board / traffic / wheel / setup over the served verbs; the `handoff:` row joins with "Explicit session links""; now board / traffic / wheel / handoff / setup over the served verbs, the `handoff:` row `handoff:  link, unlink` between `wheel:` and `setup:`.
- The cli landmark row: was the rows `board:` · `traffic:` · `wheel:` · `setup:`, the `handoff:` row owned by "Explicit session links" (per 2026-10-17-cli-output-discipline — the colour spans built and checked; --help group labels built, and 2026-10-17-optional-plugin-install — plugin install built: setup: row, verb entity, pinned lines); now `board:` · `traffic:` · `wheel:` · `handoff:` · `setup:`, the `handoff:` row `handoff:  link, unlink` between `wheel:` and `setup:`, held by `help_is_the_grouped_verb_table` and `help_lists_only_served_verbs`.
**Why:** `viola --help` gained the row `handoff:  link, unlink` between `wheel:` and `setup:`, met by the two help cases; the chunk retired the hold on the explicit verbs and every master line naming "Explicit session links" as the row's pending owner.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the link verbs' refusals in Error recovery
**Section:** §8 Cognitive Accessibility › Error recovery (cognitive-accessibility.md: three new cognitive-errors values groups and a new cognitive-errors procedure group)
**Change:**
- Strict-modes: a driven instance that fails the strict-modes check is refused with exit 21 before any log opens, no log line written: the fixed instance-state `unable` / `hint:` pair on stderr, `hint:` last; under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout with stderr empty and no hint.
- Stopped instance: a driven instance with no live endpoint is refused with exit 21, the not-running `hint:` as the last stderr line.
- Wrapper refusal: a pair the wrapper refuses as invalid params (`-32602`, e.g. `driver` equal to `driven`) reaches the verb as a fault, exit 20, no hint; the verbs never refuse it client-side.
- Verification, in `tests/cli_links.rs`: `link_verbs_against_a_stopped_instance_exit_21` reads the stopped-instance refusal, and `link_verbs_refuse_a_widened_instance_before_any_log` the strict-modes refusal (a group-writable driven instance refused before any log, its mode read back as found; `cfg(unix)`, so ubuntu-latest and macos-latest only).
**Why:** `link_call` runs `check_instance` before `client::start`, then the liveness pre-check through `client::unreachable`, and sends a wrapper error, `-32602` included, through `client::fault`; the check-first order is the founder's own answer to this chunk's P4 fork, live by dialog, relayed verbatim by the operator (inputs#I3).
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the link verbs' result lines among the literal-pinned strings
**Section:** §8 Cognitive Accessibility › Plain language (cognitive-accessibility.md: two new cognitive-plain-language procedure groups; a11y-plan.md: the Plain language Verification line)
**Change:**
- New verification: the three result lines (`write_linked`, `write_already_linked`, `write_unlinked`; their text in design-system §Surface: cli), each one write with no SGR in any mode, pinned by literal in the unit `link_verb_lines_are_the_design_lines_in_one_write` (`src/human/verbs.rs`) and the root case `link_verbs_print_the_design_lines` (`tests/cli_links.rs`).
- New: the `link` line's time is the reply's `since` as UTC clock time in whole seconds with `Z`, pinned by the unit `link_since_is_the_utc_time_alone`; the names each line prints are the verb's own parsed arguments, never names read back from the reply or the log.
- a11y-plan.md: was "by literal" for `viola list`, `viola --help`, `viola plugin install` and the server-verify hint (per 2026-10-26-server-verification-before-any-frame — the server-verify hint among the literal-pinned strings); now for those and the `viola link` / `viola unlink` result lines.
**Why:** the chunk added three fixed result lines, each one `write_all` and plain in every mode, and pinned them by literal with no trycmd case, so the named exceptions to trycmd needed them added.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/
