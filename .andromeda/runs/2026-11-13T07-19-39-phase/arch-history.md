@history_item n=1 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (bounded readers, a cross-cutting pattern held by a census) — created `tests/contract_bounded_readers.rs` (guard of `decision/bounded-readers`): `BOUNDARIES`/`EXCEPTIONS` tables, the census `product_readers_are_each_bounded_or_named` judging each reader call over its statement, an `EXCEPTIONS` needle matched against the whole source line, and the self-check `census_finds_an_unbounded_read_and_passes_a_bounded_one`. Item 1 replaces this census's hand scanner (`code_only`) with the shared masking scanner: the verdicts, the whole-line needle rule and the self-check must stay unchanged.

@history_item n=2 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the sanitised error surfaces pattern and its contract binary) — created `tests/contract_error_bodies.rs` (guard of `decision/sanitised-error-surfaces`): `CLI_DOCUMENTS`/`MCP_RESULTS`/`CHANNEL_FAULTS`, `NOT_ERROR_BODIES`, `PLUMBING`, the census `product_error_writers_are_each_named` and `thiserror_messages_interpolate_nothing` (guard of `decision/thiserror-messages-interpolate-nothing`). Its scanner `blanked` is one of the three item 1 replaces; every census verdict must stay the same.

@history_item n=3 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the error-body check, and no after-every-test helper) — put `error_body_hit`/`assert_error_body_clean` in `tests/support/events.rs` and recorded that no after-every-test helper is built (`decision/no-after-every-test-error-body-helper`). A new shared test-support scanner or home helper (items 1, 6) sits beside it in `tests/support/` and must not bring back that helper.

@history_item n=4 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** (the OS-gated tests census) — created `tests/contract_os_gated_tests.rs` (guard of `decision/os-gated-tests-named`): the `GATES` table and a census over `tests/` and `crates/*/tests/` that reads attribute runs and `#[cfg]`/`#[cfg_attr]` predicates outside their string values (`feature = "unix-like"` is no hit), plus the 8-case self-check. Its hand scanner is item 1's third. Masking string literals is the census's own way of ignoring string values, so the string flag has to keep that behaviour exactly.

@history_item n=5 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the OS-gate census faults a gate naming another OS) — added the fault class `census_reads_a_gate_naming_another_os_as_a_fault_even_when_named` (`decision/no-other-os-test-gates`), its planted Windows cases building the word from a `WINDOWS` const, and noted that it caught a multi-line `cfg_attr` a line grep cannot see. The shared scanner must keep multi-line attributes and the planted cases working. The same class makes item 10's dead `#[cfg(not(unix))]` constants in `crates/viola-e2e/src/harness/run.rs` left-overs to delete, not gates to name.

@history_item n=6 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the code-word rule and the exit-cause table) — every exit-21 cause and exit-1 code is a row of `tests/contract_error_bodies.rs` passing `is_code_word`. That table grows inside the binary whose scanner item 1 replaces, so after the swap it must still read the same rows.

@history_item n=7 marker=2026-10-10-windows-mutation-grade
**2026-10-10-windows-mutation-grade** — brought back syn `=2.0.119` (`full`, `parsing`, `printing`, `visit`) and proc-macro2 (`span-locations`) as direct dependencies of the test-only `viola-e2e` alone (`decision/syn-test-only`). A shared scanner built on syn has to live where that row allows. No product crate or root test-support may take syn.

@history_item n=8 marker=2026-10-10-session-links
**2026-10-10-session-links** (the link set in the replay, the snapshot and the start) — created `src/run/links.rs`'s `LinkSet`: restored at start through `events::read_from` and `apply_link_line`, written through `Snapshots::update` at each change, with a failed append leaving the set as it was and a landed line kept even when the snapshot write fails (`decision/link-set-follows-log`, `contract/instance-snapshot/links`). Item 2 merges `links.rs:152` with `snapshot.rs:58`, and that order and failure rule must survive the merge.

@history_item n=9 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** (the wheel as landed) — set the rule that every snapshot write goes through `src/run/snapshot.rs`. Item 2's links↔snapshot merge must keep that one writer (`contract/instance-snapshot/writer`, `decision/link-set-changes-only-on-link-calls`).

@history_item n=10 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the served link and unlink methods; links.rs in the directory structure) — added `LinkSet::link`/`unlink` sharing `driver_of`, gave `remove` its product caller, and set that each method answers only after its event line and the snapshot write (`contract/channel-link-unlink/answer-order`, `contract/channel-link/already-linked`, `contract/channel-unlink/result`). These are the code paths item 2's merge touches.

