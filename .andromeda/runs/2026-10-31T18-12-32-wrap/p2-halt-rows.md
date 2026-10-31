# The P2 halt — check 7's contradictions, each raised as written (`inputs#I5` item 4)

## Q2 — architecture (10)

- fanout-architecture--cascade-g1.md 2 · lines 124–124 · «a refusal is the `instance-unreachable` result with `detail: null`» · the standing line gives a refused `check_instance` a null detail, the proposal gives it the cause `StrictModes`, and it adds after the line without changing it
  - Recommended: apply: fanout-architecture--cascade-g1.md 1 rewrites line 124.
- fanout-architecture--cascade-g1.md 4 · lines 305–305 · «a refused strict-modes check or a preflight refusal prints `{"v":1,"error":"<code>","detail":null}` (`strict-modes-failed`» · revive's listed preflight codes hold `already-live` and no `stale-heartbeat`, while line 255 has revive refuse a stale name through the same collision check, which the proposal makes print `stale-heartbeat`
  - Recommended: apply: fanout-architecture--cascade-g1.md 7 rewrites line 305's code list, `stale-heartbeat` in it.
- fanout-architecture--cascade-g1.md 8 · lines 64–64 · «the cause has no exit code, `--json` code or `RefusalReason` of its own» · the file gives the server-verify cause no `--json` code of its own, the proposal gives it a word through `UnreachableCause::ServerVerify` that the two clients write
  - Recommended: apply: fanout-architecture--cascade-g1.md 16 rewrites line 64's clause.
- fanout-architecture--cascade-g1.md 14 · lines 38–38 · «prints exactly `error: internal error` on stderr once» · the file has every `cli` verb print that human line on stderr with no `--json` exception, the proposal has `--json` print a stdout document in its place with stderr empty
  - Recommended: apply: fanout-architecture--cascade-g1.md 12 extends line 38.
- fanout-architecture--cascade-g1.md 18 · lines 64–64 · «the cause has no exit code, `--json` code or `RefusalReason` of its own» · the file says this cause has no `--json` code and gives it no platform exception, the proposal names the code `server-verify-failed` and says it does not arise on macOS
  - Recommended: apply: fanout-architecture--cascade-g1.md 16 rewrites line 64's clause.
- fanout-architecture--cascade-g2.md 6 · lines 111–117 · «prints exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout (exit 21)» · lines 111 and 117 give the exit-21 document's `detail` as null, the proposal as one of five closed causes
  - Recommended: apply: fanout-architecture--cascade-g2.md 5 and 7 rewrite lines 111 and 117.
- fanout-architecture--cascade-g2.md 8 · lines 117–117 · «print the same `{"v":1,"error":"instance-unreachable","detail":null}` on stdout under `--json` (exit 21) for their refused strict-modes check of the home and the target instance, made by `client::refuse_untrusted`» · line 117 has `refuse_untrusted`'s document read `detail` null, the proposal has it write `StrictModes` (`strict-modes-failed`)
  - Recommended: apply: fanout-architecture--cascade-g2.md 7 rewrites line 117.
- cascade-compare-architecture.md 1 · lines 36–36 · «a refusal `run`'s start makes after a passed preflight prints no document» · the file says those start refusals print no document and lists five codes, the proposal has each print one and lists ten
  - Recommended: reject: fanout-architecture--cascade-g1.md 11 rewrites line 36, and fanout-architecture--cascade-g2.md 9 and 10 hold the fact in standard-contracts.
- cascade-compare-architecture.md 3 · lines 305–305 · «a refused strict-modes check or a preflight refusal prints `{"v":1,"error":"<code>","detail":null}` (`strict-modes-failed`» · the same conflict as fanout-architecture--cascade-g1.md 4, which states the same fact: revive's listed preflight codes hold no `stale-heartbeat`, while line 255 sends a stale name through the same collision check
  - Recommended: reject: the verdict itself names it the same conflict and the same fact as fanout-architecture--cascade-g1.md 4.
- cascade-compare-architecture.md 6 · lines 111–117 · «prints exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout (exit 21)» · lines 111 and 117 give `detail` as null, the proposal as `"<cause>"` of five closed causes; it also restates fanout-architecture--cascade-g2.md 6 and 8
  - Recommended: reject: fanout-architecture--cascade-g2.md 5, 6, 7 and 8 hold the fact; the verdict says it restates them.

## Q3 — security-plan (6)

- fanout-security-plan--cascade-g1.md 13 · lines 27–27 · «`viola_state::liveness::live_endpoint`; else `{"error":"instance-unreachable","detail":null}`» · the file gives MCP's no-endpoint case one answer with a null detail, the proposal has `frame` call a reading that tells `instance-dead` from `unwrapped-name` there
  - Recommended: apply: fanout-security-plan--cascade-g1.md 2 rewrites line 27's clause.
- fanout-security-plan--cascade-g1.md 14 · lines 27–27 · «MCP returns `instance-unreachable` with `detail: null` and writes no line» · the file holds MCP's detail null on a mismatch, on a strict-modes refusal (line 36 too) and with no endpoint, the proposal puts each one's cause in `detail`
  - Recommended: apply: fanout-security-plan--cascade-g1.md 2, 3, 4, 5 and 8 rewrite lines 27 and 36.
- fanout-security-plan--cascade-g2.md 10 · lines 22–22 · «{"v":1,"error":"instance-unreachable","detail":null}» · line 22 gives an exit-21 strict-modes refusal under `--json` a null `detail`, and lines 12–13 give MCP's failed server check and strict-modes refusal `detail: null`, where the proposal has `detail` carry `strict-modes-failed` and `server-verify-failed` on both surfaces
  - Recommended: apply: fanout-security-plan--cascade-g2.md 1, 2 and 3 rewrite lines 12, 13 and 22.
- fanout-security-plan--cascade-g2.md 16 · lines 38–38 · «a refusal `run`'s start makes after a passed preflight prints no document» · the standing line closes the `revive --json` codes at five and has a start refusal after a passed preflight print no document, the proposal lists ten codes and one document for every refusal of the shared start; the proposal adds a line beside line 38 and leaves its text standing
  - Recommended: apply: fanout-security-plan--cascade-g2.md 7 rewrites line 38.
- fanout-security-plan--cascade-g2.md 17 · lines 38–38 · «a refusal `run`'s start makes after a passed preflight prints no document» · the standing line has `start` print no document and holds no `internal-error` in its closed codes, the proposal has an anyhow error from `start` and the earlier steps print `internal-error`; the line speaks of a refusal and the proposal of an anyhow error, and no standing line states the `pump_child` half
  - Recommended: apply: fanout-security-plan--cascade-g2.md 7 rewrites line 38.
- cascade-compare-security-plan.md 1 · lines 99–99 · «a refusal is `instance-unreachable` with `detail: null`, no frame and no line» · the file gives an MCP strict-modes refusal `detail: null`, the proposal has that refusal name `strict-modes-failed` in `detail` on an MCP result
  - Recommended: apply: fanout-security-plan--cascade-g1.md 1 rewrites line 99; the threat model's own record of what reaches a document.

## Q4 — design-system (8)

- fanout-design-system--cascade.md 4 · lines 83–83 · «The stale-heartbeat and unwrapped-name causes have no code yet.» · the file says the stale-heartbeat cause has no code, the proposal gives a stale holder the code `stale-heartbeat`
  - Recommended: apply: fanout-design-system--cascade.md 5 rewrites line 83.
- fanout-design-system--cascade.md 9 · lines 135–135 · «a refusal `run`'s start makes after a passed preflight prints no document as landed» · the file gives `revive --json` five codes and no document for a refusal of `run`'s start (line 144 likewise), the proposal one document for every such refusal and ten closed codes
  - Recommended: apply: fanout-design-system--cascade.md 8 rewrites line 135 and 12 rewrites line 144.
- fanout-design-system--cascade.md 10 · lines 144–144 · «every other exit 1 has no `--json` document (a `run` start refusal» · the file gives a `run` start refusal reached through `revive`, and every other exit 1, no document; the proposal prints the `internal-error` document for a program not found and for an anyhow error of the start
  - Recommended: apply: fanout-design-system--cascade.md 12 rewrites line 144.
- fanout-design-system--cascade.md 14 · lines 144–144 · «every other exit 1 has no `--json` document» · the file gives the catch site's exit 1 no document, the proposal prints the `internal-error` document on stdout under `--json`
  - Recommended: apply: fanout-design-system--cascade.md 12 rewrites line 144.
- fanout-design-system--cascade.md 18 · lines 152–152 · «`{"v":1,"error":"instance-unreachable","detail":null}` (a per-cause `detail` code awaits an arch amendment)» · the file fixes exit 21's `detail` at null with per-cause codes still awaited (lines 83 and 168 likewise), the proposal carries one of five closed causes there
  - Recommended: apply: fanout-design-system--cascade.md 5, 13 and 17 rewrite lines 83, 152 and 168.
- fanout-design-system--cascade.md 19 · lines 152–152 · «the `list`, `link` and `unlink` refusals and the six frame verbs' strict-modes refusals included» · the file's `detail` is null for these strict-modes refusals, the proposal reads `strict-modes-failed` in that document
  - Recommended: apply: fanout-design-system--cascade.md 13 rewrites line 152.
- fanout-design-system--cascade.md 20 · lines 83–83 · «The stale-heartbeat and unwrapped-name causes have no code yet.» · the file says the unwrapped-name cause has no code, the proposal reads `unwrapped-name` for an absent directory, a regular file or a symlink
  - Recommended: apply: fanout-design-system--cascade.md 5 rewrites line 83.
- fanout-design-system--cascade.md 21 · lines 83–83 · «at their exit-21 server-verification refusal, whose `--json` document stays `detail:null`» · the file keeps the server-verification and not-running documents at `detail:null` (line 168 likewise), the proposal names each of these causes in `detail`
  - Recommended: apply: fanout-design-system--cascade.md 5 and 17 rewrite lines 83 and 168.

## Q5 — test-plan (12)

- fanout-test-plan--cascade-g1.md 10 · lines 87–87 · «logs each closed `Unreachable` cause (`instance-dead` · `server-verify-failed`» · the file's closed list of `Unreachable` causes names two, the proposal's `UnreachableCause` holds five
  - Recommended: apply: fanout-test-plan--cascade-g1.md 13 rewrites line 87; `Unreachable`'s four and `UnreachableCause`'s five are two closed sets, and the entry names both.
- fanout-test-plan--cascade-g2.md 4 · lines 56–57 · «each get `isError` with `{"error":"instance-unreachable","detail":null}`» · the standing impostor and widened-instance cases read `detail` null, as does line 68, while the proposal gives a server that fails verification `server-verify-failed` and a refused `check_instance` `strict-modes-failed`
  - Recommended: apply: fanout-test-plan--cascade-g2.md 2, 3 and 6 rewrite lines 56, 57 and 68.
- fanout-test-plan--cascade-g3.md 1 · lines 91–91 · «CLI exit 21 with `{"error":"instance-unreachable","detail":null}`; MCP `is_error:true` with the same body» · the standing Step 7 gives a killed wrapper and a dead instance `detail` null on both surfaces, the proposal gives MCP a lost cause and a row after `cleanup` that reads `instance-dead` or `unwrapped-name`
  - Recommended: apply: fanout-test-plan--cascade-g3.md 2 rewrites line 91.
- fanout-test-plan--cascade-g3.md 3 · lines 275–275 · «As landed, `cross_exit_causes.rs` does not exist» · the proposal names a witness in `tests/cross_exit_causes.rs`, a file this line says does not exist; the catch site's document and its human line themselves stand in no group
  - Recommended: apply: fanout-test-plan--cascade-g3.md 5 rewrites line 275.
- fanout-test-plan--cascade-g3.md 6 · lines 275–275 · «As landed, `cross_exit_causes.rs` does not exist and no rstest table joins the causes» · the proposal says `tests/cross_exit_causes.rs` asserts each cause, the line says the file does not exist, and the change lands after the line and leaves its words as they are · its own line rewritten
  - Recommended: apply: fanout-test-plan--cascade-g3.md 5 rewrites line 275.
- fanout-test-plan--cascade-g3.md 7 · lines 282–282 · «its `--json` document is every exit-21 cause's `{"v":1,"error":"instance-unreachable","detail":null}`» · the line gives every exit-21 cause `detail` null, the proposal gives five closed `detail` causes
  - Recommended: apply: fanout-test-plan--cascade-g3.md 9 rewrites line 282.
- fanout-test-plan--cascade-g3.md 16 · lines 293–293 · «A refusal `run`'s start makes after a passed preflight (`batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, a `claude` not on `PATH` as `internal-error`) prints no document under `revive --json`» · the line says these four refusals print no document, the proposal lists them among revive's `--json` documents, and the change lands after the line and leaves its words as they are · its own line rewritten
  - Recommended: apply: fanout-test-plan--cascade-g3.md 15 rewrites line 293.
- fanout-test-plan--cascade-g3.md 17 · lines 293–293 · «a `claude` not on `PATH` as `internal-error`) prints no document under `revive --json` and no case asserts one» · the line says no case asserts this refusal's document, the proposal names its witness `a_revive_with_no_claude_on_its_path_is_internal_error`, and the change lands after the line and leaves its words as they are · its own line rewritten
  - Recommended: apply: fanout-test-plan--cascade-g3.md 15 rewrites line 293.
- fanout-test-plan--cascade-g3.md 18 · lines 293–293 · «(`batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, a `claude` not on `PATH` as `internal-error`) prints no document under `revive --json` and no case asserts one» · the line says `batch-script-child` has no document and no case, the proposal names `a_revive_whose_claude_is_a_script_is_the_batch_script_child_document`, and the change lands after the line and leaves its words as they are · its own line rewritten
  - Recommended: apply: fanout-test-plan--cascade-g3.md 15 rewrites line 293.
- cascade-compare-test-plan.md 5 · lines 10–10 · «never hand-written. Three exceptions: the chaos tests» · the line closes hand-written on-disk state, `snapshot.json` among it, at three exceptions, and `record_live` writing an `InstanceSnapshot` from a handler test is a fourth it does not name
  - Recommended: apply: cascade-compare-test-plan.md 4 rewrites line 10's closed count, three to four exceptions.
- cascade-compare-test-plan.md 10 · lines 34–34 · «`<code>` one of `strict-modes-failed`, `already-live`, `no-session`, `session-live`, `cwd-missing`» · the file closes `revive --json`'s exit-1 document to these five codes (line 43 repeats "its five codes"), while the proposal's `revive_json_of_a_stale_holder_is_the_stale_heartbeat_document` gives a sixth, `stale-heartbeat`
  - Recommended: apply: fanout-test-plan--cascade-g1.md 2 and 4 rewrite lines 34 and 43.
- cascade-compare-test-plan.md 12 · lines 63–63 · «exit 21 with the instance pair or the `instance-unreachable` document, no frame and no line» · the standing row names the bare `instance-unreachable` document with no `detail`, the proposal names it with `detail` `strict-modes-failed` · its own line rewritten
  - Recommended: apply: its own line rewritten.

## Q6 — obs-plan (12)

- fanout-obs-plan--cascade-g1.md 1 · lines 490–490 · «Each cause behind exit 21 (dead instance, strict-mode fail, server-verify fail)» · the file names three causes behind exit 21, the proposal a closed set of five; its "every word is written" also meets line 165, where a refused strict-modes check writes no line
  - Recommended: apply: fanout-obs-plan--cascade-g1.md 12 rewrites line 490. Its "every word is written" means into the two documents; a refused strict-modes check still writes no line, as line 165 says.
- fanout-obs-plan--cascade-g1.md 13 · lines 165–165 · «each cause behind exit 21 and exit 1 gets a distinct detail code, except a strict-modes check refused before any log opens» · the file's one named exception is the strict-modes check, the proposal adds a socket-directory cause with no code; its macOS half already stands at lines 459 and 491
  - Recommended: apply: fanout-obs-plan--cascade-g1.md 4 extends line 165 with the two named carve-outs, `inputs#I2` Q4.
- fanout-obs-plan--cascade-g2.md 6 · lines 95–95 · «exit 21 `detail:"instance-dead"` with `during` `connect` (no live snapshot, a replayed one included, or the connect failed)» · the file reads every no-live-snapshot exit 21 as `instance-dead`, the proposal reads an absent path, a regular file or a symlink as `unwrapped-name`; line 112 likewise gives exit 21 only `instance-dead` or `server-verify-failed`
  - Recommended: apply: fanout-obs-plan--cascade-g2.md 7 rewrites line 95, and fanout-obs-plan--cascade-g3.md 1 line 112.
- fanout-obs-plan--cascade-g2.md 8 · lines 95–95 · «`detail:"instance-dead"` with `during` `connect` (no live snapshot, a replayed one included, or the connect failed) or `call` (the connection lost after it)» · the file gives `instance-dead` a `during` of `connect` or `call` and names no `endpoint-lost`, the proposal makes `instance-dead` `connect` only and `call` belong to `endpoint-lost` alone; line 112 states the same
  - Recommended: apply: fanout-obs-plan--cascade-g2.md 7 rewrites line 95, and fanout-obs-plan--cascade-g3.md 1 line 112.
- fanout-obs-plan--cascade-g3.md 4 · lines 112–112 · «exit 21 carries `detail:"instance-dead"` plus `during` ∈ `connect|call` (endpoint vanished mid-wait = `during:"call"`)» · the file logs the vanished endpoint as `instance-dead` `during:"call"`, the proposal makes `call` belong to `endpoint-lost` alone and `instance-dead` `connect` only; line 95 states the same, and fanout-obs-plan--cascade-g2.md 8 proposes this fact too
  - Recommended: reject: the verdict names it the same fact as fanout-obs-plan--cascade-g2.md 8, in the same file; fanout-obs-plan--cascade-g3.md 1 rewrites line 112.
- fanout-obs-plan--cascade-g3.md 5 · lines 95–95 · «exit 21 `detail:"instance-dead"` with `during` `connect` (no live snapshot, a replayed one included, or the connect failed)» · the file reads every no-live-snapshot exit 21 as `instance-dead`, the proposal reads an absent path, a regular file or a symlink as `unwrapped-name`; line 112 likewise gives exit 21 only `instance-dead` or `server-verify-failed`, and fanout-obs-plan--cascade-g2.md 6 proposes this fact too
  - Recommended: reject: the verdict names it the same fact as fanout-obs-plan--cascade-g2.md 6, in the same file.
- fanout-obs-plan--cascade-g4.md 4 · lines 26–26 · «`already-live` (a `live` or a `stale` holder of the name» · the file gives a stale holder's exit 1 the code `already-live`, the proposal `stale-heartbeat` · its own line rewritten
  - Recommended: apply: its own line rewritten, fanout-obs-plan--cascade-g4.md 2.
- fanout-obs-plan--cascade-g4.md 8 · lines 28–28 · «closes the `process-exit` `detail` at 13 values» · the file closes the catalog at 13 values, the proposal adds six words to it, which makes 19
  - Recommended: apply: fanout-obs-plan--cascade-g4.md 5 rewrites line 28, 13 to 19.
- fanout-obs-plan--cascade-g4.md 12 · lines 16–16 · «exactly `error: internal error`» · the file gives `revive <name> --list`'s catch-site line as exactly this with no `--json` exception, the proposal prints the `internal-error` document in its place under `--json`, stderr empty
  - Recommended: apply: fanout-obs-plan--cascade-g4.md 11 extends line 16.
- fanout-obs-plan--cascade-g4.md 19 · lines 40–40 · «a refusal returns `instance-unreachable` with `detail: null`» · the file gives a refused `check_instance` in `frame` a null detail, the proposal maps it to the cause `StrictModes`
  - Recommended: apply: fanout-obs-plan--cascade-g4.md 18 rewrites line 40.
- cascade-compare-obs-plan.md 1 · lines 27–27 · «{"v":1,"error":"instance-unreachable","detail":null}» · the file's exit-21 document for `link` / `unlink` carries a null detail, the proposal's `detail` is one of five closed cause words
  - Recommended: apply: fanout-obs-plan--cascade-g4.md 17 rewrites line 27.
- cascade-compare-obs-plan.md 5 · lines 95–95 · «exit 21 `detail:"instance-dead"` with `during` `connect` (no live snapshot, a replayed one included, or the connect failed)» · the file reads every no-live-snapshot exit 21 as `instance-dead`, the proposal keeps that for a directory (a stale instance included) but reads anything else as `unwrapped-name`; fanout-obs-plan--cascade-g2.md 6 proposes the same reading, and the stale-instance clause stands nowhere in the file
  - Recommended: reject: fanout-obs-plan--cascade-g2.md 6 holds the reading and fanout-obs-plan--cascade-g2.md 7 rewrites line 95; the verdict names it the same reading.

## Q7 — a11y-plan (3)

- fanout-a11y-plan--cascade-g1.md 6 · lines 8–8 · «a refusal `run`'s start makes after a passed preflight prints no document as landed» · the standing clause says such a start refusal prints no document, the proposal says each prints one exit-1 document
  - Recommended: apply: fanout-a11y-plan--cascade-g1.md 3 removes line 8's clause.
- fanout-a11y-plan--cascade-g1.md 7 · lines 8–8 · «a refusal `run`'s start makes after a passed preflight prints no document as landed» · the standing clause gives a start's failure after the preflight no document, the proposal gives a `claude` not found the `internal-error` document
  - Recommended: apply: fanout-a11y-plan--cascade-g1.md 3 removes line 8's clause.
- fanout-a11y-plan--cascade-g2.md 3 · lines 19–31 · «under `--json` the `{"v":1,"error":"instance-unreachable","detail":null}` document with stderr empty and no hint» · the file gives exit 21's `--json` document a null `detail` for the server-verify and strict-modes causes (also lines 92 and 105), the proposal one of five closed words, `strict-modes-failed` and `server-verify-failed` among them
  - Recommended: apply: fanout-a11y-plan--cascade-g2.md 2, 4, 5, 13 and 14 rewrite lines 19, 25, 31, 92 and 105.

## Q8 — no companion rewrites the standing line (2)

- cascade-compare-architecture.md 5 · lines 42–42 · «`verify` exits 1 with the home-state pair, before its `cli` log and any step line, when `check_home_tree` refuses its home, `ledger/`, `bin/` or `plugin/`, or `update_stamps` refuses `ledger/stamps.json`» · the proposal has `verify`'s `process-exit` carry `strict-modes-failed` and keeps only a refused home tree lineless, while line 42 puts both named strict-modes refusals, the `update_stamps` one included, before any log; the rest of the record stands in no group
  - Recommended: apply, and rewrite `occupied-resources.md:42`'s `verify` sentence: `update_stamps`'s refusal is no longer before any line; it writes `strict-modes-failed` on verify's `process-exit` under a parsed `VIOLA_NAME` (`src/cmd/verify.rs`, `verify_window_a_widened_stamps_file_refuses_with_its_code`). A refused home tree alone stays lineless. The chunk's boundary reads this as `verify`'s `update_stamps` refusal already ending in its own `process-exit`, so the line was already wrong; no refused check gained a line.
- fanout-obs-plan--cascade-g4.md 6 · lines 26–26 · «`strict-modes-failed` is no longer one of them» · the file excludes `strict-modes-failed` from the `process-exit` exit 1 details, the proposal lists it among `verify`'s codes beside exit 1's `interrupted`
  - Recommended: apply, and rewrite `log-coverage.md:26`'s clause "`strict-modes-failed` is no longer one of them" to scope it to `run`'s and `revive`'s start: a refused check there still writes no line, and `verify`'s stamps refusal does carry the word. Neither fanout-obs-plan--cascade-g4.md 2 nor 3 reaches that clause.
