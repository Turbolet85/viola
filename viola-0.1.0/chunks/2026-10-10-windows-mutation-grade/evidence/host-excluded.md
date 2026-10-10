# Host-excluded mutants — the recipe's reading on the final tree, and what the harness read

Written at step 9, 2026-10-10. Two readers stand side by side here:

- **the recipe**: the audit's pinned `cover.py` (`.andromeda/runs/2026-10-08T10-08-51-code-audit/`), run over
  `cargo mutants --list --json` per workflow item by `forecast.py` in the implement run dir
  (`.andromeda/runs/2026-10-10T02-45-34-implement/`). It is a forecast: it builds nothing and grades nothing. For
  Windows it reads `rustc --print cfg --target x86_64-pc-windows-msvc`; for Linux, the host's own cfg;
- **the harness**: the new reader in `run --mutants` (`harness/run/mutants/host.rs`), which leaves a missed mutant
  out when a `cfg` covering its whole span is false on the host it was built for. It was run on the Linux dev host
  only (`witness-runs.md`) when this file was written at step 9; step 11 then held the dispatch. Its reading on a
  Windows host was added after the dispatch, in the section "Added after the dispatch" below.

The recipe was read at 2026-10-10T03:02Z, after the last source edit (03:00:46Z) and before the first witness run
(03:03:40Z); no source or test file changed afterwards, so it is the final tree's reading.

## The recipe, per workflow item, on the final tree

642 mutants over the nine items (644 at HEAD when the plan was written: viola-state lists 156, was 159;
viola-e2e 69, was 68).

| job | package | mutants | left out on `windows-2025` | left out on Linux |
|---|---|---|---|---|
| `mutants (viola-pty)` | viola-pty | 89 | 5 | 25 |
| `mutants (viola-channel)` | viola-channel | 148 | 4 | 31 |
| `mutants (viola-state)` | viola-state | 156 | 7 | 69 |
| `mutants (viola-agent-claude)` | viola-agent-claude | 40 | 0 | 0 |
| `mutants (viola-cmd-run)` | viola | 50 | 4 | 10 |
| `mutants (viola-main)` | viola | 34 | 0 | 0 |
| `mutants (viola-run-env)` | viola | 33 | 0 | 8 |
| `mutants (viola-panic-frames)` | viola | 23 | 9 | 9 |
| `mutants (viola-e2e)` | viola-e2e | 69 | 1 | 0 |
| total | | 642 | **30** | **152** |

The plan predicted 29 on Windows at HEAD (viola-state 6). The final tree reads 30: viola-state is 7, because
`restrict` is now a `cfg(unix)` function of its own and its one mutant (`fs.rs:19:5`) is a Unix twin on a Windows
host. The plan predicted 160 on Linux at HEAD; the final tree reads 152.

### On `windows-2025`, by file and predicate (30)

| job | file | predicate | mutants |
|---|---|---|---|
| viola-pty | `crates/viola-pty/src/lib.rs` | `unix` | 5 |
| viola-channel | `crates/viola-channel/src/client.rs` | `unix` | 1 |
| viola-channel | `crates/viola-channel/src/endpoint.rs` | `unix` | 1 |
| viola-channel | `crates/viola-channel/src/server.rs` | `unix` | 2 |
| viola-state | `crates/viola-state/src/fs.rs` | `unix` | 1 |
| viola-state | `crates/viola-state/src/strict.rs` | `unix` | 6 |
| viola-cmd-run | `src/cmd/run.rs` | `all(windows, not(target_arch = "x86_64"))` | 4 |
| viola-panic-frames | `src/panic_frames.rs` | `unix` | 9 |
| viola-e2e | `crates/viola-e2e/src/harness/cleanup.rs` | `unix` | 1 |

The viola-e2e row is `cleanup.rs:145:9` (delete `!` in `unconnectable`), which was `140:9` before this chunk's
edit of that file.

### On Linux, by file and predicate (152)

| job | file | predicate | mutants |
|---|---|---|---|
| viola-pty | `crates/viola-pty/src/lib.rs` | `windows` | 16 |
| viola-pty | `crates/viola-pty/src/sideload.rs` | `windows` | 9 |
| viola-channel | `crates/viola-channel/src/client.rs` | `windows` | 10 |
| viola-channel | `crates/viola-channel/src/client.rs` | `all(windows, test)` | 1 |
| viola-channel | `crates/viola-channel/src/server.rs` | `windows` | 2 |
| viola-channel | `crates/viola-channel/src/server/win.rs` | `windows` | 12 |
| viola-channel | `crates/viola-channel/src/test_support.rs` | `windows` | 6 |
| viola-state | `crates/viola-state/src/fs.rs` | `windows` | 17 |
| viola-state | `crates/viola-state/src/pin.rs` | `windows` | 18 |
| viola-state | `crates/viola-state/src/strict.rs` | `windows` | 34 |
| viola-cmd-run | `src/cmd/run.rs` | `all(windows, target_arch = "x86_64")` | 4 |
| viola-cmd-run | `src/cmd/run.rs` | `all(windows, not(target_arch = "x86_64"))` | 4 |
| viola-cmd-run | `src/cmd/run.rs` | `windows` | 2 |
| viola-run-env | `src/run/env.rs` | `windows` | 8 |
| viola-panic-frames | `src/panic_frames.rs` | `windows` | 9 |

