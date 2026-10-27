# security · picked accounting · group 1 of 2

## Picked
threat-model-summary#1.3 · carried · the pinned executable, the plugin exec paths and the snapshot's `pinned_bin` are the integrity-sensitive files the absent-copy write and the rebuild measurement reach
threat-model-summary#1.5 · carried · the measurement's evidence (embedded build paths, section dumps) must hold no absolute path carrying the OS username
threat-model-summary#5.1 · carried · the race's fault is the agent's hook spawn of the pinned path, and the hook stays fail-open with no hook-side refusal added
threat-model-summary#7.1 · carried · the chunk re-asserts the home-tree check made by `run`, `revive`'s start and `verify` before the pin
threat-model-summary#8.1 · carried · the code-bearing artefacts Claude Code runs, never deleted, held to the home-tree check before the log, are what the race writes
threat-model-summary#10.1 · carried · the toolchain and lockfile are inputs of the rebuild hash the chunk measures, and v1 adds no release stage
threat-model-summary#11.1 · carried · the per-OS release build through `scripts/release-check.sh` is the measurement's build route
authentication-authorization#1.8 · carried · `replace_private` (edited) sets the mode before any byte and the strict-modes write rule must pass whatever a concurrent start leaves under `bin/<key>/`
authentication-authorization#1.9 · carried · `check_home_tree` through `checked_exe` runs before the pin and is re-asserted unchanged
input-validation#1.4 · not in scope · hook stdin parsing and the capture arms are untouched; the race fault is a failed spawn before any stdin read
input-validation#1.6 · carried · `verify`'s home-tree check precedes its edited `pin_exe` call and `revive`'s start reaches the pin through `run`'s `checked_exe`
input-validation#1.12 · carried · P3 re-reads the stamps and snapshot readers for which key on the build hash, each behind the strict-modes check
input-validation#1.20 · carried · `pin_companions`' write-if-absent, held open and re-hash are the model the Windows exe arm is compared against
input-validation#2.0 · not in scope · the chunk adds no external reader; the pin reads the exe's own bytes
data-protection#1.1 · carried · the pinned copy's 0700 mode, hash-before-reuse, the plugin rewrite's same-bytes acceptance and the owed race line are the rules the fix rules over
dependency-security#1.1 · not in scope · the vendored ConPTY audit is not reached; the companions' bytes do not change
dependency-security#4.1 · not in scope · the four ConPTY pins do not change; `pin_companions` reads them unchanged
bootstrap-phases#1.1 · carried · the pinned exe's SHA-256 re-hash and the per-start plugin rewrite with absolute pinned paths must hold under the concurrent first start
