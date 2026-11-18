@history_item n=1 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Established Decisions: 4 rows) — added `decision/verify-own-trust-record` (verify writes a trust record only into the config file its child reads, never creating an absent file, removes it after; "run and revive write no trust record") and `decision/trust-record-ledger-row` (the 23rd relied row, Run F after Run E); widened `decision/verify-child-set` to six runs and `decision/claude-first-no-adapter-trait`'s test-only exception (fake agent and rig read a config's trust record). This chunk is the product write those rows held back: the "run and revive write no trust record" clause must be amended, the writer (`trust.rs` / `trust_record.rs`) reused, and the bound kept as ratified (delegate confirm PROVISIONAL at that wrap).

@history_item n=2 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Occupied Resources: 4 rows + 1 row) — `decision/env-strip-names-only` / `decision/env-strip-reads-names-only` gained the one exception "viola verify reads the value of CLAUDE_CONFIG_DIR, only when claude_env_keep keeps that name"; `contract/config-json/claude-env-keep` says keeping it makes verify read the value; `decision/verify-removes-only-own-dirs` covers verify's own trust record. The product write reads that same value at run/revive (R2), so all three exception cells name verify alone and must widen, a new env read each time.

@history_item n=3 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Cross-cutting Patterns, Conventions, Infrastructure Patterns, Inherited Defaults) — `decision/no-environment-configuration` except gained "CLAUDE_CONFIG_DIR's value, read by verify only when kept"; `decision/env-var-viola-prefix` except gained "verify's kept CLAUDE_CONFIG_DIR"; `decision/claude-shapes-in-one-crate` and `decision/claude-shapes-isolated` gained the test-only trust-record readers. Each names verify (or test code) as the only reader, so a run/revive reader of the value and of the trust-record shape lands against them; the record shape must stay inside viola-agent-claude, the file write in viola-state/root bin (`decision/agent-claude-pure`).

@history_item n=4 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Standard Contracts: 9 + 7 rows) — `contract/ledger-stamps/rows` and `contract/ledger-stamps/content` let trust-record hold the writer's code word, and added the 14 closed `contract/trust-writer-words/*` (written, removed, config-absent, config-unreadable, config-link, config-not-regular, config-foreign, config-oversize, config-malformed, config-dir-unusable, home-unknown, key-present, lock-held, lost). This is the closed word set the "one closed word per start" in the run log will draw from; key-present and the refusal words map onto this chunk's skips and refusals, and any new word (an unverified-CLI or parent-trusted skip) extends a closed set.

@history_item n=5 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Established Decisions: 1 row, the rule reworded) — `decision/trust-record-ledger-row` was reworded to "asks, per CLI version, whether a trust record … opens it with no modal", because the one live reading on 2.1.325 then read fail. Trap: the CLI honouring the record is a per-version measured fact, so the product write may rely on it only behind a verified stamp holding the row `pass`.

@history_item n=6 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** — `decision/claude-first-no-adapter-trait`, `decision/claude-shapes-in-one-crate` and `decision/claude-shapes-isolated` exceptions now also say the rig seeds its private Claude config; `contract/ledger-stamps/first-screen` (one closed word per interactive run: trust, imports, box, none, exited) and `first-settle-ms` added; `contract/viola-cli-arguments/verify` gained the hidden rig-only `--record-first-screens`; `decision/run-never-reads-screen-content` names the rig-only first-screen record. The row read `pass` live on 2.1.325 (S0B); S1/S2 will read the first screen word and seed the rig's private config the same way, and run still reads no screen content.

@history_item n=7 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — the NO_COLOR / TERM / COLORTERM reads were ratified as a boundary widening with the standing precedent "any further environment read … needs its own ratification". Reading CLAUDE_CONFIG_DIR's value at run/revive is such a read (R2 ratifies it), so every environment exception row must be widened with that ratification named.

@history_item n=8 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the R8 strip: exact names, `claude_env_keep` the one pass-list) — the strip matches `CLAUDE*` names exactly, `claude_env_keep` the one pass-list, the identity floor's names always removed, and "the strip and the logs never read a value". The child reads `<CLAUDE_CONFIG_DIR>/.claude.json` only when the keep list passes that name, so picking "the file the started child will read" follows `decision/child-env-claude-strip` and `contract/config-json/claude-env-keep`.

@history_item n=9 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — compiled `SIGNATURES` with the modals "Yes, I trust this folder" and "Yes, allow external imports"; `run` passes signatures only when `cli_verified` (full gate verified, partial otherwise); measured on 2.1.288 that trust is inherited from a trusted parent while the imports approval is keyed on the git root. This is the precedent for reading trust the CLI's way with the parent walk, and a reminder that `decision/no-typing-into-trust-dialogs` covers the imports dialog too, which a trust record does not clear.

@history_item n=10 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** — the live round took no readings: inside the work tree the local scope landed at its root, and with `proj/` its own git repo both wrapped starts were held by the folder-trust dialog. Trap and motivation for this chunk: a fresh untrusted folder blocks a wrapped start at the trust dialog, and the key is the git root inside a repository (S1/S2 folder shape).

@history_item n=11 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — `decision/wheel-starts-with-driver` now says a key the human types into the folder-trust dialog before the session starts takes the wheel. A start whose write was refused or lost leaves the dialog to the human with no key from viola, and that human key moves the wheel; a start the record trusts shows no dialog and keeps `driver`.

@history_item n=12 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (the version gate placed and split by crate) — placed the version gate in `run`'s start order (after the pinned copy and plugin, before the bind), with `cli_version` / `cli_verified` going into the first snapshot, and split it: Claude parsing and verdict in the pure agent crate, the stamps I/O in `viola_state::stamps`. The trust write goes "after the version gate and before the spawn" on `cli_verified`, a new step in `decision/run-start-order` that must keep the agent crate free of file I/O.

