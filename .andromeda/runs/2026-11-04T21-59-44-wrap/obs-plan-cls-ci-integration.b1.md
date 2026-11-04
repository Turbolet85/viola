@answer n=1 entity=ci-integration#1.1 v=no
why: the session did not touch the diag upload step or its path, since .github/ and ci.yml did not move
@answer n=2 entity=ci-integration#1.2 v=no
why: the harness artifact and its if-failure capture step are in ci.yml, which the session did not change
@answer n=3 entity=ci-integration#1.3 v=yes mode=add-to
why: F1 applies, since scripts/junit-os-sets.py now consumes the junit-windows-2025, junit-macos-latest and junit-ubuntu-latest artifacts through gh run download as an operator-pass reading
@answer n=4 entity=ci-integration#1.4 v=no
why: the hyperfine perf export and its per-OS upload are unchanged, and the session ran no perf step change
@answer n=5 entity=ci-integration#1.5 v=no
why: the row records no OTel exporter, and the session added none
@answer n=6 entity=ci-integration#2.1 v=no
why: the lint stage (clippy, lint-probes.sh, cargo deny) is unchanged, since the session added no lint rule or script
@answer n=7 entity=ci-integration#2.2 v=no
why: the in-memory writer unit tests and their schema check are unchanged, since no product source or schema moved
@answer n=8 entity=ci-integration#2.3 v=no
why: the fake-agent diagnostics homes and their target/e2e-home root are unchanged, since the session moved only the Frozen guard and a ps reader
@answer n=9 entity=ci-integration#2.4 v=yes mode=add-to
why: F3 applies, since the census binary contract_os_gated_tests joins the E2E layer run under run --e2e
@answer n=10 entity=ci-integration#2.5 v=no
why: the mutation job and its obs-code scope are unchanged, since no obs code changed
@answer n=11 entity=ci-integration#3.1 v=no
why: the G1 bare-instrument rg check is unchanged, since .github/ and the instrument call sites did not move
@answer n=12 entity=ci-integration#3.2 v=no
why: the G2 script and its fake-agent seam exemption are unchanged, since src/cmd/hook/seam.rs did not move
@answer n=13 entity=ci-integration#3.3 v=no
why: the G4 schema-conformance check and its scope are unchanged, since the schemas and the check body did not move
@answer n=14 entity=ci-integration#4.1 v=no
why: the test-job order, the target/e2e-home root and the chaos carve-out all stand, since the session moved no home path
@answer n=15 entity=ci-integration#5.1 v=no
why: the secret-scan timing and the keep-homes setting are in ci.yml, which the session did not change
