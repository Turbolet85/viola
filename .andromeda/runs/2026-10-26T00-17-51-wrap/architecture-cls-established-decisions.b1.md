# Classified — architecture · established-decisions.b1

## Answers
established-decisions#1.1 · no · the session adds no Tokio, and the cargo deny bans and channel-tokio-free checks re-read green
established-decisions#2.1 · no · the reached run.rs and main.rs changes are test code only, main's DLL-search restriction keeps its second-statement place, and src/conpty.rs and vendor/conpty are unedited
established-decisions#3.1 · no · the reached gate.rs change is test code only, so no product feed, quiet-period or verdict behaviour moved
established-decisions#4.1 · no · the reached send/test_support.rs change is test code only, so no send path, confirmation window or refusal moved
established-decisions#5.1 · no · viola-state is unchanged and the reached budget.rs change is test code only, so no log or budget write rule moved
established-decisions#6.1 · no · viola-state is unchanged, so the shared replace helper and its retry bounds hold as written
established-decisions#7.1 · yes · add-to · the Windows listener's remote-client rejection and anonymous-token DACL denial are now witnessed on the windows-2025 CI leg, where a DACL text read-back alone asserted them before
established-decisions#8.1 · no · viola-mcp and its tools are unchanged, and the same-user trust sentence holds since the new witnesses cover other-user and remote admission only
established-decisions#9.1 · no · plugin/ and the registered hooks are unchanged, so the exec-form tier map holds
established-decisions#10.1 · no · the reached statusline.rs change is test code only, and the capture arm's silent exit-0 contract holds
established-decisions#11.1 · no · the reached dialog.rs change is test code only, so the dialog-expired line is unchanged
established-decisions#11.2 · no · the recorded 671 s live measurement is untouched by a test-only session
established-decisions#12.1 · no · the pending ceiling and its two owning route entries are untouched by a test-only session
