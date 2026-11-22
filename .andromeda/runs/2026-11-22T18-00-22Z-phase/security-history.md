@history_item n=1 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** (fuzz lockfile exemption) — set `fuzz/Cargo.lock` outside `cargo deny` as a ratified test-only exemption, with its own advisory and source audit. This is the except of decision/cargo-deny-gate. The chrono re-measure must not treat the fuzz graph as covered by the root deny run.

@history_item n=2 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** (`FAKE_AGENT_PUMP_DELAY_MS` carve-out) — first fake-agent-only test seam (pump delay, capped, compiled out of release), plus the ban on any further seam without its own Decisions Log entry (decision/new-env-read-needs-log-entry, decision/test-seams-fake-agent-only). Trap: measuring `SendSlot::confirm` "under a failing snapshot write", or the paste-writer survivors, must not add a new env seam. Use injected values or unit tests.

@history_item n=3 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** (test seam `FAKE_AGENT_HOOK_PANIC`, ratified by the founder) — the second and last test seam. Each such seam was a boundary widening ratified live by the founder. A third seam added for this chunk's measurements would need the same.

@history_item n=4 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** (the mutation legs retire from CI) — CI runs no mutation job; mutation belongs to the epoch-boundary audit (decision/mutation-testing-at-audit). Items 2, 3 and 6 (killing tests for survivors, the `mutants` nextest profile) must keep mutation out of CI, and the chunk runs no mutation run.

@history_item n=5 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** (the vt100 degrade on the PTY output row) — the feed runs under catch_unwind; a caught panic poisons the model and send reads input-not-ready (decision/vt100-feed-under-panic-catch). Any `screen.rs` killing test or slot mend must keep this degrade.

