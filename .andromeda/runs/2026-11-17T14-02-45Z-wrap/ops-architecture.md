@row_add n=1 row=decision/advisory-ignore-grounds area=build-system fact=F1 detector=D-arch-decisions
name: Advisory ignores on two grounds only
rule: An advisory ignore in the dependency policy stands on one of two grounds only, the rmcp advisory response or an unmaintained upstream with no safe upgrade, and each ignore carries its reason and is re-read at each epoch boundary.
guard: cargo deny check
basis: fct-s6xve3jr
because: The advisory ignore list gained a second admitted ground beside the rmcp response, a standing rule of the dependency policy that no build-system row yet holds.
@row_add n=2 row=decision/tied-boot-parent-death-signals area=occupied-resources fact=F3 detector=none
name: In-test boot tied by parent-death signals
rule: An in-test harness boot ties its stamping viola verify to the test thread by a parent-death SIGTERM and its supervisor by a parent-death SIGKILL.
why: Verify's own stop then removes its probe dirs and writes no stamp, so a killed test leaves no stamping verify behind.
guard: killed_test_leaves_no_stamping_verify
basis: fct-pr4bii4w
because: The stamping verify's parent-death signal moved from SIGKILL to SIGTERM while the supervisor's stays SIGKILL, a test-side process rule no occupied-resources row states.
@row_add n=3 row=decision/every-in-test-boot-tied area=occupied-resources fact=F4 detector=none
name: Every in-test harness boot passes tied
rule: Every in-test harness boot in viola-e2e passes tied, so no booted session or stamping verify outlives its test.
guard: viola_e2e_census_finds_no_untied_boot
basis: fct-5occom3n
because: The chunk tied every boot of viola-e2e's tests and added a census holding that no untied boot stands, a rule future tests must respect.
@row_add n=4 row=decision/shared-test-helper-cases-no-dev-deps area=conventions fact=F5 detector=none
name: Shared test helper cases need no extra dev-dependency
rule: A shared test helper taken by path carries its own test cases into every taker, so its cases use no dev-dependency a taker lacks.
why: A helper whose cases used rstest could not be taken by a crate that carries no rstest.
basis: fct-fnnj7js2
because: The shared source-scan masker's rstest table failed to compile in viola-e2e and became a plain case table, a convention binding every helper taken by path.
@row_none n=1 fact=F2 type=5
what: one entry of the advisory ignore list, held id by id with its reason in deny.toml and in security-plan's ignore row; architecture keeps only the grounds an ignore may stand on.
