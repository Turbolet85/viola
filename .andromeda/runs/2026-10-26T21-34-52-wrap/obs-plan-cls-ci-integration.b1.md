# Classified — obs-plan · ci-integration.b1

## Answers
ci-integration#1.1 · no · the claim still holds on the changed src/obs.rs, which refuses only a widened diagnostics file at open and moves no path, as the green G2 guard shows
ci-integration#1.2 · no · the agent-run merge and its if: failure() capture sit in the unreached harness and workflow, and the report states no harness, CI or gate change
ci-integration#1.3 · no · the JUnit path and its upload sit in the unreached nextest config and workflow, and the row names no test count
ci-integration#1.4 · no · the five hook row names and the export paths are unchanged, and the re-measured run --perf maxima stay under SPINE_DEADLINE
ci-integration#1.5 · no · the row is N/A, since no OTel exporter or telemetry dependency was added
ci-integration#2.1 · no · the lint and deny gates are unchanged and green on the changed tree (ci#40403389629), and no dependency changed
ci-integration#2.2 · no · the claim still holds on the changed schemas/diag-line.v1.json, whose one change adds an enum word that widens what validates, and the unit tests stay green on ci#40403389629
ci-integration#2.3 · no · the claim still holds on the changed tests/support/home.rs, since the green G2 guard still finds homes under target/e2e-home and a deliberate carve-out panic would turn G2 red under that root
ci-integration#2.4 · no · the E2E harness, the G2 and G4 steps and their uploads are unchanged, and the run ci#40403389629 is green on three OSes
ci-integration#2.5 · no · the mutants job scores src/panic_frames.rs, which the session does not reach, and no module was added or removed (Untouched, §1)
ci-integration#3.1 · no · G1 scans every workspace Rust file, and the green run ci#40403389629 passes it with its rg step unchanged
ci-integration#3.2 · no · src/cmd/hook/seam.rs is unreached and the forced seam panic still fires after obs init (F5), so the exemption and the probes read as before
ci-integration#3.3 · no · the claim still holds on the changed schemas/diag-line.v1.json, whose one change is an added enum word that G4 still validates every G2-scope line against, as the green run ci#40403389629 shows
ci-integration#4.1 · no · the test-then-gate order and the always() conditions are untouched, and the green G2 guard shows the homes survive to the gate with the viola-chaos carve-out still outside the root
