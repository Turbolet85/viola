@history_item n=1 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (4. Unit Test Strategy, ~ decision/bounded-readers-keep-the-cap) — added the hyprctl reader to the readers that keep no more than the frame cap, a cut reply read as oversize, held by its BOUNDARIES row of `contract_bounded_readers`. This chunk pins that cap's exact edge (exactly `MAX_FRAME` accepted, one more `oversize`) at `crates/viola-hyprland/src/read.rs`; the row's guard and its BOUNDARIES witness are what the new test extends.

@history_item n=2 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (5. Integration Test Strategy, ~ decision/fake-agent-as-claude; 8. Mocking, ~ decision/fakes-for-external-services; 2. Test Strategy, ~ decision/ci-never-real-claude) — the fake agent now also stands as hyprctl in a test-scoped bin dir, replaces the real hyprctl, and CI never touches the real hyprctl. The read's exact-frame case must run through that fake `hyprctl`, never the real one, as the scope's boundary also demands.

@history_item n=3 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (7. Test Data & Fixtures, ~ decision/fake-agent-on-test-path, ~ contract/fake-agent-cli/argv-only-options, ~ decision/fake-agent-argv-only except) — standing as hyprctl by its own file name, the fake agent reads its mode and reply set from files beside its exe, no env var. A `MAX_FRAME` / `MAX_FRAME + 1` reply must be planted that way (a reply file or mode beside the exe), never through an env seam or argv.

@history_item n=4 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (6. E2E, ~ decision/property-tests-on-parsers, ~ decision/property-tests-with-proptest; 1. Test Scope Summary, ~ decision/parser-property-fuzz-coverage) — the hyprctl reply parsers joined the 512-case proptests (committed regressions) and the fuzz replay as `hyprctl_json`. The new exact-frame cases on `parse_dispatch` sit beside that parser surface; they don't replace the property suite and mustn't disturb its regressions directory.

@history_item n=5 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (1. Test Scope Summary, ~ decision/fixtures-recorded-by-live-verify; 7., ~ decision/fixture-hygiene-walk, ~ decision/fixtures-walked-for-hygiene, ~ decision/recorder-scrub, ~ decision/dirty-payload-refuses-recording; 11., ~ decision/no-production-data-in-tests) — recorded Hyprland replies come only from the test-only `viola-hypr-tap`, and the hygiene walk covers them. A synthetic oversize reply is a test-built plant, not a recorded fixture: it must not be committed under the recorded fixtures, or the walk and keep-list would judge it.

@history_item n=6 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the bounded-reader enumeration in the Contract suite) — built `tests/contract_bounded_readers.rs` with literal `BOUNDARIES` / `EXCEPTIONS` tables, each row naming its bounded read, cap and witness test, plus the `product_readers_are_each_bounded_or_named` census (decision/bounded-readers). If the hyprctl rows' witness becomes the new exact-frame test, that row's needle and witness must still stand, or the census reads red.

@history_item n=7 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the `viola mcp` stdin line cap's tests) — this is the direct precedent for the edge this chunk pins: `capped_lines_*` held a line of exactly the cap whole and one byte more releasing no byte, at the literal `16 * 1024 * 1024` too. That's the exact-cap / cap+1 pair shape to copy for `read_within` and `parse_dispatch`.

@history_item n=8 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the bounded compare read of `replace_private_shared`) — precedent for a read through `take(len + 1)` deciding the bound the way a whole read would. It's the same drain-through-`MAX_FRAME + 1` mechanism at `read.rs:121`, which the exact-frame test exercises from both sides.

@history_item n=9 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the bounded readers' cap-driving witnesses) — set the precedent that a BOUNDARIES row's witness must drive past its cap, not just read the bounded read (e.g. `statusline_shell_out_past_the_frame_cap_keeps_exactly_the_cap`, `drain_keeps_cap_bytes_of_an_endless_pipe`), and discharged an owed clause doing it. The hyprctl reader's exact-frame and one-more cases are this kind of cap-driving witness.

@history_item n=10 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the bare temp-dir home and the liveness-check case) — the trap here: an oracle moved by a shared body can hide a test that never reached its subject. The `MAX_FRAME`-exact `parse_dispatch` case expects `Refused` (not success text). It has to show it got past the size check and isn't refused for another reason, or the `>`→`>=` control could pass vacuously.

@history_item n=11 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (remove-the-guard witnesses bounded by `WITHIN`) — set the per-chunk remove-the-guard rule (decision/bounded-remove-the-guard-witness): a control whose neutralised guard lets a product process start needs a bounded witness, since nextest's default profile has no kill line (one hung 27 min). The read's control drives a fake `hyprctl` child, so its witness and the `gate.py control` run need a kill deadline.

@history_item n=12 marker=2026-10-13-epoch-4-survivors
**2026-10-13-epoch-4-survivors** (the Epoch 4 survivors disposed) — precedent for this chunk's form. Boundary-audit survivors were disposed by a nextest case in the package that owns the mutated file (decision/mutants-tested-by-owning-package), each shown red under its hand-applied mutant and green restored, with no mutation run made, and the next boundary audit confirms the kills. Here that means `viola-hyprland`'s own tests must kill both `>`→`>=` sites.

@history_item n=13 marker=2026-10-13-epoch-4-survivors-ii
**2026-10-13-epoch-4-survivors-ii** (the 16 typed.rs survivors disposed) — same hand-shown-kill precedent: each survivor killed by a case in its own module, red under the hand-applied mutant, no mutation run, confirmation left to the next epoch-boundary audit. That matches this chunk's "the next audit catches both survivors".

@history_item n=14 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (`prepare` killed on every host, the Epoch 5 survivors disposed) — 16 audit survivors were each disposed by a nextest case in the owning package, red under the hand-applied mutant and reverted byte-exact, no mutation run made, none equivalent. This is the latest form of the survivor-closing chunk this one repeats for `read.rs:77` and `reply.rs:151`.

