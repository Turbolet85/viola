# Implement outcome — 2026-10-26-explicit-session-links

**Outcome: green + smoke.** Next → the operator pass (entries 24 hygiene, 25 push, 26 CI read), then
`/andromeda-wrap-session`.

## Gates
- First full firing (trail `.andromeda/runs/2026-10-26T14-59-26-implement/gate-2026-10-26-explicit-session-links.json`):
  `entries 26 · green 22 · red 0 · recorded 1 · timeout 0 · not-run 3` — the three `leg = 'operator'` entries.
  No fix-loop iteration. No `defer` key in the block, so no deferral and nothing voided; the header's
  `walk-class rust 4 … · uncommitted 457` voids nothing here.
- Counts read from the logs: unit `nextest-unit` 2495 passed, 0 failed · `binary(cli_links)` 12 passed ·
  `binary(channel_links)` 14 passed (P5's vacuous exit-94 baseline now green) · `binary(channel_endpoint)` 5 passed ·
  `binary(cli_output_plain)` 6 passed · `binary(security_negatives_server_identity)` 25 passed (19 → 25) ·
  `cargo deny check` green · perf 7 passed, 0 failed, hyperfine max per row: session-start 2.3 ms, user-prompt-submit
  2.3 ms, stop 2.2 ms, session-end 2.1 ms, pre-tool-use 2.6 ms (P5: 2.3 / 2.3 / 2.1 / 2.3 / 2.5 ms) · pre-push
  `"ok":true`, stage `linux-tests`, coverage 3133 passed, 0 failed, gate breaches none · `df -h /tmp` recorded:
  tmpfs 32G, 6.8G used, 25G avail, 22 %.
- Remove-the-guard controls (testing.md 2026-09-25), each red on exactly its guarded cases, then restored and the
  touched entries re-run green (`--only 2,3,8,9,10,13,15`: `green 7 · red 0`):
  `viola-0.1.0/chunks/2026-10-26-explicit-session-links/evidence/remove-the-guard.md` — A the instance check before
  any log (1 of 1 red), B `driver_of`'s `driven` and self checks (4 of 18 red, unit and wire), C the verbs' server
  check (6 of 8 red, the two control rows green).
- Guard entry 15, its pass line whole: `guard: allowed 13 — .config/nextest.toml, src/cmd/link.rs, src/cmd/mod.rs,
  src/cmd/run.rs, src/cmd/unlink.rs, src/human.rs, src/human/verbs.rs, src/run/links.rs, tests/channel_endpoint.rs,
  tests/channel_links.rs, tests/cli_links.rs, tests/cli_output_plain.rs, tests/security_negatives_server_identity.rs
  · companions excluded 0 · changed 13 · excluded 0` then `guard: clean`. No file excluded.
- Matrix: `matrix.py show --chunk 2026-10-26-explicit-session-links` reads `claimed by … : 0`, as the plan states
  (`v1-13`, `v1-36` not claimed): no ledger write.

## Smoke (P3)
smoke ✓ — the plan's smoke entries ran as P2 gates (16–18, 21 green); by hand, session `p-esl-p3` booted
(`"ok":true`), and against its live wrapper: `viola link overseer builder` → `link    overseer -> builder  since
15:12:28Z` (exit 0), again → `link    overseer -> builder  already linked`, `--json` →
`{"v":1,"ok":{"since":"2026-10-26T15:12:28.810Z","already_linked":true}}`, `viola unlink overseer builder` →
`unlink  overseer -> builder`, `--json` → `{"v":1,"ok":{}}`, `viola link builder builder` → `error: wrapper fault
-32602`, exit 20 (the plan's constraint); the log held one `link` and one `unlink` line, both `source: wrapper`,
the snapshot's `links` `[]`; `viola --help` shows `handoff:  link, unlink` between `wheel:` and `setup:`; `status`
`state:"ready"`; `cleanup` `processes_gone:true, endpoint_gone:true, home_removed:true`.

## Files changed
- New: `src/cmd/link.rs`, `src/cmd/unlink.rs`, `tests/channel_links.rs`.
- Modified: `src/run/links.rs`, `src/cmd/run.rs`, `src/cmd/mod.rs`, `src/human/verbs.rs`, `src/human.rs`,
  `tests/channel_endpoint.rs`, `tests/cli_output_plain.rs`, `tests/security_negatives_server_identity.rs`,
  `tests/cli_links.rs`, `.config/nextest.toml`.
- Outside research's lists: none (`gate.py scope`: `scope: clean — changed 13 · listed 13 · recorded 0`; no
  `scope-record.md`).

## Deviations from the plan, with justifications
1. **Step 4, the test-module gate of `src/cmd/link.rs`.** The step gates each verb module's whole test module
   `#[cfg(test)]` + `#[cfg(not(target_os = "macos"))]`, but `link.rs`'s module also holds the reply-shape table and
   the line case, and the plan's own macOS line says the verbs' other cases run there. Only the server-check case
   is gated off macOS (the shape of `src/cmd/release.rs`'s module); testing.md 2026-10-26 asks a whole-module gate
   only of a module whose every case is gated. `src/cmd/unlink.rs`'s module, whose one case is gated, is gated whole
   with the two stacked attributes.
2. **Step 6, one assertion of `channel_wrapper_logs_each_call_with_corr_and_conn`.** The step keeps every assertion,
   but the case asserted the conn-less response line's `method` equals `unlink`; the channel names only the closed
   method list (`crates/viola-channel/src/server.rs` `method_label`) and logs `anything` with no `method` key, so the
   assertion became that key's absence. Its `-32601` assertions and the joins are unchanged.
3. **Step 4, the shared args.** `viola unlink` takes `link::LinkArgs` (the same two positionals and `--json`), so the
   thin second verb holds only its `request` under `cli.unlink` and its one case.
4. **Step 10, the frame writer.** The raw frames go through `viola_channel::Client::request` straight to the
   endpoint, which adds the envelope's `v` and `sender` and runs no server check, so each frame reaches the wrapper
   past every CLI check, as the step asks; no hand-built byte frame.
5. **Step 1, the refusal table.** Beside the step's name classes, the inline table carries the absent case for both
   params (`driver_absent`, `driven_absent`); the wire table (step 10) keeps the step's list exactly.

## Surfaced
- No spec↔reality gap.
- One observation for the wrap's reading of design S1–S3 (no amendment authored): a `since` that is no RFC 3339
  instant prints through `escape_message`, as the plan's step 3 names, and message mode keeps `\n`, `\t` and
  non-ASCII as they are; the wrapper's own `since` is a line's `ts` or the restore's `unknown`, so a printed line
  holds a newline or a non-ASCII byte only from a hand-altered log line.

## Process hygiene
`gate.py census --comm viola --comm viola-harness --comm viola-fake-agent --comm claude --args projects/viola/target`
(trail `.andromeda/runs/2026-10-26T14-59-26-implement/census-2026-10-26-explicit-session-links.json`): `census: 15
running`, none holding `projects/viola/target`.

| process | started by | final state |
|---|---|---|
| harness session `p-esl-smoke` (supervisor, wrapper, fake agent) | a P2 entry (17) | terminated — entry 21 `processes_gone:true`, `endpoint_gone:true` |
| `run --perf`'s `perf-<pid>` session | this run (entry 22) | terminated — its own cleanup; none in the census |
| root-test wrappers and fake agents (entries 8–13, 23, the three controls) | this run | terminated — none in the census |
| harness session `p-esl-p3` (P3 smoke) | this run | terminated — `cleanup` `processes_gone:true`, `endpoint_gone:true` |
| the 15 census rows (the overseer pair's bridge builds under `~/dev/projects/additional/viola-lab/…`, the product `viola wait`, the host's `claude` sessions, this session's own `viola mcp` under `~/.viola/bin/`) | NOT DERIVED — not this run's | left running: not this run's — their owners' |

## Decisions & corrections
- Guard refusal: the Bash guard refused a `cat` heredoc with a scratchpad file target (its file-target arm); the
  append went through the Edit tool.
- Guard refusal: the Bash guard refused a `cd` into the gate's log dir (it moves the session cwd); re-issued with
  absolute paths.
- Standing learning met: testing.md 2026-09-25 (remove-the-guard), each neutralising edit read back by `grep -n` and
  each restore read back (its 2026-09-25 extension), recorded in `evidence/remove-the-guard.md`.
- The operator's word for this run (the session's opening request): "please run implement per the skill on
  2026-10-26-explicit-session-links, through its operator pass and the CI read. Journal each long run in the run dir
  before you wait on it." — journal at `.andromeda/runs/2026-10-26T14-59-26-implement/journal.md`.
