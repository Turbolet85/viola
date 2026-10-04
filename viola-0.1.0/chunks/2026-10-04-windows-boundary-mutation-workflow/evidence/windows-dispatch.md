# The first `windows-mutants` dispatch (plan step 8)

- Dispatch: `gh workflow run windows-mutants.yml --ref build/viola-0.1.0` (exit 0). Run **37174673472**
  (`workflow_dispatch`, `headSha` `60c569b40ff5223b6487168194d602c8fbb1baa2` = the pushed HEAD, the pre-CI commit whose
  CI read `verdict: green · checks 15/15`, ci#37174418732).
- Waiter: `gh run watch 37174673472 --interval 60` in the background (exit 0); run `completed` / `failure`.
- Verdict row, read once and REPORT-ONLY (`ci.py conclusion --sha HEAD --name mutants`):
  `60c569b40ff5 verdict: red · checks 6/21 · first-fail +555 s mutants (viola-state) · wall 1609 s` — `failed 5`
  (every job but viola-agent-claude). Each red is dispositioned below.
- Logs read per job by `gh api --allow-escape-sequences repos/Turbolet85/viola/actions/jobs/<id>/logs` (ANSI escapes
  stripped): the harness document is the job's `{"v":1,"cmd":"run",…}` line, the outcome lines are cargo-mutants'
  stderr stream.

## The six jobs
| job (id) | wall | baseline | tested | caught | unviable | missed | timeout | `ok` | `verdict` | `scratch_bytes` |
|---|---|---|---|---|---|---|---|---|---|---|
| viola-pty (111354716344) | 10 m 38 s | 10 s build + 2 s test | 79 | 64 | 10 | 5 | 0 | false | package | 0 |
| viola-channel (111354716424) | 9 m 47 s | 29 s + 1 s | 125 | 104 | 17 | 4 | 0 | false | package | 0 |
| viola-state (111354716308) | 9 m 15 s | 40 s + 4 s | 65 | 57 | 7 | 1 | 0 | false | package | 0 |
| viola-agent-claude (111354716328) | 5 m 08 s | 27 s + 1 s | 40 | 36 | 4 | 0 | 0 | **true** | package | 0 |
| viola (111354716241) | 26 m 49 s | 88 s + 10 s | 131 | 94 | 24 | 13 | 0 | false | package | 0 |
| viola-e2e (111354716513) | 16 m 12 s | 44 s + 34 s | 68 | 59 | 7 | 2 | 0 | false | package | 0 |
| **total** | | | **508** | 414 | 69 | 25 | **0** | | | |

- The 508 tested equals P4's forecast mutant count (79 · 125 · 65 · 40 · 131 · 68), job for job.
- Every job ran under 27 min against its 120-minute `timeout-minutes`, so no job timed out. The overseer's 3.4× risk
  (the Linux `wait` slowdown) did not reach the ceiling here.
- Each document's `files` equals that matrix item's list, and its `suites[].failures` equals its MISSED outcome lines.
- Absolute-path probe on each harness document (`D:\`, `C:\`, `/home/`): **0** hits in all six. Upload probe on each
  job log (`upload-artifact` / `Uploading artifact`): **0**. Every path in the documents is repo-relative
  (`mutants.out/outcomes.json`, `target/run-archive/1`).
- `unviable ≤ caught` holds in every job.

## The 34 owed coordinates (graded from this run's outcome lines)
| coordinate | mutant | grade |
|---|---|---|
| `crates/viola-e2e/src/harness/run/mutants/scratch.rs:48:5` | `prepare → Ok(None)` | caught (4 s test) |
| `crates/viola-e2e/src/harness/run/mutants/scratch.rs:54:8` | delete `!` in `prepare` | caught |
| `crates/viola-pty/src/lib.rs:234:5` | `terminate → true` | caught |
| `crates/viola-pty/src/lib.rs:234:5` | `terminate → false` | caught |
| `crates/viola-pty/src/lib.rs:242:46` | `!=` → `==` in `terminate` | caught |
| `crates/viola-pty/src/lib.rs:290:9` | `HostTerminal::enter → None` | caught |
| `crates/viola-pty/src/lib.rs:290:9` | `HostTerminal::enter → Some(Default::default())` | unviable |
| `crates/viola-pty/src/lib.rs:304:54` | `!=` → `==` | caught |
| `crates/viola-pty/src/lib.rs:304:59` | `&&` → `\|\|` | caught |
| `crates/viola-pty/src/lib.rs:304:95` | `!=` → `==` | caught |
| `crates/viola-pty/src/lib.rs:309:10` | delete `!` | caught |
| `crates/viola-pty/src/lib.rs:355:87` | `==` → `!=` in `host_size` | caught |
| `crates/viola-channel/src/client.rs:163:5` | `retry_busy → true` | caught |
| `crates/viola-channel/src/client.rs:163:5` | `retry_busy → false` | caught |
| `crates/viola-channel/src/client.rs:163:26` | `==` → `!=` | caught |
| `crates/viola-channel/src/client.rs:163:45` | `&&` → `\|\|` | caught |
| `crates/viola-channel/src/client.rs:163:52` | `<` → `==` | caught |
| `crates/viola-channel/src/client.rs:163:52` | `<` → `>` | caught |
| `crates/viola-channel/src/client.rs:163:52` | `<` → `<=` | caught |
| `crates/viola-channel/src/client.rs:177:5` | `open → Ok(Default::default())` | unviable |
| `crates/viola-channel/src/client.rs:182:5` | `open_by → Ok(Default::default())` | unviable |
| `crates/viola-channel/src/client.rs:211:19` | `!=` → `==` in `open_by` | caught |
| `crates/viola-channel/src/client.rs:215:12` | delete `!` in `open_by` | caught |
| `crates/viola-channel/src/server.rs:83:5` | `listen → Ok((Default::default(), Default::default()))` | unviable |
| `crates/viola-channel/src/server.rs:94:25` | `==` → `!=` in `listen` | caught |
| `src/panic_frames.rs:37:5` | `raw_frames → vec![]` · `vec![0]` · `vec![1]` | caught ×3 |
| `src/panic_frames.rs:50:5` | `module_of → None` · `Some((String::new(), 0))` · `(…, 1)` · `Some(("xyzzy".into(), 0))` · `(…, 1)` | caught ×5 |
| `src/panic_frames.rs:55:79` | `==` → `!=` in `module_of` | caught |

**34 graded: 30 caught · 4 unviable · 0 missed · 0 timeout.** No owed coordinate needs a kill test, so plan step 9
does not fire. The nine `src/panic_frames.rs` grades close obs-code mutation for this boundary. None of the 34 was
counted before this run.

Unviable reasons: an unviable outcome is a mutant whose build failed. The build logs stay in the runner's
`mutants.out/log/`, and the workflow uploads nothing by design, so the build text was not read. Read from the types,
each replacement needs a `Default` its type does not provide: `HostTerminal` (`lib.rs:290:9`), interprocess's `Stream`
in `io::Result<Stream>` (`client.rs:177:5`, `:182:5`), and its `Listener` in `(Listener, Guard)` (`server.rs:83:5`).

`src/cmd/run.rs:318:5` (`sideload_outcome`, `#[cfg(all(windows, not(target_arch = "x86_64")))]`) stays **not
measurable**: its four mutants read MISSED only because the x86_64 runner compiles that body out (see below). It is
not counted caught.

## Survivors outside the 34 — Epoch 3 boundary-audit items (per C2, not killed here)
Each coordinate's cfg was read from source at `60c569b`.

**Not measurable on x86_64 (4):** `src/cmd/run.rs:318:5` ×4 — `("", None)` · `("", Some(Default::default()))` ·
`("xyzzy", None)` · `("xyzzy", Some(Default::default()))`.

**Host-excluded: `#[cfg(unix)]` bodies compiled out on the Windows runner (19).** The mutated file is the same, but
the build is not, so the tests run an unchanged binary (verification-harness.md 2026-09-25, the mirror case). The
Linux host grades these natively.
- `crates/viola-channel/src/client.rs:231:5` (unix `open_by → Ok(Default::default())`) · `endpoint.rs:76:5` (unix
  `host_socket_dir → Default::default()`) · `server.rs:75:9` (unix `Guard::drop → ()`) · `server.rs:108:5` (unix
  `listen → Ok((Default::default(), Default::default()))`);
- `crates/viola-pty/src/lib.rs:315:9` ×2 (unix `HostTerminal::enter → None` / `Some(Default::default())`) · `:317:64`
  (`!=` → `==`) · `:322:71` (`==` → `!=`) · `:367:76` (`!=` → `==` in `host_size`'s `cfg(unix)` block);
- `crates/viola-e2e/src/harness/cleanup.rs:140:9` (delete `!` in `unconnectable`'s `cfg(unix)` block);
- `src/panic_frames.rs:69:5` ×3 (unix `raw_frames`) · `:80:5` ×5 (unix `module_of`) · `:82:77` (`==` → `!=`).

**Windows-equivalent (1):** `crates/viola-state/src/fs.rs:17:5` (`restrict → Ok(())`). Its `#[cfg(not(unix))]` body is
`let _ = (path, mode); Ok(())`, so on Windows the mutant is the function itself. The Unix mode tests kill it on Linux.

**A shared-body Windows survivor (1):** `crates/viola-e2e/src/harness/cleanup.rs:106:35` (`+` → `-` in
`cleanup_one`'s `Instant::now() + KILL_DEADLINE`). The body is not cfg-gated. The Linux witness (`leak.md`, attempt 2)
graded the same mutant `caught`, and the Windows runner graded it MISSED. Its killing test reaches the deadline only
on Unix. This is the one survivor here that needs a Windows test or an equivalence argument at the audit.

## A fact for the audit
Four of the six scoped file lists hold `#[cfg(unix)]` twins beside the Windows bodies (the files carry both halves).
So these jobs read red on every dispatch until the audit classifies the host-excluded set: 19 of this run's 25
misses. The verdict row says so (`failed 5`), and C2 sends survivors to the audit rather than making the workflow a gate.
