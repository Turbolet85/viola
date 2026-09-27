
## 2026-09-27-wrapper-channel — viola-channel landed: Unix endpoint path and arbiter, frame rules, bind step, dependencies
**Section:** §Stack (Wrapper IPC, Logging) · §Established Decisions [Session Liveness], [Error Handling] · §Conventions (CLI exit code 1) · §Standard Contracts (Instance snapshot, Wrapper channel frames) · §Occupied Resources (IPC endpoints, Workspace crates, Repository `fuzz/`) · §Infrastructure Patterns (code-graph rust plane, Licence, Crate dependency direction, directory tree, local pre-push gate)
**Change:**
- IPC endpoints, Unix: was `$TMPDIR/viola-<h12>.sock`; now `<socket dir>/viola-<h12>.sock` in the per-user 0700 dir (`$XDG_RUNTIME_DIR/viola/`, `$TMPDIR/viola/`, fallback `/tmp/viola-<uid>/`; security-plan arch amendment 2 folded), chmod 0600 after the bind, the `<socket dir>/viola-<h12>.lock` sibling as the start arbiter. Test-only `viola-test-chan-<pid>-<label>` registered beside `viola-test-sqos-*`.
- Frames: one line ≤ `MAX_FRAME` (`\n` counted), longer → `-32600` + close; only `hook.event` id-less, any other id-less frame not dispatched (`parse-rejected{channel-frame, malformed}`); `conn` stripped before dispatch, never identity.
- Start order: the endpoint bind has its step (taken → exit 1 `squatted-name`, fixed stderr pair); `endpoint` is present from the first snapshot; only the version gate still has no step.
- Stack: tracing gains `attributes`; veil `=0.3.0` (no `toggle`); the server DACL converted by windows-sys `Win32_Security_Authorization`; libc for the Unix socket dir.
- `viola-channel` joins Landed so far, the sync crates, the rust plane and the licence list; its deps + serde, serde_json, thiserror, tracing, veil, libc (sync, tokio-free as landed). `viola-pty` gains tracing, so [Error Handling]'s exact set is portable-pty, tracing, windows-sys, libc.
- Fuzz targets `viola_name` and `channel_frame`. Pre-push: `vm-release` (WSL VM terminated after the ubuntu verdict) and `windows-tests` on the host; host stages at `CARGO_BUILD_JOBS=16`.
**Why:** the wrapper channel chunk landed the crate and its bind. Detector proposals citing source or manifest lines were rejected and their report-carried facts re-raised by the orchestrator; the collateral facts they carried (dir fallback on a relative value, lock mode, unlink order, dependency versions) were not written.
**Ref:** .andromeda/runs/2026-09-27T12-33-51-wrap/
