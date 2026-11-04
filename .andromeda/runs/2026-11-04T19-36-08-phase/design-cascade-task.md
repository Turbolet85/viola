@task_prose part=chunk
Brings Linux and macOS to parity with Windows: the `viola run` wrapper runs its child under portable-pty's openpty backend behind the same pty seam as ConPTY (spawn, read, write, resize, wait, kill behave as on Windows, as the fake agent reads them).
Makes the full fake-agent suite one suite on all three OSes: every end-to-end and integration test of a behaviour both Unix OSes share runs on ubuntu-latest and macos-latest; a test left cfg(windows), cfg(unix) or cfg(target_os) names the reason its behaviour is one OS's where it stands.
Re-reads the obs gates G1 to G4 and the CI test, perf and lint jobs green on ubuntu-latest and macos-latest beside windows-2025, and re-asserts one diagnostics schema: process-log lines, events.ndjson, snapshots and every --json document carry the same fields and closed words on every OS.
Takes up the carried integration-tier witness of v1-40: under the outer PTY a `viola list` beside a stale wrapper reads SGR dim across the whole row, NAME dim and bold.
Windows stays the live-supported target: no Windows behaviour, pin or test moves; no live claude session, everything on the fake agent; no new exit code.
@task_file n=1 path=tests/run_cli.rs
@task_file n=2 path=tests/support/home.rs
@task_file n=3 path=.github/workflows/ci.yml mark=re-reads
what: the test, perf and lint jobs over matrix.os reading green on ubuntu-latest and macos-latest beside windows-2025
@task_item n=1 part=needs
The cli surface's platform list: macOS and Linux terminals are supported beside Windows Terminal and conhost, so every human output pattern of the cli surface must print the same lines on Unix as on Windows.
@task_item n=2 part=needs
The cli colour decision order (first match wins: --json, viola run, hook and mcp, NO_COLOR, TERM=dumb, stdout not a terminal, Windows VT enable failed): on Unix the Windows VT step never applies, and the depth choice (COLORTERM truecolor or 24bit, TERM containing 256color, else the 16-colour fallback) is what a Unix test under openpty reads.
@task_item n=3 part=needs
The cli tokens for a stale row: SGR dim on the whole stale row, bold NAME, the DIALOG word amber with its kind word and the cock winning over dim, a reset after every styled span, no background ever set; the v1-40 witness reads these bytes from the row's first byte to its last under the outer PTY.
@task_item n=4 part=needs
The viola list component pattern: the BAY line, six fixed columns, rows ordered by name, the UNWRAPPED read-only separator, stale liveness after a heartbeat older than 5 s, control characters escaped as \xHH; the Unix suite's list cases read the same board as Windows'.
@task_item n=5 part=needs
viola run and a passed viola revive print nothing once the child starts and emit no colour, glyph or cursor control: the openpty wrapper's passthrough must keep the child's terminal untouched exactly as the ConPTY wrapper does.
@task_item n=6 part=needs
The cli width rule: COLUMNS is never read, only stdout's own terminal width counts and a piped stdout has none, so only an unwrapped NAME is cut and only where that width is known; a Unix test under openpty sets the PTY size, never COLUMNS, and lines are never wrapped.
@task_item n=7 part=needs
The viola send readback mirror ([RB], [  ], [/ ], unconfirmable) and its stdout and stderr split, plus the wait and last result lines and their message-mode escaper: the shared fake-agent send, wait and last cases read these same lines and the same exit codes on Unix.
@task_item n=8 part=needs
Exit-code phraseology: the typed set stays 0, 1, 2, 10 to 14, 20, 21 with each code's human stderr form and --json document; hook still exits 0 with no body and no stderr, mcp writes MCP frames only.
@task_item n=9 part=needs
The closed words every --json document carries, the same on every OS: exit 21's six detail causes (strict-modes-failed, instance-dead, unwrapped-name, endpoint-lost, server-verify-failed, socket-dir-refused) and revive's start refusal codes; a case one OS cannot fill reads the documented null or closed word, never a missing key or an OS-only word.
@task_item n=10 part=needs
The strict-modes refusals' fixed home-state and instance-state unable: and hint: pairs (list, plugin install, link, unlink, send, wait, last, answer, pause, release) and the run and revive start refusals' fixed-message lines: on Unix they print the same fixed text, with no path and no pid.
@task_item n=11 part=needs
Diagnostics home: stack traces never print and full detail goes only to instances/<name>/diagnostics/; the per-OS diagnostics schema the chunk re-asserts is that home's content.
@task_item n=12 part=needs
Platform-Specific Notes: the Git Bash rewritten-path warning and conhost-without-VT fallback are Windows-only behaviours, the named reason a test of them may stay cfg(windows).
@task_item n=13 part=needs
Human output is ASCII only (->, [RB], [/ ], [  ], two spaces between columns, no link column in viola list) on every OS, in piped, NO_COLOR and TERM=dumb output too.
@task_standing n=1
convention: each new error, refusal or hint line a test, seam or wrapper path adds is a fixed message that prints no upstream text, path, pid, anyhow chain or stack trace; full detail goes to diagnostics only
home: Anti-Patterns › Per-Surface Bans (cli)
@task_standing n=2
convention: each new test that reads human CLI output keeps results on stdout and waiting, refusal, hint and error lines on stderr, and expects no colour, glyph or cursor control under --json, non-TTY, NO_COLOR, TERM=dumb or viola run
home: Anti-Patterns › Per-Surface Bans (cli)
@task_item n=1 part=not_needed
Typography's per-OS font fallback stacks and the Linux fallback assertions (DejaVu faces, no horizontal scroll, ch-sized columns): they are 0.2.0 frontend inputs, and the cli uses the user's terminal font, so Linux and macOS parity of the CLI touches none of them.
@task_item n=2 part=not_needed
The web-spa surface and every part marked a 0.2.0 frontend design input: the frontend stays deferred and 0.1.0 renders no page.
@task_item n=3 part=not_needed
The Color Palette's hex values, contrast table, surface scale, text hierarchy, semantic and domain status colour rows and border progression: page colours, not the cli's SGR tokens.
@task_item n=4 part=not_needed
Spacing tokens, Depth Strategy and Border Radius: page geometry for a frontend that does not exist in 0.1.0.
@task_item n=5 part=not_needed
Motion and the expression level's duration scale: the chunk adds no animation, and the cli has no motion at any level.
@task_item n=6 part=not_needed
Iconography's web glyphs (the arrow pair, the CSS linear-gradient strike, the icon library and size grid): web-only, the cli's ASCII words are the only glyph rule this chunk follows.
@task_item n=7 part=not_needed
Brand Identity's personality, design direction, library shortlist and the readback box's drawn page states: brand narrative and frontend drawing, no rule a pty backend or a cross-OS test acts on.
@task_item n=8 part=not_needed
The Self-Validation Protocol's swap, squint, token and sameness tests: checks of a new visual design, and this chunk designs no new interface.
@task_item n=9 part=not_needed
Rejected Defaults and the Universal Bans on decorative colour and convergent choices: the chunk adds no styled element and no new visual state.