@history_item n=15 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — moved mutation to the epoch boundary, with no per-chunk, pre-push or CI mutation gate and `run --mutants` named-only (decision/mutation-at-epoch-boundary, mutation-testing-on-demand, mutation-gate-cadence, mutation-not-a-ci-gate). This is why the chunk runs no mutation run and its controls go through `gate.py control` instead.

@history_item n=16 marker=2026-10-03-mutation-scoring-completion
**2026-10-03-mutation-scoring-completion** (run --mutants --package; the boundary tier's form) — the boundary tier scores a whole member with `run --mutants --package <member>`, which is how `viola-hyprland` read 96.00 %, 2 missed. A mutant is only killed by its own package's tests, so the pins belong in `viola-hyprland`'s test modules, as the scope places them.

@history_item n=17 marker=2026-09-24-epoch-1-cleanup
**2026-09-24-epoch-1-cleanup** (an unviable swamp is red) — added `unviable <= caught` to the mutation verdict (decision/mutation-run-failure-conditions, mutation-run-verdict). That's part of the rule the next audit will apply to `viola-hyprland` after this chunk's kills.

@history_item n=18 marker=2026-10-10-windows-mutation-grade
**2026-10-10-windows-mutation-grade** (host-excluded missed mutants) — "missed" in the verdict means missed beyond host-excluded `cfg` records. The two survivors are not host-excluded (Linux-reachable `>` comparisons), so they count as missed until a test kills them.

@history_item n=19 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the always-admit survivor as 0.1.0's known limit) — the one standing exception to the mutation verdict (the client seam's always-admit mutant, matrix#v1-44). It doesn't cover these `MAX_FRAME` survivors, which must be killed rather than carried as a known limit.

@history_item n=20 marker=2026-09-24-observability-gates
**2026-09-24-observability-gates** (mutants kill below the floor) — put every in-test wait a mutant can reach, and the mutants profile kill, below cargo-mutants' 20 s timeout floor so a caught mutant is never graded Timeout (decision/mutants-kill-below-cargo-mutants-timeout). The new read test's wait on the fake `hyprctl` must stay inside that bound for the next audit to grade it caught.

@history_item n=21 marker=2026-09-27-browser-verdict-reachability
**2026-09-27-browser-verdict-reachability** (root waits at 7 s) — set the root `WITHIN` bound at 7 s, below the 10 s mutants kill, proven by a remove-the-guard pair (decision/root-wait-bound, within-7s-wait-bound). Any wait the new test makes on the hyprctl child takes `WITHIN`, not a sleep (decision/no-sleep-synchronization, no-test-retries-or-sleeps).

@history_item n=22 marker=2026-10-04-windows-boundary-mutation-workflow
**2026-10-04-windows-boundary-mutation-workflow** (the mutants profile waits for running tests) — `[profile.mutants]` fail-fast became `terminate = "wait"` under the slow-timeout kill (decision/mutants-fail-fast-wait). That's the profile the next audit runs the new `viola-hyprland` cases under; a control reaching its case "past fail-fast" depends on it.

@history_item n=23 marker=2026-10-25-windows-endpoint-admission
**2026-10-25-windows-endpoint-admission** (nextest the only supported runner) — nextest is the runner and a plain `cargo test` isn't supported (decision/nextest-is-the-runner). The new cases and their controls are read under nextest.

@history_item n=24 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (fake-agent copies planted one after another, ETXTBSY) — a test planting more than one fake-agent copy must plant every copy before any thread starts a child (decision/fake-agent-copies-planted-first). This applies if the exact-frame and one-more read cases each plant a fake `hyprctl` copy.

@history_item n=25 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** (the pinned copy's no-clobber placement) — a placed copy is persisted only once its write handle is closed so it can exec at once (decision/placed-copy-execs-at-once, guard `place_private_new_placed_copy_execs_at_once`). This is the ETXTBSY trap a freshly placed fake `hyprctl` can hit.

@history_item n=26 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (the fake agent's agents files, the test-scoped PATH) — first set the pattern the hyprctl personality follows: the fake standing as a named binary in a test-scoped bin dir, answering from mode files beside its exe (`agents-mode` with an `oversize` mode), no env var, by the founder's own word. It's the precedent for planting an oversize reply.

@history_item n=27 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** (Path 5 records the classifier's tables; the real CLI has two local routes) — a route entry's live readings of a real session are that chunk's own local measurement, not a test layer and never in CI (decision/live-readings-not-a-test-layer). This chunk takes no live desktop or real `hyprctl` step at all.

@history_item n=28 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the OS scope: Linux only, one CI leg; the CI matrix [ubuntu-latest]; the OS-gate census reads another OS as a fault) — the product is Linux only with CI on one `ubuntu-latest` leg (decision/linux-only, tests-run-on-linux-ubuntu-latest, single-os-ci-leg, linux-only-ci-matrix). A gate naming windows or macos is a census fault (guard `contract_os_gated_tests`), so the new cases need no OS gate.

@history_item n=29 marker=2026-09-27-wrapper-channel
**2026-09-27-wrapper-channel** (Wrapper channel oversize; channel_frame fuzz) — the earlier `MAX_FRAME` boundary precedent: channel ndjson framing pinned at the frame cap plus and minus one, with oversize close readings measured. It's the same boundary convention this chunk applies to the Hyprland seam's two bounds.

@history_item n=30 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** (the masters as rows) — turned the test plan's text into rows, each verified against the code. Every picked row of this chunk's rows file comes from this, and any amendment this chunk needs takes the `+ / ~ / −` row form the 2026-11-14-hyprland-seam entries use.
