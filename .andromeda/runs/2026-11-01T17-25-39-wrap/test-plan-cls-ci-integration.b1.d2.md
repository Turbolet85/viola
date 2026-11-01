# Classified — test-plan · ci-integration.b1

## Answers
ci-integration#1.1 · no · the lint commands, the sync-crates target set and the lib and bin orphans count hold on the reached Cargo.toml and crates, the new [[test]] target is neither lib nor bin, and no dependency moved
ci-integration#1.2 · no · the deny, tokio-ban and deny-probe commands rest on deny.toml and the normal dependency graph, which the session leaves unchanged since no dependency moved and the source edits add no edge
ci-integration#1.3 · no · the unit command and nextest process-per-test hold on the changed .config/nextest.toml, and the new unit cases name no count
ci-integration#1.4 · no · the integration command and its parallel mode hold, and the entity names no binary or count that the new test binary moves
ci-integration#1.5 · no · the e2e stage still runs on all three OSes, and the Unix-only new binary is not named by this entity
ci-integration#1.6 · no · the coverage run still covers the conpty_sideload [[test]] alongside the rest on the changed Cargo.toml, the entity enumerates no binaries and the thresholds are unmoved
ci-integration#1.7 · no · the ten mutant jobs and the five root-package items split by file hold, since the session adds no matrix item and no source file, and the mutation gate arithmetic is unmoved
ci-integration#1.8 · no · the perf command, the hook export and the schema-check step still run, the session moves no perf value and the entity names no enum or hook timing
ci-integration#1.9 · no · the 1.96 toolchain floor and the workspace rust-version hold, as no toolchain, dependency or tool version moved
ci-integration#1.10 · no · the fuzz replay command names only the fuzz targets, which the session does not touch
ci-integration#1.11 · no · the release build names only --bin viola, which a fake-agent [[test]] table is not built by, and --locked holds with no dependency moved
ci-integration#1.12 · no · the gate reads the junit, llvm-cov and hyperfine outputs against the §10 thresholds, all unmoved by the session
