@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (the six landed ledger rows) — founded the closed capability ledger, `viola verify` and the stamps envelope, and left the screen signatures owed to the readiness gate. Origin of decision/cli-capability-ledger and decision/capability-ledger-gate. The scope says no relied row or probe changes, so this chunk has to stay inside the ledger's single-gate rule.

@history_item n=2 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** (the readiness gate as landed) — set up the pure `viola_agent_claude::screen::Screen` that `run`'s feed thread feeds (`src/run/gate.rs`). A caught panic poisons it until a resize, and a poisoned model reads `input-not-ready`. It set the signature format to `Signatures { input_box, modals }`: two compiled literal lists with **row-contains matching**, marked PROVISIONAL. That row-contains match is the overseer's suspected cause of the quoted-modal refusal, and this chunk's discriminator replaces or qualifies it. The entry also put vt100 into viola-agent-claude, added the `vt100_feed` fuzz target and its seeds, and listed the module layout the fix will touch.

@history_item n=3 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — landed send-issued, send-confirmed and send-refused, and the `not-delivered` details. It set the send refusal order, with `input-not-ready` also covering a missing child and a paste that fails after send-issued. It made the gate partial while no signature row was compiled: a poisoned screen or one not quiet in time refuses, and a quiet screen is ready with no row read. A dropped feed copy poisons like a panic (`sync_channel(256)`). This is the origin of decision/partial-gate-on-unverified-cli and contract/send/paste-failure-after-issue. The chaos home (`TestHome::outside_scan`, `tests/chaos_feed_panic.rs`) is the named carve-out behind decision/chaos-home-outside-gates.

@history_item n=4 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** — the running-turn state lives in `WheelSlot`. Only a `release` that hands back a human-held wheel clears it, so recovery is `pause` then `release`. The defect report says pause then release did not lift the refusal, which places it in the gate (`input-not-ready`) and not in the turn or wheel state. That matters when this chunk confirms or overturns the hypothesis.

@history_item n=5 marker=2026-10-05-t00-09-13-wrap
**2026-10-05-t00-09-13-wrap** — the per-version signature, quiet-period and maximum-wait rows were moved onto the founder-ratified typed-input verify probe (local only). The precedent here: the screen literals are ledger-validated per version, never assumed.

@history_item n=6 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** (screen signatures compiled, ten ledger rows) — compiled `SIGNATURES` (`modals: ["Yes, I trust this folder", "Yes, allow external imports"]`) and added the `modal-signature` and `input-box-signature` rows. `run` passes `Some(&SIGNATURES)` exactly when `cli_verified`, so the full gate, with its modal check, runs only on a verified CLI. `Screen::rows()` is verify-only. This is the source of decision/screen-signatures-compiled, decision/screen-rows-read-only-by-verify and decision/unverified-cli-transport-only. A quoted-modal refusal can only happen on a verified CLI, and the discriminator must not reach row text through any exit other than verify's.

@history_item n=7 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** (registry rows for verify's interactive runs and screen fixtures) — registered `schemas/claude-screen.v1.json` and the signature-row-only `Screen.<phase>.json` fixtures under `fixtures/claude/<v>/`. It also added the fake agent's `--screens` and `--turn-stop`, plus the no-key-into-a-dialog rule (decision/verify-never-types-into-dialog). These are the recorded and fake screens the chunk's tests in both directions will be built from.

@history_item n=8 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — Run B's added pastes go only into rows holding the input-box literal and no modal, and after a long paste the rows come from a wait bounded by `PROBE_DEADLINE`. Verify's paste guard reads the same modal predicate as `run`'s gate, so changing how a modal row is detected also changes verify's guard and Run B's pastes (the scope's "one predicate" note).

