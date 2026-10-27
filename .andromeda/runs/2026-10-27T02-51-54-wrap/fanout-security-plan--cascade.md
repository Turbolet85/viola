proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/data-protection.md
    near: 12
    section: "## Data Protection"
    change: "In the plain-home sentence replace «the refused entry an inherited Users (`S-1-5-32-545`) write ACE» with «the refused entry an inherited allow ACE granting a write right to a SID the check does not trust, whichever SID holds it (on one `windows-2025` runner Authenticated Users `S-1-5-11`, mask `0x001301BF`, held the write while Users held read only)»; the verdict `Writable` and the rest of the line stand."
    sidecar: "data-protection: the Windows plain home's refused entry is an inherited untrusted write grant read by its right, not a fixed Users ACE (measured disproved, chunk 2026-10-27-pinned-copy-race-and-reproducible-builds)."
    rationale: "Report §Spec claims disproved by measurement: on ci#40425866096 Users held read only and Authenticated Users held the inherited write; the product verdict `Err(Writable)` was right and only the oracle naming `S-1-5-32-545` failed; the report marks this site changed."
    basis: .andromeda/security-plan-detail/data-protection.md:12
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/data-protection.md
    near: 17
    section: "## Data Protection"
    change: "Drop the sentence «The `pin_exe` first-start race and reproducible builds are owed to the route entry \"Pinned copy race and reproducible builds\".» and end the line with «The first-start race is closed by the no-clobber placement and reproducible builds are read by measurement B, both below.»; the `pin_bytes` / `pin_key` / `pin_exe` delegation clause stands."
    sidecar: "data-protection: the owed line on the pin_exe first-start race and reproducible builds is closed by chunk 2026-10-27-pinned-copy-race-and-reproducible-builds."
    rationale: "Report §Expected amendments, security-plan §Data Protection: the owed line is closed, naming the race's fix and measurement B's reading (carried; `owner-title` and `repro-builds` each 1 standing, this line, changed)."
    basis: .andromeda/security-plan-detail/data-protection.md:17
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/data-protection.md
    near: 17
    section: "## Data Protection"
    change: "Add the sub-bullet «    - The pinned copy's absent-copy write is a no-clobber placement through `viola_state::fs::place_private_new`, never a replace: a temp file beside the target (`NamedTempFile::new_in`), its mode set before any byte, the bytes written, `sync_all`, the write handle closed (`into_temp_path`), then `persist_noclobber`. `replace_private_shared` keeps only the plugin files and the ConPTY companions.»"
    sidecar: "data-protection: new detail, the pinned copy placed by place_private_new (closed temp file, persist_noclobber), never replaced."
    rationale: "Report §Changes, Symbols / APIs: `place_private_new` (fs.rs:316-324) and `pin_bytes`' `NotFound` arm now calling `place_absent` (pin.rs:108-116) instead of `replace_private_shared`, whose doc comment no longer names the pinned copy."
    basis: crates/viola-state/src/fs.rs:316-324
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/data-protection.md
    near: 17
    section: "## Data Protection"
    change: "Add the sub-bullet «    - A present name is never replaced: `AlreadyExists` is `Placed::Present`, the target untouched and the temp file removed; any other failure is an error that leaves the target as it was, with no retry and no new error variant.»"
    sidecar: "data-protection: new detail, a present pinned-copy name is never replaced (Placed::Present, target untouched, temp removed, no retry)."
    rationale: "Report §Changes, Symbols / APIs (`AlreadyExists` is `Ok(Placed::Present)`) and §Outcome: `…_over_a_present_name_is_present_and_leaves_it` gives `Present`, the target byte-identical, no temp left."
    basis: crates/viola-state/src/fs.rs:339-346
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/data-protection.md
    near: 17
    section: "## Data Protection"
    change: "Add the sub-bullet «    - On a lost placement (`Present`: a concurrent first start placed the copy first) `pin_bytes` re-hashes the copy that stands: equal bytes are reused, different bytes are `PinError::HashMismatch` with the file left as found, never overwritten or deleted; `run.pin_copy` keeps `ok|pinned-hash-mismatch` and a lost placement reads `ok`.»"
    sidecar: "data-protection: new detail, a lost pinned-copy placement re-hashes the standing copy (reuse on equal, HashMismatch left as found on different)."
    rationale: "Report §Changes, Symbols / APIs (`place_absent`) and §Schema / config; §Outcome: `place_absent_over_a_copy_another_start_placed` (`same_bytes` → `Ok`, `other_bytes` → `HashMismatch`, the file byte-identical) green."
    basis: crates/viola-state/src/pin.rs:108-116
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/data-protection.md
    near: 17
    section: "## Data Protection"
    change: "Add the sub-bullet «    - Measurement B: clean release builds of one tree can give two different files whose loaded sections are all equal, differing only in `.note.gnu.build-id`, `.symtab` and `.strtab` (three `anon.<hash>.<n>.llvm.<number>` local symbols); no cause is claimed, so one tree can key two `bin/<key>/` and `plugin/<key>/` folders. Acting on it is the founder's, carried on «Version done-check».»"
    sidecar: "data-protection: new detail, measurement B, one tree can build two files with equal loaded sections and so key two bin/plugin folders; cause unexplained, a founder item."
    rationale: "Report §Spec claims disproved by measurement (measurement B: six cold builds, two files, 3 of 34 sections unequal, none loaded) and §Decisions & corrections item 4 (founder item, CARRY on «Version done-check»)."
    basis: .andromeda/security-plan-detail/data-protection.md:17
    fact: F5
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 33
    section: "## Authentication & Authorization"
    change: "In the Unix implementation sentence, after «then `sync_all`s and `persist`s it into place.» add «The pinned `viola`'s absent-copy write is no replace: `viola_state::fs::place_private_new` sets the temp file's mode (0700) the same way before any byte, `sync_all`s it, closes its handle and places it with `persist_noclobber`, never over a present name.» and drop «, or 0700 for the pinned `viola`» from the `replace_private` clause."
    sidecar: "auth: ~/.viola/ access control names place_private_new as the pinned viola's mode-before-any-byte write, no longer replace_private."
    rationale: "Report §Changes, Symbols / APIs: `pin_bytes`' `NotFound` arm moved from `replace_private_shared` to `place_absent` / `place_private_new`, which runs `restrict` before any byte; §Outcome: mode set before any byte (`restrict` before `write_all`, fs.rs:326-347), 0700 / 0600 on Unix."
    basis: crates/viola-state/src/fs.rs:326-347
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 42
    section: "## Authentication & Authorization"
    change: "Stack reference: after «tempfile 3.27.0 `persist` via `viola_state::fs::replace_private`» add «; `into_temp_path` + `persist_noclobber` via `viola_state::fs::place_private_new` (the pinned `viola`)»."
    sidecar: "auth: the access-control stack reference adds tempfile's into_temp_path + persist_noclobber through place_private_new."
    rationale: "Report §Changes, Dependencies: `tempfile 3.27.0` already holds `persist_noclobber` and `into_temp_path`, now used by `place_private_new` (fs.rs:339-346)."
    basis: crates/viola-state/src/fs.rs:339-346
    dependent-of: D-cascade
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 34
    section: "## Authentication & Authorization"
    change: "After the `cfg(windows)` case «a plain home made before viola refused `Writable`» add «(read by the write right whichever SID holds it, with a control: a `windows-2025` runner's inherited temp-dir ACL differs by host, Users holding the write on one and Authenticated Users on another while Users held read only, so a case naming one fixed SID depends on the runner)»."
    sidecar: "auth: the Windows plain-home refusal is witnessed by the inherited untrusted write right, whichever SID holds it, since runner ACLs differ by host."
    rationale: "Report §Decisions & corrections (the windows-2025 runners differ in a temp dir's inherited ACL by host, westus3 Users write, northcentralus Authenticated Users write) and §Outcome red 2: the case reads the grant by its right with a viola-made-home control, green on ci#40439188053."
    basis: crates/viola-state/src/strict.rs:746-760
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 128
    section: "## Threat Model Summary"
    change: "After «The pinned binary and plugin folders are never deleted in v1.» add the sub-bullet «    - The pinned binary is placed once and never replaced: two concurrent first starts once raced to exec the copy while another start still held it open for write (`ETXTBSY`, raw 26 on Linux, read on the product run, 4 of 24 two-instance boots); the placement closes the write handle before the name appears, and 0 of 24 failed after.»"
    sidecar: "threat-model: filesystem boundary gains the measured concurrent-first-start ETXTBSY race on the pinned copy and its closed-handle placement fix."
    rationale: "Report §Summary and §Outcome (the product race two-sided: 4 of 24 at the base, each `spawn_os_error:26`, 0 of 24 after; single-instance control 0 of 24) and §Decisions & corrections (a closed-handle placement is what defeats `ETXTBSY`)."
    basis: crates/viola-state/src/fs.rs:339-346
    fact: F4
note: The disproved Users-SID claim also stands in other masters the report marks changed (architecture-detail/occupied-resources.md:137 and the test-plan sites test-plan.md:634, integration-test-strategy.md:34, test-data-fixtures.md:10, unit-test-strategy.md:26), and the pinned copy's placement in architecture (established-decisions.md:53, architecture.md:319, the two Deployment / Distribution heads); those are their masters' cascades, not security-plan's.
