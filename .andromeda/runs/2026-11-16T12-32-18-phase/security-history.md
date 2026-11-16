@history_item n=1 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — a new attack-surface vector: the human's terminal on `viola run`'s stdin. It added the stdin vector, with the stdin classifier's closed non-editing list (F-W2) as its trust boundary. The list was widened once by seven measured terminal-reply shapes after `CSI`, each matched by exact grammar. The nearest human keys and their one-field or one-prefix neighbours still count as typing, and no reply is added unmeasured. The classifier reads bytes, not time. A key that reaches this stdin is the human's, whoever typed it; agent-typed keys through `wtype` on a nested compositor were ratified for one start. Both widenings were the founder's own word, given after the grammars were shown to him. Why it matters: this is the direct precedent and register for this chunk (`decision/stdin-closed-non-editing-list`, `decision/non-editing-list-measured-only`, `decision/human-typing-takes-wheel`). The survey grammar has to join the same way: measured, exact grammar, its neighbours still typing, ratified as a boundary widening. Adding screen state to a classifier that "reads bytes, not time" changes that vector's stated boundary, so it must be re-stated.

@history_item n=2 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — the seventh dated gap: pause and release frames, and their validation. It served the wheel's `pause` / `release` and refused a driver-originated `release` (a string `from`) with `release-from-driver`. CLI `release` forwards `VIOLA_NAME`. Why it matters: these are the wheel controls a survey key must leave untouched. The wheel's holder and cause are closed values (`decision/closed-types-for-protocol-fields`), so a survey key must change no holder and add no free-string cause.

@history_item n=3 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — the vt100 degrade as landed on the PTY output row. A tee writes the human's bytes first, then a feed thread runs every feed and resize under `catch_unwind`. A caught panic poisons the model until a size change. vt100 panics are reachable at real small sizes. Why it matters: if the survey's screen state must reach the wheel decision, it comes from this model. A poisoned or unavailable model must never make a key read as non-editing; it must fail toward typing (`decision/vt100-feed-isolated`, `decision/vt100-inside-catch-unwind`).

@history_item n=4 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — send's input validation as landed; the feed bound closed. The tee-to-feed queue became `sync_channel(FEED_CAPACITY)`: 256 messages, a `try_send` after the human's write, and a dropped copy that poisons the model until a size change. The model sits behind a mutex shared with the gate, with the catch inside the lock. Why it matters: `decision/vt100-feed-bounded-queue`. A wheel classifier that reads screen state would take that shared lock from the stdin path. It must never block the passthrough, and it must handle a model that is poisoned or behind.

@history_item n=5 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — the vt100 passthrough after a caught feed panic, witnessed on the outer terminal; the bounded-reader census. `chaos_feed_panic_refuses_send_and_keeps_passthrough` reads the outer terminal. `tests/contract_bounded_readers.rs` requires every product reader call to be bounded in its statement or named in `EXCEPTIONS` (`product_readers_are_each_bounded_or_named`). Why it matters: a new reader added for the survey probe or its fixtures must pass the census (`decision/bounded-readers-census`). The feed-panic guard named by `decision/vt100-feed-under-panic-catch` has to stay green if the classifier starts reading the model.

@history_item n=6 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — verify's typed-input PTY probe: the widening, its dirs, residual and refusal. verify's interactive `claude` children are direct spawns under the R8 strip at 80×24. Their output is capped at `MAX_FRAME`, fed into a `Screen` under `catch_unwind` and read only against compiled literals. No byte goes into a CLI-native dialog, and a start that shows a modal is killed with no key. Screen fixtures keep signature rows only, and the named refusal uses the closed codes `home-path` · `absolute-path` · `username` · `email`. Each piece was the founder's own live ruling. Why it matters: a survey ledger row and probe extend this child set (`decision/verify-children-direct-spawn`, `decision/verify-never-answers-dialogs`, `decision/verify-writes-no-dialog-byte`, `decision/fixture-recording-scrubbed`). Keying a survey answer inside a verify child would cross the no-dialog-byte rule, so that needs its own ruling.

