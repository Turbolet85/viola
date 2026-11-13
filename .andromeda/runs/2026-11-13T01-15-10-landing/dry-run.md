rows-convert · proposal 2026-11-12T03-23-49 · word 76f59d6ac300f77d5e8feffa1e02d5865bda97e766fc55fec6e0b9318a5e3b6d · of 0922fbab47d9 · head 2d3788258131 · pin 744

re-check 1 · the masters' text pin (744, the proposal's 766): ok
re-check 2 · the store is the list: ok
re-check 3 · the files a row's verdict stands on (0 row(s) moved since 8834195b0683): ok
re-check 4 · the word: ok
re-check 5 · the proposal's recorded tool (rec v1.32): ok
re-check 6 · the install reads rows: ok
re-check 7 · no edge from outside the family: ok
  chunks: 0 pending · 0 gated
  masters: architecture text · security-plan text · design-system text · layout-templates text · test-plan text · obs-plan text · a11y-plan text
  rec.py check: clean
  tree: nothing uncommitted
  proposal 2026-11-12T03-23-49 · finished · every file by its sha
re-check 8 · the strict ground: ok

gone: drop 45 · keep 3 · stale 7 · 55 line(s)
guards: stands 6 · absent 2 · 8 line(s)
divergences: code 75 · holds 6 · rule 14 · none 5 · 100 line(s)
stale: none 3 · keep 2 · code 18 · 23 line(s)
rekey: no fate · 0 line(s)
relied: cut 12 · keep 45 · 57 line(s)
moved: no fate · 0 line(s)
divergences · layout-templates · decision/board-vs-strip · code
  rule · written: `viola list` shows the whole bay, and every other verb acts on one strip named by its `ViolaName`.
  rule · lands: `viola list` shows the whole bay, `verify` and `plugin install` name no strip, and every other verb a person runs acts on one strip named by its `ViolaName`. · new text
divergences · layout-templates · decision/shared-captions-fixed · rule
  rule · written: The six captions, words and slot order are fixed, and the terminal fleet console reuses them.
  rule · proposed: The six captions, words and slot order are fixed.
  rule · lands: The six captions, words and slot order are fixed, and the terminal fleet console reuses them. · = written
divergences · design-system · decision/stable-session-slots · code
  rule · written: Every session keeps a stable slot in the strip bay for its whole life, and nothing is re-sorted.
  rule · lands: Every session keeps a slot in the strip bay stable across state changes: wrapped sessions in name order, then unwrapped ones in name order; nothing is re-sorted by state, and a slot shifts only when a session arrives or leaves. · new text
divergences · design-system · decision/two-hues-one-role-each · none
  rule · written: Only two hues are allowed beyond the monochrome ladder, and each is locked to one role.
  rule · lands: no row
divergences · design-system · decision/list-is-the-board · code
  rule · written: `viola list` is the board, and every other verb acts on one strip.
  rule · lands: `viola list` is the board, `verify` and `plugin install` act on no strip, and every other verb a person runs acts on one strip. · new text
divergences · a11y-plan · decision/fixed-copy-from-design · code
  rule · written: The CLI's fixed copy set (`no wrapped sessions — start one with  viola run <name> -- claude`, `unknown`, `n/a`) comes from design.
  rule · proposed: The CLI's fixed copy set (`no wrapped sessions  start one: viola run <name> -- claude`, `unknown`, `n/a`) comes from design.
  rule · lands: The CLI's fixed copy set (`no wrapped sessions  start one: viola run <name> -- claude`, `unknown`, `n/a`) comes from design. · = proposed
divergences · a11y-plan · decision/refusal-ends-with-hint · rule
  rule · written: Every human-mode CLI refusal ends with a `hint:` recovery line (SC 3.3.3).
  rule · lands: Every human-mode CLI refusal ends with a `hint:` recovery line (SC 3.3.3). · = written
divergences · a11y-plan · decision/codes-beside-plain-word · code
  rule · written: A kebab-case reason or detail code (`not-delivered`, `human-typing`, `unverified-cli`) never appears without the plain word `unable`.
  rule · lands: A kebab-case reason or detail code (`not-delivered`, `human-typing`, `unverified-cli`) on a refusal line never appears without the plain word `unable`; the board's cells show status codes such as `unverified-cli` without it. · new text
divergences · a11y-plan · decision/no-screen-content-parsing · code
  name · written: No screen-content parsing
  name · lands: No screen-content parsing beyond signatures · new text
  rule · written: The rendered `claude` TUI screen is never parsed for content, and screenshots are never taken as a11y evidence (R7).
  rule · lands: The rendered `claude` TUI screen is parsed only for compiled input-box and modal signatures, never for other content, and screenshots are never taken as a11y evidence (R7). · new text
divergences · security-plan · decision/cli-tool-version-floors · code · as decision/cli-tool-exact-pins
  name · written: Minimum floors for external CLI tools
  name · lands: Exact pins for external CLI tools · new text
  rule · written: External CLI and audit tools are expressed as minimum version floors, not exact pins; exact pins are only for lock-pinnable library dependencies.
  rule · lands: External CLI and audit tools are installed at exact pinned versions, in CI and by the ripgrep install script, and CI actions are pinned by commit SHA; no tool is expressed as a minimum version floor. · new text
  why · written: an exact pin triggers a false drift escalation when a patch-behind build is installed
  why · lands: CI installs each tool at its pin itself, and no drift check compares a locally installed build against a pin. · new text
divergences · security-plan · decision/nutype-input-validation · code
  rule · written: Input validation uses nutype, already in the stack; no new validation library is added, and validated names are enforced at the CLI, MCP handlers, channel link and unlink params and the notify path filter.
  rule · lands: Input validation uses nutype, already in the stack; no new validation library is added, and validated names are enforced at the CLI, MCP handlers and channel link and unlink params. · new text
divergences · security-plan · decision/client-exit-13-first · code · as decision/client-refusals-first
  rule · written: A CLI frame verb's own exit-13 refusals come first, then its strict-modes refusal, before its liveness and server checks.
  rule · proposed: A CLI frame verb's strict-modes refusal comes first, then its own exit-13 refusals, before its liveness and server checks.
  rule · lands: A CLI frame verb's strict-modes refusal comes first, then its own exit-13 refusals, before its liveness and server checks. · = proposed
divergences · security-plan · decision/exec-commands-absolute-pinned-path · code
  name · written: Exec commands use the absolute pinned path
  name · lands: Exec commands use an absolute path · new text
  rule · written: Every exec-form command viola writes for the CLI is the absolute pinned bin/<version>-<hash>/viola path, never a PATH lookup.
  rule · lands: Every exec-form command viola writes for the CLI is an absolute path, never a PATH lookup: the pinned bin/<version>-<hash>/viola path for run, revive and verify, and the installing exe's canonical path in plugin install's `.mcp.json`. · new text
divergences · security-plan · decision/fixture-scrub-and-refusal · rule
  rule · written: Before any write, verify --record scrubs the home prefix to ~ and the user word to <user>, and refuses the whole recording with exit 1 and nothing written when a home or absolute path, the user word or an email-shaped token survives.
  rule · proposed: Before any write, verify --record scrubs the home prefix to ~ and the user word to <user>, and refuses the recording (exit 1, nothing written) if a home or absolute path or the user word survives in a payload, or a path, the user word or an email-shaped token survives in a kept screen row.
  rule · lands: Before any write, verify --record scrubs the home prefix to ~ and the user word to <user>, and refuses the whole recording with exit 1 and nothing written when a home or absolute path, the user word or an email-shaped token survives. · = written
divergences · security-plan · decision/exact-third-party-pins · rule
  rule · written: The workspace pins every third-party dependency version exactly.
  rule · lands: The workspace pins every third-party dependency version exactly. · = written
divergences · security-plan · decision/cli-names-through-name-check · rule
  rule · written: Every viola name argument on the CLI goes through the viola name check, one parser shared by the verbs.
  rule · proposed: Every viola name argument on the CLI goes through ViolaName::try_new; the verbs share one parse_name, and run keeps its own identical copy.
  rule · lands: Every viola name argument on the CLI goes through the viola name check, one parser shared by the verbs. · = written
divergences · security-plan · decision/budget-json-checked-read · none
  rule · written: budget.json is read only after path checks on the home and the file, under a shared lock, capped and tolerant, v 1 only; every failure is no reading and no line, and the read never writes or creates anything.
  rule · proposed: budget.json is read only after path checks on the home and the file, under a shared lock on a sibling lock file (created if absent), capped and tolerant, v 1 only; every failure is no reading and no line, and the read never writes budget.json.
  rule · lands: no row
divergences · security-plan · decision/no-rust-cache-in-release · holds
  rule · written: Swatinem/rust-cache is never restored in the v1.x release workflow.
  rule · lands: Swatinem/rust-cache is never restored in the v1.x release workflow. · = written
divergences · security-plan · decision/no-stored-secrets · code
  except · written: the test-only seams compiled out of release builds, and the terminal colour facts read by viola list's human mode
  except · proposed: the test-only seams compiled out of release builds, the terminal colour facts read by viola list's human mode, and XDG_RUNTIME_DIR read by the channel endpoint to place the socket directory
  except · lands: the test-only seams compiled out of release builds, the terminal colour facts read by viola list's human mode, and XDG_RUNTIME_DIR read by the channel endpoint to place the socket directory · = proposed
divergences · security-plan · decision/strict-modes-check · rule
  rule · written: Every process that reads snapshot.json, ledger/stamps.json or statusline_command refuses when the home, its trusted folders or a trusted file is group- or world-writable, not owned by euid, or a symbolic link, and when the home or instance folders grant any group or other permission.
  rule · proposed: Every process that reads snapshot.json, ledger/stamps.json or statusline_command refuses when the home, its trusted folders or a trusted file is group- or world-writable or not owned by euid, and when the home or instance folders grant any group or other permission.
  rule · lands: Every process that reads snapshot.json, ledger/stamps.json or statusline_command refuses when the home, its trusted folders or a trusted file is group- or world-writable, not owned by euid, or a symbolic link, and when the home or instance folders grant any group or other permission. · = written
divergences · architecture · decision/no-tmux-or-shell-dependency · rule
  rule · written: No design choice may depend on tmux or a shell.
  rule · lands: No design choice may depend on tmux or a shell. · = written
divergences · architecture · decision/tolerant-external-payloads · code
  rule · written: Hook JSON, `claude agents --json` and statusline payloads are parsed with no `deny_unknown_fields`, with defaults and options, the raw value kept for the fixture recorder and serde_path_to_error for drift reports.
  rule · lands: Hook JSON, `claude agents --json` and statusline payloads are parsed with no `deny_unknown_fields`, with defaults and options; agents rows keep the raw value for the fixture recorder, and serde_path_to_error gives drift reports for hook JSON only. · new text
divergences · architecture · decision/thiserror-per-crate-anyhow-in-bin · code
  rule · written: Each workspace crate has one thiserror error enum; anyhow is used only in the `viola` bin.
  rule · proposed: Each workspace crate except `viola-core` has one thiserror error enum; anyhow is used only in the `viola` bin.
  rule · lands: Each workspace crate except `viola-core` has one thiserror error enum; anyhow is used only in the `viola` bin. · = proposed
divergences · architecture · decision/nutype-errors-fold-into-crate-enum · code · as decision/nutype-errors-not-folded
  name · written: nutype validation errors fold into the crate enum
  name · lands: nutype validation errors stay out of crate enums · new text
  rule · written: A nutype validation error is folded into the owning crate's error enum.
  rule · proposed: A nutype validation error is not folded into a crate error enum; a failed `ViolaName::try_new` is discarded (`.ok()`) or read as invalid params.
  rule · lands: A nutype validation error is never folded into a crate error enum: the caller drops it, refusing the input (a usage error, invalid params, a closed field word) or treating it as absent (a config default with a rejection, no instance). · new text
divergences · architecture · decision/env-var-viola-prefix · code
  except · written: Ratified: test seams `FAKE_AGENT_PUMP_DELAY_MS` and `FAKE_AGENT_HOOK_PANIC`; `NO_COLOR`, `TERM` and `COLORTERM`, read never set, only to decide human output's SGR bytes.
  except · proposed: Ratified: test seams `FAKE_AGENT_PUMP_DELAY_MS`, `FAKE_AGENT_HOOK_PANIC`; `NO_COLOR`, `TERM`, `COLORTERM` (read, never set; SGR bytes); `PATH`, read to find the program after `--`.
  except · lands: Ratified: test seams `FAKE_AGENT_PUMP_DELAY_MS`, `FAKE_AGENT_HOOK_PANIC`; `NO_COLOR`, `TERM`, `COLORTERM`, read only for SGR bytes; `PATH`, read and set for the child; `XDG_RUNTIME_DIR`, read only. · new text
