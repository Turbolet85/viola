# H2 measurement loop — the step the operator pass inserts (plan step 8)

Written at /implement, 2026-09-29. **Not committed into ci.yml by /implement.** The operator pass inserts it for
each measurement push (plan steps 10 and 12's verification push) and removes it again by restoring
`.github/workflows/ci.yml` byte-for-byte to `90aba7c` (step 11; the byte-restore guard entry reads green only then).

## Where it goes
In the `test` job of `.github/workflows/ci.yml`, directly after the step `Coverage and doctest (sh shim)` (the last
of the two `run --coverage` steps) and before `Harness lifecycle (pwsh shim)`, so it runs after the job's
instrumented suite, and before every upload. Indented as the job's other steps (6 spaces before `- name`).

## The step (verbatim)

```yaml
      # MEASUREMENT ONLY (2026-09-29-h2-conpty-resize-probe): removed before the chunk's wrap.
      - name: H2 loop (measurement only)
        if: matrix.os == 'windows-2025'
        shell: bash
        run: |
          set -euo pipefail
          shopt -s nullglob
          iterations="${H2_ITERATIONS:-200}"
          h2tmp="$RUNNER_TEMP/h2-tmp"
          mkdir -p "$h2tmp"
          TMP="$(cygpath -w "$h2tmp")"
          TEMP="$TMP"
          export TMP TEMP
          watch="$h2tmp/viola-pty-watch"
          junit=target/nextest/ci/junit.xml
          cp "$junit" "$RUNNER_TEMP/h2-junit.xml"
          losses=0
          for i in $(seq "$iterations"); do
            if cargo llvm-cov nextest --no-report --profile ci -p viola-pty \
                -E 'test(/spawn_runs_a_raw_child_that_sees_its_size_a_resize/)' > "$RUNNER_TEMP/h2-iter.log" 2>&1; then
              continue
            fi
            kept=("$watch"/*)
            if [ "${#kept[@]}" -eq 0 ]; then
              echo "h2-loop: iteration $i failed with no kept report"
              tail -n 40 "$RUNNER_TEMP/h2-iter.log"
              exit 1
            fi
            losses=$((losses + 1))
            echo "h2-loop: iteration $i lost"
            for f in "${kept[@]}"; do
              echo "--- ${f##*/}"
              cat "$f"
            done
            rm -f "${kept[@]}"
          done
          cp "$RUNNER_TEMP/h2-junit.xml" "$junit"
          echo "h2-loop: iterations $iterations · losses $losses"
```

## What each part is for
- **Same instrumentation as the reds:** `cargo llvm-cov nextest` under the `ci` nextest profile (retries 0, the
  30 s × 4 kill), the instrumented build the two sightings ran under. One test selected per iteration: the red test.
- **The temp dir is pinned** (`TMP`/`TEMP` → `$RUNNER_TEMP/h2-tmp`), so the kept reports are found where the loop
  reads them whatever Git Bash's own temp mapping is. `std::env::temp_dir()` reads `TMP` first on Windows.
- **A loss** is a failed iteration that left kept reports: they are printed whole (codes only, per the recorder) and
  removed, so the next loss starts clean. The job log is their record — no upload is added.
- **Fail-closed:** any setup command failing fails the step (`set -euo pipefail`), and so does an iteration that fails
  with NO kept report (a build failure, a nextest usage error): the measurement is then broken, not a loss, and the
  last 40 lines of that iteration's output are printed.
- **A measurement, not a gate:** a loss never fails the step. It ends with the one tally line
  `h2-loop: iterations 200 · losses {n}` and exits 0.
- **The job's own JUnit** (`target/nextest/ci/junit.xml`, which `Upload JUnit` ships) is copied aside before the
  loop and restored after it, since each loop iteration's `--profile ci` run rewrites it.
- Adds no `uses:`, no upload, no `github.event` value, no viola env var. `H2_ITERATIONS` is the loop's own count
  (default 200), read by nothing in viola.

## Measured on this host before handing it over (a script check, not runner evidence)
- First form carried `--no-clean` beside `--no-report`: cargo-llvm-cov 0.9.1 refuses the pair
  (`--no-report may not be used together with --no-clean`). The fail-closed branch caught it on iteration 1
  (`failed with no kept report`, exit 1). The flag was dropped; the command is now the plan's exactly.
- `H2_ITERATIONS=3` with `RUNNER_TEMP` pointed at a scratch dir: exit 0, `h2-loop: iterations 3 · losses 0`, about
  9 s wall for the three (the first included the instrumented build), `junit.xml` sha1 identical before and after, and
  the pinned temp dir held the recorder's `viola-pty-watch/` directory (the redirect reached the test).
- The loss branch was not forced here; it is exercised by the reports it prints on the runner.
