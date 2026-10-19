# Code Audit — viola · Epoch 5 — Driver surface · 2026-10-19T22:54:35Z
mode trend · HEAD b6ddc19856 · baseline dffad3e02d (Epoch 4 — Session state & governance) · span 1
overshoot: this run 0 commits (at the boundary: HEAD is the flip of 2026-10-19-overseer-switch-to-viola); the baseline's 0 commits
trend-breaks: none — every tool's leading version token equals the baseline's (jscpd 5.0.16, tokei 14.0.0, rust-code-analysis 0.0.25, cargo-machete 0.9.2, cargo-mutants 27.1.0, cargo-llvm-cov 0.9.1, cargo-nextest 0.9.146, rustc 1.98.1, code-graph.py f22becbb)
ledger: 6 parseable records after this run's append

## Proposals

### M1 — monotonic · sizes.file_max — +1023 lines
**Movement:** 2678 → 2709 → 3732 (Epoch 3 → Epoch 4 → Epoch 5); worse at both diffs.
**Evidence:** the maximum is `crates/viola-agent-claude/src/ledger.rs` at all three boundaries (tokei code lines). Raw lines 3069 → 4230 (+1161) since the baseline; its `#[cfg(test)]` module starts at line 1205 at the baseline and line 1514 at HEAD, so the module above it grew 1204 → 1513 raw lines (+309) and the test module 1865 → 2717 (+852). `git log --numstat` over the window: +1207 / −46 across 6 commits (c-churn: 6 commits for the file). Its max per-function cognitive is 12 (under the ceiling of 15).
**Suspected shape:** three quarters of the growth is the in-file test module: each capability-ledger row this epoch added (the resume, agents-join and compact rows among them) brought its probe tests into the same file.
**Proposal:** move the `#[cfg(test)]` module of `ledger.rs` into its own file (`ledger/tests.rs`), or split the row definitions by row family, so the file stops being the size maximum by test mass.

### M2 — monotonic · sizes.over_800 — +3 files
**Movement:** 12 → 19 → 22 (Epoch 3 → Epoch 4 → Epoch 5); worse at both diffs.
**Evidence:** the 22 files over 800 code lines at HEAD (tokei code lines; `—` = under 800 or absent at the baseline):

| file | Epoch 4 lines | Epoch 5 lines |
|---|---|---|
| `crates/viola-agent-claude/src/ledger.rs` | 2709 | 3732 |
| `src/bin/viola-fake-agent.rs` | 1262 | 1797 |
| `tests/cli_verify.rs` | 1028 | 1470 |
| `src/run/dialog.rs` | 1065 | 1399 |
| `src/cmd/run.rs` | 1365 | 1379 |
| `src/cmd/verify/typed.rs` | — (764) | 1199 |
| `src/cmd/hook.rs` | 1046 | 1172 |
| `crates/viola-pty/src/lib.rs` | 1148 | 1149 |
| `crates/viola-e2e/src/harness/run.rs` | 949 | 1092 |
| `crates/viola-e2e/src/harness/run/mutants.rs` | 1032 | 1032 |
| `tests/cli_send.rs` | 938 | 1004 |
| `tests/cli_fake_agent.rs` | 924 | 966 |
| `src/run/wheel.rs` | 919 | 957 |
| `src/human/board.rs` | — (new) | 949 |
| `crates/viola-agent-claude/src/hook.rs` | 849 | 916 |
| `crates/viola-channel/src/server.rs` | 865 | 865 |
| `crates/viola-e2e/src/harness/boot.rs` | — (753) | 854 |
| `crates/viola-state/src/events.rs` | 845 | 850 |
| `src/run/budget.rs` | 849 | 850 |
| `crates/viola-state/src/board.rs` | — (611) | 840 |
| `tests/support/home.rs` | — (670) | 814 |
| `tests/cli_instance_state.rs` | 859 | 814 |

Entrants (5): `src/cmd/verify/typed.rs` 764 → 1199; `src/human/board.rs` new → 949; `crates/viola-e2e/src/harness/boot.rs` 753 → 854; `crates/viola-state/src/board.rs` 611 → 840; `tests/support/home.rs` 670 → 814.
Left (2): `src/human.rs` (now 1064); `src/run/send.rs` (now 2088) — net +3.
**Suspected shape:** the entrants are the epoch's own surfaces — the typed verify probes (`src/cmd/verify/typed.rs`), the board read model and its human view (`crates/viola-state/src/board.rs`, `src/human/board.rs`), the harness boot and the test home helper; `src/run/send.rs` left by being split into `src/run/send/*`.
**Proposal:** the same split that took `src/run/send.rs` under 800 (a module directory by concern) for `src/cmd/verify/typed.rs` (1199) and `src/human/board.rs` (949, new this epoch) first: they are the two largest entrants, and `typed.rs` is the second hotspot (90).

