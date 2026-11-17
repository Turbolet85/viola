@task_prose part=session
The supply-chain gate reads green again: the cargo-deny advisory ignore list widened from three ids to four, adding RUSTSEC-2026-0543 (chrono unmaintained, no patched version) on a second ground, the founder's unmaintained-upstream ground of 2026-11-17, its entry carrying its reason and a re-read at each epoch boundary.
The comment above the ignore list now states both grounds: the disabled-feature rmcp advisories and the unmaintained upstream with no safe upgrade. Bans, licences, sources and the graph are unchanged; no dependency, lockfile or tool version moved.
The fuzz workspace's own advisory audit now prints four advisory-not-detected warnings, up from three, and still passes.
Test-only: the e2e harness's in-test boot now ties its stamping viola verify to the test thread by a parent-death SIGTERM, up from SIGKILL, so verify's own guard ends its children and removes its own run dirs when a test is killed. The supervisor keeps its parent-death SIGKILL.
Two tests were added: a killed-test witness that no stamping verify, run dir or stamp outlives its test, and a census that no in-test boot in the e2e crate goes untied. The shared source masker's case table became one labelled test, with no rstest dev-dependency added.
CI went green on all 7 checks, supply-chain among them. No product symbol, command, status shape, log format or contract changed.
@task_file n=1 path=deny.toml
what: the cargo-deny configuration of the root graph: the advisory ignore list with each id's reason and the comment stating its grounds, beside the bans, the licence allow list and the crates.io-only sources
@task_file n=2 path=crates/viola-e2e/src/harness/boot.rs
what: the e2e test harness's in-test boot, which ties the supervisor and its stamping viola verify to the test thread by a parent-death signal (verify now SIGTERM, supervisor SIGKILL)
@task_file n=3 path=crates/viola-e2e/tests/harness_lifecycle.rs
what: the e2e harness lifecycle tests, whose in-test boots now pass tied
@task_file n=4 path=crates/viola-e2e/tests/cli.rs
what: the e2e crate's CLI tests driving a booted session, whose in-test boot now passes tied
@task_file n=5 path=crates/viola-e2e/tests/harness_tied.rs
what: the killed-test process hygiene tests: a killed test leaves no harness session and no stamping verify, its run dirs or a stamp, and a census that no in-test boot goes untied; test-side removal of the victim's own leftover dirs only
@task_file n=6 path=tests/support/source_scan.rs
what: the one shared source masker the census tests take, blanking comments and strings before a census scans code; its own cases now run as one labelled table test
@task_item n=1 part=touches
decision/rmcp-advisory-ignores holds that the ignore list is exactly RUSTSEC-2026-0457, RUSTSEC-2026-0461 and RUSTSEC-2026-0462 on the disabled-rmcp-feature ground alone. It is now stale: there are four ids and a second ground.
@task_item n=2 part=touches
The Dependency Security rule that an advisory may be ignored only as the Critical CVE response for a disabled feature is widened by the unmaintained-upstream ground. decision/cve-sla-disabled-feature and decision/cve-sla-enabled-feature still hold for the first ground, but the new ground's row belongs beside them.
@task_item n=3 part=touches
decision/cargo-deny-gate, decision/ci-cargo-deny-gate and decision/cargo-deny-audit: the root-graph cargo-deny gate went from red on RUSTSEC-2026-0543 back to green. It went green by an admitted ignore, not by an upgrade.
@task_item n=4 part=touches
decision/fuzz-lockfile-own-audit, decision/fuzz-lockfile-audited-separately and the fuzz workspace exemption rows: the fuzz lockfile's own advisory audit now carries four advisory-not-detected warnings, up from three. Touch any row whose cell counts those warnings or names the ignored ids.
@task_item n=5 part=touches
decision/scheduled-advisory-run and decision/manual-dependency-updates: the new ignore is to be re-read at each epoch boundary, with Epoch 4's boundary named. It is a standing review cadence for one ignore.
@task_item n=6 part=touches
decision/exact-dependency-pins and decision/exact-third-party-pins: chrono comes in through rmcp 3.4.1 and schemars 1.2.2 via the MCP crate, beside viola's and viola-state's own use. No safe upgrade exists and no pin moved.
@task_item n=7 part=touches
decision/verify-probe-dirs-cleanup and decision/verify-probes-confined-to-own-dir: verify's Run A OS-temp dir is removed by its own guard on pass, fail, Ctrl-C or TERM; SIGKILL, a crash or a failed handler install leave it for removal by hand. Booted tests now take the TERM path, and a killed-test witness reads that the dirs are gone.
@task_item n=8 part=touches
decision/e2e-home-removal-own-dirs-only and decision/test-removal-own-dirs-only: the new killed-test witness removes only the victim's own leftover home and run dirs. Two empty verify dirs left by SIGKILL control firings were removed by hand.
@task_item n=9 part=touches
decision/tests-use-own-home and decision/test-homes-gitignored: the new witness boots its victim in its own scratch home under the OS temp dir.
@task_item n=10 part=touches
decision/bounded-readers-census and the other census rules: the census tests share one source masker, not a copy, and the e2e crate now takes that one masker for its untied-boot census.
@task_item n=11 part=touches
decision/stamps-written-only-by-verify: the killed-test witness reads that a stopped verify writes no stamps file, because verify returns interrupted before updating stamps.
@task_fact n=1 slug=dependency-security
fact: An advisory ignore is admitted on one of two grounds only: the Critical CVE response for a disabled rmcp feature, or an unmaintained upstream with no safe upgrade (the founder's ground of 2026-11-17). Each ignore entry carries its reason, the comment over the list states both grounds, and each unmaintained-upstream ignore is re-read at every epoch boundary.
@task_fact n=2 slug=dependency-security
fact: The advisory ignore list holds exactly four ids: RUSTSEC-2026-0457, RUSTSEC-2026-0461 and RUSTSEC-2026-0462 for disabled rmcp features, and RUSTSEC-2026-0543 (chrono unmaintained, brought in by rmcp and schemars through the MCP crate beside viola's and viola-state's own use) on the unmaintained-upstream ground. RUSTSEC-2026-0543 is to be re-read at Epoch 4's boundary.
@task_fact n=3 slug=data-protection
fact: A test-booted stamping viola verify is tied to its test thread by a parent-death SIGTERM, and the supervisor by SIGKILL. A killed test therefore lets verify's own guard end its children and remove its run dirs, and verify writes no stamp.
@task_item n=1 part=untouched
Authentication and authorization: the peer euid checks, the socket directory and mode checks, the strict-modes checks and their refusal contracts are unchanged. No product code changed.
@task_item n=2 part=untouched
Input validation: frame caps, paste text character sets, hook and MCP input checks, the name type and the validators in the core crate are untouched.
@task_item n=3 part=untouched
Error handling and every product contract: CLI exit codes, channel methods and errors, MCP tool results, refusal words and log lines are unchanged.
@task_item n=4 part=untouched
Secret management: the credential strip, the hermetic pre-push environment, the canary scan classes and the live rig's credential handling are untouched.
@task_item n=5 part=untouched
The threat model's absences: no network listener, outbound call, release stage or secret store was added.
@task_item n=6 part=untouched
CI workflow rows: action SHA pins, least workflow permissions, zizmor, toolchain and MSRV pins, and the release build's test-feature refusal are unchanged. No workflow or tool version moved.
@task_item n=7 part=untouched
Data protection's fixture hygiene, plugin file rewriting, pinned-binary hashing and log modes are untouched.
