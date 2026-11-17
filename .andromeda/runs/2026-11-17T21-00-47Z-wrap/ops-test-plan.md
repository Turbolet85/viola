@row_set n=1 row=contract/verify-stop/signal cell=promise seen=1 detector=none
value: Removes only its own run dirs and its own trust record, stdout empty, stderr exactly unable: verify was stopped before it finished / hint: run viola verify again to stamp this CLI, exit 1, no stamps.json, role log ending process-exit detail interrupted.
basis: fct-2t56x2kc
because: verify now writes a trust record for its own temp dir in Run F and a SIGTERM stop removes that record too, so the stop promise of removing only its own run dirs is incomplete.
@row_add n=1 row=decision/verify-under-test-owned-user-home area=test-data-bootstrap fact=F1 detector=none
name: Test verifies run under a test-owned user home
rule: Every test-started viola verify and every harness boot stamping runs under a test-owned user home whose config file is seeded empty and owner-only, and the fake agent is told that config file.
why: verify's --record scrub reads HOME, so a test-owned HOME would make a recording's absolute paths unclean.
except: A --record verify keeps the real HOME and points a kept CLAUDE_CONFIG_DIR at the test-owned user home instead.
basis: fct-shari3xw, fct-nvuvt53v
because: verify now writes a trust record into the config its child reads, so every test-started verify must point that write at a test-owned file, a convention no standing row states.
@row_add n=2 row=decision/no-verify-start-outside-test-home area=integration-test-strategy fact=F2 detector=none
name: Every test verify start tied to a test-owned home
rule: A census over the test sources holds every test-started viola verify start tied to a test-owned home, with a closed exempt list whose every entry carries its reason, and is proven red on a planted untied start.
guard: every_test_verify_runs_under_a_test_home
basis: fct-shari3xw, fct-2t56x2kc
because: a new census binary holds the test-owned-home rule for verify starts, a gate no standing census row states.
@row_add n=3 row=contract/fake-agent-cli/trust-config area=7-test-data-fixtures fact=F3 detector=none
boundary: viola-fake-agent command line
item: --trust-config
promise: The fake agent reads a trust record from the named config file by its own key rule, the nearest ancestor holding a git entry else the canonical cwd, through a bounded read; any failure reads untrusted.
basis: fct-ytf3ugzg
because: the fake agent gains an argv option whose record read decides its trust screen, a promise of its command line that no standing row holds.
@row_add n=4 row=contract/fake-agent-cli/trust-record-blank area=7-test-data-fixtures fact=F4 detector=none
boundary: viola-fake-agent command line
item: --trust-record-blank
promise: The test-only hold --trust-record-blank makes a start trusted by the trust record alone draw no screen, holding verify's Run F window open for a stop case.
basis: fct-n65gud5r, fct-ytf3ugzg
because: the fake agent gains a test-only hold option on its command line that no standing row holds.
@row_add n=5 row=decision/live-failing-reading-not-verification area=6-e2e-test-strategy fact=F5 detector=none
name: Live reading recorded as read, never as verification
rule: A live reading that fails a ledger row is recorded as read and never as verification of the row; a confounded live reading leaves the row unverified on that CLI version and its live re-read owed.
basis: fct-vw5u2c4x, fct-uvrzoqvc
because: the wrap directive rules how a failing or confounded live reading is recorded, a rule of the live tier no standing row states.