No other check fired: no new cycle, no duplication-up, no complexity-creep, no dead-growth, no coverage-drop, no mutation-drop at equal scope.

## Informational

- **Mutation, per unit** (`caught/(caught+missed)`, jobs 2, the nextest `mutants` profile; each score against its last scored value at the same scope — none dropped ≥ 10 pts):

| unit | score | state · scope | last scored · delta | caught / missed / not measured / timeout / unviable |
|---|---|---|---|---|
| viola | — | exited-incomplete | baseline-test-failure (see below) | — |
| viola-core | 100.0 | unit · 55 mutants | 100.0 (Epoch 4, unit) · +0.0 | 49 / 0 / 0 / 0 / 6 |
| viola-pty | 100.0 | unit · 115 mutants | 98.51 (Epoch 4, unit) · +1.49 | 67 / 0 / 25 / 0 / 23 |
| viola-channel | 100.0 | unit · 200 mutants | 100.0 (Epoch 3, unit) · +0.0 | 148 / 0 / 33 / 0 / 19 |
| viola-state | 99.31 | unit · 384 mutants | 96.25 (Epoch 4, unit) · +3.06 | 287 / 2 / 69 / 0 / 26 |
| viola-agent-claude | 99.48 | unit · 648 mutants | 99.57 (Epoch 4, unit) · -0.09 | 574 / 3 / 0 / 0 / 71 |
| viola-mcp | 92.68 | unit · 76 mutants | first score | 38 / 3 / 0 / 0 / 35 |
| viola-e2e | 98.91 | unit · 799 mutants | 99.73 (Epoch 4, unit) · -0.82 | 725 / 8 / 0 / 0 / 66 |

- **Survivors, complete** (16 = Σ counts.missed; the run at this boundary):

| unit | site | mutation |
|---|---|---|
| viola-agent-claude | `crates/viola-agent-claude/src/ledger.rs:143:9` | `replace LedgerRow::relied_here -> bool with true` |
| viola-agent-claude | `crates/viola-agent-claude/src/statusline.rs:118:5` | `replace probe_document -> Option<String> with Some("xyzzy".into())` |
| viola-agent-claude | `crates/viola-agent-claude/src/statusline.rs:118:5` | `replace probe_document -> Option<String> with Some(String::new())` |
| viola-e2e | `crates/viola-e2e/src/harness/run/mutants/scratch.rs:48:5` | `replace prepare -> Result<Option<(PathBuf, u64)>, String> with Ok(None)` |
| viola-e2e | `crates/viola-e2e/src/harness/run/mutants/scratch.rs:54:8` | `delete ! in prepare` |
| viola-e2e | `crates/viola-e2e/src/harness/session.rs:112:12` | `delete ! in <impl Drop for HarnessSession>::drop` |
| viola-e2e | `crates/viola-e2e/src/harness/session.rs:112:9` | `replace <impl Drop for HarnessSession>::drop with ()` |
| viola-e2e | `crates/viola-e2e/src/harness/session.rs:115:69` | `replace == with != in <impl Drop for HarnessSession>::drop` |
| viola-e2e | `crates/viola-e2e/src/harness/session.rs:121:12` | `delete ! in <impl Drop for HarnessSession>::drop` |
| viola-e2e | `crates/viola-e2e/src/harness/status.rs:71:36` | `replace + with * in read_list` |
| viola-e2e | `crates/viola-e2e/src/harness/status.rs:71:36` | `replace + with - in read_list` |
| viola-mcp | `crates/viola-mcp/src/lib.rs:66:5` | `replace serve -> Result<(), ServeError> with Ok(())` |
| viola-mcp | `crates/viola-mcp/src/tools.rs:196:9` | `replace <impl ServerHandler for Server>::get_info -> ServerConfig with Default::default()` |
| viola-mcp | `crates/viola-mcp/src/tools.rs:205:9` | `replace <impl ServerHandler for Server>::list_tools -> Result<ListToolsResult, ErrorData> with Ok(Default::default())` |
| viola-state | `crates/viola-state/src/liveness.rs:75:5` | `replace live_endpoint -> Option<String> with None` |
| viola-state | `crates/viola-state/src/liveness.rs:77:47` | `replace != with == in live_endpoint` |

Readings: `viola-state`'s 10 baseline survivors (the `fs.rs` retry loop and `board.rs`) are all killed now; its two are new code (`liveness.rs` `live_endpoint`, the Epoch 5 liveness pre-check). `viola-mcp`'s three (`serve`, `get_info`, `list_tools`) are most likely killed only by `crates/viola-e2e/tests/mcp_tools.rs`, outside the package run (an inference, not run). `viola-e2e`'s eight sit in its harness: the mutants scratch `prepare`, `HarnessSession`'s `Drop` and `read_list`'s offset.

- **Not measured on this host (x86_64-unknown-linux-gnu)** (127 = Σ counts.not_measured): missed mutants whose whole span the host's build excludes, by the pinned `cover()` reading of `rustc --print cfg` — one row each, then the union.

