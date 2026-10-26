# tests · cascade read · group 4 of 5

## Carried
- (.andromeda/test-plan-detail/e2e-test-strategy.md:261) The exit-cause matrix must hold the server-verification mismatch as its own exit-21 cause on every OS (Linux and Windows a same-user impostor listener on the recorded endpoint, macOS the loosened socket dir), each case asserting exit 21, its `--json` document and a last `hint:` line that names it and no other cause; E1's unwrapped-row `send` must keep exiting 21 through the new check.
  > Every case asserts the exit code, the `--json` document, and a last stderr `hint:` line that names that cause. A shared hint across causes fails the table
  (.andromeda/test-plan-detail/e2e-test-strategy.md:262)
  >   - Exit 21 (`instance-unreachable`) causes:
  >     - the home fails strict-modes (Unix 0770 home; Windows synthetic-descriptor stub)
  >     - server verification fails. On Linux and Windows: `snapshot.json` pid or start time does not match the listening process (a same-user impostor listener on the recorded endpoint), as a `#[cfg(not(target_os = "macos"))]` case. On macOS the client checks only euid and the socket dir (security excerpt, Vector 1), so the `#[cfg(target_os = "macos")]` case loosens the verified per-user socket dir to 0755 and asserts exit 21. A same-user impostor inside a valid 0700 dir on macOS is an accepted upstream limitation, not asserted as a refusal.
  >     - no instance is running for the name: the endpoint is gone after `cleanup`
  >     - the name is an unwrapped `claude agents --json` row, never a target (E1); as landed a `tests/cli_list.rs` case, `e1_send_naming_an_unwrapped_row_is_unreachable`, not a `cross_exit_causes.rs` row, proving exit 21 only: the unwrapped-name hint is not built, owed to the existing entry "Exit-cause code catalogue" (`v1-18`)
  (.andromeda/test-plan-detail/e2e-test-strategy.md:282)
  >   - Driver-facing hints never contain `release`: this holds for every refusal row in the table (`human-typing`, `budget-paused`), not only in Paths 5 and 6.
  (.andromeda/test-plan.md:604)
  > - **Exit-cause matrix** (`cross_exit_causes.rs`): one negative test per cause of exit 21 and of exit 1, each asserting the exit code, the `--json` document and a last `hint:` line that names its cause — a hint shared across causes fails the table; `revive`'s refusals are cases of `tests/cli_revive.rs`; driver-facing hints never contain `release`.
  (.andromeda/test-plan.md:545)
  > - A `send` to the unwrapped row's name exits 21: a row `name` is never a target. As landed: `tests/cli_list.rs` `e1_send_naming_an_unwrapped_row_is_unreachable` (exit 21; the unwrapped-name hint not built, owed to "Exit-cause code catalogue").
  (.andromeda/test-plan.md:889)
  > - NEVER use interprocess `try_overwrite`, or reuse an endpoint name across tests. Take the endpoint only from `snapshot.json`.
  (.andromeda/test-plan.md:873)
  > - NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:242) The hook fail-open matrix must gain a server-verification failure case beside its channel-failure case, for `hook.event` and `hook.dialog` alike: exit 0, empty stdout and stderr, no frame written.
  > and the hook fail-open matrix in root `hook_fail_open.rs` (assert_cmd).
  (.andromeda/test-plan-detail/e2e-test-strategy.md:244)
  > The hook fail-open matrix cases are oversize stdin, malformed JSON, clap error, channel failure, forced panic and missing `VIOLA_NAME`. The sweep also covers the `--home` strict-modes refusals.
  (.andromeda/test-plan.md:598)
  > - **Security sweep:** nextest runs the MCP schema-violation cases (each `invalid-params` with its field, no frame written) and the hook fail-open matrix — oversize stdin, malformed JSON, clap error, channel failure, a forced panic and a missing `VIOLA_NAME`, each exit 0 with no body — plus the `--home` strict-modes refusals; the panic seam fires only under the `fake-agent` feature and never reaches the release build.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:288) The server-verification control must have its own negative in `security_negatives_*.rs` unless one stands elsewhere, landing beside the Windows client SQOS case whose viola-client half lands with `viola-channel`'s first consumer; the cross-user peer stays asserted only at the peer-credential boundary.
  > - **Security control negatives** (`security_negatives_*.rs`; one negative test per security-plan control that had none):
  (.andromeda/test-plan-detail/e2e-test-strategy.md:290)
  > - **Windows client SQOS** (`#[cfg(windows)]`, client-side server verification). A test-owned same-user pipe server accepts the viola client's connection, calls `ImpersonateNamedPipeClient`, then reads the thread token's `TokenImpersonationLevel`. It must be `SecurityIdentification`, never `SecurityImpersonation` or higher, which proves the client opened with `SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION`.
  (.andromeda/test-plan-detail/e2e-test-strategy.md:290)
  > The `security_negatives_*.rs` case against the viola client lands with `viola-channel`, its first consumer.
  (.andromeda/test-plan.md:606)
  > - **Security control negatives** (`security_negatives_*.rs`): one negative test per security-plan control that had none — the Unix socket directory refusal, Windows client SQOS at the identification level, the Windows listener rejecting remote clients and denying an anonymous open (each beside its controls), raw-channel `link` / `unlink` name validation, a Windows `--home` outside the profile given a protected DACL (its test home never under `%TEMP%`), umask-independent modes, and single writers of `stamps.json` and `snapshot.json`.
  (.andromeda/test-plan.md:476)
  > - Cross-OS-user peers. Covered by the peer-credential boundary unit test (§4) and same-user SDDL read-back (§5).
  (.andromeda/test-plan.md:468)
  > - **ipc-internal (wrapper channel):** the sync client, a raw interprocess stream and jsonschema; the endpoint is read only from `snapshot.json`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:250) The new unreachable and fail-open outcomes must carry codes and fixed messages only: neither the compared pid and start time nor a path may reach any CLI `--json`, MCP `isError` or channel `error.data`, and a verification fault surfacing as `-32603` stays exactly `"internal error"` / `null`.
  > - **Error sanitization and secret scan:** after every E2E test, the `events` support helper checks:
  >   - CLI `--json`, MCP `isError` and channel `error.data` contain no absolute path (drive letter, `/home/`, `/Users/`, the temp home prefix), no serde path and no `Caused by`
  >   - every -32603 is exactly `"internal error"` / `null`
  (.andromeda/test-plan.md:600)
  > - **Error sanitization and secret scan:** after every E2E test, no absolute path, serde path or `Caused by` in any error payload, every `-32603` exactly `"internal error"` / `null`, no `Cookie` header or stripped-variable value in any log, `diagnostics/` files 0600 on Unix, and the fixed canary in no home-level diagnostics line; a hit is reported by place and class, never the matched bytes.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:316) An endpoint vanishing during `wait` must still read exit 21 through the new check, and a killed wrapper must still give CLI exit 21 and MCP `instance-unreachable`; the SIGSTOP `stale` and post-SIGKILL gone readings must keep holding beside the edited pid + start-time match in `liveness.rs`.
  >   - endpoint vanishes during `wait`: exit 21
  >   - SIGSTOP the wrapper (Unix) for over 5 s: `liveness:"stale"`, and a new `run` exits 1; after SIGKILL: omitted as gone
  (.andromeda/test-plan.md:613)
  > - **Chaos suite** (`chaos_*.rs`): a torn append counted and healed on a fresh line, a no-EOF child's exit detected on the handle, an endpoint vanishing during `wait` (exit 21), a SIGSTOPped wrapper read `stale` and its name refused to a new `run`, a forced vt100 panic refusing `send` `input-not-ready` while passthrough continues, forward-compat lines counted in `skipped`, and a `.cmd` child refused exit 1.
  (.andromeda/test-plan.md:504)
  > a killed wrapper gives CLI exit 21 and MCP `instance-unreachable`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:235) E5 steps 1–3 and the five snapshot readers' move to the replay are owed to this entry but held behind the founder's replay ruling: they are built here only if it lands, and then the two set-against signals (`wheel:"human"`, a rebuilt snapshot) must be restated; without it each reader keeps today's outcome.
  > Steps 1–3 stay unbuilt while `run`'s collision check calls `read_snapshot`; the five other snapshot readers taking the replay are owed to "Server verification before any frame" (the review card the founder approved). A replayed snapshot carries no `cwd`, so a revive over one is refused `cwd-missing`, and a revived start's first record is `wheel{holder:"driver", cause:"start"}` too. Two of its signals are set against what landed, for the entry that builds it to restate: every start appends `wheel{holder:"driver", cause:"start"}` (as measured at `tests/chaos_torn_append.rs`, the second start's first record), so `wheel:"human"` does not hold after step 3; and the replay writes no file, so no snapshot is rebuilt from it.
  (.andromeda/test-plan.md:590)
  > - **As landed: in part.** The replay's product readers (`viola revive`, `viola list`), its budget and link legs and its `list --json` leg are covered; steps 1–3 stay unbuilt while `run`'s collision check calls `read_snapshot`, owed to the route entry "Server verification before any frame".
  (.andromeda/test-plan.md:592)
  > - Two of the signals above are set against what landed, for the entry that builds it to restate: every start appends `wheel{holder:"driver", cause:"start"}`, so `wheel:"human"` does not hold after step 3, and the replay writes no file, so no snapshot is rebuilt from it.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:99) The Path 4 suites must keep passing on all three CI OSes through the edited `answer` and `hook.dialog` pre-frame checks against a genuine wrapper, and the MCP `answer` result and refusal must keep equalling the CLI's `--json` document less `v`.
  > As landed (chunk 2026-10-04-dialog-answers-by-dialog-id), `tests/cli_answer.rs` over `fixtures/fake-scripts/path4.json` covers cli and ipc-internal (the wrapper channel) on all three CI OSes
  (.andromeda/test-plan-detail/e2e-test-strategy.md:99)
  > The MCP `answer` step landed at chunk 2026-10-17-mcp-server-for-drivers in `crates/viola-e2e/tests/mcp_tools.rs` `mcp_tools_path4_path5_answer_by_id_and_a_paused_send`, its result and refusal compared with the CLI's `--json` document less `v`.
  (.andromeda/test-plan.md:470)
  > - **ipc-internal (MCP stdio):** an rmcp client over a `viola mcp` child the test spawns and owns, closing its stdin and awaiting its exit, never rmcp's `TokioChildProcess`; beside it, a replay of the recorded MCP exchange.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:131) The Path 5 suites must keep passing on all three CI OSes through the edited `pause` / `release` / `send` pre-frame checks against a genuine wrapper, the MCP `send` step included.
  > and `tests/cli_wheel.rs` (`pause` / `release`, the `human-typing` refusals with detail `null` | `manual-pause`, hints without `release`, `release-from-driver` exit 20) cover tui, cli and ipc-internal (the wrapper channel) on all three CI OSes
  (.andromeda/test-plan-detail/e2e-test-strategy.md:132)
  > The MCP `send` step and its refusal checks landed at chunk 2026-10-17-mcp-server-for-drivers in `crates/viola-e2e/tests/mcp_tools.rs` `mcp_tools_path4_path5_answer_by_id_and_a_paused_send` (the refusal compared with the CLI's document less `v`; no MCP result, body or tool description names `release`).
- (.andromeda/test-plan-detail/e2e-test-strategy.md:146) The verification must run before the frame and pass for a genuine wrapper, so `release` with `from` still reaches the wrapper's -32602 / exit 20 (never an exit 21), and no hint the new exit-21 cause prints, human-mode or MCP, names `release`.
  >   - `release` gives `{wheel:"driver", budget_paused:false}`, and the next `send` exits 0.
  >   - After `pause`, `send` exits 10 with `detail:"manual-pause"`.
  >   - Driver-facing hints never suggest `viola release`. With `VIOLA_NAME=overseer` set, the human-mode `send` refusal (`detail:null` and `manual-pause`) ends in a last stderr line matching `^hint: ` that does not contain `release`. The MCP refusal text and `structured_content` do not contain `release` either. Drivers are refused `release` (step 7), so such a hint would send the driver into a -32602 / exit 20 dead end.
  >   - `release` with `from` gives exit 20, `{"error":"wrapper-fault","detail":{"code":-32602,…}}` carrying `release-from-driver`.
  (.andromeda/test-plan-detail/e2e-test-strategy.md:138)
  >   5. Run `viola release builder --json`, then `viola send builder --json`.
  >   6. Run `viola pause builder --json`, then `send`.
  >   7. Run `viola release builder --json` with env `VIOLA_NAME=overseer`.
  (.andromeda/test-plan.md:520)
  > `release` returns the wheel and `release` with `from` exits 20; a harness-injected turn flips no wheel and refuses `send` `turn-running` until its `turn-ended`; driver-facing hints never suggest `viola release`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:199) The verification added to the hook's `hook.dialog` path must keep the hook returning within the spine deadline, and `answer`'s exit 12 and MCP `refused: unverified-cli` must still come from the verified wrapper.
  >   - The `question` event is logged once. Hook stdout is empty, with exit 0 and returning within the spine deadline.
  >   - `answer` exits 12 `{"refusal":"unverified-cli","detail":null}`. MCP gives `is_error:true` with text `refused: unverified-cli`.
  (.andromeda/test-plan.md:536)
  > the `question` event is logged once with an empty hook stdout, and `answer` exits 12 `unverified-cli` (MCP alike)

