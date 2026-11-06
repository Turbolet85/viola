@proposal n=1 detector=D-arch-decisions severity=warning basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-xc746owh
section: §Established Decisions [PTY] (fct-omwrdnlc)
change: The [PTY] decision reads portable-pty `=0.9.0` pinned behind viola's own `pty` seam (spawn · read · write · resize · wait · kill), the behaviour clauses unchanged (`pty_backend()` stays `openpty`); the rationale drops «0.8.1 is spike-proven; moving off 0.8.1 … is owed, owner: "Version done-check"» for: on 0.9.0 the graph drops `pin-utils` (RUSTSEC-2026-0441 closed) and the `serial` family, `viola-pty` compiles unchanged and its 33 tests pass, exact as the workspace pins every third-party version.
sidecar: [PTY] portable-pty `=0.8.1` → `=0.9.0`; the owed move off 0.8.1 (owner "Version done-check") discharged; RUSTSEC-2026-0441 closed (chunk 2026-11-06-version-done-check).
rationale: Report Dependencies fct-xc746owh: `portable-pty` `=0.8.1` → `=0.9.0` in `[workspace.dependencies]`, nix 0.25.1 → 0.28.0, pin-utils and the serial crates removed, `cargo tree -i pin-utils` exit 101, `pty_backend()` stays `openpty`; Retired fct-uufjzm4k names the `=0.8.1` pin and its Windows rationale; Outcome fct-b2ryvv3f viola-pty 33/33 on `=0.9.0`. The locked decision names a pin the chunk moved off.
@proposal n=2 detector=D-arch-decisions severity=warning dependent_of=D-arch-decisions basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-xc746owh
section: §Stack and Technologies (fct-zegjs5rw)
change: The stack sentence reads «portable-pty `=0.9.0` behind viola's own `pty` seam» in place of «portable-pty `=0.8.1` behind viola's own `pty` seam»; the rest of the sentence unchanged.
sidecar: §Stack and Technologies: portable-pty `=0.8.1` → `=0.9.0` (chunk 2026-11-06-version-done-check).
rationale: Report Dependencies fct-xc746owh moves the workspace pin to `=0.9.0`; the stack line restates the [PTY] pin of proposal 1.
@proposal n=3 detector=D-arch-decisions severity=warning dependent_of=D-arch-decisions basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-xc746owh
section: §Infrastructure Patterns → build-system (fct-j6b6zix5)
change: The `[workspace.dependencies]` clause reads «(portable-pty as `=0.9.0`; rmcp minor pinned as `>=3.4.1, <3.5`, default features off, resolved 3.4.1)».
sidecar: build-system: the workspace pin of portable-pty `=0.8.1` → `=0.9.0` (chunk 2026-11-06-version-done-check).
rationale: Report Dependencies fct-xc746owh: `portable-pty` `=0.8.1` → `=0.9.0` in `[workspace.dependencies]` (Cargo.toml); the key restates the retired pin.
@proposal n=4 detector=D-arch-decisions severity=warning dependent_of=D-arch-decisions basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-xc746owh
section: §Infrastructure Patterns → project-directory-structure (fct-irhgihut)
change: The tree's `viola-pty/` line reads «# pty seam over portable-pty =0.9.0 (HostTerminal raw mode: libc termios;» in place of «=0.8.1»; fold into the same whole-fact edit as F434's pointer-line removal on this fact.
sidecar: project-directory-structure: the viola-pty tree comment's portable-pty pin `=0.8.1` → `=0.9.0` (chunk 2026-11-06-version-done-check).
rationale: Report Dependencies fct-xc746owh moves the pin to `=0.9.0`; Expected amendments fct-ngwjb7xx item 3 already edits fct-irhgihut whole, so the pin must move in that same edit or survive it.
@proposal n=5 detector=D-arch-decisions severity=warning dependent_of=D-arch-decisions basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-q2lwjk3d
section: §Infrastructure Patterns → ci-cd-approach (fct-3nc6guvb)
change: The jobs sentence reads «Jobs wired today in `ci.yml` (7, every one green on ci#42288148289)» — the «but `supply-chain`, red on RUSTSEC-2026-0441 alone» clause removed; the job descriptions unchanged.
sidecar: ci-cd-approach: `supply-chain` no longer red on RUSTSEC-2026-0441; all seven `ci.yml` jobs green on ci#42288148289 (chunk 2026-11-06-version-done-check).
rationale: Report Dependencies fct-xc746owh: RUSTSEC-2026-0441 closed, `cargo deny check` advisories ok, `ignore = []`; Cross-project fct-q2lwjk3d: ci#42288148289 seven jobs green including supply-chain; Outcome fct-bggboyf2. The key still states supply-chain red on that advisory.
@proposal n=6 detector=D-retired severity=warning basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-uufjzm4k
section: §Established Decisions [CLI Conventions] (fct-nvft5tqc)
change: The decision keeps «Human text by default, `--json` for agents, a typed exit code per refusal. Prompt text comes from stdin or `--file`, never from a leading-slash argument.»; the sentence «viola warns when an argument carries a Git Bash rewritten-path prefix.» is removed and the rationale reads «agents need machine-parseable refusals», the Git Bash / Windows-path reason dropped.
sidecar: [CLI Conventions]: the Git Bash rewritten-path warning retired with its emitter, unit table and CLI witness (T2); the stdin/`--file` rule stands (chunk 2026-11-06-version-done-check).
rationale: Report Retired fct-uufjzm4k names the Git Bash rewritten-path warning (T2, `inputs#I2`); Symbols fct-zmvefc57 removes `warn_if_rewritten` / `looks_rewritten` from `src/cmd/send.rs` and the only stderr line `send` wrote on a rewritten path; fct-fiaocty5 says the `--file`/stdin rule stands.
@remark
Left to the cascade (detail files): the same pin/swap-trigger/Git Bash/plain_drive retirements in fct-ad44nn2e, fct-kxiazeyf, fct-co2wsjkw (retire), fct-onw4pywk, fct-k6htnrfj; outside these detectors but in the report's expected amendments: the ceiling owner «Version done-check» → 0.2.0's first entry in fct-4nt6qtyp (PROVISIONAL kept), Measurement B «owed to "Version done-check"» → known, no action in 0.1.0 in fct-c3wssf3l, and T1's five ratified divergences in fct-sha7o35z / fct-nhsg4k7g / fct-vf75c7sp; fct-zhr3wx2i's rationale «exec form skips Git Bash path rewriting» is a Windows reason not named in Retired, left for the orchestrator to judge.
