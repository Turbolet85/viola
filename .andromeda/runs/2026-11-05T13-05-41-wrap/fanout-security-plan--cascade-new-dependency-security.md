@proposal n=1 detector=D-new-detail severity=warning fact=F1 file=dependency-security anchor=fct-o63eny7e basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-eks2ad24
section: ## Dependency Security
change: - `cargo deny check advisories` reads red on RUSTSEC-2026-0441 alone (`pin-utils 0.1.0` unmaintained, reached through `portable-pty =0.8.1` → `nix 0.25.1` from `viola-pty`), in CI's `supply-chain` job and locally. It is owned by the CARRY on «Version done-check», whose remedy moves `portable-pty` off `=0.8.1` to a release whose graph drops `pin-utils`. No ignore is widened: the `[advisories]` ignore list keeps its one entry.
sidecar: Dependency Security detail gains the advisories red on RUSTSEC-2026-0441 (pin-utils via portable-pty =0.8.1), owned by the CARRY on «Version done-check», no ignore widened.
rationale: report fct-eks2ad24 (inputs#I3, inputs#I4: the CARRY, remedy portable-pty off =0.8.1, no ignore widened), fct-swb6jipj (ci#42030025050 supply-chain red on RUSTSEC-2026-0441 alone), fct-wrfywvkp (the one advisory ignore stays one).
@proposal n=2 detector=D-new-detail severity=warning fact=F2 file=dependency-security anchor=fct-i5izpzxc basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-swb6jipj
section: ## Dependency Security
change: - The `portable-pty =0.8.1` pin was justified by the 0.9.0 Windows garbage-read bug; that bug left with the Windows code, so the pin now stands on no Windows reason, and «Version done-check» measures moving off it.
sidecar: Dependency Security detail records that the portable-pty =0.8.1 pin lost its Windows reason (the 0.9.0 garbage-read bug) with the Windows code; Version done-check measures moving off it.
rationale: report fct-swb6jipj (the P5 card's Beyond-the-plan CARRY row: the 0.9.0 Windows garbage-read bug that justified the =0.8.1 pin leaves with this chunk), fct-www7rgz5 (portable-pty stays =0.8.1).
@proposal n=3 detector=D-new-detail severity=warning fact=F3 file=dependency-security anchor=fct-pgy5bb7v basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-efiidt7r
section: ## Dependency Security
change: - A `[[licenses.exceptions]]` entry naming a crate no longer in the graph is dropped, never kept: cargo-deny warns `license-exception-not-encountered` for it, and such an exception narrows nothing and widens nothing (overseer technical answer, 2026-11-05). The allow list and the per-crate admission rule are untouched by such a drop.
sidecar: Dependency Security detail gains the rule that a licence exception naming a crate absent from the graph is dropped (license-exception-not-encountered; it narrows and widens nothing).
rationale: report fct-efiidt7r (inputs#I5 item 7, CI job 127253812277's license-exception-not-encountered warning), fct-yq5vc7zj and fct-wrfywvkp (the recvmsg exception dropped at this wrap under it).
@remark
Each proposal is a new `values · deps` group; the standing fct-32d4ty7r (recvmsg) and the retiring dep-conpty / windows-mutants groups are the strong readers' edits, not proposed here.
