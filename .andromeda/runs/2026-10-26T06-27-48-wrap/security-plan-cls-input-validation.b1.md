# Classified — security-plan · input-validation.b1

## Answers
input-validation#1.1 · no · no source changed, so the paste-text validator and its callers are the same code
input-validation#1.2 · no · no source changed, so the dialog free-text path still runs the same validate_paste_text
input-validation#1.3 · no · no channel framing source changed, and the serde, serde_json and nutype versions it names are unmoved in the lock
input-validation#1.4 · no · no hook stdin source changed, and the serde_path_to_error and chrono versions it names are unmoved in the lock
input-validation#1.5 · no · no MCP handler source changed, and the rmcp and schemars versions it names are unmoved in the lock
input-validation#1.6 · no · no CLI argument or home-resolution source changed, and the clap version it names is unmoved in the lock
input-validation#1.7 · no · no config.json reader changed, and the nutype version it names is unmoved in the lock
input-validation#1.8 · no · no registry-name reader changed, and the windows-sys version it names is unmoved in the lock
input-validation#1.9 · no · no seam source changed, and the session adds no env var
input-validation#1.10 · no · no seam source changed, and the session adds no env var
input-validation#1.11 · no · no colour-fact read changed, and the session adds no env var
input-validation#1.12 · no · no state-file reader changed, and the report leaves input-validation's readers untouched
input-validation#1.13 · no · no tailing source changed, and the notify version it names is unmoved in the lock
input-validation#1.14 · no · no child-output reader changed, and the session makes no child-spawn change
input-validation#1.15 · no · no vt100 feed or screen source changed, and the vt100 version it names is unmoved in the lock
input-validation#1.16 · no · no program-resolution source changed, and the report shows no child-spawn surface changed
input-validation#1.17 · no · no session-id or resume-argument source changed
input-validation#1.18 · no · no statusline source read changed
input-validation#1.19 · no · no statusline shell-out source changed
input-validation#1.20 · no · no DLL-search or ConPTY companion source changed, the vendored binaries did not move, and the windows-sys version it names is unmoved in the lock
input-validation#1.21 · no · the session adds no upload, webhook or database surface
