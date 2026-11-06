@answer n=1 entity=authentication-authorization#1.1 v=no
why: the entity says there are no accounts or passwords, and the session adds no account or credential surface.
@answer n=2 entity=authentication-authorization#1.2 v=no
why: the report states no product surface (verb, flag, channel method) was added, so the nine served methods and all-or-nothing capability stand.
@answer n=3 entity=authentication-authorization#1.3 v=no
why: the live witness re-read unmoved on Linux (socket directory 0700, socket 0600) holds the pair, and its bind-side code lives in viola-channel server.rs, which the change did not reach.
@answer n=4 entity=authentication-authorization#1.4 v=no
why: strict.rs is reached, but the directory verdict's pass path is witnessed live (socket directory 0700) and tests/security_negatives_socket_dir.rs is green in CI run 42288148289 on 849aad18.
@answer n=5 entity=authentication-authorization#1.5 v=no
why: the never-chmod behaviour of a refused directory is exercised by tests/security_negatives_socket_dir.rs, green in CI run 42288148289 on 849aad18, an unmoved gate reading.
@answer n=6 entity=authentication-authorization#1.6 v=yes mode=stale
why: strict.rs, client.rs and hook.rs are reached whole, and the report does not show the one-helper sharing of check_endpoint_dir across the bind and the three client seams still holds.
@answer n=7 entity=authentication-authorization#1.7 v=no
why: its owning code, src/cmd/run.rs (socket_dir_ready), is not among the files the change reached.
@answer n=8 entity=authentication-authorization#1.8 v=no
why: the leftover-socket rule lives in crates/viola-e2e/src/harness/cleanup.rs, which the change did not reach.
@answer n=9 entity=authentication-authorization#1.9 v=no
why: the accept-loop peer check lives in viola-channel server.rs, which the change did not reach.
@answer n=10 entity=authentication-authorization#1.10 v=no
why: the accept-loop drop (accept_loop, serve_admitting) lives in viola-channel server.rs, which the change did not reach.
@answer n=11 entity=authentication-authorization#1.11 v=no
why: the Linux peer euid and pid checks in the reached client and hook connect paths are exercised by the e2e suite green in CI run 42288148289 on 849aad18, an unmoved gate reading.
@answer n=12 entity=authentication-authorization#1.12 v=yes mode=stale
why: the seam order (directory check, connect, peer euid, server verification) is set in the reached files src/cmd/client.rs and src/cmd/hook.rs, and no witness re-reads that ordering.
@answer n=13 entity=authentication-authorization#1.13 v=no
why: the release-from-driver guard and the viola-core params sit outside the reached files, and the change adds no channel method or param.
@answer n=14 entity=authentication-authorization#1.14 v=yes mode=add-to
why: F3 (the ledger row statusline-rate-limits reads plan state, so verify can go red on an unchanged CLI 2.1.287) joins the version gate's cli_verified:false refusal recorded here.
@answer n=15 entity=authentication-authorization#1.15 v=yes mode=contradict
why: task item 6 closes the Epoch 7 OPEN mark on the board's read rule on the founder's own word, so the OPEN wording on this row is no longer true.
@answer n=16 entity=authentication-authorization#1.16 v=yes mode=stale
why: strict.rs (check_home_tree) and pin.rs are reached whole, and the report witnesses only the plugin files at start and the pinned copy under bin, not the tree's check order and write rule.
@answer n=17 entity=authentication-authorization#2.1 v=no
why: the observed server pid is read in viola-channel client.rs and tokio_client.rs, which the change did not reach.
@answer n=18 entity=authentication-authorization#2.2 v=no
why: the peer euid reading and admission sit in viola-channel client.rs, which the change did not reach.
@answer n=19 entity=authentication-authorization#2.3 v=no
why: the server verdict and verify_server live in viola-state liveness.rs, which the change did not reach.
@answer n=20 entity=authentication-authorization#2.4 v=no
why: the no-endpoint reading lives in viola-state liveness.rs, which the change did not reach; the reached client.rs only calls it.
@answer n=21 entity=authentication-authorization#2.5 v=yes mode=stale
why: the link and unlink check_instance call through client::refuse_untrusted sits in the reached src/cmd/client.rs, and the report does not show that ordering still holds.
@answer n=22 entity=authentication-authorization#2.6 v=yes mode=stale
why: the CLI refusal order and output sit in the reached files src/cmd/client.rs and src/cmd/send.rs, and no witness re-reads the refusal order.
@answer n=23 entity=authentication-authorization#2.7 v=no
why: the MCP frame mapping lives in viola-mcp tools.rs and result.rs, which the change did not reach.
@answer n=24 entity=authentication-authorization#2.8 v=no
why: tests/security_negatives_socket_dir.rs is green on the ubuntu-latest leg of CI run 42288148289 (seven jobs green on 849aad18), an unmoved gate reading that re-runs its negatives.
@answer n=25 entity=authentication-authorization#2.9 v=no
why: the three named unit tests are green in CI run 42288148289 on 849aad18, an unmoved gate reading, and the change adds no injected-value seam.