## Bindings
- **exit-cause matrix ↔ integration-test-strategy**: the same-user impostor listener of the exit-21 case is bound through the server API on the endpoint read only from `snapshot.json`, never `try_overwrite` or a reused name (.andromeda/test-plan-detail/e2e-test-strategy.md:264) (.andromeda/test-plan.md:889)
- **exit-cause matrix ↔ quality-gates / CI per-OS legs**: the `#[cfg(not(target_os = "macos"))]` and `#[cfg(target_os = "macos")]` verification cases are proved only on their own runners, a Linux-only run proving neither (.andromeda/test-plan-detail/e2e-test-strategy.md:264) (.andromeda/test-plan.md:873)
- **security negatives ↔ unit-test-strategy**: the cross-user peer the E2E tier cannot create stays at the peer-credential boundary unit test, the E2E negative holding only same-user identity (.andromeda/test-plan-detail/e2e-test-strategy.md:290) (.andromeda/test-plan.md:476)
- **chaos liveness readings ↔ unit-test-strategy**: the pid or start-time mismatch reading gone is a clock-free unit case of the liveness classifier, while the chaos suite asserts only the product's exit code and status field (.andromeda/test-plan-detail/e2e-test-strategy.md:317)
- **hook fail-open matrix ↔ Path 7 spine deadline ↔ quality-gates perf rows**: a verification failure fails open like a channel failure, and the passing verification stays inside the hook's spine deadline the perf rows gate on hyperfine `max` (.andromeda/test-plan-detail/e2e-test-strategy.md:244) (.andromeda/test-plan-detail/e2e-test-strategy.md:199)
- **error sanitization ↔ exit-cause matrix ↔ cross-surface parity**: the exit-21 document, the MCP `instance-unreachable` result and the hook's empty streams carry codes only, and the MCP and CLI payloads agree for the same unreachable outcome (.andromeda/test-plan-detail/e2e-test-strategy.md:251) (.andromeda/test-plan-detail/e2e-test-strategy.md:99)
- **E5 ↔ test-data-fixtures**: a bad snapshot for the held readers is planted only by altering the wrapper's own bytes, and the readers' replay is built only behind the founder's ruling (.andromeda/test-plan-detail/e2e-test-strategy.md:235) (.andromeda/test-plan.md:905)
