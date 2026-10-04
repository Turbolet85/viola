# The cfg(unix) mutants scored natively (step 8)

The audit's 13 not-measured mutants (`.andromeda/runs/2026-10-01T09-18-50-code-audit/proposals.md`, the table after
the M1 survivors), scored on the Linux host. Each run is a one-off witness, never a `[[gate]]` entry (founder rule
2026-09-28). Every run uses `TMPDIR=<repo parent>/viola-mutants-scratch`, a NOCOW btrfs dir (overseer direction;
`evidence/m3.md` §Where the copy goes). Counts come from each run's own `mutants.out/outcomes.json` and `*.txt` lists.

A whole-file run grades every mutant in the file, the Windows-only bodies included. cargo-mutants mutates source text,
so a mutation inside a `cfg(windows)` item or block compiles out on Linux and grades missed. Those are **not measured
here; owed to `:70`**, the founder-ruled Windows-runner boundary workflow (overseer disposition), and never counted as
caught.

## The 13 coordinates

| # | coordinate | mutation | predicate | grade (native) |
|---|---|---|---|---|
| 1 | `crates/viola-pty/src/lib.rs:315:9` | `HostTerminal::enter → None` | cfg(unix) | **caught** (runs 1 and 2) |
| 2 | `crates/viola-pty/src/lib.rs:317:64` | `!=` → `==` in `HostTerminal::enter` | cfg(unix) | **caught** (runs 1 and 2) |
| 3 | `crates/viola-pty/src/lib.rs:315:9` | `HostTerminal::enter → Some(Default::default())` | cfg(unix) | **unviable** (`HostTerminal` has no `Default`; runs 1 and 2) |
| 4 | `crates/viola-pty/src/lib.rs:322:71` | `==` → `!=` in `HostTerminal::enter` | cfg(unix) | **caught** (runs 1 and 2) |
| 5 | `crates/viola-pty/src/lib.rs:367:76` | `!=` → `==` in `host_size` | cfg(unix) | **caught** (runs 1 and 2) |
| 6 | `crates/viola-channel/src/client.rs:231:5` | `open_by → Ok(Default::default())` | cfg(unix) | **unviable** (no `Default` for the Unix `Stream`) |
| 7 | `crates/viola-channel/src/endpoint.rs:76:5` | `host_socket_dir → Default::default()` | cfg(unix) | **caught** |
| 8 | `crates/viola-channel/src/server.rs:75:9` | `<impl Drop for Guard>::drop → ()` | cfg(unix) | **caught** |
| 9 | `crates/viola-channel/src/server.rs:108:5` | `listen → Ok((Default::default(), Default::default()))` | cfg(unix) | **unviable** (no `Default` for the Unix `Listener` / `Guard`) |
| 10 | `src/panic_frames.rs:69:5` | `raw_frames → vec![0]` | cfg(unix) | **caught** |
| 11 | `src/panic_frames.rs:80:5` | `module_of → None` | cfg(unix) | **caught** |
| 12 | `src/panic_frames.rs:80:5` | `module_of → Some((String::new(), 1))` | cfg(unix) | **caught** |
| 13 | `src/cmd/run.rs:318:5` | `sideload_outcome → ("xyzzy", None)` | cfg(all(windows, not(target_arch = "x86_64"))) | **not measurable** (below) |

## viola-pty — `run --mutants --package viola-pty --file crates/viola-pty/src/lib.rs`

| run | start → end (UTC) | tested | caught | missed | timeout | unviable | archive |
|---|---|---|---|---|---|---|---|
| 1 | 00:16:34 → 00:19:33 | 70 | 47 | 13 | 0 | 10 | `target/run-archive/38` |
| 2 (confirming, after the kill tests) | 00:22:00 → 00:25:03 | 70 | 50 | 10 | 0 | 10 | `target/run-archive/42` |

**Run 1's survivors outside cfg(windows), killed in viola-pty's own tests:**
- **`lib.rs:26:5` `pty_backend → ""` and `→ "xyzzy"`.** Only Windows-only tests assert the backend name. The kill is
  `pty_backend_is_openpty_off_windows` (`cfg(not(windows))`), which asserts `"openpty"`. Both are caught in run 2.
- **`lib.rs:223:9` `<impl Pty for PortablePty>::close → ()`.** The existing case calls `close()` only after the child
  has exited, and on Linux the reader ends when the slave side closes anyway. The kill is `close_hangs_up_a_live_child`
  (Unix):
  - a `block`-mode child is spawned with no reader or writer clone holding the master;
  - `close()` drops the master, the kernel hangs up the terminal (SIGHUP), and the child must exit inside the rig's 7 s
    bound, below the mutants profile's 10 s kill;
  - **remove-the-guard:** with `close`'s body neutralised the case reads red ("the child outlived the master's close",
    7.05 s); restored byte-identical (insertions-only diff), it reads green.

  Caught in run 2.

**Run 2's 10 missed: not measured here; owed to `:70`.** All sit in `cfg(windows)` code Linux never compiles:
- `lib.rs:234:5` `terminate → true` and `→ false`, and `:242:46` `!=` → `==` (the `#[cfg(windows)] fn terminate`);
- `lib.rs:290:9` `HostTerminal::enter → None` and `→ Some(Default::default())`, `:304:54`, `:304:59`, `:304:95`, and
  `:309:10` (the `#[cfg(windows)] HostTerminal::enter`);
