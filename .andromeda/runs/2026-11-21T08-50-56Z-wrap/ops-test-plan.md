@row_set n=1 row=decision/r8-claude-env-strip cell=rule seen=1 detector=D-tests-count-literals
value: Every inherited CLAUDE* environment name, CLAUDE_CODE_MESSAGING_TOKEN and _SOCKET among them, is stripped from the child, always including every identity-floor name.
basis: fct-wu5hyrfe
because: The identity floor moved from 11 to 13 names, so the row's 11-name count is stale and the floor is named without a count.
@row_set n=2 row=decision/claude-env-strip cell=rule seen=1 detector=D-tests-count-literals
value: Every inherited CLAUDE-prefixed variable is removed by exact name unless in the persistent keep set, and every identity-floor name is removed whatever the keep set says.
basis: fct-wu5hyrfe
because: The identity floor grew from eleven to thirteen names, so the row's claim that the floor stays eleven names is false.
@row_set n=3 row=decision/literal-test-oracles cell=rule seen=1 detector=D-tests-count-literals
value: Tests never import the product's own identity floor, ledger rows or refusal ordering as the oracle; expected values such as the identity-floor names, the refusal order and exit codes are written as literals in the test.
basis: fct-wu5hyrfe
because: The identity floor moved from 11 to 13 names, so the row's 11-name example is a stale count literal.
@row_set n=4 row=decision/identity-floor-tested-against-literal-copy cell=rule seen=1 fact=F1 detector=D-tests-coverage
value: The R8 strip is tested against a literal copy of the identity floor, and the four non-identity hook names CLAUDE_PLUGIN_ROOT, CLAUDE_PROJECT_DIR, CLAUDE_ENV_FILE and CLAUDE_PLUGIN_DATA against a literal copy of their own, never imported ones; those four are never on the floor.
basis: fct-e2cnjuju
because: The chunk adds a four-name non-identity hook list pinned against a literal copy beside the floor, which the row says less of.
@row_add n=1 row=contract/claude-fixtures/env-names area=7-test-data-fixtures fact=F2 detector=none
boundary: Recorded Claude fixtures
item: env names
promise: The recorded env-names fixture Env.recorded-1.json holds names only, never a value, in the shape v 1 plus added; it is recorded by viola verify through the record scrub and checked by its own hygiene class.
basis: fct-y2ah5a4y
because: A new recorded fixture class with its own file format and hygiene class lands, and no fixture row states it.
@row_add n=2 row=contract/fake-agent-hooks/session-start-env area=test-data-fixtures fact=F3 detector=none
boundary: viola-fake-agent hook execution
item: SessionStart env
promise: The fake agent gives every SessionStart hook child each name of its set's recorded env names, always with the fixed value fake-agent.
basis: fct-yw2wvymn
because: The fake agent now hands the recorded env names to SessionStart hook children, a promise the tests rely on that no fake-agent row states.
@row_add n=3 row=decision/strip-control-oracle-keeps-guarded-names area=test-strategy fact=F4 detector=none
name: Strip control oracle keeps the guarded names
rule: A remove-the-guard control over the CLAUDE env strip uses an oracle that keeps the guarded identity-floor names, so removing the floor guard is seen.
why: The strip removes any unkept CLAUDE name anyway, so an oracle keeping none cannot see the floor.
guard: tui_env_strip
basis: fct-hbi6eyso
because: The floor-grows control did not hold until its oracle kept the floor names, a rule for every later control over the strip.
@row_add n=4 row=decision/capture-arm-env-names-only area=e2e-test-strategy fact=F6 detector=D-tests-coverage
name: Capture arm writes env names only
rule: The hook capture arm writes an owner-only env-names file on SessionStart alone, holding names only and no value, every non-CLAUDE name only counted; kept names never reach the stamp or the record.
guard: security_negatives_env_capture
basis: fct-cyougjyd
because: The capture arm gains an environment read whose names-only, SessionStart-only and kept-name bounds are held by a new canary security negative suite.
@row_none n=1 fact=F5 type=5
what: Which tests pin a fixture set's recorded session ids as literals is an inventory of test sources; the code and the sidecar hold it.
@row_none n=2 fact=F7 type=8
what: The identity-floor row's one live recording on 2.1.328 and the bundle-strings basis of one floor name are a measurement record; the report and the route hold it.
