# The capture witness — red on a bare scoped default, green on the helper (plan steps 2, 3)

Run dir `.andromeda/runs/2026-10-25T23-49-29-implement/`, Linux dev host, base `b6740d59`. The remove-the-guard run
of testing.md 2026-09-25: the witness was written first against a bare `tracing::subscriber::with_default` (no pinned
dispatcher), read red, then moved onto `test_support::with_scoped_default` and read green.

## Red — the witness over a bare `with_default` (step 2), 2026-10-25T23:51Z

Gate entry 8, `--entry 8` (trail `gate-2026-10-25-windows-endpoint-admission.json`, first firing, log `8.log`):

```
  8 unit        red · exit 0 ✗ (exit 100) · exit 100 · 6.3s · 1426 B → 8.log · cargo nextest run --workspace --features fake-agent -E 'te… (153 chars)
entries 24 · green 0 · red 1 (8) · recorded 0 · timeout 0 · not-run 23
```

Its failing block (`gate.py show --n 8`):

```
        FAIL [   0.014s] (1/1) viola::bin/viola tests::scoped_capture_sees_a_line_whose_callsite_another_thread_registered_first
    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 1045 filtered out; finished in 0.00s
    thread 'tests::scoped_capture_sees_a_line_whose_callsite_another_thread_registered_first' (1773987) panicked at src/main.rs:635:9:
    assertion `left == right` failed
      left: []
     right: ["state-recovered"]
```

The capture holds no line: the test thread's own emission was lost, because the spawned, subscriber-less thread
registered the callsite first while the capture was the only registered dispatcher.

## Green — the same witness through `crate::test_support::with_scoped_default` (step 3), 2026-10-25T23:52Z

The witness's one edit: its capture call moved onto the helper; the emitter, the spawn and the assertion unchanged.
Gate entry 8, `--entry 8`, second firing (log `8.2.log`):

```
  8 unit        green · exit 0 · 4.52s · 541 B → 8.2.log · cargo nextest run --workspace --features fake-agent -E 'te… (153 chars)
entries 24 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 23
```

Its log (`gate.py show --n 8`, `firings of entry 8 in this trail: 2 — 1 red · 2 green`):

```
        PASS [   0.006s] (1/1) viola::bin/viola tests::scoped_capture_sees_a_line_whose_callsite_another_thread_registered_first
     Summary [   0.009s] 1 test run: 1 passed, 3005 skipped
```
