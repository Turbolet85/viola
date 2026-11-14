@row_set n=1 row=decision/fixtures-recorded-by-live-verify cell=rule seen=1 detector=D-tests-count-literals
value: The recorded agent hook, dialog, framing, status line, resume and screen fixtures are refreshed only by a live viola verify run, locally; the recorded MCP exchange fixtures by the test-only viola-mcp-tap; the recorded Hyprland replies by the test-only viola-hypr-tap.
basis: fct-stztm4lu
because: A second test-only recorder, viola-hypr-tap, now records the Hyprland replies, so the row naming who refreshes each fixture kind must name it.
@row_set n=2 row=decision/fixture-hygiene-walk cell=rule seen=1 fact=F3 detector=none
value: One run-time directory walk checks every fixture for absolute paths, non-placeholder usernames, email-shaped tokens and usernames split across a screen row seam, validates each file against its JSON schema, and holds each Hyprland reply to its closed per-request string keep-list.
basis: fct-w4fzcahg
because: The hygiene walk gained a Hyprland class that checks the recorder's closed keep-list, so the walk's rule must state that check.
@row_set n=3 row=decision/fixtures-walked-for-hygiene cell=rule seen=1 detector=none
value: Every committed fake script, Claude fixture and Hyprland fixture is walked against its schema and for absolute paths, usernames and emails, with planted cases proving each class red.
basis: fct-b2vvoxjd
because: The recorded Hyprland fixtures now join the hygiene walk with their own planted cases, so the row listing what is walked is incomplete without them.
@row_set n=4 row=decision/fake-agent-on-test-path cell=rule seen=1 fact=F2 detector=none
value: The fake agent stands as claude in a test-scoped bin dir, or as hyprctl by its own file name, answering from files beside its exe with no env var, and every test-started list or revive runs under a test-scoped PATH.
basis: fct-q6e6jeog
because: The fake agent gained a hyprctl personality chosen by its file name and configured from files beside its exe, so the row on where and how it stands must say so.
@row_set n=5 row=decision/fake-agent-argv-only cell=except seen=1 detector=none
value: Standing as hyprctl by its own file name, the fake agent reads its mode and reply set from files beside its exe.
basis: fct-woso52s6
because: The hyprctl personality takes its mode word from a file beside its exe rather than argv, an exception the argv-only rule must name.
@row_set n=6 row=contract/fake-agent-cli/argv-only-options cell=promise seen=1 detector=none
value: The fake agent's modes and options are taken from argv only, never from the environment; standing as hyprctl, its mode is read from a file beside its exe.
basis: fct-woso52s6
because: The hyprctl personality reads its mode from a file beside its exe, so the argv-only promise no longer holds without that exception.
@row_set n=7 row=decision/fake-agent-as-claude cell=rule seen=1 detector=none
value: The only external boundary, child processes, is served in tests by viola-fake-agent as claude on a test-scoped PATH, and as hyprctl in a test-scoped bin dir.
basis: fct-q6e6jeog
because: hyprctl is now a second external child process, served in tests by the same fake agent, so the row naming the boundary's only stand-in is incomplete.
@row_set n=8 row=decision/fakes-for-external-services cell=rule seen=1 detector=none
value: The real claude CLI is replaced by the native viola-fake-agent binary, both as the PTY child and as claude on a test-scoped PATH, the real hyprctl by the same binary as hyprctl, and Anthropic usage values by generated statusline stdin; tests never reach the real services.
basis: fct-q6e6jeog
because: The real hyprctl is now an external service the tests replace with the fake agent, so the list of replaced services must hold it.
@row_set n=9 row=decision/ci-never-real-claude cell=rule seen=1 fact=F4 detector=none
value: CI never touches the real claude or the real hyprctl.
basis: fct-qmcnrnzq
because: The Hyprland seam is held in CI only by the recorded set replayed through the fake and no CI image carries hyprctl, so the never-in-CI rule extends to it.
@row_set n=10 row=decision/bounded-readers-keep-the-cap cell=rule seen=1 fact=F5 detector=none
value: The status line shell-out, the version gate drain, the hyprctl reader and the thresholds, dialog hold and env-keep readers keep no more than the frame cap of what they read; a command the cut ends is a failed command.
basis: fct-ckgexzpj
because: The new hyprctl reader keeps at most the frame cap and reads a cut reply as oversize, held by its BOUNDARIES row, so the list of bounded readers must hold it.
@row_set n=11 row=decision/property-tests-on-parsers cell=rule seen=1 detector=D-tests-coverage
value: proptest at 512 cases covers every parser surface: paste validation, ndjson framing at the frame cap plus and minus one, the hook stdin, statusline, agents and hyprctl reply parsers, the vt100 feed and prompt-submitted normalisation, with its regressions directory committed.
basis: fct-arnp5nr7
because: The hyprctl reply parsers are a new parser surface under 512-case proptests with committed regressions, so the row listing every parser surface is incomplete.
@row_set n=12 row=decision/parser-property-fuzz-coverage cell=rule seen=1 detector=D-tests-coverage
value: Property and fuzz tests cover paste-text validation, channel ndjson framing at the MAX_FRAME boundary, the hook stdin parser, the agents listing parser, the hyprctl reply parsers, the vt100 feed and prompt-submitted normalisation.
basis: fct-arnp5nr7
because: The hyprctl reply parsers joined the property tests and the fuzz replay as hyprctl_json, so the row listing the covered parser surfaces must name them.
@row_set n=13 row=decision/property-tests-with-proptest cell=rule seen=1 detector=D-tests-coverage
value: Input parsers and validators are property-tested with proptest at 512 cases; proptest-regressions files are committed for the viola-core, hook, statusline, agents and Hyprland reply suites.
basis: fct-arnp5nr7
because: The Hyprland reply parsers' proptests commit their own regression seeds, so the row naming the suites with committed regressions is incomplete.
@row_set n=14 row=decision/no-production-data-in-tests cell=rule seen=1 detector=none
value: Tests never use production data; fixtures come only from `viola verify` probe text or a test-only recorder's capture, scrubbed of paths and usernames.
basis: fct-w4fzcahg
because: The Hyprland fixtures come from the test-only recorder viola-hypr-tap rather than viola verify probe text, so the row's only-source claim is false as written.
@row_set n=15 row=decision/recorder-scrub cell=rule seen=1 detector=none
value: The viola verify recorder rewrites home paths to ~ and usernames to <user> in hook payloads and status line variants before writing, and the test-only viola-mcp-tap and viola-hypr-tap apply the same scrub.
basis: fct-w4fzcahg
because: viola-hypr-tap runs the same home-path and username scrub after its keep-list, so the row naming the recorders that apply the scrub must hold it.
@row_set n=16 row=decision/dirty-payload-refuses-recording cell=rule seen=1 detector=none
value: A recorded payload, MCP exchange or Hyprland reply set that fails the unclean check refuses the whole recording.
basis: fct-w4fzcahg
because: viola-hypr-tap refuses a dirty Hyprland set whole by file and closed code, so the refusal rule must cover that recording too.
@row_add n=1 row=decision/hyprland-replies-keep-list area=7-test-data-fixtures fact=F1 detector=none
name: Hyprland replies kept to a closed string keep-list
rule: Hyprland replies are recorded live by the test-only viola-hypr-tap, one set per compositor version, each fixture keeping only a closed per-request list of strings (window addresses, the digit-form version tag, the ok dispatch text) and every other string replaced by the placeholder.
guard: record_replaces_every_string_off_the_allowlist
basis: fct-w4fzcahg
because: The chunk chose a closed per-request keep-list, not a title scrub, for what a recorded Hyprland reply may keep, a fixture convention no row states.
@row_none n=1 fact=F6 type=5
what: HyprlandError's place among the error-body census's not-error-body entries is an inventory entry the census code holds.
