@answer n=1 entity=security-anti-patterns#1.1 v=yes mode=stale
why: the Windows-remnant removal reaches cmd/client.rs, cmd/hook.rs and cmd/send.rs whole, and the report does not show the frame-verification path holds on them

@answer n=2 entity=security-anti-patterns#2.1 v=yes mode=stale
why: tests/support/home.rs and the harness mod.rs are reached whole by the Windows-remnant removal, and the report does not show the linked-base keepers hold on them

@answer n=3 entity=security-anti-patterns#3.1 v=yes mode=stale
why: the statusline-rate-limits row now reads red (23 pass 1 fail) on an unchanged CLI, so the stamp-gated dialog answering rests on a verify that can read red

@answer n=4 entity=security-anti-patterns#3.2 v=yes mode=stale
why: crates/viola-state/src/stamps.rs and strict.rs are reached whole by the Windows-remnant removal, and the report does not show the verify check order holds on them

@answer n=5 entity=security-anti-patterns#4.1 v=yes mode=stale
why: src/cmd/list.rs and src/bin/viola-fake-agent.rs are reached whole by the change, and the report does not show the colour-fact and test-seam reads hold on them

@answer n=6 entity=security-anti-patterns#4.2 v=yes mode=stale
why: cmd/client.rs and cmd/hook.rs, which host the peer_admitted seams, are reached whole, and the report does not show those seams hold on them
