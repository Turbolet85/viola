old:   `cfg(unix)`, `cfg(target_os = …)`) carries the reason its behaviour is one OS's, named where it stands. [inferred: 14
  files under `tests/` carry a `cfg(windows)` today (`command grep -rl 'cfg(windows)' tests/`); which of their cases read a
  Windows-only behaviour and which are a parity gap is P3's reading]
new:   `cfg(unix)`, `cfg(target_os = …)`) carries the reason its behaviour is one OS's, named where it stands. Verified at P3
  against the last green run's JUnit (ci#41339639367): ubuntu 3446, macOS 3400, Windows 3376 cases, 0 failed, 0 skipped;
  ubuntu-not-macOS is the macOS no-peer-pid carve-out (44) and four Linux-only cases each naming its reader or host fact;
  the one shared subject run on one OS only is `run_child_ends_when_its_wrapper_is_terminated` (`cfg(windows)`), and the
  one shared fake-agent behaviour read on Linux only is `fake_agent_exit_outlives_a_hangup_that_lands_while_it_exits`
  (its `/proc` reader); 10 of the 50 OS-gated test functions under `tests/` carry no reason line (`doc:…/research`).
old:   perf and lint jobs read green on `ubuntu-latest` and `macos-latest` beside `windows-2025`. [inferred: `ci.yml` already
  runs the `test`, `perf` and `lint` jobs over the three OSes (its `matrix.os` lines 18, 161, 294); what "green on both"
  still lacks is P3's reading against the last green run]
new:   perf and lint jobs read green on `ubuntu-latest` and `macos-latest` beside `windows-2025`. Verified at P3: every job of
  ci#41339639367 on `79ae3d69` reads success, `test`, `perf` and `lint` on all three OSes included; this chunk keeps them
  green on its own pushed sha.
old: - One diagnostics schema: the process-log lines (`diag-line.v1.json`), `events.ndjson`, the snapshots and every `--json`
  document carry the same fields and the same closed words on every OS; a field or word that one OS cannot fill reads its
  documented null or closed word, never a missing key or an OS-only word. [inferred: whether any schema or writer differs
  per OS today is P3's reading]
new: - [premise-corrected: obs-plan §3's null encoding makes a null `corr` / `instance` a key absence, so "never a missing key"
  contradicts the schema; `schemas/diag-line.v1.json` is one file G4 reads on every OS job] One diagnostics schema: the
  process-log lines (`diag-line.v1.json`, `diag-detail.v1.json`), `events.ndjson`, the snapshots and every `--json` document
  hold to the same schema and the same closed words on every OS, a null encoded as the schema encodes it (key absence on
  process-log lines) and no OS-only key or word; OS-dependent values are words of the schema's own fields (`pty_backend`
  `conpty-sideload` · `conpty` · `openpty`; `sideload_fallback` written only on a Windows degrade). Verified at P3: no
  writer or schema differs per OS, and G4 read green on all three OS jobs at ci#41339639367.
old: - [inferred] That CARRY's mechanism, its marker kept verbatim: "hypothesis: on such a terminal a live session loses the
  wheel at its start, as start 6 did on foot." Its measurement needs a live terminal and a real `claude`; this chunk has
  no live session (`inputs#I1` item 5), so what of it is doable under the fake agent is P4's.
new: - [unverifiable at HEAD: its measurement needs a live terminal and a real `claude`, and this chunk has none, `inputs#I1`
  item 5; the fake agent cannot answer it, since the list joins a shape only when measured on a real terminal on the
  founder's word] That CARRY's mechanism, its marker kept verbatim: "hypothesis: on such a terminal a live session loses
  the wheel at its start, as start 6 did on foot." It stays a hypothesis and stays carried.
old:   `run_child_ends_when_its_wrapper_is_terminated` (`tests/run_cli.rs`) stays `cfg(windows)`, and no real CLI was read on
  Unix.
new:   `run_child_ends_when_its_wrapper_is_terminated` (`tests/run_cli.rs`) stays `cfg(windows)`, and no real CLI was read on
  Unix. Spot-checked at HEAD: the kill-path cases ran green on both Unix runners at ci#41339639367; and at P3 a SIGKILL of
  a harness-booted wrapper on the Linux dev host left its fake-agent child gone at the first 100 ms read
  (`doc:…/research`, Patterns detected), so the standalone pin can run on Unix with its terminate step per OS. The real-CLI
  Unix reading needs a live session and stays carried with the tab-close leg.
old:   proved it at the unit tier only (`board_row_style_a_stale_row_is_dim_end_to_end_and_only_dialog_is_amber`), because the
  `mutants` profile's 10 s kill leaves no room beside the ~4 s stamp and the 5 s stale threshold; its first real consumer
  is this entry's Unix fake-agent suite.
new:   proved it at the unit tier only (`board_row_style_a_stale_row_is_dim_end_to_end_and_only_dialog_is_amber`), because the
  `mutants` profile's 10 s kill leaves no room beside the ~4 s stamp and the 5 s stale threshold; its first real consumer
  is this entry's Unix fake-agent suite.
- [premise-corrected: the 5 s threshold need not be waited out; `run_refuses_a_stale_name` (`tests/cli_instance_state.rs`)
  makes a stale wrapper at once by freezing it (`Frozen`, `SIGSTOP`) and back-dating its heartbeat 60 s] That CARRY's
  mechanism, re-derived at HEAD: the witness fits the `mutants` profile's 10 s kill with no wait beyond the boot.
old:   whether it stands met, or this chunk can claim nothing of it, is P4's. [inferred: CLAUDE.md reads "the first live run was
  on the Linux dev host"; which run, and whether it confirms the fake agent's Unix behaviour as `v1-53` asks, is P3's
  reading]
new:   whether it stands met, or this chunk can claim nothing of it, is P4's. Verified at P3: the earlier live Linux run is
  chunk 2026-10-08-first-live-test-and-self-drive's live round on the dev host (eight live `viola run` starts on `claude`
  2.1.287, `round: COMPLETE`, stamped `2.1.287  17 pass  0 fail`, its readers over `evidence/live-run.ndjson` and
  `evidence/live-sessions.ndjson` green); whether it confirms the fake agent's Unix behaviour as `v1-53` asks is P4's
  question.
old: - `v1-40`'s integration-tier witness is this entry's CARRY; whether `v1-40` itself is open or verified is P4's reading.
new: - `v1-40`'s integration-tier witness is this entry's CARRY; `v1-40` itself reads verified (chunk
  2026-10-17-cli-output-discipline), so the witness adds evidence and claims nothing.
