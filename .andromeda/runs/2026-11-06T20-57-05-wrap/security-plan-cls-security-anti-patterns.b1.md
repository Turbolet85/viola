@answer n=1 entity=security-anti-patterns#1.1 v=yes mode=stale
why: the Windows-remnant removal reached src/cmd/client.rs and src/cmd/hook.rs whole, where the peer euid/pid verification seams sit, and the report does not show them unmoved
@answer n=2 entity=security-anti-patterns#2.1 v=yes mode=stale
why: tests/support/home.rs (prepare_home_base) and the e2e harness are reached whole and the report does not show the linked-base keepers unmoved
@answer n=3 entity=security-anti-patterns#3.1 v=yes mode=add-to
why: the statusline-rate-limits row reads the account's plan state, so verify can go red (23 pass 1 fail) on unchanged CLI 2.1.287 and the stamp-gated dialog answering must carry that, owner 0.2.0
@answer n=4 entity=security-anti-patterns#3.2 v=yes mode=stale
why: crates/viola-state/src/stamps.rs, strict.rs and src/run/version_gate.rs are reached whole and the report does not show the check-before-read ordering unmoved
@answer n=5 entity=security-anti-patterns#4.1 v=yes mode=stale
why: src/cmd/list.rs (colour reads) and src/bin/viola-fake-agent.rs (test seams) are reached whole and the report shows no env var added but not that their reads and carve-outs are unmoved
@answer n=6 entity=security-anti-patterns#4.2 v=no
why: the report shows no env, flag, feature or other product surface was added, so no seam to force a refusing peer-euid decision exists and the claim holds
