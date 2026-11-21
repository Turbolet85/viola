@history_item n=1 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — (1. A11y Scope Summary, 2 rows) narrowed `~ contract/tui-passthrough/human-keystrokes` and `~ contract/tui-passthrough/viola-terminal-bytes` to "once the wrapped child holds the terminal", so a start's automatic verify window (lines L1–L5 plus verify's own output) is the one exception. Why it matters: the chunk adds probes to `viola verify`'s runs and child set, and those run inside this window. Any new probe output or added wall time has to stay inside the stated exception and leave the child's terminal bytes and keys untouched. The rows are PROVISIONAL until the founder's own word.

@history_item n=2 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — (2. A11y Strategy) `~ decision/human-keystrokes-never-blocked` gained its except: the start's automatic verify window, before any child exists, where a key waits in the terminal's own buffer. Why it matters: a second session sending into a verify child, and any extra model turn the probe adds, lengthen that window. The chunk must keep it pre-child and not let it reach a wrapped session's keystrokes (PROVISIONAL).

@history_item n=3 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — `~ contract/viola-verify/step-line`: the step line went from pass|fail to pass|absent|fail, with `absent` allowed only on the browser-tools-prompt row. Why it matters: this is the precedent for a new ledger row adding a step-line word. The two new relied rows print `[NN/NN] … pass|fail`, and NN's total grows (27 to 29). Any absent-style word for them would need this row amended again (PROVISIONAL, delegate overseer).

@history_item n=4 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — set verify's interrupted pair: Ctrl-C or TERM before the rows are checked prints no step line and no summary, only `unable: verify was stopped before it finished` and `hint: run viola verify again to stamp this CLI`, then exits 1. The stop path adds no time bound. Why it matters: new probe runs and the second session must end at the stop with the same pair, and their kill and pump join must also end at once (`contract/interrupted-verify/*`, `contract/viola-verify/interrupted-pair`).

@history_item n=5 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (scope entities: the strict-modes refusals of `run`, revive's start and `verify`) over a failing home tree (home, `ledger/`, `bin/`, `plugin/`), `verify` refuses before its log opens with exit 1 and the home-state pair, and prints no step line or summary. `update_stamps`'s own check gives the same pair. Why it matters: the chunk extends the ledger and stamps that `verify` writes under `ledger/`, so `contract/start-refusal/home-strict-modes` must still hold before any new probe or capture write.

@history_item n=6 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the strict-modes refusals' text, witnesses and pinned pairs) the fixed instance-state and home-state pairs are pinned by `INSTANCE_PAIR` / `HOME_PAIR` literals, and `verify_refuses_a_widened_ledger_before_any_step` reads the home-state pair (`tests/cli_verify.rs`). Why it matters: these are the witnesses a widened verify (new runs, a capture arm) must keep green, and they rest on `decision/strict-modes-refusal-before-any-write`.

@history_item n=7 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (an unstattable home, per OS) a home under a regular file is a strict-modes refusal: verify gives the home-state pair with exit 1. Why it matters: this is the origin of the picked `decision/unstattable-home-is-strict-modes-refusal`, read by `verify_under_a_regular_file_refuses_the_home`. It also set the trap that a test planting a regular file above a checked path needs an oracle fitted to it.

@history_item n=8 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (strict-modes refusals, the Windows ACL and NotFound arms retired) the unstattable home is the home-state pair alone, and the strict-modes cases run on ubuntu-latest. Why it matters: it gives the current oracle for verify's strict-modes refusal that the chunk's verify changes must keep.

@history_item n=9 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the tui boundary on openpty alone) the zero-viola-bytes case compares the outer-PTY stream byte for byte with an unwrapped fake-agent run, and each boundary clause is its own nextest case on ubuntu-latest. Why it matters: the fake agent's replay of the new probes in CI must not perturb that byte-for-byte oracle (`decision/zero-viola-bytes-on-the-terminal`, `decision/boundary-tests-sequenced-on-records`).

@history_item n=10 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the a11y cases' CI home) the cli and tui cases run in the `test` job's single `coverage` suite on `[ubuntu-latest]`, gated `coverage,doctest`, with the runner `scripts/agent-run.sh run`. Why it matters: any new output-discipline case for the probe rows lands there (`decision/cli-invariants-pass-in-ci`).