@history_item n=11 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the hook's one connect and its fail-open on a failed server check) — made the hook's event and dialog paths share one connect in `src/cmd/hook.rs` (`deliver` and `ask`), failing open on `server-verify-failed`, with SessionEnd's direct append. This explains the duplicated pair at `src/cmd/hook.rs` 1075↔1152 that item 2 merges (`decision/dialog-hook-fails-open-on-server-check`, `contract/hook-diagnostics/server-verify-failed`).

@history_item n=12 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the hook's instance check before anything under the home) — moved `canonical`/`trusted` into `src/cmd/hook.rs` ahead of `read_diagnostics_level` and obs init, shared by the event, dialog and statusline arms. Any hook.rs merge must keep the check first, before anything is read (`decision/hook-without-trusted-instance-exits-silently`, `decision/hook-exits-on-untrusted-instance`).

@history_item n=13 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (what each replay reader decides) — made the hook's `connect` read through `read_snapshot_or_replay`, treating a replayed or absent snapshot as unreachable (`decision/hook-fails-open-on-replayed-snapshot`). This is part of the hook.rs connect logic item 2 may merge.

@history_item n=14 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a client's refused socket directory; the peer euid on both sides) — added `check_endpoint_dir` and the peer-euid admission to hook.rs `connect` (`NotSent::SocketDir`, `Asked::SocketDir`), with one `hook-decision` at WARN. A hook.rs merge must keep this order: strict, then socket dir, then euid, then pid (`decision/frame-client-check-order`, `contract/hook-diagnostics/socket-dir-refused`).

@history_item n=15 marker=2026-10-13-epoch-4-cleanup
**2026-10-13-epoch-4-cleanup** (send and human split by phase) — precedent for splitting files over 800 code lines: `src/human.rs` and `src/run/send.rs` became module roots with `pub(crate) use` re-exports so every caller keeps its path, `#[cfg(test)]` cases went to their own files, and each split was a pure move with no file over 800 code lines. Item 3 follows the same pattern.

@history_item n=16 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the split modules, `relied_here` and the `viola-mcp` serde edge retired) — the last cleanup split (`verify/typed.rs`, `human/board.rs`), with every caller's path kept. It registered `crates/viola-agent-claude/src/ledger/test_cases.rs` as `#[cfg(test)] #[path = "ledger/test_cases.rs"] mod tests`, which is exactly the file item 3 splits, so the `#[path]` declaration and the directory-tree row move with it.

@history_item n=17 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the recorded live MCP exchange: the tap) — created `viola-mcp-tap`, a `[[bin]]` of `viola-e2e` (`record`/`fixture`, scrub and `screen_fault`), now `name/binary/viola-mcp-tap` and `decision/test-binaries-never-released`. Item 5's 19 tap survivors are closed either by tests or by a recorded exclusion from the mutants scope.

@history_item n=18 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** (the 2.1.302 fixture set and the long-turn screen switch) — set the fake agent at thirteen argv options, "the count's rule is the list". The `src/bin/viola-fake-agent.rs` split (item 3) must keep every option and mode word (`statusline-echo`, `agents`), so these counts and their registry rows stay true (`decision/fake-agent-after-double-dash`).

@history_item n=19 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (the fake agent's agents mode) — gave the fake agent its `agents` mode word, read from files beside its exe, with no option or env seam added. Splitting `viola-fake-agent.rs` must keep this behaviour byte-identical and add no env read (`decision/fake-agent-test-seams`, `decision/harness-env-var-prefix`).

@history_item n=20 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — the real CLI runs only locally, and `viola verify` runs in CI against the fake agent's print mode. `tests/cli_verify.rs` is the guard of `decision/real-claude-local-only`, so splitting it (item 3) must keep that guard's tests and names.

@history_item n=21 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the ConPTY companions' owner/DACL check) — introduced `viola_state::strict::check_present` for `pin_companions` on `bin/<key>/conpty/`, and the read-rule entry lists it in `strict`. This is the only caller item 4's dead presence check ever had.

@history_item n=22 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the ConPTY companions and their vendored binaries retired) — removed `pin_companions`, its check (`PinError::StrictModes`) and `seed_conpty`, which left `check_present` with no caller. Item 4 can delete it; its remaining mention is `.claude/docs/services/viola-state.md`.

