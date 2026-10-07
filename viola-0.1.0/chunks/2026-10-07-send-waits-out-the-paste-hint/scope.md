# Scope — 2026-10-07-send-waits-out-the-paste-hint

**Working entry** (`working-route.md:98`, Epoch 3 — Windows slice II: driving verbs and live proof):
Send waits out the paste hint — on a verified CLI a send issued while the paste hint stands is delivered once the
input box returns, within 8.5 s.

## Intent
After a long paste the `claude` CLI shows `paste again to expand` where its input-box literal was, for 8.0 s from
the paste. Today a verified wrapper refuses a `send` issued in that window `not-delivered` / `input-not-ready` at
the first quiet instant. This chunk builds the remedy the founder chose: on a verified CLI the readiness gate keeps
waiting for the input box on a quiet screen that holds no compiled literal, and its bound rises from 5 s to 8.5 s,
so the send is delivered once the input box is back. The moved bound is what a stamped ledger row is checked
against, so the chunk ends with one by-path `viola verify` of `claude` 2.1.287.

## Authority
- **The founder's hint ruling** (his own live answer, 2026-10-07T10:29Z, relayed by the overseer; the entry's
  first CARRY; recorded in chunk 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
  `evidence/hint-card-answer.md`, that chunk's `inputs/I6-relay-hint-card.md.txt`): options 1 and 2 of his card
  together. On a verified CLI the gate waits for the input box on a quiet literal-less screen, and the bound rises
  to 8.5 s. He was told the price: one by-path re-verify (5 live starts), `send`'s longest block 18.5 s, the fake
  agent's 8 000 ms hold cap to raise, no human keystroke delayed.
- **Placement** (the overseer's direction at that chunk's wrap): right after it and ahead of "First live test and
  self-drive".
- **The operator's directive at this take-up** (inputs#I1, a verbatim copy):
  - the live cap is the one by-path re-verify of 2.1.287, 5 live starts, and no more without the founder;
  - the founder is away until late evening: a technical fork goes to the overseer; a fork that is the founder's is
    answered provisionally by the overseer and listed for him;
  - a NEW boundary widening is not answered, it is held: the plan needs none;
  - a red re-verify round is not retried: stop and report;
  - the chunk is sized against one builder window.

## What the chunk builds
1. **The gate's verdict on a verified CLI.** With the compiled signatures in hand, a quiet screen that holds no
   input-box literal and no modal literal no longer ends the gate at the first quiet instant: the gate keeps
   waiting, and refuses `input-not-ready` only when the bound has run out. Site re-verified at the take-up:
   `Screen::verdict`, `crates/viola-agent-claude/src/screen.rs:122-152`, its last arm at `:147-151`.
   - A modal literal on any row still refuses at once (the row loop returns before the input-box read,
     `screen.rs:141-144`).
   - A poisoned screen still refuses at once (`screen.rs:128-130`).
   - The unverified reading keeps its three outcomes, and one of them moves with the constant.
     `[premise-corrected: the not-quiet arm at screen.rs:131-135 reads GATE_MAX_WAIT ahead of the signature split,
     so an unverified screen that never goes quiet is refused at 8.5 s where it was 5 s; a quiet unverified screen
     is still Ready with no row read; the founder's card said so under option 2]`
   - `QUIET_PERIOD` (300 ms) is unchanged.
2. **The bound.** `GATE_MAX_WAIT` moves from 5 s to 8.5 s. Sites re-verified: the constant at `screen.rs:14`, its
   pin at `:184`, the boundary cases at `:230-232`.
3. **The outcome the entry names.** On a verified CLI a `send` issued while the paste hint stands is delivered once
   the input box returns, and the send is confirmed as any other. "Within 8.5 s" is counted from the instant the
   send enters the gate, after its refusal rungs (`src/run/send.rs:338` hands `slot.clock.now()` to
   `Gate::wait_ready` as `waiting_since`), not from the Stop.
4. **The hint that outlasts the bound.** A screen that stays quiet and literal-less past the bound is still refused
   `not-delivered` / `input-not-ready`, now at the bound and with nothing typed (the refusal at `send.rs:339-341`
   stands ahead of `send-issued` and of the paste).
