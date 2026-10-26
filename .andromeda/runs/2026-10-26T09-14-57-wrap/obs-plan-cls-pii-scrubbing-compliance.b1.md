# Classified — obs-plan · pii-scrubbing-compliance.b1

## Answers
pii-scrubbing-compliance#1.1 · no · the Cookie-header secret scan over all diagnostics/ is outside the reached files, and the task says the secret scan covers the new homes and lines
pii-scrubbing-compliance#1.2 · no · R8 strip is not changed (E2 untouched), and the secret scan covers the new homes and lines, so the identity-floor canary rule holds
pii-scrubbing-compliance#1.3 · no · no scripts/, .github/ or dependency file changed, so no signing credential enters the runtime or a CI artifact
pii-scrubbing-compliance#1.4 · no · the new check adds only codes-only process-exit and hook-decision lines and opens no channel.request on a mismatch, so prompt and assistant text reach no home-level line
pii-scrubbing-compliance#1.5 · no · a dialog mismatch writes only hook-invoked and a codes-only hook-decision, with no hook.dialog response, so dialog text stays out of home-level lines
pii-scrubbing-compliance#1.6 · yes · stale · the change reaches tests/contract_fake_agent_drift.rs and the hook and channel read paths, and no report line shows the detail-only routing of drift reports holds on them
pii-scrubbing-compliance#1.7 · no · the hook statusline arm and its shell-out span are not reached, as the report's untouched list states
pii-scrubbing-compliance#1.8 · no · the new lines hold no path (asserted), so home-level lines stay path-free, and panic hooks are not reached
pii-scrubbing-compliance#1.9 · no · the new lines hold no endpoint (asserted), and live_endpoint only returns the LiveEndpoint struct
pii-scrubbing-compliance#1.10 · no · the new lines hold no pid or start time (asserted), and the server_pid readers write and log nothing
pii-scrubbing-compliance#1.11 · no · the change adds no agent row or unwrapped session id to any output, and the reached set names no row reader or list renderer
pii-scrubbing-compliance#1.12 · no · no event value, schema, exit code or --json code was added, and server-verify-failed is the existing closed code
pii-scrubbing-compliance#1.13 · no · fixtures are not a telemetry source, and the report adds no fixture content to obs
pii-scrubbing-compliance#2.1 · yes · add-to · F7: tests/security_negatives_server_identity.rs joins the verification tests, scanning every output and the cli line with digit_runs for a pid, start time, path or endpoint in a server-verification refusal
