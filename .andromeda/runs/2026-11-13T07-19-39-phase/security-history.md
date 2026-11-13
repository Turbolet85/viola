@history_item n=1 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the bounded-reader census, the compare read, the own-executable reads named) — created `tests/contract_bounded_readers.rs` with `BOUNDARIES`, `EXCEPTIONS` and the census `product_readers_are_each_bounded_or_named` plus a self-check reading a planted unbounded read as a hit; guards decision/bounded-readers-census. Item 1 swaps its `code_only` scanner for the shared one: the census verdicts, the planted-hit self-check and the own-executable exceptions have to read the same afterwards.

@history_item n=2 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the error bodies' census and what it does not drive) — created `tests/contract_error_bodies.rs` (`CLI_DOCUMENTS`, `MCP_RESULTS`, `CHANNEL_FAULTS`, `NOT_ERROR_BODIES`, `PLUMBING`), guards `product_error_writers_are_each_named` and `thiserror_messages_interpolate_nothing`, and the `tests/support/events.rs` error-body check. Its `blanked` scanner is one of the three that item 1 replaces. Both guards (decision/every-error-writer-named, decision/error-messages-interpolate-nothing) have to keep their verdicts. No `-32603` is driven in a scanned home (decision/internal-error-held-at-unit-layer), so a test that closes a survivor must not drive one.

@history_item n=3 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the product error enums, by census) — set the `thiserror` derive census at nine product enums, read by `tests/contract_error_bodies.rs`. Item 1's scanner swap and the file splits must not move this count. Any enum touched by the merges (item 2) or by the `check_present` removal (item 4) keeps fixed `Display` messages.

@history_item n=4 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the error-enum exceptions and the thiserror census) — re-worded the census rule: nine derives over product sources, with the test-only `crates/viola-e2e` derive outside the count. It also ratified `ServeError`, `StampsError`, `PinError`, `StampsReadError` and `Refused` as named exceptions (decision/error-enum-convention-exceptions). A shared scanner that changes which files count as product sources would move the count. `Refused` sits in `strict.rs`, which item 4 edits.

@history_item n=5 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** — added `tests/contract_os_gated_tests.rs`, the `GATES` census of OS-gated tests and files. Its scanner is the third one item 1 replaces, and its verdicts have to stay unchanged. Item 10 deletes `#[cfg(not(unix))]` constants from `crates/viola-e2e/src/harness/run.rs`, so check how the census reads those gates once they are gone.

@history_item n=6 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the IPC endpoint is a Unix socket alone; CI runs one ubuntu-latest leg) — made viola Linux-only by the founder's rulings of 2026-11-04. The Windows and macOS code, tests and workflows are recoverable at tag `multi-os-final` (`ecbe7ab3`), and `windows-mutants.yml` and the mutation scratch were retired. This is the precedent for item 10: the `#[cfg(not(unix))]` constants in `harness/run.rs` are dead code from that cut and need no new ruling.

@history_item n=7 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the ConPTY sideload and its vendored binaries retired) — removed `pin_companions`, the only product user of `strict::check_present`, and took `check_present` off the viola-state bootstrap line. That is why item 4 finds the function dead. The docs mention at `.claude/docs/services/viola-state.md:6` is a leftover of this cut.

@history_item n=8 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (`check_instance`'s set and the read rule as built; the ConPTY companions' owner/DACL check) — introduced `check_present` beside `check_home_tree` and `check_diagnostics` in `viola_state::strict`, along with the `Readable` / `Symlink` refusals. Its one caller was `pin_companions`, which runs before the held open. Item 4 deletes or wires this function, and the other strict-modes entry points (decision/strict-modes-check, decision/strict-modes-before-read) have to stay untouched.

@history_item n=9 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the home strict-modes check is owner and mode alone) — made the strict-modes check owner and mode only. The home is 0700 directories and 0600 files, the read rule is any group or other bit, and an unstattable home is refused. This is why test homes made at 0755 redden (item 6). The shared helper has to make a home the way viola does (decision/home-owner-only, decision/private-file-modes).

@history_item n=10 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the board's one rule on the home) — `viola list` now runs `check_read` on the home, so a home another user may read (0750, and so 0755) is refused with or without an instance. Ratified by 2026-11-06-version-done-check (the board's read rule confirmed). This is another reason homes made at 0755 fail, and the item 6 helper exists to avoid it.

@history_item n=11 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** (the Windows plain home refused by its inherited write right) — precedent for a test oracle failing because of the host's permissions and not the product. The fix was a viola-made-home control that cannot pass vacuously. Item 6's shared helper rests on the same idea: the product verdict stays, only the test home is made correctly.

@history_item n=12 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** (the pinned `viola` placed by `place_private_new`; the first-start race closed, a lost placement re-hashed) — built `place_private_new` (mode before any byte, `sync_all`, close the handle, `persist_noclobber`, `AlreadyExists` → `Placed::Present`, any other failure an error with no retry) and the `pin_bytes` re-hash on a lost placement. Rows: decision/pinned-copy-no-clobber-placement, decision/lost-placement-rehash, decision/pinned-binary-placed-once. These are the error-kind guards behind the viola-state `fs.rs:123` and `pin.rs:91` survivors (item 5). New tests have to hold these behaviours without changing them.