| site | mutation | predicate |
|---|---|---|
| `crates/viola-pty/src/lib.rs:240:5` | `replace terminate -> bool with false` | `cfg(windows)` |
| `crates/viola-pty/src/lib.rs:240:5` | `replace terminate -> bool with true` | `cfg(windows)` |
| `crates/viola-pty/src/lib.rs:248:46` | `replace != with == in terminate` | `cfg(windows)` |
| `crates/viola-pty/src/lib.rs:296:9` | `replace HostTerminal::enter -> Option<Self> with None` | `cfg(windows)` |
| `crates/viola-pty/src/lib.rs:296:9` | `replace HostTerminal::enter -> Option<Self> with Some(Default::default())` | `cfg(windows)` |
| `crates/viola-pty/src/lib.rs:310:54` | `replace != with == in HostTerminal::enter` | `cfg(windows)` |
| `crates/viola-pty/src/lib.rs:310:59` | `replace && with || in HostTerminal::enter` | `cfg(windows)` |
| `crates/viola-pty/src/lib.rs:310:95` | `replace != with == in HostTerminal::enter` | `cfg(windows)` |
| `crates/viola-pty/src/lib.rs:315:10` | `delete ! in HostTerminal::enter` | `cfg(windows)` |
| `crates/viola-pty/src/lib.rs:434:9` | `replace console::is_console -> bool with false` | `cfg(windows)` |
| `crates/viola-pty/src/lib.rs:434:9` | `replace console::is_console -> bool with true` | `cfg(windows)` |
| `crates/viola-pty/src/lib.rs:436:76` | `replace != with == in console::is_console` | `cfg(windows)` |
| `crates/viola-pty/src/lib.rs:444:13` | `replace console::<impl super::Utf16Source for Console>::read_units -> io::Result<usize> with Ok(0)` | `cfg(windows)` |
| `crates/viola-pty/src/lib.rs:444:13` | `replace console::<impl super::Utf16Source for Console>::read_units -> io::Result<usize> with Ok(1)` | `cfg(windows)` |
| `crates/viola-pty/src/lib.rs:457:19` | `replace == with != in console::<impl super::Utf16Source for Console>::read_units` | `cfg(windows)` |
| `crates/viola-pty/src/lib.rs:475:87` | `replace == with != in host_size` | `cfg(windows)` |
| `crates/viola-pty/src/sideload.rs:25:5` | `replace restrict_dll_search -> bool with false` | `cfg(windows)` |
| `crates/viola-pty/src/sideload.rs:25:5` | `replace restrict_dll_search -> bool with true` | `cfg(windows)` |
| `crates/viola-pty/src/sideload.rs:25:86` | `replace != with == in restrict_dll_search` | `cfg(windows)` |
| `crates/viola-pty/src/sideload.rs:34:5` | `replace search_restricted -> bool with false` | `cfg(windows)` |
| `crates/viola-pty/src/sideload.rs:34:5` | `replace search_restricted -> bool with true` | `cfg(windows)` |
| `crates/viola-pty/src/sideload.rs:41:5` | `replace preload -> Result<(), PtyError> with Ok(())` | `cfg(windows)` |
| `crates/viola-pty/src/sideload.rs:41:8` | `delete ! in preload` | `cfg(windows)` |
| `crates/viola-pty/src/sideload.rs:58:5` | `replace preloaded -> bool with false` | `cfg(windows)` |
| `crates/viola-pty/src/sideload.rs:58:5` | `replace preloaded -> bool with true` | `cfg(windows)` |
| `crates/viola-channel/src/client.rs:230:26` | `replace == with != in retry_busy` | `cfg(windows)` |
| `crates/viola-channel/src/client.rs:230:45` | `replace && with || in retry_busy` | `cfg(windows)` |
| `crates/viola-channel/src/client.rs:230:5` | `replace retry_busy -> bool with false` | `cfg(windows)` |
| `crates/viola-channel/src/client.rs:230:5` | `replace retry_busy -> bool with true` | `cfg(windows)` |
| `crates/viola-channel/src/client.rs:230:52` | `replace < with <= in retry_busy` | `cfg(windows)` |
| `crates/viola-channel/src/client.rs:230:52` | `replace < with == in retry_busy` | `cfg(windows)` |
| `crates/viola-channel/src/client.rs:230:52` | `replace < with > in retry_busy` | `cfg(windows)` |
| `crates/viola-channel/src/client.rs:244:5` | `replace open -> io::Result<Stream> with Ok(Default::default())` | `cfg(all(windows, test))` |
| `crates/viola-channel/src/client.rs:249:5` | `replace open_by -> io::Result<Stream> with Ok(Default::default())` | `cfg(windows)` |
| `crates/viola-channel/src/client.rs:262:5` | `replace open_handle -> io::Result<std::os::windows::io::OwnedHandle> with Ok(Default::default())` | `cfg(windows)` |
| `crates/viola-channel/src/client.rs:290:19` | `replace != with == in open_handle` | `cfg(windows)` |
| `crates/viola-channel/src/client.rs:294:12` | `delete ! in open_handle` | `cfg(windows)` |
| `crates/viola-channel/src/server.rs:110:25` | `replace == with != in listen` | `cfg(windows)` |
| `crates/viola-channel/src/server.rs:99:5` | `replace listen -> Result<(Listener, Guard), ChannelError> with Ok((Default::default(), Default::default()))` | `cfg(windows)` |
| `crates/viola-channel/src/server/win.rs:21:5` | `replace owner_only_sddl -> String with "xyzzy".into()` | `cfg(windows)` |
| `crates/viola-channel/src/server/win.rs:21:5` | `replace owner_only_sddl -> String with String::new()` | `cfg(windows)` |
| `crates/viola-channel/src/server/win.rs:25:5` | `replace owner_only_descriptor -> io::Result<SecurityDescriptor> with Ok(Default::default())` | `cfg(windows)` |
| `crates/viola-channel/src/server/win.rs:30:5` | `replace user_sid -> io::Result<String> with Ok("xyzzy".into())` | `cfg(windows)` |
| `crates/viola-channel/src/server/win.rs:30:5` | `replace user_sid -> io::Result<String> with Ok(String::new())` | `cfg(windows)` |
| `crates/viola-channel/src/server/win.rs:32:82` | `replace == with != in user_sid` | `cfg(windows)` |
| `crates/viola-channel/src/server/win.rs:42:5` | `replace token_user_sid -> io::Result<String> with Ok("xyzzy".into())` | `cfg(windows)` |
| `crates/viola-channel/src/server/win.rs:42:5` | `replace token_user_sid -> io::Result<String> with Ok(String::new())` | `cfg(windows)` |
| `crates/viola-channel/src/server/win.rs:50:15` | `replace == with != in token_user_sid` | `cfg(windows)` |
| `crates/viola-channel/src/server/win.rs:57:68` | `replace == with != in token_user_sid` | `cfg(windows)` |
| `crates/viola-channel/src/server/win.rs:77:5` | `replace from_sddl -> io::Result<SecurityDescriptor> with Ok(Default::default())` | `cfg(windows)` |
| `crates/viola-channel/src/server/win.rs:88:18` | `replace == with != in from_sddl` | `cfg(windows)` |
| `crates/viola-channel/src/test_support.rs:116:9` | `replace win::dacl_of -> String with "xyzzy".into()` | `cfg(windows)` |
| `crates/viola-channel/src/test_support.rs:116:9` | `replace win::dacl_of -> String with String::new()` | `cfg(windows)` |
| `crates/viola-channel/src/test_support.rs:66:9` | `replace win::canonical_sddl -> String with "xyzzy".into()` | `cfg(windows)` |
| `crates/viola-channel/src/test_support.rs:66:9` | `replace win::canonical_sddl -> String with String::new()` | `cfg(windows)` |
| `crates/viola-channel/src/test_support.rs:93:9` | `replace win::user_sid -> String with "xyzzy".into()` | `cfg(windows)` |
| `crates/viola-channel/src/test_support.rs:93:9` | `replace win::user_sid -> String with String::new()` | `cfg(windows)` |
| `crates/viola-channel/src/tokio_client.rs:99:5` | `replace open -> io::Result<Stream> with Ok(Default::default())` | `cfg(windows)` |
| `crates/viola-state/src/fs.rs:105:9` | `replace win::protected_sddl -> String with "xyzzy".into()` | `cfg(windows)` |
| `crates/viola-state/src/fs.rs:105:9` | `replace win::protected_sddl -> String with String::new()` | `cfg(windows)` |
| `crates/viola-state/src/fs.rs:110:9` | `replace win::profile_dir -> io::Result<PathBuf> with Ok(Default::default())` | `cfg(windows)` |
| `crates/viola-state/src/fs.rs:112:86` | `replace == with != in win::profile_dir` | `cfg(windows)` |
| `crates/viola-state/src/fs.rs:124:19` | `replace == with != in win::profile_dir` | `cfg(windows)` |
| `crates/viola-state/src/fs.rs:127:46` | `replace == with != in win::profile_dir` | `cfg(windows)` |
| `crates/viola-state/src/fs.rs:134:9` | `replace win::protect_outside_profile -> io::Result<()> with Ok(())` | `cfg(windows)` |
| `crates/viola-state/src/fs.rs:142:9` | `replace win::protect -> io::Result<()> with Ok(())` | `cfg(windows)` |
| `crates/viola-state/src/fs.rs:149:9` | `replace win::set_dacl -> io::Result<()> with Ok(())` | `cfg(windows)` |
| `crates/viola-state/src/fs.rs:160:17` | `replace == with != in win::set_dacl` | `cfg(windows)` |
| `crates/viola-state/src/fs.rs:178:23` | `replace == with != in win::set_dacl` | `cfg(windows)` |
| `crates/viola-state/src/fs.rs:194:9` | `replace win::dacl_of -> io::Result<*mut ACL> with Ok(Default::default())` | `cfg(windows)` |
| `crates/viola-state/src/fs.rs:197:94` | `replace == with != in win::dacl_of` | `cfg(windows)` |
| `crates/viola-state/src/fs.rs:198:13` | `replace || with && in win::dacl_of` | `cfg(windows)` |
| `crates/viola-state/src/fs.rs:99:9` | `replace win::wide -> Vec<u16> with vec![0]` | `cfg(windows)` |
| `crates/viola-state/src/fs.rs:99:9` | `replace win::wide -> Vec<u16> with vec![1]` | `cfg(windows)` |
| `crates/viola-state/src/fs.rs:99:9` | `replace win::wide -> Vec<u16> with vec![]` | `cfg(windows)` |
| `crates/viola-state/src/pin.rs:126:14` | `replace < with <= in retry_open` | `cfg(windows)` |
| `crates/viola-state/src/pin.rs:126:14` | `replace < with == in retry_open` | `cfg(windows)` |
| `crates/viola-state/src/pin.rs:126:14` | `replace < with > in retry_open` | `cfg(windows)` |
| `crates/viola-state/src/pin.rs:126:33` | `replace && with || in retry_open` | `cfg(windows)` |
| `crates/viola-state/src/pin.rs:126:49` | `replace == with != in retry_open` | `cfg(windows)` |
| `crates/viola-state/src/pin.rs:126:5` | `replace retry_open -> bool with false` | `cfg(windows)` |
| `crates/viola-state/src/pin.rs:126:5` | `replace retry_open -> bool with true` | `cfg(windows)` |
| `crates/viola-state/src/pin.rs:131:5` | `replace open_held -> io::Result<File> with Ok(Default::default())` | `cfg(windows)` |
| `crates/viola-state/src/pin.rs:140:5` | `replace open_held_waiting -> io::Result<File> with Ok(Default::default())` | `cfg(windows)` |
| `crates/viola-state/src/pin.rs:142:18` | `replace += with *= in open_held_waiting` | `cfg(windows)` |
| `crates/viola-state/src/pin.rs:142:18` | `replace += with -= in open_held_waiting` | `cfg(windows)` |
| `crates/viola-state/src/pin.rs:144:23` | `replace match guard retry_open(e.raw_os_error(), attempts) with false in open_held_waiting` | `cfg(windows)` |
| `crates/viola-state/src/pin.rs:144:23` | `replace match guard retry_open(e.raw_os_error(), attempts) with true in open_held_waiting` | `cfg(windows)` |
| `crates/viola-state/src/pin.rs:159:5` | `replace pin_companions -> Result<HeldCompanions, PinError> with Ok(Default::default())` | `cfg(windows)` |
| `crates/viola-state/src/pin.rs:171:23` | `replace match guard e.kind() == io::ErrorKind::NotFound with false in pin_companions` | `cfg(windows)` |
| `crates/viola-state/src/pin.rs:171:23` | `replace match guard e.kind() == io::ErrorKind::NotFound with true in pin_companions` | `cfg(windows)` |
| `crates/viola-state/src/pin.rs:171:32` | `replace == with != in pin_companions` | `cfg(windows)` |
| `crates/viola-state/src/pin.rs:178:18` | `replace != with == in pin_companions` | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:189:9` | `replace win::wide -> Vec<u16> with vec![0]` | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:189:9` | `replace win::wide -> Vec<u16> with vec![1]` | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:189:9` | `replace win::wide -> Vec<u16> with vec![]` | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:193:9` | `replace win::check -> Result<(), Refused> with Ok(())` | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:200:9` | `replace win::persistent_acls -> Option<bool> with None` | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:200:9` | `replace win::persistent_acls -> Option<bool> with Some(false)` | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:200:9` | `replace win::persistent_acls -> Option<bool> with Some(true)` | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:204:81` | `replace == with != in win::persistent_acls` | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:222:15` | `replace != with == in win::persistent_acls` | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:227:9` | `replace win::owner_and_dacl -> Option<(String, Option<Vec<Allow>>)> with None` | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:227:9` | `replace win::owner_and_dacl -> Option<(String, Option<Vec<Allow>>)> with Some(("xyzzy".into(), None))` | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:227:9` | `replace win::owner_and_dacl -> Option<(String, Option<Vec<Allow>>)> with Some(("xyzzy".into(), Some(vec![Default::default()])))` | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:227:9` | `replace win::owner_and_dacl -> Option<(String, Option<Vec<Allow>>)> with Some(("xyzzy".into(), Some(vec![])))` | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:227:9` | `replace win::owner_and_dacl -> Option<(String, Option<Vec<Allow>>)> with Some((String::new(), None))` | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:227:9` | `replace win::owner_and_dacl -> Option<(String, Option<Vec<Allow>>)> with Some((String::new(), Some(vec![Default::default()])))` | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:227:9` | `replace win::owner_and_dacl -> Option<(String, Option<Vec<Allow>>)> with Some((String::new(), Some(vec![])))` | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:245:19` | `replace != with == in win::owner_and_dacl` | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:252:13` | `delete match arm Some((owner, Some(aces))) in win::owner_and_dacl` | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:259:9` | `replace win::aces -> Option<Option<Vec<Allow>>> with None` | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:259:9` | `replace win::aces -> Option<Option<Vec<Allow>>> with Some(None)` | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:259:9` | `replace win::aces -> Option<Option<Vec<Allow>>> with Some(Some(vec![Default::default()]))` | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:259:9` | `replace win::aces -> Option<Option<Vec<Allow>>> with Some(Some(vec![]))` | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:268:53` | `replace == with != in win::aces` | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:273:31` | `replace != with == in win::aces` | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:290:9` | `replace win::sid_string -> Option<String> with None` | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:290:9` | `replace win::sid_string -> Option<String> with Some("xyzzy".into())` | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:290:9` | `replace win::sid_string -> Option<String> with Some(String::new())` | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:293:62` | `replace == with != in win::sid_string` | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:297:65` | `replace != with == in win::sid_string` | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:313:9` | `replace win::user_sid -> Option<String> with None` | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:313:9` | `replace win::user_sid -> Option<String> with Some("xyzzy".into())` | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:313:9` | `replace win::user_sid -> Option<String> with Some(String::new())` | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:315:86` | `replace == with != in win::user_sid` | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:327:27` | `replace != with == in win::user_sid` | `cfg(windows)` |

