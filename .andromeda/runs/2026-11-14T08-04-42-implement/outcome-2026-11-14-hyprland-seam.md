# Implement outcome — 2026-11-14-hyprland-seam

**Outcome: green + smoke.** L1 and L2 are recorded, each on the operator's own word. L2 measured the plan's
dispatch argv wrong for 0.56.2, and the argv is corrected in the same change (plan step 10). Steps 1–13 are built.
Every non-operator entry is green, all four controls hold, and the smoke entries are green. Next: the operator pass,
per the directive in `relay-2.md` (`inputs#I6`): hygiene (entry 36), land, the pre-CI commit, the push (entry 37) and
the `ci.py conclusion` read (entry 38). Then `/andromeda-wrap-session`.

## L1 (`evidence/operator-3-L1.txt`; relay `relay-1.md`, `inputs#I5`)
- `--operator 3`: `3 probe green · exit 0 · 0.38s · 0 B → 3.log · history: unmoved`.
- It recorded `fixtures/hyprland/0.56.2/`:
  - `Clients`, `ActiveWindow`, `Workspaces` and `Version` `.recorded-1.json`;
  - `Version.absent-1.json` (exit 1; the reply is not JSON, so it was replaced by the placeholder).
- Every string in every recorded `stdout` is a window address, the tag `v0.56.2` or the placeholder. Everything
  else is numbers and booleans.

## L2 (`evidence/operator-4-5-L2.txt`; relay `relay-2.md`, `inputs#I6`)
- **Firing 1:** entry 4 green (`false`). Entry 5 green, but both dispatches recorded exit 7 and the placeholder.
- **The raw reply**, read by hand from the same granted dispatch: `hyprctl dispatch workspace 6` exits 7 with
  `error: [string "return hl.dispatch(workspace 6)"]:1: ')' expected near '6'`. 0.56.2 runs `dispatch <args>` as
  the Lua `hl.dispatch(<args>)`.
- **The Lua forms**, read from the host's own files:
  - `/usr/share/hypr/hyprland.lua` binds `hl.dsp.focus({ workspace = i })`;
  - the Hyprland binary's message lists `window` among `hl.focus`'s arguments.
- **Both corrected forms answer `ok
`, exit 0**, on the workspace and window already shown and focused:
  `hl.dsp.focus({ workspace = 6 })` and `hl.dsp.focus({ window = "address:0x555a9c521790" })`.
- **Correction, same change:** `Request::args()` now gives `dispatch` plus one Lua dispatcher expression, built only
  from a closed `Address` or an `i32`. Step 3's success text `ok` stands, measured.
- **Firing 2:** entries 4 and 5 green. Both dispatch fixtures record exit 0 and `ok
` under the corrected argv.
- The session was left unlocked; nothing locked it.

## Gates (trail `.andromeda/runs/2026-11-14T08-04-42-implement/gate-2026-11-14-hyprland-seam.json`)
- **Firing 1, the whole block:** `entries 38 · green 24 · red 7 (15,17,18,20,21,23,35) · recorded 1 · timeout 0 · not-run 6`.
- **Fix:** 15, 20 and 21 failed on a single test, `reply::tests::version_prop_parse_never_panics_and_reads_back_its_tag`.
  The tag `v00.0.0` reads as version 0.0.0 but prints back as `v0.0.0`. The property compared the tag text; it now
  compares versions. The case `leading_zeros` is pinned, and the seed proptest wrote to
  `crates/viola-hyprland/proptest-regressions/reply.txt` is kept.
- **`--only 6,7,15,20,21`:** `entries 38 · green 5 · red 0`.
- **`--entry 35` (pre-push), before L1:** red only on the empty walk, `hyprland_fixtures_pass_hygiene`.
- **Firing 2, the whole block after L1 and step 11:** `entries 38 · green 31 · red 0 · recorded 1 · timeout 0 · not-run 6`
  (the not-run six are the operator entries 3, 4, 5, 36, 37, 38). Pre-push (35) is green. 17
  (`contract_hyprland_seam`) is green: 5/5, and its walks are non-empty.
- **Firing 3, after L2 and the argv correction:** `entries 38 · green 31 · red 0 · recorded 1 · timeout 0 · not-run 6`
  (the not-run six are the operator entries).
- **Smoke:** entries 29–34 (cleanup, boot, status, G2, schema-check, cleanup) are green in both firings; the last
  cleanup read `processes_gone` and `endpoint_gone` true. P3 ran as P2 gates and was not re-run.
