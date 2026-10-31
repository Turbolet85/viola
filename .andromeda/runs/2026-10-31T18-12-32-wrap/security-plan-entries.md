## 2026-10-31-exit-cause-code-catalogue — the socket-directory failure at bind and its owed code
**Section:** §Authentication & Authorization (authentication-authorization.md: IPC access control (Unix); IPC client-side server verification, the macOS cases)
**Change:**
- IPC access control (Unix) gains the failure at bind: a failure at `bind_endpoint`'s `create_private_dir` has no refusal code, it is an anyhow error at the catch site, and its code is owed to the route entry "Unix endpoint and home hardening".
- The macOS cases' CARRY to that entry was the socket directory loosened to 0755 giving exit 21 and the client euid check; now those two and the socket-directory refusal's code.
**Why:** The requirement listed a socket-directory refusal the code does not hold, as the chunk measured. The code's leaving this chunk is the delegate's word, the overseer, 2026-10-31, PROVISIONAL (`inputs#I3`); its staying with that entry is also a named carve-out of the founder's own word, live, 2026-10-31 before 16:52Z, through the overseer's dialog, given after the widening was shown to him (`inputs#I2` Q4). These two rows carry no PROVISIONAL mark; this entry carries the word.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — the no-endpoint reading, dead against unwrapped
**Section:** §Authentication & Authorization (authentication-authorization.md: IPC client-side server verification (anti-impersonation), its MCP clause; IPC client-side server verification, the no-endpoint reading, new, and `viola link` / `viola unlink`) · §Error Handling (error-handling.md: an MCP frame tool's unreachable result) · §Threat Model Summary (threat-model-summary.md: boundary-ipc, its new item)
**Change:**
- New row: with no `LiveEndpoint`, `viola_state::liveness::without_endpoint` reads `symlink_metadata` on the instance directory alone: a directory is `instance-dead`, anything else (absent, a regular file, a symlink) `unwrapped-name`. It opens nothing and writes no line, and the strict-modes check stays first at every caller. Its callers are `Unreachable::without_endpoint` in `src/cmd/client.rs` and `frame` in `crates/viola-mcp/src/tools.rs`. The row carries the OPEN mark: this reading PROVISIONAL, its owner line naming how a name with no endpoint tells a dead instance from an unwrapped name.
- An MCP frame tool with no live endpoint was `{"error":"instance-unreachable","detail":null}`; now `detail` is `<cause>`, `without_endpoint`'s reading (authentication-authorization.md, error-handling.md).
- `viola link` / `viola unlink` with no `live_endpoint` was exit 21 with the not-running hint; now exit 21 through `Unreachable::without_endpoint`, `instance-dead` with the not-running hint or `unwrapped-name` with the unwrapped-name hint.
- The threat model's new item states the reading and points to the row for its status.
**Why:** Every no-endpoint arm now reads the instance directory, so a name never started and a stopped instance read apart. The cause in `detail` is the founder's own word, live, 2026-10-31 before 16:52Z, through the overseer's dialog, given after the widening was shown to him (`inputs#I2` Q1). The reading itself is the delegate's word, the overseer, PROVISIONAL (`inputs#I2` Q2). That these rows lie inside his answers, and applying check 7's contradiction on the new row, are the overseer's as delegate, PROVISIONAL (`inputs#I6`).
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — exit 21 names its cause in detail
**Section:** §Authentication & Authorization (authentication-authorization.md: IPC client-side server verification (anti-impersonation); `~/.viola/` access control; the CLI refusal; the MCP refusal, new) · §Error Handling (security-plan.md: MCP `isError` results; error-handling.md) · §Input Validation (input-validation.md: CLI arguments / stdin) · §Threat Model Summary (threat-model-summary.md: boundary-ipc; boundary-mcp)
**Change:**
- Exit 21's `detail` was null on every cause; now a closed set of five, `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`. CLI `--json` reads `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, MCP's `isError` result the same less `v`.
- `viola_core::UnreachableCause` has no serde path; the two clients write every word: `client::unreachable_document` the CLI's one exit-21 writer (`refuse_untrusted`, `src/cmd/plugin.rs`), `src/cmd/list.rs`'s `refused_document` the board's `strict-modes-failed`, `unreachable` in `crates/viola-mcp/src/result.rs` MCP's.
- A strict-modes refusal was `detail: null`; now `strict-modes-failed`, on the frame verbs, `link` / `unlink`, the board and the MCP frame tools. The CLI refusal's "same document" for it is gone.
- A failed server check was `detail: null`; now `server-verify-failed`, on the CLI refusal and MCP. error-handling.md's "adds no error word" is gone: its `error` is `instance-unreachable` as for every cause.
- New row, the MCP refusal: `frame` maps a refused `check_instance`, no `live_endpoint` and a failed `server_verified` to their causes.
- The threat model's boundary-ipc gains the five, each with what it means (`endpoint-lost` the wrapper lost with the call open, `server-verify-failed` off macOS), holding no path, pid or upstream text.
**Why:** Every cause used to print one document; each now prints its own, and no exit moved. The cause in `detail`, CLI and MCP, is the founder's own word, live, 2026-10-31 before 16:52Z, through the overseer's dialog, given after the widening was shown to him (`inputs#I2` Q1); MCP's no-line record is a named carve-out of the same word (`inputs#I2` Q4). That these rows lie inside his answers, and applying check 7's contradictions, are the overseer's as delegate, PROVISIONAL (`inputs#I6`).
**Kept:** `error` stays `instance-unreachable` on every cause, MCP's text stays `refused: instance-unreachable`, and each refusal writes no frame and no line.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — a stale holder's code at the collision check
**Section:** §Authentication & Authorization (authentication-authorization.md: the Collision check clause) · §Error Handling (error-handling.md: new row)
**Change:**
- New row: `collision_check` prints and logs `stale-heartbeat` for a stale holder and `already-live` for a live holder and a fresh replayed beat. A frame verb against a stale instance reads `instance-dead` (`live_endpoint` gives none for a stale wrapper), stale heartbeat being exit 1's cause alone.
- The Collision check clause's source cite was `src/cmd/run.rs:418`; now `src/cmd/run.rs:438`, the chunk having added lines ahead of it.
**Why:** The chunk retired `already-live` as a stale holder's refusal, so exit 1 tells the two holders apart.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — verify's refusal codes
**Section:** §Data Protection (data-protection.md: the values · data-rest block, new) · §Error Handling (error-handling.md: `viola verify`'s strict-modes refusal)
**Change:**
- New record: `verify` writes `batch-script-child`, `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed` or `unclean-fixture` (`Ended::Refused`) on the `process-exit` it writes under a parsed `VIOLA_NAME`. A refused home tree is lineless, a failing row is no refusal and no code, the fixture's `<file>` enters no code, and `verify` takes no `--json`.
- `update_stamps`'s `StampsError::StrictModes` was grouped with the refused home tree under no line; now it exits 1 with the home pair, no step line and no summary, and its `process-exit`, under a parsed `VIOLA_NAME`, carries `strict-modes-failed`. The refused home tree keeps no line.
**Why:** `verify` gained a code per refusal. The stamps arm is a reading of the chunk's diff, where it returns `Ended::Refused("strict-modes-failed")`, the report not naming `update_stamps` there. That `verify`'s causes reach the log and the hint, having no `--json`, is a named carve-out of the founder's own word, live, 2026-10-31 before 16:52Z, through the overseer's dialog, given after the widening was shown to him (`inputs#I2` Q4).
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — the two cause hints
**Section:** §Error Handling (error-handling.md: new row)
**Change:**
- New row: the `unwrapped-name` and `endpoint-lost` hints come last on stderr in plain ASCII, their `<name>` the parsed `ViolaName`.
**Why:** Each of the two causes gained its own human hint. The `endpoint-lost` hint is the delegate's word, the overseer, PROVISIONAL (`inputs#I2` Q3); this row carries no PROVISIONAL mark, and this entry carries the word.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — the catch site's document under --json
**Section:** §Error Handling (error-handling.md: new row) · §Threat Model Summary (threat-model-summary.md: boundary-cli, its new item)
**Change:**
- New row: where the `cli` catch site prints its internal-error line (`prints_internal_error`) and `json_asked` (`src/main.rs`) reads `--json` among the arguments before a `--`, `{"v":1,"error":"internal-error","detail":null}` prints on stdout in place of `error: internal error`, stderr empty. `run`, `revive`'s start, `mcp` and `hook` stay silent there.
- boundary-cli gains the argv read: `json_asked` reads `--json` ahead of clap, as `role_of` reads its role, and chooses only the form of the catch site's output.
**Why:** The catch site's fault gained its machine form; `json_asked` sits outside the clap parsing that boundary names and selects nothing else. The document in place of the stderr line, exit 1 and the human line staying, is the founder's own word, live, 2026-10-31 before 16:52Z, through the overseer's dialog, given after the widening was shown to him (`inputs#I2` Q1).
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — the exit-cause table
**Section:** §Error Handling (error-handling.md: new row)
**Change:**
- New row: `tests/cross_exit_causes.rs`, a root `cross_` binary with `required-features = ["fake-agent"]`, holds the exit-21 and exit-1 cause table, with `every_code_is_a_code_word`, `no_two_codes_share_a_hint`, its control `shared_hints_reads_a_planted_duplicate`, and `no_hint_names_release`.
**Why:** The chunk added the table as the witness of the causes across surfaces; it reads green.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — revive --json documents every refusal of its start
**Section:** §Input Validation (input-validation.md: CLI arguments / stdin, the `revive` arguments and two new records; Child executable resolution)
**Change:**
- `revive --json`'s document was for a refused strict-modes check or a preflight refusal; now for any refusal of the start it shares with `run`.
- Its closed exit-1 codes were five, `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`; now ten, adding `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`, in the new `revive --json` codes record.
- The clause that a refusal of `run`'s start after a passed preflight prints no document, and its pointer owed to the route entry "Exit-cause code catalogue", are gone.
- New record: an anyhow error from `trusted`, `checked_exe`, `open_wrapper_log`, `preflight` or `start` prints `internal-error` and returns the same error, so the catch site still logs its chain and its line; a `pump_child` error prints no document, the child having held the terminal, and viola writes nothing past it.
- Child executable resolution: under `viola revive --json`, through `refused_with`, a refused batch script prints `{"v":1,"error":"batch-script-child","detail":null}` on stdout in place of the pair, stderr empty, and a `claude` not found prints `internal-error`, each before `spawn_child`.
**Why:** The chunk closed the post-preflight gap owed to it, and `v1-39` reads met. `run`'s start reaching a machine reader through `revive --json` is a named carve-out of the founder's own word, live, 2026-10-31 before 16:52Z, through the overseer's dialog, given after the widening was shown to him (`inputs#I2` Q4). That these rows lie inside his answers, and applying check 7's contradictions on the two new records, are the overseer's as delegate, PROVISIONAL (`inputs#I6`).
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/