@history_item n=7 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — the capture arm answers, Runs C and D, R2 closed. Probe dialogs are answered only by the hook's capture-arm output, never by a typed byte. The stamp count moved to `/14`. Why it matters: this is the precedent for the `except` of `decision/verify-writes-no-dialog-byte`. If the survey can only be dismissed by a key, verify cannot answer it the way Runs C and D do, and the probe design must face that.

@history_item n=8 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — Run B's four compiled pastes and its wait; the seventeen-row stamp. Pastes go only into rows that hold the input-box literal and no modal literal. A modal, a poisoned screen or the deadline ends the added pastes with no key. The founder ratified this as a boundary widening, live. Why it matters: a new ledger row moves the stamp counter and makes older stamps read unverified (`decision/dialog-answers-need-verify-stamp`, `contract/cli-exit-codes/12`). Any added Run B step follows this pattern and needs its own ratification.

@history_item n=9 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — Run B's fifth compiled paste, eighteen stamp rows. A new ledger row and its probe widen the ratified verify child set, a widening the founder answered at P4. A stamp that holds exactly the older id set, all `pass`, reads `cli_verified:false`. Why it matters: this is the stamp-growth precedent a survey ledger row repeats (`decision/unverified-cli-gate`).

@history_item n=10 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — Run B's probe --settings, the statusline capture arm, the relied set. Behaviour relied on with no ledger row was owed rows; the stamp is read over the relied set. Why it matters: this is the precedent for the CLAUDE.md invariant the scope cites, that every reliance on undocumented `claude` behaviour is a ledger row with a probe. It is also the relied-set rule a survey row joins.

@history_item n=11 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — Run E in the verify child set. A new verify child run widened the ratified child set, which the founder answered at P4. Its probe dir and guard, the stamp `/22`, and the transcript residual were measured. Why it matters: if the survey probe needs a new run or start rather than a Run B step, this is the precedent and the ratification it needs (`decision/verify-probe-dirs-cleanup`, `decision/probe-artifacts-cleanup`).

@history_item n=12 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — the claude agents --json child of list, revive and verify Run B. Run B's one bounded read widened verify's child set. The ledger went to twenty-three rows, and a stamp of exactly the twenty-two pre-chunk ids reads unverified. Child output is parsed tolerantly into a closed failure. Why it matters: this is the stamp-growth precedent plus `decision/child-output-bounded` / `decision/child-output-tolerant` for any probe read.

@history_item n=13 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — Run B's sixth paste, its residual, and the twenty-four-row stamp. The paste only goes into a settled input box. The transcript residual was measured per round, and the stamp moved to `/24`. Why it matters: these are the current stamp-count arithmetic and the residual-accounting precedent a survey row and probe must extend (`decision/verify-probe-captures-transient`).

@history_item n=14 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder's Epoch 5 batch: the card confirms his. Run B's added pastes, probe settings, agents read and Run E stand on his own word, and the delegate marks are superseded. Why it matters: verify-child widenings rest on the founder's own word, and a delegate's confirm is only PROVISIONAL. A survey probe's widening needs the same.

@history_item n=15 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — verify's dirs removed on a Ctrl-C / TERM stop. Probe dirs are removed when verify passes, fails or is stopped. Each guard removes only what its own run created. SIGKILL or a crash leaves the dirs to be removed by hand. Why it matters: `decision/verify-probe-dirs-cleanup` and `decision/probe-artifacts-cleanup` bind any new probe dir or capture the survey probe adds.

@history_item n=16 marker=2026-10-18-plugin-precedence-live-readings
**2026-10-18-plugin-precedence-live-readings** — the live round's rig outside the work tree, its trust keys and residual. The rig passes the folder-trust dialog by keys into its own pty only: at most 2 arrows and 1 Enter on a read `selected`, and no key on any other screen. The founder ratified this with a fixed start count. Why it matters: this is the precedent for any key a live rig types to make the survey appear or to answer it. The scope says such keys wait on his word at P3's end.