@history_item n=11 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — `~ decision/no-screen-content-parsing`: the screen is parsed only for compiled signatures (footer literals, the input box's rule and prompt rows, modal literals), and the prompt row only for its opening char, never the text after it. Why it matters: the harness-prefix probe reads "its prompt's raw start" in a verify child. If that read touches the rendered screen and not a hook payload, it falls under this ban, which is a trap for the probe's shape.

@history_item n=12 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — `~ decision/no-screen-content-parsing` gained its first except: the test-only live rig reads the trust selector, box state and modals as closed words. Why it matters: the chunk's "stamped live" run goes through the live rig's verify arm. Any new screen read needs this except widened through an amendment, as each later chunk did.

@history_item n=13 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** — `~ decision/no-screen-content-parsing` except widened to the box row's cell attributes after a turn end, as closed words, never text. Why it matters: this is the precedent of a ledger-row chunk widening the screen-reading except for a live probe. The harness-prefix probe would need the same route if it reads anything on screen.

@history_item n=14 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — `~ decision/no-screen-content-parsing` except: the box row's cell attributes are read "after a turn end or send". Why it matters: a probe that sends with no turn end, as a second session sending into the verify child might, sits on this widening (PROVISIONAL).

@history_item n=15 marker=2026-11-19-a-fired-scheduled-wake-keeps-the-wheel
**2026-11-19-a-fired-scheduled-wake-keeps-the-wheel** — `~ decision/no-screen-content-parsing` except: adds box-row cell attributes "before any send", which gives the current picked wording. Why it matters: it is the latest standing form of the except that a new probe read must fit into (PROVISIONAL).

@history_item n=16 marker=2026-11-20-a-quoted-modal-never-blocks-a-send
**2026-11-20-a-quoted-modal-never-blocks-a-send** — `~ decision/no-screen-content-parsing` rule: the signatures include "the compiled shapes over them". Why it matters: this is the current rule text. A harness-prefix probe that detects a harness turn by screen must stay within compiled literals and shapes, never prompt text (PROVISIONAL).

@history_item n=17 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — `~ decision/run-diagnostics-off-terminal`: the start's trust step reports its word in one `trust-record` diagnostics line, never as a refusal, exit code or hint. Why it matters: this is the precedent for where a start-side or verify-side step reports its outcome. The identity-floor capture (env names) must likewise go to diagnostics or the stamp, never to the terminal, and never hold a value.

@history_item n=18 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — `~ decision/time-bounds-never-on-human` and `~ decision/time-bounds-fall-on-the-driver` add the start's Claude config lock wait, which falls on viola's own start. Why it matters: a second session or an extra model turn in verify adds time to the start's automatic verify. Any new bound must be listed in both rows and fall on viola's start or child, never on a human key.

@history_item n=19 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — `~ decision/time-bounds-never-on-human` and `~ decision/time-bounds-fall-on-the-driver` dropped the statusline deadline, and `− decision/statusline-output-passed-byte-for-byte`. Why it matters: it is the history of the time-bound lists the chunk may extend, and it shows the rule that every bound viola holds is enumerated with whom it falls on.

@history_item n=20 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — `~ contract/cli-exit-codes/11` was retired and never reused, `~ decision/time-bounds-fall-on-the-driver` dropped the budget pause, and four `−` rows removed the gate and config keys. Why it matters: the picked exit-code rows (10, 14, 2, 20, 21) and time-bound rows carry this history. A new probe refusal must not reuse exit 11.

@history_item n=21 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — added the 40 s `COMPACT_CONFIRM_WINDOW` for a `/compact` send on a verified CLI as a driver-only bound (`not-delivered` / `no-prompt-submitted`). Why it matters: it is the precedent of a capability-ledger-row chunk that added a time bound and amended §8's bound list. It is the source of the picked `contract/send-refusal/no-prompt-submitted`.

@history_item n=22 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — the readiness gate's `GATE_MAX_WAIT` falls on the driver, a human key during the wait takes the wheel, and the send is refused `human-typing`. This is guarded by `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins`, sequenced on records. Why it matters: harness-prefix turns "never move the wheel". A second session sending into a verify child must not register as human input, and this case's records sequencing is the pattern (`decision/human-key-takes-the-wheel`, `contract/send-refusal/input-not-ready`).

@history_item n=23 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — recorded the first live reading of the takeover clause as supplemental and never gating. Why it matters: it is the source of `decision/live-readings-never-gate`. The chunk's live stamp is evidence, not a CI gate.

@history_item n=24 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — recorded the takeover clause's second live reading on the product binary over a live `claude`, again supplemental and never gating. Why it matters: it is the precedent for how a live reading against a real CLI version is recorded beside the gating fake-agent cases.

@history_item n=25 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master became rows (decision, contract and absence), with relied rows 45 kept and 12 cut. Why it matters: it is the origin of every picked row's handle and of the row-form amendment entries (`+`/`~`/`−`) the chunk's Expected amendments must use.

@history_item n=26 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (the census of error writers and the body check) added `error_bodies_each_name_a_writer_and_a_witness_that_stand`, `product_error_writers_are_each_named` and `thiserror_messages_interpolate_nothing`, plus the `assert_error_body_clean` classes (`absolute-path`, `caused-by`, `serde-path`, `planted`). Why it matters: any new error writer for the probes or the capture arm must be named in the census. The planted-value body check is a model for proving no env value reaches a file, stamp, log or screen.

@history_item n=27 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (a peer's refusal screened to a code word; `unknown`'s detail) a peer reply's refusal or detail that is no code word reads `unknown` or nothing, and `unknown`'s detail is a code word or nothing. Why it matters: these are the source of `decision/peer-replies-screened-to-code-words` and `contract/code-word/format`. Any closed id the new `hook --capture` arm emits must be a code word.

@history_item n=28 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (the exit-cause table and one hint per code) added `no_two_codes_share_a_hint`, `no_hint_names_release` and `tests/cross_exit_causes.rs` as the exit-cause table. Why it matters: any new code or hint the chunk adds (for example a probe refusal) is held by these guards, which back the picked `decision/one-hint-per-code` and `decision/no-hint-names-release`.

@history_item n=29 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (exit 21's `--json` document names its cause; a lost or stale wrapper; a name with no endpoint) the closed `detail` words are `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost` and `server-verify-failed`. Why it matters: these are the origin of the picked `contract/instance-unreachable-detail/*` and `contract/cli-exit-codes/21` rows. A second session reaching the verify child through a viola endpoint would meet these refusals.

@history_item n=30 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — `~ contract/instance-unreachable-detail/unwrapped-name`: the symlink clause was dropped because a symlink reads `strict-modes-failed`. Why it matters: it gives the current form of a picked row.

@history_item n=31 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — (the CLI frame verbs' server-verify refusal; the plugin against an impostor wrapper) frame verbs refuse a server that is not the recorded wrapper with exit 21 and the server-verify hint last. A hook call meeting such a server sends no frame and fails open with exit 0, empty streams, under 1.0 s. Why it matters: the harness-prefix probe's second session may reach the child through a viola endpoint or a hook. These are the source of `contract/frame-verb-refusal/server-verify-failed` and `contract/hook-call/server-verify-failed`.

@history_item n=32 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — (the eight CLI frame verbs' server-verify refusal) widened the refusal to `link` and `unlink`. Why it matters: it gives the current verb list in `contract/frame-verb-refusal/server-verify-failed`.

@history_item n=33 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (a refused socket directory at the start, at the eight frame verbs, and at hook and MCP) a start refuses with exit 1 and the `unable:` / `hint:` fixed pair, frame verbs with exit 21 `socket-dir-refused`, and hook calls fail open with a WARN `hook-decision`. Why it matters: these are the source of `contract/start-refusal/socket-dir`, `contract/frame-verb-refusal/socket-dir-refused`, `contract/hook-call/socket-dir-refused` and `decision/socket-dir-refusal-before-any-frame`, which any probe path through the endpoint or hook must keep.

@history_item n=34 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (server-verify and socket-directory refusals, the macOS and off-Unix carve-outs retired) `tests/security_negatives_server_identity.rs` and `tests/security_negatives_socket_dir.rs` run on ubuntu-latest. Why it matters: these are the current witnesses for the picked refusal rows.

@history_item n=35 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the plugin against a strict-modes-refused instance) a refused hook call returns exit 0 at once with empty streams, under 1.0 s, with no `config.json` read, no stdin read, no frame, no line and no SessionEnd append. Why it matters: it is the source of `contract/hook-call/strict-modes-failed`. A new `hook --capture` arm for the identity-floor probe must keep this fail-open shape.

@history_item n=36 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — (the cli output-discipline checks on std::process::Command and literal pins; the §8 verification lines pinned by literal) cli checks drive the binary through `std::process::Command`, and every fixed CLI string, refusal layout and typed exit code is pinned by literal. Why it matters: new verify step-line labels or any new fixed string must take a literal pin (`decision/cli-a11y-by-literal-tests`, `decision/refusal-layout-literal-pins`).

@history_item n=37 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — built the colour spans and the colour decision (reads `NO_COLOR`, `TERM`, `COLORTERM` and the stdout TTY, with none under `--json` or a passthrough role) and the rule that each state is carried by its word. Why it matters: new verify lines must stay plain where SGR is barred, with `fail` and `unable` never coloured (`decision/colour-decision-inputs`, `decision/sgr-beside-its-word`, `decision/unable-fail-never-coloured`).

@history_item n=38 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — a passed `viola revive` enters the TUI passthrough boundary, and revive's refusals use the fixed `unable:` / `hint:` form. Why it matters: a revive start runs the same start and so the same automatic verify. It is the source of `decision/single-vendor-content-boundary` and the zero-SGR list naming a passed revive.

@history_item n=39 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — (the tui passthrough boundary after a contained vt100 feed panic) no viola literal reaches the outer terminal after a feed panic. Why it matters: verify children are read through the same vt100 feed (`contract/tui-passthrough/vendor-content`), and a probe that reads the screen inherits this boundary.

@history_item n=40 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — the CLI output-discipline invariants in crate tests are covered by the epoch-boundary mutation audit (`run --mutants`) and not a per-chunk gate. Why it matters: it is the source of the picked `decision/mutation-audit-covers-crate-invariants`. Crate-level probe tests meet mutation only at the audit.

@history_item n=41 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — cli and tui cases run as nextest cases in the `coverage` suite, with no a11y driver or suite value of their own. Why it matters: new probe cases must not add a separate a11y driver (`decision/cli-invariants-pass-in-ci`).

@history_item n=42 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — `~ decision/dialog-hold-bounds-the-driver` state went from provisional to standing after the live measurement. Why it matters: it gives the current state of a picked row among the time bounds a verify child's dialogs meet.

@history_item n=43 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — the census retired the frontend stubs: §5–§7 and the wholly-web keys read N/A, with no web or graphical frontend. Why it matters: it is the source of the picked absences and N/A overviews, so the chunk adds no web or graphical a11y surface.
