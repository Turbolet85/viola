@history_item n=1 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** (the loopback GUI leaves 0.1.0) — retired the HTTP GUI (token, cookie, Host allowlist, CSP, SSE) and left API Security the stub «The frontend is deferred to 0.2.0; its technology is not chosen»; the listener ban stands. This is the stub the console's surface section replaces: the console must add no listener or HTTP surface and no GUI secret.

@history_item n=2 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (version census 0.2.0) — retired the frontend stub line from API Security and Windows signing: the founder confirmed 0.2.0 has no web or graphical surface and Linux is the one target. The console's surface is a terminal surface inside that ruling; any surface wider than a TUI in a terminal is a widening.

@history_item n=3 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — admitted NO_COLOR / TERM / COLORTERM as the only colour facts (read by list's human mode, compared only, never logged), with COLUMNS / CLICOLOR / CLICOLOR_FORCE / CI not read and width from `viola_pty::host_size()`; any other env read needs its own Decisions Log entry. A console that reads terminal size or colour, or a new env, extends rows decision/terminal-colour-facts, decision/no-width-or-colour-override-env and decision/new-env-read-needs-log-entry: a widening and a fork.

@history_item n=4 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (verify's stop and the colour decision lose their Windows arms; strict-modes owner and mode alone; CI one ubuntu leg) — the colour decision lost its Windows VT arm, the strict-modes check is owner and mode alone (an unstattable home a refusal), and CI runs one `ubuntu-latest` leg. The console's colour, strict-modes and test legs are Linux-only from this base.

@history_item n=5 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — made `list` a strict-modes entry point that checks the home, `instances/` and every instance before opening any file and before its log, refusing the whole board, and set `escape_row` to escape every control char as `\xHH` so each row is one line. This is the precedent for any console that reads the board (decision/list-scan-checks-before-open, decision/human-output-escapes-controls, decision/escape-terminal-controls).

@history_item n=6 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the board's one rule on the home; the client's peer euid check and the order at a seam) — the board holds the home to the read rule (`check_read`), so a home another user may read is refused with or without an instance; every client seam checks the socket directory, connects, decides the peer euid, then verifies the server. These rows bind a console that scans the board or sends frames (decision/list-home-read-rule, decision/board-home-read-rule, decision/frame-connect-check-order). NEVER add a seam that forces a refusing peer decision.

@history_item n=7 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the board's read rule confirmed; portable-pty pinned =0.9.0; the error-enum exceptions and the thiserror census) — the founder confirmed the board's read rule; portable-pty is pinned `=0.9.0` with no advisory ignore; the thiserror derive census counts product sources at nine (the test-only harness outside it). A console PTY driver rides the `=0.9.0` pin. A new product error enum moves the census count, and the error-enum convention must hold.

@history_item n=8 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** (the arm's check before its log) — trap: obs init sets an existing home to 0700, and the chunk measured that a check made after it passes a 0770 home. So the instance check has to run before the role's log opens. The console's own diagnostics role must keep this order (decision/strict-check-before-narrowing, decision/strict-refusal-leaves-no-trace).

@history_item n=9 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (the two instance checks ahead of any log) — fixed `revive`'s and `hook statusline`'s checks ahead of any log, because a log opened before or after a refused check narrowed the home and erased the state it refuses. This is the rationale behind decision/start-checks-home-tree-before-log-opens and decision/strict-refusal-leaves-no-trace.

@history_item n=10 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (`viola link` / `viola unlink`'s instance check before any log) — trap for later verbs: opening a role file narrows an existing home to 0700, so a check placed after `client::start` passes a 0770 home; the witness reads the mode after the refusal. A new console verb with its own role must check before its role file opens.

@history_item n=11 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the eighth dated gap; the tokio ban's tree form and the rmcp release-graph gate) — `mcp` checks its home and `diagnostics/` through `check_path` before anything under the home opens (exit 1, nothing printed, no role file). The tokio ban runs per sync crate as sole root, with `viola-channel`'s Tokio client behind its feature and covered by `channel-tokio-free.sh`. This is the precedent for a new role's start check, and the tokio ban binds any async terminal or event library the console draws in (decision/sync-crates-tokio-free, decision/no-tokio-in-sync-crates).

@history_item n=12 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the diagnostics opens refuse a symlink or a widened path) — `open_role_file` / `write_detail` run `check_diagnostics` before anything is narrowed; a refusal is a failed open, so the role writer goes to a sink and the detail line is dropped. The console's own diagnostics role inherits this (decision/diagnostics-open-fails-to-sink, decision/diagnostics-strict-modes-fail-open).

@history_item n=13 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (`check_instance`'s set and the read rule; the home tree checked at `run` and `revive`'s start) — set the read rule on the home, the instance and `diagnostics/`, with fixed `Readable` / `Symlink` refusals, and `check_home_tree` before the wrapper log (exit 1, no log, no line, no child). This defines the strict-modes rows and refusal messages the console's checks reuse (contract/strict-refusal-message/readable, contract/strict-refusal-message/symlink, contract/strict-modes-refusal/run-revive).

@history_item n=14 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (`ledger/stamps.json` checked before `ledger/` is made) — added the ban: never run the strict-modes check after a create, a mode set or a log open that narrows the path it checks. This is the standing order rule (decision/strict-check-before-narrowing, decision/stamps-checked-under-lock, decision/verify-home-tree-check).

@history_item n=15 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the snapshot readers move onto `read_snapshot_or_replay`) — no product caller uses `read_snapshot`; every reader goes through the classified read or the replay after its strict-modes check. A console that reads state for its view or its test exposure must take this path (decision/classified-snapshot-read-only, decision/state-reads-after-strict-modes).

@history_item n=16 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the six CLI frame verbs share `link` / `unlink`'s instance check) — `client::refuse_untrusted` runs `check_instance` first in every frame verb, exit 21 with the instance pair and no line. This is the precedent for any console action that sends a frame (decision/frame-verbs-instance-check-first).

@history_item n=17 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** — readers count torn lines, unknown kinds and unknown fields and never write; the next append heals a torn last line. A test driver that asserts on the console's log lines, or a console reading `events.ndjson`, reads under these rules (decision/readers-tolerate-torn-lines, decision/events-reader-bounded-lines, decision/state-readers-never-write).

@history_item n=18 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — trap: vt100 0.16.2 panics are reachable at real small sizes (a 24×1 screen with a wide character; 1×1; 1×2), so feeds run under `catch_unwind` and a poisoned model degrades. The quarter-screen layout's smallest size and its degrade rule must not lean on an uncaught vt100 or terminal-library feed at tiny sizes.

@history_item n=19 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the bounded-reader census; `viola mcp`'s stdin capped at `MAX_FRAME`) — `product_readers_are_each_bounded_or_named` requires every product reader to be bounded in its statement or named. The MCP stdin precedent is a per-line `MAX_FRAME` cap that refuses an over-long line whole. A console reading keystrokes from stdin is a new reader that the census will catch (decision/bounded-readers-census, decision/max-frame-cap).

@history_item n=20 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — added the attack-surface vector of the human's terminal on `viola run`'s stdin: a closed non-editing classifier, widened once by seven terminal reply shapes by exact grammar. A key reaching that stdin is the human's whoever typed it, and agent-typed keys needed the founder's word. This is the precedent for a console reading keys and for a keystroke driver typing them.

@history_item n=21 marker=2026-11-17-first-readings-on-the-standing-rig
**2026-11-17-first-readings-on-the-standing-rig** (Authentication & Authorization) — the live rig's key grant is a closed argv grant, renewed by the founder's word per chunk, and its stop needs no word. This is the nearest precedent for a driver that types keys into a pty. The scaffold's driver is fake-agent only, under a test-owned home and its own pty, so it stays outside this live grant.

@history_item n=22 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — a test-side removal under `target/e2e-home` takes only dirs the test side created; the keepers refuse a linked base unless it is a real owner-only directory. Trap: with the relative-target check neutralised a keeper makes `backing/` in the copied tree. The console tests' test-owned homes fall under this (decision/test-removal-own-dirs-only, decision/e2e-home-removal-own-dirs-only).

@history_item n=23 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** — added a fourth way a test-side removal knows its dir: another build's stamp dir whose lock it takes with `try_lock`. Ratified 2026-11-16T06-05-33-wrap. This is the current text of decision/test-removal-own-dirs-only that the console tests' home handling must respect.

@history_item n=24 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — the e2e-home keepers also refuse a backing below the 9 GiB headroom floor, with a fixed message. This is a constraint on where the console's test homes live.

@history_item n=25 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** — the `FAKE_AGENT_PUMP_DELAY_MS` test seam: fake-agent only, compiled out of release, configures nothing, disables no control; another seam needs its own Decisions Log entry. A test-only structured state read that the console exposes to its driver is a new seam under this rule (decision/test-seams-compiled-out, decision/test-seams-fake-agent-only, decision/new-env-read-needs-log-entry).

@history_item n=26 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** (test seam `FAKE_AGENT_HOOK_PANIC` and G2's exact-path exemption) — the second seam and G2's one exact-path panic exemption; any further seam or G2 exemption needs its own entry. This binds a test seam or a deliberate panic site in the console (decision/g2-single-panic-exemption, decision/g2-zero-panics-single-exemption).

@history_item n=27 marker=2026-09-27-epoch-2-cleanup
**2026-09-27-epoch-2-cleanup** — `test-support` is enabled only by the root dev-dependencies, and `release-check` refuses `test-support` / `fake-agent` artifacts (probe 5/5). A keystroke test driver binary or feature must stay test-only and be refused by the release build (decision/release-build-no-test-binaries, decision/release-build-without-test-features, decision/test-support-dev-only).

@history_item n=28 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** (registry migration) — moved release-check (refuses any executable other than `viola` and any test-only artifact) and the G2 exemption rule into the body. A new test-only binary for the driver trips release-check unless it is kept off the release graph (contract/release-check-output/test-only-feature).

@history_item n=29 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** — set the `deny.toml` four families and the sole-root `deny-sync.toml` tokio ban, each ban proven live by `deny-probes.sh`. Any new dependency that draws or reads a terminal (the scope's fork) passes this gate (decision/cargo-deny-gate, decision/ci-cargo-deny-gate, decision/c-building-crates-banned).

@history_item n=30 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** — exempted the fuzz workspace from `cargo deny` (test-only), committed both lockfiles and pinned exact versions. This is the exemption the cargo-deny gate row names; a new dependency must be an exact pin in the root graph (decision/exact-dependency-pins).

@history_item n=31 marker=2026-09-25-security-prerequisites
**2026-09-25-security-prerequisites** — admitted 0BSD only by per-crate exceptions, never through `allow`, each further exception needing its own entry. A terminal crate with a licence outside MIT / Apache-2.0 / Zlib / Unicode-3.0 is a boundary widening (decision/licence-allow-list).

@history_item n=32 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (one licence exception and one deny target triple) — 0BSD stays only for `doctest-file`; an exception naming an absent crate is dropped; `deny-sync.toml` runs for `x86_64-unknown-linux-gnu` alone. This is the licence and ban state a new terminal dependency meets.

@history_item n=33 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (serde_json's `unbounded_depth` banned; deny-probes 14/14) — the `[[bans.features]]` rows and the probe-count rule (`probes=(` rows plus the planted channel probe). A new ban for a terminal crate moves this count.

@history_item n=34 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (Dependency Security) — the ignore list went from empty to three reasoned rmcp ids, and the gate now fails on any advisory not on its reasoned list. This is the lineage of decision/advisory-ignore-list that the chrono carry `cry-h6vzhlce` re-reads.

@history_item n=35 marker=2026-11-17-supply-chain-green-and-booted-tests-end
**2026-11-17-supply-chain-green-and-booted-tests-end** — widened the advisory rule to a second ground (unmaintained, no safe upgrade, brought in upstream), added RUSTSEC-2026-0543 (chrono) and made the re-read at each epoch boundary that ground's only control. It was unmeasured whether cargo-deny keeps ignoring an id once a safe upgrade exists. The carry `cry-h6vzhlce` is exactly this re-read: an upgrade found removes the ignore, none keeps it one more epoch on the operator's word (decision/advisory-ignore-grounds, decision/advisory-ignore-list).

@history_item n=36 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (the advisory rule widened, ratified by the founder) — ratified decision/advisory-ignore-grounds and decision/advisory-ignore-list and the widened rule text. The chrono ignore stands on the founder's word and is re-read at the boundary this chunk carries.

@history_item n=37 marker=2026-10-26-tokio-rustsec-2026-0386-lock-bump
**2026-10-26-tokio-rustsec-2026-0386-lock-bump** — an advisory was closed by a lock bump with no ignore widening, and `deny.toml` was left untouched. This is the precedent for the chrono re-read: prefer an upgrade over keeping an ignore.

@history_item n=38 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (cargo-deny advisories green, the ignore list empty) — RUSTSEC-2017-0008's entry left with its crate (a narrowing): an ignore leaves when its subject leaves. This is the precedent for removing RUSTSEC-2026-0543 if chrono has a safe upgrade.

@history_item n=39 marker=2026-09-24-three-os-ci-headless-harness-skeleton
**2026-09-24-three-os-ci-headless-harness-skeleton** (pinned actions and toolchain source) — set the workspace `rust-version` floor at 1.96, checked in CI. A terminal crate must build at the 1.96 floor (decision/msrv-floor).

@history_item n=40 marker=2026-09-24-observability-gates
**2026-09-24-observability-gates** — CI runs the artifact canary scan over test homes before every upload, with a diagnostics-file-mode class. A new diagnostics role file in a test home is scanned (contract/artifact-canary-scan-classes/diagnostics-mode, contract/artifact-canary-scan-classes/content-canary).

@history_item n=41 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — narrowed the canary classes to the stripped CLAUDE* needles, the content canary and the non-0600 diagnostics file, removing the GUI-only forms. These are the classes a console role's files meet (contract/artifact-canary-scan-classes/claude-stripped).

@history_item n=42 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** (Data Protection) — the event, heartbeat and diagnostics files' owner-only append open never follows a symlink and sets 0600 through the handle. The console role's diagnostics file inherits this (decision/logs-owner-only).

@history_item n=43 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the home strict-modes check is owner and mode alone) — 0700 dirs, 0600 files, the pinned `viola` 0700; an unstattable home is a refusal; a diagnostics file takes the read rule. This is the current text of decision/home-owner-only, decision/private-file-modes and decision/unstattable-home-refused.

@history_item n=44 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the error bodies' census; the product error enums, by census) — every error body is enumerated with its writer and witness, an unnamed product error writer fails `product_error_writers_are_each_named`, and an interpolating `thiserror` message fails. A console error output or enum must be named in the census (decision/every-error-writer-named, decision/error-messages-interpolate-nothing).

@history_item n=45 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the catch site's document under --json) — `json_asked` reads `--json` before `--`, and the catch site prints the internal-error document. `run`, `revive`'s start, `mcp` and `hook` stay silent there. Whether the console's start verb is silent at the catch site or prints the document is a choice against contract/internal-error-output/json.

@history_item n=46 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (Error Handling; Threat Model Summary; Data Protection) — adopt joined each list of starts in turn: the home tree before log, socket dir before bind, the silent internal-error roles, the trust-record write. This is the precedent for how a new starting verb (the console's) cascades into every row that names the starts.

@history_item n=47 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (Threat Model Summary) — `open` joined decision/state-tree-strict-modes-before-use as a verb that holds the home tree before any log. A console verb that reads under the home joins this row the same way.

@history_item n=48 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** (Threat Model Summary, the tx 4464 correction) — trap: `focus` was wrongly listed among the home-tree verbs, because it runs the instance check, not `check_home_tree`, and was moved to decision/cli-frame-verbs-strict-modes-first. Name the console's check by what it actually calls.

@history_item n=49 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (Threat Model Summary) — `viola up` put the recorded cwd on its terminal's argv, widening decision/cwd-only-in-snapshot and decision/spawn-cwd-snapshot-only, provisional until ratified (2026-11-23-viola-open-2 tx; 2026-11-27T19-07-11Z-wrap). Any console display or spawn of a recorded cwd is a further crossing of these rows.

@history_item n=50 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (Data Protection; Threat Model Summary) — the start writes the trust flag of an untrusted spawn folder, skipping the home folder and trusted folders, and the cwd keys the trust record in the Claude config. This produced the current exceptions of decision/nothing-written-under-claude-home, decision/no-writes-under-claude-home, decision/viola-writes-nothing-in-claude-dir and decision/start-trust-skips-trusted-folders. The scaffold must not read or write the real `~/.claude*`.

@history_item n=51 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (Data Protection) — added decision/window-record-kept-to-snapshot-and-event-log: the window record leaves only through `list --json` and the MCP list item, and no address, pid or workspace enters a process-log line. This binds the console's own diagnostics role's lines (ratified 2026-11-27T19-07-11Z-wrap).

@history_item n=52 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — every CLI verb resolves its home from `--home`, else the checked `VIOLA_NAME` / `VIOLA_DIR` pair, else `~/.viola`, canonicalised when it exists and creating nothing. The console verb's home takes this order (decision/cli-home-resolution-order, contract/cli-state-root/home-flag, contract/cli-state-root/viola-dir).

@history_item n=53 marker=2026-10-18-plugin-precedence-live-readings
**2026-10-18-plugin-precedence-live-readings** (the VIOLA_DIR home step ratified) — the founder ratified the `VIOLA_DIR` step and canonicalisation for the CLI verbs and `mcp`. This is the standing basis of decision/cli-home-resolution-order.

@history_item n=54 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Authentication & Authorization) — decision/home-write-rule-checkers now names only `mcp`'s start and `plugin install` as checking the home to the write rule. A console role's start check must state which rule it holds the home to.

@history_item n=55 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (send's input validation as landed) — `validate_paste_text` runs client-side before any frame and again first in the wrapper, and the paste is one bracketed `write_all`. These are the paste rows carry `cry-boujwuyr`'s M1 (`send_through`'s guarded section, `framing_turns`) rests on (decision/paste-validation-on-both-sides, decision/cli-input-checked-before-frame).

@history_item n=56 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (trailing CR and LF removed; an empty typed text refused) — the `empty-text` refusal on both sides, and the trailing CR / LF removal. These are standing send rules an M1 refactor of `send_through` must keep (decision/empty-typed-text-refused, contract/send-refusal/empty-text).

@history_item n=57 marker=2026-10-09-inner-cr-and-crlf-in-a-sent-text
**2026-10-09-inner-cr-and-crlf-in-a-sent-text** — inner CR and CR LF are typed as one LF after validation; the founder's ruling; a later change typing a character the received text did not hold needs its own ruling. This bounds the M1 refactor of the send path (decision/typed-text-line-ending-normalisation, decision/refuse-control-characters, decision/reject-dont-strip-paste-text).

@history_item n=58 marker=2026-11-22-paste-write-bounded
**2026-11-22-paste-write-bounded** — a paste not finished within its compiled bound is cut (bracket closed, no CR, `paste-cut`). This is the current except of decision/bridge-send-one-bracketed-paste and decision/send-text-is-one-bracketed-paste, inside `send_through`'s guarded section that M1 looks at.

@history_item n=59 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — Run B's framing pastes and the stamp counter moving `/14` → `/17`, each stamp step recounted by hand. This is the origin of `framing_turns`' per-paste probes (M1) and of the hand-moved relied-row literals M3 replaces.

@history_item n=60 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the statusline shell-out, the settings override and the relied set hold on every build) — `LedgerRow::relied()` is all rows on every build, and each new row moved `/N` and `M pass  K fail` literals by hand across the plan. M3's one-source relied count changes how decision/dialog-answers-need-verify-stamp and contract/cli-exit-codes/12 are witnessed.

@history_item n=61 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** (Authentication & Authorization; Security Anti-Patterns) — the verified rule became "every relied row counts: pass, or absent on the browser-tools-prompt row alone". This is the current text of decision/dialog-answers-need-verify-stamp and contract/cli-exit-codes/12. M3's single relied-row source must keep the absent-counts exception.

@history_item n=62 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** (Security Anti-Patterns) — exit 12 `unverified-cli` now names `answer` or `switch`. This is the current promise of contract/cli-exit-codes/12 that the relied-row count feeds.

@history_item n=63 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** (Input Validation) — added decision/verify-stamp-first-screen-closed-words: the stamp records first screens only as closed words, whole numbers and literal ids. This is the stamped-set surface M3's "stamped-set table held once" touches.

@history_item n=64 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (Data Protection) — added decision/verify-echo-probe-residuals for Run B's seventh paste. M1's single step function over the per-paste probes must keep each paste's ratified residual unchanged.

@history_item n=65 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** (Data Protection) — added decision/verify-context-probe-residual for Run B's eighth paste (`/context`), accepted as measured. This is the last probe M1's per-paste step function folds.

@history_item n=66 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** (question option text crosses into the event and `wait` payloads) — R1's upstream-text list gained question and option text. Any console showing upstream text treats it as content, never a command (decision/upstream-text-is-content, decision/upstream-text-is-never-a-command).

@history_item n=67 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — verify's PTY children are direct spawns under the R8 strip; viola writes nothing under `~/.claude`. This set the standing child-spawn and credential rows (decision/claude-credentials-stripped-from-child, decision/real-cli-local-only). The scaffold boundary of no real `claude` keeps CI to the fake agent (decision/ci-no-secrets-no-real-claude).
