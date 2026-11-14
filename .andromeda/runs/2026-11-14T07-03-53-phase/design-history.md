@history_item n=1 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the cli Platform narrowed to Linux terminals) — the Platform line became «Linux terminals (the core on any current distribution, CI on `ubuntu-latest`)» on the founder's rulings of 2026-11-04. This is where decision/linux-terminal-platform comes from. The seam has to be Linux-only, and its tests have to go green on `ubuntu-latest`, where no compositor runs, so the fake compositor is needed.

@history_item n=2 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the socket-directory pair at the start of run and revive) — added `unable: the socket directory for <name> cannot be trusted` / `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions` before any snapshot, child or bind. This is the row contract/viola-run-start-refusals/socket-dir. If the seam uses Hyprland's request socket, it needs the same 0700 / owner trust or a recorded reason it does not, and the scope says so. Any new refusal follows this precedent of a fixed pair that names no path.

@history_item n=3 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a refused socket directory at the eight frame verbs) — a refused socket directory at the eight frame verbs exits 21 before any frame: `unable  <name>  instance-unreachable`, then the hint of cause `socket-dir`, with `--json` `detail` `socket-dir-refused`. This is the row contract/frame-verb-refusals/socket-dir. It is the model for a closed outcome on an untrusted socket, and the seam must not reuse or disturb its cause list.

@history_item n=4 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (`socket-dir-refused`, exit 21's sixth cause and a log code) — added `socket-dir-refused` as the sixth closed cause of exit 21 and as an obs-plan D-20 log detail code. This is the precedent for the seam's closed codes-only outcomes (ok / unsupported / unreachable / malformed). A new code widens a closed list and needed the founder's word.

@history_item n=5 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (`revive --json`'s eleventh code; `server-verify-failed` and a refused peer euid; the wrapper's silent drop) — `revive` gained `socket-dir-refused`, and one code covers both a failed create and a refused check. The client euid check and the silent drop of a refused peer were built. These are precedents for treating a peer on a socket as untrusted, which is what the seam must do with the compositor's socket and its replies.

@history_item n=6 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the socket-directory pair's wording confirmed) — the OPEN mark on the socket-directory pair's wording was retired: the founder confirmed it as his own. The rows' socket-dir wording is settled, and any Hyprland-socket refusal must not reword it.

@history_item n=7 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the pending arch block spent, the log's codes and the two carve-outs) — set the D-20 log detail codes. It also recorded that the socket-directory cause first reached the catch site as an anyhow error with no code. This is a trap for the seam: a failure that falls through as an anyhow error leaks into the catch site instead of a closed outcome.

@history_item n=8 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the catch site's `internal-error` document) — the catch site prints `error: internal error` or `{"v":1,"error":"internal-error","detail":null}` and logs the chain only to the diagnostics. This bears on decision/no-stack-traces: no seam error may surface as a chain or a trace.

@history_item n=9 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the code word and the screen on a peer's reply) — both clients screen a peer's `refusal` and `detail` through `viola_core::is_code_word`, and anything that is not a code word reads as `unknown` or as no detail. This is the direct precedent for reading Hyprland replies as untrusted upstream output and keeping only closed viola values. It bears on decision/no-internals-in-errors and decision/hint-never-quotes-text.

@history_item n=10 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (exit 20's closed `detail` and the fixed `wrapper fault` message; `unknown`'s detail) — `viola_channel::sanitised_fault` replaced the peer's `message` and `data` with fixed text and closed data. `unknown`'s detail is now a code word or nothing. This pattern keeps upstream text out of errors, and the seam's replies must follow it.

@history_item n=11 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the `--json` failures the witness drives) — each `--json` failure holds no ESC, no path and no upstream text (`tests/cli_error_bodies.rs`). This is the witness shape for decision/no-internals-in-errors. A window title, class, address or socket path from the seam must never reach such a document.

@history_item n=12 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the server-verification hint as built) — the hint is plain ASCII and holds no pid, start time, path or endpoint, and tests assert this. It is a precedent for the seam's codes-only logging and refusals: no reply text, address, pid, path or socket name.

@history_item n=13 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (the unwrapped rack built) — `viola list` reads `claude agents --json`, and a failed, oversize or malformed read prints the board as if there were no `claude`. This is the nearest precedent for a capped, tolerant read of an external tool's JSON that falls back to a closed outcome and never guesses, like the seam's `unsupported` / `malformed` answers. The upstream-derived cells are named as such.

@history_item n=14 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** (session-end line and message mode) — message mode prints every control character but `\n` and `\t` as `\xHH`. It is the existing escaper for upstream text that does reach output. Under the scope, window titles and classes are content that reaches no log line, error or refusal, so the seam needs no such display path.

@history_item n=15 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** (the named unclean-fixture refusal) — the fixed `a recorded payload still holds a path or a username` became `a recorded fixture is not clean: <file> <code>`, with closed codes and never the content. This is the row contract/viola-verify-refusals/unclean-fixture. The Hyprland replies recorded live for the fake must be scrubbed like every fixture, so this refusal shapes how they are cleaned and checked.

@history_item n=16 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (`viola verify`'s step counter and its exit-1 phraseology) — the origin of verify's refusals, among them the recorded-payload path/username refusal that n=15 later renamed. This history lies behind the fixture-cleanliness rule the Hyprland recordings must meet.

@history_item n=17 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (verify's relied count on every build) — `LedgerRow::relied()` reads 24 on every build, and the summary is `stamped 2.1.287  24 pass  0 fail`. The scope keeps the Claude capability ledger Claude-only and puts Hyprland's "measured live" record alongside it. Folding Hyprland rows into the ledger would change verify's counter, which the design pins.

@history_item n=18 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (version census 0.2.0 — brand, surface and bans scoped to the CLI) — no web or graphical surface remains. The design direction is the one the CLI carries and the terminal fleet console inherits, and spinner and redraw bans are scoped to a verb's output. This explains the brand-identity N/A and overview rows. The seam serves the later window and console entries but builds no surface of its own.

@history_item n=19 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** (the masters as rows) — the master's text became rows, each verified against the code. Every picked row (decision/fixed-phraseology-vocabulary, the socket-dir contracts, decision/no-internals-in-errors and the others) comes from this conversion. A plan that changes behaviour must amend them by handle.

@history_item n=20 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** (Surface: cli: 2 rows) — two rows were amended. contract/refusal-hints/refusal-line: hints are keyed by reason, and an opaque or absent detail prints no hint. contract/exit-21-json-cause/unwrapped-name: the strict-modes check now refuses a symlink in place of an instance directory. These are the most recent precedents for amending rows by handle and for refusing a symlink when trusting a filesystem path. A Hyprland socket path under `$XDG_RUNTIME_DIR` would need the same symlink refusal.