@history_item n=13 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Established Decisions: run-start-order; Cross-cutting: mixed-version-tolerance) — `decision/run-start-order` lost the budget-gate step and now reads resolve, collision, pin, version gate, bind, first snapshot, heartbeat, start events, spawn; `decision/mixed-version-tolerance` gained "a stamp holding a retired ledger row verifies on the relied rows alone". This chunk inserts its write into the start-order row (the last such change was this removal). The verdict precedent: a 22-row stamp reads unverified to a 23-row build.

@history_item n=14 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — the precedent that adding a relied row makes "every home stamped before this chunk read `cli_verified:false` until its next `viola verify`" (repeated at each later row add). It backs the scope's inferred point that a home stamped at 22 rows is transport-only and writes no trust record until re-verified (`decision/version-verified-all-relied-rows`, `decision/unverified-cli-transport-only`).

@history_item n=15 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** (decisions on the six-row stamp, the strict stamps read) — `run`'s version gate reads `ledger/stamps.json` through `read_stamps_strict` behind the strict-modes check; a refusal is `cli_verified:false` with one `parse-rejected{ledger-stamps, strict-modes-failed}`. The write's "on a verified CLI" condition reads that one verdict (`decision/version-gate-strict-stamps-read`, `decision/malformed-stamps-unverified`), never the stamps again.

@history_item n=16 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (the snapshot's `cwd` and the child's second cwd source) — added `contract/instance-snapshot/cwd` and made a revived start spawn in the snapshot's recorded cwd, read after the strict-modes check, else `cwd-missing`; program resolution and the version gate never read it (a ratified widening). For revive, "the child's folder" whose key is written is that recorded cwd, while `decision/revive-program-resolution` keeps the gate on the reviving process's own directory.

@history_item n=17 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (one read per revive, one state-recovered line; the home-tree check before the start's log) — revive's preflight reads the snapshot once and hands that read on (`decision/revive-reads-snapshot-once`), and a replayed snapshot gives no cwd (`decision/revive-refused-without-cwd`); every start runs `check_home_tree` before its wrapper log opens. The write must reuse the start's one snapshot read and come after these checks (`decision/strict-check-before-log-open`).

@history_item n=18 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`viola revive --json` documents every refusal of its start) — every refusal of the shared start under `revive --json` prints a closed exit-1 document, an anyhow error of the start prints `internal-error` and its chain is still logged (`contract/revive-json/exit-1-document`, `contract/viola-revive-json-error-codes/*`). Constraint: the trust write's refusals are no start refusal and must add no exit, document or code; a refused write leaves the start going.

@history_item n=19 marker=2026-09-27-instance-state-and-start-order
**2026-09-27-instance-state-and-start-order** — chose tempfile 3.27.0 `persist` through one shared helper (`replace_private`, mode set on the temp file before the write) over atomic-write-file (BSD-3-Clause, outside `deny.toml`), and `.lock` siblings opened for write. This is the origin of `decision/tempfile-snapshot-writer`, `decision/atomic-replaced-state-files` and `decision/private-mode-before-write`, so the config replace goes through the shared helper with no call-site temp + rename.

@history_item n=20 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** ([Snapshot writer]: `replace_private` is one attempt; the separate lock files) — `replace_private` is one attempt, a failed persist returning at once with the target as found, and the Windows retry is gone; the separate `.lock` layout is kept. The R3 "at most 3 attempts with read-back" is therefore the trust writer's own loop over a one-attempt replace, and taking the CLI's lock is a separate lock file, not the appended file.

@history_item n=21 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** ([Snapshot writer]: no-clobber placement) — added `place_private_new` (`persist_noclobber`) beside the replace helpers and stated that no call site carries its own temp + rename code. The config replace must reuse a `viola_state::fs` helper, and the writer must keep the file's existing mode as `trust_record.rs` already does, not take a fixed 0600.

@history_item n=22 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — added `decision/bounded-readers`, held by the census `product_readers_are_each_bounded_or_named` in `tests/contract_bounded_readers.rs`, and banned serde_json `unbounded_depth`. The product's read of the Claude config file (past `MAX_FRAME` gives `config-oversize`) must be bounded in its own statement or the census goes red, and a new product read site needs its `BOUNDARIES` row.

@history_item n=23 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — added `absence/status-line-interception` (no settings layer, no `--settings` at start) and narrowed `decision/tolerant-external-payloads` to hook JSON and `claude agents --json`. A settings route to trust is closed by this absence and by the measurement; the config file parse is an external payload that must be read tolerantly (unknown keys kept, other keys left as found).

@history_item n=24 marker=2026-09-24-diagnostics-plane
**2026-09-24-diagnostics-plane** — `run` logs to the home-level `diagnostics/run-<name>.ndjson`, codes only, and content-bearing detail goes only to the instance's owner-only detail files (`decision/codes-only-role-logs`, `decision/process-logs-hold-codes-only`, `contract/diagnostics-role-files/run-name`). The one closed word per start goes into the run role file as a code, with nothing of the config file, path or key on any line; `schemas/diag-line.v1.json`'s enums must admit the word.

@history_item n=25 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the one-enum divergences ratified as named exceptions) — ratified `ServeError`, `StampsError`, `PinError`, `StampsReadError` and `Refused` beside `PtyError` as named exceptions to one enum per crate (`decision/thiserror-one-enum-per-crate`, `decision/one-error-enum-per-crate`). Any new error type for the product write is another divergence that needs its own ratification; folding it into an existing enum is the precedent (as `StampError` and `Refusal` were folded).