@history_item n=23 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (strict modes: the Windows owner/DACL arm retired; `[Snapshot writer]`: `replace_private` is one attempt) — removed the Windows arms of `crates/viola-state/src/{strict,fs,pin}.rs` (`retry_replace`, `REPLACE_ATTEMPTS`, `retry_open`). It also states the current shape of `fs.rs`/`pin.rs` behind item 5's error-kind guard survivors (`fs.rs:123`, `pin.rs:91`).

@history_item n=24 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** (the no-clobber placement; the pinned copy placed once) — added `place_private_new`, where `AlreadyExists` gives `Placed::Present` and any other error leaves the target, and `pin::place_absent`, which re-hashes on `Present` and refuses `pinned-hash-mismatch`. These are the error-kind guards item 5 closes with tests (`decision/pinned-copy-no-clobber-placement`, `decision/pinned-copy-rehash`, `contract/viola-run-start-refusals/pinned-hash-mismatch`). It also added the forced-window test parameter, which is not a seam.

@history_item n=25 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (`[MCP]` the viola mcp stdin line cap; the mcp verb's session end) — added `viola_mcp::stdin::CappedLines`: oversize refused whole, a cut read as end of stream, and one `parse-rejected{mcp-stdin, oversize, count:1}` at WARN written by `serve`. These are item 5's `viola-mcp` survivors: `lib.rs:95` (assert the WARN line), `stdin.rs:84` (a buffer smaller than the pending line) and `stdin.rs:15` `CHUNK` (`decision/mcp-stdin-line-cap`, `decision/mcp-stdin-oversize-refused-whole`, `contract/mcp-stdin-oversize/parse-rejected`).

@history_item n=26 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (`viola mcp`'s exit on a stdin cut before `initialize`) — measured exit 1, not 11, before `initialize`. Exit 11 is only the unit child's own code (`serve_child_entry`). A test written for the `lib.rs` survivor must assert the measured exits (`contract/viola-mcp-exit-codes/1-oversize-before-initialize`, `contract/viola-mcp-exit-codes/0-oversize-after-initialize`).

@history_item n=27 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the test-only child marker) — registered `MCP_SERVE_TEST_CHILD`, read only by `viola-mcp`'s `#[cfg(test)]` `serve_child_entry`. This is the precedent and naming rule for any self-spawn marker the mcp survivor tests need (`decision/harness-env-var-prefix`).

@history_item n=28 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** (the keepers' headroom floor) — added `HEADROOM_FLOOR_MIB` (9 GiB): one compiled constant per keeper, equal in both, read by `statvfs`, with a fixed message. This is the code behind item 5's 2 `free_bytes` survivors in `crates/viola-e2e/src/harness/mod.rs` (`decision/e2e-home-headroom-floor`, `contract/e2e-home-refusal/headroom-message`). Its basis, the quota'd tmpfs EDQUOT, also bears on item 8.

@history_item n=29 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — mutation testing left CI and the pre-push and now runs only through `agent-run run --mutants` for the epoch-boundary audit (`decision/mutation-at-epoch-boundary`). This is why item 5 closes survivors with tests or recorded scope exclusions and produces no mutation result.

@history_item n=30 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (mutation runs: no scratch arm, no Windows leg) — `run --mutants` lost its scratch arm, and the audit runs on Linux alone, with the operator's NOCOW `TMPDIR` as an environment fact. A recorded mutants-scope exclusion for the tap (item 5) goes into this Linux-only flow.

@history_item n=31 marker=2026-10-13-epoch-4-survivors
**2026-10-13-epoch-4-survivors** — precedent for closing an audit's named survivors without a mutation run: a guard made reachable by passing the decision in from the caller, and a shared report file made per-process. This is the closest prior pattern for item 5.

@history_item n=32 marker=2026-10-03-mutation-scoring-completion
**2026-10-03-mutation-scoring-completion** (the mutation scratch dir on the Linux host) — measured `/tmp` on the dev host as a 32 GB `usrquota` tmpfs (a full run died at 460/709) and required the operator's `TMPDIR` on NOCOW btrfs as an environment fact. This is the origin of item 8's quota'd temp dir, and item 8 has to name that temp root in one place.

@history_item n=33 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — made `target/e2e-home` a link to `/tmp/viola-e2e-home-<uid>` on the dev host, as an environment fact named by no variable, flag or config key, and added the keepers `prepare_home_base`/`Workspace::ensure_e2e_home` with the linked-base checks. It also ratified removals outside the working directory. Item 8's "set once" temp root must respect that no-variable rule (`decision/dev-host-e2e-home-on-tmpfs`, `decision/linked-e2e-home-base-strict`, `decision/test-side-removal-outside-workdir`).

@history_item n=34 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the test-owned socket roots) — registered `SocketRoot`/`boot_on_socket_root` under `/tmp` (`XDG_RUNTIME_DIR`). This is another test-side temp root that item 8's single named temp root has to account for.

@history_item n=35 marker=2026-10-02-epoch-2b-cleanup
**2026-10-02-epoch-2b-cleanup** — routed both test-home removals through `remove_owned` (`tests/support/home.rs`), deleting `owner.json` last. A shared viola-made-home helper (item 6) and a refused recursive `rm` (item 8) must keep the owner-record-last rule (`decision/test-home-owner-sweep`, `decision/test-homes-cleaned-by-owner`).

@history_item n=36 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (the instance check before any log, on both arms) — established that opening a log narrows an existing home to 0700, so a check made after it passes a widened home. This explains why tests on homes made at 0755 went red and why item 6's helper must make a home the way viola does, at 0700 (`decision/strict-check-before-log-open`, `decision/instance-check-before-any-read`).

@history_item n=37 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the read rule and check_instance's widened set) — set the read rule (the write rule plus no group or other bit) on the home, the instance directory and `diagnostics/`, which is what refuses a 0755 home. These are the 19 Epoch 6 reds that item 6's helper fixes (`decision/strict-read-rule`, `decision/strict-modes-check-before-log`).

@history_item n=38 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (`viola list`'s one home rule) — moved `board::scan` to `check_read` on the home, so a 0750 home refuses the whole board with or without an instance (`decision/list-home-read-rule`). This is another standing test path that item 6's 0700 helper must satisfy.

@history_item n=39 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** (the shared Frozen guard) — moved two local twins into one home, `tests/support/frozen.rs`, used from two test files (`decision/root-tests-freeze-wrappers-with-one-guard`). This is the precedent for item 6's single test-home helper and item 7's single remove-the-guard control helper.

@history_item n=40 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the root `WITHIN` waits: 31 sites in 23 files) — the count of `Instant::now() + WITHIN` under `tests/` is kept by its grep rule. Splitting `tests/cli_verify.rs` (item 3) or adding shared helpers (items 6, 7) can change the site and file counts in `occupied-resources` (`decision/root-waits-share-one-bound`), so re-measure them at wrap.

@history_item n=41 marker=2026-10-01-t12-19-55-wrap
**2026-10-01-t12-19-55-wrap** — gated GUI reads by the per-launch `viola_<port>` cookie. This is the origin of the GUI cookie pair the secret scan's `cookie` class catches, which item 9 removes.

@history_item n=42 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — made 0.1.0 a pure CLI and removed the cookie-gated `/api/*`, `ui/<port>.url` and the `viola-ui` GUI. The `cookie` class's subject has been gone since then; the `secret-scan` uploads gate (`decision/uploads-gated-on-secret-scan`) stays.

@history_item n=43 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — the 0.2.0 census found that viola has no web or graphical frontend (absences `absence/graphical-interface`, `absence/web-or-graphical-frontend`, `absence/listening-network-port`). This confirms that item 9's removal of the cookie class matches the masters.

@history_item n=44 marker=2026-09-26-ci-chunk-base-and-union-verdict
**2026-09-26-ci-chunk-base-and-union-verdict** — set `chunk.diff` as the one file `secret-scan` skips, by exact path, with the upload excluding it (`decision/chunk-diff-secret-scan-skip`, `decision/secret-scan-skips-only-chunk-diff`). Editing `crates/viola-e2e/src/harness/secret_scan.rs` for item 9 must keep that one skip and every other class.

@history_item n=45 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the OS target: Linux only) — no Windows or macOS code, test, fixture or CI job may stand in the tree, and the cut code is recoverable at `multi-os-final` (`decision/linux-only`, `absence/windows-and-macos-support`). This is the ruling behind item 10's dead `#[cfg(not(unix))]` constants in `harness/run.rs`.

@history_item n=46 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (test support and harness: the Windows and macOS arms retired; residual Windows reasons) — swept the harness's Windows arms and ran a residual census that found wording no sweep pattern named. The `#[cfg(not(unix))]` lines in `crates/viola-e2e/src/harness/run.rs` (item 10) are left-overs that sweep missed.