- `lib.rs:355:87` `==` → `!=` (the `#[cfg(windows)]` block of `host_size`).

## viola-channel
`run --mutants --package viola-channel --file crates/viola-channel/src/client.rs --file crates/viola-channel/src/endpoint.rs --file crates/viola-channel/src/server.rs`

| run | start → end (UTC) | tested | caught | missed | timeout | unviable | archive |
|---|---|---|---|---|---|---|---|
| 1 | 00:25:27 → 00:27:17 | 102 | 73 | 13 | 0 | 16 | `target/run-archive/43` |

- **The audit's four coordinates:**
  - `endpoint.rs:76:5` `host_socket_dir → Default::default()`: **caught**.
  - `server.rs:75:9` `Guard::drop → ()`: **caught**.
  - `client.rs:231:5` `open_by → Ok(Default::default())`: **unviable** (the Unix `Stream` has no `Default`).
  - `server.rs:108:5` `listen → Ok((Default::default(), Default::default()))`: **unviable** (neither the Unix `Listener`
    nor `Guard` has a `Default`).

  No viola-channel mutant outside cfg(windows) survived. So no equivalence argument is needed, and the Unix IPC access
  control's mutants (security-plan §Authentication & Authorization) are all caught or unviable. No kill test was
  written, so no confirming re-run is owed.
- **The 13 missed: not measured here; owed to `:70`.** All sit in `cfg(windows)` items:
  - `client.rs:163` ×7 (`#[cfg(windows)] fn retry_busy`: `→ true`, `→ false`, `&&` → `||`, `==` → `!=`, and `<` → `==`
    / `>` / `<=`);
  - `client.rs:177:5` (`#[cfg(all(windows, test))] fn open`);
  - `client.rs:182:5`, `:211:19`, `:215:12` (`#[cfg(windows)] fn open_by`);
  - `server.rs:83:5`, `:94:25` (`#[cfg(windows)] fn listen`).

## src/panic_frames.rs (root package `viola`)
`run --mutants --package viola --file src/panic_frames.rs`

| run | start → end (UTC) | tested | caught | missed | timeout | unviable | archive |
|---|---|---|---|---|---|---|---|
| 1 | 00:27:51 → 00:29:30 | 23 | 14 | 9 | 0 | 0 | `target/run-archive/44` |

- **Every Unix mutant is caught: the audit's three plus the rest of the Unix bodies.**
  - `raw_frames` (`:69:5`, `→ vec![]`, `vec![0]`, `vec![1]`);
  - `module_of` (`:80:5`, `→ None`, `Some((String::new(), 0))`, `Some((String::new(), 1))`,
    `Some(("xyzzy".into(), 0))`, `Some(("xyzzy".into(), 1))`);
  - `:82:77` `==` → `!=` (the `dladdr` result check).

  So no obs-code mutant is missed or timed out in the measurable set (obs-plan §9 Mutation row). No kill test was
  written, so the panic path's raw-frame shape is untouched, no symbolising path was added (obs-plan §7 Panic hooks),
  and no confirming re-run is owed.
- **The 9 missed: not measured here; owed to `:70`.** All sit in the `#[cfg(windows)]` bodies:
  - `raw_frames` (`:37:5` `→ vec![]`, `vec![0]`, `vec![1]`);
  - `module_of` (`:50:5` ×5, and `:55:79` `==` → `!=`).

## Summary
- **The 12 cfg(unix) coordinates, graded natively:** 9 caught, 3 unviable (#3, #6, #9: a `Default` replacement for a
  type with no `Default`, confirmed in each mutant's build log), and none missed.
- **The 13th is not measurable** and is never counted as caught.
- **Beyond the 12, every mutant in the three runs outside cfg(windows) is caught or unviable**, after the three
  viola-pty kills.
- **Not measured here, owed to `:70`:** 10 viola-pty mutants, 13 viola-channel and 9 `panic_frames`, each a mutation in
  a `cfg(windows)` item or block.

## The 13th: `src/cmd/run.rs:318:5`, not measurable
- **The mutant.** `sideload_outcome → ("xyzzy", None)` in the
  `#[cfg(all(windows, not(target_arch = "x86_64")))] fn sideload_outcome`, whose whole body is `("not-built", None)`:
  the `not-built` arm of `run.conpty_sideload`.
- **The guarded control.** The ConPTY sideload fallback. On a Windows build with no vendored ConPTY for its
  architecture, `run` reports the sideload as `not-built` and runs on the inbox ConPTY, after the System32-only DLL
  search restriction (`viola_pty::sideload::restrict_dll_search`).
- **Why no host or runner here compiles it.** The dev host is Linux. CI's only Windows runner is `windows-2025`, which
  is x86_64, so `target_arch = "x86_64"` holds there and the x86_64 `sideload_outcome` compiles instead. No host or
  runner this project has builds a non-x86_64 Windows target.
- **Never counted as caught.** It stays not measurable until a non-x86_64 Windows target is built somewhere. This
  record is in M1's exemption form (`2026-10-02-epoch-2b-cleanup/evidence/m1.md`): what the mutant does, why it cannot
  be graded, where the control's effect lives.
