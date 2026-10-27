# Apply — architecture

## Verdicts
- fanout-architecture.md 1 · applied · .andromeda/architecture.md:319#1, .andromeda/architecture.md:319#2
- fanout-architecture.md 2 · applied · .andromeda/architecture.md:37#1, .andromeda/architecture.md:37#2
- fanout-architecture--cascade.md 2 · folded into fanout-architecture.md 2
- fanout-architecture.md 3 · applied · .andromeda/architecture.md:69#1
- fanout-architecture--cascade.md 8 · folded into fanout-architecture.md 3
- fanout-architecture--cascade-new-occupied-resources.md 1 · applied · .andromeda/architecture-detail/occupied-resources.md:110+#1
- fanout-architecture--cascade.md 9 · applied · .andromeda/architecture-detail/occupied-resources.md:137#1
- fanout-architecture--cascade.md 10 · applied · .andromeda/architecture-detail/occupied-resources.md:252+#1
- fanout-architecture--cascade.md 1 · applied · .andromeda/architecture-detail/established-decisions.md:53#1, .andromeda/architecture-detail/established-decisions.md:53#2, .andromeda/architecture-detail/established-decisions.md:53#3
- fanout-architecture--cascade.md 4 · applied · .andromeda/architecture-detail/established-decisions.md:56+#1
- fanout-architecture--cascade.md 5 · applied · .andromeda/architecture-detail/established-decisions.md:56+#2
- fanout-architecture--cascade.md 6 · applied · .andromeda/architecture-detail/established-decisions.md:56+#3
- fanout-architecture--cascade.md 7 · applied · .andromeda/architecture-detail/established-decisions.md:256+#1
- fanout-architecture--cascade.md 3 · applied · .andromeda/architecture-detail/stack-and-technologies.md:18#1

## Edits
- line: .andromeda/architecture.md:319
  old: «the pinned copy of the binary that `viola run` creates if it is absent (0700 on Unix)»
  new: «the pinned copy of the binary, which `viola run` places once when it is absent, with no clobber, and never replaces (0700 on Unix)»
- line: .andromeda/architecture.md:319
  old: «an existing copy is re-hashed on every start, and a mismatch refuses the start»
  new: «an existing copy is re-hashed on every start, and so is one a concurrent first start placed first, and a mismatch refuses the start»
- line: .andromeda/architecture.md:37
  old: «**[Snapshot writer] tempfile 3.27.0 `persist` through ONE shared helper.**»
  new: «**[Snapshot writer] tempfile 3.27.0 `persist` and `persist_noclobber` through the shared helpers of `viola_state::fs`.**»
- line: .andromeda/architecture.md:37
  old: «`persist`s, a failure leaving the target as it was;»
  new: «`persist`s, a failure leaving the target as it was; the pinned copy is never replaced: its absent-copy write is `place_private_new`, a no-clobber placement of a closed temp file;»
- line: .andromeda/architecture.md:69
  old: «the content hash keys the copy because the version alone collides across rebuilds.»
  new: «the content hash keys the copy because the version alone collides across rebuilds; one tree can build two files whose loaded sections are equal and so key two copies, its cause not explained and acting on it owed to the working-route entry "Version done-check".»
