# tests · Test Scope Summary — category read

## Carried
- (.andromeda/test-plan-detail/test-scope-summary.md:11) The served `link` / `unlink` channel methods fall inside the channel's testable same-user surface, so their dispatch, refusal order and server verification must be asserted, not flagged untestable.
  > same-user behaviour is testable: framing, method dispatch, refusal order, `-32602` on a higher `v`, the pid/start-time server verification, and `run` exiting 1 on a squatted name.
- (.andromeda/test-plan-detail/test-scope-summary.md:12) If the two verbs become frame-writing CLI verbs, the server check they take through `client::connect` must be asserted the way the six landed verbs are: an impostor gets no frame and exit 21 beside a control, on Linux and Windows, with the macOS case owed.
  > **As landed (server verification):** the pid/start-time server verification is asserted on Linux and Windows: the six CLI frame verbs by `tests/security_negatives_server_identity.rs` (an impostor server gets no frame and the verb exits 21, beside a control that reaches the recorded wrapper; §6 Security control negatives), the hook by `tests/hook_fail_open.rs`'s impostor rows (exit 0, no body, no frame), the MCP frame tools by `crates/viola-mcp/tests/handlers.rs` (`instance-unreachable`, no frame), and the decision table by the `viola-state` unit `server_verdict_rows`. These negatives compile out on macOS, where the client reads no peer pid; the macOS cases are owed (§6 Exit-cause matrix).
