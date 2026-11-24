@row_set n=1 row=contract/cli-human/internal-error cell=promise seen=1 detector=none
value: Without --json an internal error prints the line "error: internal error" byte for byte; run, revive's start, adopt's start, mcp and hook stay silent there.
basis: fct-zdl3zlbi, fct-spvlmkow
because: The word adopt is now a silent role at the internal-error catch site, so the closed silent list must name adopt's start.
@row_set n=2 row=contract/strict-modes-refusal/no-line cell=promise seen=3 detector=none
value: A refused check at run, viola revive, viola adopt, viola verify, viola focus, viola up's board scan, the frame verbs, the hook and the MCP frame tools writes no line.
basis: fct-gnvnr36o, fct-spvlmkow
because: viola adopt refuses a widened home or instance dir before any log, so it joins the closed list of refusals that write no line.
@row_set n=3 row=decision/real-claude-local-only cell=rule seen=2 detector=none
value: Claude-facing behaviour is tested against the fake agent and recorded fixtures; the real claude CLI and Anthropic backend are mocked at unit level and reached only locally, through viola verify, the standing live rig's live starts and the gated live-reading scripts, never in CI.
basis: fct-zdl3zlbi, fct-wphbklsj
because: The chunk adds a test-only gated live-reading script that starts the real claude locally and is never run by a gate in live form, a route the row's closed list did not name.
@row_set n=4 row=decision/session-live-read-fails-open cell=rule seen=1 detector=none
value: revive --fork skips the session-live reading, and a failed claude agents read passes it, on revive and adopt alike.
basis: fct-k2nhjvap
because: viola adopt shares revive's live-elsewhere reading and starts when its agents read fails, so the fail-open rule now covers adopt.
@row_set n=5 row=decision/verify-driven-tests-20s-kill cell=except seen=2 detector=D-tests-count-literals
value: A verify-driven test named verify_window_ takes that class's 80 s kill instead.
basis: fct-xpheltqv, fct-gle6vr2m, fct-vuok4m6r
because: The two verify-driven cases of security_negatives_env_capture moved into the verify_window_ class, so their bound is no longer the 20 s ci kill the rule names.
@row_add n=1 row=decision/adopt-proven-live area=e2e-test-strategy fact=F1 detector=none
name: Real adopt proven only by the gated live reading R5
rule: Real adopt of a plain claude session, the resume of a real ended session and the refusal of a real live one, is proven only by the gated live reading R5; the fake-agent rehearsal of its script is never a live reading.
basis: fct-frpgpyuj, fct-vuok4m6r
because: The chunk proves adopt over the fake agent only and leaves the real-CLI clause to R5, a test strategy the plan must keep.
@row_add n=2 row=decision/adopt-preflight-order area=e2e-test-strategy fact=F2 detector=none
name: adopt preflight order
rule: viola adopt takes the instance check, the pinned exe, the wrapper log, the holder, then the live-elsewhere reading, in that order; the first refusal wins and a refused holder takes no agents read.
guard: adopt_preflight_order_the_first_refusal_wins, adopt_preflight_order_a_refused_holder_takes_no_agents_read
basis: fct-zdl3zlbi, fct-56rsq5py
because: The new verb fixes its own refusal order, which no standing row states.
@row_add n=3 row=decision/adopt-refuses-live-session area=e2e-test-strategy fact=F3 detector=none
name: adopt refuses a session open elsewhere
rule: viola adopt refuses with exit 1 session-live when the session id is open in another claude or held by a live adopted instance, with the pair unable: that session is open in another claude / hint: close it there first, then adopt it again, and starts no child.
guard: adopt_of_a_session_open_in_another_claude_is_session_live
state: provisional
basis: fct-zdl3zlbi, fct-k2nhjvap, fct-vuok4m6r
because: The new verb's live-elsewhere refusal and its own hint are a contract the plan must keep, its wording standing provisional.
@row_add n=4 row=contract/adopt-refusal-code/strict-modes-failed area=e2e-test-strategy fact=F4 detector=none
boundary: adopt refusal codes
item: strict-modes-failed
promise: A closed code of the adopt --json refusal document.
basis: fct-spvlmkow, fct-zdl3zlbi
because: viola adopt has its own closed list of --json refusal codes, which the revive rows do not cover.
@row_add n=5 row=contract/adopt-refusal-code/session-live area=e2e-test-strategy fact=F4 detector=none
boundary: adopt refusal codes
item: session-live
promise: A closed code of the adopt --json refusal document.
basis: fct-spvlmkow, fct-zdl3zlbi
because: viola adopt has its own closed list of --json refusal codes, which the revive rows do not cover.
@row_add n=6 row=contract/adopt-refusal-code/already-live area=e2e-test-strategy fact=F4 detector=none
boundary: adopt refusal codes
item: already-live
promise: A closed code of the adopt --json refusal document, shared with run's start.
basis: fct-spvlmkow
because: viola adopt has its own closed list of --json refusal codes, which the revive rows do not cover.
@row_add n=7 row=contract/adopt-refusal-code/stale-heartbeat area=e2e-test-strategy fact=F4 detector=none
boundary: adopt refusal codes
item: stale-heartbeat
promise: A closed code of the adopt --json refusal document, shared with run's start.
basis: fct-spvlmkow
because: viola adopt has its own closed list of --json refusal codes, which the revive rows do not cover.
@row_add n=8 row=contract/adopt-refusal-code/pinned-hash-mismatch area=e2e-test-strategy fact=F4 detector=none
boundary: adopt refusal codes
item: pinned-hash-mismatch
promise: A closed code of the adopt --json refusal document, shared with run's start.
basis: fct-spvlmkow
because: viola adopt has its own closed list of --json refusal codes, which the revive rows do not cover.
@row_add n=9 row=contract/adopt-refusal-code/squatted-name area=e2e-test-strategy fact=F4 detector=none
boundary: adopt refusal codes
item: squatted-name
promise: A closed code of the adopt --json refusal document, shared with run's start.
basis: fct-spvlmkow
because: viola adopt has its own closed list of --json refusal codes, which the revive rows do not cover.
@row_add n=10 row=contract/adopt-refusal-code/socket-dir-refused area=e2e-test-strategy fact=F4 detector=none
boundary: adopt refusal codes
item: socket-dir-refused
promise: A closed code of the adopt --json refusal document, shared with run's start.
basis: fct-spvlmkow
because: viola adopt has its own closed list of --json refusal codes, which the revive rows do not cover.
@row_add n=11 row=contract/adopt-refusal-code/internal-error area=e2e-test-strategy fact=F4 detector=none
boundary: adopt refusal codes
item: internal-error
promise: A closed code of the adopt --json refusal document, shared with run's start.
basis: fct-spvlmkow
because: viola adopt has its own closed list of --json refusal codes, which the revive rows do not cover.
@row_add n=12 row=contract/adopt-json/error-before-child-pump area=e2e-test-strategy fact=F4 detector=none
boundary: adopt --json
item: error before the child pump
promise: An error before the child pump prints the internal-error document.
basis: fct-zdl3zlbi, fct-pwskfqkr
because: viola adopt prints the internal-error document for an error before its child pump, a promise no standing row makes for adopt.
@row_add n=13 row=decision/adopt-id-session-shape area=e2e-test-strategy fact=F5 detector=none
name: adopt takes its id only in the session-id shape
rule: viola adopt takes its session id only in the session-id shape, ASCII hex of either case, as one argv element after --resume of a direct spawn; a malformed id, a --parent equal to --name, or words after -- are a usage exit 2 before anything is read.
guard: adopt_with_a_malformed_id_is_a_usage_error, adopt_args_refuse_as_usage
state: provisional
basis: fct-jtbdy5p2, fct-tc34su7x, fct-zdl3zlbi
because: The operator-typed id reaches the child's argv, and its accepted shape and usage exits are a decision the plan must keep, standing provisional.
@row_add n=14 row=decision/child-argv-read-from-proc area=test-data-fixtures fact=F6 detector=none
name: Child argv read from the live child's cmdline
rule: A test claim about the child's argv is read from the live child's /proc cmdline, never from the fake agent's receipt.
why: The fake agent's start receipt records no argv.
basis: fct-tc34su7x, fct-rzw2xxtn, fct-mpth4ozp
because: The chunk found the start receipt carries no argv and fixed where an argv claim is read, a convention later tests must follow.
@row_add n=15 row=decision/adopt-lineage-and-cwd area=e2e-test-strategy fact=F7 detector=none
name: adopt lineage parent and snapshot cwd
rule: viola adopt's lineage parent is --parent, else the inherited VIOLA_NAME unless equal to the given name, and its snapshot cwd is the folder adopt was run in.
guard: adopt_lineage_takes_the_given_parent_else_the_inherited_one
basis: fct-zdl3zlbi, fct-spvlmkow
because: The new verb fixes how its lineage parent and recorded folder are chosen, which no standing row states.
