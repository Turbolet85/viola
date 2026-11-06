@proposal n=1 detector=D-cascade severity=warning file=threat-model-summary anchor=fct-33o4e7zi basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Threat Model Summary
change: In the Where list, «`bin/<version>-<hash>/viola(.exe)` is the pinned executable.» reads «`bin/<version>-<hash>/viola` is the pinned executable.»; every other line unchanged.
sidecar: threat-model-summary config row — the pinned executable path loses its Windows `(.exe)` form (Linux-only, founder 2026-11-04, tag multi-os-final).
rationale: the report's Retired bullet removes the Windows code (recoverable at `multi-os-final`); the pinned binary is `viola` alone on Linux.
@proposal n=2 detector=D-retired severity=warning file=threat-model-summary anchor=fct-t7qeqkiu basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Threat Model Summary
change: Entry point's first bullet reads «One endpoint per `viola run`: socket file `<dir>/viola-<h12>.sock`, `<dir>` the per-user 0700 directory (`$XDG_RUNTIME_DIR/viola/` or `/tmp/viola-<uid>/`) (§ Authentication & Authorization, IPC access control (Unix)).»; the trust boundary's first bullet reads «How that is enforced (socket file mode, directory permissions) is not decided in arch …»; Consequences: «On Unix the socket sits» → «The socket sits», the bullet «On Windows, the named-pipe namespace is shared across the machine.» dropped, «As built, on Linux and Windows every frame-writing client» → «As built, every frame-writing client», «ahead of that comparison, on Linux and macOS, each runs» → «ahead of that comparison each runs», and the closing clause «macOS, where the peer reports no pid, compares no pid or start time, the named carve-out, owned by no route entry» dropped; in the `instance-unreachable` bullet the parenthetical «(Linux, Windows and macOS, on macOS an euid mismatch alone)» after `server-verify-failed` dropped; every other clause unchanged.
sidecar: threat-model-summary IPC vector — the Windows named pipe, shared pipe namespace, pipe DACL and the macOS no-pid carve-out retired with the Linux-only cut; the socket in its per-user 0700 directory is the one endpoint.
rationale: the report's Retired bullet retires the Windows named-pipe endpoint and its SDDL/SQOS controls and the macOS socket dir and no-peer-pid carve-out; Symbols/APIs (fct-xxutheix) give the socket-dir forms and fold `PEER_EUID_REPORTED` / `SERVER_PID_REPORTED`.
@proposal n=3 detector=D-cascade severity=warning file=threat-model-summary anchor=fct-qxr7dgpn basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Threat Model Summary
change: «`viola list`'s board scan holds the home to the read rule on every OS (`check_read`)» reads «`viola list`'s board scan holds the home to the read rule (`check_read`)»; the rest of the bullet unchanged.
sidecar: threat-model-summary CLI vector — the «on every OS» clause dropped from the board scan's read rule (Linux-only product; the rule stays).
rationale: the report's Symbols/APIs remove board.rs's OS arms with `board_scan`'s read rule staying; the Counts bullet (os-target class) rules an «on every OS» clause stale wording, the claim kept.
@proposal n=4 detector=D-retired severity=warning file=threat-model-summary anchor=fct-feql22h5 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Threat Model Summary
change: Both `viola(.exe)` read `viola` (the code-bearing artefacts bullet and the `check_home_tree` list); «the strict-modes check (Unix owner and mode, Windows owner and DACL)» reads «the strict-modes check (owner and mode)»; the clause «, and on Windows `pin_companions` checks `bin/<version>-<hash>/conpty/` and each companion before the held open» dropped, the sentence ending «… its `hooks/` and `.claude-plugin/`); `marketplace/` is off the integrity set.»
sidecar: threat-model-summary filesystem vector — the Windows owner/DACL strict-modes form, the ConPTY companion check and the `(.exe)` path form retired; the Unix owner-and-mode check is unconditional.
rationale: the report removes `strict.rs`'s Windows owner/DACL arm and `pin.rs`'s `pin_companions` and held-open companions (Symbols/APIs), and its Retired bullet names the Windows home owner/DACL arm and the ConPTY sideload.
@proposal n=5 detector=D-retired severity=warning file=body anchor=fct-2i3lkhzs dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Threat Model Summary
change: «to the strict-modes check (owner and mode, or owner and DACL) before any log opens» reads «to the strict-modes check (owner and mode) before any log opens»; the rest of the vector line unchanged.
sidecar: Threat Model Summary filesystem vector broad line — the owner-and-DACL strict-modes form retired with the Windows arm.
rationale: the body's broad line that threat-model-summary#9.1 details carries the same retired Windows owner/DACL form (report Retired bullet).
@proposal n=6 detector=D-retired severity=warning file=threat-model-summary anchor=fct-uemrwfo5 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Threat Model Summary
change: «`list` and `mcp` spawn `claude agents --json`, resolving `claude` from their own PATH with npm-shim → `claude.exe` resolution.» reads «`list` and `mcp` spawn `claude agents --json`, resolving `claude` as a bare name on their own PATH.»; in Run B «on a wrapped OS (Unix), and only when the pinned path and the captures dir are shell-plain, it also gets» reads «only when the pinned path and the captures dir are shell-plain, it also gets», and «Windows passes none, Runs A, C and D keep their arguments» reads «Runs A, C and D keep their arguments»; every other clause unchanged.
sidecar: threat-model-summary child-spawn vector — the npm-shim to `claude.exe` resolution and the wrapped-OS / Windows split of Run B's `--settings` retired (resolver: a given path as given, a bare name on PATH; `WRAPPED_HERE` folded).
rationale: the report's Symbols/APIs remove `resolve_program`'s `Search::windows`, `pathext`, `is_script`, `SHIM_TARGET` and fold `WRAPPED_HERE`'s `true`; the Retired bullet names the `.cmd`/`.bat` refusal.
@proposal n=7 detector=D-retired severity=warning file=threat-model-summary anchor=fct-v2lh3e2i basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: ## Threat Model Summary
change: «on a native matrix of `windows-2025`, `macos-latest` and `ubuntu-latest`» reads «on the one-leg matrix `[ubuntu-latest]` (every job and artifact name keeps its `${{ matrix.os }}` form)»; in Jobs «the per-OS `rmcp features` lint step» → «the `rmcp features` lint step», «(per OS under coverage)» → «(under coverage)», «the per-OS `perf` job» → «the `perf` job», «the per-OS release build» → «the release build»; every other clause unchanged.
sidecar: threat-model-summary CI/CD — the `windows-2025` and `macos-latest` legs retired; `ci.yml`'s matrix is `[ubuntu-latest]` for test, perf, lint and release.
rationale: the report's Harness/gate surface keeps `strategy.matrix.os: [ubuntu-latest]` on test, perf, lint and release; the Retired bullet names the `windows-2025` and `macos-latest` CI legs.
@proposal n=8 detector=D-retired severity=warning file=body anchor=fct-ju2d5wlo dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: ## Threat Model Summary
change: «over a native `windows-2025`, `macos-latest` and `ubuntu-latest` matrix» reads «over the one-leg `[ubuntu-latest]` matrix»; the rest of the CI/CD line unchanged.
sidecar: Threat Model Summary CI/CD broad line — the three-OS matrix now `[ubuntu-latest]`.
rationale: the body's broad line that threat-model-summary#13.1 details names the same retired legs (report Harness/gate surface).
@proposal n=9 detector=D-new-detail severity=warning file=threat-model-summary anchor=fct-v2lh3e2i fact=F4 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-swb6jipj
section: ## Threat Model Summary
change: Add a sub-bullet under **CI/CD**, after the Jobs bullet: «  - The Windows and macOS code, tests and CI jobs removed by the Linux-only ruling (founder, 2026-11-04) stay recoverable at the git tag `multi-os-final` (`ecbe7ab3`), pushed to origin; it holds `src/conpty.rs` and `windows-mutants.yml`.»
sidecar: threat-model-summary CI/CD — added: the removed Windows/macOS code and workflows recoverable at tag `multi-os-final` (`ecbe7ab3`).
rationale: the report's Cross-project claims record the tag `multi-os-final` → `ecbe7ab3` pushed to origin, holding `src/conpty.rs` and `windows-mutants.yml` (task fact F4).
@remark
No scoped detector fires (no dependency added, no new input surface, no auth library added); body lines fct-55ydrrgu (tier justification, Windows pipe namespace) and fct-2jp5ovui (Networking, named pipes and $TMPDIR) are stale but detail entities outside this group's picks; #13.1's v1.x winget/Homebrew/Scoop clause is sourced from architecture Stack (Release) and is arch's to re-word; the Auth library windows-sys row belongs to authentication-authorization.