- (.andromeda/test-plan-detail/test-scope-summary.md:33) `link` and `unlink` are named entities of the CLI `--json` / exit-code contract, so the verbs must land inside its closed exit set and its one-document shape.
  > CLI verbs and the `--json` / exit-code contract (`run · revive · send · wait · last · list · answer · pause · release · link · unlink · verify · plugin install`, global `--home`, exits 0/1/2/10–14/20/21;
- (.andromeda/test-plan-detail/test-scope-summary.md:41) The cli surface under test lists `link · unlink`, so the verbs need cli-surface tests when served.
  > **Surface:** cli (`viola` verbs: `send · wait · last · list · answer · pause · release · link · unlink · verify · plugin install · run` and `revive` launch semantics;
- (.andromeda/test-plan-detail/test-scope-summary.md:42) The verbs' tests must drive the binary as a subprocess with a per-test temp home and `--json`.
  > **Driver:** subprocess + `assert_cmd` / `Command::cargo_bin("viola")`, with `--home <per-test temp>` and `--json`
- (.andromeda/test-plan-detail/test-scope-summary.md:43) The verbs' signal is the exit code plus one JSON document, and their three human result lines are checked by line match with no SGR outside a TTY.
  > **Signal:** exit code (0, 1, 2, 10–14, 20, 21) plus one JSON document on stdout (`{"v":1,"ok":…}` / `{"v":1,"refusal":…,"detail":…}` / `{"v":1,"error":…}`). Human mode is checked by stdout/stderr line match: `[RB] read back … cursor <n>`, `unable  <name>  <reason>  <detail>` + last-line `hint:`, `stamped <ver>  <n> pass  <n> fail`. There must be no SGR under non-TTY / `NO_COLOR` / `TERM=dumb`.
- (.andromeda/test-plan-detail/test-scope-summary.md:57) `link` and `unlink` are listed methods of the wrapper channel surface, so their served answers are this surface's to assert.
  > methods `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`, `hook.dialog`, `hook.event`)
- (.andromeda/test-plan-detail/test-scope-summary.md:58) The channel cases (`tests/channel_links.rs` and the raw-channel name negative) must use the sync client or a raw local-socket client, with schema assertion.
  > **Driver:** language-native IPC test client (the `viola-channel` sync client, or a raw `interprocess` `local_socket` client for malformed frames) + JSON schema assertion
- (.andromeda/test-plan-detail/test-scope-summary.md:59) The served methods' results (`{since, already_linked}`, the empty object) and their `-32602` refusals (a foreign `driven`, `driver` equal to `driven`) are asserted as the success and protocol-fault payload shapes, cross-checked against the log cursor; today's `-32601` leaves.
  > **Signal:** IPC response payload check. Success is `{"result":{"ok":…}}`. Refusal is `{"result":{"refusal":…,"detail":…}}` with the first-applicable ordering. Protocol faults are `{"error":{code,message,data}}` with codes -32700 / -32600 / -32601 / -32602 (`data:{supported, wrapper}`) / -32603 (`"internal error"`, `data:null`). This is cross-checked against the `events.ndjson` byte-offset `cursor`.
- (.andromeda/test-plan-detail/test-scope-summary.md:65) No `link` / `unlink` MCP tool may be added: `tools/list` must stay free of them, which bounds how `v1-13`'s MCP half can be met.
  > There must be no `release` / `pause` / `link` / `unlink` tools in `tools/list`.
- (.andromeda/test-plan-detail/test-scope-summary.md:76) Scenario E3 needs two wrappers (`overseer`, `builder`), which `boot` must start in one home.
  > Starts one or more `viola run <name> -- <fake agent> <args>` wrappers.
- (.andromeda/test-plan-detail/test-scope-summary.md:106) Every link / unlink test must seed its own state in a fresh home.
  > Self-bootstrapping per test: a fresh `--home` temp dir.
- (.andromeda/test-plan-detail/test-scope-summary.md:171) The first product path appending `unlink` lines must keep the replay's link recovery true: a logged `unlink` removes the pair from the recovered `links`.
  > As landed, the replay recovers `links` from the logged `link` / `unlink` lines as `{driver, driven, since}` items (chunk 2026-10-10-session-links), the replay writes no file, and `viola revive` is its first product reader (chunk 2026-10-10-viola-revive)
- (.andromeda/test-plan.md:24) The chunk's test breadth is set by the Comprehensive tier.
  > **Test tier:** `Comprehensive (2)`
- (.andromeda/test-plan.md:170) `link` / `unlink` is one of the E2E expansion scenarios beyond the seven paths, which this entry owes.
  > Covered by E2E suite expansion in Phase 3 if needed (beyond the 7):
  > - R5: unwrapped session hooks are a silent no-op.
  > - R8: the `CLAUDE*` env strip.
  > - `link` / `unlink`.
- (.andromeda/test-plan.md:184) A served verb's server pid / start-time mismatch must be a negative test with CLI exit 21.
  > - server pid / start-time mismatch vs `snapshot.json` → CLI exit 21 / MCP `instance-unreachable` / hook exit 0 with no body
- (.andromeda/test-plan.md:200) The verbs' `<driver>` / `<driven>` arguments must be refused when they are not a `ViolaName`, tested as path-traversal negatives.
  > - path-traversal names (`../x`, uppercase, `/`) rejected by `parse_viola_name`
- (.andromeda/test-plan.md:201) Whichever form the founder picks, a `--home` failing strict-modes must be refused with the verb's exit code under test (the instance-check option makes it a pre-snapshot refusal).
  > - `--home` / `VIOLA_DIR` failing strict-modes → exit 1 / 21 / hook 0
- (.andromeda/test-plan.md:247) `v1-13`'s claim that the `--json` and MCP results carry the same facts is a cross-surface-coordination check.
  > - MCP and CLI return identical `ok` / refusal payloads for the same channel result
- (.andromeda/test-plan.md:265) The new methods' and verbs' error outputs must pass the sanitization assertions.
  > assertions that CLI `--json`, MCP `isError` and channel `error.data` contain no absolute paths, serde paths or values, anyhow chains or tool `input`. `-32603` must be exactly `"internal error"` / `data:null`.
- (.andromeda/test-plan.md:271) Link tests must wait on logged events and offsets, never sleeps.
  > Waits must key on logged events and byte offsets, not sleeps.
- (.andromeda/test-plan.md:320) The new suites must carry the layer prefixes (`tests/channel_links.rs` under `channel_`, the verb cases under `cli_`).
  > - Rust integration binaries use a `<surface>_<topic>.rs` prefix so nextest filtersets can select layers: `cli_`, `hook_`, `tui_`, `channel_`, `chaos_`, `contract_` in root; `path_`, `mcp_`, `cross_` in `viola-e2e`.
- (.andromeda/test-plan.md:323) New test functions follow the naming pattern.
  > General pattern: snake_case `<subject>_<condition>_<expected>`
- (.andromeda/test-plan.md:386) A served verb's server-check refusal must be asserted inside its own span, with its closed unreachable cause logged, as each frame verb's is.
  > the server check refused inside each frame verb's and the hook's span with each closed unreachable cause logged
- (.andromeda/test-plan.md:408) The third CARRY's `handoff:  link, unlink` help row moves the `HELP` literal that pins `viola --help`.
  > and `viola --help`, pinned by the `HELP` literal in `tests/cli_output_plain.rs` (chunk 2026-10-17-cli-output-discipline).
- (.andromeda/test-plan.md:435) `link` and `unlink` join "every method" of the channel suites: schema-validated shapes, cursor, `-32602` on a higher `v`, oversize frame and server verification.
  > - **Wrapper channel:** every method through the crate and root channel suites, with schema-validated result, refusal and error shapes, the `cursor` equal to the log length at acceptance, `-32602` on a higher `v`, `-32600` and a close on an oversize frame, server verification, and wrapper-side paste validation in which `control-character` wins over every other refusal.
- (.andromeda/test-plan.md:437) A strict-modes refusal of the verbs must read CLI exit 21, as for every other verb.
  > strict-modes refusals give exit 1 for `run` / `mcp` / `viola revive`, hook 0 with no output, and CLI 21 for every other verb.
- (.andromeda/test-plan.md:445) The verbs' cli cases must cover the `--json` document per exit code and the human result lines with no SGR.
  > root `tests/cli_*.rs` cover the `--json` document per exit code, the human layouts by trycmd or literal with no `\x1b[` under non-TTY / `NO_COLOR` / `TERM=dumb`
- (.andromeda/test-plan.md:447) Test-started `viola` drops the `VIOLA_*` names, so E3 step 2's `VIOLA_NAME=overseer` must be set per child explicitly.
  > - A test-started `viola` drops `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN`, and a path oracle compares `resolved(home)`, never `TestHome::path()`.
- (.andromeda/test-plan.md:561) Scenario E3's steps 1 and 3 are owed with the verbs: a doubled `link` yields one record, and `unlink` appends one.
  > - **Steps:**
  >   1. Boot `overseer` and `builder`. Run `viola link overseer builder --json` twice.
  >   2. Run `viola send builder` with `VIOLA_NAME=overseer`.
  >   3. Run `viola unlink overseer builder --json`.
  > - **Verification signal:**
  >   - Exactly one `link{driver:"overseer",driven:"builder"}` record exists, because re-linking adds none.
  >   - After unlink: an `unlink` record exists.
- (.andromeda/test-plan.md:570) E3's "owed, built in part" note must be restated once the verbs land (or stay as held if the founder's word keeps them out).
  > - **As landed: owed, built in part.** `viola link` and `viola unlink` are not built (the wrapper answers both `-32601`) and are held to the route entry "Explicit session links"; the implicit link that a `send` or `answer` from another instance records is held by root cases of `tests/cli_links.rs`.
