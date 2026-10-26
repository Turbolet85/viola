# Task — arch · 2026-10-26-explicit-session-links

## The chunk
Serves the explicit session links held since chunk 2026-10-10-session-links: the wrapper's `link` and `unlink` channel methods (today answered `-32601`) and the two CLI verbs `viola link <driver> <driven>` / `viola unlink <driver> <driven>` with `--json`, the verb's instance being the driven one, in the form the founder names at P4 (a dated strict-modes gap, the instance check ahead of the snapshot read as `viola revive` takes it, or no verbs).
It gives `LinkSet::remove` its first product caller (the `unlink` method), adds the three human result lines (`link … since …Z`, `link … already linked`, `unlink …`) and their root cases in a new `tests/channel_links.rs`, and adds the `handoff:  link, unlink` row to `viola --help`.
Folds two more CARRYs: a logged link name shown on a result line is re-parsed through `ViolaName::try_new`; `v1-13`'s `link` half (every human `link` line derivable from its machine result, the `--json` result carrying the same facts).
Re-reads at P3: the client path at HEAD (the server identity check every frame verb now makes), whether the instance check can be taken for these two verbs alone, whether `v1-13`'s MCP half is provable, and that no file of `crates/viola-agent-claude`, `fixtures/claude` or `src/cmd/verify` is touched.