## What the harness read, beside the recipe (Linux only)

| run | harness `host_excluded` | the recipe, same mutants | difference |
|---|---|---|---|
| witness 1: the whole viola-pty item | 25 rows, all `windows` | 25 rows, all `windows` | none: equal by name and by predicate |
| witness 2: the chunk's lines in `host.rs`, `mutants.rs`, `cleanup.rs` | none (field absent) | no row of the recipe's Linux list lies in these lines | none |
| witness 3: the chunk's lines in `strict.rs`, `fs.rs` | 16 rows, all `windows` | each of the 16 is in the recipe's Linux list for viola-state, same predicate | none on these 16; the item's other 53 rows were not mutated by this scoped run |

Not measured by any run of this chunk:

- the harness's reading on a Windows host, for any job;
- the harness's reading on Linux for viola-channel, viola-cmd-run, viola-run-env and viola-panic-frames, and for
  the 53 viola-state rows outside the chunk's lines. Among them are three shapes the reader's unit cases cover
  with fixtures but no real run has read: a file left out by its `mod` declaration (`server/win.rs`), a predicate
  with an unknown key beside a false one (`all(windows, test)`), and an architecture key;
- one known difference of rule, with no effect on these 642: the recipe decides every `target_*` key, the
  harness decides `target_family`, `target_os` and `target_arch` and reads any other key as unknown (the plan's
  rule). No predicate in the workflow's files uses another `target_*` key.

## Added after the dispatch (2026-10-10T08:51Z): what the harness read on `windows-2025`

The first item of the list above is now measured. Run 38036448183 on `dd5161d55743` (`windows-dispatch.md`, on the
founder's word, `inputs#I5`) ran the harness on a Windows host for all nine jobs:

| job | harness `host_excluded` | the recipe's Windows list | difference |
|---|---|---|---|
| viola-pty | 5, all `unix` | 5 | none |
| viola-channel | 4, all `unix` | 4 | none |
| viola-state | 7, all `unix` | 7 | none |
| viola-agent-claude | field absent | 0 | none |
| viola-cmd-run | 4, all `all(windows, not(target_arch = "x86_64"))` | 4 | none |
| viola-main | field absent | 0 | none |
| viola-run-env | field absent | 0 | none |
| viola-panic-frames | 9, all `unix` | 9 | none |
| viola-e2e | 1, `unix` | 1 | none |
| total | 30 | 30 | equal by mutant name and by predicate |

The recipe was read again on this tree at 08:02Z and printed the table above unchanged (642 mutants, 30 and 152).
The architecture-key shape was read by a real run here (`viola-cmd-run`). Still not read by any real run: the
harness on Linux for the items and rows the second item above names, with the `mod`-declaration shape and the
unknown-key shape among them.

## The four `src/cmd/run.rs:385:5` mutants: not measured on any host this project builds on

`sideload_outcome` under `cfg(all(windows, not(target_arch = "x86_64")))`, replaced by `("", None)`,
`("", Some(Default::default()))`, `("xyzzy", None)` and `("xyzzy", Some(Default::default()))`.

- The Linux dev host does not compile the body (`windows` is false).
- The `windows-2025` runner is x86_64 and does not compile it either (`not(target_arch = "x86_64")` is false).
- No workflow of this project builds a Windows target of another architecture.

So both readers leave the four out on both hosts, and no test of any run can grade them. They are recorded here
as **not measured on any host**: not caught, not equivalent, not graded. At run 37761947926 they read MISSED on
`windows-2025`, which was the host counting a body it had not compiled. They are owed to no route entry yet; the
wrap's route step is told (the plan's Implementation notes).

## Behind a `cfg!` const: compiled on both hosts, never left out

The eight mutants behind `HOST_IS_WINDOWS` (`fs.rs`, the six retry-loop mutants) and `HOST_SCRATCH`
(`scratch.rs`, the two `prepare` mutants) compile on both hosts, so neither reader leaves them out. On the host
that does not take the arm they read missed and stay recorded "not measured here" (`survivors.md`, the second
table). A Linux `--package` run of viola-state or viola-e2e therefore still reads red by them.
