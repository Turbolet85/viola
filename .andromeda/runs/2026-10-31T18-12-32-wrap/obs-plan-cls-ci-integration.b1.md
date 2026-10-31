# Classified — obs-plan · ci-integration.b1

## Answers
ci-integration#1.1 · no · the upload glob `target/e2e-home/**` already covers the new binary's homes, and the report's Untouched list says nothing in `.github` moved
ci-integration#1.2 · no · the harness logs and status artifact is `target/agent-run/`, and no harness verb or status shape moved
ci-integration#1.3 · yes · stale · `.config/nextest.toml` is a reached file and the report does not show its `[profile.ci.junit]` path still holds on the changed file
ci-integration#1.4 · no · the perf rows and their upload sit in `.github`, which is untouched, and no perf row or budget moved
ci-integration#1.5 · no · `Cargo.toml` is reached but no dependency was added or bumped, so no OTel exporter enters
ci-integration#2.1 · no · CI green on the final HEAD and pre-push 3403 passed, so the lint, `cargo check` and `cargo deny` gates still hold on the changed files
ci-integration#2.2 · no · `schemas/diag-line.v1.json` grew only the process-exit `detail` list, and the in-memory tests still validate against it
ci-integration#2.3 · no · the new root binary and the setups moved to `tests/support/start.rs` keep their homes under `target/e2e-home/`, as the task's Touches state
ci-integration#2.4 · no · G2 and G4 still run and read the schema, whose only moved list is process-exit `detail`, and no verb or gate moved
ci-integration#2.5 · no · the scored counts are from an earlier dispatch on `src/panic_frames.rs`, which the session does not reach
ci-integration#3.1 · no · the G1 rg rule lives in `.github` and `scripts`, which are untouched, and CI is green on the final HEAD
ci-integration#3.2 · no · the G2 script and the seam path `src/cmd/hook/seam.rs` are not reached (`src/cmd/hook.rs` is not in the diff), and no panic line moved
ci-integration#3.3 · no · G4 still checks every line against `schemas/diag-line.v1.json`, which grew one enum list, and this row names no list
ci-integration#4.1 · no · step order and the keep-homes rule sit in `.github/workflows/ci.yml`, which is untouched, and the new homes stay under `target/e2e-home/`
ci-integration#5.1 · no · the secret-scan reading sits in `.github/workflows/ci.yml`, which is untouched, and no scan step moved