divergences · architecture · decision/harness-env-var-prefix · code
  name · written: Test-harness variables carry the AGENT_RUN_ prefix
  name · lands: Test-harness knobs carry the AGENT_RUN_ prefix · new text
  rule · written: Test-harness-only environment variables use the `AGENT_RUN_` prefix and are never read by the `viola` binary.
  rule · lands: Test-harness knobs use the `AGENT_RUN_` prefix and are never read by the `viola` binary; the `FAKE_AGENT_*` seams, read only by fake-agent builds, and each crate's test-private self-spawn markers (`PTY_SEAM_TEST_*`, `MCP_*_TEST_*`) keep their own names. · new text
divergences · architecture · decision/timestamp-field-names · code
  rule · written: Every event and frame carries its instant in the field `ts`; other instants use `*_at` fields.
  rule · lands: Every instance event, and the `hook.event` frame, carries its own instant in the field `ts`; request frames carry no instant, process-log and detail lines carry `timestamp`, and other instants use `*_at` fields, except the start instant a link record holds. · new text
divergences · architecture · decision/percent-newtype · none
  rule · written: A percentage is the `Percent` newtype, 0 to 100, serialised as a JSON number.
  rule · lands: no row
divergences · architecture · decision/fake-agent-feature-on-every-member · code · as decision/fake-agent-feature-on-named-members
  rule · written: The sync crates, viola-mcp and viola-e2e each declare a code-free fake-agent feature, so a package-scoped fake-agent build of any member resolves.
  rule · proposed: The sync crates viola-pty, viola-agent-claude and viola-state, and viola-mcp and viola-e2e, each declare a code-free fake-agent feature, so a package-scoped fake-agent build of any of them resolves.
  rule · lands: The sync crates viola-pty, viola-agent-claude and viola-state, and viola-mcp and viola-e2e, each declare a code-free fake-agent feature, so a package-scoped fake-agent build of any of them resolves. · = proposed
  name · written: A fake-agent feature on every member crate
  name · lands: A fake-agent feature on each named member crate · new text
divergences · architecture · decision/no-env-config-channel · none
  rule · written: Environment variables are not a configuration channel; a dependency or feature that reads one breaks this rule like a direct read does.
  rule · lands: no row
divergences · architecture · decision/no-environment-configuration · code
  except · written: NO_COLOR, TERM and COLORTERM, read by viola's own colour layer for list's human mode; two fake-agent test seams compiled only under the fake-agent feature.
  except · proposed: NO_COLOR, TERM, COLORTERM for list's colour; XDG_RUNTIME_DIR for the endpoint socket dir; PATH for program lookup; two fake-agent test seams under the fake-agent feature.
  except · lands: NO_COLOR, TERM and COLORTERM for list's human-mode colour; XDG_RUNTIME_DIR for the socket dir; PATH for program lookup; two fake-agent test seams compiled only under the fake-agent feature. · new text
divergences · architecture · decision/budget-gate-reads-hook-written-file · none
  rule · written: The wrapper may refuse a driver on budget.json, read behind the strict-modes check on the home and the file and failing open: every failure is no reading, with no log line and nothing created.
  rule · proposed: The wrapper may refuse a driver on budget.json, read behind the strict-modes check on the home and the file and failing open: every failure is no reading, with no log line; the read creates the budget.json.lock sibling.
  rule · lands: no row
divergences · architecture · decision/thiserror-one-enum-per-crate · code
  rule · written: Each workspace crate has one thiserror error enum, the refusal reason type lives in the core crate, and anyhow is used only in the root viola bin.
  rule · proposed: Each workspace crate except viola-core has one thiserror error enum, the refusal reason type lives in the core crate, and anyhow is used only in the root viola bin.
  rule · lands: Each workspace library crate except viola-core has one thiserror error enum, the refusal reason type lives in the core crate, and anyhow is used only in the root viola bin. · new text
divergences · test-plan · decision/workflows-audited-and-sha-pinned · holds
  rule · written: CI runs zizmor on the workflows and asserts every Action is SHA-pinned, gating on exit code.
  rule · lands: CI runs zizmor on the workflows and asserts every Action is SHA-pinned, gating on exit code. · = written
divergences · test-plan · decision/machine-parseable-test-output · code
  rule · written: Every test layer produces machine-parseable output, and harness commands print one JSON document on stdout.
  rule · proposed: Every test layer produces machine-parseable output, and harness commands other than logs print one JSON document on stdout.
  rule · lands: Every test layer produces machine-parseable output, and harness commands other than logs print one JSON document on stdout. · = proposed
divergences · test-plan · decision/no-test-retries-or-sleeps · code
  rule · written: nextest runs with no retries, and no thread or tokio sleep is used for synchronisation.
  rule · lands: nextest runs with no retries, and no thread or tokio sleep is used for synchronisation, save as the poll interval of a loop that rechecks its condition, which in a test wait also runs under a deadline. · new text
divergences · test-plan · decision/insta-snapshots-for-decision-bodies · code
  rule · written: Decision bodies are pinned by insta JSON snapshots with the dialog_id and ts fields redacted.
  rule · proposed: Decision bodies are pinned by insta snapshots of the body text.
  rule · lands: Decision bodies are pinned by insta snapshots of the body text. · = proposed
divergences · test-plan · decision/pty-exit-from-wait-not-eof · code
  rule · written: viola-pty reports a child's exit from wait on the handle, never from reader EOF; consumers are tested with a mockall double of the six-method seam whose reader never returns EOF while wait returns an exit status.
  rule · proposed: viola-pty reports a child's exit from try_wait on the handle, never from reader EOF; consumers are tested with a mockall double of the seven-method seam whose reader never returns EOF while try_wait returns an exit status.
  rule · lands: viola-pty reports a child's exit from try_wait on the handle, never from reader EOF; consumers are tested with a mockall double of the seven-method seam whose reader never returns EOF while try_wait returns an exit status. · = proposed
divergences · test-plan · decision/wrapper-channel-integration-driver · code
  rule · written: Wrapper channel integration tests drive the viola-channel sync client, a raw interprocess local socket stream for malformed frames, and jsonschema validators built from the product's schemars schemas.
  rule · lands: Wrapper channel integration tests drive the viola-channel sync client, a raw interprocess local socket stream for malformed frames, and jsonschema validators built from the committed schema files, which check diagnostics lines while reply frames are compared directly. · new text
divergences · test-plan · decision/on-disk-state-tested-on-real-files · code
  rule · written: Module-to-disk state is tested over real ndjson, snapshots and lock siblings in a tempfile home, with a concurrent-append check that every line of every shared diagnostics file parses as one JSON object.
  rule · lands: Module-to-disk state is tested over real ndjson, snapshots and lock siblings in a tempfile home, with a concurrent-append check on the hook role and detail files, whose open and write paths every diagnostics file shares, that every line parses as one JSON object. · new text
divergences · test-plan · decision/pty-tested-on-real-openpty · rule
  rule · written: PTY integration spawns viola-fake-agent through viola-pty on a real openpty, and a key is written after a resize only once the child reports the new size.
  rule · lands: PTY integration spawns viola-fake-agent through viola-pty on a real openpty, and a key is written after a resize only once the child reports the new size. · = written
divergences · test-plan · decision/child-holds-no-viola-paths · code · as decision/child-env-names-viola-instance
  rule · written: The agent child is given no viola socket or home path.
  rule · proposed: The agent child is given VIOLA_NAME, VIOLA_DIR (its instance directory) and VIOLA_BIN (the pinned copy path).
  rule · lands: The agent child is given VIOLA_NAME, VIOLA_DIR (its instance directory) and VIOLA_BIN (the pinned copy path). · = proposed
  name · written: Child holds no viola socket or home path
  name · lands: Child given its viola name, instance dir and binary · new text
divergences · test-plan · decision/proptest-strategies · code
  rule · written: proptest strategies, with proptest-derive, are what generate property-test data.
  rule · proposed: proptest strategies are what generate property-test data.
  rule · lands: proptest strategies are what generate property-test data. · = proposed
divergences · test-plan · decision/product-state-from-real-runs · code
  except · written: chaos tests corrupt files the product already wrote; server-identity impostors build their own snapshot.json
  except · lands: chaos tests corrupt files the product already wrote; server-identity impostors build their own snapshot.json; unit tests and viola-state's own tests seed state files for the code under test. · new text
divergences · test-plan · decision/stamps-written-only-by-verify · code
  except · written: -
  except · lands: viola-state's own unit tests write it, through the stamps store or as seeded bytes, to test that store and its strict-modes check. · new text
divergences · test-plan · decision/injected-clock · code
  rule · written: Time-sensitive data is injected per test behind viola_core::Clock, which tests advance; the wall clock is read only through that clock seam.
  rule · lands: Time-sensitive data is injected per test behind viola_core::Clock, a monotonic clock tests advance, or handed to a pure decision as an instant; the wall clock is read directly, through Utc::now, for record stamps and for the instant a decision takes. · new text
  except · written: A link line's since is stamped from the wall clock and asserted by its shape only, and the budget gate's three evaluation points read the wall clock.
  except · lands: A link line's since is stamped from the wall clock and asserted by its shape only. · new text
divergences · test-plan · decision/fixture-hygiene-walk · rule
  rule · written: One run-time directory walk checks every fixture for absolute paths, non-placeholder usernames, email-shaped tokens and usernames split across a screen row seam, and validates each file against its JSON schema.
  rule · lands: One run-time directory walk checks every fixture for absolute paths, non-placeholder usernames, email-shaped tokens and usernames split across a screen row seam, and validates each file against its JSON schema. · = written
divergences · test-plan · decision/no-test-owned-real-time · code
  rule · written: Test-side code uses deterministic, injected clocks: no test-owned timer, sleep or elapsed-time verdict.
  rule · lands: Test-side code uses deterministic, injected clocks: no test-owned timer, sleep or elapsed-time verdict, save a generous bound on a latency no injected clock reaches, in another process or on a real socket. · new text
divergences · test-plan · decision/fd-test-reads-live-child-table · rule
  rule · written: The env-strip and fd tests read the running child's live fd table, never the fake agent's self-dump.
  rule · lands: The env-strip and fd tests read the running child's live fd table, never the fake agent's self-dump. · = written
divergences · test-plan · decision/no-fd-count-assertion · rule
  rule · written: The child fd test asserts what the child holds, never a count of its descriptors.
  rule · lands: The child fd test asserts what the child holds, never a count of its descriptors. · = written
divergences · test-plan · decision/property-tests-with-proptest · code
  rule · written: Input parsers and validators are property-tested with proptest at 512 cases, each with its regression file committed.
  rule · proposed: Input parsers and validators are property-tested with proptest at 512 cases; proptest-regressions files are committed for the viola-core, hook, statusline and agents suites.
  rule · lands: Input parsers and validators are property-tested with proptest at 512 cases; proptest-regressions files are committed for the viola-core, hook, statusline and agents suites. · = proposed
divergences · test-plan · decision/concurrent-append-whole-lines · code
  rule · written: Separate processes appending at once to each shared diagnostics file, including detail lines larger than 4 KiB, must leave every line parsing as one JSON object; a torn or interleaved line fails the test.
  rule · lands: Separate hook processes appending at once to the hook role file and the detail file, including detail lines larger than 4 KiB, must leave every line parsing as one JSON object; every diagnostics file appends through those same two paths, and a torn or interleaved line fails the test. · new text
divergences · test-plan · decision/time-faked-through-seams · code
  rule · written: Sync crates fake time with mock_instant or the injected Clock, MCP adapter timers use tokio's paused test clock, and a pure decision takes its instant as an argument.
  rule · lands: Time is faked only through the injected viola_core Clock, with no mock_instant; the MCP adapter runs no tokio timer, so no paused test clock, and a pure decision takes its instant as an argument. · new text
divergences · test-plan · decision/utc-now-through-clock-seam · code · as decision/utc-now-read-directly
  name · written: Wall clock read through the clock seam
  name · lands: Wall clock read directly, not through the clock seam · new text
  rule · written: chrono Utc::now() is read only through the clock seam.
  rule · lands: chrono Utc::now() is read directly for wall-clock stamps and for the instant a pure decision such as the budget gate's takes; the clock seam carries monotonic Instants only. · new text
  except · written: The budget gate's evaluation points read it and hand the instant to its pure decision; the link line's since is stamped directly, so tests assert since by shape only.
  except · lands: The link line's since is stamped directly, so tests assert since by shape only. · new text
divergences · test-plan · decision/fake-agent-replays-recorded-bytes · code
  rule · written: The fake agent offers each hook the recorded fixture bytes and builds no payload, save the resume launch SessionStart for a set without the recorded resume variant and the compact replay's session id.
  rule · lands: The fake agent offers each hook the recorded fixture bytes and builds no payload, save the fields it sets: the typed prompt of a default UserPromptSubmit, the launch SessionStart's source and session_id under --resume whichever variant fires, and the compact replay's session_id. · new text
