@proposal n=1 detector=D-retired severity=warning basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-xc746owh
section: Dependency Security
change: fct-serp72ds — `[advisories]`: `unmaintained = "all"` (the default), `unsound = "all"`, `yanked = "deny"`, and `ignore = []`: no advisory is ignored (the RUSTSEC-2017-0008 `serial` entry left with its crate when portable-pty moved to `=0.9.0`); any future ignore needs a `reason` naming the disabled feature.
sidecar: Dependency Security [advisories] — the RUSTSEC-2017-0008 ignore retired with the `serial` crate family (portable-pty =0.9.0); the ignore list is empty (chunk 2026-11-06-version-done-check).
rationale: report Dependencies fct-xc746owh: `deny.toml` `[advisories] ignore` → `ignore = []`, the RUSTSEC-2017-0008 entry left with its crate; Retired fct-uufjzm4k names the RUSTSEC-2017-0008 ignore and the `serial` family; Counts fct-u65xlcax: ignore entries 1 → 0.
@proposal n=2 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-xc746owh
section: Dependency Security
change: fct-uydjlfhj — the root-graph pin example reads portable-pty `=0.9.0` (not `=0.8.1`); the rest of the pinning sentence stands.
sidecar: Dependency Security Pinning — portable-pty `=0.8.1` → `=0.9.0` (chunk 2026-11-06-version-done-check).
rationale: report Dependencies fct-xc746owh: `portable-pty` `=0.8.1` → `=0.9.0` in `[workspace.dependencies]`; Retired fct-uufjzm4k names the `=0.8.1` pin and its Windows rationale.
@proposal n=3 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-uufjzm4k
section: Dependency Security
change: fct-4r3p2tgk — Within 7 days of a RustSec/GHSA advisory against a crate and feature viola enables: bump the crate (no named swap path stands; the portable-pty-psmux trigger is spent).
sidecar: Dependency Security CVE SLA — the portable-pty → portable-pty-psmux 0.9.7 swap path retired, its trigger spent by portable-pty =0.9.0 dropping pin-utils (chunk 2026-11-06-version-done-check).
rationale: report Retired fct-uufjzm4k: the PTY swap trigger (portable-pty-psmux path) is now spent, 0.9.0 dropping pin-utils upstream; Dependencies fct-xc746owh: RUSTSEC-2026-0441 closed, `cargo tree -i pin-utils` exit 101.
@proposal n=4 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-q2lwjk3d
section: Data Protection
change: fct-p7rzex2i — Handle inheritance: channel handles never leak into the `claude` child: portable-pty closes every inherited descriptor in the child, witnessed by the live round's child fd table (fds 0–2 on the pty, no socket that is viola's endpoint or the wrapper's); no PTY swap path stands.
sidecar: Data Protection Handle inheritance — «recorded, not established» lifted on the Version done-check measurement; the portable-pty-psmux swap candidate retired (chunk 2026-11-06-version-done-check).
rationale: report Cross-project fct-q2lwjk3d: the child's `/proc/<child>/fd` 31 fds, fds 0–2 on the pty, no socket that is viola's endpoint or the wrapper's, and `inputs#I7` orders the mechanism to read «portable-pty closes every inherited descriptor in the child»; Retired fct-uufjzm4k: the portable-pty-psmux swap trigger spent.
@remark
D-security-input, D-security-auth and D-security-deps hold: no new external-input surface or auth/secret handling; the new graph crates (`nix` 0.28.0, `serial2`, `cfg_aliases`) hit no §Dependency Security ban (`cargo deny check` bans ok). Detail-file hits fct-6nwwowx3, fct-zj6omeqt (dependency-security) and fct-gnmmvxa5 (error-handling) still state the =0.8.1 / Version done-check owner claims and are the cascade's.