Union: 127 rows under 2 predicates (`cfg(all(windows, test))`, `cfg(windows)`) — their grading is the Windows / macOS legs' (`windows-mutants.yml`).

- **Root unit `viola` — not scored, `baseline-test-failure`.** The founder's scope was CARRY 2's two files (`src/bin/viola-fake-agent.rs`, `src/cmd/verify.rs`: 265 of 1543 listed). Two attempts, both cargo-mutants exit 4 at the UNMUTATED baseline under the `mutants` profile's 10 s kill: 21:50Z on a host at load ~67 (another project's sweep: TIMEOUTs across `cli_revive` / `cli_send` / `cli_verify`, 4 `send_window_*` "wrapper builder not ready"), and 22:51Z at load ~4: 3 TIMEOUTs at 10.00 s — `contract_fake_agent_drift fake_agent_dialog_replay_matches_every_recorded_dialog_set`, `cli_send send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins` and `..._on_a_verified_cli::case_1_hint`. In this run's green coverage pass (`ci` profile, no kill) the same tests take **10.577 s**, 9.010 s and 9.346 s: the first cannot pass a 10 s kill on this host at all, the two within a second of it. The `cli_revive` tests take 5.2–6.2 s each (the Epoch 5 preflight's bounded agents read). So the root package's baseline does not fit the profile at HEAD — structural, not load. The last scored root value stays E4's 97.03 (unit scope); no comparison is made. A change-direction for the founder: a `mutants`-profile override for these tests (as `verify_window_` / `send_window_` have), or a faster replay — before CARRY 2 can be graded by a tool run.
- **CARRY 2** (`viola-0.1.0/chunks/2026-10-13-paste-newline-ledger-row/evidence/carry2-kills.md`, its four sites c1–c4) — **still owed**: the root unit above never reached a mutant. Its kills stand on that chunk's hand-applied mutants only.
- **CARRY 3 — graded: all three `viola-state` retry-loop mutants CAUGHT on windows-2025** (dispatch [39326811932](https://github.com/Turbolet85/viola/actions/runs/39326811932), HEAD `b6ddc19856`): `crates/viola-state/src/fs.rs:295:18` `replace += with *= in replace_private_retrying` — caught after a 11 s test; `crates/viola-state/src/fs.rs:298:23` `replace match guard attempts < REPLACE_ATTEMPTS && retry(e.error.raw_os_error()) with true in replace_private_retrying` — caught after a 11 s test; `crates/viola-state/src/fs.rs:298:51` `replace && with || in replace_private_retrying` — caught after a 11 s test. Each test run ends at 11 s with a failing test, not at the kill (a timeout reads `TIMEOUT`). The job's 7 misses (`fs.rs:19` `restrict`, `strict.rs` `unix::reading` ×4 and `unix::euid` ×2) all sit under `#[cfg(unix)]` — not measurable on Windows.
- **The `windows-mutants.yml` dispatch after `c6e1301`** (the root package's Windows baseline) (the founder's "Dispatch now"; run 39326811932, failure): 4 of 10 jobs green.

| job | conclusion | reading |
|---|---|---|
| mutants (viola-panic-frames) | failure | baseline FAILED: viola::contract_fake_agent_drift fake_agent_dialog_replay_matches_every_recorded_dialog_set |
| mutants (viola-cmd-run) | success | 64 mutants tested in 63m: 1 missed, 49 caught, 14 unviable |
| mutants (viola-main) | failure | baseline FAILED: viola::contract_fake_agent_drift fake_agent_dialog_replay_matches_every_recorded_dialog_set |
| mutants (viola-state) | success | 159 mutants tested in 16m: 7 missed, 141 caught, 11 unviable |
| mutants (viola-agent-claude) | success | 47 mutants tested in 6m: 43 caught, 4 unviable |
| mutants (viola-channel) | success | 169 mutants tested in 9m: 5 missed, 144 caught, 20 unviable |
| mutants (viola-pty) | failure | 89 mutants tested in 4m: 11 caught, 78 unviable — harness verdict `unviable-exceeds-caught` |
| mutants (viola-cmd-verify-stop) | failure | baseline FAILED: viola::cli_budget path6_the_boundaries_and_the_window_word |
| mutants (viola-e2e) | failure | baseline FAILED: viola-e2e harness::run::mutants::scratch::tests::wipe_scratch_takes_an_instance_tree_the_check_accepts_where_a_plain_dir_does_not |
| mutants (viola-run-env) | failure | baseline FAILED: viola::contract_fake_agent_drift fake_agent_dialog_replay_matches_every_recorded_dialog_set |

The three root-package jobs that time out at the baseline time out on the same test as this host's second root attempt (`fake_agent_dialog_replay_matches_every_recorded_dialog_set`): one cause on both OSes. `viola-e2e`'s Windows baseline fails `wipe_scratch_takes_an_instance_tree_the_check_accepts_where_a_plain_dir_does_not` (left `Ok(())`, right `Err(Writable)`) — a Windows-only assertion; `viola-cmd-verify-stop`'s on `cli_budget path6_the_boundaries_and_the_window_word` at 10 s.

- **`verify_window_without_screens_fails_every_interactive_row` against the 45 s kill:** 42.952 s alone on an unloaded host under the `mutants` profile (a one-test nextest run, this audit), and 43.635 s inside the root unit's second baseline — 2.0 s and 1.4 s of headroom.
- **The Epoch 5 chunks' code** (end-to-end, machine-contract, output-discipline, mcp, plugin, compact, dialog-hold, the switch): graded at unit scope by every library unit above (viola-core, -pty, -channel, -state, -agent-claude, -mcp) and viola-e2e; the part in the root package (`src/`) is not graded this boundary (the root unit's baseline above).
- **First score `viola-mcp`: 92.68** (76 mutants, 35 unviable); `viola-channel` 100 against its last scored value, E3's 100 (it was not run at E4).
- **Top-N entrants — sizes.top (10):** `src/cmd/verify/typed.rs` 1199; `crates/viola-e2e/src/harness/run.rs` 1092.
- **Top-N entrants — complexity.top (10):** `main` `src/bin/viola-fake-agent.rs:1102` 17; `role_of` `src/main.rs:94` 17; `board_line` `src/human/board.rs:156` 16; `measure` `src/cmd/verify.rs:125` 14.
- **Top-N entrants — duplication.top (10): 9** — `tests/cli_verify.rs:77` ↔ `tests/contract_ledger_probes.rs:14` 52 L; `crates/viola-e2e/tests/mcp_tools.rs:156` ↔ `crates/viola-e2e/tests/path6_budget_harness.rs:46` 29 L; `crates/viola-e2e/src/harness/run.rs:323` ↔ `tests/cli_verify.rs:79` 27 L; `crates/viola-agent-claude/src/ledger.rs:1524` ↔ `crates/viola-e2e/src/harness/run.rs:323` 26 L; `tests/cli_verify.rs:508` ↔ `tests/cli_verify.rs:1071` 24 L; `crates/viola-e2e/src/harness/run.rs:350` ↔ `tests/cli_verify.rs:106` 23 L; `src/run/links.rs:120` ↔ `src/run/snapshot.rs:58` 22 L; `crates/viola-e2e/src/harness/mod.rs:368` ↔ `tests/cli_instance_state.rs:601` 20 L; `src/run/dialog.rs:542` ↔ `src/run/wheel.rs:816` 20 L.
- **Top-N entrants — hotspots (10):** `src/cmd/verify/typed.rs` 90; `src/cmd/verify.rs` 84; `crates/viola-e2e/src/harness/run.rs` 77; `tests/contract_fixture_hygiene.rs` 75; `crates/viola-agent-claude/src/ledger.rs` 72; `tests/contract_fake_agent_drift.rs` 70; `tests/support/home.rs` 63; `src/main.rs` 51.
- **Top-N entrants — graph.fan_in_top (20):** `rust-analyzer cargo viola 0.1.0 support/` 95; `rust-analyzer cargo viola 0.1.0 support/home/impl#[TestHome]new().` 91.
- **Unused dependency, new:** `viola-mcp` → `serde` (cargo-machete; `serde` derives may arrive through `schemars` / `rmcp` re-exports — a candidate, not a verdict). `fuzz` → `arbitrary` stands from E4.
- **Coverage run:** 2958/2958 tests passed in 66.6 s; no `.profraw` refused or truncated — this host's reading for the open `pre-push` WATCH on "Newer claude CLI rows" (`evidence/profraw-red.md`), one clean run.

## Below threshold — no action

- duplication-up: `duplication.pct` 3.12 → 3.54 (+0.42 pt, +13.5 %) — under the +0.5 pt AND +15 % floor.
- count-under-ratio: not met — `clones` 256 → 340 and `duplicated_lines` 2132 → 2917 rose with `pct`; population `total_lines` 68257 → 82429 (+20.8 %); split src 153 · test 166 · mixed 21 pairs.
- complexity-creep: `over_ceiling` 2 → 4 (+2) — under +3; the four over 15: `main` `src/bin/viola-fake-agent.rs:1102` 17; `role_of` `src/main.rs:94` 17; `board_line` `src/human/board.rs:156` 16; `send` `src/run/send.rs:73` 16.
- dead-growth: `zero_ref_candidates` 0 → 0 (13 residuals hand-read, all false positives — c-dead.json).
- coverage-drop: line 98.45 → 98.63.
- new-cycle: `graph.cycles` 0 → 0.
- monotonic `duplication.pct`: 3.13 → 3.12 → 3.54 (Epoch 3 → Epoch 4 → Epoch 5) — not worse at both diffs.
- monotonic `complexity.over_ceiling`: 4 → 2 → 4 (Epoch 3 → Epoch 4 → Epoch 5) — not worse at both diffs.
- monotonic `dead.zero_ref_candidates`: 0 → 0 → 0 (Epoch 3 → Epoch 4 → Epoch 5) — not worse at both diffs.
- monotonic `coverage.line`: 98.12 → 98.45 → 98.63 (Epoch 3 → Epoch 4 → Epoch 5) — not worse at both diffs.
- mutation `viola-core`: 100.0 → 100.0 (unit, Epoch 4 → Epoch 5).
- mutation `viola-pty`: 98.51 → 100.0 (unit, Epoch 4 → Epoch 5).
- mutation `viola-channel`: 100.0 → 100.0 (unit, Epoch 3 → Epoch 5).
- mutation `viola-state`: 96.25 → 99.31 (unit, Epoch 4 → Epoch 5).
- mutation `viola-agent-claude`: 99.57 → 99.48 (unit, Epoch 4 → Epoch 5).
- mutation `viola-e2e`: 99.73 → 98.91 (unit, Epoch 4 → Epoch 5).
- sizes: p50 292 → 277, p90 865 → 850; totals 58997 → 71204 loc, 147 → 179 files (populations, never movements).
- complexity percentiles: cyclomatic p50/p90 1.0/4.0 → 1.0/4.0, cognitive 0.0/2.0 → 0.0/2.0.
- churn (no threshold): 34.92 → 41.93 %, files churned 40 → 56.
- graph (no threshold): cross-unit edges 11 → 17; fan-out {'viola': 5, 'viola-e2e': 3, 'viola-state': 1, 'viola-channel': 1, 'viola-agent-claude': 1} → {'viola': 6, 'viola-mcp': 4, 'viola-e2e': 4, 'viola-agent-claude': 1, 'viola-channel': 1, 'viola-state': 1}.

## Skips

- `coverage.branch` — declined: the project coverage command does not instrument branches (as at the two prior records)
- `mutation:viola` — baseline-test-failure: files scope (CARRY 2's two files, 265 mutants listed): two attempts, both exit 4 at the unmutated baseline under the nextest mutants profile's 10 s kill. 21:50Z (host load avg ~67, another project's sweep): TIMEOUTs in cli_revive / cli_send / cli_verify and 4 send_window_* 'wrapper builder not ready'. 22:51Z (load avg ~4): 3 TIMEOUTs at 10.00 s, contract_fake_agent_drift fake_agent_dialog_replay_matches_every_recorded_dialog_set (10.577 s PASS unkilled in this run's coverage pass) and cli_send send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins / ..._on_a_verified_cli::case_1_hint (9.010 s / 9.346 s unkilled). CARRY 2's grading stays owed.
- `mutation:viola (outside src/bin/viola-fake-agent.rs and src/cmd/verify.rs: 1278 of 1543 listed)` — declined: the founder's word at the attended confirm: the root unit scoped to CARRY 2's two files
