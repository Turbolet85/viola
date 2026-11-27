@history_item n=1 marker=amd-puxntbrv
**amd-puxntbrv** — set `[profile.mutants]` slow-timeout with a viola-e2e override and recorded cargo-mutants' 20 s auto-timeout floor; a CI mutants red was two waits spinning to 20 s under that floor. This is the origin of the rule item 1 rests on: every kill line has to stay below the floor so a caught mutant is never graded Timeout.

@history_item n=2 marker=amd-zfag6eyk
**amd-zfag6eyk** — the counted verdict requires `unviable <= caught`, and cargo-mutants output streams live. Item 1's addendum score (caught / missed / timeout / unviable) is judged against this rule.

@history_item n=3 marker=amd-idgxwnez
**amd-idgxwnez** — liveness is the pure `classify(beat_age, same_process)` (5.0 s live, 5.1 s stale, a mismatch gone), and `run --mutants` keeps no test home. This is the viola-state liveness surface item 4's survivor kills sit beside.

@history_item n=4 marker=amd-w5k5ft3a
**amd-w5k5ft3a** — `run --mutants` builds in its own `target/mutants`, because a copied default target made root tests drive the unmutated binary. Item 1's recorded form passes `--copy-target=false`, so this trap bears on the addendum run.

@history_item n=5 marker=amd-5gt53dby
**amd-5gt53dby** — mutation moved to the epoch boundary: no per-chunk, pre-push or CI mutation gate, and the verdict rule is unchanged. This is why item 1 allows exactly one announced run and items 2 and 4 are shown by hand.

@history_item n=6 marker=amd-sna6vty2
**amd-sna6vty2** — added `run --mutants --package`, pinned the diff prefixes, and set the rule that every Linux dev-host mutation run takes a NOCOW btrfs `TMPDIR`. Item 1's `TMPDIR` under `~/dev/projects/viola-mutants-scratch/` follows this.

@history_item n=7 marker=amd-pdxmgdpi
**amd-pdxmgdpi** — the mutants profile became `fail-fast { max-fail = 1, terminate = "wait" }`, with the slow-timeout kill bounding any hang below the 20 s floor. This is the profile item 1 moves `path4_…` within.

@history_item n=8 marker=amd-7m34ynih
**amd-7m34ynih** — `run --mutants` uses a fixed `--build-timeout=400` instead of a multiplier. This applies to item 1's addendum invocation.

@history_item n=9 marker=amd-gsat5bin
**amd-gsat5bin** — the `verify_window_` kills moved with the 8.5 s gate under a standing rule: a kill line moves only for a floor that is by design, with a planted-hang control recorded both ways. Item 1's move of a whole-verify test into the class has to carry that control.

@history_item n=10 marker=amd-2iuivqew
**amd-2iuivqew** — the `verify_window_` override stands first in `[profile.mutants]` (first match wins), pinned by `mutants_verify_window_override_stands_before_the_harness_package_override`. A harness test that waits out the gate joined the class BY NAME after its baseline was killed under mutants and the unit went unscored. This is the direct precedent for item 1's `path4_…` rename.

@history_item n=11 marker=amd-3qaxqoxx
**amd-3qaxqoxx** — the first whole-member viola-e2e score (718 mutants, 0 timeout) and the shared root helpers in `tests/support/{…,cli.rs}`. This is the precedent for recording a member's score, which item 1's addendum lands.

@history_item n=12 marker=amd-s6awooo7
**amd-s6awooo7** — host-excluded missed mutants are left out of the verdict, and `survived` = missed − host-excluded + timeout. Item 1's addendum counts are read this way.

@history_item n=13 marker=amd-nkky7uav
**amd-nkky7uav** — the Epoch 4 survivors were disposed by nextest cases in the owning package, some after a behaviour-preserving restatement, each red under a hand-applied mutant and green restored, with no mutation run. A trap is also recorded: `-j 2` jobs sharing one `TMPDIR` read viola-pty's `PortablePty::resize` missed through a shared report file. This is the method for items 2 and 4, and a warning for item 1's `-j 5`.

@history_item n=14 marker=amd-vavqagnr
**amd-vavqagnr** — the 16 `typed.rs` survivors were killed after restatements (named predicates, generic `SharedPty`, plain functions), each shown red and green by hand. This is the precedent for item 2's `pump.rs` restart-path kills.

@history_item n=15 marker=amd-q46zizw5
**amd-q46zizw5** — after the pure-move split of `src/run/send.rs` and `src/human.rs`, the moved unit tests were re-cited with every name and case kept. This is the precedent for items 3, 6 and 7, which require no test edits beyond a re-pointed path or name.

