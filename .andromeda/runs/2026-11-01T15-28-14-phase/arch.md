# arch extract

## Relevance
relevant: the chunk works the Unix leg of the plan's local endpoint, its strict-modes entry points, the exit-cause surfaces and the recorded macOS gap.

## Constraints
- §Occupied Resources → IPC endpoints requires the Unix socket in a per-user 0700 directory, never the `/tmp` or `$TMPDIR` root, with the 0600 chmod after the bind and the sibling `.lock` as the start arbiter; whether the code already verifies owner, mode and not-a-symlink before the bind and the connect is research's question (per architecture §Occupied Resources → IPC endpoints; the trust rule it serves, same OS user and no other, per architecture §Cross-cutting Patterns → Local endpoint trust boundary).
- §Occupied Resources → IPC endpoints requires every other process to connect to the snapshot's recorded `endpoint`, with none rebuilding the path, so a client's directory check has the recorded path to read and not a re-resolved environment; `run`'s collision check stands outside that section's server-check list, and whether the directory and euid checks reach it is research's question (per architecture §Occupied Resources → IPC endpoints).
- §Conventions → Error handling schema fixes the exit set at `0, 1, 2, 10–14, 20, 21`, a refused home-tree check of `run`, `revive`'s start or `verify` at 1, and a refused strict-modes check of `list`, `plugin install`, `link`, `unlink` or a CLI frame verb at 21; each new refusal lands inside that set, and which exit each one reads is research's question (per architecture §Conventions → Error handling schema).
- §Standard Contracts → CLI `--json` output closes both refusal documents: exit 21's `detail` is `viola_core::UnreachableCause`'s five words and exit 1's start refusal is a closed code list, so a new socket-directory or euid word extends those closed sets and their one writers, `client::unreachable_document` and `client::refused_document` (per architecture §Standard Contracts → CLI `--json` output).
- §Infrastructure Patterns → Crate dependency direction gives `viola-channel` the Unix `libc` edge for the socket directory and no edge to `viola-state`, which holds the `strict` module, so the check's placement either fits those edges or names a new one; the code sits on sync paths, Tokio only in `viola-mcp` (per architecture §Infrastructure Patterns → Crate dependency direction; architecture §Cross-cutting Patterns → Tokio containment).
- §Established Decisions → [Message Broker / IPC] records the macOS server check as `server_verdict` returning `Matches` with no comparison, and owes this entry the macOS client euid check, the socket-directory check and the 0755 exit-cause case; whether macOS reports a peer euid through the seam is research's question (per architecture §Established Decisions → [Message Broker / IPC]).
- §Occupied Resources → Filesystem names `viola list`'s two home checks as disagreeing (`board::scan`'s `check_path` the write rule, `check_instance` the read rule) and owes their reconciliation to this entry; the row does not say which rule stands, and that section's `instances/<ViolaName>/` row holds the home to the read rule (per architecture §Occupied Resources → Filesystem).

## Patterns to follow
- The server check's seam: `Client::server_pid` and `TokioClient::server_pid` read `peer_creds()` and write and log nothing, the verdict sits in `viola_state::liveness`, and the sync and Tokio clients move together, so a client euid read has both flavours to cover (per architecture §Established Decisions → [Message Broker / IPC]).
- The check order on a frame path: the instance strict-modes check (`client::refuse_untrusted`, the hook's `trusted`, `strict::check_instance` in the MCP frame tools) before any connect, then the server comparison before the first frame, a refusal writing no frame and no line (per architecture §Standard Contracts → Channel methods).
- A refused check leaves the mode or ACL as found, with nothing opened and no line written, the shape a refused socket directory takes (per architecture §Occupied Resources → Filesystem).
- Exit 21's human side: the cause keyed in `src/cmd/client.rs` `hint_cause`, the hint text in `src/human/hints.rs`, last on stderr and plain ASCII (per architecture §Conventions → Error handling schema).

## Anti-patterns to avoid
- Linux abstract-namespace socket names, or a socket directly in the `/tmp` or `$TMPDIR` root (per architecture §Occupied Resources → IPC endpoints).
- A `hook` that refuses visibly: exit 2, a body or stderr on one of this chunk's refusals; it fails open, and a lost endpoint loses the event, not the turn (per architecture §Established Decisions → [Hook Contract]; architecture §Established Decisions → [Hook Transport]).
- A path, pid or anyhow chain in a refusal body: anyhow's chains stay the owner's post-mortem, and a body carries code words and fixed messages only (per architecture §Established Decisions → [Error Handling]; architecture §Cross-cutting Patterns → Sanitised error surfaces).

## Contract bindings
- arch ↔ security: architecture §Cross-cutting Patterns → Local endpoint trust boundary sets the boundary and leaves its per-OS enforcement and any per-connection proof to the security plan.
- arch ↔ design-system: architecture §Conventions → Error handling schema leaves exit 21's hint wording to design-system's surface cli-send.
- arch ↔ tests: each new code word is a row of `tests/contract_error_bodies.rs` (architecture §Cross-cutting Patterns → Sanitised error surfaces) beside the scope's `tests/cross_exit_causes.rs` row; the 0755 case is test-plan §6's (architecture §Established Decisions → [Message Broker / IPC]).
- arch ↔ obs: a refused check writes no line ahead of any log (architecture §Occupied Resources → Filesystem); the log format stays obs's (architecture §Cross-cutting Patterns → Diagnostic output channels).

## Acceptance criteria contributions
- A socket directory failing owner, mode or not-a-symlink is refused before the bind and before the connect, its mode left as found and the socket still 0600 after a passed bind (per architecture §Occupied Resources → IPC endpoints).
- No exit code is added, and every new refusal is a closed code word passing `is_code_word`, a row of `tests/contract_error_bodies.rs`, with no path, pid or upstream text in its body (per architecture §Cross-cutting Patterns → Sanitised error surfaces).
- The sync crates stay tokio-free and `scripts/channel-tokio-free.sh` stays green, any new crate edge named in the plan's table (per architecture §Infrastructure Patterns → Crate dependency direction).
- `viola list` on a 0750 home reads the same verdict with no instance and with one (per architecture §Occupied Resources → Filesystem).
