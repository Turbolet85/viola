@history_item n=1 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Data Protection: 3 row(s)) — gave `decision/no-writes-under-claude-home` and `decision/viola-writes-nothing-in-claude-dir` their one exception: verify's own trust record for its fresh temp folder. The first row says outright «Not a rule for run or revive», and the entry says «the product's write for run and revive stays unruled». It also added `decision/trust-record-writer-guarded` (never creates an absent config; refuses a link, a non-regular file, another owner, an oversize or malformed document and a present key; takes the lock only when free; tries three times; keeps the file's mode; touches no other key). Both exception clauses are the "Expected amendments" this chunk's wrap must re-word for run/revive under R2. The guarded writer is the one the product write reuses, and every refusal it holds is a case this chunk must prove writes nothing.

@history_item n=2 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Threat Model Summary: 1 row(s)) — added the same verify-only exception to `decision/nothing-written-under-claude-home`. It is the third row saying viola writes nothing under the Claude home that this chunk's product write at run/revive widens. The exception must be widened in the same wrap, and the record's persistence must be stated, since a product record is not removed after the way verify's is.

@history_item n=3 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Authentication & Authorization: 1 row(s)) — `decision/explicit-mode-before-write` gained the except clause that the trust record writer replaces Claude's config keeping that file's own mode. The product write goes through the same writer, so it rests on this exception. A second writer, or a writer that sets a mode, would break the row.

@history_item n=4 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Secret Management: 4 row(s)) — `decision/r8-claude-env-strip`, `decision/credential-strip-by-name-only`, `decision/env-not-config-channel` and `decision/no-stored-secrets` each gained one exception: viola verify reads CLAUDE_CONFIG_DIR's value, only when claude_env_keep keeps that name, to choose the config holding its trust record, else the user HOME's `.claude.json`. R2 has run/revive read that same one value for the same purpose. All four exceptions name verify alone and must be widened to run/revive. That is a new environment read, so it needs its log entry (`decision/new-env-read-needs-log-entry`).

@history_item n=5 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Input Validation: 1 row(s)) — added `decision/trust-record-closed-words`: a write or removal ends in one of 14 closed code words, stamped as the row's value when nothing was written, never a path, key or value. The chunk's «one closed word per start in the run log, nothing of the file anywhere» extends this closed set to a new sink, the run log. Any new outcome (a lost race after 3 attempts, already trusted by a parent, the home folder, an unverified CLI) must be a closed word, never a path.

@history_item n=6 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Data Protection: 2 row(s)) — `decision/verify-probe-dirs-cleanup` and `decision/probe-artifacts-cleanup` now say a SIGKILL leaves verify's trust record to be removed by hand. This is precedent for how a left record is named. The product record is meant to persist, unlike verify's, so the plan must state that difference rather than borrow verify's cleanup rows.

@history_item n=7 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** (Threat Model Summary: 2 row(s)) — `decision/verify-signature-only` and `decision/verify-screen-read-against-literals` gained the rig-only first-screen exception (whole scrubbed rows per run short of the input box, kept local and git-ignored). S1/S2 read their first screen (`trust` or not) through this machinery, and no new row-text egress may be added beyond it.

@history_item n=8 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** (Secret Management: 1 row(s)) — added `decision/live-rig-seed-from-bundle-logic`: the rig seeds its child's private config only with bundle-logic keys (used-config markers, the browser-tools prompt off). The scope's S1/S2 seed `claudeInChromeDefaultEnabled` and a sentinel OTHER project's entry into the rig's private `.claude.json`. The sentinel is a new seeded key, so check it against this rule and the founder's grant.

@history_item n=9 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** (Input Validation: 1 row(s)) — added `decision/verify-stamp-first-screen-closed-words`: the stamp gained `first_screen`, `first_settle_ms` and `first_facts`, held to closed words. The verified-CLI precondition and S1's «first screen reads `trust`» predicate read this stamped surface.

@history_item n=10 marker=2026-11-17-first-readings-on-the-standing-rig
**2026-11-17-first-readings-on-the-standing-rig** (Secret Management: 1 row(s)) — `decision/live-rig-isolated-child` changed from a sha256/mtime witness (disproved by a host writer of the shared user config) to the mark witness: the user's `~/.claude.json` is read only after a moved stamp, searched for rig-made needles, the bytes dropped, one closed word kept. The scope binds S1: «the founder's real `~/.claude.json` read only by the mark witness». This is also a trap: host writers touch that file concurrently, the same race R3 accepts as residual.