@history_item n=16 marker=amd-4gylryuv
**amd-4gylryuv** — the previous Epoch 5 cleanup: four mutants overrides with `verify_window_` first, a paste-hint class added by name with its own pin test, and each kill moved with a planted-hang TIMEOUT/PASS pair, never by raising the default. This is the same shape as item 1.

@history_item n=17 marker=amd-xafqgfyt
**amd-xafqgfyt** — after a size split the moved units were re-cited (`src/human/board/`, `src/cmd/verify/typed/`), with every case name kept. This is the precedent for item 6's three splits.

@history_item n=18 marker=amd-czzd3xpo
**amd-czzd3xpo** — the Epoch 5 boundary audit's 16 survivors were each disposed by a case in the owning package, 10 killed directly and 6 after a restatement, red under a hand-applied mutant and green when reverted byte-exact, with no mutation run. This is the template for items 2 and 4.

@history_item n=19 marker=amd-3fwhzts6
**amd-3fwhzts6** — a remove-the-guard witness whose neutralised guard lets a product process start must be bounded (by `WITHIN`, through `Running`); an unbounded witness hung 27 minutes. This binds the controls for items 2, 4 and 5 and backs row `decision/bounded-remove-the-guard-witness`.

@history_item n=20 marker=amd-jqhveuv3
**amd-jqhveuv3** — cargo-mutants runs only the mutated package's tests, so a crate's mutants must be killed by its own tests (viola-hyprland's planted stub). Items 2 and 4 need their killing tests inside `viola-pty` and `viola-state`; row `decision/mutants-tested-by-owning-package`.

@history_item n=21 marker=amd-mgtz4fdl
**amd-mgtz4fdl** — the always-admit peer-euid survivor cannot be killed by construction. Item 1's addendum score will meet it.

@history_item n=22 marker=amd-bdohn7zj
**amd-bdohn7zj** — that survivor stands as the version's known limit, read at each epoch-boundary audit. It is not one of this chunk's survivors to kill.

@history_item n=23 marker=amd-edv5gfnm
**amd-edv5gfnm** — two equivalent survivors stand as a known limit beside the peer-admission one. The addendum and the next audit read these too.

@history_item n=24 marker=amd-nssv2hvq
**amd-nssv2hvq** — `mutants.out/outcomes.json` is written at the repo root, and the dev host's `TMPDIR` scratch stays an operator fact that the harness does not set. Item 1 sets `TMPDIR` by hand.

@history_item n=25 marker=amd-fj3n3jnt
**amd-fj3n3jnt** — added `decision/os-gated-cases-in-binary-root`: when a test binary is split, its gated cases stay in the root file. This binds item 6 if a split touches a test binary.

@history_item n=26 marker=amd-fz3qikah
**amd-fz3qikah** — the OS-gated census `tests/contract_os_gated_tests.rs` walks only files directly under `tests/`, and an unnamed or stale gate fails. Any moved gated case in items 3, 6 or 7 must keep its `GATES` row standing.

@history_item n=27 marker=amd-tveox2hq
**amd-tveox2hq** — the root waits went to 12 s and the mutants default kill to 15 s. The 15 s limit is the one item 1 must NOT raise (row `decision/root-wait-bound`).

@history_item n=28 marker=amd-gjhi22be
**amd-gjhi22be** — the mutants default kill moved from 10 s to 15 s as a stated departure from "never raise the default". This is the 15 s kill `path4_…` loses to, and the boundary keeps it unchanged (row `decision/mutants-kill-below-cargo-mutants-timeout`).

@history_item n=29 marker=amd-bqdm7xly
**amd-bqdm7xly** — the `verify_window_` class kill went from 60 s to 80 s in both ci and mutants. Note the scope's "20 s × 4" (80 s) for that override.

@history_item n=30 marker=amd-3ovvlunp
**amd-3ovvlunp** — the `verify_window_` override was widened to also match the fake-agent dialog and framing replay tests. This is a precedent for extending the class beyond the name prefix, which bears on item 1's census of "tests that run a whole verify".

@history_item n=31 marker=amd-sf4g3hkj
**amd-sf4g3hkj** — the two verify-driven cases of `security_negatives_env_capture` moved into the `verify_window_` class, and `decision/verify-driven-tests-20s-kill` gained its exception. This is a direct precedent for item 1's move of a whole-verify test into the class.

@history_item n=32 marker=amd-3y22aq63
**amd-3y22aq63** — revive's strict-modes check moved ahead of any log, and its cases assert no line added. Item 3's shared revive/adopt prologue must keep this order unchanged.

@history_item n=33 marker=amd-64rglrie
**amd-64rglrie** — `run` and `revive`'s start check the home tree before `open_wrapper_log` (`run_refuses_a_widened_home_tree_before_its_log`). This is the ordering item 3 consolidates and the guard of row `decision/run-checks-before-child-spawn`.

@history_item n=34 marker=amd-fwkd577g
**amd-fwkd577g** — revive's preflight has four readings including `session-live`, with `preflight_order_*` units. This is the prologue content item 3 moves without changing behaviour.

@history_item n=35 marker=amd-uztwp2ux
**amd-uztwp2ux** — viola adopt shares revive's session-live reading and the silent internal-error role. It is the origin of the adopt copy that item 3 dedupes.

@history_item n=36 marker=amd-lymf32fj
**amd-lymf32fj** — viola adopt joined the strict-modes refusals that write no line. Row `contract/strict-modes-refusal/no-line` must stay green across item 3's move.

@history_item n=37 marker=amd-rhpvycfa
**amd-rhpvycfa** — the start's socket-directory check is shared by `revive`'s start, with witnesses in `security_negatives_socket_dir.rs` and `cross_exit_causes.rs`. This is part of the shared start path that items 3 and 6 (run.rs) touch.

@history_item n=38 marker=amd-dtswm36t
**amd-dtswm36t** — `revive --json` refusal codes, including start refusals after preflight, plus the `pump_child` no-document rule. These pins must stay unedited across item 3.

@history_item n=39 marker=amd-dzdiejp7
**amd-dzdiejp7** — added `decision/run-start-verify-and-trust-after-bind`: the start's automatic verify and trust steps sit after the bind and before the first snapshot. This is the ordering item 6 must preserve when it splits the window and trust steps out of `src/cmd/run.rs`.

@history_item n=40 marker=amd-2batlkms
**amd-2batlkms** — a placing open's kept view is proven only by the gated live reading R7. This is the latest change to the window step in `run.rs` that item 6 moves (no rework of the window code).

@history_item n=41 marker=amd-aqoh4tur
**amd-aqoh4tur** — viola switch: send is refused turn-running from a switch's request until the new child's session-start, and there are nine frame verbs. The switch restart is what drives `pump.rs`'s restart path, the code item 2's survivors and item 6's pump split live in.

@history_item n=42 marker=amd-zfsdx5lb
**amd-zfsdx5lb** — the PTY seam runs on openpty alone, exit is detected on the handle, and the `TerminateProcess` fallback is gone. This is the current `viola-pty` shape behind the `reap_within` / `drain` survivors (rows `decision/child-exit-read-on-the-process-handle`, `decision/pty-exit-from-wait-not-eof`).

@history_item n=43 marker=amd-fyqhu6pl
**amd-fyqhu6pl** — added `decision/live-rig-clean-start` after R2's red was traced to rig faults. This is the R2 open script's history, which item 5 extends with lock and fresh-scratch preconditions.

@history_item n=44 marker=amd-2o2uut5r
**amd-2o2uut5r** — added `decision/window-placement-proven-live`: viola open's real placement is proven only by a live reading. This is the R2 reading whose script item 5 mends, and item 5's check must run no live arm.

@history_item n=45 marker=amd-trdsiny2
**amd-trdsiny2** — added `decision/window-reopen-proven-live`: the gated R4 rig refuses first until granted, and its rehearse form is never a live reading. Its sibling scripts carry the preconditions item 5 copies into `r2-open.sh`.

@history_item n=46 marker=amd-ghzjwcs6
**amd-ghzjwcs6** — added `decision/refusal-rehearsal-reads-own-effects`: a rehearsal of a refusal reads the refusal's own effects (no ledger, no process started). This fits item 5's check, which shows `r2-open.sh` refusing on a locked desktop or a stale scratch dir.

@history_item n=47 marker=amd-h4gpb7h6
**amd-h4gpb7h6** — `decision/real-claude-local-only` names the gated live-reading scripts as never run by a gate in live form. Item 5's check must use a rehearsal or a `hyprctl` stub, never the live arm.

@history_item n=48 marker=amd-duln5oqr
**amd-duln5oqr** — added `decision/live-rig-root-outside-repositories` and the `viola up` board-scan strict-modes rule (`decision/strict-modes-before-any-log`). Both bound item 5's scratch-dir precondition and item 6's `upped` split in `src/cmd/up.rs`.

@history_item n=49 marker=amd-7uli7gv4
**amd-7uli7gv4** — added `decision/echo-oracle-skips-allowed-word`, which came from viola open's refusal oracles. Item 7's shared `refused()` helper for open and up keeps the `--json` refusal shape those oracles read.

@history_item n=50 marker=amd-qqdv5cf3
**amd-qqdv5cf3** — the E2E layer regex `^(path[0-9]*|tui|mcp|cross|chaos|contract)_` and its integration complement (row `decision/e2e-layer-by-binary-prefix`). This binds where a moved or split test binary reports, and the boot-path binaries item 9 may declare.
