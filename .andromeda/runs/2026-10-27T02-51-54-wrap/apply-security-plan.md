# Apply — security-plan

## Verdicts
- fanout-security-plan--cascade.md 1 · applied · .andromeda/security-plan-detail/data-protection.md:12#1
- fanout-security-plan--cascade.md 2 · applied · .andromeda/security-plan-detail/data-protection.md:17#1
- fanout-security-plan--cascade.md 3 · applied · .andromeda/security-plan-detail/data-protection.md:24+#1
- fanout-security-plan--cascade.md 4 · applied · .andromeda/security-plan-detail/data-protection.md:24+#2
- fanout-security-plan--cascade.md 5 · applied · .andromeda/security-plan-detail/data-protection.md:24+#3
- fanout-security-plan--cascade.md 6 · applied · .andromeda/security-plan-detail/data-protection.md:24+#4
- fanout-security-plan--cascade.md 7 · applied · .andromeda/security-plan-detail/authentication-authorization.md:33#1
- fanout-security-plan--cascade.md 8 · applied · .andromeda/security-plan-detail/authentication-authorization.md:42#1
- fanout-security-plan--cascade.md 9 · applied · .andromeda/security-plan-detail/authentication-authorization.md:34#1
- fanout-security-plan--cascade.md 10 · applied · .andromeda/security-plan-detail/threat-model-summary.md:131+#1

## Edits
- line: .andromeda/security-plan-detail/data-protection.md:12
  old: «the refused entry an inherited Users (`S-1-5-32-545`) write ACE, so such a home is one the check refuses (as measured on `windows-2025` at viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md).»
  new: «the refused entry an inherited allow ACE that grants a write right to a SID the check does not trust (read by the right, whichever SID holds it), so such a home is one the check refuses (as measured on `windows-2025` at viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/report.md).»
- line: .andromeda/security-plan-detail/data-protection.md:17
  old: «The `pin_exe` first-start race and reproducible builds are owed to the route entry "Pinned copy race and reproducible builds".»
  new: «An absent copy is placed, never replaced, a copy another start placed first is re-hashed before reuse, and one tree's release builds can key two folders (each in the groups below).»
- after: .andromeda/security-plan-detail/data-protection.md:24
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · data-rest -->»
  add: «»
  add: «- **Code-bearing artefacts (the pinned copy's placement):** `pin_bytes` writes an absent `bin/<version>-<hash>/viola(.exe)` by a no-clobber placement, never a replace: `viola_state::fs::place_private_new` creates a temp file beside it (`NamedTempFile::new_in`), sets its mode before any byte, writes the bytes, runs `sync_all`, closes the write handle (`into_temp_path`), then places it with `persist_noclobber`, so the name never appears while a handle holds it open for write. `replace_private_shared` keeps only the plugin files and the ConPTY companions.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/security-plan-detail/data-protection.md:24
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · data-rest -->»
  add: «»
  add: «- **Code-bearing artefacts (a present pinned-copy name):** `place_private_new` never replaces a present name: `AlreadyExists` is `Placed::Present`, the target untouched and the temp file removed; any other failure is an error that leaves the target as it was, with no retry.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/security-plan-detail/data-protection.md:24
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · data-rest -->»
  add: «»
  add: «- **Code-bearing artefacts (a lost placement):** On `Placed::Present` (a concurrent first start placed the copy first) `pin_bytes` re-hashes the copy that stands: equal bytes are reused, and different bytes are `PinError::HashMismatch` with the file left as found, never overwritten or deleted. A lost placement whose copy is reused reads `ok` on `run.pin_copy` (`ok|pinned-hash-mismatch`).»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/security-plan-detail/data-protection.md:24
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · data-rest -->»
  add: «»
  add: «- **Code-bearing artefacts (one tree's release builds):** Clean release builds of one tree can give two files whose loaded sections are equal, so one tree can key two `bin/<key>/` and `plugin/<key>/` folders; the reading stands at architecture §Established Decisions [Deployment / Distribution], no cause is claimed, and acting on it is the founder's.»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/security-plan-detail/authentication-authorization.md:33
  old: «(std `set_permissions`: 0600, or 0700 for the pinned `viola`) before any byte is written, then `sync_all`s and `persist`s it into place.»
  new: «(std `set_permissions`: 0600) before any byte is written, then `sync_all`s and `persist`s it into place. The pinned `viola` is never replaced: its absent copy goes through `viola_state::fs::place_private_new`, which sets the temp file's mode (0700) the same way before any byte is written, `sync_all`s it, closes its write handle and places it with `persist_noclobber`, never over a present name (§ Data Protection).»
- line: .andromeda/security-plan-detail/authentication-authorization.md:34
  old: «a plain home made before viola refused `Writable`»
  new: «a plain home made before viola refused `Writable` (read by the inherited write right whichever SID holds it, with a viola-made-home control)»
- line: .andromeda/security-plan-detail/authentication-authorization.md:42
  old: «tempfile 3.27.0 `persist` via `viola_state::fs::replace_private`»
  new: «tempfile 3.27.0 `persist` via `viola_state::fs::replace_private`; `into_temp_path` + `persist_noclobber` via `viola_state::fs::place_private_new` (the pinned `viola`)»
- after: .andromeda/security-plan-detail/threat-model-summary.md:131
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · boundary-filesystem -->»
  add: «»
  add: «- **Vector:** Filesystem state under `~/.viola/`, the pinned binary's first placement by concurrent first starts.»
  add: «  - **Trust boundary:** The pinned binary is placed once and never replaced. An exec of a copy that another start still holds open for write fails with `ETXTBSY` (raw 26 on Linux), so the placement closes its write handle before the name appears (`viola_state::fs::place_private_new`, § Data Protection): 0 of 24 two-instance boots failed, as measured at viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/report.md.»
  add: «»
  add: «<!-- /detail -->»

## Patterns
[[pattern]]
id = "sec-01"
fixed = "the refused entry an inherited Users (`S-1-5-32-545`) write ACE"

[[pattern]]
id = "sec-02"
fixed = "first-start race and reproducible builds are owed to the route entry"

[[pattern]]
id = "sec-03"
fixed = "or 0700 for the pinned `viola`"
