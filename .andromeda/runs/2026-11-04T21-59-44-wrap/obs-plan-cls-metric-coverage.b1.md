@answer n=1 entity=metric-coverage#1.1 v=no
why: the session changed no diagnostics writer or event field, so the Counter grouping still reads the same records
@answer n=2 entity=metric-coverage#1.2 v=no
why: no raw duration_ms sample source changed, as the session moved only test files and product code is unchanged
@answer n=3 entity=metric-coverage#1.3 v=no
why: the liveness rule and budget.read_at are unchanged, the stale-row witness only exercises the existing rule in a test
@answer n=4 entity=metric-coverage#2.1 v=no
why: the hook-decision duration source and hook_event values are unchanged, no hook code moved
@answer n=5 entity=metric-coverage#2.2 v=no
why: channel-response duration_ms and the method set are unchanged, no channel code moved
@answer n=6 entity=metric-coverage#2.3 v=no
why: send-confirmed duration_ms and src/run/send.rs are unchanged, the session moved no send code
@answer n=7 entity=metric-coverage#2.4 v=no
why: the send-confirmed and send-refused outcome fields are unchanged, the session added no refusal or confirmation
@answer n=8 entity=metric-coverage#2.5 v=no
why: channel-response{method:"wait"}.outcome is unchanged, and the macOS wait verification statement keeps its behaviour
@answer n=9 entity=metric-coverage#2.6 v=no
why: state-recovered.detail and the chaos heal path are unchanged, the session added no recovery code
@answer n=10 entity=metric-coverage#2.7 v=no
why: the list --json skipped field is unchanged, the list change is a terminal-stream test assertion only
@answer n=11 entity=metric-coverage#2.8 v=no
why: parse-rejected{parser, detail, count} is unchanged, no parser code moved
@answer n=12 entity=metric-coverage#2.9 v=no
why: the send-refused unknown and unsupported-version reasons are unchanged, the session changed no refusal codes
@answer n=13 entity=metric-coverage#2.10 v=no
why: the process-exit fields and exit_source values are unchanged, the session's SIGKILL test changes no product exit line
@answer n=14 entity=metric-coverage#2.11 v=no
why: event:"panic" and the panic hook are untouched, no src file changed
@answer n=15 entity=metric-coverage#3.1 v=no
why: the macOS server-verify refusal as an euid mismatch alone holds on unchanged behaviour, and this row names no case count
@answer n=16 entity=metric-coverage#3.2 v=no
why: refuse_untrusted writes no line and its exit code 21 is unchanged, the session changed no client code
@answer n=17 entity=metric-coverage#4.0 v=no
why: the hook-decision quantile snippet reads unchanged hook_event and duration_ms fields, no hook source moved
