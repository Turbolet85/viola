@answer n=1 entity=pii-scrubbing-compliance#1.1 v=no
why: the cookie-header secret-scan rules are unchanged (only the secret_scan.rs OS arm leaves) and the cookie scrub claim is untouched
@answer n=2 entity=pii-scrubbing-compliance#1.2 v=no
why: the R8 strip still logs names only and never reads values, the session only drops the Windows registry reader and the case-insensitive fold, and the identity floor stays
@answer n=3 entity=pii-scrubbing-compliance#1.3 v=no
why: the removed workflows and pwsh and vendor steps are not secret-echo paths, so the no-echo claim holds
@answer n=4 entity=pii-scrubbing-compliance#1.4 v=no
why: the veil Redact and PII classification of prompt and assistant text are untouched by the session
@answer n=5 entity=pii-scrubbing-compliance#1.5 v=no
why: the #[redact] markers on tool input and dialog answers are untouched by the session
@answer n=6 entity=pii-scrubbing-compliance#1.6 v=no
why: the serde drift report seam is not among the session's changes
@answer n=7 entity=pii-scrubbing-compliance#1.7 v=no
why: the statusline.shell_out span is not among the session's changes
@answer n=8 entity=pii-scrubbing-compliance#1.8 v=no
why: the session removes only Windows and macOS path code, so the basename fields and workspace-relative panic_location on Linux hold
@answer n=9 entity=pii-scrubbing-compliance#1.9 v=no
why: endpoint and pinned_bin stay unlogged, and endpoint_kind is not among the diag-line words the session removes
@answer n=10 entity=pii-scrubbing-compliance#1.10 v=no
why: the typed pid, timing and budget fields are not in the process-exit detail catalog or any other changed closed list
@answer n=11 entity=pii-scrubbing-compliance#1.11 v=no
why: claude agents rows and parse-rejected counts are not touched by the session
@answer n=12 entity=pii-scrubbing-compliance#1.12 v=no
why: the link and unlink logging is not changed, and the session's channel change is the endpoint surface only
@answer n=13 entity=pii-scrubbing-compliance#1.13 v=no
why: the channel-response detail enum is not among the schema words the session removes, and the refusal enums stay closed
@answer n=14 entity=pii-scrubbing-compliance#1.14 v=no
why: removing Windows and macOS fixtures does not change that obs adds no fixture content
@answer n=15 entity=pii-scrubbing-compliance#2.1 v=yes mode=contradict
why: the verification block still names the Windows arms the session removes, namely the mutants host scratch with HOST_SCRATCH and scratch_bytes, the Windows DACL strict-modes check, and the Linux-and-Windows and Linux-and-macOS test scopes