5. **The fake agent's hold.** `--paste-hint-ms` is capped at 8 000 ms (`src/bin/viola-fake-agent.rs:33`), under an
   8.5 s bound. The cap is raised, as the founder was told.
   `[premise-corrected: the case that needs a hold past the bound is verify's own,
   verify_window_paste_hint_past_the_gate_maximum_still_stamps (tests/cli_verify.rs:1057), whose 6 000 ms hold is
   "past the gate maximum" only against 5 s]`
   The cap becomes 10 000 ms, for that case alone. Item 4's refusal is proved at the unit layer on the injected
   clock: no end-to-end case blocks to the bound, and the cross-process `input-not-ready` path keeps its
   end-to-end proof in the trust-dialog case (the overseer's answer, inputs#I2).
6. **The cases that turn.** `send_under_the_paste_hint_on_a_verified_cli` (`tests/cli_send.rs:551-554`, cases
   `hint` and `no_hint`): the `hint` case turns from refused to delivered. `Screen::verdict`'s own cases and the
   gate's cases (`src/run/gate.rs`) follow items 1, 2 and 4.
7. **The `input-not-ready` hint line.** Its advice is `viola wait <name>, then send again` (design-system `:764`,
   layout-templates `:415`; both re-verified at the take-up; the product string is `src/human.rs:216`). Whether
   that line changes is this entry's to settle (the second CARRY). Settled at P4: the line is left as it is, and
   no product string moves (the overseer's answer, inputs#I2). After this build it is no longer printed for the
   paste-hint cause; it stays for a modal, a screen that never goes quiet, a poisoned screen and a hint past the
   bound. Listed for the founder: the line still advises `viola wait` for those causes, and a rewording is his.
8. **No human keystroke is delayed.** The wait is on the automation side only: a key pressed while the gate waits
   goes to the child as today and takes the wheel. a11y-plan's keystroke clause is re-read at this build (the
   third CARRY; the clause stands at a11y-plan `:99`, `:624` and `:990`, re-verified). The chunk shows it with a
   case, not by argument.
   `[premise-corrected: the key is never held (the pump holds human bytes only across the one paste write, and the
   gate takes the model's lock per read, src/run/gate.rs:163-179), but send reads the wheel once, at its arrival
   (send.rs:314), and not again after the gate's wait: a human who starts typing during the wait is pasted over
   once the input box returns]`
   Settled at P4 (the overseer's answer, inputs#I2, a technical fork): after the gate's wait `send` reads the wheel
   and the running turn again, in the documented order, and refuses `human-typing` or `turn-running` with nothing
   typed. This tightens the residual architecture records for a turn that starts during the wait; it widens
   nothing. Listed for the founder as a fact.
9. **`send`'s longest block becomes 18.5 s** (8.5 s of gate, 10 s of confirmation). No bound in the tree was sized
   against 15 s: `viola send`'s client waits on `Client::request`, which has no deadline
   (`src/cmd/send.rs:227-228`), and the tests' `send` helper waits on the child with no bound of its own
   (`tests/cli_send.rs:139-150`). The MCP tool-call bound is an open architecture item and is not settled here.
   `[premise-corrected: what was sized against the gate's bound is the nextest verify_window_ class, which waits
   the maximum four times: .config/nextest.toml:23-27 and :54-58, 15 s x 3 on CI and 15 s x 2 under mutants,
   against about 21 s today and about 35 s at 8.5 s]`
   Settled at P4 (the overseer's answer, inputs#I2): both overrides move in proportion, 20 s × 3 on CI and
   15 s × 3 under `mutants`, with the planted-hang control read both ways in `evidence/`. No other kill moves, and
   `WITHIN` stays 7 s.
10. **The one by-path re-verify.** `GATE_MAX_WAIT` is the bound the stamped `quiet-period` row's measured settles
    are checked within (`crates/viola-agent-claude/src/ledger.rs:760-761`), and `viola verify`'s first settle uses
    it (`src/cmd/verify/typed.rs:487`); both sites re-verified. A moved bound loosens a stamped row's check, so the
    built binary runs `viola verify` once against `claude` 2.1.287 named by path: 5 live starts, each ledgered
    before it starts. A red round is not retried: stop and report (inputs#I1).
    - The round stamps a fresh home of its own under `target/e2e-home/`, as the 2026-10-06 record round did. It
      reads no standing home and nothing in `target/e2e-home.disk/`; nothing there is deleted by this chunk.
    - The round records no fixture: it is a re-verify, not a `--record` round.
    - A stamp is read by its row ids alone (`ledger.rs:919-927`), so a stamp written under the 5 s bound is still
      read as verified; the round is the founder's price, not a mechanical need of the tree.
11. **`viola verify`'s own settles.** The ordinary settle falls back at the first quiet instant past the bound
    (`typed.rs:478-498`), so it moves to 8.5 s on a screen with no literal; Run B's wait after a long paste
    (`box_wait`, `typed.rs:505-523`) reads `PROBE_DEADLINE` and does not move.

## Measured facts the entry carries (each spot-checked at P3)
- "measured at that chunk on `claude` 2.1.287 (`evidence/hint-window.md`)": the hint is a timer of 8.0 s from the
  last long paste (nine timings, 8.000 s to 8.023 s; a second long paste restarts it, a short one does not), 3.8 s
  to 7.0 s of it after the turn's Stop. The evidence file holds the nine rows.
- "measured at that chunk": today a `send` in the window is refused `input-not-ready` 0.63 s after it was issued,
  at the first quiet instant, so the 5 s bound plays no part. At HEAD the last arm of `Screen::verdict` still
  returns `InputNotReady` on a quiet screen with no input-box literal.
- "measured at that chunk": a no-cursor `viola wait` issued in the window ran to its deadline, so the hint line's
  advice does not lead out of the window (the evidence file's step 4).
- "measured at that chunk": on an unverified CLI a short text typed under the hint was submitted and confirmed.
  Unmeasured: a long or a repeated text pasted under the hint.
- "what the founder was told it costs (the same chunk's `evidence/hint-card.md`)": the bound is what the stamped
  `quiet-period` row is checked against, hence the re-verify; `send`'s longest block becomes 18.5 s
  (`CONFIRM_WINDOW_FALLBACK` is 10 s); the fake agent's cap is 8 000 ms, under an 8.5 s bound.
- The sites the card names, all three found at the take-up: `Screen::verdict`, `src/run/gate.rs`, the two cases of
  `send_under_the_paste_hint_on_a_verified_cli`.

## Out of scope
- Typing under the hint (the card's option 3): no new compiled literal, no eighteenth ledger row, no new probe, no
  fixture. The hint's own text is not read by the gate.
- A `send` whose text ends in a newline (`working-route.md:100`, its own entry).
- What a no-cursor `viola wait` does after a turn has ended. Only the hint line's wording is this entry's.
- The MCP `send` tool and its client timeout (`working-route.md:122`).
- Any live start beyond the one re-verify round, and any boundary widening (inputs#I1). The round runs inside the
  standing `viola verify` boundary of security-plan as it is written.
- The masters: read here, amended only by the wrap.

## Surfaces and contracts touched
- `viola-agent-claude`: the screen model's readiness verdict and its two constants; the `quiet-period` ledger row's
  check.
- `viola` (root bin): the `run` gate (`src/run/gate.rs`), `send`'s rungs after the gate and its blocking time,
  `verify`'s settles, the fake agent's `--paste-hint-ms`, the nextest `verify_window_` overrides.
- `ledger/stamps.json` for 2.1.287 in the round's own home, written only by `viola verify`.
- Expected at the wrap: architecture [Screen Model], [CLI Version Compatibility] and [Human Takeover / Wheel];
  test-plan §3, §4, §6, §7 and §10. design-system and layout-templates take nothing: the hint line stands.

## Sizing
One builder window (inputs#I1). The build is one verdict arm, one constant, the fake agent's cap, the nextest
overrides, about a dozen unit cases turned or added, three end-to-end cases turned or added, and one live round
with no fixture: twelve files at most. It fits one window.

## CI since the last wrap
- `56e67bb33e63` (the wrap commit of 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host): green, checks 15/15,
  wall 394 s, ci#37615944213. Nothing to fold.
