# Classified — obs-plan · ci-integration.b1

## Answers
ci-integration#1.1 · no · the diagnostics upload path, its retention and its gating are CI-owned, and the session changed no CI step
ci-integration#1.2 · no · the agent-run logs and status upload is unchanged; the cleanup change moves only the endpoint_gone reading and keeps the document's shape
ci-integration#1.3 · yes · stale · `.config/nextest.toml` is a reached file whose ci profile gained a binary term (kill line 25 to 26), and the report does not show the junit path claim still holds on the changed file
ci-integration#1.4 · no · the per-hook perf export and its max assertion are unchanged, and the measured hook means of 2.0 to 2.3 ms leave the timings-only artifact rule intact
ci-integration#1.5 · no · no OTel exporter was added, so the N/A row still holds
ci-integration#2.1 · no · CI is green on all three OSes, the lint probes and G1 ran green and were not edited, and viola-channel stays sync and Tokio-free with no dependency added
ci-integration#2.2 · no · diag-line.v1.json is still the schema the unit tests parse against; the change only grew its detail enums, which this row does not state
ci-integration#2.3 · no · the report moves no rule on homes, and the new SocketRoot helper places socket roots under /tmp, not viola homes outside target/e2e-home
ci-integration#2.4 · no · the cleanup change touches only the endpoint_gone reading, and the agent-run logs, status and G2/G4 rule this row names is not moved
ci-integration#2.5 · no · the row scopes obs code only, and the new always-admit survivor sits in product code in server.rs, which this row does not cover
ci-integration#3.1 · no · the G1 gate command ran green and was not edited, and the report shows its claim holds
ci-integration#3.2 · no · G2 ran green and scripts/g2-zero-panics.sh was not edited; the seam exemption is by the exact path src/cmd/hook/seam.rs, which the change did not reach
ci-integration#3.3 · no · G4 still checks every scoped line against the schema, obs still owns both schemas, and diag-line.v1.json changed only in enum counts that this row does not state
ci-integration#4.1 · no · the report moves no rule on homes, AGENT_RUN_KEEP_HOMES and the gate order are untouched, and no CI step changed
ci-integration#5.1 · no · the report moves no home rule, ci.yml keeps AGENT_RUN_KEEP_HOMES and the if: always() scan, and the viola-chaos home stays out