@history_item n=9 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** (the paste-hint reading and the fake agent's eighth option) — added `--tag-turn-screen <phase>`, which draws a recorded `Screen.<phase>` after one turn. It is the precedent for a fake-agent screen switch that reproduces a live screen in CI with no live start, the route this chunk needs for a quoted-modal screen.

@history_item n=10 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** (the paste hint measured on the live CLI) — measured the 8.0 s paste-hint timer. A send under the hint was refused `input-not-ready` at the first quiet instant, which led to the founder's decision that the gate waits for the input box with an 8.5 s bound. This is the history behind decision/send-waits-out-paste-hint and decision/gate-timings-compiled.

@history_item n=11 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — `GATE_MAX_WAIT` became 8.5 s. On a verified CLI a modal row refuses at once, while a quiet screen with no input-box row waits until the bound. The wait lives in `Screen::verdict`; `Gate::wait_ready` is unchanged. Send reads the wheel and the running turn again after the gate. This is the modal-refuses-at-once branch the quoted literal wrongly triggers. Recorded trap: a verified, literal-less screen read through a clock that never advances waits forever, so the new tests need an advancing clock.

@history_item n=12 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** (live readings, and the hint line's wording landed) — landed the founder's wording for the `input-not-ready` hint line (contract/human-hint-lines/input-not-ready). It is the text a driver sees for a modal-caused refusal, so it must stay true for a real modal.

@history_item n=13 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (an empty typed text is refused `empty-text`) — added `empty-text` to the closed `not-delivered` detail set and put it second in the send refusal order. That shapes contract/send-refusal/order, contract/refusal-order/send and contract/refusal-reason/not-delivered, which this chunk must leave unchanged.

@history_item n=14 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — `list` is a caller-side board: it reads the disk, contacts no wrapper and reads no screen. This answers part of the scope's question of whether `status`/`list` read the same predicate: they do not, which is why `list` showed the session live and idle while send was refused.

@history_item n=15 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (the agents read, the join and the unwrapped rack land) — `status` (idle/busy) comes only from one `claude agents --json` read joined by pid/session id, never from the screen model. Behind decision/list-is-caller-side-read and contract/board/status; the board and the gate are deliberately apart.

@history_item n=16 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — the dialog relay is hook-driven (`hook.dialog`, `DialogSlot`, one pending dialog per instance), not screen-driven. This bears on the scope's question of whether the dialog relay reads the gate predicate. The CLI-native modals the screen gate guards bypass every hook, which is decision/vt100-readiness-gate's rationale.

@history_item n=17 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** (the second input-box literal and the three-screen input-box-signature row) — `input_box` gained `(shift+tab to cycle)`, because 2.1.302 hides `for agents` and a verified send would otherwise be refused `input-not-ready`. This is the precedent for a gate false refusal on a verified CLI being fixed in the signature predicate with a recorded witness. The readiness rule ("an input-box row and no modal row") was kept unchanged.

@history_item n=18 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** (the 2.1.302 fixture set and the long-turn screen switch) — added the test-only `--long-turn-screen <phase>` and the 2.1.302 set. It is the precedent for a synthetic or recorded screen reproducing a live stop without a live start, and the fake-agent option count has to move with any new option.

@history_item n=19 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** ([Screen Model], the poisoning episode's parse-rejected detail) — one `parse-rejected{parser:"vt100-feed"}` per poisoning episode, detail `panicked` or `oversize` (contract/parse-rejected-event/vt100-feed). This is a separate `input-not-ready` cause that the fix must keep distinct from a modal standing.

@history_item n=20 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (`unknown`'s detail is a code word or null) — refusal details are closed code words, and an unrecognised code-word detail reads as opaque text. This is behind contract/refusal-reason/detail and decision/refusal-details-closed-set. Any new cause or detail for a quoted versus a standing modal can only be added in viola-core, and the scope adds none.

@history_item n=21 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (CI on the one `ubuntu-latest` leg) — CI runs only on `ubuntu-latest` against the fake agent. This is the frame for decision/ci-ubuntu-fake-agent, decision/ci-against-fake-agent and decision/real-cli-probe-local-only. With no live start in this phase, both directions of the tests must hold under the fake agent on that one leg.

@history_item n=22 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (Established Decisions) — the latest rewrite of decision/screen-signatures-compiled. A screen is ready when no row holds a modal literal and either a footer literal stands or the box's own rows do (a rule row of at least 40 rule chars above a prompt-char row). decision/verify-box-wait-for-literal was rewritten with it. This is the exact rule this chunk amends: the "no row holds a modal literal" clause is the part a transcript quote defeats. The precedent: a predicate change lands on both the gate and verify's box wait.

@history_item n=23 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (Occupied Resources and Standard Contracts) — screen fixtures keep footer and modal rows whole, and box rows as the literal alone (decision/screen-fixtures-signature-rows-only). `measured.typed_probe` gained `box_only_guards`. This limits what a recorded or synthetic quoted-modal screen can hold, and it shows how a new measured count lands with no `v` bump.

@history_item n=24 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (Established Decisions and Stack and Technologies) — decision/vt100-readiness-gate gained its except (the test-only live rig also reads vt100 for its trust dialog, selector and input box), and decision/vt100-screen-model was widened the same way. The rig also reads modal and box state, so a changed predicate may need checking against the rig's own readers.

@history_item n=25 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** — decision/run-never-reads-screen-content now names the rig-only first-screen record of whole scrubbed rows as the second row-text exit. It also added the `--record-first-screens` excepts on the fixture rows, and `measured.first_screen` / `first_settle_ms`. Any discriminator that reads row text in `run` breaks this rule: it must stay a verdict inside the agent crate, never row text leaving the screen.

@history_item n=26 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Established Decisions) — added decision/trust-record-ledger-row: Run F passes only when Run A showed the trust modal and Run F's start shows no modal and the input box. The trust-modal literal's detection is part of a relied row's check, so the real-modal side of this chunk's fix must keep that check true.

@history_item n=27 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** (Established Decisions) — added decision/browser-tools-prompt-ledger-row: pass when a first screen is the browser-tools prompt, absent when none met it, fail otherwise, and the one row whose `absent` counts toward verified. The browser-tools prompt is a newer modal the product knows. "Every compiled modal literal on its recorded screens" in the scope includes it, and its readings depend on the same modal detection.

@history_item n=28 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** (Standard Contracts and Design Philosophy) — decision/version-verified-all-relied-rows, contract/ledger-stamps/verified-version, decision/capability-ledger and contract/ledger-stamps/content were reworded to "pass, or absent on browser-tools-prompt alone". `measured.first_screen` gained `browser-tools`. This sets which CLI counts as verified, which in turn decides whether the full modal-checking gate runs at all.

@history_item n=29 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Conventions and Established Decisions) — took `budget-paused` out of contract/refusal-order/send and contract/send-refusal/order. That left today's order: control-character, empty-text, human-typing, turn-running, the gate's `input-not-ready`, then the re-read rungs. This chunk must keep the order as it stands.

@history_item n=30 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** (Established Decisions) — added decision/send-flight-lock: the post-gate section runs under one wrapper flight lock, a human key never takes it, and the gate's wait sits before it. Also added decision/submitted-form-prompt-match. A gate verdict change must not move work across the lock boundary.

@history_item n=31 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — widened decision/send-confirmed-after-the-fact, decision/every-send-confirmed, contract/send-results/unconfirmable and contract/events-ndjson/send-confirmed-order for stdout-echo local commands, and added decision/local-command-echo-confirmation and decision/local-command-echo-probe. Delivered sends go through these confirmation rules, which follow the gate this chunk changes.

@history_item n=32 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — added decision/local-command-context-probe (Run B's eighth paste, `/context`) and widened contract/ok-payload/unconfirmable to cover any listed command on an unverified CLI. Run B has eight compiled pastes behind the same per-paste modal and box guard, so a predicate change reaches every one of them.

@history_item n=33 marker=2026-11-19-a-fired-scheduled-wake-keeps-the-wheel
**2026-11-19-a-fired-scheduled-wake-keeps-the-wheel** (Standard Contracts) — contract/event-data/prompt-submitted-origin and contract/event-prompt-submitted/origin now say that the wrapper also relabels a CLI-fired scheduled wake `harness`. This is the latest change to the send and wheel contracts this chunk borders. It is PROVISIONAL (delegate overseer) and its base is `9e2b7e7d`.

@history_item n=34 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** (the claude-first-no-adapter-trait except) — the test-only exception list of decision/claude-first-no-adapter-trait, decision/claude-shapes-in-one-crate and decision/claude-shapes-isolated sits at its 200-character bound. Claude-specific screen knowledge belongs in viola-agent-claude. Adding a new test-only Claude-shape reader outside it would force another cut of that exception.

@history_item n=35 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** — decision/root-waits-share-one-bound moved to 12 s `WITHIN`. Any new root test wait for the gate's verdict shares that bound.