- **Delta rule:** nothing deferred. Every changed language's gates ran.

## Controls (`control-1.toml` in the run dir; evidence `evidence/control-1.txt`, `-2.txt`, `-3.txt`)
- Firing 3, after L2: `control: 4 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed` — each
  control's own case `FAIL` in its removed log (`evidence/control-3.txt`).
- Firing 1, before L1: `control: 3 holds · 0 timeout · 0 does not hold · 1 unreproduced` (hygiene-title's entry 18
  was red with the guard in place, on the empty walk).
- Firing 2, after L1: `control: 4 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed`.
  - oversize — `FAIL [   0.269s] (82/82) viola-hyprland reply::tests::parse_clients_takes_max_frame_bytes_and_refuses_one_more_as_oversize`
  - deadline-kill — `FAIL [   0.714s] (82/82) viola-hyprland read::tests::read_past_the_deadline_kills_and_reaps_its_child`
  - title-scrub — `FAIL [   0.005s] (6/7) viola-e2e::bin/viola-hypr-tap tests::record_replaces_every_string_off_the_allowlist`
  - hygiene-title — `FAIL [   0.032s] ( 91/119) viola::contract_fixture_hygiene planted_hyprland_lastwindowtitle_is_rejected`

## Files changed
- **New:**
  - `crates/viola-hyprland/{Cargo.toml, src/lib.rs, src/reply.rs, src/read.rs, proptest-regressions/reply.txt}`
  - `src/bin/viola-fake-agent/hyprctl.rs`
  - `crates/viola-e2e/src/bin/viola-hypr-tap.rs`
  - `schemas/hyprland-fixture.v1.json`
  - `fuzz/fuzz_targets/hyprctl_json.rs`
  - `fuzz/corpus/hyprctl_json/` (9 synthetic seeds)
  - `fixtures/hyprland/0.56.2/` (L1's five recorded files and L2's `FocusWindow` and `Workspace`)
  - `tests/contract_hyprland_seam.rs` (step 11, with its `[[test]]` table in `Cargo.toml`)
- **Modified:**
  - `Cargo.toml`, `Cargo.lock`
  - `scripts/sync-crates.txt`
  - `src/obs.rs` (eight targets)
  - `schemas/diag-line.v1.json` (`hyprctl-probe`, `hyprctl-json`)
  - `src/bin/viola-fake-agent.rs`
  - `tests/support/fake.rs` (`hyprctl_bin`)
  - `tests/contract_bounded_readers.rs`, `tests/contract_error_bodies.rs`, `tests/contract_fixture_hygiene.rs`
  - `crates/viola-e2e/Cargo.toml`
  - `fuzz/Cargo.toml`, `fuzz/Cargo.lock` (the `veil` edge restored and the new crate added; `--locked` entry 24 green)
  - `.gitattributes`
- **Out-of-list edit** (scope record, rev 1): `src/bin/viola-fake-agent/test_cases_modes.rs` · in-intent ·
  serves step 7 · self — the fake `hyprctl`'s unit test.
- **Scope read:** `scope: clean — changed 39 · listed 38 · recorded 1 (companion 0 · mechanical 0 · in-intent 1 · widening 0) · absorbed 15 · excluded 230`.

## Deviations from the plan, with justifications
1. **Deadline witness (step 5).** The case plants a 0700 script that writes its pid and then execs
   `/bin/sleep 30`, instead of spawning `/bin/sleep` directly. The read's closed argv (`-j clients`) makes
   `sleep` exit at once, so the deadline would never fire. The pid is read through `/proc/<pid>`, and the case
   stays bounded below 7 s.
2. **Fields the parsers read (step 3).** Beyond the plan's list, they read `swallowing`, `grouped` and
   `workspaces[].lastwindow` as addresses. Step 8's allowlist keeps those keys, and step 8 says a key joins only
   where a parser of step 3 reads it.
3. **Oversize before exit status (step 5).** The read checks for an over-cap reply before it checks the exit
   status, and reports it as `oversize`. A child whose reply runs past the cap is cut off by the read, which
   makes its exit (SIGPIPE, or the deadline kill) meaningless.
4. **The recorded-set unit case (step 3).** `every_recorded_reply_parses_through_its_own_parser` walks
   `fixtures/hyprland/` as none while it is absent. The empty walk's refusal stays with
   `hyprland_fixtures_pass_hygiene` and step 11's contract, so the unit entry can be green before L1.