divergences · test-plan · decision/fixtures-walked-for-hygiene · rule
  rule · written: Every committed fake script and Claude fixture is walked against its schema and for absolute paths, usernames and emails, with planted cases proving each class red.
  rule · proposed: Every committed fake script and Claude fixture is walked against its schema and for absolute paths and usernames (emails too for MCP exchanges and screens), with planted cases proving each class red.
  rule · lands: Every committed fake script and Claude fixture is walked against its schema and for absolute paths, usernames and emails, with planted cases proving each class red. · = written
divergences · test-plan · decision/fixtures-recorded-by-live-verify · code
  rule · written: The recorded agent fixtures are refreshed only by a live viola verify run, locally.
  rule · proposed: The recorded agent hook, dialog, framing, status line, resume and screen fixtures are refreshed only by a live viola verify run, locally; the recorded MCP exchange fixtures by the test-only viola-mcp-tap.
  rule · lands: The recorded agent hook, dialog, framing, status line, resume and screen fixtures are refreshed only by a live viola verify run, locally; the recorded MCP exchange fixtures by the test-only viola-mcp-tap. · = proposed
divergences · test-plan · decision/fuzz-nightly-and-corpus-gate · code · as decision/fuzz-weekly-and-corpus-gate
  rule · written: cargo-fuzz targets run as a time-boxed nightly job, plus a corpus replay gate, with each parser getting a target as it lands.
  rule · proposed: cargo-fuzz targets run as a time-boxed weekly job, plus a corpus replay gate, with each parser getting a target as it lands.
  rule · lands: cargo-fuzz targets run as a time-boxed weekly job, plus a corpus replay gate, with each parser getting a target as it lands. · = proposed
  name · written: Fuzzing nightly with a corpus replay gate
  name · lands: Fuzzing weekly with a corpus replay gate · new text
divergences · test-plan · decision/tests-never-read-host-sessions · code
  rule · written: A test's list child runs with PATH set to an empty bin dir, and the fake agent is planted as claude only through one shared helper.
  rule · lands: A test's own list child runs with PATH set to one bin dir alone, empty or holding the planted fake claude, and the fake agent is planted as claude only by shared code: two test-support helpers and the e2e harness boot. · new text
divergences · test-plan · decision/paste-injection-test-layers · holds
  rule · written: Paste injection is tested by unit property tests, integration and E2E.
  rule · lands: Paste injection is tested by unit property tests, integration and E2E. · = written
divergences · test-plan · decision/hook-unknown-fields-tolerated · code
  rule · written: Unknown fields in a hook payload are counted, never fatal.
  rule · proposed: Unknown fields in a hook payload are skipped, never fatal; only a known field of the wrong type is recorded as a drift entry.
  rule · lands: Unknown fields in a hook payload are skipped, never fatal; only a known field of the wrong type is recorded as a drift entry. · = proposed
divergences · test-plan · decision/one-global-tracing-install · code
  rule · written: The binary installs a global tracing default in exactly one place, the gate.
  rule · lands: The binary installs a global tracing default in exactly one place, its observability init; a unit test installs one only as the one global-subscriber test of its process. · new text
divergences · test-plan · decision/diagnostics-checked-before-narrowing · code
  rule · written: A diagnostics folder, role file or detail file that fails the strict check is a failed open, checked before anything is narrowed and left as found; the writer goes to a sink and a detail line is dropped.
  rule · lands: A diagnostics folder, role file or detail file that fails the strict check is a failed open, checked before anything is narrowed and left as found; a failed role open installs no subscriber and a detail line is dropped. · new text
divergences · test-plan · decision/no-nested-harness-in-nextest · code
  name · written: No test shells out to the harness
  name · lands: No nested cargo run inside nextest · new text
  rule · written: No nextest test shells out to agent-run or the harness binary, and in-test sessions skip the build, so there is never a nested cargo run inside nextest.
  rule · lands: No nextest test shells out to agent-run, a test calls the viola-harness binary only on paths that never build, and in-test sessions skip the build, so there is never a nested cargo run inside nextest. · new text
divergences · test-plan · decision/members-declare-fake-agent-feature · code
  rule · written: A mutable workspace member declares a fake-agent feature, a no-op where it has none.
  rule · proposed: Every workspace member but viola-core declares a fake-agent feature, a no-op where it has none.
  rule · lands: Every workspace member but viola-core declares a fake-agent feature, a no-op where it has none. · = proposed
divergences · test-plan · decision/ci-keeps-test-homes · code
  rule · written: In CI, AGENT_RUN_KEEP_HOMES=1 is set workflow-wide (only run --mutants overrides it to 0), so every rstest home and every harness session home stays under target/e2e-home until obs-plan's G2, G4, secret scan and scan-gated uploads have run.
  rule · proposed: In CI, AGENT_RUN_KEEP_HOMES=1 is set job-wide in the test and perf jobs (only run --mutants overrides it to 0), so every rstest home and every harness session home of those jobs stays under target/e2e-home until obs-plan's G2, G4, secret scan and scan-gated uploads have run.
  rule · lands: In CI, AGENT_RUN_KEEP_HOMES=1 is set job-wide in the test and perf jobs (only run --mutants overrides it to 0), so every rstest home and every harness session home of those jobs stays under target/e2e-home until obs-plan's G2, G4, secret scan and scan-gated uploads have run. · = proposed
divergences · test-plan · decision/ci-uploads-only-after-secret-scan · code
  rule · written: On CI failure the home's diagnostics, events and harness artifacts leave CI only in scan-gated uploads, after the secret scan has passed on them; there is no unscanned upload.
  rule · proposed: On CI failure the home's diagnostics, events and harness artifacts leave CI only in uploads gated on a passed secret scan; the only ungated uploads (the supply-chain reports among them) carry no home content.
  rule · lands: On CI failure the home's diagnostics, events and harness artifacts leave CI only in uploads gated on a passed secret scan; the only ungated uploads (the supply-chain reports among them) carry no home content. · = proposed
divergences · test-plan · decision/ci-tools-version-pinned · code
  rule · written: Every CI-invoked binary is version-pinned.
  rule · lands: Every tool CI installs is version-pinned; binaries the runner image ships, such as jq and python3, run unpinned. · new text
divergences · test-plan · decision/no-secrets-in-logs · code
  rule · written: No log line and nothing in diagnostics/ holds a Cookie header or a stripped CLAUDE* value, and diagnostics/ is mode 0600.
  rule · lands: No log line and nothing in diagnostics/ holds a Cookie header or a stripped CLAUDE* value, and diagnostics/ is mode 0700 with its files 0600. · new text
divergences · test-plan · decision/integration-binary-surface-prefix · code
  name · written: Integration binaries prefixed by surface
  name · lands: Root integration binaries prefixed by surface · new text
  rule · written: Rust integration binaries use a <surface>_<topic>.rs prefix, the surface prefix naming the layer.
  rule · lands: Root integration binaries use a <surface>_<topic>.rs prefix, the surface prefix naming the layer; crate-local integration binaries need no surface prefix. · new text
divergences · test-plan · decision/fixture-chain-exists-twice · code · as decision/fixture-chain-exists-once
  name · written: Fixture chain exists twice
  name · lands: Fixture chain exists once · new text
  rule · written: The home, fake_agent_path, stamped_home and booted_wrapper chain exists twice, as the sync root copy and the viola-e2e copy, against one contract.
  rule · lands: The home, fake_agent_path, stamped_home and booted_wrapper chain exists once, as the sync root copy in tests/support; viola-e2e has no copy and boots its sessions through its own harness_session. · new text
divergences · obs-plan · decision/tracing-json-line-file-pipeline · holds
  rule · written: Telemetry is tracing plus tracing-subscriber writing JSON-per-line files under diagnostics/, correlated by corr and consumed through agent-run logs and status.
  rule · lands: Telemetry is tracing plus tracing-subscriber writing JSON-per-line files under diagnostics/, correlated by corr and consumed through agent-run logs and status. · = written
divergences · obs-plan · decision/agents-probe-parse-failure-unknown · code
  rule · written: A parse failure of the claude agents --json or --version probe becomes "unknown" plus a counter.
  rule · proposed: A parse failure of the claude agents --json probe becomes status "unknown" plus a parse-rejected counter; an unparsed --version answer leaves cli_version absent and the version unverified.
  rule · lands: A parse failure of the claude agents --json probe becomes status "unknown" plus a parse-rejected counter; an unparsed --version answer leaves cli_version absent and the version unverified. · = proposed
divergences · obs-plan · decision/hook-logger-init-no-blocking-io · code
  rule · written: Logger initialisation in viola hook adds no blocking I/O beyond one file open and append.
  rule · proposed: Logger initialisation in viola hook runs the strict-modes checks, creates the owner-only diagnostics directories and opens the role file once for append, adding no other I/O.
  rule · lands: Logger initialisation in viola hook reads the diagnostics level, runs the strict-modes checks, creates or narrows the home and diagnostics/ to 0700 and opens the role file once for append at 0600, adding no other I/O. · new text
divergences · obs-plan · decision/mcp-log-file-sink · code
  rule · written: The `mcp` process always logs to the file sink `<home>/diagnostics/mcp.ndjson`, with stderr JSON only when that file cannot be opened, and the log writer is set explicitly.
  rule · lands: The `mcp` process always logs to the file sink `<home>/diagnostics/mcp.ndjson`, with no stderr fallback: a file that cannot be opened exits 1 printing nothing, and the log writer is set explicitly. · new text
divergences · obs-plan · decision/one-log-file-per-role · code
  rule · written: Each process role writes its own file under `diagnostics/`, no role has a stdout sink, and stderr JSON is used only by `mcp` when its file cannot be opened.
  rule · lands: Each process role writes its own file under `diagnostics/`, no role has a stdout sink, and no role falls back to stderr JSON when its file cannot be opened. · new text
divergences · obs-plan · decision/run-threads-inside-catch-unwind · code
  rule · written: The run PTY pump and handle-wait threads each run inside catch_unwind; a worker panic that leaves main running with no process-exit is forbidden.
  rule · lands: The two run PTY pump threads each run inside catch_unwind, and the child's exit is polled inline on run's main thread under main's catch_unwind; a worker panic that leaves main running with no process-exit is forbidden. · new text
divergences · obs-plan · decision/payload-types-derive-redact · rule
  rule · written: Every channel, hook and MCP payload type derives veil Redact, so Debug never reveals content.
  rule · proposed: The channel Request type derives veil Redact, so its params never show in Debug; other payload types do not derive it.
  rule · lands: Every channel, hook and MCP payload type derives veil Redact, so Debug never reveals content. · = written
divergences · obs-plan · decision/correlation-propagation · code
  rule · written: Every channel call logs corr and conn on both sides, every send line carries the send cursor, and every dialog line carries dialog_id.
  rule · lands: Every channel call logs corr and conn on both sides; a send line from send-issued on carries the send cursor, a wrapper refusal before it the events.ndjson end offset as corr, and a client-side refusal no corr; every dialog line carries dialog_id. · new text
divergences · obs-plan · decision/deterministic-emission · code
  except · written: -
  except · lands: The readiness gate reads the vt100 screen before a paste, and its verdict can refuse a send as input-not-ready. · new text
divergences · obs-plan · decision/build-fails-on-any-panic · code
  name · written: Build fails on any panic
  name · lands: Build fails on any panic line · new text
  rule · written: The build fails on any panic event line, or on any panic evidenced by an exit code without its line.
  rule · lands: The build fails on any panic event line in the kept homes' role files, bar the fake-agent seam's forced panic; no gate infers a panic from an exit code. · new text
divergences · obs-plan · decision/closed-event-and-corr-on-every-line · code
  name · written: Closed event enum and corr on every line
  name · lands: Closed event enum on every line, corr where correlated · new text
  rule · written: Every log line carries the closed event enum and corr, so the signal pyramid never collapses into free text.
  rule · lands: Every log line carries the closed event enum, and corr rides only on the correlated events the schema allows it on, so the signal pyramid never collapses into free text. · new text
divergences · obs-plan · decision/diag-lines-schema-checked-in-unit-tests · code
  name · written: Emitted lines schema-checked in unit tests
  name · lands: Emitted lines schema-checked in integration and E2E tests · new text
  rule · written: Unit tests write to an in-memory writer and parse every emitted line against schemas/diag-line.v1.json.
  rule · lands: Emitted role lines are checked against schemas/diag-line.v1.json by integration tests and the E2E schema_check over real process output; unit tests check only detail lines, against schemas/diag-detail.v1.json. · new text
divergences · obs-plan · decision/run-worker-panic-ends-run · code
  rule · written: A panic in the PTY pump or handle-wait thread is sent to main, which takes run's exit path; a worker panic leaving main running with no process-exit is forbidden.
  rule · lands: A panic in either PTY pump thread is sent to main, which kills the child and takes run's exit path, exit 1 internal-error; the child's exit is polled on main, not on a handle-wait thread; a worker panic leaving main running with no process-exit is forbidden. · new text
