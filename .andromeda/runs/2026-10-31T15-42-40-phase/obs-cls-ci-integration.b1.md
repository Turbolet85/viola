# Classified — obs · ci-integration.b1

## Answers
ci-integration#1.1 · no · standing S1 · the target/e2e-home/ root that the diag-artifact upload covers; the chunk's new homes stay under that root and inherit the upload unchanged
ci-integration#1.2 · no · the harness capture of agent-run logs and status copies files only; the chunk moves no word in it and adds no capture
ci-integration#1.3 · no · the JUnit upload condition on every test job; the chunk edits no workflow and its new test IDs ride the upload unchanged
ci-integration#1.4 · no · hyperfine perf exports and their per-OS perf job; the chunk leaves the perf budgets and hook latency gates as they are
ci-integration#1.5 · no · OTel trace export is N/A in v1; the chunk adds no exporter or egress
ci-integration#2.1 · no · standing S2 · the clippy lint gate (disallowed-macros, print and dbg bans, G1) that every change of the chunk's kind owes; S8 and S9 name its print and span rules
ci-integration#2.2 · yes · the in-memory unit tests parse emitted lines against schemas/diag-line.v1.json, which the chunk edits with the new process-exit detail values, so those values must validate there
ci-integration#2.3 · no · standing S1 · every integration-test home lives under target/e2e-home/, which the new cross-exit test file's homes follow; the chaos carve-out is untouched
ci-integration#2.4 · no · standing S6 · the zero-panic gate G2 and schema gate G4 over test homes; the chunk's schema obligation is carried by #2.2 and #3.3
ci-integration#2.5 · no · the epoch-boundary mutation run over obs code is a check every obs change owes equally; the chunk names no mutation run and adds none
ci-integration#3.1 · no · standing S9 · the bare-#[instrument] gate G1 with the skip_all first-argument form, which every span-bearing change owes
ci-integration#3.2 · no · standing S6 · the G2 zero-panic gate over home-level role files; the seam exemption is untouched
ci-integration#3.3 · yes · G4 checks each process-exit line of the G2 scope against schemas/diag-line.v1.json, and the chunk's new detail values must be in that schema for those lines to pass
ci-integration#4.1 · no · standing S1 · homes are kept until G2, G4 and the scan have read them (AGENT_RUN_KEEP_HOMES), the home lifetime every integration-test home owes
ci-integration#5.1 · no · the v1-47 secret-scan timing is an open decision the chunk neither moves nor reads; its new homes are scanned at job end as any home is