5. **Environment-strip case.** It removes `CARGO_MANIFEST_DIR` rather than `PATH`, because dash sets a default
   `PATH` when it is unset.
6. **Control cases (step 14).** The plan wrote each case as `<binary-id> <test path>`, and gate v1.33 refused
   that form at exit 2. The controls file names the test path alone.
7. **The `HyprlandError` census row (step 12).** It is a `NOT_ERROR_BODIES` row (`pub enum HyprlandError {`,
   "no verb calls the seam yet, so no surface writes one"), and `crates/viola-hyprland/src` joins both censuses'
   `PRODUCT_ROOTS`.
8. **The dispatch argv (step 2), corrected by L2's measurement as step 10 directs.** Plan:
   `dispatch focuswindow address:<addr>` / `dispatch workspace <id>`. Measured on 0.56.2:
   `dispatch` + `hl.dsp.focus({ window = "address:<addr>" })` / `dispatch` + `hl.dsp.focus({ workspace = <id> })`.
   The pins moved in `lib.rs`, `viola-hypr-tap.rs` and step 11's argv reader. Measured for the wrap: a refused
   dispatch exits 7 with an `error:` text, and the read maps any non-zero exit to `unreachable` (step 4's word
   mapping). So `refused` reads only on an exit-0 reply other than `ok`.
9. **Fixture names.** Recorded fixtures are named by the PascalCase stem of the request word (`Clients`,
   `ActiveWindow`, `Workspaces`, `Version`, `FocusWindow`, `Workspace`), as in `Agents.recorded-1.json`. The
   no-instance reply is `Version.absent-1.json`.

## Surfaced
None. No spec↔reality gap was found, and no capability is claimed: the matrix has no row for this chunk.

## Process hygiene
| process | started by | final state |
|---|---|---|
| `viola` ×8 (`~/dev/projects/additional/viola-lab/prototype/…/viola run …`/`wait …`, `~/.viola/bin/0.1.0-…/viola mcp`, `~/.local/viola-product/bin/viola wait …`) | NOT DERIVED — another tree's bridge; not this run's | left running: not this run's — their owner stops them |
| harness session `p-hypr-smoke` (entries 30–34) | this run (a block entry) | terminated — entry 34 read `processes_gone` and `endpoint_gone` true |
| the control clone's firings (×2) | this run (`gate.py control`) | terminated — `clone removed` |
| L1's and L2's `hyprctl` reads, and the three hand-issued dispatches | this run (entries 3–5, `--operator`; by hand) | terminated — the census reads no `hyprctl` |

census (second reading): `census: 8 running` (comm viola, viola-fake-agent, hyprctl, viola-harness, viola-hypr-tap;
args projects/viola/target). All eight are the other tree's bridge or the product install: `viola-lab/prototype`
runs and waits, `~/.viola/bin/… mcp`, `~/.local/viola-product/bin/viola wait`. None is this repository's. No
`hyprctl` or `viola-hypr-tap` survives L1.

## Decisions & corrections
- The operator's directive (relayed in the invocation): no live `claude` start; L1 and L2 not granted; stop at
  step 10; the controls file carries a `case` per control; journal each long run before a wait (`journal.md`);
  after the recordings, the operator pass with the `ci.py conclusion` read.
- Guard refusals, each re-issued once:
  - bash-guard refused a `cd` into the skill's references dir; the file was read with Read.
  - bash-guard refused a `cat` heredoc append; it went through Edit.
- `gate.py control` refused `case = "<binary-id> <name>"` at exit 2. The file was corrected and re-fired once.
- The operator's own word for L1, «Да, L1 разрешаю», 08:35Z, was relayed by the overseer (`relay-1.md`, `inputs#I5`).
  L2 was not granted: `hyprctl locked` read true and the operator was away. The directive: stop before entry 4;
  no commit, no push.
- The operator's own word for L2 («Разблокировала», 09:04Z, relayed by the overseer; `relay-2.md`, `inputs#I6`):
  L2 granted; the session stays unlocked and is not to be locked. Then re-run the block and the controls, then the
  operator pass with the `ci.py` read.
- Three dispatches were issued by hand outside the tool (the plan's argv once, then the two corrected forms once
  each) to read the raw reply the recorder replaces. Each was the granted L2 dispatch on the window and workspace
  already focused and shown; the corrected form was then re-recorded through entry 5.