@history_item n=17 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — added `decision/live-round-rig-keys-own-pty` and `decision/live-session-screens-kept-local`. A live round's rig types only a bounded key set into its own pty, on the founder's word for that chunk and anew for each round, and reads `statusLine.command` only by a read-only hash. Live screen frames stay under a gitignored local path, and only closed-word summaries land. Why it matters: these are the binding rows for this chunk's live starts. Any survey key a rig types is outside the listed key set, so it needs the founder's fresh word. Any survey screen captured live stays local.

@history_item n=18 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — the scratch footer reading of a newer CLI accepted, and Run B's pastes going into rows that hold an input-box literal. The scratch reading is one tmpfs-home start with no key typed, keeping only scrubbed footer rows. It is no product path and no standing permission, so another reading needs its own answer. Why it matters: `decision/scratch-footer-reading-residual`. A survey-screen reading on the installed CLI is a new reading that needs its own answer.

@history_item n=19 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — two scratch measurements across the credential boundary. Its standing rule: the founder's answers are spent, and another such probe needs its own showing. Why it matters: a live measurement of the survey grammar cannot borrow earlier scratch answers.

@history_item n=20 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — verify's print-mode probe residual accepted. The CLI's empty project dir under `~/.claude/projects/` was accepted into the class of the verify transcripts, and viola itself still writes nothing under `~/.claude`. Why it matters: any new residual a survey probe leaves under `~/.claude` must be measured and accepted the same way (`decision/no-writes-under-claude-home`, `decision/nothing-written-under-claude-home`).

@history_item n=21 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** — the live round's residual under ~/.claude. The CLI's own writes under `~/.claude` during a live round are an accepted residual, ratified by the founder. viola itself writes nothing there. Why it matters: `decision/viola-writes-nothing-in-claude-dir`. If making the survey appear changes a CLI setting, it is the CLI's write, never viola's, and needs its own word.

@history_item n=22 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — changed `decision/screen-fixture-signature-rows-only` and added `decision/input-box-readiness-from-literals`. A screen fixture keeps only signature rows. A box rule row is kept as its rule chars alone and a prompt row as the prompt char alone. Any other box row, a kept row the scrub would rewrite, or a username split across a row seam is refused. Readiness reads the input box's rows from compiled literals. Why it matters: a recorded survey screen under `fixtures/claude/<version>/` must fit this signature-row form. A survey signature literal joins the compiled set the same way.

@history_item n=23 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — changed `decision/compiled-classification-sources`, so local commands are classified by compiled name. It added `decision/verify-echo-probe-residuals`: a seventh Run B paste and its residuals as standing rows. Why it matters: a survey screen signature must come only from a compiled `viola-agent-claude` ledger row, never from runtime text. This is the most recent verify-probe-and-residual precedent, still PROVISIONAL on a delegate card confirm.

@history_item n=24 marker=2026-09-29-fake-agent-drift-contract
**2026-09-29-fake-agent-drift-contract** — the cross-session tag filed as harness origin. The compiled-prefix classification set was widened by a ratified widening, with its side effect shown. The first live test measures it and owns the ledger row. Why it matters: this is the precedent for widening a closed classifier set. Show the side effect (a human key misread), ratify it, and back it with a measured ledger row.

@history_item n=25 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — Authentication & Authorization: `decision/unverified-cli-gate` gained an except clause. StopFailure is registered with no capability ledger row and no verify probe. Why it matters: this is the one named exception to the ledger-row-with-probe rule. The survey grammar takes a row and a probe, not another exception, unless the founder rules otherwise.

@history_item n=26 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — the StopFailure no-ledger-row answer ratified by the founder. The `decision/unverified-cli-gate` except clause now stands on his own word, with its live reading owned by a recorded-fixture entry. Why it matters: this is the current standing of the gate row this chunk's ledger row is read against.

@history_item n=27 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — Threat Model Summary: changed `decision/child-output-tolerant` and `decision/child-output-tolerant-unknown`, so a failure becomes unknown or a closed failure word. Why it matters: these are picked rows. Any survey-probe read of child output fails to unknown or a closed word.

