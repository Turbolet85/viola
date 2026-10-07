# Report — 2026-10-07-send-waits-out-the-paste-hint

**Chunk:** Send waits out the paste hint — on a verified CLI the gate waits for the input box on a quiet literal-less screen, the bound 8.5 s, one by-path re-verify of claude 2.1.287
**Date:** 2026-10-07
**Commits:** `e574e73` chore(2026-10-07-send-waits-out-the-paste-hint): operator pre-CI commit, for the run this chunk's verdict reads (the only commit since the last wrap; basis `git log --format='%h %s' 56e67bb3..HEAD`)

Authority, as the overseer directed at this wrap (the run dir's `directive.md`): the wait and the 8.5 s bound are
the founder's live ruling of 2026-10-07T10:29Z, relayed by the overseer. The second read of the wheel and the
running turn after the wait is the overseer's technical answer of 2026-10-07T12:10Z (inputs#I2), not the founder's.

## Changes (structured — detectors read this)
- **Files:** ten source and test files, exactly research's ten (`gate.py scope`: changed 10 · listed 10):
  `crates/viola-agent-claude/src/screen.rs`, `crates/viola-agent-claude/src/ledger.rs` (tests only),
  `src/run/gate.rs` (tests only), `src/run/send.rs`, `src/cmd/verify/typed.rs` (tests only),
  `src/bin/viola-fake-agent.rs`, `tests/cli_send.rs`, `tests/cli_verify.rs`, `tests/support/verify.rs` (one
  comment), `.config/nextest.toml`. New: the chunk's `evidence/` (seven files) and `inputs/`.
- **Symbols / APIs:**
  - `viola_agent_claude::screen::GATE_MAX_WAIT` is 8 500 ms (was 5 s). Its readers are unchanged and read it by
    name: `Screen::verdict` (the not-quiet arm and the new waiting arm), the `quiet-period` ledger row's check
    (`ledger.rs`, both measured settles must be at or under it), and `viola verify`'s settle fallback
    (`src/cmd/verify/typed.rs` `settled`). `QUIET_PERIOD` (300 ms) and `CONFIRM_WINDOW_FALLBACK` (10 s) are
    unchanged.
  - `Screen::verdict`, with signatures (a verified CLI): a quiet screen with no modal row and no input-box row is
    now `Wait` while less than `GATE_MAX_WAIT` has passed since the send entered the gate, and `InputNotReady`
    from the bound on. It was `InputNotReady` at the first quiet instant. A modal row and a poisoned screen still
    refuse at once. Without signatures (an unverified CLI) a quiet screen is still `Ready` with no row read, and
    a screen that never goes quiet is refused at 8.5 s where it was 5 s. Sole product caller: `Gate::wait_ready`
    (`src/run/gate.rs`), not edited.
  - The wrapper's `send` (`src/run/send.rs`): after the gate returns `Ready` it reads the wheel again, then the
    running turn again, in that order. A human-held wheel refuses `human-typing` (with the wheel's own detail);
    else a running turn refuses `not-delivered` / `turn-running`. Neither issues `send-issued` nor pastes. The
    reads at the send's arrival, ahead of the gate, are unchanged. Sole product caller of `Gate::wait_ready`.
  - A `send` on a verified CLI issued while the paste hint stands is now delivered once the input box returns
    (was refused `not-delivered` / `input-not-ready` 0.63 s in). A hint that outlasts the bound is refused
    `input-not-ready` at the bound, with nothing typed.
  - `send` blocks for at most 18.5 s: 8.5 s of gate plus the 10 s confirmation window (was 15 s).
  - The fake agent's test-only `--paste-hint-ms` cap (`PASTE_HINT_CAP_MS`) is 10 000 ms (was 8 000 ms). Its
    option count stays eight; no option was added.
  - No new IPC method, endpoint, event kind, socket, port, env var, flag, config key, crate, ledger row, compiled
    literal or fixture. The `input-not-ready` hint line in `src/human.rs` is unchanged.
- **Crates / modules:** changed `viola-agent-claude` (`screen`), the root bin (`run::send`; tests in `run::gate`
  and `cmd::verify::typed`; the fake agent bin). None added or removed.
- **Dependencies:** none added, none bumped (`Cargo.toml` and `Cargo.lock` byte-identical to the base: the
  preservation guard entry is green).
- **Schema / config:** none. No `obs_event!` site, field or detail was added (`schema-check` green over the booted
  home). `.config/nextest.toml` is under Harness / gate surface.
- **Spec-master edits:** none by this chunk before the wrap.
- **Counts / qualifiers moved** (each with where the masters state it; basis: the sweeps of this wrap's
  scratch helper over the seven masters and every `.andromeda/registries/**` file):
  - the gate's maximum wait 5 s → 8.5 s. `GATE_MAX_WAIT`: 6 hits, all in architecture (`:48` ×2, `:70`, `:91` ×3),
    0 elsewhere. The value `5 s` beside it stands at `architecture.md:48` and `:70`. The same value restated as
    "the gate's 5 s maximum" stands in the test-plan key file
    `registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10` and `:11`.
  - the fake agent's hold cap 8 000 ms → 10 000 ms: `architecture.md:355`, `test-plan.md:1077` (`paste-hint-ms`:
    architecture 1 line, test-plan 2 lines, the key file 0).
  - `[profile.ci]` `test(/verify_window_/)` 15 s × 3 (45 s) → 20 s × 3 (60 s); `[profile.mutants]` the same
    filter 15 s × 2 (30 s) → 15 s × 3 (45 s): the same key file, `:10` and `:11` (`verify_window_`: 4 hits there,
    1 in the test-plan body at `:654`).
  - the no-screen `verify_window_` case's floor "about 21 s" → 34.3 s measured on the dev host under every profile
    and 34.3 s to 35.3 s on the three CI legs: the same key file, `:10` and `:11`.
  - verify's hint case holds a cleared screen 9 s after the long paste's Stop (was 6 s), and read 13.1 s to
    13.7 s on the three CI legs (the key file says "about 10 s"): `test-plan.md:654` and the key file `:10`, `:11`.
  - test counts, each leg thirteen more than at the previous chunk: ubuntu 1 682 → 1 695, macOS 1 678 → 1 691,
    windows 1 706 → 1 719 (ci#37623727247 against ci#37609247992). No master states these totals (not swept: a
    total is not a claim a master carries).
- **Dev-tool versions:** none — cargo-nextest re-read at 0.9.146 on the dev host; the `claude` CLI re-read at
  2.1.287 by path (`2.1.287 (Claude Code)`, the round's version probe).
- **Harness / gate surface:** `.config/nextest.toml`, the two `verify_window_` overrides and their comments, as
  above. No `agent-run` verb, CI step, status or verdict shape changed (`scripts/` and `.github/` byte-identical to
  the base).
- **Cross-project / external claims:**
  - CI: ci#37623727247 on `e574e738f814`, `verdict: green`, checks 15/15, wall 397 s. The sha is the record: this
    wrap's own commit adds to that tree.
  - The live round against the external `claude` CLI 2.1.287, by path, fired once at 2026-10-07T12:44:07Z:
    `stamped 2.1.287  17 pass  0 fail`, `round: COMPLETE · legs fired 1/1`; `ready_settle_ms` 1 103 and
    `turn_settle_ms` 617 against the 8 500 ms bound (`evidence/reverify-round.md`).
  - `I1 · message: the operator (the overseer), the /andromeda-phase invocation arguments, 2026-10-07T11:51Z ·
    copy · n/a — a message has no live source` (cited).
  - `I2 · message: the overseer, the answers to the P4 fork round, 2026-10-07T12:10Z · copy · n/a` (cited).
  - `I3 · message: the operator (the overseer), the /andromeda-implement invocation arguments, 2026-10-07T12:22Z ·
    copy · n/a`. `inputs.py verify` read it UNCITED before this report; it is cited here: inputs#I3.
  - `inputs: 3 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 3 ·
    uncited 1 · unparsed 0`. No drift.
  - The wrap's own directive is in this wrap's run dir (`directive.md`); the inputs tool has no wrap step.
- **Reverted / negative API facts:** none shipped and reverted. Not built, by the plan's rejections: reading the
  hint's own text (a new compiled literal), a knob or env seam for the bound, an edit of `Gate::wait_ready`, a
  separate 5 s constant for verify, an end-to-end case blocking to the bound, a reworded hint line, and an
  interrupt of the gate's wait when the wheel moves (`Gate` still knows nothing of the wheel; the driver hears the
  refusal when the wait ends).
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - Plan step 6 and research's sweep "cases that turn by the constant" name two cases in
    `src/cmd/verify/typed.rs`. A third turned: `feed_past_the_frame_cap_poisons_and_settles_without_rows` read a
    settle at 6 000 ms on a poisoned screen, under the new bound. Evidence: the first whole-block gate run, red in
    the unit entry, the default selection and `pre-push` on that one case (`evidence/operator-pass.md`, the last
    bullet; the implement run's gate trail). Not a master's claim. Disposition owner: curation (the sweep hazard).
  - Plan step 10 says the receipt holds "the key's own receipt line". It holds two `key` lines: the Enter that
    submitted the long text, then the human's key (`evidence/hint-red-green.md`, the last section). Not a master's
    claim. Disposition owner: curation.
- **Expected amendments (from plan):** (the search for each: the wrap's scratch sweep, a regex per claim over the
  seven masters and every registry file, hits read by bounded window)
  - architecture [Screen Model], the bound and the waiting verdict — carried: Symbols / APIs bullets 1 and 2.
    Sites: `architecture.md:48` (`GATE_MAX_WAIT`: 2 of architecture's 6 hits; 0 in any other master or key file).
  - architecture [CLI Version Compatibility], the "decided, not built" sentences — carried: Symbols / APIs bullet 4.
    Sites: `architecture.md:91` (`not built`: architecture 1 hit; design-system 1, test-plan 3 and a11y-plan 4 hits
    are other subjects, read and left; `paste hint|paste again to expand`: architecture 4 hits at `:81`, `:91` ×2,
    `:355`).
  - architecture [Human Takeover / Wheel], the second reads and the retired residual — carried: Symbols / APIs
    bullet 3. Sites: `architecture.md:70` (`starts during the`: 1 hit). The refusal order restated at
    `architecture.md:136` (§Conventions) names the gate after `turn-running` and is the same claim's second site.
  - architecture [Delivery Confirmation], the longest block — carried: Symbols / APIs bullet 5. Site:
    `architecture.md:49`, which states the sum in words and no number (`15 s`: architecture 1 hit, the SSE
    keep-alive, another subject).
  - architecture §Occupied Resources, the cap — carried: Symbols / APIs bullet 6. Site: `architecture.md:355`.
  - test-plan §3, the two kills — carried: Counts / qualifiers bullets 3 and 4. Site: the key file
    `registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10`, `:11`; the test-plan
    body holds no statement of the kills (`45 s|30 s|21 s`: 0 hits in the body).
  - test-plan §6, Path 2's `hint` case and the keystroke case — carried: Symbols / APIs bullet 4 and Outcome.
    Site: `test-plan.md:750`.
  - test-plan §7, the cap — carried: Symbols / APIs bullet 6. Site: `test-plan.md:1077`.
  - obs-plan §5, `send-confirmed.duration_ms` spans the gate's wait — carried as a qualifier: this chunk changed
    neither the field nor the instant it starts from (the send's arrival, as research found); what moved is how
    long the gate's share can be (8.5 s). Sites: `obs-plan.md:763` (the readback-latency metric row) and `:643`
    (the line's field list).
  - a11y-plan §8, the bound falls on the driver — carried: Symbols / APIs bullets 2 and 5. Site:
    `a11y-plan.md:823`, the CLI timing clause, which names `viola wait` and the dialog deadline and not the gate
    (0 hits for the gate in a11y-plan).
  - a11y-plan §3, the keystroke case during the gate's wait — carried: Outcome. Site: the key file
    `registries/contracts/a11y-plan/keyboard-test-harness.md:8` (the tui paragraph: keys injected during a driver
    `send`).
  - security-plan §Input Validation, the `send` rungs read again — not carried into security-plan: it holds no
    statement of the `send` rungs (`turn-running|human-typing|always wins|human keystroke`: 0 hits; `the gate`:
    1 hit at `:239`, the PTY-output row, which is about the screen model's lock). The fact is carried by the
    architecture sites above (`:70`, `:136`). No boundary is widened: two refusals were added.
  - "The pooled capability" (`v1-21`) — not a wrap amendment: phase P5 wrote its dated note; it stays unclaimed.
- **Coverage of new surfaces** (no new external surface; two changed hot-path behaviours and one test seam):
  - `Screen::verdict` waiting arm (inside the existing `run.readiness_gate` span) → validation n/a · instrumentation
    span✓ (the existing span encloses the whole wait and records `outcome` once; nothing added) · PII n/a · tests
    unit (seven `screen` cases, one `gate` case) + integration (`send_under_the_paste_hint_on_a_verified_cli`) ·
    a11y kbd✓ (the keystroke case) · tokens n/a
  - `send`'s second wheel and turn reads → validation refusal-rungs✓ · instrumentation log✓ (the existing
    `send-refused` record and line, codes only; no new site, field or detail) · PII n/a · tests unit (four `send`
    cases) + integration (`send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins`, green on all three CI
    OSes) · a11y kbd✓ · tokens n/a
  - fake agent `--paste-hint-ms` cap → validation capped✓ · instrumentation n/a · PII n/a · tests unit (the parse
    case) · a11y n/a · tokens n/a

## Deviations from intent
1. Plan step 4's three "after the gate wait" unit cases make the gate wait on its not-quiet arm (the input box
   already on screen, fed 3 s after the base) rather than on a literal-less screen whose input box returns
   mid-wait. `Gate::wait_ready` reads the clock while it holds the model's lock, so a feed injected from a test
   clock cannot be joined, and an un-joined feed is a race. The literal-less wait is covered by the `screen` and
   `gate` cases, by the fourth `send` case, and end to end by the hint pair.
2. A third case turned in `src/cmd/verify/typed.rs` (6 000 ms → 9 000 ms), beyond the two the plan names. A listed
   file; the fix-loop's one iteration.
3. The keystroke case asserts the `key` receipts `["0d", "6b"]` and waits for the `6b` line.
4. The `hint` / `no_hint` rstest table lost its `delivered` parameter: both cases are delivered now.
5. Guard control 2 also ran the end-to-end keystroke case; its red there is the `mutants` profile's 10 s kill,
   not an assertion.
6. Before the round's fire: a `cargo build -q`, a `--dry-run` of the round (no live start), and a `census before`
   line in the ledger. The ledger reader still reads green.

**A limit of the keystroke case, stated as a limit (the overseer's disposition).** The case sequences the key on
the wrapper's `channel-request` line. If the key reached the wheel before `send`'s first wheel read, the send
would be refused on that first rung and the case would still pass. That can only make the case falsely green,
never red, and nothing in the case tells the two rungs apart. Guard control 2 is what shows it exercises the
second read on the dev host: with that read removed the case does not pass. No reading of the kind was taken on
the CI runners.

scope record: none — `gate.py scope` clean, 0 recorded (changed 10 · listed 10, base `56e67bb3`, the parent of the
pre-CI commit).

## Decisions & corrections
- The overseer's five dispositions at this wrap (`directive.md` in the wrap run dir): the authority split above;
  the keystroke limit stated; the handoff lists the open hint-line wording for the founder; the re-verify home is
  on tmpfs and gone at a reboot, to be said on "First live test and self-drive", which stamps its own; a card that
  is the founder's is held, not answered.
- The overseer's four P4 answers (inputs#I2) and the build directive (inputs#I3): the round fired once, each
  start ledgered before it; a red round or a product fix after it returns through the overseer.
- Sweep hazard: a test literal that is "past the maximum" by a round number (`6000` against a 5 s bound) carries
  none of the constant's own values (`5000`, `4999`, `5 s`), so a sweep keyed on those values misses it. The whole
  suite found it, not the sweep.
- The fake agent receipts the Enter that submits a prompt as a `key` line of its own, so a `key`-count oracle
  after a send starts at one.
- `Gate::wait_ready` reads the injected clock under the model's lock: a test clock that acts on the feed inside
  `now()` deadlocks or races.
- A nextest-killed end-to-end case leaves its `viola-test-*` home; the fixture's sweep removed it at the next new
  home (read: `target/e2e-home/` empty after the later runs).
- The Bash tool's `find` is an embedded finder that refuses a GNU-style `-newermt` timestamp; the ISO form works.

## Outcome
Acceptance criteria, each re-asserted against the diff:
- (arch) the changed arm and both gate constants are in `viola_agent_claude::screen`, still pure; `src/run/gate.rs`
  changed only inside `#[cfg(test)]`; no manifest changed — MET (the preservation guard green; the diff of
  `gate.rs` is its test module).
- (tests) `run --unit` green, the unit filter entry selects twelve cases and passes them; `GATE_MAX_WAIT` pinned at
  8 500 ms — MET.
- (arch) after the gate's wait a human-held wheel refuses `human-typing` and a running turn `turn-running`, in that
  order, nothing issued or pasted; the three controls read red and green — MET (`evidence/guard-controls.md`).
- (obs) a send refused at the bound writes one `send-refused` with `input-not-ready`, no cursor, no `send-issued`;
  no `obs_event!` site, field or detail added; `schema-check` green — MET.
- (tests) the `hint` case red on the untouched product and green after; `no_hint` unchanged — MET
  (`evidence/hint-red-green.md`).
- (a11y) the integration filter entry green: a key during the gate's wait moves the wheel before the send's
  outcome, the send exits 10 `human-typing` with nothing typed, the key reaches the child — MET, with the limit
  stated under Deviations.
- (tests) nextest holds 20 s × 3 and 15 s × 3 for `test(/verify_window_/)`, no other kill changed; the planted
  hang killed under each and passing with none — MET (`evidence/verify-window-kill-control.md`).
- (tests) `run` green; `viola verify` against the fake agent still ends `17 pass  0 fail`, the 9 000 ms hold case
  included — MET.
- (security) no config key, `VIOLA_*` variable, flag, env seam, ledger row, compiled literal or fixture added; eight
  argv options; `src/cmd/run.rs` and `fixtures/` untouched — MET (the preservation guard).
- (design) `src/human.rs` untouched — MET.
- (arch) the live entry green, fired once, seventeen rows `pass`, the last line `stamped 2.1.287  17 pass  0 fail`;
  the ledger reader green — MET (`evidence/reverify-round.md`, `round-124407Z.txt`, `live-sessions.ndjson`).
- (tests) `pre-push` green and the CI read `verdict: green` for the final HEAD, ci#37623727247 — MET.
- No capability claimed — MET (`matrix.py show --chunk`: claimed 0).

Gates, by `run`, in block order (implement's second whole-block run, the round, and the operator pass):
- `cargo fmt --all --check` — green, exit 0
- `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` — green, exit 0
- `bash scripts/agent-run.sh run --unit` — green (`"ok":true`); red once in the first block run on the one
  `typed.rs` case, fixed
- `bash scripts/agent-run.sh run --unit --filter 'test(/waits_for_the_input_box|after_the_gate_wait/)'` — green,
  twelve cases
- `bash scripts/agent-run.sh run` — green; red once on the same case
- `bash scripts/agent-run.sh run --integration --filter 'test(/a_human_key_during_the_gate_wait/)'` — green
- `git diff --quiet 56e67bb33e63 -- …` (the preservation guard) — green, exit 0
- the `grep -A1 "filter = 'test(/verify_window_/)'" …` probe — green (`last line 1`)
- the `jq -e -s …` ledger reader — green after the round (red before it, by its baseline)
- `bash scripts/agent-run.sh cleanup --session p-hint-smoke` — green
- `bash scripts/agent-run.sh boot --session p-hint-smoke --instance builder` — green
- `bash scripts/agent-run.sh status --session p-hint-smoke` — green, `state:"ready"`
- `bash scripts/g2-zero-panics.sh` — green (`g2: clean`)
- `bash scripts/agent-run.sh schema-check` — green (3 files, 22 lines, no failure)
- `bash scripts/agent-run.sh cleanup --session p-hint-smoke` — green (`processes_gone` and `endpoint_gone` true)
- `bash scripts/agent-run.sh pre-push` — green three times (coverage 1695/1695, playwright 1/1, no breach); red
  once on the same case
- `"$HOME/.local/share/mise/installs/claude/2.1.287/claude" --version` — `leg = 'round'`: green in the recorded
  round listing (`evidence/round-124407Z.txt`)
- the verify entry (`h="target/e2e-home/viola-reverify-…" && cargo build -q && env -u … viola --home "$h/vhome"
  verify -- …`) — `leg = 'live'`: green in the same listing, 46.29 s; `round: COMPLETE · legs fired 1/1`
- `python -X utf8 …/gate.py hygiene` — `leg = 'operator'`: `hygiene: clean`, read twice before the commit
  (`evidence/operator-pass.md`)
- `git diff --quiet && git diff --cached --quiet && git push origin HEAD` — `leg = 'operator'`: exit 0,
  `56e67bb..e574e73`
- `python -X utf8 …/ci.py conclusion --sha HEAD --wait 1800` — `leg = 'operator'`: exit 0,
  `e574e738f814 verdict: green · checks 15/15`, ci#37623727247
- Smoke: ran as the six entries above (cleanup, boot, status, G2, G4, cleanup); not re-driven.

Watches: none folded.

Outcome basis: the operator pass ran. The verdicts rest on its final state: the one commit `e574e73` and its CI
run ci#37623727247, recorded in `evidence/operator-pass.md`. No fix commit followed the pre-CI commit, so no
product source changed after the live round (STOP 6 did not fire; `git diff --quiet e574e73 -- src crates tests
.config` exits 0). Implement's P4 report, given in this session's conversation, is the basis for what only it
holds (the deviations and the census).

Process hygiene (implement P4's census; re-measured at this wrap, 2026-10-07T13:01:49Z: no `viola`, fake-agent,
harness or `claude` process has its executable under this repository or its cwd in a probe dir or a test home):
| Process | Started by | Final state |
|---|---|---|
| five `claude` 2.1.287 children (print, Runs A to D) | the live leg | terminated; none with a probe-dir cwd |
| smoke session `p-hint-smoke` (supervisor, wrapper, fake agent) | this run's gate entries | terminated; `processes_gone` true |
| wrapper and fake agent of the killed keystroke control | this run | terminated; none under this repository |
| `viola verify` children of the two killed planted-hang runs | this run | terminated; none under this repository |