- after: .andromeda/architecture-detail/occupied-resources.md:110
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · res-filesystem -->»
  add: «»
  add: «- `bin/<version>-<hash>/viola(.exe)`, the pinned copy (0700 on Unix): when it is absent, `viola_state::pin::pin_bytes` places it through `pin::place_absent`, a no-clobber placement of a closed temp file (`viola_state::fs::place_private_new` at `DIR_MODE`, [Snapshot writer]), so the copy is placed once and never replaced. `Written` is ok. `Present` (a concurrent first start placed the copy first) re-hashes the copy that stands: equal bytes are reused, and different bytes are `PinError::HashMismatch`, the file left as found and the start refused `pinned-hash-mismatch` (exit 1). `run.pin_copy` reads `ok` or `pinned-hash-mismatch`, a lost placement with equal bytes reading `ok`.»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/architecture-detail/occupied-resources.md:137
  old: «and a plain home made before viola is refused, naming its inherited Users (`S-1-5-32-545`) write entry (as measured at ci#40403389629, `windows-2025`).»
  new: «and a plain home made before viola is refused for its inherited write grant to a SID `trusted_sid` does not trust, read by its right whichever SID holds it, while a home `instance_tree()` makes reads none (as measured at ci#40439188053, `windows-2025`).»
- after: .andromeda/architecture-detail/occupied-resources.md:252
  anchor: «- The protected owner-only DACL of a home viola makes outside the profile reads `D:PAI(A;OICI;FA;;;LA)(A;OICI;FA;;;SY)` on `windows-2025`, read back through `icacls /save`: canonical SDDL spells the runner's user, the local Administrator (RID 500), `LA`, so an oracle matching its SID string fails (as measured at ci#40400887151).»
  add: «- On `windows-2025` runners of one image (20261021.278.1) the temp dir's inherited ACL differs by host: Users write on one (ci#40403389629, westus3), Authenticated Users (`S-1-5-11`, mask `0x001301BF`) write with Users read only on another (ci#40425866096, northcentralus). An oracle that names one SID for a runner-inherited grant is runner-dependent, so the plain-home case reads the grant by its right (inherited, a `WRITE_RIGHTS` bit, a SID `trusted_sid` does not trust; `strict::win::tests::inherited_untrusted_writes`), with a control that cannot pass vacuously (as measured at ci#40439188053).»
- line: .andromeda/architecture-detail/established-decisions.md:53
  old: «tempfile 3.27.0 `persist` through ONE shared helper.»
  new: «tempfile 3.27.0 `persist` and `persist_noclobber` through the shared helpers of `viola_state::fs`.»
- line: .andromeda/architecture-detail/established-decisions.md:53
  old: «(0600, or 0700 for the pinned `viola`)»
  new: «(0600)»
- line: .andromeda/architecture-detail/established-decisions.md:53
  old: «(the pinned copy, the plugin files, the Windows x64 ConPTY companions under `bin/<version>-<hash>/conpty/`): a replace that fails counts as done when the target already holds exactly those bytes.»
  new: «(the plugin files, the Windows x64 ConPTY companions under `bin/<version>-<hash>/conpty/`): a replace that fails counts as done when the target already holds exactly those bytes. The pinned copy's absent-copy write is never a replace: it is the no-clobber `place_private_new` (below).»
- after: .andromeda/architecture-detail/established-decisions.md:56
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · dec-snapshot-writer -->»
  add: «»
  add: «- `viola_state::fs::place_private_new(path, bytes, mode) -> Result<Placed, StateError>` (public enum `Placed { Written, Present }`) is the no-clobber placement, the pinned copy's absent-copy write (`viola_state::pin::place_absent`): it creates a temp file beside the target (`NamedTempFile::new_in`), sets its mode before any byte (the pinned `viola` at `DIR_MODE`), writes the bytes, `sync_all`s, closes the write handle (`into_temp_path`) and calls `TempPath::persist_noclobber`. `AlreadyExists` is `Ok(Placed::Present)`, the target untouched and the temp path removed; any other failure is an error that leaves the target as it was. It has no retry and no error variant of its own.»
- after: .andromeda/architecture-detail/established-decisions.md:56
  anchor: «<!-- /detail -->»
  add: «- Its body, the private `place_private_new_with(path, bytes, mode, placed)`, runs its `placed` hook right after a `Written` placement: a forced-window test parameter, not a seam (no env read, no feature). `place_private_new_placed_copy_execs_at_once` execs the placed copy from it on Unix, and `place_private_new_placed_copy_runs_at_once` runs it on Windows.»
- after: .andromeda/architecture-detail/established-decisions.md:56
  anchor: «<!-- /detail -->»
  add: «- The write handle is closed before the placement because `NamedTempFile::persist` renames while that handle is still open, and a concurrent first start's agent exec of a copy still open for write fails `ETXTBSY` (raw errno 26): 4 of 24 two-instance boots failed so through the open-handle rename and 0 of 24 through `place_private_new` (as measured on the Linux dev host at `viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/race-before.md` and `evidence/race-after.md`).»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/architecture-detail/established-decisions.md:256
  anchor: «- On Windows x64, before the held open, `pin_companions` runs the owner/DACL check (`viola_state::strict::check_present`) on `bin/<version>-<hash>/conpty/` and on each companion that exists: a refusal is `PinError::StrictModes`, writes nothing, and the child runs on the inbox ConPTY with `sideload_fallback` `strict-modes-failed`, the exit code unchanged ([PTY]).»
  add: «- One source tree can build two different files: six cold release builds of one tree at one target path gave two, 8 179 264 B and 8 179 272 B, three each in no fixed order, unequal in 3 of 34 sections (`.note.gnu.build-id`, `.symtab`, `.strtab`) with every loaded section equal and 6 `strings` lines apart (three `anon.<hash>.<n>.llvm.<number>` local symbols), so one tree can key two `bin/<key>/` and two `plugin/<key>/` folders (as measured on the Linux dev host at `viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/reproducible-builds.md`). Its cause is not explained; acting on it is owed to the working-route entry "Version done-check".»
- line: .andromeda/architecture-detail/stack-and-technologies.md:18
  old: «for snapshots and every other replaced file;»
  new: «for snapshots and every other replaced file, and tempfile 3.27.0 `TempPath::persist_noclobber` after `into_temp_path` closes the write handle (`viola_state::fs::place_private_new`) for the pinned copy's absent-copy write, placed once and never replaced;»

## Patterns
[[pattern]]
id = "arch-01"
fixed = "through ONE shared helper"
ignore_case = true

[[pattern]]
id = "arch-02"
fixed = "that `viola run` creates if it is absent"

[[pattern]]
id = "arch-03"
fixed = "or 0700 for the pinned `viola`"

[[pattern]]
id = "arch-04"
fixed = "(the pinned copy, the plugin files"

[[pattern]]
id = "arch-05"
regex = 'inherited (Users \(`S-1-5-32-545`\)|`S-1-5-32-545`)( \(Users\))? write'