@history_item n=28 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** — the `FAKE_AGENT_PUMP_DELAY_MS` carve-out. A `fake-agent`-only test seam is compiled out of release builds, configures nothing and disables no control. Another seam needs its own Decisions Log entry. Why it matters: if a fake-agent survey screen for CI needs a switch, it must stay inside the fake agent's own scripted behaviour. A new environment seam needs its own entry (`decision/test-seams-fake-agent-only`, `decision/new-env-read-needs-log-entry`). The scope also forbids any env, flag or config that widens the list.

@history_item n=29 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** — test seam `FAKE_AGENT_HOOK_PANIC` and G2's exact-path exemption, ratified by the founder. A second seam was added, and G2 admits exactly one exemption. Why it matters: `decision/g2-single-panic-exemption` and `decision/g2-zero-panics-single-exemption`. A classifier or screen change must add no panic path, and any further exemption needs its own entry.

@history_item n=30 marker=2026-09-27-epoch-2-cleanup
**2026-09-27-epoch-2-cleanup** — test-only feature proven absent. `scripts/release-check.sh` refuses `test-support` / `fake-agent` artifacts (probe 5/5). Why it matters: `decision/release-build-without-test-features` and `decision/test-seams-compiled-out`. The fake agent's survey screen must stay out of the release build.

@history_item n=31 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — the terminal colour facts NO_COLOR / TERM / COLORTERM join the admitted env set. Another environment read needs its own Decisions Log entry. Why it matters: `decision/new-env-read-needs-log-entry` and `decision/controls-not-switchable`. No env var may widen the non-editing list, which the scope also states.

@history_item n=32 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — the hidden capture arm, verify's probe captures and the fixture scrub, plus verify's CI split. Probe captures are transient 0600 files in a 0700 per-pid dir. `--record` scrubs and refuses. The real CLI runs only locally, and verify runs in CI only against the fake agent. Why it matters: `decision/real-cli-local-only`, `decision/ci-no-secrets-no-real-claude`, `decision/verify-probe-captures-transient`. CI coverage for the survey comes from the fake agent and recorded payloads only.

@history_item n=33 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** — the R8 persistent-environment exemption and the identity floor. Why it matters: `decision/r8-claude-env-strip` and `decision/parent-identity-env-stripped` bind every verify child, the survey probe included.

@history_item n=34 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — several changes, each relevant here. The R8 strip and the fixture scrub now match names exactly. The relied set is all twenty-four rows on every build (`/24`). CI runs one ubuntu-latest leg. Why it matters: these are the current base for the R8 strip rows, the stamp arithmetic a new survey row moves, and the CI where the fake-agent survey runs.

@history_item n=35 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — run's strict stamps read; decisions on the six-row stamp. A non-`null` decision is never given unless `run`'s gate read the stamps through the strict check. Why it matters: `decision/dialog-answers-need-verify-stamp`. A new survey row joins the stamp that gates every non-`null` decision, so homes stamped before this chunk read unverified (exit 12).

@history_item n=36 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — `dialog_hold_s` validated into `DialogHold`. No value makes the hold zero or unbounded, and no env var or flag sets it. Why it matters: this is the source of `decision/config-cannot-disable-controls`.

@history_item n=37 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — Input Validation: changed `decision/config-cannot-disable-controls`, dropping the budget-gate clause. Why it matters: this is the current text of a picked row. No configuration value may widen the non-editing list either.

@history_item n=38 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — `ledger/stamps.json` is checked before `ledger/` is made, and the new ban says never run the check after a create, a mode set or a log open that narrows the path. Why it matters: `decision/strict-check-before-narrowing` and `contract/strict-modes-refusal/verify`. verify's order must hold if the survey probe adds a dir or a write.

@history_item n=39 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — the error bodies' census and the product error enums by census. `product_error_writers_are_each_named` and `thiserror_messages_interpolate_nothing`. Why it matters: `decision/every-error-writer-named` and `decision/error-messages-interpolate-nothing`. A new refusal or error from the survey probe or the classifier must be named and must interpolate nothing.

@history_item n=40 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters as rows. The master's text became verified rows, with fates given by the founder. Why it matters: the picked rows came out of this conversion. An amendment this chunk makes names rows by handle (`+` / `~` / `−`).
