# P7.1 light gate — the ASSERT per entry

`gate.py run --plan viola-0.1.0/chunks/2026-09-29-sideloaded-conpty/plan.md` (no `--skip`, no `--only`), trail
`gate-2026-09-29-sideloaded-conpty.json` in this run dir. Summary: `entries 29 · green 19 · red 2 (6,18) · recorded 0 ·
timeout 0 · not-run 8`.

- **Green by `expect` (19):** 1, 2, 4, 5, 7-17, 19-22.
- **Leg operator, re-verified by the recorded results (8):**
  - 3 (`h2-measure` clippy): valid only before the removal commit; `evidence/h2-measure-clippy.md` exit 0.
  - 23 (hygiene): `evidence/operator-pass.md` `refused 1` → renamed → `clean`.
  - 24, 28 (pushes): `fb78ddc..2d83718`, `224efc4..8f643f2` (`evidence/operator-pass.md`).
  - 25 (`ci.py`): red ci#36563179341, fixed at `224efc4`, then green ci#36563868040.
  - 26 (the `h2-loop:` read): both atoms held, `evidence/h2-with-without.md`.
  - 27 (the removal grep): no output.
  - 29 (`ci.py` on the final HEAD): green, ci#36566391084 on `8f643f2`, 15/15.
- **Entry 6 — `red — not this chunk's: basis → owner`.**
  - This run: integration 172 passed, 58 failed. The binaries are `channel_endpoint` 2 · `cli_fake_agent` 4 ·
    `cli_verify` 8 · `cli_instance_state` 7 · `cli_program_resolution` 1 · `cli_version_gate` 8 ·
    `hook_fail_open` 2 · `run_cli` 10 · `contract_ledger_probes` 1 · `hook_events` 3 · `tui_passthrough` 5 ·
    `tui_env_strip` 2, plus `conpty_sideload` 5.
  - Basis: the same binaries fail on the `fb78ddc` control on this host the same day (45 and 51 of 199,
    `chunks/2026-09-29-sideloaded-conpty/evidence/entry-6-not-this-chunk.md`).
  - `conpty_sideload`'s 5 all panic `viola never exited` at `tests/support/verify.rs:120`. That is the
    `stamped_home` fixture's `viola verify` step, reported through `tests/support/watch.rs`. It runs before any
    sideload code and is the step where the control fails 22-25 tests with the identical message. Filtered alone
    (entry 5), the binary is green.
  - Owner: the CARRY pinned at this wrap's P5 on `working-route.md:63` (the host's shared local integration reds,
    moving with the head; the operator's word, the overseer agreeing).
- **Entry 18 — the same record.**
  - linux-tests green: coverage 938/938, playwright 1/1.
  - windows-tests: coverage 926 passed, 49 failed. Those are 44 in the control-failing binaries above plus
    `conpty_sideload` 5, at the same fixture step.
  - Basis and owner as entry 6.
