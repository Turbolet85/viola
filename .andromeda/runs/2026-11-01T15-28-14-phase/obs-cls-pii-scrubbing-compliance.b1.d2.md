# Classified — obs · pii-scrubbing-compliance.b1

## Answers
pii-scrubbing-compliance#1.1 · no · standing S9 · new test homes under target/e2e-home/ are covered by the secret scan
pii-scrubbing-compliance#1.2 · yes · the new bind, connect and refusal spans and lines must keep the socket path out of every line (scope: socket path in no span or line); R8 strip fields and the canary scan are out per Not needed
pii-scrubbing-compliance#1.3 · no · the chunk adds or touches no signing credential and no CI secret
pii-scrubbing-compliance#1.4 · yes · the new hook connect fail-open line in hook-<name>.ndjson and any new connect-path span must carry no prompt text or assistant output (skip_all, no payload field)
pii-scrubbing-compliance#1.5 · yes · the hook connect and hook.dialog fail-open traces must hold no tool input, plan text or dialog text, codes only
pii-scrubbing-compliance#1.6 · no · the chunk adds no serde_path_to_error parse or drift report
pii-scrubbing-compliance#1.7 · no · statusline_command is not reached
pii-scrubbing-compliance#1.8 · yes · new refusal lines and details carry no absolute path; file fields stay basenames and bind or directory failure details carry no path
pii-scrubbing-compliance#1.9 · yes · the endpoint name and socket path appear in no span or line, only the endpoint_kind form
pii-scrubbing-compliance#1.10 · yes · new refusal details carry no pid, pid stays a typed field, and the macOS pid-gap carve-out is named
pii-scrubbing-compliance#1.11 · no · claude agents --json rows are not reached
pii-scrubbing-compliance#1.12 · yes · client::refuse_untrusted is shared by link and unlink, so their new euid and connect refusals must log no driver or driven name
pii-scrubbing-compliance#1.13 · yes · new refusal codes are closed kebab-case and admitted in schemas/diag-line.v1.json; channel-response's detail stays the three -32602 values since a refused connection writes no pair
pii-scrubbing-compliance#1.14 · no · recorded fixtures are tests-owned and not reached
pii-scrubbing-compliance#2.1 · yes · the new refusals on the six frame verbs, the mcp frame tools and the wait/last connect path print only fixed pairs or the exact instance-unreachable body with no line, and their bodies join the error-body census and the cli and mcp live witnesses
