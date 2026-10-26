# Cascade compare — security-plan · phase · 2026-10-25-windows-endpoint-admission
E items 91 · held 11 · lost at classify 4 · lost at strong 0 · beside a placed fact - · cascade only 8 · body 57 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### bootstrap-phases#2.1 · lost at classify · .andromeda/security-plan-detail/bootstrap-phases.md:24
- (.andromeda/security-plan-detail/bootstrap-phases.md:24) The chunk's new Windows tests run in the CI `test` job, whose artifact canary scan must pass before any scan-gated upload.
  > The CI `test` job does run obs-plan §9's artifact canary scan: `viola-harness secret-scan` over test-home diagnostics, the harness capture except `run --mutants`' `target/agent-run/chunk.diff`, and the nextest JUnit report, checked against this plan's NEVER-log floor. That one file is repository source text by construction (it trips the scan's own needles whenever it touches the scan's source), so it is skipped by exact path and the `harness-<os>` upload drops it (`!target/agent-run/chunk.diff`): it is never scanned and never uploaded, and a `chunk.diff` anywhere else under the capture is still scanned. It never prints or writes the matched bytes, and every scan-gated upload (`diag-`, `junit-`, `harness-<os>`) waits for it to pass.

### error-handling#1.1 · lost at classify · .andromeda/security-plan-detail/error-handling.md:8
- (.andromeda/security-plan-detail/error-handling.md:8) A pipe open that the hardened listener refuses (a remote-client rejection, a DACL denial, a squatted name) must still leave `viola hook` failing open with exit 0, no body and empty stderr, its detail the closed `channel-unreachable`; the chunk adds no hook-side refusal and moves no hook detail word.
  > `hook` never writes to stderr and never exits non-zero (Hook Contract). A security check failing in `hook` (server verification mismatch, oversized stdin, strict-modes failure) still ends in exit 0 with no body, so the dialog renders for the human. As landed, `viola hook <event>` runs the stdin cap, the `VIOLA_NAME` / `VIOLA_DIR` shape checks and the one-object parse, and fails open with `hook-decision` detail `oversize-stdin` · `malformed-json` · `channel-unreachable`; server verification and strict-modes join at Epoch 6 (Decisions Log `2026-09-28`).

### input-validation#1.3 · lost at classify · .andromeda/security-plan-detail/input-validation.md:17
- (.andromeda/security-plan-detail/input-validation.md:17) The chunk touches `viola-channel`'s `server.rs` / `server/win.rs` listener; every frame the hardened Windows listener accepts must still be read under the frame cap, closed on overflow with `-32600`, at serde_json's default depth, so the admission work leaves the framing floor unchanged.
  > **Size cap:** wrap the stream reader in `Read::take(MAX_FRAME)` before `read_line`; an over-limit frame gets `-32600` and the connection is closed. **Depth:** serde_json's default recursion limit of 128 (never `unbounded_depth`). **Version:** the `v` check (`-32602`).

### security-anti-patterns#1.1 · lost at classify · .andromeda/security-plan-detail/security-anti-patterns.md:8
- (.andromeda/security-plan-detail/security-anti-patterns.md:8) Server verification before any frame is the next entry, not this chunk: the dated exceptions stay as they stand, and nothing this chunk adds may borrow them.
  > NEVER write any channel frame (`send`, `answer`, `hook.dialog`, `wait`/`last`, `pause`/`release`, and `hook.event` notifications, which carry prompt and reply text) before verifying the server's pid (Windows `server_process_id()`) or euid + pid (Linux/FreeBSD `peer_creds()`) plus sysinfo start time against `snapshot.json`

## Held

- authentication-authorization#1.2
- authentication-authorization#1.3
- authentication-authorization#1.6
- bootstrap-phases#1.1
- dependency-security#2.1
- secret-management#1.1
- secret-management#2.1
- security-anti-patterns#5.1
- threat-model-summary#10.1
- threat-model-summary#11.1
- threat-model-summary#2.1

## Cascade only

- authentication-authorization#1.4
- authentication-authorization#1.8
- input-validation#1.12
- input-validation#1.7
- threat-model-summary#1.3
- threat-model-summary#1.5
- threat-model-summary#7.1
- threat-model-summary#8.1