@history_item n=11 marker=2026-11-17-first-readings-on-the-standing-rig
**2026-11-17-first-readings-on-the-standing-rig** (Authentication & Authorization: 1 row(s)) — `decision/live-round-rig-keys-own-pty` now reads: the founder's word anew per chunk binds only the rig's closed argv grant (at most two down-arrows and Enter only on a read trust row, one x and one DEL in a ready box); the stop (Ctrl-C into its own pty, or a no-key group kill) needs no word. The scope's P3 block asks for S1/S2's keys «granted anew; the lean none». This row bounds what may be asked, and viola itself types into no dialog.

@history_item n=12 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (Authentication & Authorization: 1 row(s)) — made the key grant one standing committed rig with a closed argv grant read per chunk. It is the earlier form of the grant S1/S2 run under.

@history_item n=13 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (Secret Management: 2 row(s)) — added `decision/live-rig-credentials-copy` (OAuth copy owner-only in scratch, under 20 minutes of token refused, removed on every exit, never recorded) and `decision/live-rig-isolated-child` (private CLAUDE_CONFIG_DIR, closed kept env list). S1 runs `viola run` under the rig's private config, and these rows bound it. viola's write must land in `<scratch>/config/.claude.json`, which means the rig's child must reach viola with CLAUDE_CONFIG_DIR kept by claude_env_keep.

@history_item n=14 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** (Authentication & Authorization: 1 row(s)) — added `decision/live-round-rig-keys-own-pty`. It is the origin of this chunk's per-chunk key word. At that round's entry 27 the founder's ruling measured that with `projects[cwd].hasTrustDialogAccepted` set, no dialog rose and no key was taken. That reading is the premise of this chunk.

@history_item n=15 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** (Data Protection: 1 row(s)) — added `decision/live-session-screens-kept-local`: live screen frames stay under a gitignored local path, and only a closed-word summary per frame lands. This binds S1/S2's kept frames and their evidence.

