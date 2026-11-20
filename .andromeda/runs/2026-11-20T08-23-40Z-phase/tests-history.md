@history_item n=1 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — (the named spine bound, the vt100 property and fuzz target) landed the vt100 feed property at 512 cases (`screen::tests::feed_prop_a_caught_panic_poisons_until_resize`) and the `vt100_feed` fuzz target (6 synthetic seeds), measured `--vt100-panic-bytes` with no new env seam. This is the screen-model test surface the gate's discriminator changes: the property, the fuzz seeds and the parser surface lists (decision/property-tests-on-parsers) must stay true, and any new screen-reading parser joins them.

@history_item n=2 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — (Path 2 as landed; the chaos home F4) built `--vt100-panic-bytes` (writes `e4 b8 ad` after `start`) and made `tests/chaos_feed_panic.rs` the one test booting in `TestHome::outside_scan()` (`viola-chaos-*`, outside G2, G4 and the secret scan, on the founder's ruling). The vt100-panic rows this chunk keeps (decision/vt100-panic-contained, vt100-panic-keeps-passthrough) rest on this carve-out; a reworked gate must still give `input-not-ready` after a caught panic.

@history_item n=3 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — (the screen fixture class) brought in the `Screen.*.json` fixtures: signature rows only, scrub-as-detector, a dirty one refused whole, walked against `schemas/claude-screen.v1.json`. It also added the fake agent's `--screens`/`--trusted-root` and the rule that wrapper boots pass `--screens --trusted-root <cwd>`. `SIGNATURES` feeds the full gate on a verified CLI and verify's settle helpers. This is the base of the "recorded screens in the store" the discriminator is chosen from. A screen this chunk adds, synthetic or not, must pass that walk and keep only signature rows (decision/screen-fixture-scrub-as-detector).

@history_item n=4 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — added `--tag-turn-screen <phase>`, the forced-window pair `send_under_the_paste_hint_on_a_verified_cli` (input-not-ready, nothing typed, under the hint hold) and `verify_pastes_no_local_command_once_the_tag_turn_screen_shows_a_modal`. These are the standing tests where a modal on the screen blocks typing. They are precedents for the refusal half this chunk must keep (a real modal still refuses) and the fake-agent screen mechanism for placing a modal.

@history_item n=5 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — set the gate's verdict as ready / wait / `input-not-ready`, with a quiet screen with no input-box literal waiting to `GATE_MAX_WAIT` 8 500 ms (unit-pinned). After a ready gate, `send` reads human-typing and turn-running again (four clock-driven unit cases). It added `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins` (a floor) and kept the refusal at the bound unit-tier only. Standing rule: a kill line moves only for a by-design floor, with a planted-hang control both ways. Any test that waits out the gate's bound in this chunk falls under that kill sizing.

@history_item n=6 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — (the typed text, `empty-text`, and the `mutants` override order) set the refusal order to control-character → empty-text → human-typing → … (decision/send-refusal-order, empty-text-refused-before-the-wheel). It also put `test(/verify_window_/)` first in `[profile.mutants]`, pinned by `mutants_verify_window_override_stands_before_the_harness_package_override`. A gate change must leave `input-not-ready` in its place in that order. A new test that waits the gate out four times belongs to that class's kill.

@history_item n=7 marker=2026-10-09-inner-cr-and-crlf-in-a-sent-text
**2026-10-09-inner-cr-and-crlf-in-a-sent-text** — pinned the typed form: every CR LF and other CR as one LF, trailing CR/LF removed (`typed_text_drops_every_trailing_newline`, `typed_text_every_inner_cr_is_one_lf`). This is the text that is classified, pasted and matched (decision/sent-text-typed-form). A send case quoting a modal literal in its text goes through the same typed form.

@history_item n=8 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — (the long-turn screen and the second literal) added the `input-box-signature` arm reading three Run B screens, with `…_long_turn_holding_the_literal_passes` and `…_holding_a_modal_fails`. It added the `long_turn` screen, `--long-turn-screen <phase>`, and the second input-box literal `(shift+tab to cycle)` beside `for agents`. It also added `verify_pastes_nothing_more_once_the_long_turn_screen_shows_a_modal` (`19 pass  5 fail`). This is the closest precedent: a screen holding a modal literal is read as not-ready by the same predicate verify's settle uses. The fix must keep these cases' verdicts or say why they move, and must not change the ledger row (scope boundary).

@history_item n=9 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — (4. Unit Test Strategy, ~ contract/readiness-gate/gate-max-wait) changed the gate so the input box reads up on its own rule and prompt rows with no footer literal ("no box rule row directly above a prompt row"). This is the current rule for the predicate this chunk changes, and the picked row's exact wording. A discriminator built on row position or the box rows must state itself against it.

@history_item n=10 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — (7. Test Data & Fixtures, + decision/screen-fixture-box-rows-as-literals) added the rule that a screen fixture keeps a box rule row as its rule chars alone and the prompt row as the prompt char alone, the hygiene walk refusing more (`planted_box_row_beyond_its_literal_is_rejected`). Trap: a recorded or synthetic screen with a transcript row quoting a modal must still fit this literal-only box-row form, and the walk refuses a prompt row with text after the prompt char.

@history_item n=11 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — (+ decision/send-cases-copy-recorded-fixtures; ~ contract/viola-send/turn-running) measured that a purely synthetic fixtures root has no screen rows, so the fake agent draws no input box and a send after the first turn reads `input-not-ready`. Send cases copy the recorded set and add their synthetic payloads. This is a direct trap for this chunk's idle-session send cases and for any "synthetic screen beside the recorded sets": an idle-session send that reads input-not-ready may be missing screen rows, not hitting the quote defect.

@history_item n=12 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — (− decision/send-confirmation-exact-match → + decision/send-confirmation-trailing-trim-match; ~ contract/viola-send/records; fake agent `--trim-prompt-end`) widened the claim to tolerate a trailing trim, ratified later (2026-11-16T06-05-33-wrap). It bears on the delivered half: a send into an idle session with a quoting row must be confirmed by the matching `prompt-submitted{origin:"driver"}` under the standing matcher.

@history_item n=13 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — (6. E2E, + decision/standing-live-rig; 7. Test Data, fake-agent trust dialog and keystroke-draws-box rows; screen-read excepts) made one committed `viola-live-rig` whose rehearsals run in CI over the fake agent. The fake agent draws a box rule and prompt row on each typed key. The rig alone reads the child's screen into closed words by compiled literals. This chunk forbids a live start, so it falls back on the rehearsal and fake-agent screens. The no-child-screen-assertion rows bind every other test. The fake agent's trust form is drawn over the recorded modal (contract/fake-agent-screens/trust-dialog).

@history_item n=14 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — (4. Unit, ~ decision/send-confirmation-trailing-trim-match why) recorded the trim set read live: a trailing run of spaces and TAB removed, a leading run, an inner run and an inner LF kept. This is the confirmation reading the delivered-send cases rely on.

@history_item n=15 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — (+ contract/fake-agent-cli/start-screen; verify step/summary `absent`; version gate counting rows; local-live pass) added the browser-tools modal literal and its CI proof through the fake agent's `--start-screen <phase>`, which draws `Screen.<phase>` and keeps it standing, no byte reaching the input box. It is the newest modal literal and the mechanism for putting a standing modal on screen in a test. This chunk's refusal half must cover every compiled modal literal on its recorded screens, this one included.

@history_item n=16 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** — (the four screen-read excepts: decision/no-child-screen-assertions, never-parse-rendered-child-screen, no-screen-parsing, discipline-at-every-layer) widened the rig's screen reading to the box row's cell attributes as closed words, never text. It also added a local-only exception for the SF arm's contained rehearsal to decision/standing-live-rig. This is a precedent for attribute-based discrimination of screen rows, a candidate for telling a transcript row from a standing modal. It is held only for the test-only rig, and every other test stays bound by the no-parse rows.

@history_item n=17 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — (+ decision/live-rig-hint-cell-reading) recorded the rig's hint-cell attribute words as a reading, never as verification. With 2026-11-17-trust-record-ledger-row's decision/live-failing-reading-not-verification, it fixes how any screen reading this chunk cites is recorded.

@history_item n=18 marker=2026-11-19-a-fired-scheduled-wake-keeps-the-wheel
**2026-11-19-a-fired-scheduled-wake-keeps-the-wheel** — (+ decision/fired-wake-keeps-wheel, + decision/rig-no-key-read-from-keys-word, ~ live-rig-hint-cell-reading; + `--stop-crons`, `--wake-after-ms`; ~ fake-agent-replays-recorded-bytes) is the previous chunk. Its wheel rows sit beside decision/injected-turn-keeps-wheel. A rig rehearsal reads no-key from its own keys word, never the shared receipt. The fake agent's closed list of set payload fields was cut to its bound, so any new fake-agent option this chunk adds must extend that list.

@history_item n=19 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (remove-the-guard witnesses bounded by `WITHIN`) set the rule behind decision/bounded-remove-the-guard-witness: a control whose neutralised guard lets a product process start needs a bounded witness, since an unbounded one hung 27 minutes under nextest's default profile. This chunk's scope asks for the remove-the-guard control on the refusal side too, so that control must spawn through `tests/support/cli.rs` `Running`, bounded.

@history_item n=20 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — (the feed-panic passthrough witnessed on the outer terminal) made `chaos_feed_panic_refuses_send_and_keeps_passthrough` byte-search the drained outer stream (the child's `e4 b8 ad`, none of viola's literals). That is the only sanctioned byte search (the except on decision/no-child-screen-assertions and tui-verdicts-from-records). A gate rework must keep this guard green.

@history_item n=21 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — (the bounded-reader enumeration) added `tests/contract_bounded_readers.rs` (`BOUNDARIES`, `EXCEPTIONS`, census `product_readers_are_each_bounded_or_named`). This is the guard of decision/bounded-readers: any new product read of screen or transcript bytes the discriminator adds must be bounded in-statement or named with a witness.

@history_item n=22 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — (the mutants profile's paste-hint class) added the `send_under_the_paste_hint_` class at a 30 s mutants kill, pinned by `mutants_paste_hint_override_holds_the_class_above_the_default_kill`. Both floors move only with a planted-hang pair, never by raising the default. This is the source of decision/send-window-paste-hint-30s-kill. A new test that waits out the 8.5 s gate needs a matching class name or must not wait it out.

@history_item n=23 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** — (~ decision/root-wait-bound, within-7s-wait-bound, send-window-paste-hint-30s-kill why, mutants-kill-below-cargo-mutants-timeout) moved `WITHIN` 7 s → 12 s and the mutants default kill 10 s → 15 s, still below cargo-mutants' 20 s floor. These are the bounds every new wait in this chunk's tests sits under.

@history_item n=24 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** — (decision/verify-window-60s-kill → verify-window-80s-kill; ~ record-only-at-zero-failing-rows except) moved the `verify_window_` class kill to 80 s in ci and mutants. It let the rig alone write first-screen files on a failed recording verify (never committed). If verify's settle shares the changed predicate, its window cases are judged under the 80 s kill. A screen file the rig wrote is not a committed fixture.

@history_item n=25 marker=2026-11-17-booted-wrapper-does-not-outlive-its-test
**2026-11-17-booted-wrapper-does-not-outlive-its-test** — (+ decision/no-untied-viola-spawn, + tied-child-spawn-thread; ~ wrapper-drop-guard; ~ supervisor-ordinary-child) tied every long-lived viola child of a root test to its thread, enforced by `cross_killed_test_census_finds_no_untied_viola_spawn`. Every new wrapper boot in this chunk's root tests must go through the tied builder.

@history_item n=26 marker=2026-11-17-supply-chain-green-and-booted-tests-end
**2026-11-17-supply-chain-green-and-booted-tests-end** — (+ decision/no-untied-harness-boot) is the viola-e2e twin: `viola_e2e_census_finds_no_untied_boot` refuses an untied harness boot. Any harness-session case this chunk adds must be tied.

@history_item n=27 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — (+ decision/run-under-test-owned-user-home, + harness-wrappers-under-session-user-home) set that every test-started `viola run`/`revive` and harness wrapper runs under a test-owned user home, because a start's trust step writes the Claude config. This holds for every boot in this chunk's tests. It also matches the scope's "nothing read of the operator's real Claude config".

@history_item n=28 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — (+ decision/verify-under-test-owned-user-home, + no-verify-start-outside-test-home, + live-failing-reading-not-verification; fake agent `--trust-config`) put every test verify under a test-owned user home, held by a census. A failing or confounded live reading is recorded as read, never as verification. It binds any stamped-home helper this chunk uses.

@history_item n=29 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — (+ decision/stamping-verify-passes-transcript; fake agent `--transcript`) requires every stamping verify against the fake agent to pass `--transcript` beside `--framing`, or the stamp misses the echo row. Any new stamped-home helper or fixture root this chunk builds must keep that.

@history_item n=30 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — (+ decision/start-lines-read-from-start-offset; ~ contract/event-log/start-wheel-record) set that a reader of a start's lines reads from that start's offset, never line 1, and that trust keys through the wrapper write a human-input wheel line before session-start. This is a trap for idle-session send cases on a reused instance name, and for wheel assertions around the send.

@history_item n=31 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — (Path 5 records the classifier's tables) pinned the closed terminal-reply list (`classifier_terminal_reply_is_not_editing`, 23 `REPLIES`). Its reading on a real terminal is a local measurement, not a suite. This is the source of decision/terminal-replies-not-editing among the picked rows: a send case must not let terminal replies move the wheel.

@history_item n=32 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** — (Path 5 and Path 2 span the running turn) means every test that sends twice must end the first turn with a scripted Stop, because the fake agent's interactive submit fires none. A refused send while a turn runs is `turn-running`, not `input-not-ready`. This chunk's "idle session" cases must end prior turns so a refusal is the gate's and not the turn's.

@history_item n=33 marker=2026-09-29-fake-agent-drift-contract
**2026-09-29-fake-agent-drift-contract** — (four harness prefixes on the raw start; the drift contract byte for byte) set contract/prompt-submitted/origin-harness (a tag mid-prompt or after a leading space stays human). It also set the byte-for-byte drift contract (decision/fake-agent-contract-suite, fixture-drift-byte-compare). Any new or changed fake-agent screen or fixture must keep `contract_fake_agent_drift` green, and the walk fails on an empty set.

@history_item n=34 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — made the fixture hygiene walk a run-time directory walk (`claude_fixtures`), not rstest `#[files]`, because rstest refuses an empty glob at compile time. This is the why of decision/fixture-hygiene-walk. A synthetic screen added beside the recorded sets is walked there.

@history_item n=35 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — (7. Test Data, ~ fixture-hygiene-walk, fake-agent-on-test-path, argv-only-options, recorder-scrub; 5., ~ fake-agent-as-claude) gave the current wording of the hygiene walk and fake-agent rows picked for this chunk: the fake agent is configured by argv alone (hyprctl the one exception). Any screen option this chunk adds to the fake agent must be argv-only.

@history_item n=36 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (the reply screen and the fault screen; the exit-2 paths that write no document) set contract/reply-screen/detail (a string detail that is no code word reads null) and the exit-2 except on decision/cli-json-one-document. A refused send's `input-not-ready` document must stay inside these screens.

@history_item n=37 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — (the CLI tests over `std::process::Command`, integration and fixtures tiers) said no manifest holds assert_cmd, trycmd or snapbox, and each fixed CLI string is a literal in its test file (decision/cli-fixed-strings-pinned-by-literal, cli-e2e-via-process). New send-outcome lines in this chunk's tests are pinned by literal.

@history_item n=38 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** — (run --e2e; the E2E regex) set the E2E layer regex `^(path[0-9]*|tui|mcp|cross|chaos|contract)_` and made the integration layer its complement. A new test binary's prefix decides its layer and suite, and decision/e2e-asserts-only-cross-process sends parser and threshold cases to the unit tier.

@history_item n=39 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the PTY seam on openpty alone; the E2E witnesses on ubuntu-latest) left the tui driver as openpty only, with the vt100-panic byte search having no per-OS split, and CI on one `ubuntu-latest` leg. The chunk's screen and gate tests are Linux only, with no OS-gated twins.
