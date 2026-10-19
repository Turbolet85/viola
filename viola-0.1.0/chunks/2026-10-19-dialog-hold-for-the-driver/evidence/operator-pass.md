# Operator pass — 2026-10-19-dialog-hold-for-the-driver

Fired by the implementer on the operator's word for this chunk: "Then the operator pass with the ci.py conclusion read
(leg=operator), and stop ready for the wrap" (the /implement invocation, 2026-10-19). It ran after the live round
closed (`evidence/live-round.md`) and implement's report was written, in `.claude/rules/ci.md`'s order. Times UTC.

## 1. The native gate (before the pre-CI commit)

`gate.py run … --only 20` on the uncommitted tree, after the round's records were written (trail
`.andromeda/runs/2026-10-19T08-22-37-implement/`):

```
 20 probe       green · exit 0 · 85.98s · 383286 B → 20.3.log · bash scripts/agent-run.sh pre-push
entries 23 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 22
```

The Windows-target lint, read on the host before the push (ci.md, 2026-10-10; the command from testing.md
2026-10-03 Ext. 2026-10-10): `CARGO_TARGET_DIR=target/wincheck cargo clippy --workspace --all-targets --features
fake-agent --target x86_64-pc-windows-msvc -- -D warnings`: exit 0, `Finished` `dev` profile.

## 2. Hygiene (entry 21, fired as written)

`python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene`:

```
hygiene: clean — read 53 (runs 43 · evidence 5 · inputs 5) · trails 12 not read · copies 3 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

## 3. The pre-CI commit

`3def5b5bafc63e47600bf377af41739df7091fef`: `chore(2026-10-19-dialog-hold-for-the-driver): operator pre-CI commit,
for the run this chunk's verdict reads`, the whole tree. `git status --short` was empty after it.

## 4. The push (entry 22, `gate.py run … --operator 22`)

```
operator entry 22 · history tripwire: git
 22 probe       green · exit 0 · 2.53s · 92 B → 22.log · history moved: refs/remotes/origin/HEAD 55699c4f→3def5b5b; refs/remotes/origin/build/viola-0.1.0 55699c4f→3def5b5b · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
```

## 5. The CI read (entry 23, fired as written) — RED

```
ci v1.3 · 2b582a9b
repo Turbolet85/viola (the push remote `origin`) · polled 21× over 622 s
3def5b5bafc6 verdict: red · checks 15/15 · first-fail +618 s test (macos-latest) · wall 618 s · runs ci#39226239371 com…
runs: ci#39226239371 push completed/failure
failed 1: test (macos-latest) (failure)
```

The run itself (`gh run list --commit <full sha>`): `attempt` 1, `conclusion` `failure`, `event` `push`. One test
failed in that job's nextest summary (`2937 tests run: 2936 passed (6 slow), 1 failed`):
`viola::cross_list_rows cross_list_rows_style_each_span_beside_its_word_on_a_terminal`, panicked at
`tests/cross_list_rows.rs:267:13`. On the first terminal environment the board's escape sequences were
`[BOLD, RESET]`, where the case expects `[BOLD, RESET, amber, RESET]`: the builder row read `DIALOG none` with the wheel
`driver` and the CLI `verified`.

**The cause, read from the code (not reproduced: no macOS host):** the case waits for the dialog's
`events.ndjson` line (`tests/support/events.rs` `raise`), then reads the board, whose DIALOG column is
`snapshot.json`'s `pending_dialog`. The wrapper's `register` (`src/run/dialog.rs`) appended that line before it wrote
`pending_dialog`, so a reader that came between the two saw the line and an empty `pending_dialog`. That is the shape
events.md 2026-10-16 forbids. The order predates this chunk, and this case passed on macOS in every green run before
this one. The basis that it is not this chunk's (a two-sided reading on the runner) is unobtainable here, so it was not
claimed. The red is folded into the chunk (a red met during a chunk), and its cause is fixed in a listed file.

**The fix (a commit on top of the pre-CI commit, never an amend):** `register` writes `pending_dialog` before it
appends the line, and writes it back to none when the append fails. New unit cases:
`hook_dialog_held_is_on_the_snapshot_when_its_line_is_logged` (a capture layer reads `pending_dialog` on disk at
the wrapper's `dialog-raised` line, which follows the append) and `hook_dialog_append_that_failed_clears_the_pending_dialog`.
Remove-the-guard readings (testing.md 2026-09-25), each on the host:
- the clear removed: the failed-append case is red (0 passed, 1 failed), then restored (`cmp` 0);
- the old order restored: the order case is red 3 of 3 runs, then restored (`cmp` 0). A first form of the order case
  (a parked `wait` woken by the line, then a snapshot read) stayed green 12 of 12 runs on the old order. It cannot
  separate the two orders on this host, so it was replaced and is not counted as a witness.

## 6. The fix commit's pass (from step 1)

**The native gate** — the whole block on the uncommitted fix (`gate.py run`, third firing; trail
`.andromeda/runs/2026-10-19T08-22-37-implement/`):

```
 20 probe       green · exit 0 · 85.55s · 383551 B → 20.4.log · bash scripts/agent-run.sh pre-push
entries 23 · green 20 · red 0 · recorded 0 · timeout 0 · not-run 3
```

The Windows-target lint: exit 0, `Finished` `dev` profile. `gate.py scope`: `clean — changed 14 · listed 14`.

The live round (`evidence/live-round.md`) was taken on the build before this fix (`target/debug/viola` sha256 prefix
`5152f377a903d8ed`). The fix moves only when `pending_dialog` is written relative to the dialog's line. It touches no
part of the hold, the answer, the hook's bound or the expiry that the round read, and no start was re-made (the cap).

**Hygiene** (entry 21, as written), before the fix commit:

```
hygiene: clean — read 3 (runs 2 · evidence 1 · inputs 0) · trails 1 not read · copies 0 not read by P1 — 0 host paths kept · binary 0 not read by P1
```
