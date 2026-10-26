# Classified — obs · pii-scrubbing-compliance.b1

## Answers
pii-scrubbing-compliance#1.1 · yes · the new test homes under target/e2e-home fall under the secret scan's Cookie-header form
pii-scrubbing-compliance#1.2 · yes · the run start's spans and lines carry no env value; the identity-floor canaries cover the new test homes
pii-scrubbing-compliance#1.3 · no · the v1 runtime holds no signing credential and the chunk touches no CI secret path
pii-scrubbing-compliance#1.4 · yes · the send-side refusal line the CARRY test reads must carry no sent text (SendParams.text)
pii-scrubbing-compliance#1.5 · no · the chunk reaches no hook, dialog, answer or plan path; its new lines carry no request params
pii-scrubbing-compliance#1.6 · no · no serde drift-report path is in the chunk's scope
pii-scrubbing-compliance#1.7 · no · the statusline shell-out, its span and the statusline command are outside the chunk's named scope
pii-scrubbing-compliance#1.8 · yes · the squatted refusal, the process-exit line and the bind event stay path-free; new panic lines carry a workspace-relative panic_location
pii-scrubbing-compliance#1.9 · yes · the endpoint name and pinned bin path never reach a span or line; only endpoint_kind is logged
pii-scrubbing-compliance#1.10 · yes · the exit-1 process-exit detail code and the squatted refusal carry no pid
pii-scrubbing-compliance#1.11 · no · no claude agents --json row is read or logged on the chunk's paths
pii-scrubbing-compliance#1.12 · yes · the new listener refusal code and the send-refused refusal code are closed kebab-case enums; squatted-name is unchanged
pii-scrubbing-compliance#1.13 · no · the chunk records and reads no fixture
pii-scrubbing-compliance#2.1 · yes · the CARRY and integration inputs must be synthetic with the fixed canary, which may not appear in any home-level diagnostics line
