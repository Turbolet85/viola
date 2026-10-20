# Codebase Research — 2026-10-19-newer-claude-cli-rows

## Scope
- **Depth:** deep · **Reads:** 19 · **Globs/Greps:** 21
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full (loaded with the touched files),
  `## Session Additions` included; applied: `--local-live` stamps `target/e2e-home/viola-live-<pid>/home` and never
  `~/.viola` (§boot / test-plan §3 `run`); the 2026-10-10 addition (the bare `claude` resolves to the newest CLI, so a
  live rig puts the wanted version's install dir first on its `PATH` and reads `claude --version` by path before its
  first start); the 2026-10-14 addition (a fake-agent side effect goes after the turn's screen write). Also
  `.claude/rules/testing.md` in full: the 2026-09-28 timing-red rule (measure the phases on the runner that failed,
  timing-only pushes allowed, never raise a bound) and the 2026-10-06 extension (a stamped-home case sized against the
  `mutants` profile's 10 s kill).
- **Platform issues consulted:** `gh search issues --repo actions/runner-images "macos-26 arm64"` → ten open issues,
  one on the failing image's own version (#14940 "macos-26-arm64 20261013.0424.1 broke GHC (linking phase)", a linker
  break, not a process stall); `"macOS runner slow process spawn"` → no result; `"macos-15 performance degradation"`
  → #11760 (closed 2025-03-14, a build-time regression on macOS 15). None reports a process that starts and then writes
  nothing for 11 s. The failing job ran image `macos-26-arm64` 20261013.0424.1, provisioner 20261012.601 (job log).
- **External inputs:**
  - `inputs#I1` — the overseer's phase directive: fold the macOS readiness flake; live starts are the founder's cap
    per chunk, a P4 fork in the away form; the host's CLI pin facts.
  - `inputs#I2` — the invocation: the founder is away, his forks go in the away form.
  - `inputs#I3` — this research's own reads outside the repository (R1–R5): the two Run B transcripts, the overseer
    home's stamps, the 2.1.302 bundle excerpts, the macOS run's attempt-1 artifacts.
  - The switch chunk's snapshots, cited by path:
    - `chunks/2026-10-19-overseer-switch-to-viola/inputs/I20-verify-2.1.302-20261019.out.txt` (the 20/4 verify);
    - `chunks/2026-10-19-overseer-switch-to-viola/inputs/I16-relay-switch-step12.md.txt` (one verify = starts 9–14).

## Files inspected
- `crates/viola-agent-claude/src/ledger.rs` (160–215, 800–880, 990–1040) — the four rows' predicates. `tag-escaping`
  and `paste-newline` are `pasted(trusted, PROBE_*_PASTE).is_some()` (`ledger.rs:1021`, `:1024`).
  - `local-command-clear` is `local_command_cleared` (`ledger.rs:830`). Its `clear_start` searches only from
    `pasted(trusted, PROBE_TAG_PASTE)` (`ledger.rs:817-824`), so it cannot pass while the tag-like paste has no
    capture.
  - `local-command-compact` is `local_command_compacted` (`ledger.rs:853`). Its `compact_start` searches only from
    `pasted(trusted, PROBE_NEWLINE_PASTE)` (`ledger.rs:843-849`), so it cannot pass while the newline paste has no
    capture.
  - So `/clear`'s and `/compact`'s rows are anchored on the two pastes before them.
- `src/cmd/verify/typed.rs` (1–160, 378–447, 650–680, 795–815) — Run B's `framing_turns` (`typed.rs:388`) types the
  long, tag-like and newline pastes, then `/compact`, then `/clear`.
  - Before each paste it needs `input_box_up(rows)`, and after each turn `rows = run.box_wait(stopped)`.
  - "The first wait that fails ends them" (`typed.rs:386`).
  - `box_wait` (`typed.rs:658`) settles only on a quiet screen with a row holding any `SIGNATURES` literal, bounded by
    `PROBE_DEADLINE` (`src/cmd/verify.rs:42`, 120 s; `typed.rs:801-809`).
  - A `None` from it makes the next `input_box_up(None)` false and ends Run B's pastes with no key typed.
- `crates/viola-agent-claude/src/screen.rs` (1–60, 405–430) — `SIGNATURES.input_box = ["for agents"]`
  (`screen.rs:39`), the one compiled input-box literal. `GATE_MAX_WAIT` 8.5 s, `QUIET_PERIOD` 300 ms. A unit case pins
  the row `"  ⏸ manual mode on · ← for agents"` (`screen.rs:422`).
- `src/run/gate.rs` (500–640) — `run`'s pre-send readiness gate takes `Some(&SIGNATURES)` on a verified CLI
  (`gate.rs:508`) and waits for the input-box literal up to `GATE_MAX_WAIT`. Past that, `send` is refused
  `input-not-ready` (`send.rs:666`, the `…without_a_literal_waits_for_the_input_box_to_the_bound` case).
- `src/cmd/run.rs` (213–252, 397–418) — `start`'s order between the wrapper's own `process-start` and the version gate:
  1. `resolve_program`
  2. `canonical_home`
  3. `collision_check`
  4. `pin_and_plugin` (`pin_exe`: the whole exe read, SHA-256, `file_key` of an existing pin, else a
     `replace_private_shared` copy — `crates/viola-state/src/pin.rs:71-92`; then the plugin files rewritten)
  5. `statusline_command`
  6. `plan_strip`
  7. `version_gate` (whose first line is `process-start{subject:"version-probe"}`)
- `tests/support/home.rs` (595–640) — `wait_ready` (`home.rs:615`) polls `Starts` (wrapper · child · receipt), then
  the snapshot, then a fresh beat. It is bounded by `WITHIN` = 7 s (`tests/support/watch.rs:14`) from the spawn, and
  panics `wrapper <name> not ready` with the watch report at `watch.rs:54`.
- `tests/cross_list_rows.rs` (19–30, 70–100, 225–300) — the two failing cases boot `builder` from `stamped_home`
  through `support::events::boot` and `builder_with_a_dialog`.
- `fixtures/claude/2.1.287/Screen.turn.json` — the one recorded turn screen. Its one kept row is
  `"  ⏸ manual mode on · ← for agents"`: the mode word and the hint share the footer row on 2.1.287.
- `src/bin/viola-fake-agent.rs` (98, 142, 187–195, 733) — screens come from `Screen.<phase>.json` of the set;
  `--tag-turn-screen <phase>` swaps the screen drawn after the tag-like turn. This is the precedent for a test-only
  screen switch.
- `crates/viola-agent-claude/src/hook.rs` (200–245, 635–670) — `prompt_text` = `unescape_tags(unwrap_pastes(raw))`, and
  `typed_text`'s CR/LF rule. These are unchanged by anything this research found.

## Graph impact (from the code-graph query; trace `.andromeda/runs/2026-10-19T23-10-05-phase/tree-query-2026-10-19-newer-claude-cli-rows.json`)
- **`SIGNATURES`** — 16 references:
  - `crates/viola-agent-claude/src/ledger.rs:16`, `:984`, `:1007`, `:1423`;
  - `crates/viola-agent-claude/src/screen.rs:414`, `:416`, `:428`, `:454`;
  - `src/cmd/run.rs:646`;
  - `src/cmd/verify/typed.rs:35`, `:338`, `:639`, `:671`;
  - `src/run/gate.rs:508`;
  - `src/run/send/test_support.rs:10`, `:90`.

  A change to the literal set reaches verify's Run A/B/C/D settles and `run`'s gate alike.
- **`box_wait`** — `src/cmd/verify/typed.rs:403`, `:418` (framing_turns) and `:806` (the `Run::box_wait` wrapper), plus
  unit cases at `:976`–`:1010`.
- **`input_box_up`** — `typed.rs:145` (the trusted run's start), `:392`, `:405`, `:418` (framing_turns), `:814`, and
  unit cases at `:896`–`:901`.
- **`pasted`** — eight call sites in `ledger.rs` (`:821`, `:838`, `:844`, `:862`, `:881`, `:1020`, `:1021`, `:1024`).
- **`pin_exe`** — production callers `src/cmd/run.rs:402` and `src/cmd/verify.rs:157`, plus unit cases in
  `crates/viola-state/src/pin.rs`.
- **`wait_ready`** — the test helper `tests/support/home.rs:552`, `:606`; the harness `crates/viola-e2e/src/harness/boot.rs`
  (four sites); the product gate `src/run/gate.rs` and `src/run/send.rs:128` (a different function of the same name).
- (Lines are the graph's 0-indexed `line` + 1.)

## Patterns detected
- **The four failures are one stop, measured, not four** (`inputs#I3` R1, R2):
  - On 2.1.302, Run B's transcript holds two user turns only (the probe prompt and the long paste). The long paste's
    turn ended 18:18:53.356Z.
  - Run C's project dir appeared 18:20:55Z, ~122 s later. That gap is `box_wait`'s 120 s `PROBE_DEADLINE` running out
    on the screen after the long paste's turn (`typed.rs:402`).
  - So the tag-like, newline, `/compact` and `/clear` pastes were never typed. `tag-escaping` and `paste-newline` find
    no capture. `local-command-clear` and `local-command-compact` cannot pass by construction, being anchored on those
    two captures (`ledger.rs:817-849`).
  - On 2.1.296 the tag-like paste went 4.9 s after the same turn's end.
  - The overseer's stamp agrees: 2.1.302's `compact_latency_ms` is null, while 2.1.296 reads 3648 (`inputs#I3` R3).
- **The cause in the CLI, read from the shipped bundle (not from a live screen)** (`inputs#I3` R4):
  - 2.1.296 draws the footer hint `" for agents"` unconditionally.
  - 2.1.302 draws it only while `restingHintExpired` is false. That flag is the app state's `agentsHintExpired`, set
    by `xxe()` from a 10 s value (`sIo=1e4`) over `Fc() != null`. `Fc` is
    `surfaceCapabilities.attacherCaps()`, which is set by an incoming `{type:"attacher-caps"}` message.
  - The literal `SIGNATURES.input_box` names is therefore not a stable input-box signature on 2.1.302. Unmeasured:
    - what makes `attacherCaps` non-null in Run B (its one `claude agents --json` read is the nearest candidate);
    - which of the bundle's twelve `Hr` definitions `xxe` calls, so whether the 10 s counts from the attach or from
      the start.
- **The same literal gates `send`**: `run`'s readiness gate reads `SIGNATURES` on a verified CLI (`src/run/gate.rs:508`).
  A 2.1.302 home stamped verified with today's literal set would refuse a `send` `input-not-ready` once the hint is gone
  (`src/run/send.rs:666`'s case is that path). The fix is a product reliance (scope's way (b)), not a probe fix alone.
- **`input-box-signature` passed on 2.1.302 by timing**: `check(InputBoxSignature)` reads the `ready` and `turn`
  screens only (`ledger.rs:1006-1008`). Both settle well inside 10 s (`ready_settle_ms` 1058, `turn_settle_ms` 317,
  `inputs#I3` R3). So the row cannot see a literal that disappears later.
- **The macOS readiness stall sits inside `run.start`, before the version gate** (`inputs#I3` R5):
  - Both failing wrappers (pids 16375 and 16377, started 40 ms apart at 20:44:56.3Z) wrote `process-start{self}`,
    then nothing for the rest of the test (≥ 11 s): no `version-probe` line, which in every other run of that minute
    followed within 33–47 ms.
  - The stalled span is `start`'s steps 1–6 above. The phase is unmeasured.
  - `pin_exe`'s whole-exe read, hash and copy is the heaviest step (the debug `viola` is ~100 MB per the handoff;
    under `llvm-cov` instrumented). That makes it the first candidate, a hypothesis.
  - The four `cross_list_rows` cases and a 47.8 s `verify_window_` case ran at once.

## Conventions to follow
- **Compiled literals only** (`screen.rs:27-29`): a new input-box literal is a measured, compiled `&'static str`.
  It is never built from screen text. Its unit case pins the row it matches (`screen.rs:422`).
- **A test-only fake-agent switch reproduces a recorded CLI shape** (`--tag-turn-screen`, `src/bin/viola-fake-agent.rs:142`):
  a capped option, named in the verification-harness rule's fake-agent paragraph.
- **verify's literals are pinned in the tests** (`tests/cli_verify.rs`, per-OS summaries such as `19 pass  5 fail` /
  `19 pass  2 fail`). Expected row ids and counts are literals, never imported (testing.md §What to assert).
- **A stamped fixture set sits on exactly one list** of `tests/contract_ledger_probes.rs` (stamped / drift-only).
- **A timing red is measured on the runner that failed** before any change (testing.md 2026-09-28).

## New files to create
- derived `fixtures/claude/2.1.302/*` by `viola verify --record` — the 2.1.302 set (spine, dialog, framing, statusline, resume, compact, agents and screen variants), recorded only at 0 fail, after the live round's fix
- `viola-0.1.0/chunks/2026-10-19-newer-claude-cli-rows/evidence/` — the live round's readings and the macOS phase measurement

## Files to modify
- `crates/viola-agent-claude/src/screen.rs` — gated: the input-box literal set gains the literal 2.1.302's resting input box keeps (measured), and its unit and verdict cases over the measured row
- `crates/viola-agent-claude/src/ledger.rs` — `TypedRun` gains the long turn's settled screen; the `input-box-signature` row reads it beside `ready` and `turn`; its unit cases
- `src/cmd/verify/typed.rs` — Run B keeps the screen `box_wait` settled on after the long paste's turn (or none) in `TypedRun`
- `src/bin/viola-fake-agent.rs` — the test-only `--long-turn-screen <phase>` switch, drawn after the long paste's turn as `--tag-turn-screen` is after the tag-like turn
- `tests/cli_verify.rs` — the reproduction case and the moved per-OS literals
- `tests/contract_ledger_probes.rs` — gated: the 2.1.302 set placed on the stamped list
- `tests/support/home.rs` — `wait_ready`'s deadline appends the stalled wrapper's state to the watch report before the panic (macOS `sample`, Linux `/proc` wchan); and, provisional, the macOS stall's fix site if its cause lies in the test-side boot
- `crates/viola-state/src/pin.rs` — provisional: the macOS stall's fix site if its cause is the pinned copy
- `src/cmd/run.rs` — provisional: the macOS stall's fix site if its cause lies elsewhere in `start`
- `.config/nextest.toml` — provisional: the `ci` 20 s kill line's verify-driven list if a new case drives verify

## Sweeps
- `"for agents"` over `crates/ src/ tests/ fixtures/` (`/usr/bin/grep -rn -F 'for agents'`, run at implement before the literal change): not yet run — research read its definition and the 16 graph references, never a text sweep; implement runs it and records each hit.

## Open questions
- Which compiled literal does 2.1.302's resting input box keep once the hint has expired (the 2.1.287 footer row also holds the mode word, `Screen.turn.json`), and does it stay true of 2.1.287's recorded rows and 2.1.296? → blocks: implementation-scope (a live 2.1.302 screen; the founder's cap)
- What attaches in Run B, and does a `viola list` (its `claude agents --json`) expire the hint in a product session? → blocks: implementation-scope (the live round reads it; it decides whether `send`'s exposure is every 2.1.302 session or only an attached one)
- Which `run.start` phase stalls on `macos-latest` → blocks: implementation-scope (measured on that runner by timing-only pushes before any change; the fix site is provisional until then)
