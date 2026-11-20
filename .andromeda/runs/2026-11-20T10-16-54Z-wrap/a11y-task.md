@task_prose part=session
A modal-dialog label quoted inside a transcript row no longer refuses a send on an idle session; a standing modal still refuses with input-not-ready.
The input-box readiness read now refuses on a standing modal only: a row holding a compiled modal literal that is option-shaped (optional selector glyph, optional numbering, the literal, only spaces after) or has no input-box row below it.
That one predicate moved for viola run's pre-send gate, verify's paste guard, the ledger post-condition and the rig's box read; the raw-presence reads (start read, verify's settle end) are unchanged by design.
The read stays a boolean over compiled characters and compiled literals; no row text is kept, returned or printed, and no refusal word, detail, hint, exit code, log line or config key was added.
Proved over the fake agent, recorded trust screens and synthetic screens only; no live claude start, nothing measured live; no count, bound or timeout moved (the gate's quiet period and max wait unchanged).
@task_file n=1 path=crates/viola-agent-claude/src/screen.rs
what: the compiled screen signatures — the input-box readiness predicate over the child's screen and the modal-dialog literals it matches, the only screen read beyond signatures, with its unit tests
@task_file n=2 path=src/run/gate.rs
what: viola run's pre-send readiness gate that refuses a send with input-not-ready while a dialog stands, and its unit tests
@task_file n=3 path=tests/support/fake.rs
what: the integration tests' fake-agent support screens — a synthetic idle screen quoting a modal label above the input box
@task_file n=4 path=tests/cli_send.rs
what: the viola send integration suite — the confirmed send and its readback on a verified session
@task_item n=1 part=touches
decision/no-screen-content-parsing: the signature-only screen read now also reads a row's shape (option-shaped, or no input-box row below) to tell a standing modal from a quoted one; still compiled literals and characters only, no content kept — the row is reworded at this wrap.
@task_item n=2 part=touches
contract/send-refusal/input-not-ready: the refusal now fires on a standing modal only, not on any screen row holding a modal label; its word, detail, hint and exit code are unchanged.
@task_item n=3 part=touches
decision/critical-path-send-readback: a confirmed send now goes through on an idle session whose transcript quotes a dialog label; a new integration case covers it.
@task_item n=4 part=touches
decision/critical-path-dialog-answer: a pending dialog drawn as options, or with no input box below it, still blocks the send until answered; that is the standing-modal half of the new rule.
@task_item n=5 part=touches
decision/boundary-check-never-inspects-claude-content and decision/single-vendor-content-boundary: the readiness read looks at vendor screen rows for shape around a compiled literal; it returns a boolean only and nothing of the vendor content crosses the boundary — check the wording still holds.
@task_item n=6 part=touches
contract/tui-passthrough/vendor-content: vendor content is still passed through untouched; only the pre-send gate's reading of it changed.
@task_item n=7 part=touches
decision/tui-boundary-assertions-only and decision/tui-boundary-driver-pin: the rig's box read uses the moved predicate; boundary tests now include a quoted-label-above-the-box screen that reads ready.
@task_item n=8 part=touches
decision/wrapped-tui-reads-as-unwrapped: a wrapped session with a quoted dialog label in its transcript now accepts a driver send as an unwrapped one would accept typed input.
@task_item n=9 part=touches
decision/time-bounds-never-on-human, decision/time-bounds-fall-on-the-driver and decision/dialog-hold-bounds-the-driver: the gate's quiet period and max wait are unchanged; fewer false refusals fall on the driver; no bound moved.
@task_item n=1 part=untouched
Colour and SGR rows (amber DIALOG, bold NAME, dim stale rows, no SGR off a terminal, word beside every SGR span): no output styling changed.
@task_item n=2 part=untouched
viola list board, caption, NAME truncation and unwrapped-separator contracts: no board output changed.
@task_item n=3 part=untouched
viola revive, link, unlink, plugin install, hook-call, MCP frame-tool and frame-verb refusal contracts, and the exit-code table: no refusal word, detail, hint, exit code or --json document added or changed.
@task_item n=4 part=untouched
TUI passthrough of human keystrokes, focus, mouse and resize sequences, viola terminal bytes, run diagnostics and the vt100 feed panic: none changed; human keystrokes are still never blocked.
@task_item n=5 part=untouched
Screen-reader targets, manual keyboard and Orca passes, CI integration, the a11y gate, SLO and flakiness rows: no a11y suite, workflow, artifact or budget changed.
@task_item n=6 part=untouched
Absences (internationalization, touch surface, web or graphical frontend, rendered page, user time limit): no capability added or removed.
@task_item n=7 part=untouched
Control-character escaping, fixed CLI strings pinned by literal, log identity tags and config.json keys: no printed string, log line or config key changed.
