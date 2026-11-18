import sys

p = sys.argv[1]
L = open(p, encoding="utf-8").read().split("\n")
assert L[535].startswith("22. **The failed verify"), L[535]
assert L[595].endswith("is read by implement from the summary and recorded as read."), L[595]

NEW = """22. **The failed verify's first screens, rig-only under a hidden option — `src/cmd/verify.rs`.** Verify's args gain
    ONE hidden flag, `--record-first-screens` (`#[arg(long, hide = true)]`: absent from `--help`, named in no user
    doc; inputs#I13 item 2). It does nothing without `--record`. Where `run` returns `Exit(1)` on a failed row
    (`:296`), when BOTH `args.record` and `args.record_first_screens` are set, it first writes, for each run `x` of A–F
    whose first reading is kept and whose word is not `box` — and, for Run A, not `trust` — one file
    `<dir>/<version>/FirstScreen.<x>.json`: `{"v":1,"run":"<x>","screen":"<word>","cols":80,"rows":[…]|null}`, its rows
    WHOLE through `ledger::scrub` (the home to `~`, the user word to `<user>`), then every row and every join between
    rows checked by `ledger::screen_fault` before any file is written; any leftover path, name or email refuses the
    whole set through `refuse_recording` (`unable: a recorded fixture is not clean: FirstScreen.<x>.json row <n>[ seam]
    <code>`, nothing written). The files go through `write_recording` (`:571`), the passing path's writer, so the mode
    and the link rule are its own. A plain `--record` (no hidden flag) behaves EXACTLY as before this chunk: a failed
    verify writes no screen at all and creates no record dir; the passing branch (`record`, `:472`) is not touched, so
    a passing verify's file list and bytes are unchanged, the flag or no flag (it acts on a failed verify only). A
    stop ends the runs as `Ended::Interrupted` before the stamp (`:249-266`), so neither writer is reached. The flag's
    doc comment names the founder's two answers and the bound (inputs#I11 items 1–2, inputs#I13). `.gitignore` gains
    `fixtures/claude/*/FirstScreen.*.json`, so a recording into the fixture tree is never committed (inputs#I11 item 1,
    «never committed»).
23. **Its cases.**
    - Unit, `src/cmd/verify.rs`'s tests: `first_screen_files_take_each_failed_run_and_no_box`, an rstest table over a
      `TypedRun`: a `none` B writes `FirstScreen.b.json`; a `box` run writes nothing; A's `trust` writes nothing and
      A's `none` writes `FirstScreen.a.json`; a run with no reading writes nothing; a poisoned run writes `rows` null.
      `first_screen_scrub_keeps_a_temp_path_and_rewrites_the_home`: a row naming `<home>/x/.viola-verify-1` reads
      `~/x/.viola-verify-1`, the user word `<user>`, a `/tmp/viola-rig-<uid>/r/cwd` path kept as written; and a home
      path split across two rows refuses the set at the seam with `home-path`, naming the file and row (keeps tests
      S1, S4; security S5). `verify_help_names_no_first_screens_option`: `viola verify --help` holds no
      `record-first-screens`.
    - `tests/cli_verify.rs`, each a `--record` verify under the record home (testing.md 2026-11-15) with the fake agent
      told another config by `--trust-config` (as `verify_trust_record_row_fails_when_the_record_is_not_read`), so Run
      F meets its trust dialog and the verify fails (exit 1, `22 pass  1 fail`):
      - `verify_record_on_a_failed_run_writes_no_screen_without_the_rig_option` — `--record` alone: the record dir does
        not exist after the run (its listing equal to before this chunk: a failed `--record` verify wrote nothing);
      - `verify_record_first_screens_on_a_failed_run_writes_its_first_screen_only` — `--record` with
        `--record-first-screens`: the record dir's `2.1.0/` holds exactly `FirstScreen.f.json` (A's trust and B–E's
        box write nothing), its `run` `f`, its `screen` `trust`, `cols` 80, its rows those the fake drew, and no
        fixture of the passing path;
      - `verify_stopped_with_record_writes_no_first_screen` — the TERM case of
        `verify_stopped_during_the_trust_record_run_removes_its_record_and_exits_1` with `--record <dir>
        --record-first-screens` added: exit 1, the interrupted pair, and `<dir>` does not exist.
      `tests/support/verify.rs`'s `verify_started` takes the before-args it hands `verify_args` (its three callers pass
      `&[]`).
    - `verify_record_writes_the_scrubbed_spine_variants_and_screens` (`tests/cli_verify/stamps.rs`), a plain passing
      `--record`, keeps its exact file list unchanged — no `FirstScreen.` file — and
      `verify_record_writes_its_trust_record_to_the_kept_config_dir_only` asserts no `FirstScreen.` file under its
      record dir.
24. **The `V` arm passes `--record` and the hidden option and keeps the written screens locally —
    `crates/viola-e2e/src/rig/mod.rs`, `crates/viola-e2e/tests/rig_rehearsal.rs`.**
    - `run_child`'s `V` argv gains `--record <scratch root>/record --record-first-screens` before `--`
      (`rig/mod.rs:613-619`). No other arm, and nothing outside the rig, passes the hidden option.
    - After verify's exit and before the teardown, `start` reads `<scratch root>/record/<version>/` for
      `FirstScreen.<x>.json` (each read through `MAX_FRAME`, parsed as JSON, never printed) and COPIES each into the
      run's kept folder beside its frames, `<out>/<run>/FirstScreen.<x>.json` (`target/live-rig/<run>/`, gitignored),
      through the rig's private writer (`isolate::write_private`, 0600). The summary gains `kept: <letters>` (the
      runs copied, comma-joined in run order, e.g. `kept: b,c,d,f`), `kept: none`, or `kept: unread` when the record
      dir exists but cannot be read. Nothing of a file's content reaches the summary or the ledger. The scratch, the
      record dir in it, is removed by the teardown as always (`scratch: absent`).
    - `rig_rehearsal_v_reads_the_trust_record_row_and_no_record_left` adds `kept: none` (a passing verify writes no
      first screen). New `rig_rehearsal_v_keeps_a_failed_runs_first_screen_locally`: the V rehearsal with the fake
      told another config by `--trust-config`, so Run F meets its trust dialog and `trust-record` fails; it asserts
      `kept: f`, `<out>/<run>/FirstScreen.f.json` present with `run` `f` and `screen` `trust`, no other `FirstScreen.`
      file there, `record-left: absent`, `scratch: absent` and the teardown whole (`assert_torn_down`).
25. **Green and landed before the start.** Every block entry before the live round green in the tree, then the one
    control below, then hygiene and the land, as at step 19.
26. **`S0R-3`, the last live verify (inputs#I11 items 1, 3–7).** The install read by path, the rig's census, `viola`
    built and its hash read right before the start, then the live entry `S0R-3` on the R6 ledger
    `evidence/live-starts.ndjson` under `--cap 1 --spare 2` (its second spare; `S0R` and `S0R-2` stand ledgered), the
    census again after. `keys: none`; `stop:` printed beside it. A start refused for the token window is uncounted:
    implement stops and says so, and sets no timer.
    - **The predicate (inputs#I11 item 5).** GREEN when the verify ran whole (`verify-exit` a number), the teardown
      words stand (`census: 0`, `credentials: removed`, `scratch: absent`, `user-config` not `changed-by-rig`,
      `record-left: absent`, `keys: none`, `stop:` printed) AND EITHER the 21-row floor held OR `kept:` names at least
      one run; RED when the floor is not held AND nothing is kept (the instrument failed). The entry's atoms carry the
      run-whole and teardown words; the disjunction is read by implement from exactly two summary lines (inputs#I13
      item 4):
      1. **`failing:`** — the ids not `pass`. It names none of the 21 floor ids (the list in Implementation notes):
         the floor HELD — implement then reads `trust-record:` and takes step 11's two ends (`pass`: the row verified
         on 2.1.325 in the rig's private home; `fail`: Run F's word and facts named, the founder's fork). It names any
         floor id: the floor is BROKEN — go to line 2.
      2. **`kept:`** (read only when the floor is broken) — `kept: <letters>` naming at least one run: GREEN as a
         diagnostic reading, never verification (test-plan `decision/live-failing-reading-not-verification`), and
         implement names each kept screen (below); `kept: none`: RED, the instrument failed, recorded as read with the
         six words and facts, and nothing is named.
      Either way implement stops before the operator pass.
"""
L[535:596] = NEW.rstrip("\n").split("\n")
open(p, "w", encoding="utf-8").write("\n".join(L))
print("folded")
