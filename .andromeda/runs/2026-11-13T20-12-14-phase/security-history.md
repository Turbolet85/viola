@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (the hidden `hook --capture` arm, verify's probe captures and the fixture scrub) — introduced `verify --record`'s scrub (home → `~`, user word → `<user>`) and its whole-recording refusal, nothing written, when a path or the user word survives. This is where item 2's `text_fault` comes from (`crates/viola-agent-claude/src/ledger.rs`). It checked paths and the user word only, and that is the gap item 2 closes. Also set the probe captures as transient 0600 files in a 0700 dir (decision/verify-probe-captures-transient).

@history_item n=2 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** (verify's typed-input PTY probe: the widening, its dirs, residual and refusal) — widened the fixture refusal to cover an email-shaped token and a username split across a row seam. It renamed the refusal to `a recorded fixture is not clean: <file> <code>` (`<file> row <n>[ seam] <code>` for a screen), with the closed codes `home-path` · `absolute-path` · `username` · `email` and never the content. The founder ruled this live. It is the origin of contract/fixture-refusal-codes/email and contract/fixture-refusal-line/form. Item 2 must reuse this closed code set and line form. It adds no new code (Boundaries: no new refusal reason).

@history_item n=3 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** (Run B's probe --settings, the statusline capture arm, the relied set and the kept workspace.repo) — Repository fixtures: a `workspace.repo` that names this repository's own remote is kept, neither rewritten nor refused, and nothing else in the rule widens. This is the founder's word. It is the `except` on decision/fixture-scrub-and-refusal and decision/fixture-hygiene. Item 2's email check at the record and on every fixture kind must not refuse that owner handle.

@history_item n=4 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (the claude agents --json child of list, revive and verify Run B) — added a recorded fixture kind: `--record` keeps only Run B's own row in `Agents.recorded-1.json`. This is one of the committed fixture kinds that items 2–4's email check and walks must reach.

@history_item n=5 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the `Mcp.` exchange recorded by the test-only tap) — `Mcp.` exchanges are recorded by `viola-mcp-tap`, scrubbed the same way and refused through `ledger::screen_fault` with the same closed codes, `email` included. The class is validated by `schemas/claude-mcp-fixture.v1.json` and hygiene-walked (decision/mcp-fixtures-via-test-only-tap). This is the precedent for item 2: the email check already runs through the screen path, while `text_fault` lacks it. It is also the existing `check_mcp` email check that item 3 extends to the hook, agents and statusline walks.

@history_item n=6 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (verify's refusal codes) — `verify` writes `unclean-fixture` among its closed refusal codes on `process-exit`, the fixture's `<file>` entering no code (contract/verify-refusal-codes/unclean-fixture). Item 2's email refusal at `--record` must surface through this existing code and add none.

@history_item n=7 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the R8 strip and the fixture scrub match names exactly; the registry reader retired) — the fixture scrub now matches the home prefix exactly, with no Windows case-fold. Item 2's added check runs beside this exact-match scrub.

@history_item n=8 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (Data Protection: 1 row) — `~ decision/screen-fixture-signature-rows-only`: screen fixtures keep a footer or modal literal whole, box rule rows as rule chars, and prompt rows as the prompt char. Another box row, a scrub-rewritten kept row or a seam-split username is refused. This is the latest change to the screen path (`screen_text_fault`/`has_email`) that item 2 generalises. The card confirm is PROVISIONAL (delegate overseer).

@history_item n=9 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (revive reads its own state: the instance check as an entry point, the replay's one caller, its arguments) — `revive`'s `<name>` goes through the shared name parser. It also introduced `strict::check_instance` (home, `instances/`, instance dir, `snapshot.json`, `events.ndjson`) and made the classified read and the replay revive's to call. This is the precedent for item 5 (`run` parses through the shared `parse_name`, decision/cli-names-through-name-check) and the base of item 6's checks.

@history_item n=10 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (`viola link` / `viola unlink`'s instance check before any log) — `<driver>`/`<driven>` go through the verbs' shared name parser. Trap: opening a role file narrows an existing home to 0700, so a check placed after `client::start` passes a 0770 home. The guard `link_verbs_refuse_a_widened_instance_before_any_log` reads the mode after the refusal, and its remove-the-guard control was 1 of 1 red. This bears on item 5's shared parser, item 6's check placement, and the guard-plus-control pattern of items 6, 11 and 12.

@history_item n=11 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the link lines the served methods write; the served `link` / `unlink` methods) — `link`/`unlink` lines carry `data {driver, driven}`, the two names only (contract/events-link-lines/data). `LinkSet` writes at restore, on a served `link` and on a served `unlink`. These are the lines item 12's replay guard `e5_the_replay_reads_the_link_the_wrapper_wrote` (`tests/cli_links.rs`) reads back.

@history_item n=12 marker=2026-10-10-session-links
**2026-10-10-session-links** (the link-set restore reads the log) — `viola_state::replay::apply_link_line` uses `data.driver`/`data.driven` only when both are strings, without re-parsing them as names. The replay's result holds the logged link set (`Replayed.links`), so a revive over a replayed snapshot reads the logged links. This is the exact behaviour item 12's replacement guard pins.

@history_item n=13 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** (own state files on read: three counts, the appender's heal, the replay as library code) — defined `read_snapshot_classified` and `viola_state::replay` (same bounds, write no file), and the torn-last-line heal on append (decision/torn-last-line-healed-on-append). This is the replay that row decision/replayed-snapshot-inert and item 12's guard rest on.

@history_item n=14 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the snapshot readers move onto `read_snapshot_or_replay`; the server check's reference values and the collision check read through the replay) — every product snapshot reader uses `read_snapshot_or_replay` after its strict-modes check, and `read_snapshot(` has 0 product callers under `src/`. A replayed snapshot carries no endpoint, pid, start time or statusline command, so it yields no frame and runs no command. Only a parsed snapshot gives reference values, and the collision check judges a replayed snapshot by its beat alone. These are the rows decision/replayed-snapshot-yields-nothing, decision/replayed-snapshot-fallbacks and decision/classified-snapshot-read-only, which item 12's guard keeps covered once the budget leg leaves.

@history_item n=15 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (budget.json read by the gate) — `read_budget` runs `check_path` on the home and the file under a shared lock and never writes. This is the budget leg whose `e5_the_replay_reads_what_the_wrapper_wrote` (`tests/cli_budget.rs`) is today the replay row's only guard. Boundaries: item 12 adds the replacement guard only and removes no budget gate.

@history_item n=16 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (`check_instance`'s set and the read rule as built) — `check_instance` holds the home, instance dir and `diagnostics/` to the read rule, and `instances/`, `snapshot.json` and `events.ndjson` to the write rule. `Refused` gained `Readable` and `Symlink` with fixed messages, and "a trusted state path is a symbolic link" is contract/strict-refusal-message/symlink. Item 6 makes `check_instance`/`check_stamps`/`check_path`/`check_read` refuse a link through `unix::reading` (`crates/viola-state/src/strict.rs`) and must reuse the existing `Symlink` variant and message, adding no new refusal.

@history_item n=17 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (`ledger/stamps.json` checked before `ledger/` is made, and `verify`'s home tree before its log) — `check_stamps` runs before `create_private_dir(ledger/)` and again under the lock, and a refusal reads and writes nothing. This added the ban on running the check after a create, a mode set or a log open that narrows the path (decision/strict-check-before-narrowing, decision/stamps-checked-under-lock). Item 6's symlink refusal in `check_stamps` must keep both call sites and this order.

@history_item n=18 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the diagnostics opens refuse a symlink or a widened path) — `open_role_file`/`write_detail` run `strict::check_diagnostics` on the existing folder and file before `ensure_private_dir` narrows anything. A refusal is a failed open (writer to `std::io::sink`, detail dropped), which is decision/diagnostics-open-fails-to-sink and decision/diagnostics-strict-modes-fail-open. Item 11 closes the lstat → open → path-chmod race in `open_private_append` (`crates/viola-state/src/fs.rs`) and `src/obs.rs`. It must keep the check before any narrowing and the fail-to-sink outcome.

@history_item n=19 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (every client checks the instance before the snapshot; the six CLI frame verbs share `link`/`unlink`'s instance check; `viola hook` checks the instance before obs init; the home tree checked at `run` and `revive`'s start) — `check_instance` is the first step at `cmd::client::refuse_untrusted` (`src/cmd/send.rs` and siblings), the hook's `trusted`, `viola-mcp`'s `frame`, and `check_home_tree` before the wrapper log. A refusal writes no frame and no line and leaves the mode as found. Item 6 changes `check_instance` and `check_path` under all of these callers, and item 5 touches `src/cmd/{send,run}.rs` around them. The check order and the no-trace outcomes (exit 21, hook exit 0, run/revive exit 1) must hold.

@history_item n=20 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the Windows legs measured: the protected DACL, a plain home refused, an unstattable home) — names the strict-modes witnesses, `tests/security_negatives_strict_modes.rs` and `tests/chaos_snapshot_replay.rs` (the guard of decision/strict-modes-check). Item 6's symlink cases and item 13's `digit_runs` copy in `tests/security_negatives_strict_modes.rs` sit in these files.

@history_item n=21 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the home strict-modes check is owner and mode alone; the Windows DACL arm retired) — the strict-modes check reads owner and mode on Linux only, and the read rule is `mode & 0o077`. An unstattable home is a strict-modes refusal (decision/unstattable-home-refused). `check_diagnostics` refuses a mode another user may use. This is the current form of `strict.rs` that item 6 edits.

@history_item n=22 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the socket-directory check; the board's one rule on the home) — `check_socket_dir` makes one `symlink_metadata` read and refuses a symlink as `Refused::Symlink`, adding `Refused::NotADirectory`. `Refused` gained a variant, not a derive, and the derives stay nine. `board::scan` uses `check_read` on the home and `check_path` on `instances/`. This is the in-tree precedent for item 6: `check_socket_dir` already uses lstat-style reads where `unix::reading` follows links. The board's read rule (decision/list-home-read-rule) must still hold after item 6.

@history_item n=23 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the tests holding the directory and peer checks, and no seam to drive them) — `tests/security_negatives_socket_dir.rs` holds the start, frame-verb and hook negatives. This entry added the ban on any env, flag or feature seam that forces a refusing peer-euid decision. Item 13's `digit_runs` copy and its `digit_runs_reads_whole_numbers_only` test live in this file. A shared helper in `tests/support/` must not add a seam.

@history_item n=24 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (a failed server check: no frame, and what each client reports) — no line or output of a server-verification refusal holds a pid, start time, path or endpoint (decision/no-identity-values-in-refusals). This is the property the duplicated `digit_runs` helper in `tests/security_negatives_server_identity.rs` exists to check. Item 13 must keep each test reading whole numbers the same way after the move to `tests/support/`.

@history_item n=25 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (exit 21 names its cause in detail; the two cause hints; the exit-cause table) — exit 21's `detail` is a closed set of causes, `strict-modes-failed` included. The `unwrapped-name`/`endpoint-lost` hints come last on stderr in plain ASCII, naming the parsed `ViolaName`. `tests/cross_exit_causes.rs` holds `no_two_codes_share_a_hint` with its planted control. Item 1 adds hints for `unverified-cli` and `answer`'s other `not-delivered` details: each must be distinct and hold no path or pid, and must add no new reason or exit (Boundaries).

@history_item n=26 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (trailing CR and LF removed; an empty typed text refused) — the founder ruled `empty-text`'s name, condition and hint (contract/send-refusal/empty-text). This is the precedent that a `not-delivered` detail carries a founder-ruled hint. Item 1 enumerates which `answer` `not-delivered` details still print none.

@history_item n=27 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** (the sixth dated gap: answer and hook.dialog frames, and their validation; run's strict stamps read) — `viola answer` runs `validate_paste_text` before any frame (exit 13, `not-delivered`). A non-null decision needs `run`'s gate to read the stamps through the strict check (decision/version-gate-strict-stamps-read, decision/unverified-cli-gate, contract/cli-exit-codes/12). These are the refusals item 1's hints cover: `unverified-cli` and `answer`'s `not-delivered` details.

@history_item n=28 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the clients' screen of a peer's reply; the error bodies' census; the product error enums, by census) — clients take a refusal or detail only as a code word (`is_code_word`), and a word outside the closed list stays visible by design. `tests/contract_error_bodies.rs` names every error writer, and `thiserror_messages_interpolate_nothing` guards fixed messages. Item 1's hint must not echo a peer's word as anything but a code word. Any `Refused`/error change from items 6 and 11 must stay within the nine-derive census with fixed `Display` (decision/fixed-error-display-messages).

@history_item n=29 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the error-enum exceptions and the thiserror census) — the derive census counts nine product derives, and `Refused` (among five) is a ratified named exception to the one-enum convention. A variant added to `Refused` by item 6 or 11 keeps the count and the exception.

@history_item n=30 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the bounded-reader census) — `tests/contract_bounded_readers.rs` `product_readers_are_each_bounded_or_named` requires every product reader call to be bounded in its statement or named (decision/bounded-readers-census). Item 11's rework of `open_private_append`/diagnostics opens and item 2's record-time check must keep the census green.

@history_item n=31 marker=2026-09-27-instance-state-and-start-order
**2026-09-27-instance-state-and-start-order** (the tempfile `persist` helper) — `replace_private` sets the mode on the temp file before any byte (decision/explicit-mode-before-write, decision/owner-only-file-modes). Item 11 replaces `open_private_append`'s path-based `restrict(path, …)` with a narrowing of the open file. It must keep the mode set before any byte reaches the file and never rely on the umask.

@history_item n=32 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (handle inheritance without the Windows mechanism) — `CreateProcessW` is dropped, and close-on-exec was recorded as not established, its measurement (the wrapped child's `/proc/<child pid>/fd` holding no channel socket) owed. This is the start of decision/no-handle-inheritance's Linux proof that items 8 and 9 rework in `tests/tui_channel_fds.rs`.

@history_item n=33 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (handle inheritance measured on portable-pty) — handle inheritance rests on portable-pty closing every inherited descriptor, witnessed by E2 and the live fd table: fds 0–2 on the pty, no viola or wrapper socket, no file under the home. Items 8 and 9 make the test always read `/proc/<pid>/fd` and drop the `fds.len() <= 5` count. The `/proc` assertions this entry names (no channel socket, no home file) must stay.

@history_item n=34 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** (`FAKE_AGENT_PUMP_DELAY_MS` carve-out) — the fake-agent-only pump-delay seam (u64 ms capped at 5 000, compiled out of release builds, configures nothing). It lets a test land a host resize in the spawn-to-pump window (decision/fake-agent-pump-delay-seam). Item 7's resize test (`tests/tui_pty_seam.rs`) sits on this seam class. Removing `pty.write(b"a")` must not grow a new seam.

@history_item n=35 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** (test seam `FAKE_AGENT_HOOK_PANIC`) — the second and last fake-agent test seam. Another seam or environment read needs its own Decisions Log entry (decision/test-seams-fake-agent-only, decision/no-control-switch-off, decision/test-seams-compiled-out). No item in this chunk (guards, controls, fd/PTY tests, shared helper) may add a seam.

@history_item n=36 marker=2026-09-27-epoch-2-cleanup
**2026-09-27-epoch-2-cleanup** (test-only feature proven absent) — `release-check` refuses `test-support`/`fake-agent` artifacts (probe 5/5), which is decision/release-build-no-test-binaries and decision/release-build-without-test-features. The chunk's new `tests/support/` helper and guards must stay test-only and out of the release graph.

@history_item n=37 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** (test-side removals reach the dev host's tmpfs backing through the home link) — a test-side removal under `target/e2e-home` takes only a dir the test side created (decision/e2e-home-removal-own-dirs-only, decision/test-removal-own-dirs-only). The keepers never follow a linked base unless one `lstat` reads a real owner-only dir. Item 6's symlink tests and item 12's guard must create and remove only their own test homes.

@history_item n=38 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** (the settings override as landed, the hook rows, the arm's check before its log) — trap: obs init sets an existing home to 0700, so a check made after it passes a home at 0770 (measured). This is the root of decision/strict-refusal-leaves-no-trace and decision/strict-check-before-narrowing. It bears on item 6's and item 11's checks and their remove-the-guard controls.

@history_item n=39 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (the two instance checks ahead of any log) — a log opened before or after a refused check narrowed the home to 0700 and erased the state the check refuses. `revive`'s check now runs before the wrapper log, and the hook statusline arm leaves no trace. This is the measured origin of decision/start-checks-home-tree-before-log-opens. Item 6's changes to the checks must not move them after any opener.

@history_item n=40 marker=2026-11-13-epoch-8-cleanup
**2026-11-13-epoch-8-cleanup** (Secret Management: 2 rows) — the artifact canary scan's cookie class was retired and `contract/artifact-canary-scan-classes/unreadable` added. Its classes still include the non-0600 diagnostics file (contract/artifact-canary-scan-classes/diagnostics-mode). Item 11's diagnostics-open change must keep diagnostics files at 0600 under the scan. This is also the cleanup that left the duplicated `digit_runs` item 13 removes.

@history_item n=41 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** (the masters as rows) — the security plan became rows (decision/contract/absence), each verified against the code. The rows landing 2026-11-13T01-15-10 owed the route the eleven CARRY lines this chunk closes (items 1–12). This is why the picked rows are the plan's binding form here.
