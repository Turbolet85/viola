# H2 localisation rule — how a kept report is read

Written at /implement (2026-09-29), before any loop ran, host or runner. Every lost iteration is classified by this
rule and nothing else; a reading that fits no class is recorded verbatim as UNCLASSIFIED, never forced into one.

## The two reports
Both live in `<temp dir>/viola-pty-watch/`, named after the test (`::` → `.`), kept on a panic or a runner kill,
removed on a pass. Codes only: pids, sizes, byte hex, booleans, counts.

- **Child report** `<test>.report` — written by the spawned child (`tests::pty_child_entry`):
  `start pid={pid} raw={bool} size={c}x{r}` · `size {c}x{r}` (the watcher: one line per size change, read with no key,
  at any position after `start`) · `byte 78` · `byte 79 size={c}x{r}` (the size read after the second key) ·
  `restored={bool}`.
- **Test report** `<test>.test.report` — written by the test thread: `resize-returned` (after `pty.resize` returned) ·
  `key-written` (after `write_all` of `y`) · `key-flushed` (after `flush`) · `dsr-cpr {n}` (how many `ESC [ 6 n` the
  rig's drain counted in the child's output up to that moment; written at the verdict, at a deadline panic, or by the
  drop of a panicking test).

## What a loss is
A **lost iteration** is a run of `spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code` (or the
`hold` sibling) that panics with `child report never showed "byte 79 size=120x40"` — the `byte 79` line absent from
the child report at the 7 s `CHILD_WITHIN` deadline.

## Classes
- **R — the resize never applied:** no `size 120x40` line in the child report.
- **K — the resize applied, the key was lost:** `size 120x40` present in the child report AND `key-flushed` present in
  the test report.
- **E — the key arrived before the size changed:** the child report holds `byte 79 size=100x30`. Not a loss but a
  mis-ordering (the wait for `byte 79 size=120x40` times out on it); recorded apart from R and K.

`dsr-cpr {n}` is recorded beside every class. It is evidence, never the verdict.

## Readings outside the classes
- `resize-returned` present but `key-written` or `key-flushed` absent (and the test killed rather than panicked): the
  write itself hung — recorded as UNCLASSIFIED with both reports.
- `start` present with `raw=false`, or no `byte 78`: a failure before the resize — not an H2 loss, recorded apart.
- Any other shape: UNCLASSIFIED, both reports quoted whole.

## Counting
Only R and K count toward the plan's **3 localised losses**. E and UNCLASSIFIED are recorded but do not count.