divergences · obs-plan · decision/hook-lines-required · code
  rule · written: Every hook process writes its hook-invoked and hook-decision lines.
  rule · lands: Every hook process in a trusted wrapped session writes its hook-invoked and hook-decision lines, bar a PreToolUse for a tool outside the dialog matcher; a hook outside such a session or on a --capture probe writes neither. · new text
divergences · obs-plan · decision/mcp-log-sink · code
  rule · written: viola mcp always logs to <home>/diagnostics/mcp.ndjson once the home and its diagnostics/ pass strict-modes, with stderr JSON only when that file cannot be opened.
  rule · lands: viola mcp always logs to <home>/diagnostics/mcp.ndjson once the home and its diagnostics/ pass strict-modes; a file that cannot be opened exits 1 printing nothing, with no stderr fallback. · new text
divergences · obs-plan · decision/hook-fail-open-logged-to-hook-file · code
  rule · written: Hook fail-open paths (oversize stdin, malformed JSON, channel failure, server-verify mismatch, refused socket directory, missing VIOLA_NAME) are logged to hook-<name>.ndjson only.
  rule · proposed: Hook fail-open paths (oversize stdin, malformed JSON, channel failure, server-verify mismatch, refused socket directory) are logged to hook-<name>.ndjson only; a missing VIOLA_NAME writes no line.
  rule · lands: Hook fail-open paths (oversize stdin, malformed JSON, channel failure, server-verify mismatch, refused socket directory) are logged to hook-<name>.ndjson only; a missing VIOLA_NAME writes no line. · = proposed
divergences · obs-plan · decision/per-parser-parse-failure-counts · code
  rule · written: Each parser (paste validation, channel framing at MAX_FRAME, the viola mcp stdin at MAX_FRAME, hook stdin, claude agents --json, vt100 under catch_unwind) keeps its own parse-failure count.
  rule · lands: Each parser that drops input (channel framing at MAX_FRAME, the viola mcp stdin at MAX_FRAME, hook stdin, claude agents --json, vt100 under catch_unwind) keeps its own parse-failure count; paste validation refuses a send as not-delivered instead. · new text
divergences · obs-plan · decision/content-sink-behind-strict-check · code
  rule · written: The diagnostics content sink opens only after a strict-modes check of the folder and file; a symlink or a folder or file another user may read refuses the open, the writer becomes a sink and the line is dropped.
  rule · proposed: The diagnostics content sink opens only after a strict-modes check of the folder and file; a symlink or a folder or file another user may read refuses the open as a failed open and the detail line is dropped.
  rule · lands: The diagnostics content sink opens only after a strict-modes check of the folder and file; a symlink or a folder or file another user may read refuses the open as a failed open and the detail line is dropped. · = proposed
divergences · obs-plan · decision/panic-count-home-level-only · code
  rule · written: The exactly-one panic rule, the viola.panic counter and gate G2 count home-level role files only; the companion panic line in a detail file is excluded.
  rule · proposed: The exactly-one panic rule and gate G2 count home-level role files only; the companion panic line in a detail file is excluded.
  rule · lands: The exactly-one panic rule and gate G2 count home-level role files only; the companion panic line in a detail file is excluded. · = proposed
divergences · obs-plan · decision/cli-log-only-under-viola-name · code
  rule · written: A short-lived cli verb opens its process log and writes process-start and process-exit only when an instance resolves under a parsed VIOLA_NAME; with no name it writes no log.
  rule · proposed: A short-lived cli verb opens its process log and writes process-start and process-exit only when an instance resolves: for list, plugin install and revive --list under a parsed VIOLA_NAME (with no name no log), for the frame verbs under the named target (link and unlink the driven) instance.
  rule · lands: A short-lived cli verb opens its process log and writes process-start and process-exit only when an instance resolves: for list, plugin install and revive --list under a parsed VIOLA_NAME (with no name no log), for the frame verbs under the named target (link and unlink the driven) instance. · = proposed
divergences · obs-plan · decision/mcp-explicit-log-writer · code
  rule · written: The viola mcp log writer is always explicit: a file, with stderr only as a fallback.
  rule · lands: The viola mcp log writer is always explicit: the mcp.ndjson file, with no stderr fallback. · new text
divergences · obs-plan · decision/structured-logs-primary-signal · holds
  rule · written: tracing-subscriber JSON-per-line logs, one file per process role under the home's diagnostics directory, are the primary observability signal, merged with the event stream by agent-run logs.
  rule · lands: tracing-subscriber JSON-per-line logs, one file per process role under the home's diagnostics directory, are the primary observability signal, merged with the event stream by agent-run logs. · = written
divergences · obs-plan · decision/panic-caught-at-every-role · code
  rule · written: A hand-written panic hook plus catch_unwind at every role's main-thread catch site, hook dispatch, the run PTY pump and handle-wait threads, and the vt100 feed turn a panic into one record.
  rule · lands: A hand-written panic hook plus catch_unwind at every role's main-thread catch site (covering hook dispatch and run's inline handle-wait), the two run PTY pump threads and the vt100 feed turn a panic into one record. · new text
divergences · obs-plan · decision/instance-source-per-role · code
  rule · written: run takes its own name argument, never an inherited VIOLA_NAME; hook and mcp take VIOLA_NAME only; cli takes the caller's VIOLA_NAME, and the target argument only when VIOLA_NAME is unset.
  rule · proposed: run takes its own name argument, never an inherited VIOLA_NAME; hook and mcp take VIOLA_NAME only; cli verbs that act on an instance (send, wait, last, answer, pause, release, link, unlink) take the target argument; list, verify, plugin install and revive --list take the caller's VIOLA_NAME only.
  rule · lands: run takes its own name argument, never an inherited VIOLA_NAME; hook and mcp take VIOLA_NAME only; cli verbs that act on an instance (send, wait, last, answer, pause, release, link, unlink) take the target argument; list, verify, plugin install and revive --list take the caller's VIOLA_NAME only. · = proposed
divergences · obs-plan · decision/private-diagnostics-files · rule
  rule · written: The diagnostics directory is 0700 and each diagnostics file 0600, opened without following symlinks; before the first write each must be owned by the effective user, have no group or other bits and not be a symlink, a failed check counting as a failed open.
  rule · lands: The diagnostics directory is 0700 and each diagnostics file 0600, opened without following symlinks; before the first write each must be owned by the effective user, have no group or other bits and not be a symlink, a failed check counting as a failed open. · = written
divergences · obs-plan · decision/failed-log-open-discards · code
  rule · written: When the role file cannot be opened, the diagnostics writer discards lines; only mcp writes to stderr instead.
  rule · proposed: When the role file cannot be opened, viola_obs_init returns the error and installs no subscriber, so no line is written; hook carries on and the other roles, mcp included, return the error.
  rule · lands: When the role file cannot be opened, viola_obs_init returns the error and installs no subscriber, so no line is written; hook carries on and the other roles, mcp included, return the error. · = proposed
divergences · obs-plan · decision/span-carried-into-tasks-and-threads · code
  name · written: Span carried into tasks and threads
  name · lands: Span not carried into tasks and threads · new text
  rule · written: Spawned async tasks are instrumented with their span, and std thread closures take a cloned span and run inside it.
  rule · lands: Spawned async tasks and std thread closures take no span from their spawner: spans never reach a log line, and correlation travels as explicit typed fields. · new text
divergences · obs-plan · decision/g1-bare-instrument-gate · code
  rule · written: Every #[instrument] must have skip_all as its first argument; the G1 gate passes only on rg exit 1 (no match), so a violation, a regex or I/O error or a missing rg fails, and a G1 failure is an SLO invariant failure.
  rule · lands: Every #[instrument] must have skip_all as its first argument; the G1 gate passes only on rg exit 1 (no match), so a violation, a regex or I/O error or a missing rg fails the CI step. · new text
divergences · obs-plan · decision/no-sleep-based-log-waits · holds
  rule · written: No log-based wait relies on sleeps.
  rule · lands: No log-based wait relies on sleeps. · = written
