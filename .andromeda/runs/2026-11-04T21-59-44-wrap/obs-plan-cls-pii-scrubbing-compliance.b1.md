@answer n=1 entity=pii-scrubbing-compliance#1.1 v=no
why: the session is test-only and changes no product code, secret-scan pattern or reached file that holds the Cookie-header rule
@answer n=2 entity=pii-scrubbing-compliance#1.2 v=no
why: the R8 env strip is product code under src, which the session did not move, so the CLAUDE* rule stands
@answer n=3 entity=pii-scrubbing-compliance#1.3 v=no
why: v1.x signing credentials are absent from the runtime and .github/ was not touched, so the CI-secret rule stands
@answer n=4 entity=pii-scrubbing-compliance#1.4 v=no
why: the veil redact derives on SendParams, LastResult and HookEventParams sit in src, which the session did not touch
@answer n=5 entity=pii-scrubbing-compliance#1.5 v=no
why: the redact attributes on HookDialogParams and AnswerParams sit in src, which the session did not touch
@answer n=6 entity=pii-scrubbing-compliance#1.6 v=no
why: the drift-report Display seam is product code under src, untouched by this test-only session
@answer n=7 entity=pii-scrubbing-compliance#1.7 v=no
why: the statusline.shell_out span is product code under src, untouched by this test-only session
@answer n=8 entity=pii-scrubbing-compliance#1.8 v=no
why: the basename and path-free rules are product logging code under src, and the session moved no path-bearing source
@answer n=9 entity=pii-scrubbing-compliance#1.9 v=no
why: config and endpoint logging is product code under src, untouched by this test-only session
@answer n=10 entity=pii-scrubbing-compliance#1.10 v=no
why: the structured pid, budget and timestamp fields are product code under src, untouched by this session
@answer n=11 entity=pii-scrubbing-compliance#1.11 v=no
why: claude agents row handling is product code under src, which the session did not move
@answer n=12 entity=pii-scrubbing-compliance#1.12 v=no
why: the link and unlink logging is product code under src, and tests/cli_links.rs, which asserts it, is not a reached file
@answer n=13 entity=pii-scrubbing-compliance#1.13 v=no
why: the closed protocol-code enums live in src and schemas/diag-line.v1.json, and the session moved neither
@answer n=14 entity=pii-scrubbing-compliance#1.14 v=no
why: recorded fixtures are tests-owned, the session added no fixture content, and fixtures/ did not move
@answer n=15 entity=pii-scrubbing-compliance#2.1 v=no
why: the session's tests/support/mod.rs change only adds the frozen module, leaving tests/support/events.rs, the secret scan, the canary and the upload rules unchanged with .github/ untouched
