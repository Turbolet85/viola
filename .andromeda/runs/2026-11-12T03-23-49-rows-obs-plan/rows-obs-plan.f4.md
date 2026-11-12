@row_add n=1 row=decision/no-process-log-without-instance area=obs-scope-summary rec=1,2
name: No process log without an instance
rule: A short-lived CLI verb run with no resolved instance writes no process-log file; its human stderr carries only fixed-message lines, and its outcome is the exit code and stdout.
@row_add n=2 row=decision/strict-modes-refusal-writes-no-line area=obs-scope-summary rec=1,3,4,5,6,7
name: Strict-modes refusal writes no line
rule: A strict-modes check refused before any log opens ends the process with nothing under the home opened and no log line; its cause is carried only by the fixed stderr pair or the --json document.
why: Ratified as the answer to whether a refused strict-modes check logs.
except: The ledger-stamps read in run's version gate and a refused update_stamps in viola verify each leave a line.
@row_add n=3 row=contract/service-identity/service-name area=obs-scope-summary rec=8,9
boundary: Service identity
item: service.name
promise: Hardcoded "viola", the binary name, never read from OTEL_SERVICE_NAME or SERVICE_NAME.
@row_add n=4 row=decision/no-secrets-in-log-lines area=obs-scope-summary rec=10,11
name: No secrets in log lines
rule: No log line may contain a Cookie header or any stripped CLAUDE* value.
guard: secret-scan test
@row_add n=5 row=contract/channel-response/detail area=obs-scope-summary rec=12,13,14,15,16,17
boundary: Channel response line
item: detail
promise: On a channel-response answering -32602, detail is one closed word: unsupported-version, invalid-params or release-from-driver, on both the server's and the client's line; it is absent on every other channel-response line and names no param or value.
@row_add n=6 row=contract/hook-decision-detail/socket-dir-refused area=span-trace-coverage rec=18,19,20,21
boundary: hook-decision line
item: socket-dir-refused
promise: A refused socket directory writes one hook-decision{detail:"socket-dir-refused"} at WARN, on the dialog path after hook-invoked{corr:null}, with decision_emitted:false; exit 0, empty streams, no frame.
@row_add n=7 row=contract/config-json/dialog-hold-s area=span-trace-coverage rec=22,23
boundary: config.json
item: dialog_hold_s
promise: The dialog hold, a whole number of seconds from 60 to 3600, default 60; any other value gives one parse-rejected malformed WARN line and the hold takes 60.