- (.andromeda/test-plan.md:604) The verbs' exit-21 causes join the exit-cause matrix, and their hints must never contain `release`.
  > the server-verification cause is a case of `tests/security_negatives_server_identity.rs` on Linux and Windows (its macOS case owed to the route entry "Unix endpoint and home hardening"), and `revive`'s refusals are cases of `tests/cli_revive.rs`; driver-facing hints never contain `release`.
- (.andromeda/test-plan.md:606) The raw-channel `link` / `unlink` name-validation negative is owed by this entry.
  > raw-channel `link` / `unlink` name validation
- (.andromeda/test-plan.md:611) Any refusal the verbs share with an MCP surface must be equal across surfaces.
  > - for the same refusal, the MCP `structured_content` `{refusal, detail}` equals the CLI `--json` `{refusal, detail}`
- (.andromeda/test-plan.md:631) Link state must come from real wrapper runs; only the server-identity impostor cases may build a `snapshot.json`.
  > - **On-disk product state:** produced only by real `viola` runs against the fake agent in a fresh home, never hand-written — except the chaos tests' corruption of files the product already wrote, the Windows x64 `seed_conpty` companions and the server-identity impostors' test-built `snapshot.json`; a case that needs a home viola created starts from `stamped_home`.
- (.andromeda/test-plan.md:813) Removing `LinkSet::remove`'s dead-code allowance and adding verbs must hold the per-OS coverage gates.
  > - Gates are enforced per OS job with `cargo llvm-cov nextest … --fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80`. Per-OS gating is deliberate: `#[cfg(windows)]` / `#[cfg(unix)]` code compiles only on its own runner.