@history_item n=13 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (`viola mcp`'s stdin capped at `MAX_FRAME` per line) — built `viola_mcp::stdin::CappedLines`: the newline is counted, an over-long line is refused whole and ends the session, and a held unterminated line within the cap is released. After the cut, one WARN `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` is written. Covers the `stdin.rs:84` survivor (a buffer smaller than the pending line), `CHUNK` at `stdin.rs:15`, and the `lib.rs:95` survivor (assert the WARN line). Rows: decision/mcp-stdin-line-cap-on-reader, contract/mcp-stdin/oversize-line.

@history_item n=14 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (viola mcp's exit on a cut before initialize) — corrected the cut before `initialize`: the process exits 1 with both streams empty plus the one `parse-rejected` line (11 is only the unit child's code). Trap: `exit 11` also matches `budget-paused`, so key on `ServeError::Initialize`. Applies to the test that asserts the `lib.rs:95` WARN line.

@history_item n=15 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the `Mcp.` exchange recorded by the test-only tap) — set the ratified shape of `viola-mcp-tap`: a direct spawn of `viola mcp` with no shell, each line through `take(MAX_FRAME)`, a 0600 private capture that is never committed, the same scrub, and closed refusal codes that never carry content (decision/mcp-fixtures-via-test-only-tap). Item 5 either tests the tap or records why it is out of mutants scope. Tests must hold this shape, and any change to it is a widening the founder ratified.

@history_item n=16 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — the e2e-home keepers refuse a linked base whose filesystem has less than the 9 GiB headroom floor free (`statvfs`). The floor is a compiled constant and no env var, flag or config sets it, and the refusal names no path. This is what `free_bytes` in `harness/mod.rs` implements, and its two survivors are closed in item 5.

@history_item n=17 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — a ratified widening. On the dev host `target/e2e-home` is a link to the `/tmp/viola-e2e-home-<uid>` tmpfs. Test-side removals take only directories the test side created, and the keepers follow a linked base only to an absolute, owner-only real directory, create it 0700 and remove nothing (decision/e2e-home-link-guard, decision/test-removal-own-dirs-only). Item 8 (the quota'd `/tmp` tmpfs and the refused recursive `rm`, the temp root named in one place) must not widen these rules. Trap: with the relative-target check neutralised, a mutation run of the keeper leaves `backing/` in the copied tree.

@history_item n=18 marker=2026-10-03-mutation-scoring-completion
**2026-10-03-mutation-scoring-completion** — the Linux host's same-named NOCOW `TMPDIR` belongs to the operator and the harness never wipes it, and `pre-push` passes only HOME and PATH under `env -i`. A host value crossing that boundary is a widening. Applies when item 8 names the host's temp root: it has to stay a constant and not a host-valued env crossing.

@history_item n=19 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — the founder's ruling moved mutation testing out of CI and pre-push to the epoch-boundary audit (decision/mutation-testing-at-audit). This is the basis for item 5: survivors are closed by tests, and this chunk produces no mutation result.

@history_item n=20 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the tests holding the directory and peer checks, and no seam to drive them) — bans any env, flag or feature seam that forces a refusing peer-euid decision; the decisions are proven over injected values (decision/no-seam-forcing-peer-refusal, decision/no-seam-for-foreign-peer). Remove-the-guard runs read the wiring once each and are not committed. Applies to item 7's remove-the-guard helper and item 5's tests: neither may add such a seam.

@history_item n=21 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the tests holding the link verbs' controls; the instance check before any log) — the remove-the-guard control precedent: moving the check after `client::start` read 1 of 1 red, and `link_verbs_refuse_a_widened_instance_before_any_log` reads the mode after the refusal. Trap: opening a role file narrows the home to 0700, so a check placed after it passes. Item 7 makes this per-chunk control a shared helper.