@history_item n=16 marker=2026-10-18-plugin-precedence-live-readings
**2026-10-18-plugin-precedence-live-readings** (the live round's rig outside the work tree, its trust keys and residual) — precedent: the rig passed the folder-trust dialog by keys into its own pty only. The residual was named as the CLI's own `~/.claude.json` trust entry per accepted dialog, and the entry recorded «viola writes nothing under `~/.claude` or into `~/.claude.json`». The proj/ folder was put outside the work tree to avoid the external-imports dialog. S1's fresh untrusted folder needs the same care: a folder inside the work tree meets the imports modal, and a git-root key moves the record.

@history_item n=17 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** (plugin install's tree, its check and the marketplace .mcp.json) — fixed «viola spawns no `claude` and writes nothing under `~/.claude`» as the founder's own answer, the root of `decision/no-writes-under-claude-home`. The product trust write is the first product-path exception to it, so it needs his ratified bound (R2) cited as the widening.

@history_item n=18 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** (the live round's residual under ~/.claude) — precedent for a ratified boundary widening in which a round writes the CLI's own configuration. Such writes were named as the CLI's own residual, ratified by the founder's word, and never viola's.

@history_item n=19 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** (verify's typed-input PTY probe: the widening, its dirs, residual and refusal) — set the rules this chunk inherits: no child sends a byte into a CLI-native dialog, a modal start is killed with no key (`decision/verify-never-answers-dialogs`, `decision/verify-writes-no-dialog-byte`), and «viola writes nothing under `~/.claude`». It also recorded a dev-host prerequisite: the overseer set repo-root trust flags in one atomic `~/.claude.json` edit with a backup. That is the earlier, manual form of the trust record keyed on the git root. A lost record must leave the dialog to the human with no key typed, consistent with these rules.

@history_item n=20 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (Authentication & Authorization: 1 row(s)) — gave `decision/unverified-cli-gate` its StopFailure exception. The scope gates the product write «on a verified CLI», which is this gate. The write must reuse the gate's verified reading and not add its own.

@history_item n=21 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** (Authentication & Authorization: 1 row(s)) — the founder ratified `decision/unverified-cli-gate`'s StopFailure exception, so it is no longer provisional. This is the row as it now stands, and the write's precondition rests on it.

@history_item n=22 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** (run's strict stamps read; decisions on the six-row stamp until :82) — `run`'s gate reads `ledger/stamps.json` through `read_stamps_strict`. A strict-modes refusal, an unreadable file or a malformed file is `parse-rejected{ledger-stamps}` and `cli_verified:false` (`decision/version-gate-strict-stamps-read`, `contract/strict-modes-refusal/version-gate`). The scope's «to close at P3 against `read_stamps_strict`, `cli_verified`» is this reader. A non-verified result must write no trust record.

@history_item n=23 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (the claude agents --json child of list, revive and verify Run B) — precedent that adding a ledger row makes a stamp of exactly the earlier ids read unverified. This is the trap in the scope's «on a verified CLI after the 23rd row»: a home stamped before `trust-record` is transport-only and must write no record until its next live verify. Plan for it, and take it as no fault, for S1's rig home (which needs a 23-row stamp).

@history_item n=24 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** (Run B's sixth paste, its residual, and the twenty-four-row stamp) — the same rule restated: a stamp short of the newest row's id reads `cli_verified:false`. The relied-set count moved again with the rows migration and `trust-record`, so the plan must read the current count from `LedgerRow::relied()` and not from this entry's number.

@history_item n=25 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home tree checked at `run` and `revive`'s start, before the wrapper log) — `check_home_tree` runs through `checked_exe` before the wrapper log, and a refusal is exit 1 with no log, line or child (`decision/start-checks-home-tree-before-log-opens`, `decision/home-tree-strict-modes-before-log`). The trust write sits at the same shared start, after the version gate and before the spawn. It must not move ahead of these checks, and its closed word lands in the log that opens after them.

@history_item n=26 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (`ledger/stamps.json` checked before `ledger/` is made, and `verify`'s home tree before its log) — added the ban on running a strict-modes check after a create, a mode set or a log open that narrows the checked path (`decision/strict-check-before-narrowing`). The product write adds a lock sibling and a temp sibling beside Claude's config. No viola-home check may be reordered behind them.

@history_item n=27 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (the two instance checks ahead of any log) — trap: a log opened before or after a refused check narrows the home to 0700 and erases the state the check refuses (`decision/strict-refusal-leaves-no-trace`). Any new start step, the trust write included, must keep the check-before-log order.

@history_item n=28 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (the revived child: the recorded cwd ratified) — ratified the snapshot's recorded cwd as revive's spawn directory, «used as the spawn directory alone», never read by program resolution or the version gate (`decision/revive-cwd-spawn-only`, `decision/revive-recorded-cwd-spawn-dir`). Using it, or its git root, as the trust key widens that ratified «spawn only» use. The plan must name this as part of R2's bound or derive the key without a second use. The cwd also carries the OS username (`decision/cwd-only-in-snapshot`, `decision/spawn-cwd-snapshot-only`), so no key may reach a log line.

@history_item n=29 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** (the terminal colour facts join the admitted env set) — precedent for admitting an environment read. It got its own row, it is compared and never echoed, and the Universal rule says NEVER another environment read without its own Decisions Log entry (`decision/new-env-read-needs-log-entry`, `decision/controls-not-switchable`). Run/revive reading CLAUDE_CONFIG_DIR's value is such a read.

@history_item n=30 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** (`FAKE_AGENT_PUMP_DELAY_MS` carve-out; `env -i`) — the origin of the rule that environment variables are no configuration channel and that another seam or environment read needs its own entry. The pre-push `env -i` crossing (HOME and PATH only; a host-valued assignment is a widening) also binds this chunk's tests that spawn the real or fake child with a kept CLAUDE_CONFIG_DIR (`decision/hermetic-child-environment`, `decision/pre-push-empty-environment-launch`).

@history_item n=31 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** (R8 persistent-environment exemption, claude_env_keep) — set `claude_env_keep` (at most 32 names matching `^CLAUDE[A-Z0-9_]*$`, rejected whole, a floor name ineffective; `contract/config-json/claude-env-keep`). R2's file choice turns on whether viola's home keeps CLAUDE_CONFIG_DIR in this set. A rejected or absent set means the user's `~/.claude.json`.

@history_item n=32 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the R8 strip and the fixture scrub match names exactly) — R8 matches names exactly, with `claude_env_keep` the pass-list and the identity floor always stripped. «Kept» is decided by this exact-name rule, so the product's choice of config file must use the same predicate the strip uses.

@history_item n=33 marker=2026-09-27-instance-state-and-start-order
**2026-09-27-instance-state-and-start-order** (the tempfile `persist` helper) — set the one shared replace helper: a temp file in the same directory, mode set before any byte, `sync_all` + `persist` (`decision/one-replace-helper`). R3's atomic replace must go through this helper, as the trust record writer already does, with no second writer.

@history_item n=34 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the bounded-reader census) — `product_readers_are_each_bounded_or_named` fails on any product reader not bounded in its statement (`decision/bounded-readers-census`). The product write's read of Claude's config, and its read-back before the spawn, must be capped at MAX_FRAME (oversize is a refusal) and pass the census.

@history_item n=35 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (serde_json's `unbounded_depth` banned in `deny.toml`) — every parse of external bytes keeps serde_json's default recursion limit, and the ban is enforced by deny probes (`decision/serde-json-bounded-depth`, `decision/serde-json-unbounded-depth-banned`, `decision/no-serde-unbounded-depth`). Claude's `.claude.json` is external bytes parsed at every wrapped start.

@history_item n=36 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the product error enums, by census) — the `thiserror` derive census (nine) and the fixed `Display` messages with no path interpolated (`decision/error-messages-interpolate-nothing`, `decision/fixed-error-display-messages`). An error type for the product write either joins the census or stays inside the existing writer's closed words, and it may never print the config path, the key or a value.

@history_item n=37 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (revive --json documents every refusal of its start) — `revive --json` documents every refusal of the start it shares with `run`, as closed exit-1 codes (`contract/revive-json/output-fields`, `contract/revive-json/strict-modes-failed`). The trust write is designed never to refuse a start. If any outcome became a refusal, it would join this closed code list, so keep it a skip with a closed word only.

@history_item n=38 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (Input Validation: 5 row(s), 1 of 2) — removed viola's last read of the CLI's own user config (`~/.claude/settings.json` as statusline source) and set `decision/home-strict-modes-before-read` and `decision/max-frame-shared-cap` as they stand. The product write brings back a read of a CLI user file at the start, so the cap list and the reads the home check precedes must take it in.

@history_item n=39 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (Threat Model Summary: 4 row(s)) — set `contract/cli-state-root/home-flag` and `contract/cli-state-root/viola-dir` to the state tree, snapshot, endpoint and ledger stamps. If the home's `claude_env_keep` now also selects which Claude config file viola writes, these promises are incomplete for run/revive.

@history_item n=40 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (Authentication & Authorization: 2 row(s)) — set `decision/strict-modes-check` to processes reading `snapshot.json` or `ledger/stamps.json`, and retired the untrusted statusline-source read. Claude's config sits outside the viola home and outside this check. Its own guards (a link, a non-regular file, another owner) live in the trust writer, not in strict-modes.

@history_item n=41 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** (the statusline source and the shell-out, the trusted-file list) — precedent: reading the user's Claude config from the product was a founder-ratified widening. The source was named «not on the trusted-file list, outside the viola home under no viola check», and «viola writes neither source». It is the nearest earlier crossing into the CLI's user files, and this chunk is the first product write into one.

@history_item n=42 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (Data Protection: 2 row(s)) — added `decision/transcript-path-redacted`: a CLI-side path carrying the OS username, and where it may appear. Precedent for the trust key and the config path, both host paths. They must reach no log line, error or stdout (`decision/refusals-hold-no-paths-pids-ids`, `decision/no-paths-in-error-output`).

@history_item n=43 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (`MAX_FRAME` consumers extended, verify's CI split) — set that the real CLI runs only locally, and that in CI verify runs only against the fake agent (`decision/real-cli-local-only`, `decision/ci-no-secrets-no-real-claude`). This chunk's e2e proof of the write must run on the fake agent's trust reading, and S1/S2 only locally.

@history_item n=44 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** (the human's terminal on `viola run`'s stdin) — a key reaching `viola run`'s stdin is the human's, whoever typed it (`decision/human-typing-takes-wheel`). The trust dialog left to the human after a refused or lost write is answered through this stdin. Any rig key in S1/S2 counts as the human's and is bounded by the argv grant.

@history_item n=45 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Threat Model Summary: 2 row(s)) — `decision/child-output-tolerant` and `decision/child-output-tolerant-unknown` now end a failure in unknown or a closed failure word. This is the pattern for the product write's reading of Claude's config: a tolerant parse whose failure is a closed word.

@history_item n=46 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the home strict-modes check is owner and mode alone) — the strict-modes check is owner and mode alone, and an unstattable home is a refusal (`decision/unstattable-home-refused`, `decision/strict-modes-before-read`). This is the check the start runs before the trust write, and the owner-and-mode check the trust writer's «another owner» refusal parallels.
