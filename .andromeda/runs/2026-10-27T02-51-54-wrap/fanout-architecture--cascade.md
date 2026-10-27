proposals:
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 53
    section: '## Established Decisions'
    change: 'The head reads `**[Snapshot writer] tempfile 3.27.0 `persist` and `persist_noclobber` through the shared helpers of `viola_state::fs`.**` (no longer ONE helper); `replace_private`''s mode parenthesis reads `(0600)`, the pinned `viola`''s 0700 now set through `place_private_new`; `replace_private_shared`''s list reads `(the plugin files, the Windows x64 ConPTY companions under `bin/<version>-<hash>/conpty/`)`, the pinned copy removed; after that sentence add `The pinned copy''s absent-copy write is never a replace: it is the no-clobber `place_private_new` (below).`; the rest of the line unchanged.'
    sidecar: 'established-decisions [Snapshot writer]: the pinned copy leaves `replace_private_shared`''s files for the no-clobber `place_private_new`; the head names the helpers of `viola_state::fs`, not one helper.'
    rationale: 'Report Changes, Symbols / APIs: `pin_bytes`''s `NotFound` arm now calls `place_absent` → `place_private_new` (`pin.rs:108-116`), where it called `replace_private_shared(&path, bytes, DIR_MODE)`; `replace_private_shared`''s doc names only the plugin files and the companions; Expected amendments: `established-decisions.md:53` changed.'
    basis: crates/viola-state/src/pin.rs:108
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture.md
    near: 37
    section: '## Established Decisions'
    change: 'The head reads `**[Snapshot writer] tempfile 3.27.0 `persist` and `persist_noclobber` through the shared helpers of `viola_state::fs`.**`; after `a failure leaving the target as it was;` add `the pinned copy is never replaced: its absent-copy write is `place_private_new`, a no-clobber placement of a closed temp file;` before `no call site carries its own temp + rename code`; the rationale unchanged.'
    sidecar: 'architecture body [Snapshot writer]: the broad line names the no-clobber placement beside `replace_private`, no longer ONE helper.'
    rationale: 'The broad line the detail at `established-decisions.md:53` serves states ONE shared helper; the report lands a second public helper in `viola_state::fs` (`place_private_new`, `fs.rs:316-324`, `TempPath::persist_noclobber` after `into_temp_path`).'
    basis: crates/viola-state/src/fs.rs:316
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 18
    section: '## Stack and Technologies'
    change: 'In the State-file primitives row, after `for snapshots and every other replaced file` add `, and tempfile 3.27.0 `TempPath::persist_noclobber` after `into_temp_path` closes the write handle (`viola_state::fs::place_private_new`) for the pinned copy''s absent-copy write, placed once and never replaced`; the rest of the row unchanged.'
    sidecar: 'stack State-file primitives: the pinned copy''s no-clobber placement through `place_private_new` joins `replace_private`.'
    rationale: 'Report Changes, Symbols / APIs and Dependencies: `place_private_new` uses `into_temp_path` + `persist_noclobber`, both already in `tempfile 3.27.0`, no version moved; the row names one helper only.'
    basis: crates/viola-state/src/fs.rs:339
    dependent-of: D-arch-decisions
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 53
    section: '## Established Decisions'
    change: 'Add after line 53 a continuation line: ` `viola_state::fs::place_private_new(path, bytes, mode) -> Result<Placed, StateError>` (public enum `Placed { Written, Present }`) is the no-clobber placement: it creates a temp file beside the target (`NamedTempFile::new_in`), sets its mode before any byte (the pinned `viola` at `DIR_MODE`), writes the bytes, `sync_all`s, closes the handle (`into_temp_path`) and calls `TempPath::persist_noclobber`; `AlreadyExists` is `Ok(Placed::Present)`, the target untouched and the temp removed; any other failure is an error that leaves the target as it was; no retry and no new error variant (chunk 2026-10-27-pinned-copy-race-and-reproducible-builds).`'
    sidecar: 'established-decisions [Snapshot writer]: new detail, `place_private_new` and `Placed`.'
    rationale: 'Report Changes, Symbols / APIs: `Placed` (`fs.rs:307-314`) and `place_private_new` (`fs.rs:316-324`), the order temp → mode → write → `sync_all` → close → `persist_noclobber`, `AlreadyExists` → `Present`.'
    basis: crates/viola-state/src/fs.rs:307
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 53
    section: '## Established Decisions'
    change: 'Add after the `place_private_new` continuation line: ` Its body, the private `place_private_new_with(path, bytes, mode, placed)`, takes a `placed` hook that runs right after a `Written` placement: a forced-window test parameter, not a seam (no env read, no feature); `place_private_new_placed_copy_execs_at_once` execs the placed copy from it (Unix; its Windows form `place_private_new_placed_copy_runs_at_once`).`'
    sidecar: 'established-decisions [Snapshot writer]: new detail, the `placed` hook of `place_private_new_with` is a test parameter, not a seam.'
    rationale: 'Report Changes, Symbols / APIs: `place_private_new_with` (`fs.rs:326-347`) and its `placed` hook, no env read, no feature; New text, by line: the two cases at `fs.rs:431-453` and `455-480`.'
    basis: crates/viola-state/src/fs.rs:326
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 54
    section: '## Established Decisions'
    change: 'Append to the Rationale line: ` The pinned copy is placed with its write handle closed because tempfile''s `NamedTempFile::persist` renames while that handle is still open, and a concurrent first start''s agent exec of a file open for write fails `ETXTBSY` (raw errno 26): 4 of 24 two-instance boots failed so before, 0 of 24 after (chunk 2026-10-27-pinned-copy-race-and-reproducible-builds, `evidence/race-before.md`, `evidence/race-after.md`).`'
    sidecar: 'established-decisions [Snapshot writer]: the rationale for the closed-handle placement, `ETXTBSY` read on the product run.'
    rationale: 'Report Summary and Decisions & corrections: a closed-handle placement defeats `ETXTBSY`; `persist` renames with the handle open, `into_temp_path` closes it first; Outcome: 4 of 24 before, each `spawn_os_error:26`, 0 of 24 after.'
    basis: viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/race-before.md
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 249
    section: '## Established Decisions'
    change: 'Add after line 249 a continuation line: ` Measurement B (chunk 2026-10-27-pinned-copy-race-and-reproducible-builds, `evidence/reproducible-builds.md`): six cold release builds of one tree at one target path gave two files (8 179 264 B and 8 179 272 B, three each, in no fixed order); 3 of 34 sections were unequal (`.note.gnu.build-id`, `.symtab`, `.strtab`), every loaded section equal, and 6 `strings` lines differ (three `anon.<hash>.<n>.llvm.<number>` local symbols). So one tree can key two `bin/<key>/` and `plugin/<key>/` folders; the cause is not explained, and acting on it is the founder''s (a CARRY on «Version done-check»).`'
    sidecar: 'established-decisions [Deployment / Distribution]: measurement B, one tree can build two files with equal loaded sections and key two pinned folders.'
    rationale: 'Report Spec claims disproved by measurement (measurement B) and Expected amendments: `established-decisions.md:247` is the decision''s site for the reading; Decisions & corrections item 4: recorded as measured, no cause claimed, a founder item.'
    basis: viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/reproducible-builds.md
    fact: F5
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture.md
    near: 69
    section: '## Established Decisions'
    change: 'After `the content hash keys the copy because the version alone collides across rebuilds` add `; one tree has built to two files whose loaded sections are equal, so it can key two copies (measurement B; no cause claimed, the act the founder''s)`; the rest unchanged.'
    sidecar: 'architecture body [Deployment / Distribution]: the broad rationale carries measurement B''s reading.'
    rationale: 'Report Expected amendments: `architecture.md:69` is one of the two head lines dispositioned as the site for measurement B''s reading.'
    dependent-of: D-new-detail
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 137
    section: '## Occupied Resources'
    change: 'The last clause reads `and a plain home made before viola is refused for its inherited write grant to a SID `trusted_sid` does not trust, read by its right whichever SID holds it (Users at ci#40403389629; Authenticated Users `S-1-5-11` with Users read only at ci#40425866096, both `windows-2025`), a home `instance_tree()` makes reading none`, in place of «naming its inherited Users (`S-1-5-32-545`) write entry (as measured at ci#40403389629, `windows-2025`)».'
    sidecar: 'occupied-resources instances/: the Windows plain-home refusal is read by its inherited untrusted write grant, not by the Users SID.'
    rationale: 'Report Spec claims disproved by measurement: at ci#40425866096 Users held read only and Authenticated Users (`S-1-5-11`, `0x001301BF`) the inherited write; the product verdict `Err(Writable)` was right and the oracle now reads the grant by its right with a control; `occupied-resources.md:137` marked changed.'
    basis: crates/viola-state/src/strict.rs:746
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 252
    section: '## Occupied Resources'
    change: 'Add after line 252 an item: `- On `windows-2025` runners of one image (20261021.278.1) the temp dir''s inherited ACL differs by host: Users write at ci#40403389629 (westus3); Authenticated Users (`S-1-5-11`, mask `0x001301BF`) write with Users read at ci#40425866096 (northcentralus). An oracle that names one SID for a runner-inherited grant is runner-dependent, so the plain-home case reads the grant by its right (inherited, a `WRITE_RIGHTS` bit, a SID `trusted_sid` does not trust; `strict::win::tests::inherited_untrusted_writes`), with a control that cannot pass vacuously (as measured at chunk 2026-10-27-pinned-copy-race-and-reproducible-builds, green at ci#40439188053).`'
    sidecar: 'occupied-resources Repository: new detail, the runner-dependent inherited ACL and the oracle read by its right.'
    rationale: 'Report Decisions & corrections: the windows-2025 runners differ in a temp dir''s inherited ACL by host; read the grant by its right with a control (`strict.rs:746-760`, `:787-793`); it sits beside the LA-alias oracle lesson of the same kind.'
    basis: crates/viola-state/src/strict.rs:787
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 23
    section: '## Occupied Resources'
    change: 'Add after line 23 a continuation line: ` `viola-fake-agent`''s `hook` receipt carries `spawn_os_error` when the spawn of an absolute `command` fails (the raw OS error as an integer, `null` when the OS gives none); a spawn that succeeds and the non-absolute arm are unchanged (chunk 2026-10-27-pinned-copy-race-and-reproducible-builds; the receipt''s fields are test-plan §7''s).`'
    sidecar: 'occupied-resources test-only binaries: the fake agent''s `hook` receipt gains `spawn_os_error` on a failed absolute spawn.'
    rationale: 'Report Changes, Symbols / APIs (test-only): `run_receipted` adds `spawn_os_error` (`json!(e.raw_os_error())`) on a failed absolute spawn, `src/bin/viola-fake-agent.rs:642-644`; it binds with test-plan §7 fake agent receipts (`test-data-fixtures.md:42`).'
    basis: src/bin/viola-fake-agent.rs:642
    fact: F7
note: F4 (the pinned copy placed once via `place_absent`, a lost placement re-hashing the standing copy) and its body line `architecture.md:319` («creates if it is absent», report-marked changed) have no picked entity, nor does Inherited Defaults `architecture.md:445` («tempfile 3.27.0 `persist` through one shared helper»), which the [Snapshot writer] amendment makes stale; F7's receipt field is owned by test-plan §7.
