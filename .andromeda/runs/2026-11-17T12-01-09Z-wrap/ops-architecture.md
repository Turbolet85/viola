@row_add n=1 row=name/other/root-tied-test-child area=occupied-resources fact=F1 detector=D-arch-resources
class: other
name: ROOT_TIED_TEST_CHILD
basis: fct-y7jldnao
because: The chunk lands the test-only self-spawn marker ROOT_TIED_TEST_CHILD outside VIOLA_*, and the register of occupied names does not hold it yet.
@row_add n=2 row=name/other/harness-tied-test-child area=occupied-resources fact=F1 detector=D-arch-resources
class: other
name: HARNESS_TIED_TEST_CHILD
basis: fct-y7jldnao
because: The chunk lands the test-only self-spawn marker HARNESS_TIED_TEST_CHILD outside VIOLA_*, and the register of occupied names does not hold it yet.
@row_add n=3 row=decision/test-viola-children-tied area=occupied-resources fact=F2 detector=none
name: Test-spawned viola children tied to their test
rule: Every long-lived viola child a root test spawns is tied to its spawning thread by a parent-death SIGKILL, and an in-test harness session's supervisor and stamping verify are tied the same way.
guard: cross_killed_test_census_finds_no_untied_viola_spawn
except: The viola-harness CLI boot's supervisor is not tied.
basis: fct-uvauzl42
because: The chunk ties every root test's viola spawn and the in-test harness session's supervisor to the test, held by a census, and no architecture row states that rule.
@row_add n=4 row=decision/test-verify-stamp-generations area=occupied-resources fact=F3 detector=none
name: Test verify stamp counts only when complete
rule: The shared test verify stamp counts only when it holds its completion file and its ledger stamps file; a hollow stamp is left standing, the next generation is built under viola-stamp-<key>-g<n> through a temporary dir renamed into place, and no stamp dir is removed.
guard: stamped_home_rebuilds_a_stamp_restored_without_its_files
basis: fct-y7jldnao
because: The chunk makes a stamp count only by its completion and stamps files and rebuilds a hollow one as a new generation without removal, and no architecture row states that rule.
