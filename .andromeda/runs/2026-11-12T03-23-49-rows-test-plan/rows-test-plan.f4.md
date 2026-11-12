@row_add n=1 row=contract/wrapper-channel/cursor area=5-integration-test-strategy rec=1,2
boundary: the wrapper channel
item: cursor
promise: A result's cursor equals the events.ndjson length at acceptance.
@row_add n=2 row=contract/wrapper-channel/-32602 area=5-integration-test-strategy rec=1,3
boundary: the wrapper channel
item: -32602
promise: A request with a higher v is too new and is answered -32602 with data {supported, wrapper}.
@row_add n=3 row=contract/wrapper-channel/control-character area=5-integration-test-strategy rec=1,2
boundary: the wrapper channel
item: control-character
promise: In wrapper-side paste validation, send or answer text holding ESC, DEL or U+0085 is refused {"refusal":"not-delivered","detail":"control-character"}, and control-character wins over every other refusal.
@row_add n=4 row=contract/event-line/ts area=5-integration-test-strategy rec=4,5
boundary: the event line
item: ts
promise: ts is RFC 3339 UTC with ms and Z.
@row_add n=5 row=contract/strict-modes-refusal/hook area=5-integration-test-strategy rec=4,5
boundary: strict-modes refusal
item: hook
promise: A strict-modes refusal in the hook exits 0 with no output, reading no config.json or stdin, opening no role file and writing no frame, no line and no SessionEnd append.
@row_add n=6 row=contract/viola-hook/no-viola-name area=5-integration-test-strategy rec=6,7,8
boundary: viola hook
item: no VIOLA_NAME
promise: With no VIOLA_NAME every hook exits 0 immediately with empty stdout and stderr, and no instances directory is created.
@row_add n=7 row=decision/controls-not-disableable area=5-integration-test-strategy rec=9,10
name: Controls not disableable
rule: No setting, whether a VIOLA_* variable, global flag or config.json key, disables a control, the socket directory check among them.
guard: setting_does_not_disable_the_socket_dir_check, every_flag_variable_and_config_key_is_a_row_of_the_table
@row_add n=8 row=decision/tty-asserted-only-on-verb-sgr area=6-e2e-test-strategy rec=11
name: TTY rendering asserted only on verb SGR
rule: CLI verb tests pin --json documents and human layouts by literals, and assert TTY rendering only on the SGR sequences in verb output under the outer PTY, never on the child agent's screen.
@row_add n=9 row=decision/hook-deadline-driver area=6-e2e-test-strategy rec=13
name: viola hook driven under an Instant deadline
rule: viola hook and hook statusline are driven by std::process::Command with an Instant deadline over recorded fixture stdin.
@row_add n=10 row=decision/env-clear-keeps-llvm-profile-file area=6-e2e-test-strategy rec=13,14
name: env_clear tests re-add LLVM_PROFILE_FILE
rule: Any test that uses env_clear() must re-add LLVM_PROFILE_FILE.
