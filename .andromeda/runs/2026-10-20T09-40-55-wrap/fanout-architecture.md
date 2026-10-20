proposals:
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources
    change: "The `target/profraw-census/` entry adds the census's second form beside the unchanged first: `scripts/profraw-census.sh --case <binary stem> <test name> <runs> <workers> [<name>]` runs the named test of the named instrumented test binary; its profiles are named `r<i>-%p-%<nproc>m.profraw`, each offered alone to the toolchain's `llvm-profdata merge --sparse -o /dev/null`; a refused file is kept under `refused/` and printed `profraw-census: refused <file> · signature <sig> · <size> B of <max> B`; its last line is `profraw-census: runs N · passed N · profiles P · refused R`, exit 0 only when every run passed and R is 0, else 1, and 2 on a usage error or with no instrumented binary or no `llvm-profdata`. Still no CI job, `pre-push` stage or harness command runs either form."
    sidecar: "2026-10-20-epoch-5-cleanup: §Occupied Resources `target/profraw-census/` — the census's `--case` form (named test, per-profile `llvm-profdata merge` check, `refused/`, its last line and exits) added; the first form's usage, line and exits kept."
    rationale: "Report Changes › Symbols / APIs (`scripts/profraw-census.sh`: the second form `--case …` with its profile naming, `refused/`, last line and exits) and Counts / qualifiers moved (F6/F13: «architecture.md `:395` — the census's usage, its last line and its exits are the first form only»); the form writes a new `refused/` dir under the registered `target/profraw-census/` resource."
    basis: "architecture.md:395; scripts/profraw-census.sh:30-37, 106-135, 150-166"
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → Project directory structure
    change: "The `scripts/profraw-census.sh` tree comment names both forms: `<runs> <workers> [<name>]` (the start test N times on the instrumented root-bin test binary) and `--case <binary stem> <test name> <runs> <workers> [<name>]` (the named test, each profile checked alone by `llvm-profdata merge`, refused files kept), both → target/profraw-census/ (no CI job, pre-push stage or harness command)."
    sidecar: "2026-10-20-epoch-5-cleanup: Project directory structure — the `profraw-census.sh` comment gains the `--case` form beside «the start test N times»."
    rationale: "Report Counts / qualifiers moved (F6/F13): «`registries/contracts/architecture/project-directory-structure.md:98`, `:100` — «the start test N times»: the first form only»; the same retired first-form-only claim as the §Occupied Resources entry."
    basis: "registries/contracts/architecture/project-directory-structure.md:98-100"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources
    change: "The `target/e2e-home` keepers' contract adds a fourth check on the Unix link branch after its three existing ones: the backing's free space (`statvfs`, `f_bavail × f_frsize`) below `HEADROOM_FLOOR_MIB` = 9 GiB (the EDQUOT reading of about 8 GiB plus 1 GiB, `evidence/headroom.md`) refuses the start with `e2e-home backing: {free} MiB free, below the {floor} MiB floor` (two counts, no path) — the root keeper `prepare_home_base` panics with it, and `Workspace::ensure_e2e_home` returns it as an `InvalidInput` error that reads the existing `build-failed` at `boot` and `verify-exit-none` at `run --local-live` (no reason added); no variable, flag or config key sets the floor. The «fixed message that names no path» wording then covers the three earlier refusals, the headroom message carrying two counts and still no path."
    sidecar: "2026-10-20-epoch-5-cleanup: §Occupied Resources `target/e2e-home` — the two keepers' headroom floor (9 GiB, statvfs, two-count message, existing closed reasons) added as their fourth check."
    rationale: "Report Changes › Symbols / APIs (`tests/support/home.rs` `HEADROOM_FLOOR_MIB` / `headroom_refusal` / `free_bytes`, `prepare_home_base`'s link branch; `crates/viola-e2e/src/harness/mod.rs` the same pair and `ensure_e2e_home`'s `InvalidInput`) and Counts / qualifiers moved (F8-headroom: «architecture.md `:371` … and `:388 @c1300,1390` («is accepted only when one `lstat` reads a real directory with no group or other bit. Anything else refuses…»)», the keepers' checks three → four); Deviations 5 (the 9 GiB basis)."
    basis: "architecture.md:388; tests/support/home.rs:138-140, 154-159, 161-179; crates/viola-e2e/src/harness/mod.rs:120-122, 148-153, 155-172"
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources
    change: "The `/tmp/viola-e2e-home-<uid>/` entry says the keepers `prepare_home_base` and `Workspace::ensure_e2e_home` refuse a target that is not a real, owner-only directory, or whose backing reads less than the 9 GiB headroom floor free (§Repository's `target/e2e-home` entry)."
    sidecar: "2026-10-20-epoch-5-cleanup: §Occupied Resources `/tmp/viola-e2e-home-<uid>/` — the keepers' refusal list gains the headroom floor."
    rationale: "Report Counts / qualifiers moved (F8-headroom): architecture.md `:371` («refuse a target that is not a real, owner-only directory») is stale-incomplete, listing the keepers' checks without the floor; the same retired three-check claim as the `:388` proposal."
    basis: "architecture.md:371"
    dependent-of: D-arch-resources
note: "D-arch-decisions holds: the one dependency move (`viola-e2e` → libc `=0.2.189`, Unix, workspace pin already in the graph) adds no library or version and contradicts no decision — crate-dependency-direction lists no `viola-e2e` dependencies, and §Stack's syn/proc-macro2 clause names only those two. No new IPC method, endpoint, event, socket, port, env var or crate landed. D-retired: the removed `.claude/docs/services/viola-ui.md` has no architecture site (no `services/`, `viola-ui` or `.claude/docs` hit in architecture.md or its key files; build-system.md:15's `e2e-web` line is history). The F8 refusal list and the census form are filed under D-arch-resources as stale registrations of registered resources; the orchestrator may reclassify them to the cascade's moved-fact channel."