new text: 150 line(s) — every cell that is neither `= written` nor `= proposed`, and every kept `relied` row: the text nobody has approved
  gone · architecture · decision/snapshots-check-mode-only · rule: The agent crate's decision-body snapshots are only checked by tests, never written by them; the root integration tests take no snapshots.
  gone · test-plan · decision/one-test-per-recorded-hook-payload → decision/recorded-hook-payloads-walked-at-run-time · name: Recorded hook payloads walked by one test
  gone · test-plan · decision/one-test-per-recorded-hook-payload → decision/recorded-hook-payloads-walked-at-run-time · rule: One contract test walks every recorded hook payload fixture at run time, and an empty walk fails.
  gone · test-plan · decision/host-unreachable-mutants-owed → decision/host-unreachable-mutants-excluded · name: Host-unreachable mutants recorded as host-excluded
  gone · test-plan · decision/host-unreachable-mutants-owed → decision/host-unreachable-mutants-excluded · rule: A missed mutant the host never compiled is recorded host-excluded: kept out of the survivors and never counted as caught.
  gone · test-plan · decision/mock-instant-behind-clock-seam → decision/test-clocks-behind-clock-seam · name: Test clocks behind the clock seam
  gone · test-plan · decision/mock-instant-behind-clock-seam → decision/test-clocks-behind-clock-seam · rule: Time in the sync crates is faked through the core Clock seam with hand-written test clocks; no mocking crate stands behind it.
  gone · obs-plan · contract/parse-rejected-detail/control-character · promise: Listed in the diagnostics schema for the paste-text parser, but never emitted: a control character in a paste is refused as send-refused not-delivered.
  gone · obs-plan · contract/derived-metrics/viola-panic → decision/panic-lines-stay-zero · name: Panic lines stay at zero
  gone · obs-plan · contract/derived-metrics/viola-panic → decision/panic-lines-stay-zero · rule: Gate G2 counts the panic lines of the home-level role files and fails on any; no viola.panic metric is derived.
  gone · obs-plan · contract/span-kinds/internal · promise: The INTERNAL spans are run.*, state.* and pty.paste_write; a hook's decision is the decision_emitted field, not a span.
  guards · test-plan · decision/cross-user-at-peer-check-boundary · guard: peer_euid_admitted_takes_only_the_owning_user
  guards · test-plan · decision/leading-slash-never-a-prompt · guard: send_leading_slash_argument_is_a_usage_error
  guards · test-plan · decision/peer-euid-admission · guard: serve_drops_a_refused_peer_before_any_frame, peer_euid_admitted_takes_only_the_owning_user
  guards · obs-plan · decision/source-allow-list-logging · guard: disallowed-macros; raw_event_hits; G1 bare instrument
  guards · obs-plan · decision/no-log-crate-bridge · guard: pty_exit_is_read_on_the_handle_while_the_output_is_held
  guards · obs-plan · decision/hook-single-log-open · guard: SPINE_BOUND
  divergences · layout-templates · decision/board-vs-strip · rule: `viola list` shows the whole bay, `verify` and `plugin install` name no strip, and every other verb a person runs acts on one strip named by its `ViolaName`.
  divergences · design-system · decision/stable-session-slots · rule: Every session keeps a slot in the strip bay stable across state changes: wrapped sessions in name order, then unwrapped ones in name order; nothing is re-sorted by state, and a slot shifts only when a session arrives or leaves.
  divergences · design-system · decision/list-is-the-board · rule: `viola list` is the board, `verify` and `plugin install` act on no strip, and every other verb a person runs acts on one strip.
  divergences · a11y-plan · decision/codes-beside-plain-word · rule: A kebab-case reason or detail code (`not-delivered`, `human-typing`, `unverified-cli`) on a refusal line never appears without the plain word `unable`; the board's cells show status codes such as `unverified-cli` without it.
  divergences · a11y-plan · decision/no-screen-content-parsing · name: No screen-content parsing beyond signatures
  divergences · a11y-plan · decision/no-screen-content-parsing · rule: The rendered `claude` TUI screen is parsed only for compiled input-box and modal signatures, never for other content, and screenshots are never taken as a11y evidence (R7).
  divergences · security-plan · decision/cli-tool-version-floors → decision/cli-tool-exact-pins · name: Exact pins for external CLI tools
  divergences · security-plan · decision/cli-tool-version-floors → decision/cli-tool-exact-pins · rule: External CLI and audit tools are installed at exact pinned versions, in CI and by the ripgrep install script, and CI actions are pinned by commit SHA; no tool is expressed as a minimum version floor.
  divergences · security-plan · decision/cli-tool-version-floors → decision/cli-tool-exact-pins · why: CI installs each tool at its pin itself, and no drift check compares a locally installed build against a pin.
  divergences · security-plan · decision/nutype-input-validation · rule: Input validation uses nutype, already in the stack; no new validation library is added, and validated names are enforced at the CLI, MCP handlers and channel link and unlink params.
  divergences · security-plan · decision/exec-commands-absolute-pinned-path · name: Exec commands use an absolute path
  divergences · security-plan · decision/exec-commands-absolute-pinned-path · rule: Every exec-form command viola writes for the CLI is an absolute path, never a PATH lookup: the pinned bin/<version>-<hash>/viola path for run, revive and verify, and the installing exe's canonical path in plugin install's `.mcp.json`.
  divergences · architecture · decision/tolerant-external-payloads · rule: Hook JSON, `claude agents --json` and statusline payloads are parsed with no `deny_unknown_fields`, with defaults and options; agents rows keep the raw value for the fixture recorder, and serde_path_to_error gives drift reports for hook JSON only.
  divergences · architecture · decision/nutype-errors-fold-into-crate-enum → decision/nutype-errors-not-folded · name: nutype validation errors stay out of crate enums
  divergences · architecture · decision/nutype-errors-fold-into-crate-enum → decision/nutype-errors-not-folded · rule: A nutype validation error is never folded into a crate error enum: the caller drops it, refusing the input (a usage error, invalid params, a closed field word) or treating it as absent (a config default with a rejection, no instance).
  divergences · architecture · decision/env-var-viola-prefix · except: Ratified: test seams `FAKE_AGENT_PUMP_DELAY_MS`, `FAKE_AGENT_HOOK_PANIC`; `NO_COLOR`, `TERM`, `COLORTERM`, read only for SGR bytes; `PATH`, read and set for the child; `XDG_RUNTIME_DIR`, read only.
  divergences · architecture · decision/harness-env-var-prefix · name: Test-harness knobs carry the AGENT_RUN_ prefix
  divergences · architecture · decision/harness-env-var-prefix · rule: Test-harness knobs use the `AGENT_RUN_` prefix and are never read by the `viola` binary; the `FAKE_AGENT_*` seams, read only by fake-agent builds, and each crate's test-private self-spawn markers (`PTY_SEAM_TEST_*`, `MCP_*_TEST_*`) keep their own names.
  divergences · architecture · decision/timestamp-field-names · rule: Every instance event, and the `hook.event` frame, carries its own instant in the field `ts`; request frames carry no instant, process-log and detail lines carry `timestamp`, and other instants use `*_at` fields, except the start instant a link record holds.
  divergences · architecture · decision/fake-agent-feature-on-every-member → decision/fake-agent-feature-on-named-members · name: A fake-agent feature on each named member crate
  divergences · architecture · decision/no-environment-configuration · except: NO_COLOR, TERM and COLORTERM for list's human-mode colour; XDG_RUNTIME_DIR for the socket dir; PATH for program lookup; two fake-agent test seams compiled only under the fake-agent feature.
  divergences · architecture · decision/thiserror-one-enum-per-crate · rule: Each workspace library crate except viola-core has one thiserror error enum, the refusal reason type lives in the core crate, and anyhow is used only in the root viola bin.
  divergences · test-plan · decision/no-test-retries-or-sleeps · rule: nextest runs with no retries, and no thread or tokio sleep is used for synchronisation, save as the poll interval of a loop that rechecks its condition, which in a test wait also runs under a deadline.
  divergences · test-plan · decision/wrapper-channel-integration-driver · rule: Wrapper channel integration tests drive the viola-channel sync client, a raw interprocess local socket stream for malformed frames, and jsonschema validators built from the committed schema files, which check diagnostics lines while reply frames are compared directly.
  divergences · test-plan · decision/on-disk-state-tested-on-real-files · rule: Module-to-disk state is tested over real ndjson, snapshots and lock siblings in a tempfile home, with a concurrent-append check on the hook role and detail files, whose open and write paths every diagnostics file shares, that every line parses as one JSON object.
  divergences · test-plan · decision/child-holds-no-viola-paths → decision/child-env-names-viola-instance · name: Child given its viola name, instance dir and binary
  divergences · test-plan · decision/product-state-from-real-runs · except: chaos tests corrupt files the product already wrote; server-identity impostors build their own snapshot.json; unit tests and viola-state's own tests seed state files for the code under test.
  divergences · test-plan · decision/stamps-written-only-by-verify · except: viola-state's own unit tests write it, through the stamps store or as seeded bytes, to test that store and its strict-modes check.
  divergences · test-plan · decision/injected-clock · rule: Time-sensitive data is injected per test behind viola_core::Clock, a monotonic clock tests advance, or handed to a pure decision as an instant; the wall clock is read directly, through Utc::now, for record stamps and for the instant a decision takes.
  divergences · test-plan · decision/injected-clock · except: A link line's since is stamped from the wall clock and asserted by its shape only.
  divergences · test-plan · decision/no-test-owned-real-time · rule: Test-side code uses deterministic, injected clocks: no test-owned timer, sleep or elapsed-time verdict, save a generous bound on a latency no injected clock reaches, in another process or on a real socket.
  divergences · test-plan · decision/concurrent-append-whole-lines · rule: Separate hook processes appending at once to the hook role file and the detail file, including detail lines larger than 4 KiB, must leave every line parsing as one JSON object; every diagnostics file appends through those same two paths, and a torn or interleaved line fails the test.
  divergences · test-plan · decision/time-faked-through-seams · rule: Time is faked only through the injected viola_core Clock, with no mock_instant; the MCP adapter runs no tokio timer, so no paused test clock, and a pure decision takes its instant as an argument.
  divergences · test-plan · decision/utc-now-through-clock-seam → decision/utc-now-read-directly · name: Wall clock read directly, not through the clock seam
  divergences · test-plan · decision/utc-now-through-clock-seam → decision/utc-now-read-directly · rule: chrono Utc::now() is read directly for wall-clock stamps and for the instant a pure decision such as the budget gate's takes; the clock seam carries monotonic Instants only.
  divergences · test-plan · decision/utc-now-through-clock-seam → decision/utc-now-read-directly · except: The link line's since is stamped directly, so tests assert since by shape only.
  divergences · test-plan · decision/fake-agent-replays-recorded-bytes · rule: The fake agent offers each hook the recorded fixture bytes and builds no payload, save the fields it sets: the typed prompt of a default UserPromptSubmit, the launch SessionStart's source and session_id under --resume whichever variant fires, and the compact replay's session_id.
  divergences · test-plan · decision/fuzz-nightly-and-corpus-gate → decision/fuzz-weekly-and-corpus-gate · name: Fuzzing weekly with a corpus replay gate
  divergences · test-plan · decision/tests-never-read-host-sessions · rule: A test's own list child runs with PATH set to one bin dir alone, empty or holding the planted fake claude, and the fake agent is planted as claude only by shared code: two test-support helpers and the e2e harness boot.
  divergences · test-plan · decision/one-global-tracing-install · rule: The binary installs a global tracing default in exactly one place, its observability init; a unit test installs one only as the one global-subscriber test of its process.
  divergences · test-plan · decision/diagnostics-checked-before-narrowing · rule: A diagnostics folder, role file or detail file that fails the strict check is a failed open, checked before anything is narrowed and left as found; a failed role open installs no subscriber and a detail line is dropped.
  divergences · test-plan · decision/no-nested-harness-in-nextest · name: No nested cargo run inside nextest
  divergences · test-plan · decision/no-nested-harness-in-nextest · rule: No nextest test shells out to agent-run, a test calls the viola-harness binary only on paths that never build, and in-test sessions skip the build, so there is never a nested cargo run inside nextest.
  divergences · test-plan · decision/ci-tools-version-pinned · rule: Every tool CI installs is version-pinned; binaries the runner image ships, such as jq and python3, run unpinned.
  divergences · test-plan · decision/no-secrets-in-logs · rule: No log line and nothing in diagnostics/ holds a Cookie header or a stripped CLAUDE* value, and diagnostics/ is mode 0700 with its files 0600.
  divergences · test-plan · decision/integration-binary-surface-prefix · name: Root integration binaries prefixed by surface
  divergences · test-plan · decision/integration-binary-surface-prefix · rule: Root integration binaries use a <surface>_<topic>.rs prefix, the surface prefix naming the layer; crate-local integration binaries need no surface prefix.
  divergences · test-plan · decision/fixture-chain-exists-twice → decision/fixture-chain-exists-once · name: Fixture chain exists once
  divergences · test-plan · decision/fixture-chain-exists-twice → decision/fixture-chain-exists-once · rule: The home, fake_agent_path, stamped_home and booted_wrapper chain exists once, as the sync root copy in tests/support; viola-e2e has no copy and boots its sessions through its own harness_session.
  divergences · obs-plan · decision/hook-logger-init-no-blocking-io · rule: Logger initialisation in viola hook reads the diagnostics level, runs the strict-modes checks, creates or narrows the home and diagnostics/ to 0700 and opens the role file once for append at 0600, adding no other I/O.
  divergences · obs-plan · decision/mcp-log-file-sink · rule: The `mcp` process always logs to the file sink `<home>/diagnostics/mcp.ndjson`, with no stderr fallback: a file that cannot be opened exits 1 printing nothing, and the log writer is set explicitly.
  divergences · obs-plan · decision/one-log-file-per-role · rule: Each process role writes its own file under `diagnostics/`, no role has a stdout sink, and no role falls back to stderr JSON when its file cannot be opened.
  divergences · obs-plan · decision/run-threads-inside-catch-unwind · rule: The two run PTY pump threads each run inside catch_unwind, and the child's exit is polled inline on run's main thread under main's catch_unwind; a worker panic that leaves main running with no process-exit is forbidden.
  divergences · obs-plan · decision/correlation-propagation · rule: Every channel call logs corr and conn on both sides; a send line from send-issued on carries the send cursor, a wrapper refusal before it the events.ndjson end offset as corr, and a client-side refusal no corr; every dialog line carries dialog_id.
  divergences · obs-plan · decision/deterministic-emission · except: The readiness gate reads the vt100 screen before a paste, and its verdict can refuse a send as input-not-ready.
  divergences · obs-plan · decision/build-fails-on-any-panic · name: Build fails on any panic line
  divergences · obs-plan · decision/build-fails-on-any-panic · rule: The build fails on any panic event line in the kept homes' role files, bar the fake-agent seam's forced panic; no gate infers a panic from an exit code.
  divergences · obs-plan · decision/closed-event-and-corr-on-every-line · name: Closed event enum on every line, corr where correlated
  divergences · obs-plan · decision/closed-event-and-corr-on-every-line · rule: Every log line carries the closed event enum, and corr rides only on the correlated events the schema allows it on, so the signal pyramid never collapses into free text.
  divergences · obs-plan · decision/diag-lines-schema-checked-in-unit-tests · name: Emitted lines schema-checked in integration and E2E tests
  divergences · obs-plan · decision/diag-lines-schema-checked-in-unit-tests · rule: Emitted role lines are checked against schemas/diag-line.v1.json by integration tests and the E2E schema_check over real process output; unit tests check only detail lines, against schemas/diag-detail.v1.json.
  divergences · obs-plan · decision/run-worker-panic-ends-run · rule: A panic in either PTY pump thread is sent to main, which kills the child and takes run's exit path, exit 1 internal-error; the child's exit is polled on main, not on a handle-wait thread; a worker panic leaving main running with no process-exit is forbidden.
  divergences · obs-plan · decision/hook-lines-required · rule: Every hook process in a trusted wrapped session writes its hook-invoked and hook-decision lines, bar a PreToolUse for a tool outside the dialog matcher; a hook outside such a session or on a --capture probe writes neither.
  divergences · obs-plan · decision/mcp-log-sink · rule: viola mcp always logs to <home>/diagnostics/mcp.ndjson once the home and its diagnostics/ pass strict-modes; a file that cannot be opened exits 1 printing nothing, with no stderr fallback.
  divergences · obs-plan · decision/per-parser-parse-failure-counts · rule: Each parser that drops input (channel framing at MAX_FRAME, the viola mcp stdin at MAX_FRAME, hook stdin, claude agents --json, vt100 under catch_unwind) keeps its own parse-failure count; paste validation refuses a send as not-delivered instead.
  divergences · obs-plan · decision/mcp-explicit-log-writer · rule: The viola mcp log writer is always explicit: the mcp.ndjson file, with no stderr fallback.
  divergences · obs-plan · decision/panic-caught-at-every-role · rule: A hand-written panic hook plus catch_unwind at every role's main-thread catch site (covering hook dispatch and run's inline handle-wait), the two run PTY pump threads and the vt100 feed turn a panic into one record.
  divergences · obs-plan · decision/span-carried-into-tasks-and-threads · name: Span not carried into tasks and threads
  divergences · obs-plan · decision/span-carried-into-tasks-and-threads · rule: Spawned async tasks and std thread closures take no span from their spawner: spans never reach a log line, and correlation travels as explicit typed fields.
  divergences · obs-plan · decision/g1-bare-instrument-gate · rule: Every #[instrument] must have skip_all as its first argument; the G1 gate passes only on rg exit 1 (no match), so a violation, a regex or I/O error or a missing rg fails the CI step.
  stale · layout-templates · contract/mcp-tool-results/success · promise: Each success is isError false with the CLI verb's --json `ok` payload alone as structuredContent, without `v` or the `ok` wrapper.
  stale · design-system · contract/colour-palette/accent · promise: The only attention colour: the `DIALOG` word of a pending dialog in `viola list` human output, as truecolor or its 256- and 16-colour fallbacks; never used for errors, warnings or budget.
  stale · architecture · overview/project-intent · text: viola is a hybrid local developer tool that lets one interactive Claude Code session drive another on the user's own subscription. It has a native Linux CLI binary and a Claude Code plugin of hooks and a stdio MCP server that calls that binary; a keyboard-driven terminal fleet console is planned, not built. It wraps the unmodified claude CLI in a PTY, types at turn boundaries, answers dialogs through hooks, and holds a one-driver wheel the human can take at any moment. It is a modular monolith: one binary with internal crates around core, and no daemon.
  stale · test-plan · contract/harness-stdout/document · promise: Every harness command but logs and supervise prints exactly one JSON document on stdout, starting with v 1, cmd naming the command and ok, in that key order; logs streams ndjson lines unless it fails, and supervise prints nothing.
  stale · test-plan · contract/cli-json/one-document-per-verb · promise: Each driver-facing verb prints one --json document and its literal exit code for every exit code but 2; a usage error or an unreadable input exits 2 with text on stderr and no document.
  stale · test-plan · contract/harness-boot/readiness-failure · promise: Exits non-zero with a JSON reason on stdout; a run that refuses to start is surfaced as reason run-exited with its instance and the exit code of its process-exit line, never its own text.
  stale · obs-plan · absence/pii-scrubbing-regime · capability: A PII-scrubbing regime for diagnostics lines beyond the NEVER-log floor
  stale · obs-plan · contract/log-levels/error · promise: Used for a panic, a process-exit that carries a detail, and a channel-response with error_code -32603.
  stale · obs-plan · contract/ipc-invocation-log/fields · promise: Channel-request and channel-response lines carry method, conn (srv_conn when the client sent none) and the corr of a frame with an id; a request line adds sender, a response line result class and an error's error code but never sender.
  stale · obs-plan · contract/viola-verify/home-tree-refused · promise: A refused home-tree check exits 1 with the home pair, opens no log and writes no line; a refused update_stamps exits 1 with the home pair and, when an instance is named, ends the opened log with a process-exit line carrying detail strict-modes-failed.
  stale · obs-plan · contract/channel-response/internal-error · promise: A -32603 is logged at ERROR with result_class error, error_code -32603 and no detail; the fixed text internal error is only the reply's message, never a log field.
  stale · obs-plan · contract/claude-child-process-log/handle-wait-thread → contract/claude-child-process-log/handle-wait-exit · promise: process-start is logged once the child is spawned and process-exit once the pump returns; process-exit{subject:"claude-child"} carries child_exit_status and exit_source handle-wait or kill-fallback, and is never emitted on reader EOF.
  stale · obs-plan · contract/dialog-spans/span-chain · promise: hook.handle, its channel.request(hook.dialog) on a thread of its own → channel.dispatch › run.dialog_register › run.dialog_await; the decision is the hook-decision line inside hook.handle, not a span; driver side answer.client › channel.request(answer) → channel.dispatch.
  stale · obs-plan · contract/detail-line/fields · promise: Each detail line carries timestamp (millisecond UTC format), level, target, message, event, process and instance, then its content fields, and no corr or conn key.
  stale · obs-plan · contract/diagnostics-line/instance · promise: Every diagnostics line of a process with an instance carries it as field instance; a process without one, such as an mcp server with no instance name, writes no instance key.
  stale · obs-plan · contract/agent-run-logs/filters · promise: The merged output is filterable by --instance, --kind and --process.
  stale · obs-plan · contract/send-line/wrapper-side · promise: send-* lines whose corr is a send cursor are emitted only by the target wrapper; only its send-refused line carries side:"wrapper", its send-issued and send-confirmed lines carry no side, and a client-side send-refused carries side:"client" and no corr.
  stale · obs-plan · contract/diagnostics-files/mcp · promise: Role mcp writes its role file mcp.ndjson in the home's diagnostics folder, which never carries user content; a role file that cannot be opened ends the server with exit 1 and nothing on stderr.
  relied · design-system · fct-mfvo2or6@1 → decision/list-refuses-board-on-strict-modes-failure · name: Strict-modes failure refuses the whole board · rule: A failed strict-modes check of the home or any instance refuses the whole list board with exit 21, in the fixed unable and hint pair or, under --json, the instance-unreachable document with detail strict-modes-failed. · why: The MCP list tool reads the same refusal document through the shared board reader, and the list tests pin exit 21 and the document byte for byte.
  relied · security-plan · fct-cw3l7aky@1 → decision/send-text-is-one-bracketed-paste · name: Send text typed as one bracketed paste · rule: Send text is written into the PTY as one bracketed paste followed by Enter, so an embedded paste terminator or control sequence would end the paste early and reach the claude TUI as keystrokes. · why: The control-character refusal exists to keep the paste closed. It runs in the wrapper's send, in CLI send and answer, and in the MCP send and answer tools.
  relied · security-plan · fct-ohbg3pqy@1 → decision/one-socket-dir-check-for-bind-and-clients · name: One socket-directory check for bind and clients · rule: The wrapper's bind and the three client seams share one check on the endpoint's parent directory before they bind or connect. The three seams are the CLI frame verbs, the hooks and the MCP frame tools. · why: Both sides of the socket read the same verdict, so a directory refused to a client is refused to a start, and the socket-dir-refused detail means one thing.
  relied · security-plan · fct-6oinutja@1 → decision/start-checks-socket-dir-before-bind · name: Socket directory checked before any bind, snapshot or child · rule: A start, run or revive alike, checks the socket directory before any bind, first snapshot or child, makes it only when absent, and checks again so the loser of a race to make it reads the same verdict. · why: When the directory is refused, the start negative test expects no child, no bind, no snapshot and the directory as found.
  relied · security-plan · fct-pf2l572y@1 → decision/refused-peer-dropped-before-any-frame · name: Refused peer dropped before any frame · rule: A peer the server refuses is dropped before its thread starts and before any frame is read, so it reads end-of-file, and no line and nothing on the terminal is written. · why: The server-side euid admission rests on it, and the serve test expects a refused peer's request to end closed with nothing dispatched.
  relied · security-plan · fct-mhqlyrp7@2 → decision/start-checks-home-tree-before-log-opens · name: Start checks the home tree before its log opens · rule: Run and revive's start check the whole home tree before the wrapper log opens: the home first, then the instance, ledger, bin and plugin folders and the pinned copy's own. A refusal exits 1 with no line or child. · why: Opening the log narrows the home to its owner, which would erase the state the check refuses; the revive and start tests expect the widened folder left as found.
  relied · security-plan · fct-zpaxdj2d@2 → decision/unreadable-peer-euid-is-refused · name: Unreadable peer euid refused, read silently · rule: A peer whose euid cannot be read is refused, and reading a connected stream's peer euid or pid writes and logs nothing. · why: Server admission and the client server check both rest on it, and the negative tests expect no pid or uid in any output or line.
  relied · security-plan · fct-5ryfhirx@2 → decision/only-matches-verdict-lets-a-frame-out · name: Only a Matches server verdict lets a frame out · rule: Only a Matches verdict, from a parsed snapshot read after the strict-modes check, lets a client write a frame; a mismatched pid or start time, or an unverifiable server, is refused before any frame. · why: The CLI verbs, the hooks and the MCP frame tools rely on it to reach only the recorded wrapper; server-verify-failed names the refusal.
  relied · security-plan · fct-p7bllbag@2 → decision/frame-verb-check-order · name: Frame verb check order, link and unlink included · rule: Link and unlink take the same order as every CLI frame verb: the strict-modes check, the liveness pre-check, then the socket-directory, euid and server checks before their one frame. · why: The socket-directory negative test runs link and unlink alongside the other six frame verbs and expects exit 21 with no frame on either side.
  relied · architecture · fct-y6frgi47@1 → decision/replayed-snapshot-gives-no-live-endpoint · name: Replayed snapshot gives no live endpoint · rule: A replayed snapshot carries no endpoint, pid or start time, so its readers treat the instance as unreachable and a start's collision check judges liveness from the heartbeat alone, refusing already-live within the stale bound. · why: Clients, hooks and the MCP tools find no endpoint through the replay, and the snapshot-replay chaos test expects a fresh beat to refuse already-live.
  relied · architecture · fct-xcjb7ry2@1 → decision/plugin-install-rewrites-marketplace-whole · name: Plugin install rewrites the marketplace tree whole · rule: The plugin marketplace tree is rewritten whole on every plugin install, any other entry there is left as found, and viola never reads the tree back. · why: A rerun re-points the MCP server at the current exe and home, the rerun test expects the same tree, and only its folders need the strict-modes check.
  relied · architecture · fct-q7tym3q2@1 → decision/silent-roles-at-internal-error-catch · name: Run, revive, mcp and hook silent at the catch site · rule: Run, revive's start, mcp and hook print nothing at the internal-error catch site; only a CLI verb prints the internal-error line or document there. · why: MCP stdout carries only protocol frames, a hook must stay silent to fail open, and a wrapper's terminal belongs to its child.
  relied · architecture · fct-dhyimyqh@1 → decision/connect-failure-dead-closed-stream-lost · name: Failed connect is instance-dead, lost stream endpoint-lost · rule: A failed connect reads as the instance-dead cause and a closed or broken stream as the endpoint-lost cause, on the CLI and in the MCP tools alike. · why: The exit 21 detail words, the process-exit during field and the MCP instance-unreachable detail all rest on the split.
  relied · architecture · fct-kbitf2mh@2 → decision/socket-dir-check-verdicts · name: Socket-directory check verdicts, never chmods · rule: The socket-directory check passes an absent path and never chmods, so a refused directory stays as found. It refuses a symlink, a non-directory, another owner, any group or other bit, and an unstattable path. · why: A start makes an absent directory itself, and the negative tests for start, frame verbs and hooks expect a loosened or linked directory left as found.
  relied · architecture · fct-kan7g7fj@1 → decision/mcp-frame-tools-strict-modes-check · name: MCP frame tools refuse a strict-modes failure · rule: Every MCP frame tool refuses a home or target instance directory that fails the strict-modes check with the instance-unreachable result and detail strict-modes-failed, sending no frame. · why: The handlers test expects the refusal before any snapshot read or frame, so an untrusted home never steers a frame to a wrapper.
  relied · architecture · fct-au3pdba5@1 → decision/mcp-unreachable-result-for-every-cause · name: MCP frame tools report every unreachable cause one way · rule: Every unreachable cause an MCP frame tool meets surfaces as the instance-unreachable result with the cause in detail and no line of its own; every cause but endpoint-lost is found before any frame is sent. · why: MCP drivers branch on the one instance-unreachable error and its closed detail; in the MCP process the result is the only record of the cause.
  relied · architecture · fct-vwfyd52k@1 → decision/dialog-hook-fails-open-on-server-check · name: Dialog hook fails open on a failed server check · rule: A dialog hook whose server check fails sends no hook.dialog and exits 0 at once with no decision body, so the dialog renders for the human. · why: The fail-open hook contract and the server-identity negative tests rely on it, so a process that is not the recorded wrapper never answers a dialog.
  relied · architecture · fct-4qhayrrs@1 → decision/hook-without-trusted-instance-exits-silently · name: Hook with no trusted instance exits 0 untouched · rule: A hook whose instance check refuses or finds no wrapped instance exits 0 before reading config or stdin, and writes no line, frame or file. · why: The fail-open hook contract and the hook strict-modes tests rely on it, and the check must come before the log open that narrows the home.
  relied · architecture · fct-vag5jbae@1 → decision/verify-refuses-untrusted-home-or-stamps · name: Verify refuses an untrusted home tree or stamps · rule: A strict-modes refusal of the home tree, before any probe, or of the stamps, again at the write, makes viola verify exit 1 with the home pair and leaves the stamps neither read nor written. · why: Run's version gate trusts the stamps verify writes, and the verify window test expects the widened stamps byte-equal with strict-modes-failed on the line.
  relied · architecture · fct-e4yo5lcc@1 → decision/no-live-endpoint-dead-or-unwrapped · name: No live endpoint reads instance-dead or unwrapped-name · rule: Without a live endpoint, whether the wrapper is stale, gone or replayed, a client reads an existing instance directory as instance-dead and anything else as an unwrapped name. · why: The CLI frame verbs and the MCP frame tools take their exit 21 detail and hint from it, and the cross exit-cause tests pin both words.
  relied · architecture · fct-5kgumnrt@1 → decision/revive-reads-snapshot-once · name: Revive reads the snapshot once · rule: A revive reads the snapshot once through the replay for both its holder check and its recorded directory, so a replayed snapshot makes one replay and logs one state-recovered line. · why: The cwd-missing revive test expects one replay and one state-recovered line in the role file.
  relied · architecture · fct-mefdhndu@1 → decision/socket-dir-refused-covers-create-and-check · name: Socket-dir create failure and refusal read the same · rule: A failed socket-directory create and a refused check both refuse the start as socket-dir-refused with exit 1. A failed bind does not: it reads squatted-name when another process holds the endpoint and internal-error otherwise. · why: Machine readers branch on the socket-dir-refused start document and exit line, and the cross exit-cause test pins it at exit 1 for a start.
  relied · architecture · fct-5ijnay4v@1 → decision/start-pins-the-bytes-it-checked · name: Start pins the very bytes it checked · rule: A start reads its own exe once, keys the home-tree check by those bytes and pins those same bytes, so the copy it runs is the copy whose folders it checked. · why: The strict-modes check of the pinned copy's bin and plugin folders and the pinned-hash-mismatch refusal both assume no second read of the exe.
  relied · architecture · fct-nbf2wdvs@2 → decision/verify-run-e-resumes-run-b-first-session · name: Verify's Run E resumes Run B's first session · rule: Verify's Run E resumes the session id taken from Run B's first SessionStart, only when that id has a session id's shape; with no such id Run E starts no child, so Run E depends on Run B running before it. · why: The resume ledger row checks Run E's first SessionStart for source resume and that same id, and the five runs execute in order A to E.
  relied · architecture · fct-2oafusjr@1 → decision/hook-missing-instance-dir-exits-silently · name: Hook with a missing instance folder exits 0 silently · rule: A hook whose VIOLA_DIR names an instance folder that does not exist fails the canonical-directory step, exits 0 and writes nothing: no role file, no stdin read, no frame and no event line. · why: Claude Code runs the plugin's hooks in sessions viola does not wrap; the hook fail-open contract and its no-instance-directory test rely on it.
  relied · architecture · fct-sbmskf35@2 → decision/path-under-a-file-is-a-strict-modes-refusal · name: A path under a regular file is a strict-modes refusal · rule: A trusted path whose stat fails for any reason other than absence, as under a regular file, is refused as unreadable: verify exits 1 with the home's pair and the frame verbs exit 21 with the instance's pair. · why: Only an absent path may be skipped by the check; verify's regular-file test pins the exit code and the pair.
  relied · architecture · fct-flit34us@1 → decision/cleanup-reads-refused-or-missing-socket-gone · name: Cleanup reads a refused or missing socket as gone · rule: Harness cleanup reads the endpoint as gone when a connect is refused or the path is absent, and removes no leftover socket file, so cleanup reads ok after a force-killed wrapper. · why: A killed wrapper removes nothing and the next bind takes its leftover; the force-killed-wrapper lifecycle test asserts ok and the file still standing.
  relied · architecture · fct-w6vpbrjt@2 → decision/root-tests-freeze-wrappers-with-one-guard · name: Root tests freeze a wrapper through one SIGSTOP guard · rule: Root tests make a wrapper read stale by stopping it with SIGSTOP through one shared guard, which waits until the process state reads stopped and sends SIGCONT on drop. · why: The list-row and instance-state tests rely on it for a stale wrapper whose pid and start time still match while its heartbeat ages.
  relied · architecture · fct-vp2lxvsd@2 → decision/clients-read-server-pid-euid-from-peer-creds · name: Clients read server pid and euid from peer credentials · rule: Both channel clients, sync and async, read the serving process's pid and effective uid from the connected socket's peer credentials, the identity each frame-writing client checks before its first frame. · why: The CLI, hook and MCP server checks admit the euid and match the pid against the snapshot's recorded pid ahead of any frame.
  relied · architecture · fct-e6lqqdby@1 → decision/server-start-time-read-through-sysinfo · name: Server start time is read from the OS through sysinfo · rule: The server check reads the serving pid's start time through sysinfo, with a zombie reading as gone, and matches it in whole seconds against the snapshot's recorded start time before the first frame. · why: The CLI, hook and MCP clients all verify the server this way, so a reused pid with another start time reads as a mismatch.
  relied · architecture · fct-ekks4dfz@1 → decision/stamps-writer-checks-before-and-under-lock · name: Stamps writer checks strict modes before and under its lock · rule: The stamps writer runs the strict-modes check of the ledger folder and the stamps file before creating the folder and again under its lock, and a refusal reads no stamps, never runs the update and writes none. · why: Verify stamps the home through it and run trusts those stamps; a path another user could write must not be read or rewritten.
  relied · architecture · fct-fr2pme2q@1 → decision/link-set-changes-only-on-link-calls · name: Link set changes only on implicit links and link calls · rule: The wrapper's link set changes only on an implicit link from a send or an answer and on a served link or unlink call, with each change appended to the instance log first and then mirrored to the snapshot. · why: Restoring the set at start replays only link and unlink lines, so the snapshot and the log agree only while no other path changes the set.
  relied · architecture · fct-a6y4blve@2 → decision/frame-client-check-order · name: Frame-writing clients run their checks in one order · rule: A CLI or MCP frame call runs the strict-modes check, the liveness pre-check, the socket-directory check, the peer euid decision and then the server identity check, in that order; the hook keeps the same order without the liveness pre-check. · why: Each refusal maps to its own exit-21 cause, and no check may open the home or write a frame before the ones ahead of it pass.
  relied · test-plan · fct-njspb5nv@3 → decision/child-exit-read-on-the-process-handle · name: Child exit is read on the process handle, never on EOF · rule: The pump detects the child's exit by polling the process handle and never by output EOF, then drains the output still in flight for a bounded time. · why: The wrapper's exit code relies on it, and the pump tests use a reader that never ends so only the handle can end the pump.
  relied · test-plan · fct-pb23i77m@1 → decision/harness-boot-creates-stamps-and-starts · name: Harness boot makes the home, stamps it and starts wrappers · rule: The harness boot takes a fresh home under a new session folder, stamps it through viola verify unless asked to leave it unstamped, then starts one wrapper per instance under a supervisor and waits until each is ready. · why: E2e tests and agent sessions rely on a stamped, running home before they seed their own data.
  relied · test-plan · fct-wok64pug@1 → decision/cleanup-removes-home-record-and-supervisor · name: Cleanup removes the home, session record and supervisor · rule: Per session, harness cleanup stops the supervisor and kills it and the recorded processes if needed, removes the session home unless homes are kept, and drops the session record. · why: Boot refuses a session id whose record exists, and cleanup of all sessions finds sessions only by their records.
  relied · test-plan · fct-7tiauhj7@1 → decision/error-body-check-reports-place-and-class · name: Error-body check reports place and class, never bytes · rule: The error-body check reads a hit of a planted value as class planted, ahead of absolute-path, caused-by and serde-path, and its failure names the place and the class and never the bytes it read. · why: A failing witness must print no canary into JUnit, which the secret scan reads.
  relied · obs-plan · fct-xpt2kzwy@1 → decision/panic-before-role-file-leaves-no-record · name: A panic before the role file opens leaves no record · rule: A panic before any role file is opened, in argument parsing or the home and instance checks, leaves no log record, because the panic hook has no sink yet and returns without writing. · why: Strict-modes refusals rely on nothing being created under the home before their check; the panic hook set first keeps the default panic text off stderr.
  relied · obs-plan · fct-bprhan45@2 → decision/refused-diagnostics-path-is-a-failed-open · name: A refused diagnostics path is a failed open · rule: A refused diagnostics folder or file is exactly a failed open: the role file does not open, and a detail line is dropped with nothing written and nothing on stderr. · why: The hook carries on with no subscriber, the other roles end as an internal error, and no line is ever written through a widened folder or a symlink.
  relied · obs-plan · fct-dujo6hv4@1 → decision/pre-log-strict-refusals-write-no-line · name: Strict-modes refusals before the log opens write no line · rule: Refusals of the instance and home-tree checks that run before a role file opens write no line in any log and open nothing under the home. · why: Opening a log narrows the home and would erase the refused state; the strict-modes negative tests assert that no diagnostics folder appears.
  relied · obs-plan · fct-25dvifnk@1 → decision/mcp-unreachable-carries-one-cause-word · name: Each MCP unreachable result carries one cause word · rule: Each MCP unreachable outcome (a refused check, a dead or unwrapped instance, a lost endpoint, an unverified server or a refused socket directory) carries exactly one UnreachableCause word as its detail. · why: The MCP process writes no exit line for it, so the result's cause word is the cause's only record for the calling agent.
  relied · obs-plan · fct-wmupioom@1 → decision/not-a-directory-refusal-message-fixed · name: NotADirectory refusal message is fixed and pathless · rule: The NotADirectory strict-modes refusal reads the fixed message "a trusted state path is not a directory" and names no path. · why: The security plan's fixed-message rule keeps paths out of error text, and the fixed-messages unit test pins every refusal's text.
  relied · obs-plan · fct-4gke2rkl@1 → decision/clients-screen-peer-replies-to-code-words · name: Clients screen peer replies to code words and a fixed fault · rule: The CLI and MCP clients screen a peer reply before using it: a refusal and its detail must be code words, a wrapper fault keeps only its code, a fixed message and closed data, and the screening adds no span and no line. · why: No path or upstream text may cross from a wrapper into a document, a tool result or a log.
  relied · obs-plan · fct-474ly2yg@1 → decision/conn-in-params-joins-cross-process-logs · name: Conn in request params joins logs across processes · rule: Every channel client request carries its conn label in params, and the server logs that conn on its lines (or its own srv label when the conn is missing or malformed) without handing it to the wrapper's answer. · why: Joining a client's request line to the wrapper's handling lines across processes relies on the shared label.
  relied · obs-plan · fct-rar73ens@1 → decision/flat-log-lines-and-one-level-gate · name: Log lines are flat and Targets is the only level gate · rule: The subscriber flattens event fields to the top level of each line, and the Targets filter is the only configurable level gate; its level comes from the home's diagnostics level and never from the environment. · why: The diagnostics schema and the log readers read event fields as top-level keys, and only the viola crates' targets pass the gate.

