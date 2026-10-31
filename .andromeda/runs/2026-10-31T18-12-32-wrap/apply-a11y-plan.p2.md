# Apply — a11y-plan · part 2 of 2

## Verdicts
- fanout-a11y-plan--cascade-g2.md 1 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:19#1
- fanout-a11y-plan--cascade-g2.md 2 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:19#2
- fanout-a11y-plan--cascade-g2.md 3 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:21+#1
- fanout-a11y-plan--cascade-g2.md 4 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:25#1
- fanout-a11y-plan--cascade-g2.md 5 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:31#1
- fanout-a11y-plan--cascade-g2.md 6 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:33+#1
- fanout-a11y-plan--cascade-g2.md 7 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:39+#1
- fanout-a11y-plan--cascade-g2.md 8 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:49#1
- fanout-a11y-plan--cascade-g2.md 9 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:55#1
- fanout-a11y-plan--cascade-g2.md 10 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:57+#1
- fanout-a11y-plan--cascade-g2.md 11 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:58+#1
- fanout-a11y-plan--cascade-g2.md 12 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:81+#1
- fanout-a11y-plan--cascade-g2.md 13 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:92#1
- fanout-a11y-plan--cascade-g2.md 14 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:105#1
- fanout-a11y-plan--cascade-g2.md 15 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:119+#1
- fanout-a11y-plan--cascade-g2.md 16 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:120+#1
- fanout-a11y-plan--cascade-g2.md 17 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:133+#1

