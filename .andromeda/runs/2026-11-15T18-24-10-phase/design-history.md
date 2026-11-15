@history_item n=1 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — replaced verify's fixed «a recorded payload still holds a path or a username» with the named `a recorded fixture is not clean: <file> <code>` refusal (closed codes, never the content); the source of contract/viola-verify-refusals/unclean-fixture, whose `home-path` code is what item 5's record-home naming rule (no `home` path component in a `--record` test home) keeps the in-repo `--record` callers clear of.

@history_item n=2 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — fixed verify's interrupted run: no step line, no summary, exactly `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, stdout empty, exit 1 (contract/viola-verify-refusals/interrupted); `cli_verify::verify_stopped_during_the_resume_run_removes_its_dir_and_exits_1`, item 3's red receipt-wait case, witnesses this pair, so a change to `WITHIN` must not change what it asserts.

@history_item n=3 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder ratified the interrupted pair's wording (planning mark gone); the strings the stopped-verify test asserts are founder-worded and stay byte for byte under this test-only chunk.

@history_item n=4 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — landed `revive`'s fixed-order refusals (stop at first; `run`'s already-live and stale pairs unchanged) and «prints nothing once the child starts» (contract/viola-revive-refusals/order, contract/viola-revive-output/after-child-start); the behaviour `chaos_revive::revive_after_a_killed_wrapper_resumes_the_logged_session_in_the_recorded_directory`, one of item 2's ~5 s slow cases, exercises.

@history_item n=5 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — gave `revive --json` its exit-1 documents `{"v":1,"error":"<code>","detail":null}`; precedent for the revive-codes row the slow chaos cases sit beside.

@history_item n=6 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (run's and revive's start refusals at the home-tree check) — `run`/`revive` check home, `ledger/`, `bin/`, `plugin/`, the pinned copy and its `plugin/<key>/` before any log, no child; home pair vs instance pair (contract/viola-run-start-refusals/home-tree-check); the start path `cli_instance_state::run_refuses_a_tampered_pinned_copy` and `run_rewrites_the_plugin_folder_each_start` (item 2's slow class) drive, and why a test home's modes matter.

@history_item n=7 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (verify's home-tree and stamps refusals) — widened verify's no-step-line case to «stopped by Ctrl-C/TERM, or refused by its home tree's check before its log» (contract/viola-verify-output/stopped-or-refused); any helper item 5 adds for `--record` homes must still produce owner-only homes or the run refuses with the home-state pair.

@history_item n=8 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the six frame verbs' strict-modes refusal prints the instance-state pair) — fixed the instance-state pair's wording and its printers (contract/fixed-refusal-pairs/instance-state); `cli_instance_state` cases assert it, and this chunk must not move it.

@history_item n=9 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (`link` / `unlink`'s exit-21 strict-modes refusal takes the fixed instance-state pair) — made the instance-state pair `revive`'s shared pair, naming the instance checked; background for the instance-state row the slow `cli_instance_state` tests rest on.

@history_item n=10 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the catch site's `internal-error` document) — added «`run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site, under `--json` too» (contract/cli-catch-site/silent-verbs, decision/silent-catch-sites); `tests/hook_fail_open.rs` (item 1) witnesses `hook`'s silence, exit 0 and empty stderr, which the socket-name fix must keep.

@history_item n=11 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`revive --json`'s documents and its ten codes) — every refusal of the start `revive` shares with `run` prints one document; `internal-error` for a program not found or an anyhow start error, the catch site logging its chain (contract/viola-revive-json-codes/internal-error); a precedent the revive chaos cases sit under.

@history_item n=12 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the pending arch block spent, the log's codes and the two carve-outs) — names `pinned-hash-mismatch` among D-20's log detail codes; the code behind contract/viola-run-start-refusals/pinned-hash-mismatch, which `run_refuses_a_tampered_pinned_copy` (item 2) witnesses.

@history_item n=13 marker=2026-11-01-t02-46-44-wrap
**2026-11-01-t02-46-44-wrap** — the founder's word superseded the PROVISIONAL marks of the home-and-code-bearing-file-integrity and exit-cause-code-catalogue entries above; their rows stand as his, not open for a test-only chunk to reword.

@history_item n=14 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (exit 2's fixed stderr lines, no `--json` document) — reworded exit 2 as clap usage (never from `viola hook`) or the fixed `UNUSABLE_RESPONSE` / `UNUSABLE_TEXT` lines (contract/cli-exit-codes/2); the «never from `viola hook`» clause is part of the hook contract `hook_fail_open.rs` guards.

@history_item n=15 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the socket-directory pair at the start of run and revive) — `run`/`revive` refuse an untrusted socket directory before any snapshot, child or bind; the product's own endpoint rules beside which item 1 renames only the test channel's socket (test-only; the product endpoint path and its refusals untouched).

@history_item n=16 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the cli Platform narrowed to Linux terminals) — Platform reads «Linux terminals (the core on any current distribution, CI on `ubuntu-latest`)» (decision/linux-terminal-platform); `sun_path`'s 108 B / 107-usable bound item 1 proves is the Linux one, the only target.

@history_item n=17 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the .cmd/.bat refusal and batch-script-child retired) — `revive`'s closed codes went to ten, pinned in `tests/cross_exit_causes.rs`; a test-pinned register the chunk's test edits must leave intact.

@history_item n=18 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the Windows VT enable retired from the colour rules) — the colour decision order lost its Windows rung; the order whose `viola hook` match (no colour, glyph or SGR, never stderr; contract/cli-colour-decision-order/viola-hook) the hook test reads.

@history_item n=19 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters became rows (every picked row's handle dates from here, verified against the code at 8834195b0683); the rows this chunk binds are contracts the code was checked to hold, so a test-only change that turns one red is a defect to fold, not a row to amend.
