@row_add n=1 row=name/crate/viola area=occupied-resources rec=1
class: crate
name: viola
@row_add n=2 row=name/crate/viola-core area=occupied-resources rec=1
class: crate
name: viola-core
@row_add n=3 row=name/crate/viola-pty area=occupied-resources rec=1
class: crate
name: viola-pty
@row_add n=4 row=name/crate/viola-channel area=occupied-resources rec=1
class: crate
name: viola-channel
@row_add n=5 row=name/crate/viola-agent-claude area=occupied-resources rec=1
class: crate
name: viola-agent-claude
@row_add n=6 row=name/crate/viola-state area=occupied-resources rec=1
class: crate
name: viola-state
@row_add n=7 row=name/crate/viola-mcp area=occupied-resources rec=1
class: crate
name: viola-mcp
@row_add n=8 row=name/crate/viola-e2e area=occupied-resources rec=1
class: crate
name: viola-e2e
@row_add n=9 row=decision/fake-agent-feature-on-every-member area=occupied-resources rec=1
name: A fake-agent feature on every member crate
rule: The sync crates, viola-mcp and viola-e2e each declare a code-free fake-agent feature, so a package-scoped fake-agent build of any member resolves.
why: so a package-scoped --features fake-agent build (cargo-mutants) resolves
except: viola-channel's fake-agent feature enables its Tokio client, so its package-scoped mutation build compiles and tests that client.
@row_add n=10 row=decision/viola-channel-tokio-free-by-default area=occupied-resources rec=1
name: viola-channel tokio-free without its feature
rule: The viola-channel dependency graph without its tokio feature carries no Tokio; only a crate that enables the feature, viola-mcp, gets the Tokio client.
guard: channel-tokio-free.sh
@row_add n=11 row=decision/test-support-never-in-product area=occupied-resources rec=2
name: Test-support feature kept out of product builds
rule: The viola-channel test-support feature is enabled only by the root package's dev-dependencies, so no product build carries it.
guard: release-check.sh refuses a release artifact built with it
@row_add n=12 row=name/crate/viola-fuzz area=occupied-resources rec=3,46
class: crate
name: viola-fuzz
@row_none n=1 rec=4 type=S
@row_add n=13 row=name/other/viola-name area=occupied-resources rec=5
class: other
name: VIOLA_NAME
@row_add n=14 row=contract/child-environment/viola-name area=occupied-resources rec=5
boundary: The wrapped child's environment
item: VIOLA_NAME
promise: Set by viola run to the instance's ViolaName; when it is absent, names no existing instance directory, or the instance fails its strict-modes check, every hook event and dialog path exits 0 silently, reading no config, no stdin and writing nothing.
@row_add n=15 row=name/other/viola-dir area=occupied-resources rec=6
class: other
name: VIOLA_DIR
@row_add n=16 row=contract/child-environment/viola-dir area=occupied-resources rec=6
boundary: The wrapped child's environment
item: VIOLA_DIR
promise: Set by viola run to the absolute path of the instance's state directory; hook, mcp and every CLI verb inside the session take the viola home as its grandparent, so a session started with --home stays in that home.
@row_add n=17 row=name/other/viola-bin area=occupied-resources rec=7
class: other
name: VIOLA_BIN
@row_add n=18 row=contract/child-environment/viola-bin area=occupied-resources rec=7
boundary: The wrapped child's environment
item: VIOLA_BIN
promise: Set by viola run to the absolute path of the pinned copy of the binary, so skills and Bash tool calls in the session can call that same copy explicitly; viola itself never reads it.
@row_add n=19 row=contract/child-environment/path area=occupied-resources rec=8
boundary: The wrapped child's environment
item: PATH
promise: Prefixed by viola run with the pinned copy's folder.
@row_add n=20 row=decision/claude-env-stripped-from-child area=occupied-resources rec=9
name: Inherited CLAUDE names stripped from the child
rule: Every inherited CLAUDE* environment name, matched exactly, is removed from the wrapped child's environment, and the identity-floor names are removed whatever the persistent environment says.
except: A name listed in config.json claude_env_keep is kept, unless it is an identity-floor name.
@row_add n=21 row=decision/env-strip-reads-names-only area=occupied-resources rec=9
name: Environment strip reads names only
rule: The child-environment strip and its logs handle variable names only and never read a value.
@row_add n=22 row=decision/plugin-env-vars-unused area=occupied-resources rec=10
name: Plugin-provided environment variables unused
rule: viola never uses CLAUDE_PLUGIN_ROOT or CLAUDE_PLUGIN_DATA to locate its binary, and reads none of those or CLAUDE_ENV_FILE and CLAUDE_PROJECT_DIR.
@row_add n=23 row=name/other/agent-run-chunk-base area=occupied-resources rec=11
class: other
name: AGENT_RUN_CHUNK_BASE
@row_add n=24 row=name/other/agent-run-keep-homes area=occupied-resources rec=11
class: other
name: AGENT_RUN_KEEP_HOMES
@row_add n=25 row=name/other/agent-run-keep-failed area=occupied-resources rec=11
class: other
name: AGENT_RUN_KEEP_FAILED
@row_add n=26 row=decision/no-live-claude-in-ci area=occupied-resources rec=11
name: No live claude in CI
rule: The harness's run --local-live refuses before any build whenever CI is present, and viola itself never reads the harness variables.
why: so the real claude never runs in CI
@row_add n=27 row=name/other/fake-agent-pump-delay-ms area=occupied-resources rec=12
class: other
name: FAKE_AGENT_PUMP_DELAY_MS
@row_add n=28 row=name/other/fake-agent-hook-panic area=occupied-resources rec=12
class: other
name: FAKE_AGENT_HOOK_PANIC
@row_add n=29 row=decision/test-seams-not-configuration area=occupied-resources rec=12
name: Test seams are not a configuration channel
rule: Test-seam environment variables are read only by a viola built with the test-only fake-agent feature, never by a release build, and a seam disables no control and widens no redaction.
@row_add n=30 row=name/other/mcp-serve-test-child area=occupied-resources rec=13
class: other
name: MCP_SERVE_TEST_CHILD
@row_add n=31 row=decision/viola-env-namespace-product-only area=occupied-resources rec=13
name: VIOLA_ environment namespace is product plumbing
rule: Test-only environment markers stay outside the VIOLA_* namespace, which is reserved for product plumbing.
@row_add n=32 row=contract/human-output-colour-env/no-color area=occupied-resources rec=14
boundary: Human output colour environment
item: NO_COLOR
promise: Present, an empty value included, switches colour off.
@row_add n=33 row=contract/human-output-colour-env/term area=occupied-resources rec=14
boundary: Human output colour environment
item: TERM
promise: Exactly dumb switches colour off; a value containing 256color picks 256 colours.
@row_add n=34 row=contract/human-output-colour-env/colorterm area=occupied-resources rec=14
boundary: Human output colour environment
item: COLORTERM
promise: truecolor or 24bit picks truecolor.
@row_add n=35 row=decision/colour-env-decides-only-sgr area=occupied-resources rec=14
name: Colour variables decide only SGR output
rule: Every viola build's human output reads NO_COLOR, TERM and COLORTERM, never sets them, only compares them, never echoes them, and they decide only whether and how deep SGR bytes are written; COLUMNS, CLICOLOR, CLICOLOR_FORCE and CI are not read.
why: ratified: the colour variables are to be read
@row_none n=2 rec=15 type=S
@row_add n=36 row=contract/config-json/v area=occupied-resources rec=16,43
boundary: config.json
item: v
promise: Carries the constant 1.
@row_add n=37 row=contract/config-json/five-hour area=occupied-resources rec=16,43
boundary: config.json
item: five_hour
promise: The five-hour budget threshold, a whole percentage 0-100, default 90.
@row_add n=38 row=contract/config-json/seven-day area=occupied-resources rec=16,43
boundary: config.json
item: seven_day
promise: The seven-day budget threshold, a whole percentage 0-100, default 85.
@row_add n=39 row=contract/config-json/diagnostics-level area=occupied-resources rec=16,43
boundary: config.json
item: diagnostics_level
promise: One of info or debug.
@row_add n=40 row=contract/config-json/claude-env-keep area=occupied-resources rec=16,43
boundary: config.json
item: claude_env_keep
promise: The pass-list of CLAUDE names kept in the child's environment: at most 32 strings matching ^CLAUDE[A-Z0-9_]*$.
@row_add n=41 row=contract/config-json/dialog-hold-s area=occupied-resources rec=16,43
boundary: config.json
item: dialog_hold_s
promise: The dialog hold in seconds, an integer 60-3600, default 60.
@row_add n=42 row=decision/config-falls-back-to-defaults area=occupied-resources rec=16
name: Unusable configuration falls back to defaults
rule: config.json is read under the frame cap, and an unusable file or value falls back to the default with a parse-rejected config-json line.
why: so no value switches the budget gate off or makes the dialog hold zero or unbounded
@row_add n=43 row=decision/pinned-binary-copy area=occupied-resources rec=17
name: Pinned copy of the binary
rule: viola run places a content-hashed pinned copy of its binary once, never clobbers or replaces it, re-hashes an existing copy on every start, and a hash mismatch refuses the start with exit 1 pinned-hash-mismatch, leaving the file as found.
@row_add n=44 row=decision/no-deletion-of-pinned-copies area=occupied-resources rec=17
name: Pinned copies and plugin folders never deleted
rule: v1 never deletes pinned copies or plugin folders, and each instance snapshot records the pinned path it uses.
why: so any later cleanup can skip copies that a live or stale instance still calls
@row_add n=45 row=decision/strict-modes-check-before-log area=occupied-resources rec=18,22
name: Strict-modes check before any log
rule: Every command that uses the viola home or an instance's files checks their modes, a folder before what it holds, before opening any log; a refused check exits 1 with the home-state or instance-state unable and hint pair, writes no line and starts no child.
why: opening a log narrows an existing home
@row_add n=46 row=decision/run-pins-checked-bytes area=occupied-resources rec=18
name: Run pins the bytes it checked
rule: viola run pins the very executable bytes its start check examined.
@row_none n=3 rec=19 type=5
@row_none n=4 rec=20 type=4
relied: The plugin marketplace tree is rewritten whole on every plugin install, any other entry is left as found, and viola never reads the tree back.
@row_add n=47 row=decision/process-logs-hold-codes-only area=occupied-resources rec=21
name: Home-level process logs hold codes only
rule: Home-level per-role process logs carry codes only, one line per write; content-bearing detail stays in the instance detail files.
@row_add n=48 row=decision/diagnostics-open-checks-modes area=occupied-resources rec=21
name: Diagnostics files checked before open
rule: Every open of a process-log or detail file first checks the existing diagnostics folder and file: a symbolic link is refused and both must meet the read rule; a refusal is a failed open, ending the role writer at a sink or dropping the detail line.
@row_add n=49 row=contract/diagnostics-lines/process-log-line area=occupied-resources rec=21,44
boundary: Diagnostics lines
item: home-level process-log line
promise: Format owned by obs and fixed by the default-deny, self-contained schema diag-line.v1.
@row_add n=50 row=contract/diagnostics-lines/detail-line area=occupied-resources rec=21,44
boundary: Diagnostics lines
item: instance detail line
promise: Format owned by obs and fixed by the default-deny, self-contained schema diag-detail.v1.
@row_add n=51 row=decision/events-log-never-rotated area=occupied-resources rec=22
name: Events log never rotated
rule: v1 never rotates or truncates an instance's events.ndjson; any future size bound must keep existing offsets valid.
why: wait's after and send's cursor are byte offsets into it
@row_add n=52 row=contract/events-ndjson/byte-offsets area=occupied-resources rec=22
boundary: events.ndjson
item: byte offsets
promise: The after of wait and the cursor of send are byte offsets into the instance's events.ndjson.
@row_add n=53 row=decision/shared-budget-reading area=occupied-resources rec=23
name: One shared budget reading
rule: The newest rate_limits reading across wrapped sessions is kept timestamped, last writer wins, replaced whole under a lock, written only by hook statusline; every failed read of it counts as no reading.
@row_add n=54 row=decision/statusline-source-precedence area=occupied-resources rec=24,25
name: Source of the user's statusline command
rule: When the home holds statusline-source.json it is the source of the user's statusline command; otherwise, only in the default home, the statusLine command of the user's Claude settings is taken; viola writes neither and reads both under the frame bound.
@row_add n=55 row=decision/verify-sole-stamps-writer area=occupied-resources rec=26
name: verify is the one stamps writer
rule: viola verify is the one writer of the ledger stamps; run's version gate reads them behind the strict-modes check, and a refusal records cli_verified false.
@row_add n=56 row=decision/verify-probe-dirs-drop-guarded area=occupied-resources rec=27,28
name: verify probe dirs removed by drop guard
rule: Every probe dir viola verify creates is private and removed whole when verify passes, fails or is stopped by Ctrl-C or TERM, only the dirs that run created, never by a pattern or a listing.
why: its captures are transient, content-bearing raw hook payloads
@row_none n=5 rec=29 type=5
@row_add n=57 row=decision/test-evidence-survives-kill area=occupied-resources rec=30,31
name: Test evidence survives a killed test
rule: Real-process test rigs stream step evidence to report files outside the viola home, kept when the test panics or is killed and removed when it passes; viola never reads or writes them.
why: so a test the runner kills keeps its evidence
@row_add n=58 row=decision/chaos-home-outside-gates area=occupied-resources rec=32
name: Chaos panic home outside the obs gates
rule: The forced vt100 feed-panic test uses a home outside the e2e home tree, so its deliberate panic line falls outside the zero-panic gate, schema conformance and the secret scan; the test asserts the panic and parse-rejected lines itself.
why: the one named carve-out to the test-data rule
guard: chaos_feed_panic
@row_none n=6 rec=33 type=5
@row_none n=7 rec=34 type=S
@row_add n=59 row=decision/recorded-fixtures-scrubbed area=occupied-resources rec=35
name: Recorded fixtures scrubbed and hygiene-walked
rule: Recorded fixtures are written only when no verify row fails, each is scrubbed and hygiene-walked, and an unclean recording is refused whole, naming the file and the check code, never the content.
@row_none n=8 rec=36 type=5
@row_none n=9 rec=37 type=5
@row_none n=10 rec=38 type=5
@row_none n=11 rec=39 type=5
@row_none n=12 rec=40 type=5
@row_none n=13 rec=41 type=5
@row_none n=14 rec=42 type=5
@row_add n=60 row=decision/config-keys-match-schema area=occupied-resources rec=43
name: Config keys equal the config schema
rule: The set of configuration keys the product knows equals the set of properties of the tolerant config.json schema, which admits additional properties.
guard: config_keys_are_the_config_schema_properties
@row_none n=15 rec=45 type=5
@row_add n=61 row=decision/secret-scan-skips-only-chunk-diff area=occupied-resources rec=47
name: Secret scan skips only the chunk diff
rule: secret-scan covers every file except the harness's chunk.diff, skipped by exact path, and the harness upload excludes that file.
@row_none n=16 rec=48 type=5
@row_add n=62 row=decision/test-homes-cleaned-by-owner area=occupied-resources rec=49
name: Test homes cleaned by owner process
rule: Harness and test homes are kept in CI for the obs gates, and a new test home removes only the dirs whose recorded owner process is gone, never by age or name.
@row_none n=17 rec=50 type=5
@row_add n=63 row=decision/mutants-own-target-dir area=occupied-resources rec=51
name: Mutation runs use their own relative target dir
rule: Mutation testing builds in its own target dir, passed to cargo-mutants as a relative path, so each copied tree builds every test binary in its own copy.
why: a copied default target keeps the original tree's binary paths, so root integration tests would drive the unmutated binary
@row_none n=18 rec=52 type=5
@row_none n=19 rec=53 type=5
@row_none n=20 rec=54 type=5
@row_none n=21 rec=55 type=6
