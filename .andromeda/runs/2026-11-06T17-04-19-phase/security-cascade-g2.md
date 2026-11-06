@carried n=1 label=security-plan/authentication-authorization fact=fct-4riic6pc
why: The chunk must add no channel method and must settle v1-13 and v1-36 by deferral or re-wording, so the served surface must stay exactly these nine methods and the one notification.
quote: Any caller that passes the OS check (IPC) gets the full surface: the nine served methods (`send`, `wait`, `last`, `hook.dialog`, `answer`, `pause`, `release`, `link`, `unlink`) and the `hook.event` notification, each behind the IPC rows below and its own params check (§ Input Validation, Channel frames).
@carried n=1 run=2 label=security-plan fact=fct-j6fyndcf
quote: RBAC / permissions — capability-based, all or nothing: any caller that passes the OS check gets the full surface; no roles, the only principal being the local OS user.
@carried n=2 label=security-plan/authentication-authorization fact=fct-2pxk6g7w
why: The founder's word on the socket-directory pair's wording, owed in this chunk, must land on this row's directory text, and the row must state whatever wording he gives.
quote: **Primary control:** the socket lives in a per-user directory with mode 0700: `$XDG_RUNTIME_DIR/viola/`, and `/tmp/viola-<uid>/` only as a fallback. The endpoint becomes `<dir>/viola-<h12>.sock`.
@carried n=2 run=2 label=security-plan fact=fct-dthgjfbs
quote: IPC access control (Unix) — the socket lives in a per-user 0700 directory, checked before bind and before any client connect (owner, mode, not a symlink), and is chmod 0600 right after the bind;
@carried n=3 label=security-plan/authentication-authorization fact=fct-pq4mk5dy
why: The board's read rule stands on a delegate's answer, and the chunk must record the founder's own word on it and then close or keep this OPEN line to match.
quote: OPEN — the board's read rule stands on a delegate's answer · owner: the Epoch 7 boundary batch
@carried n=3 run=2 label=security-plan/authentication-authorization fact=fct-65crhg3m
quote: the whole-board refusal and its no-line shape are the founder's own answer, 2026-10-13, relayed verbatim by the Viola overseer (the operator's answer at that chunk's planning, superseded 2026-10-13); it is none of the dated interim gaps; proven on Linux; the board holds the home to the read rule, so a home another user may read (a 0750 home) is refused as `ScanRefused::Home` with or without an instance)
@carried n=4 label=security-plan/authentication-authorization fact=fct-efyzkncd
why: The mutation audit's survivor, an always-admit peer_admitted at the three client seams, must stay untested by construction with no seam added to reach it, and its note must stand in matrix#v1-44 and test-plan section 1.
quote: A foreign euid and a foreign owner are not driven by any test: one account can make no foreign peer, so those decisions are held by injected values, and no seam may be added to drive them.
@carried n=4 run=2 label=security-plan/authentication-authorization fact=fct-zpaxdj2d
quote: `peer_euid_admitted(observed, own)` admits only when observed equals own (`observed == Some(own)`), so a peer whose euid cannot be read is refused.
@carried n=4 run=3 label=security-plan/authentication-authorization fact=fct-ju3c6nzj
quote: `peer_creds().euid()` must equal `geteuid()` (checked at `cmd::client::connect`, the `connect` of `cmd::hook` and the `frame` of `viola-mcp`, through `peer_admitted`; a peer whose euid cannot be read is refused, and a refused euid reads `server-verify-failed`)
@carried n=4 run=4 label=security-plan fact=fct-phspeqis
quote: past the connect admits only a peer of its own euid, an unreadable peer refused, a refusal reading `server-verify-failed`, each sending no frame.
@carried n=5 label=security-plan/authentication-authorization fact=fct-ezmg3kua
why: The OS-gated census that v1-53's claim reads and the CI green read on the one ubuntu-latest leg must count and run this whole-file gated suite, and a remnant pass must keep or justify its gate.
quote: `tests/security_negatives_socket_dir.rs`, `#![cfg(unix)]`, holds the start, frame-verb and hook negatives over a 0755 directory and a symlink to a 0700 one, on CI's `ubuntu-latest` leg.
@binding n=1
**RBAC / permissions ↔ Security Anti-Patterns (Universal) and the MCP vector**: the nine served methods hold only while no channel method or listener is added without a Decisions Log entry (security-plan fct-yfqtfw75), and the v1-13/v1-36 dispositions must leave `link` / `unlink` / `release` off the MCP tool list as an affordance, not an enforcement (security-plan fct-kfezk4o5), deferred or re-worded, never built (security-plan/authentication-authorization fct-4riic6pc).
@binding n=2
**IPC access control (Unix) ↔ Threat Model Summary networking line and the Authentication anti-pattern**: whatever wording the founder gives the socket-directory pair must read the same in this row (security-plan/authentication-authorization fct-2pxk6g7w), in the networking line (security-plan fct-2jp5ovui) and in the never-the-`/tmp`-root ban (security-plan fct-zshlqykf).
@binding n=3
**`~/.viola/` access control, the board ↔ the strict-modes read rule**: the founder's word on the board's read rule must close or keep the OPEN line (security-plan/authentication-authorization fct-pq4mk5dy) and agree with the board clause and read rule of the `~/.viola/` row (security-plan/authentication-authorization fct-65crhg3m), recorded in the verification matrix.
@binding n=4
**IPC peer checks, what no test drives ↔ the test-seam rule**: the survivor's untested zone (security-plan/authentication-authorization fct-efyzkncd) must not be reached by a new seam, and no test seam or environment read is added without its own Decisions Log entry (security-plan fct-7ackeyph); the note stands in matrix#v1-44 and test-plan section 1.
@binding n=5
**IPC access control tests ↔ CI/CD line**: the `#![cfg(unix)]` socket-directory suite (security-plan/authentication-authorization fct-ezmg3kua) runs on the one-leg `[ubuntu-latest]` matrix the CI/CD line states (security-plan fct-ju2d5wlo), which must agree with the code after the remnant removal.
