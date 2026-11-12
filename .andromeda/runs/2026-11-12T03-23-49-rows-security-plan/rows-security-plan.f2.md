@row_add n=1 row=decision/no-tokio-in-sync-crates area=dependency-security rec=1,2
name: No tokio in the sync crates
rule: tokio is banned from each sync crate's dependency graph, checked per sync crate as the sole root.
except: viola-channel's Tokio client sits behind its tokio feature; its graph without the feature is checked to hold no tokio by the cargo tree form instead.
guard: deny-sync.toml; channel-tokio-free.sh; deny-probes.sh
@row_add n=2 row=decision/actions-pinned-by-sha area=dependency-security rec=3,4,5
name: GitHub Actions pinned by full commit SHA
rule: Every GitHub Action is referenced by a full commit SHA with a version comment, never by @stable, @v2 or any mutable ref, and actions/checkout runs with persist-credentials: false.
@row_add n=3 row=decision/signed-self-update area=dependency-security rec=7,8,9
name: self_update only with signatures
rule: self_update never ships without its signatures feature (zipsign); a v1.x release requires self_update 1.3.0 built with signatures.
why: a checksum from the same release proves nothing
@row_add n=4 row=decision/no-rust-cache-in-release area=dependency-security rec=10,8,11
name: No rust-cache in the release workflow
rule: Swatinem/rust-cache is never restored in the v1.x release workflow.
@row_add n=5 row=decision/no-repo-secret-scanner area=bootstrap-phases-derive-for-route-setup-project rec=12,13,14
name: No repository secret scanner in v1
rule: v1 runs no repository secret scanner; revisit when the v1.x release workflow adds signing credentials.
why: there are no secrets in the repo or CI
@row_add n=6 row=decision/no-stored-secrets area=secret-management rec=15,16
name: No stored secrets
rule: There are no stored secrets in v1, and environment variables are not a configuration channel.
except: the test-only seams compiled out of release builds, and the terminal colour facts read by viola list's human mode
@row_add n=7 row=decision/tests-use-own-home area=secret-management rec=17,18
name: Tests use their own home
rule: Development handles secrets as production does, and tests use their own --home directory.
@row_add n=8 row=decision/dialog-answers-need-verify-stamp area=security-anti-patterns-never-do-these rec=19,20
name: Dialog answers need a verify stamp
rule: A hook.dialog is answered with a non-null decision, PermissionRequest allow included, only when ledger/stamps.json holds a viola verify stamp for the child's CLI version on which every relied ledger row passes.
why: a stamp is what switches on dialog answering
@row_add n=9 row=decision/stamps-written-only-by-verify area=security-anti-patterns-never-do-these rec=19,20,21,22
name: Only viola verify writes the stamps file
rule: No process other than viola verify ever writes ledger/stamps.json; viola verify writes it under its lock, with 0600.
why: a stamp is what switches on dialog answering, PermissionRequest allow included
@row_add n=10 row=decision/version-gate-strict-stamps-read area=security-anti-patterns-never-do-these rec=23,24
name: Version gate reads stamps through the strict-modes check
rule: A hook.dialog gets a non-null decision only when viola run's version gate read ledger/stamps.json through the strict-modes check, unlocked and capped; a refusal, an unreadable or a malformed file is cli_verified:false.