- (.andromeda/test-plan.md:868) The methods' `-32602` decision cases belong at unit / integration level, with E3 asserting only the cross-process behaviour.
  > - NEVER assert at E2E a case that a lower layer can hold. Parser, refusal-order, threshold and clock cases belong in §4, contract shapes in §5, and §6 asserts only cross-process behaviour (§2 pyramid).
- (.andromeda/test-plan.md:898) E3's two wrappers must be held by a `Drop` guard, not `assert_cmd`.
  > - NEVER use `assert_cmd` `.assert()` on the long-lived `viola run` wrapper. It blocks until exit; use std::process + a `Drop` guard.
- (.andromeda/test-plan.md:905) No link test may hand-write `snapshot.json` to plant links.
  > - NEVER hand-write `ledger/stamps.json`, `snapshot.json` or `budget.json`.

## Bindings
- **Verb server check ↔ security-plan (frame clients' server identity) and the founder's P4 form**: the verbs' impostor negative and span case follow the landed six-verb pattern (.andromeda/test-plan-detail/test-scope-summary.md:12), (.andromeda/test-plan.md:184), (.andromeda/test-plan.md:386); the strict-modes half (crossing (c)) is tested per the form named, exit 21 for a CLI verb (.andromeda/test-plan.md:201), (.andromeda/test-plan.md:437).
- **Channel `link` / `unlink` results ↔ architecture §Standard Contracts (Wrapper channel)**: the served payload and `-32602` shapes are the channel signal (.andromeda/test-plan-detail/test-scope-summary.md:59), (.andromeda/test-plan.md:435).
- **Help row ↔ layout-templates §Output structure — `viola --help`**: the `HELP` literal moves with the row (.andromeda/test-plan.md:408).
- **Result lines ↔ layout-templates (link result lines) and the second CARRY (`ViolaName::try_new` on a logged name)**: line match, no SGR (.andromeda/test-plan-detail/test-scope-summary.md:43), (.andromeda/test-plan.md:445); name negatives (.andromeda/test-plan.md:200), (.andromeda/test-plan.md:606).
- **No MCP tool ↔ `v1-13`'s MCP half (verification matrix)**: `tools/list` stays free of `link` / `unlink` (.andromeda/test-plan-detail/test-scope-summary.md:65), against the parity check (.andromeda/test-plan.md:247), (.andromeda/test-plan.md:611).
- **`unlink` lines ↔ the replay (viola-state) and E5**: the recovered `links` honour a logged `unlink` (.andromeda/test-plan-detail/test-scope-summary.md:171).
- **E3 ↔ §6 E2E Test Strategy detail**: steps 1 and 3 and the as-landed note (.andromeda/test-plan.md:561), (.andromeda/test-plan.md:570).
