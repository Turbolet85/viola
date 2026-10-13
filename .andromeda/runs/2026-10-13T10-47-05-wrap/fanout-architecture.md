proposals:
  - detector: D-arch-resources
    severity: warning
    section: "§Infrastructure Patterns → Project directory structure"
    change: "Replace the `human.rs` row: `src/human.rs` is the `human` module root (the shared writers `write_refusal` / `refuse`, `write_internal_error` / `internal_error`, `write_result` / `result`, and the `pub(crate) use` re-exports, so no caller path changed), with its children under `src/human/`: `mirror.rs` (`send`'s readback mirror `[  ] open` / `[RB] read back` / `[  ] unconfirmable` / `[/ ] unable`, `write_wrapper_fault`, `write_unable`), `escape.rs` (`escape_message`, `escape_row`), `board.rs` (`instance_state_pair`, `home_state_pair`, `BayBudget`, `bay_line`, `board_rows`, `write_board`), `wait.rs` (the `wait` / `last` lines), `verbs.rs` (the `answer`, `pause` / `release` and budget lift/open lines), `hints.rs` (`HUMAN_TYPING_HINT`, `answer_hint`, `send_hint`); the caller list unchanged."
    sidecar: "Project directory structure: `src/human.rs` row rewritten as the module root plus its six children under `src/human/` (chunk 2026-10-13-epoch-4-cleanup split)."
    rationale: "Report Changes › Symbols / APIs and › Crates / modules: `src/human.rs` is now the module root keeping only the shared writers and re-exports; mirror, escape, board, wait, verbs and hints moved to `src/human/{mirror,escape,board,wait,verbs,hints}.rs`. The row still describes one file holding all human-facing text; the report's Expected amendments name this site."
    basis: ".andromeda/registries/contracts/architecture/project-directory-structure.md:21; src/human.rs:17-28; src/human/board.rs:142-168 @144; src/human/hints.rs:23-52 @26"
  - detector: D-arch-resources
    severity: warning
    section: "§Infrastructure Patterns → Project directory structure"
    change: "Replace the `run/` `send.rs` row: `send.rs` is the `send` module root (the wrapper's `send` method on `Ctx` with its refusal order; re-exports `parse_from`, `SendSlot`, `append_hook_event`), with its children under `run/send/`: `slot.rs` (the one-in-flight `SendSlot` with the remembered session id, `claim` / `settle` / `confirm`, and `append_hook_event`: the driver relabel and the `session-start` post-condition claim), `classify.rs` (`Match`, `Confirms`, `classify`: the local-command decision over `LOCAL_COMMANDS`), `params.rs` (`parse`, `parse_from`), `budget.rs` (`Ctx::budget_paused`), `relabel.rs` (only the `#[cfg(test)]` prompt-match and relabel cases) and the `#[cfg(test)]` `test_support.rs`."
    sidecar: "Project directory structure: `run/send.rs` row rewritten as the module root plus its children under `run/send/` (chunk 2026-10-13-epoch-4-cleanup split)."
    rationale: "Report Changes › Symbols / APIs and › Crates / modules: `src/run/send.rs` keeps `Ctx` and `send`; the slot, claim/settle, `append_hook_event` moved to `slot.rs`, the local-command decision to `classify.rs`, `parse` / `parse_from` to `params.rs`, `budget_paused` to `budget.rs`, plus `relabel.rs` and `test_support.rs`. The row still places the slot, the local-command decision, the relabel and the claim in `send.rs`; the report's Expected amendments name this site."
    basis: ".andromeda/registries/contracts/architecture/project-directory-structure.md:31; src/run/send/slot.rs:39-160, 162-204 @169; src/run/send/classify.rs:25-33 @27; src/run/send/params.rs:13-22 @14; src/run/send/budget.rs:5-16"
  - detector: D-arch-resources
    severity: warning
    section: "§Infrastructure Patterns → Build system"
    change: "In the Lint bullet, `The root bin carries no print allow: its only human writer is src/human.rs` now reads: its only human writer is the `human` module (`src/human.rs`, which keeps `write_refusal`, `write_internal_error`, `write_result` and their wrappers, and its children under `src/human/`); the rest of the bullet unchanged."
    sidecar: "Build system: the root bin's only human writer is the `human` module (`src/human.rs` + `src/human/`), after the chunk 2026-10-13-epoch-4-cleanup split."
    rationale: "Report Changes › Symbols / APIs: the human writers now span `src/human/` (e.g. `escape_message`, `write_unable`, `write_answered`, `answer_hint` named in this bullet moved to children); the report's Expected amendments list `build-system.md:4` as a site stating the moved fact. Same retired claim as the `human.rs` directory row."
    basis: ".andromeda/registries/contracts/architecture/build-system.md:4"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Established Decisions → [CLI Version Compatibility]"
    change: "The citation `(send_hint, src/human.rs)` reads `(send_hint, src/human.rs; in src/human/hints.rs since chunk 2026-10-13-epoch-4-cleanup)`; wording, exit code and refusal unchanged."
    sidecar: "[CLI Version Compatibility]: `send_hint` cited at `src/human/hints.rs` after the chunk 2026-10-13-epoch-4-cleanup split."
    rationale: "Report Changes › Symbols / APIs: `send_hint` (product) moved to `src/human/hints.rs`; the report's Expected amendments name `architecture.md:91` (`send_hint`, `src/human.rs` → `src/human/hints.rs`). Same retired claim as the `human.rs` directory row."
    basis: ".andromeda/architecture.md:91; src/human/hints.rs:23-52 @26"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Established Decisions → [Delivery Confirmation]"
    change: "The sentence `As landed (src/run/send.rs), the wrapper claims the in-flight send's matching prompt …` cites `(src/run/send.rs; since chunk 2026-10-13-epoch-4-cleanup the claim, the driver relabel and the settle sit in src/run/send/slot.rs: SendSlot::claim / settle and append_hook_event)`; the ordering claim unchanged."
    sidecar: "[Delivery Confirmation]: the prompt claim / relabel / settle cited at `src/run/send/slot.rs` after the chunk 2026-10-13-epoch-4-cleanup split."
    rationale: "Report Changes › Symbols / APIs: `SendSlot` and its methods incl. `claim` / `settle` / `confirm`, and `append_hook_event`, moved to `src/run/send/slot.rs`; the report's Expected amendments name `architecture.md:49` (the claim / relabel / settle, now `slot.rs` and the root's `send`). Same retired claim as the `send.rs` directory row."
    basis: ".andromeda/architecture.md:49; src/run/send/slot.rs:76-91 @79, 119-130 @120, 162-204 @169"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Established Decisions → [Delivery Confirmation]"
    change: "The citation `(as landed by chunk 2026-10-06-local-command-send-outcomes, src/run/send.rs)` for the local-command decision reads `(as landed by chunk 2026-10-06-local-command-send-outcomes, src/run/send.rs; since chunk 2026-10-13-epoch-4-cleanup the decision is classify in src/run/send/classify.rs)`; the exact-equality rule unchanged."
    sidecar: "[Delivery Confirmation]: the local-command decision cited at `src/run/send/classify.rs` (`classify`) after the chunk 2026-10-13-epoch-4-cleanup split."
    rationale: "Report Changes › Symbols / APIs: `Match`, `Confirms`, `classify` (the local-command decision) moved to `src/run/send/classify.rs`; the report's Expected amendments name `architecture.md:49` (the local-command decision `as landed … src/run/send.rs`, now `classify.rs`). Same retired claim as the `send.rs` directory row."
    basis: ".andromeda/architecture.md:49; src/run/send/classify.rs:25-33 @27"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources"
    change: "In the `<repo parent>/viola-mutants-scratch/` row, `The scratch is measured, removed and recreated before every counted, scoped or package run` reads: measured, removed and recreated owner-only through `viola_state::fs::create_private_dir` (on a Windows host the protected `D:P(A;OICI;FA;;;<user>)(A;OICI;FA;;;SY)` DACL on the topmost folder it creates outside the profile, so every temp copy laid inside inherits owner-only and an instance tree inside it passes `check_instance`; 0700 elsewhere, where `prepare` never reaches the wipe), witnessed on CI's `test (windows-2025)` leg by `wipe_scratch_takes_an_instance_tree_the_check_accepts_where_a_plain_dir_does_not`; whether the root package's Windows mutation baseline reads green is not yet measured (the next `windows-mutants.yml` dispatch)."
    sidecar: "Occupied Resources: the host mutation scratch is re-created owner-only (`create_private_dir`, protected DACL on Windows) at each wipe — chunk 2026-10-13-epoch-4-cleanup."
    rationale: "Report Changes › Symbols / APIs (`scratch.rs` `wipe` now re-creates the scratch through `viola_state::fs::create_private_dir`, was `fs::create_dir_all`) and › Harness / gate surface (on a Windows host `run --mutants` now re-creates `<repo parent>/viola-mutants-scratch` owner-only); › Cross-project (CI ci#38375200217, the witness PASSED); › Insufficient fixes (the baseline's green reading NOT measured). The registered row still describes a plain recreate; the report's Expected amendments name `architecture.md:452`."
    basis: ".andromeda/architecture.md:452; crates/viola-e2e/src/harness/run/mutants/scratch.rs:62-64, 252-281 @258"
note: "No new IPC method, endpoint, event, socket, port, env var or workspace crate (report › Crates / modules: no crate added); the new `viola-e2e` → `viola-state` path dependency adds no crate to Cargo.lock and is within the Crate dependency direction contract, which leaves viola-e2e's dependencies unlisted — no D-arch-decisions drift. The Windows-target clippy (`CARGO_TARGET_DIR=target/wincheck … --target x86_64-pc-windows-msvc`) is a chunk gate entry only, neither pre-push nor CI, so Build system's Lint bullet and §Occupied Resources take no row for it or for `target/wincheck/`; the report places it on test-plan §9's Lint row."