## Reached files
- src/run/links.rs · edits `LinkSet::remove` (its product caller; its `#[cfg_attr(not(test), allow(dead_code))]` ends)
- src/cmd/run.rs · edits the wrapper's channel dispatch of `link` and `unlink` (served instead of `-32601`) and the unit case that answers them `-32601`
- src/cmd/mod.rs · edits the `HELP_TEMPLATE` verb table (the `handoff:  link, unlink` row between `wheel:` and `setup:`) and the subcommand dispatch
- src/cmd/client.rs · re-reads the client path at HEAD (`connect`'s server identity check, the liveness pre-check)
- crates/viola-state/src/strict.rs · re-reads `check_instance`, the instance check `viola revive` takes ahead of its snapshot read
- tests/channel_links.rs · edits test-plan §6 Scenario E3 steps 1 and 3 and the raw-channel name negative (a new file)
- tests/cli_output_plain.rs · edits the `HELP` literal of `help_is_the_grouped_verb_table` and `help_lists_only_served_verbs`
- viola-0.1.0/verification-matrix.json · edits `v1-13`'s `link` half

## Needs
- The [Links] decision: a link `driver → driven` recorded by the implicit path or by an explicit `viola link`, removed by `viola unlink`, the log the single source of the link set; its "explicit verbs held" clause and the five Part A shapes landed on the operator's word, not ruled by the founder.
- The channel methods `link` / `unlink` `{driver, driven}` sent to the driven instance's wrapper: `link` records the pair and answers `{since, already_linked}`, `unlink` removes it and answers `{}`, an `unlink` of an absent pair appends nothing; a `driven` that is not the serving wrapper's name and a `driver` equal to `driven` are `-32602`; today both are `-32601`.
- The channel request list and the error schema's protocol fault codes (`-32601` unknown method, `-32602` bad params, `-32603` a link that cannot be recorded) as they bind the two methods' answers.
- The event data of `link` / `unlink` (`{driver, driven}`, `source: wrapper`), log-only, never waking `wait`, and the paragraph saying no product path appends `unlink` yet, which this chunk ends.
- The instance snapshot's `links` (`{driver, driven, since}`, `since` the `link` line's `ts`): rewritten on each change, restored at every start (a revived one included), the set following the log so a failed append leaves it as it was.
- The snapshot envelope's replay rebuild of the log-derived `links` (the replay's link half over `link` and `unlink` lines).
- The CLI conventions for the two verbs: `viola link <driver> <driven>` / `viola unlink <driver> <driven>`, arguments mirroring the channel `params`, `--json`, the CLI adding `from` from its own `VIOLA_NAME` when set.
- The CLI `--json` documents the verbs print (`{"v":1,"ok":{…}}`, `instance-unreachable` exit 21, `wrapper-fault` exit 20) and the CLI exit codes they take (0, 1, 2, 20, 21).
- The server identity check every frame-writing client makes before its first frame: the snapshot's `pid` and `started_at` against the connected peer on Linux and Windows through `client::connect`, macOS the recorded gap; a mismatch writes no frame and exits 21 `instance-unreachable` with one `server-verify-failed` line.
- The liveness pre-check the verbs make before connecting (the snapshot's pid + start time alive, the heartbeat live, an `endpoint` present; else exit 21), per session liveness.
- The strict-modes half of the widening: reading `snapshot.json`'s `endpoint`, `pid` and `started_at` without strict-modes as a dated gap until "Home and code-bearing file integrity", or `viola_state::strict::check_instance` ahead of the snapshot read as `viola revive` takes it; the other eight clients' strict-modes work stays that entry's.
- The local endpoint trust boundary and the IPC endpoint the verbs connect to (the `endpoint` recorded in the snapshot; Unix socket in the per-user 0700 dir, Windows pipe through the SQOS Identification open, never interprocess's default connect).
- Name validation: `driver` and `driven` only through `ViolaName::try_new` before any path join or frame, and a logged link name shown on a result line re-parsed through `ViolaName::try_new` (the second CARRY).
- The `since` value on the `link … since …Z` line and in the `ok` payload: RFC 3339 UTC with milliseconds and `Z`.
- The subcommand list (`link`, `unlink` among the served subcommands) and the `viola --help` grouped verb table that gains its `handoff:` row only once clap serves the verbs.
- The [MCP] decision that `link` and `unlink` are CLI-only human controls over the link topology: no MCP tool is built, and `v1-13`'s MCP half is P3's to read.
- Home resolution for the two verbs (`--home`, then the `VIOLA_DIR` instance home, then `<user home>/.viola/`) and `VIOLA_NAME` as the source of `from`.
- Tokio containment: the two verbs and the wrapper's two methods run on the sync client and the std-thread channel server, never a runtime.
- The capability-ledger boundary: a link is recorded from viola's own parameters, so no ledger row, `viola verify` child, re-stamp or live `claude` start; P3 re-reads it.
- The ndjson append discipline for the `link` / `unlink` lines the wrapper appends (one `write` per line under the `.lock` sibling, never truncating `events.ndjson`).

## Standing conventions
- S1 · every new test runs in its own viola home (`--home`) against the fake agent named after `--`, never the real `claude` · .andromeda/architecture-detail/established-decisions.md
- S2 · every new test home lives under `target/e2e-home/viola-test-*/home`, removed only when its recorded owner process is gone · .andromeda/architecture-detail/occupied-resources.md
- S3 · every new channel `params`, `--json` document and event line carries `v` (and `params` the `sender`), readers tolerant with no `deny_unknown_fields` · §Conventions (Interfaces and versioning)
- S4 · every new log line is codes-only, one `write`, in its role's home-level `diagnostics/` file; content-bearing detail only in the instance's `diagnostics/detail-*.ndjson`; stdout reserved for the `--json` result · §Cross-cutting Patterns (Diagnostic output channels)
- S5 · every new error type or variant is a `thiserror` enum with fixed messages, anyhow only in the root bin, no path, pid, endpoint or upstream text in an external error · §Conventions (Rust error types)
- S6 · every OS-specific branch (the server check's macOS gap included) compiles and is tested on its CI runner · §Cross-cutting Patterns (Cross-platform discipline)
- S7 · JSON field names snake_case and enum wire values kebab-case in every new payload · §Conventions (Naming patterns)

## Not needed
- The budget governor, `budget.json`, the statusline wrapper and `release --budget`: `v1-13`'s budget half landed with chunk 2026-10-10-budget-governor.
- The hook transport and hook contract, `hook.event` / `hook.dialog` and the dialog hold: no hook path changes.
- The PTY seam, the vt100 readiness gate, delivery confirmation and paste-text validation: a link types nothing into the child.
- The wheel and its holders, `pause` / `release`: a link moves no wheel.
- The `answer` response shapes per dialog kind and the implicit link on `send` / `answer`, landed by chunk 2026-10-10-session-links and left as they stand.
- The `list` payload and the `claude agents --json` join: the board shows no link column.
- `viola revive`'s preflight, resume argument shapes, recorded `cwd` and session chain: only its instance check is a candidate form here.
- The plugin, `viola plugin install`'s marketplace, the pinned exe and its re-hash: the verbs add no plugin file.
- The Windows ConPTY companions, the DLL search restriction and `vendor/conpty/`.
- The colour facts `NO_COLOR` / `TERM` / `COLORTERM`, read only by `viola list`'s styled board.
- `config.json`'s thresholds, `dialog_hold_s` and the `statusline-source.json` source.