## Edits
- line: .andromeda/a11y-plan-detail/cognitive-accessibility.md:19
  old: «under `--json` its strict-modes and preflight refusals give one `{"v":1,"error":"<code>","detail":null}` document with no hint and empty stderr (`revive_json_*` in `tests/cli_revive.rs`; chunk»
  new: «under `--json` every refusal of the start it shares with `run` gives one `{"v":1,"error":"<code>","detail":null}` document with no hint and empty stderr (`revive_json_*` in `tests/cli_revive.rs`, the start rows in `tests/cross_exit_causes.rs`; chunk»
- line: .andromeda/a11y-plan-detail/cognitive-accessibility.md:19
  old: «under `--json` the `{"v":1,"error":"instance-unreachable","detail":null}` document with stderr empty and no hint.»
  new: «under `--json` the `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}` document with stderr empty and no hint.»
- after: .andromeda/a11y-plan-detail/cognitive-accessibility.md:21
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · cognitive-errors -->»
  add: «»
  add: «- **CLI, exit 21 under `--json`:** one document per cause, `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` on stdout with stderr empty, no hint and no ESC, `<cause>` being one of the closed words `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost` and `server-verify-failed`.»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/a11y-plan-detail/cognitive-accessibility.md:25
  old: «on stderr, `hint:` last; under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`»
  new: «on stderr, `hint:` last; under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`»
- line: .andromeda/a11y-plan-detail/cognitive-accessibility.md:31
  old: «`hint:` last, stdout empty; under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`»
  new: «`hint:` last, stdout empty; under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`»
- after: .andromeda/a11y-plan-detail/cognitive-accessibility.md:33
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · cognitive-errors -->»
  add: «»
  add: «- **CLI, a lost or stale wrapper:** a wrapper lost with the call open is `endpoint-lost`; a frame verb against a stale instance reads `instance-dead`, stale heartbeat being exit 1's cause alone.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/a11y-plan-detail/cognitive-accessibility.md:39
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · cognitive-errors -->»
  add: «»
  add: «- **CLI, a start against a holder:** exit 1 tells a stale holder from a live one: `stale-heartbeat` for a `stale` holder, `already-live` for a live holder and a fresh replayed beat.»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/a11y-plan-detail/cognitive-accessibility.md:49
  old: «the `cli` catch site's fault line stands, with no hint.»
  new: «the `cli` catch site's fault line stands, with no hint; under `--json` a frame verb there prints the `internal-error` document on stdout in that line's place, stderr empty (`verify` takes no `--json`).»
- line: .andromeda/a11y-plan-detail/cognitive-accessibility.md:55
  old: «a driven instance with no live endpoint is refused with exit 21, the not-running `hint:` as the last stderr line.»
  new: «a driven instance with no live endpoint whose instance directory stands is refused with exit 21, the not-running `hint:` as the last stderr line; a driven name with no live endpoint and no such directory reads `unwrapped-name`, its own `hint:` last.»
- after: .andromeda/a11y-plan-detail/cognitive-accessibility.md:57
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · cognitive-errors -->»
  add: «»
  add: «- **CLI, a name with no endpoint:** the refusal reads the instance directory: a directory is `instance-dead`, anything else (absent, a regular file, a symlink) is `unwrapped-name`, and the strict-modes check stays first.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/a11y-plan-detail/cognitive-accessibility.md:58
  anchor: «»
  next: «<!-- detail: values · cognitive-errors -->»
  add: «<!-- detail: values · cognitive-errors -->»
  add: «»
  add: «- **CLI, the `unwrapped-name` and `endpoint-lost` human forms:** each ends with its own `hint:` as the last stderr line, in plain ASCII, the name in a hint being the parsed `ViolaName`.»
  add: «»
  add: «<!-- /detail -->»
  add: «»
- after: .andromeda/a11y-plan-detail/cognitive-accessibility.md:81
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · cognitive-errors -->»
  add: «»
  add: «- **CLI, a fault under `--json`:** where a CLI verb prints `error: internal error`, under `--json` it prints the `internal-error` document with a null `detail` on stdout in that line's place, stderr empty; without `--json` the line stands, and `run`, `revive`'s start, `mcp` and `hook` stay silent there.»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/a11y-plan-detail/cognitive-accessibility.md:92
  old: «the hint last, stdout empty; under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`»
  new: «the hint last, stdout empty; under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`»
- line: .andromeda/a11y-plan-detail/cognitive-accessibility.md:105
  old: «the exact `instance-unreachable` document»
  new: «the exact `instance-unreachable` document whose `detail` is `strict-modes-failed`»
- after: .andromeda/a11y-plan-detail/cognitive-accessibility.md:119
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · cognitive-errors -->»
  add: «»
  add: «- **Verification, one hint per code:** in `tests/cross_exit_causes.rs`, `no_two_codes_share_a_hint`, with its control `shared_hints_reads_a_planted_duplicate`, holds each code to its own hint, and `no_hint_names_release` holds every hint from naming `release`.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/a11y-plan-detail/cognitive-accessibility.md:120
  anchor: «»
  next: «<!-- detail: procedure · cognitive-errors -->»
  add: «<!-- detail: procedure · cognitive-errors -->»
  add: «»
  add: «- **Verification, the exit-cause table:** `tests/cross_exit_causes.rs`, a `fake-agent` E2E binary, is the exit-cause table: its rows check each cause's exit, document and hint in human and `--json` modes, and the comment at its top names the causes held outside the table.»
  add: «»
  add: «<!-- /detail -->»
  add: «»
- after: .andromeda/a11y-plan-detail/cognitive-accessibility.md:133
  anchor: «  - The `message` literal `wrapper fault` under `--json` and on MCP, for a code outside the five known, is pinned by the unit case `sanitised_fault_carries_nothing_of_the_peer_but_closed_values` (`crates/viola-channel/src/lib.rs`).»
  add: «  - The `unwrapped-name` and `endpoint-lost` hints (`src/human/hints.rs`) are pinned by the unit `send_hint_is_the_design_string` and by the exit-cause table (`tests/cross_exit_causes.rs`).»

## Patterns
[[pattern]]
id = "a11y2-01"
fixed = 'its strict-modes and preflight refusals give one'

[[pattern]]
id = "a11y2-02"
fixed = 'the `{"v":1,"error":"instance-unreachable","detail":null}` document'

[[pattern]]
id = "a11y2-03"
fixed = 'last; under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`'

[[pattern]]
id = "a11y2-04"
fixed = 'stdout empty; under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`'

[[pattern]]
id = "a11y2-05"
fixed = 'exactly `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty'

[[pattern]]
id = "a11y2-06"
fixed = 'with no live endpoint is refused with exit 21'

[[pattern]]
id = "a11y2-07"
fixed = 'the exact `instance-unreachable` document with stderr empty'
