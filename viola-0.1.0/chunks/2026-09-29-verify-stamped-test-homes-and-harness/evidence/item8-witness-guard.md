# Item 8 witness — remove-the-guard readings (testing.md 2026-09-25)

Test: `viola::channel_endpoint stop_wait_holds_while_the_endpoint_answers` (`tests/channel_endpoint.rs`),
through plan gate 4 (`bash scripts/agent-run.sh run --integration --filter 'binary(channel_endpoint) &
test(/stop_wait_holds_while_the_endpoint_answers/)'`), on the Windows host, 2026-09-29.

| reading | `wait_endpoint_gone` (`tests/support/home.rs`) | gate 4 | test |
|---|---|---|---|
| guard in place (first block run) | polls until the endpoint is unconnectable | green, exit 0 | PASS 0.220 s |
| guard neutralised | an early `return` before the first poll (site re-read, edit confirmed by grep before the run) | red, exit 1 | FAIL 0.018 s — panicked at `tests\channel_endpoint.rs:174:5`: "the wait never read the live endpoint as reachable" |
| guard restored | the neutralising lines removed (`grep -c __never__` = 0) | green, exit 0 | PASS |

With the wait neutralised the helper returns without polling, so no `true` reading reaches the test's channel,
the sender drops with the finished thread, and `recv_timeout` reads disconnected at once: the red is the one the
plan predicts (plan step 5), not a timeout.

The endpoint the witness holds is a bound, unserved `viola_channel::Server` on the registered test namespace
`viola-test-chan-<pid>-gone`: dropping it closes the listener's handle, so the pipe goes. A served one would not
serve here on Windows, because its accept thread keeps the listener after the `Serving` guard drops.

# Gate 6's `artifact` atom — a directory's mtime

Gate 6 (`bash scripts/agent-run.sh run`, `artifact = 'target/agent-run/artifacts/'`) read `exit 0 ✓`,
`contains "ok":true ✓` and `artifact STALE (… 3466s older than this run)` at its re-run. The atom reads the
directory's mtime. Overwriting existing files inside a directory does not move that mtime on NTFS. The files the
run wrote were fresh, written inside the entry's own window, which ended when its log closed at 09:34:21 +0200:

- `junit-nextest-unit.xml` 09:33:54 +0200
- `junit-nextest-integration.xml` 09:34:20 +0200 (archived as `target/run-archive/352/`)

The archived integration JUnit names `viola::contract_ledger_probes contract_ledger_probes_pass_over_every_recorded_set`
once (0.994 s, passed); the same run read 728/728 unit and 225/225 integration.

The final whole-block run (19 green, gates 21-23 operator) read gate 6 the same way: `exit 0 ✓`,
`contains "ok":true ✓`, `artifact STALE (… 113s older than this run)`. Its files were written at 09:38:13 +0200
(unit) and 09:38:40 +0200 (integration, archived as `target/run-archive/359/`, which names
`contract_ledger_probes_pass_over_every_recorded_set` once). The directory's mtime moved only later, at
09:40:27 +0200, when pre-push's coverage stage created a file inside it. The atom's subject is the directory, and
nothing in this chunk's diff writes it.