architecture: decision 506 · contract 481 · absence 15 · name 58 · overview 2 · 1062 row(s)
security-plan: decision 446 · contract 168 · absence 21 · name 0 · overview 2 · 637 row(s)
design-system: decision 94 · contract 231 · absence 3 · name 15 · overview 1 · 344 row(s)
layout-templates: decision 51 · contract 173 · absence 2 · name 0 · overview 0 · 226 row(s)
test-plan: decision 742 · contract 632 · absence 5 · name 0 · overview 0 · 1379 row(s)
obs-plan: decision 596 · contract 463 · absence 17 · name 0 · overview 1 · 1077 row(s)
a11y-plan: decision 131 · contract 110 · absence 5 · name 0 · overview 0 · 246 row(s)
rows 4971 of 7 master(s)

no row · design-system · decision/two-hues-one-role-each · The second hue is the web page's handoff blue, and the web palette leaves the product in 0.2.0; amber keeps its own row.
no row · security-plan · decision/budget-json-checked-read · The budget gate and its budget.json reading leave the product in 0.2.0 (census row F-08).
no row · architecture · decision/percent-newtype · Percent's only users are the budget-gate thresholds, and the budget gate leaves the product in 0.2.0.
no row · architecture · decision/no-env-config-channel · a duplicate of decision/no-environment-configuration
no row · architecture · decision/budget-gate-reads-hook-written-file · The budget gate and its budget.json reading leave the product in 0.2.0 (census row F-08).
refused rows: 2 — a row the fix round refused lands nothing, under any fate; its records are answered by its line alone where no landed row names them
refused row · security-plan · decision/actions-pinned-by-sha · [ROW_FORM] taken — another row stands under this handle · fct-dj5p5k3a@1 named by decision/checkout-no-persisted-credentials · decision/actions-pinned-by-sha
refused row · test-plan · decision/snapshot-replay-from-log · [ROW_FORM] taken — another row stands under this handle · fct-3svwp7h4@1 named by decision/links-survive-wrapper-restart · decision/replay-writes-no-file · decision/replayed-snapshot-has-no-endpoint · decision/collision-check-over-replay-reads-beat · decision/list-reads-live-through-replay · contract/event-log/start-wheel-record · decision/snapshot-replay-from-log
route owed · layout-templates · decision/shared-captions-fixed · The planned fleet console (0.2.0, Epoch 6) reuses the six board captions, their words and slot order (src/human/board/rows.rs:42).
route owed · a11y-plan · decision/refusal-ends-with-hint · Every refusal reason prints a hint, unverified-cli and answer's other not-delivered details included (src/cmd/send.rs:85 · src/human/mirror.rs:65-67).
route owed · security-plan · decision/fixture-scrub-and-refusal · The email check runs on every kind of committed fixture and at the record, not on screens alone (crates/viola-agent-claude/src/ledger.rs:1327-1334).
route owed · security-plan · decision/exact-third-party-pins · The chunk that closes V182 settles rmcp's version (Cargo.toml:223, held to >=3.4.1, <3.5), and its wrap writes the rmcp exception in the form that proves right.
route owed · security-plan · decision/cli-names-through-name-check · run parses its name through the shared parse_name rather than its own copy of send's (src/cmd/run.rs:50).
route owed · security-plan · decision/strict-modes-check · The trusted-state checks refuse a symbolic link too, as the diagnostics and socket-directory checks do: check_instance, check_stamps, check_path and check_read stop following one (crates/viola-state/src/strict.rs:240).
route owed · architecture · decision/no-tmux-or-shell-dependency · The status-line shell-out leaves (crates/viola-agent-claude/src/statusline.rs:124): 0.2.0's entry «Status-line interception removed» (census F-09), already on the route.
route owed · test-plan · decision/pty-tested-on-real-openpty · The needless key at tests/tui_pty_seam.rs:64 goes; the size receipt follows a resize on its own.
route owed · test-plan · decision/fixture-hygiene-walk · The email check runs on the hook, agents and statusline fixture walks too, not only on the Mcp and Screen walks (tests/contract_fixture_hygiene.rs:72) — one work with security-plan · decision/fixture-scrub-and-refusal.
route owed · test-plan · decision/fd-test-reads-live-child-table · The self-dump fallback goes and the test always reads /proc/<pid>/fd, Viola being Linux-only (tests/tui_channel_fds.rs:25).
route owed · test-plan · decision/no-fd-count-assertion · The fds.len() <= 5 count assertion goes (tests/tui_channel_fds.rs:34-37); the /proc table assertions stay.
route owed · test-plan · decision/fixtures-walked-for-hygiene · Emails are walked for in every kind of committed fixture, not only MCP exchanges and screens (tests/support/hygiene.rs:10) — one work with security-plan · decision/fixture-scrub-and-refusal.
route owed · obs-plan · decision/payload-types-derive-redact · The hook and MCP payload types derive veil Redact too; today only the channel Request does (crates/viola-channel/src/frame.rs:51).
route owed · obs-plan · decision/private-diagnostics-files · The diagnostics open refuses a symlink and narrows the mode on the open file, closing the race after the earlier lstat (crates/viola-state/src/fs.rs:44-51).
equal lead · design-system · decision · «no hint names viola release» · decision/no-hint-names-release · decision/no-hint-names-viola-release
twin section · design-system · ## Motion (calibrated to the base expression level `0.2`) · motion-calibrated-to-the-base-expression-level-0-2 · motion
twin section · design-system · ## Surface: cli · surface-cli · surface
equal lead · a11y-plan · decision · «linear static cli output» · decision/linear-static-output · decision/linear-static-cli-output
equal lead · a11y-plan · decision · «fixed cli strings pinned by literal» · decision/fixed-cli-strings-pinned · decision/fixed-cli-strings-pinned-by-literal
equal lead · a11y-plan · contract · «cli exit codes · 2» · contract/cli-usage-error/exit-2 · contract/cli-exit-codes/2
twin section · a11y-plan · ## 1. A11y Scope Summary · 1-a11y-scope-summary · a11y-scope-summary
twin section · a11y-plan · ## 4. ARIA Patterns & Roles · 4-aria-patterns-roles · aria-patterns-roles
twin section · a11y-plan · ## 6. Visual Design Verification · 6-visual-design-verification · visual-design-verification
twin section · a11y-plan · ## 8. Cognitive Accessibility · 8-cognitive-accessibility · cognitive-accessibility
equal lead · security-plan · decision · «mcp tool list is an affordance» · decision/mcp-tool-list-affordance · decision/mcp-tool-list-not-enforcement
equal lead · security-plan · decision · «hook security fields in closed enums» · decision/hook-security-fields-closed · decision/hook-security-fields-closed-enums
equal lead · security-plan · decision · «tracing-subscriber version floor» · decision/tracing-subscriber-floor · decision/tracing-subscriber-version-floor
twin section · security-plan · ## Bootstrap phases (derive for route / setup-project) · bootstrap-phases-derive-for-route-setup-project · bootstrap-phases
twin section · security-plan · ## Security Anti-Patterns (NEVER do these) · security-anti-patterns-never-do-these · security-anti-patterns
equal lead · architecture · decision · «interprocess local sockets behind the channel seam» · decision/interprocess-local-socket-channel · decision/interprocess-local-socket-ipc
equal lead · architecture · contract · «mcp tool results · refusal» · contract/mcp-tool-result/refusal · contract/mcp-tool-results/refusal
equal lead · architecture · decision · «exec-form command hooks» · decision/exec-form-hooks · decision/exec-form-command-hooks
equal lead · architecture · decision · «hooks and mcp never search path» · decision/no-path-lookup · decision/hooks-and-mcp-never-search-path
equal lead · architecture · decision · «human text by default, --json for agents» · decision/cli-output-conventions · decision/cli-human-text-json-exit-codes
equal lead · architecture · decision · «environment strip reads names only» · decision/env-strip-reads-names-only · decision/env-strip-names-only
equal lead · architecture · decision · «nothing published to crates.io» · decision/no-crates-io-publish · decision/no-crates-io-publishing
equal lead · architecture · contract · «viola verify process-exit detail · program-not-found» · contract/verify-process-exit/program-not-found · contract/verify-exit-detail/program-not-found
equal lead · architecture · contract · «viola verify process-exit detail · pinned-hash-mismatch» · contract/verify-process-exit/pinned-hash-mismatch · contract/verify-exit-detail/pinned-hash-mismatch
equal lead · architecture · contract · «viola verify process-exit detail · version-unreadable» · contract/verify-process-exit/version-unreadable · contract/verify-exit-detail/version-unreadable
equal lead · architecture · contract · «viola verify process-exit detail · strict-modes-failed» · contract/verify-process-exit/strict-modes-failed · contract/verify-exit-detail/strict-modes-failed
equal lead · architecture · contract · «viola verify process-exit detail · unclean-fixture» · contract/verify-process-exit/unclean-fixture · contract/verify-exit-detail/unclean-fixture
equal lead · architecture · contract · «viola verify process-exit detail · interrupted» · contract/verify-process-exit/interrupted · contract/verify-exit-detail/interrupted
equal lead · architecture · contract · «viola verify exit codes · 1» · contract/viola-verify-exit-codes/1 · contract/viola-verify-exit-codes/1-relied-row-failed
equal lead · test-plan · decision · «no orphan modules» · decision/cargo-modules-orphans-gate · decision/no-orphan-modules
equal lead · test-plan · decision · «each exit cause has its own hint» · decision/hint-names-its-cause · decision/each-exit-cause-own-hint
equal lead · test-plan · decision · «mutation testing outside ci» · decision/mutation-not-a-ci-gate · decision/mutation-testing-off-ci
equal lead · test-plan · decision · «perf verdict from the harness gate» · decision/perf-verdict-from-harness · decision/perf-verdict-from-harness-gate
equal lead · test-plan · decision · «least workflow permissions» · decision/workflow-least-permissions · decision/least-workflow-permissions
equal lead · test-plan · decision · «single writers of snapshot and stamps» · decision/single-writers · decision/single-writer-state-files
equal lead · test-plan · decision · «stamps written only by verify» · decision/stamps-only-from-verify · decision/stamps-written-only-by-verify
equal lead · test-plan · contract · «instance-unreachable causes · socket-dir-refused» · contract/instance-unreachable-cause/socket-dir-refused · contract/unreachable-cause/socket-dir-refused
twin section · test-plan · ## 1. Test Scope Summary · 1-test-scope-summary · test-scope-summary
twin section · test-plan · ## 2. Test Strategy · 2-test-strategy · test-strategy
twin section · test-plan · ## 4. Unit Test Strategy · 4-unit-test-strategy · unit-test-strategy
twin section · test-plan · ## 5. Integration Test Strategy · 5-integration-test-strategy · integration-test-strategy
twin section · test-plan · ## 6. E2E Test Strategy · 6-e2e-test-strategy · e2e-test-strategy
twin section · test-plan · ## 7. Test Data & Fixtures · 7-test-data-fixtures · test-data-fixtures
twin section · test-plan · ## 8. Mocking & Stubbing Discipline · 8-mocking-stubbing-discipline · mocking-stubbing-discipline
twin section · test-plan · ## 9. CI Integration · 9-ci-integration · ci-integration
twin section · test-plan · ## 10. Quality Gates & Coverage Targets · 10-quality-gates-coverage-targets · quality-gates-coverage-targets
equal lead · obs-plan · decision · «child exit from the process handle» · decision/child-exit-from-process-handle · decision/child-exit-from-handle-wait
equal lead · obs-plan · decision · «no otel sdk exporter» · decision/no-otel-sdk-exporter · decision/no-otel-exporter
equal lead · obs-plan · decision · «redaction cannot be disabled» · decision/redaction-not-disableable · decision/redaction-cannot-be-disabled
equal lead · obs-plan · decision · «drift reports go to diagnostics only» · decision/drift-reports-to-diagnostics · decision/drift-reports-diagnostics-only
equal lead · obs-plan · decision · «future reporter applies the never-log floor» · decision/future-reporter-never-log-floor · decision/reporter-never-log-floor
equal lead · obs-plan · decision · «veil toggle feature banned» · decision/veil-toggle-feature-banned · decision/veil-toggle-banned
equal lead · obs-plan · decision · «no telemetry from screen content» · decision/no-telemetry-from-screen-content · decision/no-telemetry-from-screen
equal lead · obs-plan · decision · «hook log write failure swallowed» · decision/hook-log-failure-swallowed · decision/hook-log-write-failure-swallowed
equal lead · obs-plan · decision · «mcp rejections carry codes only» · decision/mcp-results-codes-only · decision/mcp-rejections-codes-only
equal lead · obs-plan · decision · «log level never from the environment» · decision/log-level-not-from-env · decision/log-level-never-from-env
equal lead · obs-plan · decision · «no log rotation in v1» · decision/no-log-rotation-v1 · decision/no-log-rotation-in-v1
equal lead · obs-plan · decision · «deterministic event emission» · decision/deterministic-event-emission · decision/deterministic-emission
equal lead · obs-plan · decision · «dialog content never logged» · decision/no-dialog-content-in-logs · decision/dialog-content-never-logged
twin section · obs-plan · ## 1. Obs Scope Summary · 1-obs-scope-summary · obs-scope-summary
twin section · obs-plan · ## 2. Telemetry Strategy · 2-telemetry-strategy · telemetry-strategy
twin section · obs-plan · ## 4. Span / Trace Coverage · 4-span-trace-coverage · span-trace-coverage
twin section · obs-plan · ## 5. Metric Coverage · 5-metric-coverage · metric-coverage
twin section · obs-plan · ## 6. Log Coverage · 6-log-coverage · log-coverage
twin section · obs-plan · ## 7. Error Capture & Reporting · 7-error-capture-reporting · error-capture-reporting
twin section · obs-plan · ## 8. PII Scrubbing & Compliance · 8-pii-scrubbing-compliance · pii-scrubbing-compliance
twin section · obs-plan · ## 9. CI Integration · 9-ci-integration · ci-integration
twin section · obs-plan · ## 10. SLO Invariants & Telemetry Budgets · 10-slo-invariants-telemetry-budgets · slo-invariants-telemetry-budgets
twin section · obs-plan · ## 11. Obs Anti-Patterns (NEVER do these) · 11-obs-anti-patterns-never-do-these · obs-anti-patterns
anchor «Stack and Technologies» · architecture: 48 row(s)
anchor «Inherited Defaults» · architecture: 16 row(s)
anchor «Infrastructure Patterns» · architecture: 77 row(s)
anchor «Anti-Patterns (Universal)» · security-plan: 64 row(s)
anchor «Anti-Patterns (Universal)» · design-system: 17 row(s)
anchor «Anti-Patterns (Universal)» · test-plan: 47 row(s)
anchor «Anti-Patterns (Universal)» · obs-plan: 68 row(s)
anchor «Anti-Patterns (Universal)» · a11y-plan: 15 row(s)
anchor «Threat Model Summary» · security-plan: 101 row(s)
anchor «Project Intent» · architecture: 6 row(s)
anchor «Quality Gates» · test-plan: 54 row(s)
anchor «test-plan §3» · test-plan: 291 row(s)
anchor «obs-plan §3» · obs-plan: 133 row(s)
anchor «a11y-plan §3» · a11y-plan: 18 row(s)
anchor «Surface» · design-system: 264 row(s)
anchor «Surface» · layout-templates: 226 row(s)
anchor «(?i)development style» · architecture: 1 row(s)

the TX: 8098 node(s) · 3593 edge(s) — every `section` and `fact` of the masters and their key documents retired with its text whole, 4971 row(s), one `doc` revision and one sidecar entry a master