@history_item n=6 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (send's input validation as landed) — the paste is one bracketed `write_all` through `viola-pty`'s `PasteHandle`, and `validate_paste_text` runs first on both sides. This is the base for the paste-writer survivors in `crates/viola-pty/src/pump.rs` and for the send slot.

@history_item n=7 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** (verify's typed-input PTY probe: dirs, residual) — Run B is trusted in `<cwd>/.viola-verify-<pid>/`. It leaves CLI transcripts under `~/.claude/projects/` as an accepted residual. The overseer once edited the `~/.claude.json` project flags by hand. This bears directly on item 7: a project entry pointing under a verify scratch root may be the CLI's own write, not viola's, and the measurement must tell the two apart.

@history_item n=8 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** (Run B's compiled pastes and its wait) — the framing pastes go only into a settled input box and wait up to `PROBE_DEADLINE` (120 s). This is the history of `framing_turns`' per-paste probes (discretionary item 8): a refactor into one step function must keep the box guard and the no-key rule.

@history_item n=9 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** (test-side removals through the home link) — test-side removals take only test-created dirs (decision/e2e-home-removal-own-dirs-only). Trap: neutralised keeper checks leave `backing/` in a mutated tree. This bears on item 5's boot supervisor test (binary copies, harness boot).

@history_item n=10 marker=2026-10-07-a-send-ending-in-a-newline-is-confirmed
**2026-10-07-a-send-ending-in-a-newline-is-confirmed** (trailing LF removed after validation) — the typed text is the only text send uses, for classification, confirmation, byte count and paste (decision/refused-text-never-stripped). A slot mend must keep confirmation keyed to the typed text. Any other removal needs its own ruling.

@history_item n=11 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** (the human's terminal on `viola run`'s stdin) — the stdin classifier's closed non-editing list (decision/human-typing-takes-wheel, contract/wheel-event/human-input). A paste-cut or slot end must leave the wheel rule as it is.

@history_item n=12 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (trailing CR and LF removed; an empty typed text refused) — the precedent of a corrective epoch-cleanup chunk landing a closed refusal (contract/send-refusal/empty-text) by the founder's ruling. A new closed word from this chunk is the operator's fork, as the scope says.

@history_item n=13 marker=2026-10-09-inner-cr-and-crlf-in-a-sent-text
**2026-10-09-inner-cr-and-crlf-in-a-sent-text** (inner CR or CR LF typed as one LF) — set decision/typed-text-line-ending-normalisation. The typed text alone feeds delivery confirmation and the paste. It binds any test or mend on the slot's confirm and the paste writer.

@history_item n=14 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** (verify's dirs removed on a Ctrl-C / TERM stop) — verify's guards remove only what their own run created, never by pattern or listing, and no remover of an earlier run's dirs exists. SIGKILL leaves them by hand. Item 7's mend, if any, must not add a pattern sweep of earlier runs' leftovers.

@history_item n=15 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (the claude agents --json child of list, revive and verify Run B) — set up Run B's one bounded agents read: direct spawn under R8, stdout through `take(MAX_FRAME + 1)`, killed at 5 s, closed `oversize`/`malformed`, and the agents-probe pair holding no row text, id, cwd or path. Item 4 gives this read a bound of its own (`src/run/agents.rs:39`). The 5 s kill and the fail-open in list and revive are what it changes or keeps. A new bound value is the operator's fork.

@history_item n=16 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the mcp process spawns claude agents --json) — the MCP `list` tool shares CLI list's agents read path (decision/claude-agents-resolved-from-path). A separate bound for verify's read must not move list's, revive's or MCP's bound unmeasured.

@history_item n=17 marker=2026-10-18-plugin-precedence-live-readings
**2026-10-18-plugin-precedence-live-readings** (the live round's rig, its trust keys and residual) — records that the CLI itself writes a `~/.claude.json` trust or project entry per accepted dialog, which viola does not write. This is a precedent for item 7: a project entry under a scratch root can be the CLI's own write.

@history_item n=18 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** (verify's print-mode probe residual accepted) — the print-mode probe leaves the CLI's empty project dir for `ledger/probes/<pid>/` under `~/.claude/projects/`, accepted into the residual class. This is the closest precedent for item 7's "named residual in the masters or a mend".

@history_item n=19 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** (e2e-home keepers refuse a backing below the headroom floor) — an earlier epoch cleanup added a refusal in the harness keepers that widens no boundary, with a compiled constant and no env, flag or config. This is a precedent for the harness changes of item 5 (`crates/viola-e2e/src/harness/boot.rs`).

@history_item n=20 marker=2026-10-26-tokio-rustsec-2026-0386-lock-bump
**2026-10-26-tokio-rustsec-2026-0386-lock-bump** (the RUSTSEC-2026-0386 red closed) — the precedent of an advisory closed by a lock bump to a patched version, `deny.toml` untouched and no ignore widened. This is the "found → the ignore leaves with the upgrade" path for chrono.

@history_item n=21 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** (the first-start race on the pinned copy closed) — decision/pinned-binary-placed-once: ETXTBSY on exec of a copy still open for write, closed by a no-clobber placement after the handle closes. Item 5 moves the boot test's binary copies out of the timed body, so this trap applies to any copy the harness execs.

@history_item n=22 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the bounded-reader census; the vt100 passthrough witness) — `product_readers_are_each_bounded_or_named` (decision/bounded-readers-census) requires every product reader to be bounded or named, so a new or reshaped read in `src/run/agents.rs` or `slot.rs` must pass it. `chaos_feed_panic_refuses_send_and_keeps_passthrough` guards decision/vt100-feed-under-panic-catch, and slot mends must keep it green.

@history_item n=23 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the error bodies' census and what it does not drive) — no `-32603` is driven in a scanned home; the unit layer holds it (decision/internal-error-held-at-unit-layer, decision/every-error-writer-named). The slot's never-settled claimed line under a failing snapshot write should be measured at the unit layer. Any new error writer must be named in the census.

@history_item n=24 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the tests holding the directory and peer checks, and no seam to drive them) — the precedent and ban: decisions are proven over injected values, never by an env, flag or feature seam that forces a failure. This applies to driving a failing snapshot write for item 1 and to the survivors' killing tests.

@history_item n=25 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (advisories red on RUSTSEC-2026-0441; the relied set on every build) — the precedent of an unmaintained advisory held red, owned by a CARRY, no ignore widened, its remedy a measured crate move. This is the shape of the chrono measurement. The relied set was also folded to one source (`LedgerRow::relied()`), and the counter literals moved repeatedly. This is a trap for discretionary item 9: the relied-row count must come from ONE source, never a written literal.

@history_item n=26 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (cargo-deny advisories green, the ignore list empty) — RUSTSEC-2017-0008 left with its crate as a narrowing, and the ignore list was emptied. This is a precedent that an ignore leaves `deny.toml` with the upgrade that clears it.

@history_item n=27 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (Dependency Security, Input Validation and Data Protection rows) — decision/ci-cargo-deny-gate now fails on any advisory not on its reasoned ignore list, the gate wording the chrono item rests on. Added decision/input-box-readiness-from-literals, one predicate for verify and run's gate, where the `screen.rs:126`/`:135` survivors sit. decision/screen-fixture-signature-rows-only keeps fixtures for new `screen.rs` tests signature-only.

@history_item n=28 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Input Validation: 3 rows) — added decision/hyprctl-read-deadline: a child deadline applied ahead of its measured need and held provisional. This is a precedent for item 4's own bound on verify's agents read: a new time bound is measured and named, never a retry.

@history_item n=29 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** (Security Anti-Patterns: test-removal-own-dirs-only) — widened decision/test-removal-own-dirs-only to another build's stamp dir under try_lock. This binds any harness cleanup change in item 5.

@history_item n=30 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** (Input Validation: 1 row; Threat Model Summary: 2 rows) — retired the exact-match row for decision/delivery-confirmation-trailing-tolerance (guard `submitted_form_takes_a_trailing_run_the_cli_trims`) and dropped the exact-equality why from decision/typed-text-line-ending-normalisation. Item 1's end for a claimed line whose settle never comes (`SendSlot::confirm`) must keep this tolerance and its guard.

@history_item n=31 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** (the send trailing-only tolerance ratified by the founder; the StopFailure no-ledger-row answer ratified) — the trailing-only tolerance is the founder's word, and a leading trim waits for a live reading. StopFailure stands with no ledger row. A slot mend must not widen the match, and a relied-row count must not count StopFailure.

@history_item n=32 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (Input Validation: 3 rows; Security Anti-Patterns: 1 row; Data Protection: verify-echo-probe-residuals) — added decision/transcript-read-bounded (count-only, no symlink, MAX_FRAME), which the slot's local-command confirm path reads. decision/compiled-classification-sources classifies local commands by their compiled name only, which ledger-side killing tests must hold. The echo probe's residual row is a sibling of item 7's residual.

@history_item n=33 marker=2026-11-17-supply-chain-green-and-booted-tests-end
**2026-11-17-supply-chain-green-and-booted-tests-end** (Dependency Security: 3 rows; the advisory rule widened) — added decision/advisory-ignore-grounds and decision/advisory-ignore-list, with RUSTSEC-2026-0543 (chrono) as the first ignore on the unmaintained-upstream ground. The re-read at each epoch boundary is that ground's only control. The entry also records as unmeasured whether cargo-deny keeps ignoring an id once a safe upgrade exists, which is exactly the measurement this chunk owes. The `deny.toml` reason line must keep "re-read at each boundary".

@history_item n=34 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Data Protection, Threat Model Summary, Secret Management and Input Validation rows) — verify writes its own trust record into the Claude config and removes only that record, and a SIGKILL leaves it. The SIGKILL leftover was added to the excepts of decision/no-writes-under-claude-home, decision/nothing-written-under-claude-home, decision/verify-probe-dirs-cleanup and decision/probe-artifacts-cleanup. The entry also added decision/trust-record-writer-guarded and decision/trust-record-closed-words, and verify reads CLAUDE_CONFIG_DIR only when kept. This is the core of item 7: the measurement must say whether the project entry found is this record or another write, under a test-owned config chosen this way. A new outcome word is the operator's fork.

@history_item n=35 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (Data Protection, Threat Model Summary and Input Validation rows) — run's and revive's start set the trust flag of an untrusted spawn folder, which stays, through the same guarded writer. The cwd stands only in the snapshot and as a trust-record key. The closed words are 14 for the writer and 16 for the start. Item 7 must tell verify's removable record apart from a start's kept flag and the CLI's own project entries, and must print no path.

@history_item n=36 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** (Threat Model Summary and Data Protection: the rig-only first-screen except) — the excepts of decision/verify-signature-only, decision/verify-screen-read-against-literals and decision/screen-fixture-signature-rows-only. These bind any `screen.rs` and framing-turns change.

@history_item n=37 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** (Authentication & Authorization: unverified-cli-gate) — decision/unverified-cli-gate counts every relied row: pass, or absent on browser-tools-prompt alone. Items 2 and 9 (ledger survivors, relied-row count from one source) must keep this counting rule.

@history_item n=38 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** (the StopFailure exception settled, kept on the founder's word) — the except of decision/unverified-cli-gate: StopFailure is registered with no ledger row and no verify probe. A relied-row count from one source must not count it.

@history_item n=39 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** (Data Protection: 3 rows; Threat Model Summary: send contract) — added decision/verify-context-probe-residual, accepted only as measured. That is a precedent for wording item 7's residual. contract/wrapper-channel-methods/send answers ok with confirmed false (unconfirmable) for a local command sent before its transcript exists, so item 1's new end for an unsettled claimed line must fit the send contract's closed outcomes.

@history_item n=40 marker=2026-11-20-a-quoted-modal-never-blocks-a-send
**2026-11-20-a-quoted-modal-never-blocks-a-send** (Input Validation and Threat Model Summary rows) — decision/input-box-readiness-from-literals moved to "never while a standing modal is up" and gained the quote except. The verify read rows now read compiled shapes over literals. These are the rules the `screen.rs` survivors' killing tests and the framing-turns step function must pin.

@history_item n=41 marker=2026-11-20-harness-prefix-and-identity-floor-ledger-rows
**2026-11-20-harness-prefix-and-identity-floor-ledger-rows** (Input Validation, Data Protection and Secret Management rows) — set decision/capture-arms-isolated (env names only, never a value) and decision/env-fixture-names-only (guards `env_fixtures_pass_hygiene`, `record_refuses_an_env_name_carrying_the_user_word`). Also set decision/env-probe-names-and-counts-only, the thirteen-name identity floor and decision/non-identity-hook-names. These are what `env_names_fault` in `ledger.rs` (survivor ×3, listed first) checks, and its killing tests must feed names only.

@history_item n=42 marker=2026-11-21-cross-session-prefix-ledger-row
**2026-11-21-cross-session-prefix-ledger-row** (Threat Model Summary, Data Protection and Secret Management rows) — decision/verify-children-direct-spawn gained Run B's accept literal and the sender's uds address. Item 4's agents read inside Run B and the framing turns sit in this ratified child set, which must not widen. Added decision/verify-cross-session-probe-residuals (closed-word files a SIGKILL leaves) and decision/verify-runtime-dir-held-in-memory: siblings for item 7, whose output must hold no path.

@history_item n=43 marker=2026-11-21-mcp-calls-bounded
**2026-11-21-mcp-calls-bounded** (Error Handling: call-deadline) — added the closed cause `call-deadline` with a new call bound. This is the latest precedent that a new time bound and its closed word are a fork. It applies to item 4's agents-read bound and item 1's slot end.

@history_item n=44 marker=2026-11-22-paste-write-bounded
**2026-11-22-paste-write-bounded** (Threat Model Summary: 2 rows; Error Handling: paste-cut) — the paste-cut excepts on decision/bridge-send-one-bracketed-paste and decision/send-text-is-one-bracketed-paste (`PASTE_BOUND`), plus contract/channel-refusal/paste-cut. Item 3's killing tests for the `pump.rs:191/:337/:372` survivors pin this cut behaviour.

@history_item n=45 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (Epoch 4 ratifications; Input Validation and Threat Model Summary rows) — the founder ratified the widened advisory rule and the chrono ignore, and the SIGKILL-leaves-the-trust-record excepts. Item 7 therefore extends a ratified residual, and the chrono re-read is owed at this boundary. decision/config-cannot-disable-controls now says no config lifts the unverified CLI gate. The verify signature rows gained the full compiled-shape parenthetical, the latest state for `screen.rs` and framing-turns work.