@history_item n=22 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** (test seam `FAKE_AGENT_HOOK_PANIC` and G2's exact-path exemption) — G2's one exemption is a panic line whose location is exactly `src/cmd/hook/seam.rs:<digits>`, compared as a whole string. The seam is fake-agent only (decision/g2-single-panic-exemption, decision/fake-agent-hook-panic-seam). Splitting `src/bin/viola-fake-agent.rs` (item 3) or merging inside `src/cmd/hook.rs` (item 2) must not move the seam's file. Any further exemption needs its own Decisions Log entry.

@history_item n=23 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** — the `FAKE_AGENT_PUMP_DELAY_MS` carve-out: fake-agent only, capped at 5 000, compiled out of release builds. Any other seam or env read needs its own Decisions Log entry (decision/fake-agent-pump-delay-seam, decision/test-seams-fake-agent-only, decision/new-env-read-needs-log-entry). Splitting `viola-fake-agent.rs` keeps the seams as they are, and the item 6–8 helpers add no new env read.

@history_item n=24 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** — recorded that `scripts/release-check.sh` refuses any executable other than `viola` and any `test-support` / `fake-agent` artifact (`--probe` 5/5), with `test-support` enabled only by root `[dev-dependencies]` (decision/release-build-without-test-features, decision/release-build-no-test-binaries). New test-support helpers, the split fake-agent modules and any tap tests have to stay outside the release graph.

@history_item n=25 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** (the first-start race closed) — added the threat vector: an exec of a copy still held open for write fails with `ETXTBSY`. The fix is the closed-handle placement, measured 4/24 → 0/24 two-instance boot failures. Applies to the `pin.rs` survivor tests: placement order is a security property and tests must not reorder it.

@history_item n=26 marker=2026-09-24-observability-gates
**2026-09-24-observability-gates** — CI runs the artifact canary scan (`viola-harness secret-scan`) before every test-home upload, against the NEVER-log floor, and never prints matched bytes (decision/artifact-canary-scan, decision/canary-scan-never-echoes-matches). Item 9 edits the class set of this scan in `crates/viola-e2e/src/harness/secret_scan.rs`.

@history_item n=27 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — narrowed the scan classes to the twelve `claude-stripped` needles, the case-insensitive `cookie:` header, the content canary and a non-0600 diagnostics file, and removed `?t=`, `viola_<port>=` and the GUI token. That narrowing was ruled not a widening because the removed forms match only GUI output that no path writes. This is the direct precedent for item 9, which removes the `cookie:` class (contract/artifact-canary-scan-classes/cookie-header). Every other class must still be caught.

@history_item n=28 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — the GUI left 0.1.0, but the `cookie:` / `viola_<port>=` scan forms were kept on purpose "because `secret_scan.rs` still tests them". This explains why the cookie class survived until now and why item 9 removes it together with its tests.

@history_item n=29 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** (the Epoch 5 entries' provisional records, superseded) — the founder's batch left "the axum ban and the GUI scan forms of 2026-10-16-remove-the-browser-leg" as a separate founder item. Item 9 removes the last GUI-era scan form, the cookie class (`v2-06`), which is that item's subject. Check that `v2-06` carries the founder's word. The master's row change is the wrap's amendment, not this phase's.

@history_item n=30 marker=2026-09-26-ci-chunk-base-and-union-verdict
**2026-09-26-ci-chunk-base-and-union-verdict** — `chunk.diff` is skipped by exact path and never uploaded, because it is repository source text that trips the scan's own needles (decision/ci-artifact-canary-scan). Item 9's edits to `secret_scan.rs` touch the scan's own source, so the skip must stay.

@history_item n=31 marker=2026-10-10-session-links
**2026-10-10-session-links** — the link-set restore (`LinkSet::restore` in `src/run/links.rs`) uses `replay::apply_link_line`. That rule takes a line's driver and driven only when both are strings and does not parse them again, and it appends and logs nothing (decision/link-restore-trusts-written-names). Item 2's merge of `src/run/links.rs:152` with `src/run/snapshot.rs:58` must keep this behaviour.

@history_item n=32 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the link lines the served methods write, and the names they keep out; the served `link` / `unlink` methods) — `LinkSet::record` / `remove` append `link` / `unlink` lines carrying only `{driver, driven}`. No name goes into the process log, and neither method reads `from` (decision/link-names-kept-out-of-process-log, contract/wrapper-channel-methods/link, contract/wrapper-channel-methods/unlink). This must hold through the `links.rs` / `snapshot.rs` merge.

@history_item n=33 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (`viola hook`'s event and dialog paths check the instance before obs init; the snapshot readers move onto `read_snapshot_or_replay`) — in `src/cmd/hook.rs`, `canonical` then `trusted` (`check_instance`) are shared by the event, dialog and statusline paths and run before the diagnostics level and obs init. Hook `connect` makes one read for the event and dialog paths, and a replayed snapshot fails open (decision/hook-instance-check-first, decision/hook-strict-check-first). Item 2's merge of the duplicated pair in `hook.rs` must keep that order and the fail-open shapes.

@history_item n=34 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the client's peer euid check and the order at a seam; a refused socket directory at a hook) — fixed the order on every client seam, `cmd::hook::connect` included: socket-directory check, connect, peer euid decision, then server verification. A hook on a refused directory writes one WARN `hook-decision` with `socket-dir-refused` (decision/client-seam-check-order, contract/socket-dir-refusal/hook). If the duplicated pair item 2 merges in `hook.rs` sits on the connect path, the merged function must keep this order and output.

@history_item n=35 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (Input Validation; Data Protection) — the previous chunk added decision/input-box-readiness-from-literals and changed decision/screen-fixture-signature-rows-only (box rule and prompt rows kept as literals alone, any other box row refused). Both are provisional, confirmed by a delegate. `tests/cli_verify.rs` and the ledger's `test_cases.rs` hold their witnesses, so the item 3 split has to keep those test bodies byte-identical under the same names.
