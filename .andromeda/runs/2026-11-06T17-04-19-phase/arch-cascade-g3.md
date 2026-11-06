@carried n=1 label=architecture/established-decisions fact=fct-anqliuvh
why: The live «overseer drives a builder through viola» session must record its implicit link only through a `send` or `answer` whose caller has `VIOLA_NAME` set, and v1-36's link-set clause must be disposed against this rule.
quote: **[Links] A link `driver → driven` is recorded when a `send` or `answer` is issued from inside a wrapped session to another instance, or by an explicit `viola link <driver> <driven>`. `viola unlink` removes it.** A call counts as coming from inside a wrapped session when the calling process has `VIOLA_NAME` set. The CLI and the MCP server pass that name as `from` in the request `params`, and the driven instance's wrapper appends a `link` event to its `events.ndjson` only when that `driver → driven` pair is not already in its link set.
@carried n=1 run=2 label=architecture fact=fct-3iyhqr2o
quote: Links are events in the log, the snapshot's link set derived by replaying them; the implicit link and the explicit verbs are built, the verbs checking the driven instance's files before any log and its wrapper serving `link` / `unlink`.
@carried n=2 label=architecture/established-decisions fact=fct-ikuc6e4y
why: v1-13's budget-release clause must be deferred with its reason or re-worded against `release --budget` as a CLI-only, per-instance lift, on the founder's word.
quote: The pause lifts when `resets_at` passes or on `viola release --budget`. `release --budget` leaves the wheel where it is, and a plain `release` returns the wheel to `driver` without lifting a budget pause. `release --budget` is sent to one instance's wrapper and lifts the pause for that instance only, until the crossed window's `resets_at` passes, so the next over-threshold reading does not re-pause it.
@carried n=2 run=2 label=architecture fact=fct-tkwxvt3j
quote: **[MCP] rmcp 3.4.1, stdio, tools `send · wait · last · answer · list`. `release` is CLI-only.** Refusals are `isError: true` tool results mapped from `result.refusal`; `pause`, `link` and `unlink` are CLI-only too, as human controls over the wheel and the link topology.
@carried n=3 label=architecture/established-decisions fact=fct-qwpklckh
why: The live dialog hold near the 3600 s ceiling must run on a CLI version whose stamp holds every relied row `pass`, because an unverified version withholds dialog answers, and any re-stamp it needs must fit the directive's two live sessions.
quote: A version is verified only when its stamp holds every relied row `pass` (`LedgerRow::relied()`, below); `cli_verified` (and `cli_version` when parsed) is recorded in the snapshot and the `claude-child` `process-start`. On an unverified version viola still types, runs the wheel and emits events, but withholds dialog answers, and drivers get `unverified-cli`.
@carried n=3 run=2 label=architecture/established-decisions fact=fct-qwpklckh
quote: so every home stamped before chunk 2026-10-18-compact-post-condition-ledger-row reads `cli_verified:false` until its next `viola verify`);
@carried n=3 run=3 label=architecture/established-decisions fact=fct-qwpklckh
quote: re-stamp of every stamped version (2.1.287 and 2.1.302 today, each `24 pass  0 fail` on the Linux dev host, 2.1.302 as measured at `viola-0.1.0/chunks/2026-10-19-newer-claude-cli-rows/evidence/live-round.md`; a verify is six `claude` starts).
@carried n=3 run=4 label=architecture fact=fct-7vnaqpcq
quote: on an unverified one viola still types, runs the wheel and emits events, but withholds dialog answers (`unverified-cli`); no `viola verify` run writes a byte into a CLI dialog.
@carried n=4 label=architecture/established-decisions fact=fct-vag5jbae
why: A fold or ratification of `StampsError` must carry this text's `StampsError::StrictModes` with it, so the verify home-pair mapping names the enum that stands.
quote: The one stamps writer, `update_stamps`, runs `strict::check_stamps` before it creates `ledger/` and again under its lock before the capped read; a refusal (`StampsError::StrictModes`, [Error Handling]) reads and writes nothing, and verify maps it to the same home pair.
@carried n=5 label=architecture/established-decisions fact=fct-44n3w6o5
why: The killed-wrapper leg of item 11 must be read against the pid + start-time `gone` rule, the fake-agent reading standing beside it until a live leg is taken.
quote: A bare pid is unsafe because pids get reused, which is why start time is matched too: a snapshot whose pid is dead, or alive with another start time, is `gone` whatever the heartbeat says (Session liveness).
@carried n=5 run=2 label=architecture fact=fct-aos27ydg
quote: **[Session Liveness] Heartbeat file primary; pid + process start time decide `gone`, and `claude agents --json` only enriches it.**
@carried n=6 label=architecture/established-decisions fact=fct-uzcv7hop
why: The mutation survivor's record (matrix#v1-44, test-plan §1) must name exactly these three client seams reading `peer_admitted`, and no seam may be added.
quote: then connects and reads `peer_admitted(client.server_euid())`, then compares the connected peer's pid and its OS start time with them before its first frame (`verify_server`; §Standard Contracts, Session liveness); the seams are `src/cmd/client.rs` `connect` with its 8 callers, `src/cmd/hook.rs` `connect` and `crates/viola-mcp/src/tools.rs` `frame`.
@carried n=7 label=architecture/established-decisions fact=fct-mefdhndu
why: The founder's word on the socket-directory pair's wording must rule over both sites of `socket-dir-refused`, the start's (exit 1) and the client's (exit 21), with its code and exit unchanged.
quote: Before any snapshot, child or `Server::bind`, `viola run` and `viola revive`'s start run `socket_dir_ready`: `check_endpoint_dir`, create the parent only when absent, check again, so a race's loser reads the same; a failed create and a refused check read one code, `socket-dir-refused`, exit 1, and a `Server::bind` failure does not read it.
@carried n=7 run=2 label=architecture/established-decisions fact=fct-2y2sv475
quote: then through `cmd::client::connect` the socket-directory check (`check_endpoint_dir`, a refusal exit 21, `socket-dir-refused`), the connect, the peer euid check and the server identity check (either refusal exit 21, `server-verify-failed`), and one request.
@carried n=8 label=architecture/established-decisions fact=fct-xqkfyxga
why: Each of the five open divergences owned by this entry must take a fold or a ratification here, and `PtyError`'s ratified exception must stay true of the moved `portable-pty`.
quote: A first open divergence, not an exception: `viola-mcp` holds `ServeError` (`Runtime`, `Initialize`, `Session`; fixed messages, returned by `serve`) beside `McpError` (`Internal`, Display `internal error`) (chunk 2026-10-17-mcp-server-for-drivers); the rule stands, and its fold or ratification is open. A second: `viola-state` holds `StampsError` (`StrictModes`, `State`; fixed messages, returned by `update_stamps`) beside `StateError`, the same. Three more the census below adds, the same: `PinError`, `StampsReadError` and `Refused`. The owner of all five: a CARRY on the working-route entry "Version done-check";
@carried n=8 run=2 label=architecture/established-decisions fact=fct-xqkfyxga
quote: (one ratified exception: `viola-pty`'s `PtyError` has a hand-written `Display`/`Error` with fixed messages, because the seam's normal dependencies are exactly portable-pty, tracing and libc; portable-pty's anyhow error is kept only as a boxed source.
@carried n=8 run=3 label=architecture fact=fct-sha7o35z
quote: One ratified exception, `viola-pty`'s hand-written `PtyError`, and two open divergences, `viola-mcp`'s `ServeError` and `viola-state`'s `StampsError`, with `PinError`, `StampsReadError` and `Refused` the three the census adds, the rule standing;
@carried n=9 label=architecture/established-decisions fact=fct-gctodeyz
why: The derive census must be re-measured at HEAD by its own command and re-worded to the measured count and list.
quote: The product sources hold 9 `thiserror::Error` derives, the census of [Error Handling]: `AgentError`, `ChannelError`, `StateError`, `PinError`, `StampsError`, `StampsReadError`, `Refused`, `McpError`, `ServeError` (by `command grep -rn "thiserror::Error" src crates --include=*.rs`).
@carried n=10 label=architecture/established-decisions fact=fct-yjcz4wzl
why: The `portable-pty` move must leave every product member's normal graph but `viola-mcp`'s tokio-free, as the sole-root loop and the tokio-free channel check the chunk re-runs read it.
quote: No other product member's normal graph reaches tokio (the sole-root loop and `scripts/channel-tokio-free.sh`).
@carried n=10 run=2 label=architecture fact=fct-l5jpezvw
quote: **[Concurrency / Backend Framework] Hybrid sync core + Tokio only in `mcp` (KEYSTONE).** CI enforces it with a sole-root `cargo deny` `tokio` ban and a `cargo check` of the sync crates on `ubuntu-latest`.
@carried n=10 run=3 label=architecture fact=fct-omwrdnlc
quote: **[PTY] portable-pty `=0.8.1` pinned behind viola's own `pty` seam (spawn · read · write · resize · wait · kill).** Child exit is detected on the process handle, never on EOF; `pty_backend()` is `openpty`; viola never holds, queues or re-orders a human key behind a resize and never answers DA1 itself. Rationale: 0.8.1 is spike-proven; moving `portable-pty` off 0.8.1 to a release whose graph drops `pin-utils` (RUSTSEC-2026-0441) is owed, owner: "Version done-check".
@carried n=11 label=architecture/established-decisions fact=fct-23yj6ah3
why: The live dialog hold near the 3600 s ceiling must stay inside the dialog tier's registered hook `timeout` of 3615 s.
quote: The dialog tier joins them (chunk 2026-10-04-dialog-answers-by-dialog-id): PreToolUse with `"matcher": "AskUserQuestion|ExitPlanMode"` and PermissionRequest, each with `"timeout": 3615` — nine exec-form entries in all.
@carried n=11 run=2 label=architecture fact=fct-4nt6qtyp
quote: A sync dialog hook waits at most the dialog hold (`dialog_hold_s`, 60–3600 s, default 60; ceiling PROVISIONAL, owner: "Version done-check"), the dialog then rendering for the human.
@carried n=12 label=architecture/established-decisions fact=fct-wtlwpexy
why: Measurement B must be brought to the founder as his item, and any action on it is his word, never this chunk's lean.
quote: One source tree can build two different files: six cold release builds of one tree at one target path gave two, 8 179 264 B and 8 179 272 B, three each in no fixed order, unequal in 3 of 34 sections (`.note.gnu.build-id`, `.symtab`, `.strtab`) with every loaded section equal and 6 `strings` lines apart (three `anon.<hash>.<n>.llvm.<number>` local symbols), so one tree can key two `bin/<key>/` and two `plugin/<key>/` folders (as measured on the Linux dev host at `viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/reproducible-builds.md`). Its cause is not explained; acting on it is owed to the working-route entry "Version done-check".
@carried n=12 run=2 label=architecture/established-decisions fact=fct-5ijnay4v
quote: `viola_state::pin::pin_key(bytes)` names the pinned copy's key, `<version>-<16 hex>`, in one place, and `pin_bytes(home, bytes)` pins bytes already read;
@carried n=12 run=3 label=architecture fact=fct-c3wssf3l
quote: the content hash keys the copy because the version alone collides across rebuilds; one tree can build two files whose loaded sections are equal and so key two copies, its cause not explained and acting on it owed to the working-route entry "Version done-check".
@carried n=13 label=architecture/established-decisions fact=fct-4jk25cex
why: The live driving session must settle whether its driver runs under `viola run` or unwrapped through the installed plugin, within the directive's two live sessions.
quote: **[Plugin Scope] The plugin loads only into wrapped sessions via `--plugin-dir`.** Both driver and driven run under `viola run`. `viola plugin install` (a local marketplace) is optional, for the MCP verbs in an unwrapped driver.
@carried n=14 label=architecture/established-decisions fact=fct-onw4pywk
why: Removing `plain_drive` (item 17) removes the Git Bash rewritten-path warning it served, so this decision's text must follow the removal or the remnant must be kept with its reason.
quote: viola warns when an argument carries a Git Bash rewritten-path prefix. Rationale: Git Bash rewrites `/skill` arguments into Windows paths (brief §6), and agents need machine-parseable refusals.
@carried n=14 run=2 label=architecture fact=fct-nvft5tqc
quote: viola warns when an argument carries a Git Bash rewritten-path prefix. Rationale: Git Bash rewrites `/skill` arguments into Windows paths, and agents need machine-parseable refusals.
@carried n=15 label=architecture/established-decisions fact=fct-hss5qy6y
why: Each Windows remnant item 17 names must be removed, or kept only with a stated reason, so that no Windows or macOS code stands in the tree.
quote: No Windows or macOS code, test, fixture or CI job stands in the tree; that code is recoverable at the tag `multi-os-final` (`ecbe7ab3`).
@carried n=15 run=2 label=architecture fact=fct-hcu7z4ke
quote: It is built and CI-tested on Linux (`ubuntu-latest`), and the on-disk state is parsed defensively. Rationale: the Windows and macOS support is cut, recoverable at the tag `multi-os-final`,
@carried n=15 run=3 label=architecture fact=fct-igbvtfv6
quote: the Windows and macOS code is out of the tree, recoverable at the tag `multi-os-final` (`ecbe7ab3`).
@binding n=1
**[Error Handling] fold or ratification ↔ Conventions error-types line, Inherited Defaults Errors line and the verify stamps writer**: each of `ServeError`, `StampsError`, `PinError`, `StampsReadError` and `Refused` must take one disposition, and the derive count must be re-measured, with every text that names them moving together: the decision (architecture/established-decisions fct-xqkfyxga), the census (architecture/established-decisions fct-gctodeyz), the body summary (architecture fct-sha7o35z), the Conventions line (architecture fct-nhsg4k7g), the Inherited Defaults line (architecture fct-vf75c7sp) and the `StampsError::StrictModes` mapping (architecture/established-decisions fct-vag5jbae).
@binding n=2
**[PTY] move off `=0.8.1` ↔ [Module Boundaries] / Tokio keystone ↔ `PtyError`'s exception**: the `portable-pty` release the chunk moves to must drop `pin-utils` and keep `viola-pty` a sync crate whose normal graph reaches no tokio, so the sole-root ban, the sync-crate check and `scripts/channel-tokio-free.sh` stay green; `PtyError`'s exception must stay true, with the seam's normal dependencies exactly portable-pty, tracing and libc and portable-pty's error a boxed source; and child exit on the handle, `openpty` and the resize and DA1 rules must hold under the new release (architecture fct-omwrdnlc, architecture/established-decisions fct-yjcz4wzl, architecture fct-l5jpezvw, architecture/established-decisions fct-xqkfyxga).
@binding n=3
**[Hook Contract] dialog-hold ceiling ↔ [Deployment] dialog-tier `timeout` ↔ [CLI Version Compatibility] verified stamp**: the live hold near 3600 s must run on a verified CLI, since an unverified one replies with no dialog answer, and must end inside the registered 3615 s hook timeout; a home stamped before chunk 2026-10-18-compact-post-condition-ledger-row reads unverified, and a re-stamp costs six `claude` starts, which the two-session directive must account for (architecture fct-4nt6qtyp, architecture/established-decisions fct-23yj6ah3, architecture/established-decisions fct-qwpklckh).
@binding n=4
**[Plugin Scope] driver placement ↔ [Links] implicit link ↔ the directive's two live sessions**: the implicit link is recorded only from a caller with `VIOLA_NAME` set, so a driver run under `viola run` records it, while an unwrapped driver using the installed plugin or the CLI records none; the plan must choose between them within two live sessions (`v1-53`'s run plus one spare) (architecture/established-decisions fct-4jk25cex, architecture/established-decisions fct-anqliuvh).
@binding n=5
**v1-13 / v1-36 dispositions ↔ [MCP] CLI-only verbs ↔ [Budget Governor] `release --budget` ↔ [Links]**: `link` and `release` (budget included) are CLI-only, so no MCP result exists for them, and the deferral or re-wording of each clause must read those surfaces as they stand, on the founder's word (architecture fct-tkwxvt3j, architecture/established-decisions fct-ikuc6e4y, architecture/established-decisions fct-anqliuvh).
@binding n=6
**Item 17 remnant removal ↔ [Scale / Product] Linux only ↔ [CLI Conventions] Git Bash warning**: removing `plain_drive` must carry the CLI Conventions' Git Bash rewritten-path sentence, in both the entity and the body line, to the removal (or keep it with its reason), so that no Windows code or text describing a removed remnant stands (architecture/established-decisions fct-hss5qy6y, architecture/established-decisions fct-onw4pywk, architecture fct-nvft5tqc).
@binding n=7
**Measurement B ↔ [Deployment] pin key**: the founder's disposition on one tree keying two copies rules over the one place the key is named, `pin_key`'s `<version>-<16 hex>`, and the Deployment rationale must follow whatever he rules (architecture/established-decisions fct-wtlwpexy, architecture/established-decisions fct-5ijnay4v, architecture fct-c3wssf3l).
@binding n=8
**Mutation survivor ↔ [Session Liveness] client seams**: the dated note at matrix#v1-44 and test-plan §1's untestable zone must name the same three seams reading `peer_admitted`, with no seam added (architecture/established-decisions fct-uzcv7hop).
@binding n=9
**Founder's word on the socket-directory pair ↔ both `socket-dir-refused` sites**: the wording ruled must apply to the start's refusal (exit 1) and the client verbs' refusal (exit 21), with code and exit unchanged (architecture/established-decisions fct-mefdhndu, architecture/established-decisions fct-2y2sv475).
@binding n=10
**Item 11 killed-wrapper leg ↔ [Session Liveness] `gone` rule**: a live leg taken, or the standing fake-agent reading `run_child_ends_when_its_wrapper_is_terminated`, must be read against the pid + start-time `gone` rule (architecture/established-decisions fct-44n3w6o5, architecture fct-aos27ydg).
