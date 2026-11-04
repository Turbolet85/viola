@carried n=1 label=design-system/surface fact=fct-vocxcyzi
why: the shared fake-agent send cases must read exit 12's refusal and its refusal object on Unix exactly as on Windows, the typed set unchanged.
quote: | 12 | `unable  unverified-cli` | refusal object |
@carried n=1 run=2 label=design-system fact=fct-tj4jq2zv
quote: 10 to 14 an `unable` refusal with its reason (a refusal object under `--json`)
@carried n=2 label=design-system/surface fact=fct-k7c3yar5
why: exit 13's not-delivered detail words must be the same closed words on ubuntu-latest and macos-latest as on windows-2025.
quote: | 13 | `unable  not-delivered  input-not-ready` / `no-prompt-submitted` / `turn-running` / `unknown-dialog` / `control-character` / `empty-text` | refusal object |
@carried n=3 label=design-system/surface fact=fct-f6euya2e
why: exit 14's unknown refusal must keep its form on every OS, a code word or nothing, never an OS-only word.
quote: | 14 | `unable  unknown  <detail>`, the detail a code word or nothing | refusal object |
@carried n=4 label=design-system/surface fact=fct-adve5pgh
why: exit 20's human line and wrapper-fault document must read the same on Unix as on Windows under the one-schema re-assertion.
quote: | 20 | `error: wrapper fault  <code>` | `{"v":1,"error":"wrapper-fault","detail":{…}}` |
@carried n=5 label=design-system/surface fact=fct-hgef4h2n
why: exit 21's six closed detail causes and its strict-modes unable:/hint: pairs must print the same fixed text and the same closed words on every OS, with no path and no pid.
quote: `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, `<cause>` one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused`; the `list`, `link`, `unlink` and `plugin install` refusals and the six frame verbs' strict-modes refusals read `strict-modes-failed`; stderr empty |
@carried n=5 run=2 label=design-system fact=fct-tj4jq2zv
quote: 21 `unable  instance-unreachable` then the cause's hint, or for a refused strict-modes check its fixed home-state or instance-state `unable:` / `hint:` pair instead (pattern 1 and the paragraphs below), its `--json` document `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, one per cause, the cause one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused`, with stderr empty.
@carried n=5 run=3 label=design-system fact=fct-ui5ydw4a
quote: prints the instance-state `unable:` / `hint:` pair naming that instance, as `viola list`'s does, or under `--json` the `instance-unreachable` document with `detail` `strict-modes-failed`, stderr empty; no frame and no line are written.
@carried n=6 label=design-system/surface fact=fct-7mkzemsx
why: link and unlink's exit-0 --json documents must carry the same fields on Unix as on Windows.
quote: Exit 0 under `--json`: `viola link`'s document is `{"v":1,"ok":{"since":<ts>,"already_linked":<bool>}}`, `<ts>` the time the wrapper recorded the link, and `viola unlink`'s is `{"v":1,"ok":{}}`.
@carried n=7 label=design-system/surface fact=fct-3vwzogmp
why: link and unlink on a refused pair must reach exit 20 and the wrapper-fault document on every OS.
quote: Exit 20 is also reached by `viola link` / `viola unlink` on a pair the wrapper refuses (`-32602`; a `driver` equal to `driven`, which the CLI never refuses itself): `error: wrapper fault  <code>`, or under `--json` the `wrapper-fault` document.
@carried n=8 label=design-system/surface fact=fct-3b32n36m
why: exit 20's closed detail for the five known codes must be the same closed set on every OS, never an OS-only message or data shape.
quote: Exit 20's `--json` `detail` is closed for the five known codes (`viola_channel::sanitised_fault`): `message` is each code's fixed text;
@carried n=9 label=design-system/surface fact=fct-utjcf3wu
why: a code outside the five must print the same fixed human line and document on Unix as on Windows.
quote: A code outside the five prints the same human line, `error: wrapper fault  <code>`, and under `--json` the document `{"v":1,"error":"wrapper-fault","detail":{"code":<code>,"message":"wrapper fault","data":null}}`; MCP carries the same fixed `message`.
@carried n=10 label=design-system/surface fact=fct-kqtzijw6
why: tests/cli_error_bodies.rs is the check that witnesses the one-document, literal-code, empty-stderr, no-path --json failures, and it must run and pass on ubuntu-latest and macos-latest.
quote: Each CLI `--json` failure `tests/cli_error_bodies.rs` drives prints one document, exits its literal code, leaves stderr empty, and holds no ESC, no path and no upstream text.
@carried n=10 run=2 label=design-system fact=fct-6xd7opxn
quote: For `--json`, the whole result, including refusals and `error` objects, is one JSON document on stdout with the typed exit code
@carried n=11 label=design-system/surface fact=fct-arozinj6
why: the catch site's internal-error document and its human line must be byte-identical on every OS.
quote: Under `--json` the `cli` catch site prints row 1's `internal-error` document on stdout in place of `error: internal error`, stderr then empty; without `--json` it prints that line, byte for byte.
@carried n=11 run=2 label=design-system fact=fct-tj4jq2zv
quote: in place of the `error: internal error` line the catch site's `{"v":1,"error":"internal-error","detail":null}` document on stdout, stderr empty)
@carried n=12 label=design-system/surface fact=fct-ue5y52fu
why: the openpty wrapper must keep run and revive's start silent once the child starts, and hook and mcp silent at the catch site, exactly as the ConPTY wrapper does.
quote: `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site, under `--json` too.
@carried n=12 run=2 label=design-system fact=fct-6xd7opxn
quote: At the catch site `run`, `revive`'s start, `mcp` and `hook` stay silent.
@carried n=12 run=3 label=design-system fact=fct-cj5433yq
quote: **`hook`** writes nothing to stderr and always exits 0. **`mcp`** writes only MCP frames on stdout.
@carried n=12 run=4 label=design-system fact=fct-w4pl2fni
quote: **`run`:** prints nothing once the child starts.
@carried n=12 run=5 label=design-system fact=fct-2t54hjhs
quote: `revive` prints nothing once the child starts, as `run`;
@carried n=13 label=design-system/surface fact=fct-pxdkcmfq
why: the Unix server check must refuse a peer whose euid is not this user's on Linux and macOS, and on macOS compare the euid alone, so no Unix test reads the macOS check as a pid or start-time comparison.
quote: a process on the endpoint that is not the recorded wrapper (its pid or start time differs, or the check cannot be made; on Linux and macOS also a peer whose euid is not this user's or cannot be read, on macOS the euid alone) gets no frame, and the verb prints `unable  <name>  instance-unreachable` with the server-verification hint last on stderr, or under `--json` the document `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}` with stderr empty.
@carried n=14 label=design-system/surface fact=fct-zp4y2mcv
why: the Unix fake-agent frame-verb cases must read instance-dead for a directory and unwrapped-name for an absent path, a regular file or a symlink, as on Windows.
quote: With no live endpoint a frame verb reads its instance directory after the strict-modes check: a directory is `instance-dead`; absent, a regular file or a symlink is `unwrapped-name`.
@carried n=15 label=design-system/surface fact=fct-aej2ym2d
why: a frame verb against a killed or lost openpty wrapper must read instance-dead or endpoint-lost on Unix as against a ConPTY one.
quote: A failed connect is `instance-dead`; a wrapper lost with the call open is `endpoint-lost`.
@carried n=16 label=design-system/surface fact=fct-ihyxsaec
why: the stale-wrapper fixture the v1-40 witness builds must make a frame verb read instance-dead, never a stale-heartbeat word outside exit 1.
quote: A frame verb against a stale instance reads `instance-dead`: stale heartbeat is exit 1's cause alone.
@binding n=1
**surface exit-code rows ↔ body Exit-code phraseology**: every typed code the Unix suite reads keeps its human stderr form and its --json document with the same closed words as on Windows, the typed set staying 0, 1, 2, 10 to 14, 20, 21 (design-system/surface fct-hgef4h2n) (design-system fct-tj4jq2zv).
@binding n=2
**surface server-verification cause ↔ the scope's macOS gap and peer-euid boundary**: the Unix server check refuses a foreign or unreadable peer euid on both Unix OSes and compares the euid alone on macOS, and no env, flag or feature seam forces that refusal (design-system/surface fct-pxdkcmfq).
@binding n=3
**surface catch-site silence ↔ body run passthrough and Platform-Specific Notes**: the openpty wrapper prints nothing once the child starts and leaves its terminal untouched, and hook keeps exit 0 with no body and no stderr on Unix (design-system/surface fct-ue5y52fu) (design-system fct-w4pl2fni) (design-system fct-cj5433yq).
@binding n=4
**surface stale-instance cause ↔ body cli tokens (the v1-40 witness)**: the stale wrapper under the outer PTY reads SGR dim across the whole `viola list` row with NAME bold, while a frame verb against the same instance reads `instance-dead` (design-system/surface fct-ihyxsaec) (design-system fct-dufqiv2l).
@binding n=5
**surface --json witness ↔ the one diagnostics schema**: tests/cli_error_bodies.rs runs on ubuntu-latest and macos-latest beside windows-2025 and holds one document, literal exit code, empty stderr and no ESC, path or upstream text on each (design-system/surface fct-kqtzijw6) (design-system fct-6xd7opxn).
