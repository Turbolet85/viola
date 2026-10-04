# obs extract

## Relevance
partial — the chunk adds no product telemetry, but it scores obs code under cargo-mutants (`src/panic_frames.rs` `raw_frames` / `module_of` ×2 are the §7 panic-hook backtrace path), reshapes `run --mutants` (whose `chunk.diff` and `mutants.out/` the obs-plan rules), and moves the `viola-harness pre-push` document, whose content the obs-plan bounds.

## Constraints
- Obs code (`obs_event!` call sites, `MillisUtc`, the panic hook, TraceLayer closures) is mutation-scored like product code. A missed or timed-out obs-code mutant, or more unviable than caught mutants, makes that `run --mutants` red (per obs-plan §9 Pipeline integration, Mutation row; §10 Build / deploy failure conditions, "a surviving cargo-mutants mutant in obs code"). The three `src/panic_frames.rs` cfg(unix) mutants (`raw_frames`, `module_of` ×2) are obs code, so each survivor is killed or recorded as equivalent, and none is left open. Whether existing tests already kill them on Linux is research's question.
- `src/panic_frames.rs` on Unix must produce raw, never-symbolised frames from `libc::backtrace` + `dladdr`: up to 62 strings of the form `0x<ip> <module path> base=0x<base> +0x<offset>` or `0x<ip> ?`. They go only to the one-line `backtrace` array in `detail-<process>.ndjson`, and are never gated by `RUST_BACKTRACE` (per obs-plan §7 Panic hooks). A test written to kill a `module_of` / `raw_frames` mutant asserts this shape. It must not move the capture onto a symbolising path (the §7 retirement of `Backtrace::force_capture()` on perf grounds).
- The `viola-harness pre-push` document is never uploaded and carries no absolute path: no repository or home path, and no Linux clone or home path. A `pre_push_` unit test and the gate's verdict path-grep entry assert this (per obs-plan §8 PII Scrubbing item 6). It still binds when the stage runs natively on the Linux host, where the repository and home paths are host paths. The §8 wording names the retired WSL clone paths (`~/viola-pre-push`, `~/.cargo`), so it reconciles at wrap and is not edited in phase.
- `mutants.out/` is never uploaded, and the host mutation scratch path is never printed: documents carry only the byte count `scratch_bytes`. `target/run-archive/<n>/` is named only by the repo-relative `archived` (per obs-plan §8 item 6, Unscanned uploads). Whatever form the viola-e2e boundary tier takes (`--copy-target=true`, the prebuild, or a test seam), it must keep both. The §8 text places the scratch "on a Windows host". Where it lives on the Linux host is research's question.
- `run --mutants`' diff stays at exactly `target/agent-run/chunk.diff`, the one path the secret scan skips and the `harness-<os>` upload excludes (per obs-plan §8 item 6; §9 Step order step 3). The C3 prefix pin (`--src-prefix=a/ --dst-prefix=b/`) changes only the header bytes, not where the diff is written.
- `viola-harness` (`crates/viola-e2e`) and the fake agent are the only members exempt from the print bans: the harness prints exactly one JSON document per command on stdout. No product crate gains a print macro through the gate-tool migration (per obs-plan §11 Logs; §3 Bootstrap phases obs-ci-gate-wire).

## Patterns to follow
- M1's survivor disposition: each survivor is killed by a new or strengthened test, or recorded as equivalent with its argument. This is the form that satisfies the §9 Mutation row's red condition for obs code (per obs-plan §9 Pipeline integration).
- Every verdict is a machine exit code: the harness document plus a `jq`-readable verdict, never human-reviewed output (per obs-plan §11 CI, "NEVER use human-review-gated log analysis"; §11 Universal).
- No-absolute-path assertions are paired: a `pre_push_` unit test plus the gate's path-grep entry (per obs-plan §8 item 6).

## Anti-patterns to avoid
- NEVER let an obs-code mutant survive undisposed, or record a not-measurable mutant without its reason (per obs-plan §10 Build / deploy failure conditions). This covers the 13th, `sideload_outcome`, if it sits on an obs-visible path.
- NEVER print the host mutation scratch, a clone path or a home path in a harness or pre-push document (per obs-plan §8 item 6).
- NEVER add retry-once to make a red harness or mutation test go green. The C3 13-red is fixed at its cause (per obs-plan §11 SLO; §10 Cross-input parallel).

## Contract bindings
- obs ↔ tests §10 Mutation gate: the §9 Mutation row defers the red/green rule for obs-code mutants to the test-plan's Mutation gate. This chunk's scoring of `src/panic_frames.rs` is judged there.
- obs ↔ tests §3 harness: the `pre-push` and `run --mutants` documents are harness output, so obs's no-absolute-path and `scratch_bytes`-only rules (§8 item 6) bind the harness document shape.
- obs ↔ security: the native stage's `env -i` HOME+PATH boundary is security's to rule on. Obs's adjacent rule is §11 CI, "NEVER inject CI env metadata into product log lines". No host env value may reach a product log line through the migrated gates.
- obs spec reconciliation: obs-plan §8 item 6 names the WSL-era clone paths and the "Windows host" scratch placement. It reconciles at this chunk's wrap (scope item 4), not in phase.

## Acceptance criteria contributions
- (obs) Every `cargo mutants` outcome for the 12 cfg(unix) mutants that falls in `src/panic_frames.rs` is caught, or recorded as equivalent with its argument. No obs-code mutant is missed or timed out at the chunk's `run --mutants` (per obs-plan §9 Pipeline integration, Mutation row; §10 Build / deploy failure conditions).
- (obs) The migrated `viola-harness pre-push` document, run natively on the Linux host, contains no absolute path: no repository path, no home path, and no `~/`-rooted clone or cargo path. A `pre_push_` unit test asserts it (per obs-plan §8 item 6).
- (obs) The viola-e2e boundary-tier `run --mutants` document names the mutation scratch only by `scratch_bytes` and the archive only by the repo-relative `archived`. Its diff lands at exactly `target/agent-run/chunk.diff` (per obs-plan §8 item 6; §9 Step order step 3).
