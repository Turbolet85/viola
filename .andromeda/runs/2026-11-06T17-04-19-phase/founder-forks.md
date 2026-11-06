# Founder forks — 2026-11-06-version-done-check (phase P4, held)

## The hold (verbatim, as it came back in founder dialog 1/7, 2026-11-06)

> HOLD, not an answer (the overseer, 2026-11-06 17:58Z): the founder is away and wants the machine quiet. Record
> nothing as his word. Stop the phase here at P4: write where P4 stands, every founder question included, to the run
> dir, run no build, test, harness or batch, and wait for the operator's word. All seven founder dialogs are re-asked
> then.

Nothing below is the founder's word. Dialog 1/7 came back with every answer the hold above; dialogs 2/7–7/7 were not
raised. On the operator's word all seven are asked again, in this order, then the overseer's technical forks.

## Where P4 stands

- **Done:** P1 (promoted, tx 412; scope rev 3 after P3's closure, tx 418; `inputs#I1` the operator's directive),
  P2 (7 extracts, 6 masters cascaded, 7 histories; every check passed), P3 (research rev 1, tx 417; lists parsed, 51
  items). Each one's evolve checkpoint is appended.
- **P4 read so far:** the 7 extracts and 7 history files (P2's reads), the 17 caps whole (`rec.py get
  cap:viola-0.1.0/v1-…`), the playbook's 4 escalate patterns. None applies: the portable-pty move narrows the graph
  (pin-utils, serial and 6 more crates leave; serial2, nix 0.28 and cfg_aliases join, every licence inside `allow`
  through `Apache-2.0`), and dropping the Git Bash warning and the unmatched RUSTSEC-2017-0008 ignore are narrowings.
  No other markerless entry lists any of the 17 caps (this is the version's last entry), so each one is proved here or
  deferred to 0.2.0.
- **P4 not yet done:** reading the 37 cascade extracts and 6 standing lists, writing `plan`, P4's evolve checkpoint.
  P5 has not started. No matrix call and no `--run-dir` matrix trail exist.
- **Decisive leans (no question; the plan's Provenance will state them):**
  - Item 1, tool or text: **text**. Every manifest has 0 hits for trycmd/assert_cmd/snapbox, and `tests/cmd/` is
    absent. Since chunk 2026-10-11-the-board-viola-list, each new fixed string has been pinned by a literal
    (a11y-plan history: «surviving only by an ever-longer exception list»). The masters get re-worded to the
    literal-assert mechanism the tests actually hold.
  - Item 3: no row to add. `PTY_SIDELOAD_TEST_CASE` left the code with the sideload (0 hits in the tree and in 4
    masters; control `SERVE_TEST` → 1 hit). The CARRY retires with that reading.
  - Item 16: 9 stays, because it is the product count. The grep's tenth hit is the test-only `HarnessError`
    (`crates/viola-e2e/src/harness/mod.rs:25`), and the grep read 10 at the census's own commit `16ef6b48` too.
    The rule's wording is what gets fixed.
  - Item 12: portable-pty `=0.8.1` → `=0.9.0`. Measured in a scratch copy of HEAD: `cargo deny check` reads
    `advisories ok, bans ok, licenses ok, sources ok`, and `cargo nextest run -p viola-pty` passes 33 of 33.
    The RUSTSEC-2017-0008 ignore goes, since its crate leaves the graph.
  - Item 15: the measurement is `tests/tui_channel_fds.rs` green on the moved graph plus one recorded table.
    The mechanism is portable-pty's `close_random_fds()`, the same in 0.8.1 and 0.9.0.
  - Route split: none proposed. On the recommendations below the proved set is 13 caps, almost all claimed by
    reading witnesses that already exist. The new work is one dependency move, a remnant sweep over 48 files, at
    most three added tests and one live start.

## Founder dialog 1/7 — caps v1-01 · v1-03 · v1-04 · v1-05 (asked once, held)

**v1-01 — Linux CI pipeline.** Every push runs build, fmt, fatal lint, the tokio-free check, the dependency policy,
zizmor and tests; a weekly advisory run; perf in its own job; tools checked first; homes kept until the gates;
uploads only after the secret scan.
1. **Prove here (Recommended).** This chunk moves portable-pty to 0.9.0, which drops pin-utils (the advisories
   red). The proof is the pushed sha's CI run with all 7 jobs green, plus nightly.yml's last scheduled run (green
   2026-11-02). Cost: one CI read in the operator pass. Every other clause already exists.
2. **Defer to 0.2.0.** Leaves the version's CI capability unverified although its only red is fixed in this chunk.
   Nothing is saved: the CI read happens anyway.

**v1-03 — The fake agent and recorded fixtures as the test contract.**
1. **Prove here (Recommended).** The boundaries its notes left open (annotations, vt100-panic bytes, statusline-echo,
   agents --json) have all landed since. The proof reads the drift contract (`contract_fake_agent_drift`), the
   answer path and the fixture-hygiene contract. If the drift suite has no planted forwarded-annotations case, one
   is added. Cost: small, at most one test case.
2. **Defer to 0.2.0.** No remaining gap needs a 0.2.0 surface, so a deferral would be for time only.

**v1-04 — Structured JSON logs from every process, and a logs view.**
1. **Prove here (Recommended).** Every role (run, hook, mcp, cli), the hook's stderr and exit guarantee, and the
   logs view's filters with torn-line records exist with witnesses. The ui role and the SSE events left 0.1.0 by his
   2026-10-16 ruling. Cost: reading the evidence only.
2. **Defer to 0.2.0.** Nothing in its CLI half is missing; a deferral would leave a built capability unverified.

**v1-05 — Observability gates.**
1. **Prove here (Recommended).** G2, G4, the canary secret-scan, the print and raw-log lint bans, and the
   forced-panic hook seam all exist. «No setting disables redaction» is proved by enumerating config.json's closed
   settings table: one new test asserting no key reaches redaction. Cost: small, one test.
2. **Defer to 0.2.0.** Defers gates that already run on every push, for want of one enumeration test.

## Founder dialog 2/7 — caps v1-07 · v1-09 · v1-10 · v1-11 (not yet raised)

**v1-07 — Quality gates: coverage, performance budgets, property/fuzz replay, zero flakes.**
1. **Prove here (Recommended).** The coverage floors 85/95/80, hook deadlines on the worst sample (hyperfine `max`,
   five rows), and seeded property tests plus a corpus replay that fails on an empty corpus all exist. Each of the
   five security-named parser surfaces (security-plan fct-h4bkgare: `validate_paste_text`, channel framing, hook
   stdin including statusline, `claude agents --json`, the vt100 feed) has a fuzz target (`fuzz/fuzz_targets/`: 6
   files) and a proptest module. Also in place: `retries = 0`, and one gate verdict per job with a missing artifact
   read as a breach. Cost: reading the evidence only.
2. **Defer to 0.2.0.** No gap remains that 0.2.0 would close.

**v1-09 — Security's eight amendments folded into the architecture (by construction).**
1. **Prove here (Recommended).** /implement enumerates where the architecture states each of the five CLI
   amendments that stand: the per-user 0700 Unix endpoint directory; `not-delivered`/`control-character` ahead of
   the refusal order; `release-from-driver`; `MAX_FRAME` 16 MiB; the largest-hook-payload ledger row; truncated
   SHA-256. The three web amendments (the cookie, the two URNs, `ui/<port>.url`) left 0.1.0 by his 2026-10-16
   ruling. Cost: small, one enumeration.
2. **Defer to 0.2.0.** The CLI amendments are all landed; a deferral would leave a done fold unverified.

**v1-10 — CL-1: send issue and outcome are events in the log.**
1. **Prove here (Recommended).** The `send-issued`/`send-confirmed`/`send-refused` records, the unconfirmable
   outcome, the client-side refusal line, and `wait` not waking on them all have tests. «The page and the tests read
   the same records» reads, by his 2026-10-16 ruling, «the tests read the logged records». Cost: reading the
   evidence only.
2. **Defer to 0.2.0.** Nothing in 0.1.0's half is missing.

**v1-11 — The live page sees a dialog clear and state refresh.**
1. **Defer to 0.2.0 (Recommended).** Its subject is the page's live feed, which left 0.1.0. Its log half has no
   liveness record by design: a dead wrapper writes no line, and liveness is read from pid and start time. Proving it
   needs a new event kind, which is new surface at the version close.
2. **Re-word and prove.** Keep the dialog clear and the status/budget records, drop liveness: an outcome weaker than
   the one written.

## Founder dialog 3/7 — caps v1-13 · v1-21 · v1-24 · v1-26 (not yet raised)

**v1-13 — Result payloads carry what the CLI prints** (CARRY item 4).
1. **Re-word and prove (Recommended).** Drop «and MCP» from the acceptance. `release` and `link` are CLI-only by the
   ruled [MCP] design (release is a human control, never a driver's). Every human line of `release --budget` and
   `link`/`unlink` is built from its `--json` result, and that is built and tested. Cost: an acceptance re-word plus
   reading the evidence.
2. **Defer to 0.2.0.** Reason: no MCP surface exists for release or link while architecture [MCP] stands.
3. **Add the MCP results.** New tools against a ruled design, at the version close. Not recommended.

**v1-21 — Timing constants named and located.**
1. **Defer to 0.2.0 (Recommended).** Five values have one home each, driven by an injected clock and read by the
   perf gate. Its last clause («every MCP call completes below the client's tool-call timeout») is unprovable now:
   the client's default is unmeasured (known only to exceed 50 s, a `wait` answered at 50 012 ms on 2.1.287), and a
   `send`'s longest block is 48.5 s plus the paste, which no bound caps under 50 s.
2. **Prove with a live measurement.** Spend the spare live session measuring claude's MCP tool-call timeout. Cost:
   the directive's second live start, plus a bound on the paste time.
3. **Re-word to the measured lower bound and prove.** A weaker outcome than the one written.

**v1-24 — The statusline source `run` records.**
1. **Prove here (Recommended).** Its only gap was the Windows half (the override and the shell-out), which left with
   the Linux-only ruling. The named source, never written, redirectable per home (`boot --statusline-echo`), and the
   pass-through witness all exist. Cost: reading the evidence only.
2. **Defer to 0.2.0.** Nothing in its Linux half is missing.

**v1-26 — Cross-plan fix-pass items honoured (manual).**
1. **Prove here by enumeration (Recommended).** A manual table names each fix-pass surface with its passing test:
   X8 (the budget-override gate display, `release --budget`'s two lines), X10 (exit 21's `instance-unreachable`),
   X11 (the `release-from-driver` display). X13 (the 401/launch copy) left 0.1.0 with the page. Cost: one manual
   review table at /implement.
2. **Defer to 0.2.0.** The hint's «fix-pass items honoured» clause would then stand unproved at the version close.

## Founder dialog 4/7 — caps v1-27 · v1-34 · v1-35 · v1-36 (not yet raised)

**v1-27 — `viola run` wraps the unmodified CLI on Linux.**
1. **Prove here (Recommended).** Every clause has a fake-agent witness. «Unmodified CLI» is read in the one live
   `viola run` on Linux, the same start as v1-53's (live session 1 of 2). Cost: shared with v1-53.
2. **Defer to 0.2.0.** The live run happens anyway for v1-53.

**v1-34 — Capability ledger, `viola verify`, and transport-only degrade.**
1. **Defer to 0.2.0 (Recommended).** Two relied shapes still have no probed row: the harness-prefix row and the R8
   identity-floor row. His 2026-10-07T09:43Z ruling minted no entry for them. A probe needs his own ruling on a
   second session addressing a verify child, and on the capture arm reading environment names.
2. **Rule the two probes now and land the rows here.** Cost: verify changes and a record round of real `claude`
   starts beyond the directive's two-session cap.

**v1-35 — Budget governor from the statusline.**
1. **Prove here (Recommended).** The Windows half left with the Linux-only ruling. «Unknown with its age» shows on
   the board's BAY line. Path 6 and the two-instance override leg are witnesses. Cost: reading the evidence only.
2. **Defer to 0.2.0.** Nothing in its Linux half is missing.

**v1-36 — Links between sessions** (CARRY item 4).
1. **Defer to 0.2.0 (Recommended).** Reason: the board clause. `viola list` keeps no link column by his ruled
   design, and `list --json` items omit `links` (pinned in `crates/viola-state/src/board.rs`'s serialise test). The
   board or page that shows the link set is 0.2.0's.
2. **Re-word and prove.** «Shown by the link/unlink output and the instance's snapshot»: weaker than «shown by the
   board».
3. **Add `links` to `list --json`.** A contract change at the version close, which needs a design word.

## Founder dialog 5/7 — cap v1-53 (not yet raised)

**v1-53 — The fake-agent suite and one live run on Linux** (CARRY item 9).
1. **Prove here (Recommended).** The proof reads the OS-gate census (`tests/contract_os_gated_tests.rs`, re-measured
   after the remnant sweep) and the pushed sha's `ubuntu-latest` JUnit. It adds one live `viola run` of real
   `claude` on Linux: live session 1 of the directive's 2, on the headless pty rig outside the repository, with no
   key and no window, ledgered before its start. Cost: one live start.
2. **Defer to 0.2.0.** The version would close with no live Linux reading of the wrapper.

## Founder dialog 6/7 — the four items owed his own word at Epoch 7's boundary (CARRY item 7; not yet raised)

Each is a delegate's answer (the overseer), PROVISIONAL until he rules it.
1. **The socket-directory pair's wording:** `unable: the socket directory for <name> cannot be trusted` → `hint:
   viola's socket directory must be a directory you own, readable only by you; viola does not change its
   permissions` (design-system/surface fct-pbvocw2a). Options: confirm (Recommended: it names no path, pid or uid,
   and is plain ASCII with `hint:` last), or give his wording.
2. **The board's read rule:** a home another user may read (any group or other bit) refuses the whole `viola list`
   board, exit 21, with or without an instance. Options: confirm (Recommended: the same read rule as
   `check_instance`, so the board refuses consistently), or ask for the write rule on the home.
3. **Chunk 2026-11-01-unix-endpoint-and-home-hardening's P5 word and its leans:** one code for a failed create and a
   refused check (`socket-dir-refused`), the hook's one `hook-decision` line, and the wrapper's silent drop of a
   refused peer. Options: confirm (Recommended), or name the lean to revise.
4. **That wrap's answer-card confirm** that 151 proposals lie inside his own answers to Q1 and Q2
   (`.andromeda/runs/2026-11-01T17-25-39-wrap/answer-card.md`). Options: confirm (Recommended), or re-open the card.

## Founder dialog 7/7 — the CARRY items that are his (not yet raised)

1. **Item 2, the dialog hold near the 3600 s ceiling.** Options: read it live in live session 1, with
   `dialog_hold_s` 3600 and a permission dialog left unanswered for about 59 minutes and then read (Recommended:
   lifts `DIALOG_HOLD_MAX`'s PROVISIONAL, cost about an hour of the one start already made for v1-53); or keep
   PROVISIONAL into 0.2.0.
2. **Item 5, Measurement B.** Options: record it as known with no action in 0.1.0 (Recommended: every loaded section
   is equal, so the worst case is a second pin folder on disk); or make the release build reproducible now (e.g. no
   build-id, a stripped symtab), a build-profile change at the close.
3. **Item 10, the wheel's terminal-reply hypothesis** (`CSI ? 6 n`, the kitty graphics query). Options: carry it to
   0.2.0 unmeasured (Recommended: the headless rig answers neither query, and a shape joins the list only measured,
   on his word); or measure it now on a live terminal he attends.
4. **Item 11, the revive legs.** Options: take the killed-wrapper leg on real `claude` at the end of live session 1
   and carry the founder-attended tab-close to 0.2.0 (Recommended: the directive allows no window); or carry both to
   0.2.0.

## The answer (appended 2026-11-06, the operator's word lifting the hold; `inputs#I2`)

The founder's own word, live, by dialog in the overseer's session, 2026-11-06 19:34Z: «Принять всё» — every
recommendation of dialogs 1/7–7/7 above, recorded as his own word, not a delegate's. The overseer's technical answers
(founder-delegated, 19:34Z): T1 ratify all five divergences as named exceptions with their reasons; T2 remove the
Git Bash warning with its emitter, the masters and the CLAUDE.md learning following; T3 this session's own wrapping
and the driver's sends into it, kinds and counts only, no live start. Live sessions stay at his cap of 2. No founder
dialog is raised again for these.

## The overseer's technical forks (apart from his; asked after the founder's)

- **T1, item 6: fold or ratify each one-enum divergence** (`ServeError` beside `McpError`; `StampsError`,
  `StampsReadError`, `PinError` and `Refused` beside `StateError`). Options: ratify all five as named exceptions
  with their reasons (Recommended: a fold at the close moves exits and messages across five enums for no behaviour
  gain; the precedents folded one enum each); fold all five; or fold the two the rule names and ratify the three the
  census added.
- **T2, the Git Bash rewritten-path warning** (`src/cmd/send.rs:53-60` and its witness in `tests/cli_send.rs`).
  Options: remove it with its emitter, the masters and the CLAUDE.md learning following (Recommended: a Linux shell
  never rewrites a leading-slash argument); or keep it with its reason.
- **T3, the witness of «overseer drives a builder through viola».** This builder session runs wrapped
  (`VIOLA_NAME=viola-builder`, its parent the installed product `viola`). Options: the chunk's evidence records this
  session's own wrapping and the driver's sends into it, kinds and counts only, read through `viola list --json` and
  the instance's event kinds (Recommended: spends no live start); or spend the spare live session on a fresh
  driver-and-builder pair.
