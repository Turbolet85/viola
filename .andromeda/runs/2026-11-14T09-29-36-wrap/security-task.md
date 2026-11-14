@task_prose part=session
The session added a Hyprland seam: a new sync library crate, viola-hyprland, that spawns the compositor client hyprctl directly with a closed argv for four JSON queries (clients, activewindow, workspaces, version) and two focus dispatches, and parses each reply tolerantly into closed types that keep no text field.
The reply read is bounded at MAX_FRAME + 1 bytes and refused whole as oversize. The child gets null stdin, the named env vars removed and a PROVISIONAL 5 s deadline, after which it is killed and reaped. Failures are five closed code words with fixed messages.
Measured live on Hyprland 0.56.2: dispatch runs as Lua, so the plan's dispatch argv was disproved and replaced by hl.dsp.focus forms. The only inserted values are a closed Address newtype or an i32 workspace id.
Added a second test-only recorder (viola-hypr-tap) with a closed per-request string keep-list scrub, a recorded fixture class with its schema and hygiene-walk check, a fake-agent hyprctl personality, a contract test, a fuzz target hyprctl_json, and two new diag-line words: subject hyprctl-probe and parser hyprctl-json.
No product caller yet: the release binary calls none of the seam. No new third-party crate was added. fuzz/Cargo.lock was refreshed after it was found stale.
@task_file n=1 path=crates/viola-hyprland/Cargo.toml
what: the manifest of the new sync seam crate over the Hyprland compositor client, its deps all workspace-pinned, its graph free of tokio
@task_file n=2 path=crates/viola-hyprland/src/lib.rs
what: the seam's closed request set and argv (no shell, closed newtypes only in the dispatch text), the Address validator that refuses and never strips, and the closed error enum with fixed messages
@task_file n=3 path=crates/viola-hyprland/src/reply.rs
what: the tolerant closed parsers of hyprctl replies, bounded at MAX_FRAME, default JSON depth, every reply text dropped at the parse
@task_file n=4 path=crates/viola-hyprland/src/read.rs
what: the bounded external reader of the hyprctl child: direct spawn of a caller-resolved program, stdin null, named env vars removed, stdout capped at MAX_FRAME + 1, a kill-and-reap deadline, the spawn logged codes-only
@task_file n=5 path=crates/viola-hyprland/proptest-regressions/reply.txt
what: the committed property-test regressions of the hyprctl reply parsers
@task_file n=6 path=crates/viola-e2e/src/bin/viola-hypr-tap.rs
what: the test-only fixture recorder for hyprctl replies: a closed per-request string keep-list, every other string replaced by a placeholder, then the fixture scrub and whole refusal, files written owner-only
@task_file n=7 path=src/bin/viola-fake-agent/hyprctl.rs
what: the fake agent's test-only hyprctl personality, replaying recorded fixtures by its file name, under the fake-agent test seam
@task_file n=8 path=schemas/hyprland-fixture.v1.json
what: the schema of the recorded hyprctl fixture envelope
@task_file n=9 path=fixtures/hyprland/0.56.2/ActiveWindow.recorded-1.json
what: a recorded, scrubbed hyprctl fixture (activewindow reply)
@task_file n=10 path=fixtures/hyprland/0.56.2/Clients.recorded-1.json
what: a recorded, scrubbed hyprctl fixture (clients reply)
@task_file n=11 path=fixtures/hyprland/0.56.2/FocusWindow.recorded-1.json
what: a recorded hyprctl fixture (focus-window dispatch reply)
@task_file n=12 path=fixtures/hyprland/0.56.2/Version.recorded-1.json
what: a recorded, scrubbed hyprctl fixture (version reply)
@task_file n=13 path=fixtures/hyprland/0.56.2/Version.absent-1.json
what: a recorded hyprctl fixture of the no-instance reply
@task_file n=14 path=fixtures/hyprland/0.56.2/Workspace.recorded-1.json
what: a recorded hyprctl fixture (workspace dispatch reply)
@task_file n=15 path=fixtures/hyprland/0.56.2/Workspaces.recorded-1.json
what: a recorded, scrubbed hyprctl fixture (workspaces reply)
@task_file n=16 path=tests/contract_hyprland_seam.rs
what: the E2E contract test of the seam: fake replays recorded replies, failures read as closed codes, a canary title reaches no log line and no error
@task_file n=17 path=fuzz/fuzz_targets/hyprctl_json.rs
what: the fuzz target over the hyprctl reply parsers
@task_file n=18 path=fuzz/corpus/hyprctl_json/
what: the synthetic seed corpus of the hyprctl reply fuzz target
@task_file n=19 path=Cargo.toml
what: the root workspace manifest: the new crate as workspace member and dev-dependency, the contract test target under the fake-agent feature
@task_file n=20 path=Cargo.lock
what: the root lockfile, gaining the new workspace crate only
@task_file n=21 path=crates/viola-e2e/Cargo.toml
what: the test-only e2e crate's manifest, gaining the test-only recorder binary
@task_file n=22 path=scripts/sync-crates.txt
what: the list of sync crates each held to its own deny-sync tokio ban, now five
@task_file n=23 path=src/obs.rs
what: the set of viola log targets, now eight
@task_file n=24 path=schemas/diag-line.v1.json
what: the diagnostics line schema: the subject enum (now 8, hyprctl-probe) and the parse-rejected parser enum (now 11, hyprctl-json)
@task_file n=25 path=src/bin/viola-fake-agent.rs
what: the fake agent test seam, now dispatching to its hyprctl personality by file name
@task_file n=26 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: the fake agent's unit tests, gaining the hyprctl replay case
@task_file n=27 path=tests/support/fake.rs
what: the test support that places the fake agent, gaining a hyprctl fake placer
@task_file n=28 path=tests/contract_bounded_readers.rs
what: the bounded-readers census: a new BOUNDARIES row for the hyprctl reply read, the seam joining PRODUCT_ROOTS
@task_file n=29 path=tests/contract_error_bodies.rs
what: the census of every product error writer: the seam joining PRODUCT_ROOTS, its error enum named as not an error body (no surface writes it yet)
@task_file n=30 path=tests/contract_fixture_hygiene.rs
what: the recorded fixture hygiene walk, gaining the Hyprland class checked against the same closed keep-list
@task_file n=31 path=fuzz/Cargo.toml
what: the separate fuzz workspace manifest, gaining the new target and crate
@task_file n=32 path=fuzz/Cargo.lock
what: the fuzz lockfile audited on its own, refreshed from a stale state (the veil edge) and for the new crate
@task_file n=33 path=.gitattributes
what: the repository attributes, gaining a line for the new recorded fixtures
@task_item n=1 part=touches
decision/max-frame-shared-cap, decision/max-frame-cap and decision/reads-capped-at-max-frame: a new external reader (the hyprctl reply) uses take(MAX_FRAME + 1), and an over-cap reply is refused whole as oversize before the exit is read.
@task_item n=2 part=touches
decision/bounded-readers-census, decision/bounded-external-readers and decision/child-output-bounded: a new bounded child-output reader and a new BOUNDARIES row in the census, with the seam joining the product roots (7 to 8).
@task_item n=3 part=touches
decision/upstream-text-is-never-a-command and decision/upstream-text-is-content: the dispatch argv now carries a Lua expression whose only inserted values are a closed Address or an i32. No element is built from reply text and no shell line is built.
@task_item n=4 part=touches
decision/statusline-only-shell-out, decision/single-shell-out and decision/statusline-shell-out: a new external child (hyprctl) is spawned directly, with no shell. The statusline stays the only shell-out, but the rule now covers a new spawn.
@task_item n=5 part=touches
decision/claude-resolved-from-path, decision/child-path-trust, decision/run-resolves-absolute-program and decision/claude-agents-resolved-from-path: the hyprctl read does no PATH lookup itself because the caller resolves the program, and with no product caller yet the resolving rule is owed.
@task_item n=6 part=touches
decision/child-output-tolerant, decision/child-output-tolerant-unknown and decision/claude-agents-read-fail-open: the hyprctl replies get a tolerant closed parse. A key of another type reads as absent and a document of the wrong shape reads as malformed.
@task_item n=7 part=touches
decision/statusline-deadline and other child-deadline rules: a new PROVISIONAL 5 s HYPRCTL_DEADLINE kills and reaps the child, and a deadline kill reads as unreachable.
@task_item n=8 part=touches
decision/fixture-recording-scrubbed, decision/fixture-scrub-and-refusal, decision/fixtures-reviewed-before-commit and decision/fixture-hygiene: a new recorded fixture class with a closed per-request string keep-list ahead of the shared scrub and the whole refusal, checked by the hygiene walk.
@task_item n=9 part=touches
decision/mcp-fixtures-via-test-only-tap and decision/screen-fixture-signature-rows-only: a second test-only recorder (viola-hypr-tap) beside the MCP tap and viola verify --record, so the number of recorders moved from 1 to 2.
@task_item n=10 part=touches
decision/release-build-no-test-binaries and decision/release-build-without-test-features: a new test-only binary in the e2e crate. The release build still carries viola only.
@task_item n=11 part=touches
decision/test-seams-fake-agent-only, decision/test-seams-compiled-out, decision/fake-agent-hook-panic-seam and decision/fake-agent-pump-delay-seam: the fake agent gains a hyprctl personality, chosen by its file name with no option or env var, and the contract test requires the fake-agent feature.
@task_item n=12 part=touches
decision/sync-crate-tokio-ban, decision/sync-crates-tokio-free and decision/no-tokio-in-sync-crates: a fifth sync crate is on the list, under its own tokio ban (count 4 to 5 where a row states it).
@task_item n=13 part=touches
decision/parser-fuzz-scope and decision/scheduled-fuzzing: a new parser surface is in fuzz scope (target hyprctl_json, fuzz targets 6 to 7).
@task_item n=14 part=touches
decision/fuzz-lockfile-own-audit, decision/fuzz-lockfile-audited-separately and decision/fuzz-workspace-exempt-from-root-deny: the fuzz lockfile was found stale (the veil edge). The fuzz replay rewrites it in the tree, so CI's supply-chain job passes a stale lockfile unseen.
@task_item n=15 part=touches
decision/error-messages-interpolate-nothing, decision/fixed-error-display-messages, decision/serde-errors-mapped-to-fixed-message and decision/error-display-no-upstream-text: a new error enum with five closed code words and fixed messages that interpolate nothing.
@task_item n=16 part=touches
decision/every-error-writer-named and decision/error-enum-convention-exceptions: the new error enum is named in the census as not an error body because no verb calls the seam yet.
@task_item n=17 part=touches
decision/serde-json-bounded-depth, decision/json-default-depth-limit and decision/no-serde-unbounded-depth: the hyprctl JSON is parsed at the default depth, and an array past it reads as malformed.
@task_item n=18 part=touches
decision/data-classes, decision/user-content-only-to-diagnostics and decision/never-log-claude-values: window titles, classes and every other reply string are dropped at the parse. They reach no log line and no error display, as the canary test shows.
@task_item n=19 part=touches
decision/nutype-input-validation, decision/refuse-control-characters, decision/refused-text-never-stripped and decision/shared-validators-in-core: a new nutype Address refuses any other byte (a control character included) and never strips. The error words are checked by the core code-word check.
@task_item n=20 part=touches
decision/new-env-read-needs-log-entry: a new seam logs process-start and process-exit with subject hyprctl-probe, and parse-rejected with parser hyprctl-json. viola reads no env var for it.
@task_item n=21 part=touches
decision/parent-identity-env-stripped and decision/hermetic-child-environment: the hyprctl read removes caller-named env vars from its child. The recorder alone removes HYPRLAND_INSTANCE_SIGNATURE for the absent variant.
@task_item n=22 part=touches
decision/test-support-dev-only: the new crate is a dev-dependency of the root package with no product caller, and is a normal dependency only of the test-only e2e crate and the fuzz workspace.
@task_fact n=1 slug=input-validation
fact: The Hyprland seam spawns hyprctl directly from a program path its caller resolves. The seam itself does no PATH lookup and reads no environment variable.
@task_fact n=2 slug=input-validation
fact: The hyprctl reply read is capped at MAX_FRAME + 1 bytes. An over-cap reply is refused whole as oversize before the exit is read, and stderr is drained to MAX_FRAME and dropped.
@task_fact n=3 slug=input-validation
fact: The hyprctl child runs with null stdin, the named env vars removed and a PROVISIONAL 5 s deadline. It is killed and reaped at the deadline, and a deadline kill or non-zero exit reads as unreachable.
@task_fact n=4 slug=security-anti-patterns-never-do-these
fact: hyprctl dispatch text is a Lua expression (Hyprland 0.56.2 runs dispatch as Lua). Its only inserted values are a closed Address (0x plus 1 to 16 lower-case hex digits) or an i32 workspace id, never reply text, and no shell line is built.
@task_fact n=5 slug=data-protection
fact: Hyprland fixtures are recorded only through a test-only tap with a closed per-request string keep-list (addresses and the digit version tag). Every other string at any depth is replaced by a placeholder, and a non-JSON reply is kept only when it is the success text ok. The shared scrub and whole refusal follow, and the hygiene walk checks the same keep-list.
@task_fact n=6 slug=data-protection
fact: The Hyprland reply types hold no text field. Window titles, classes and every other reply string are dropped at the parse and reach no log line and no error.
@task_fact n=7 slug=error-handling
fact: Hyprland seam failures are five closed words (unsupported, unreachable, oversize, malformed, refused) with fixed messages that interpolate nothing.
@task_item n=1 part=untouched
Authentication and authorization (sockets, peer euid checks, strict-modes checks of the home, server verification): the seam opens no socket of its own, reads no viola state and adds no channel method.
@task_item n=2 part=untouched
Channel, MCP tool, wrapper method, CLI exit code, revive and send contracts: no CLI verb, MCP tool or channel frame calls the seam yet.
@task_item n=3 part=untouched
Paste text, dialog free text, typed text and the stdin non-editing list rules: no text is typed into any terminal or dialog.
@task_item n=4 part=untouched
Pinned binary, plugin install, marketplace and statusline-override rows: no pinned copy, plugin file or Claude home write changed.
@task_item n=5 part=untouched
Dependency pins, licence list, GitHub Actions, toolchain pins, rmcp features and advisory floors: no new third-party crate, workflow or toolchain change.
@task_item n=6 part=untouched
Claude child credential strip, identity floor and the claude-child process-start contract items: the claude child's environment handling is unchanged.
@task_item n=7 part=untouched
Network absences (public listener, outbound network calls, remote access, websocket, webhook) and the hosting, secrets and database absences stay true, because hyprctl is a local compositor client.
@task_item n=8 part=untouched
Event log, budget.json, config.json and snapshot contracts and their writers: none were touched.
