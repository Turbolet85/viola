@leaf_group group=masters
@leaf_read leaf=CLAUDE.md lines=131
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=7 class=stale
leaf-text: a standalone, cross-platform (Windows first) bridge
source: fct-igbvtfv6 «**Linux only.** viola targets Linux: the core on any current distribution, built and tested on every commit in CI on `ubuntu-latest` against the fake agent»
replace-with: a standalone, Linux-only bridge
@leaf_edit n=2 leaf=CLAUDE.md line=21 class=retired
leaf-text: `host_stdin` (Windows: viola's own console reader, `^Z` kept); knows no agent; Windows `sideload` (System32 DLL-search restriction + absolute-path pre-load of the vendored `conpty.dll`)
source: fct-irhgihut «host_stdin(): `std::io::stdin()`;»
replace-with: `host_stdin` (`std::io::stdin()`); knows no agent
@leaf_edit n=3 leaf=CLAUDE.md line=22 class=retired
leaf-text: `TokioClient::server_pid`, none on macOS where `SERVER_PID_REPORTED` is false)
source: fct-rrvv5rrr «`viola_channel::Client::server_pid` and `tokio_client::TokioClient::server_pid` return the connected stream's peer pid through interprocess's `peer_creds().pid()` (`SO_PEERCRED`)»
replace-with: `TokioClient::server_pid`)
@leaf_edit n=4 leaf=CLAUDE.md line=22 class=retired
leaf-text: `TokioClient::server_euid`, none on Windows)
source: fct-zpaxdj2d «`Client::server_euid` / `TokioClient::server_euid` read it (`peer_creds().ok()?.euid()`), writing and logging nothing.»
replace-with: `TokioClient::server_euid`)
@leaf_edit n=5 leaf=CLAUDE.md line=22 class=retired
leaf-text: `peer_admitted`, `PEER_EUID_REPORTED`)
source: fct-zpaxdj2d «`viola-channel` `client.rs` (`peer_euid_of`, `peer_euid_admitted`, `peer_admitted`, `Client::server_euid`)»
replace-with: `peer_admitted`)
@leaf_edit n=6 leaf=CLAUDE.md line=23 class=retired
leaf-text: `verify_server`; macOS the recorded gap)
source: fct-irhgihut «the server identity check (`ServerVerdict`, `server_verdict`, `verify_server`: the recorded pid and its start time against the connected server's»
replace-with: `verify_server`)
@leaf_edit n=7 leaf=CLAUDE.md line=24 class=retired
leaf-text: `WRAPPED_HERE` (Unix), the `/bin/sh -c` argv
source: fct-sfkaxxdn «`reading`, `user_command`, `override_document`, `shell_argv`, `SETTINGS_FLAG`, `USER_SETTINGS`, `BIN_SH`, `SHELL_FLAG`, `probe_document`»
replace-with: the `/bin/sh -c` argv
@leaf_edit n=8 leaf=CLAUDE.md line=27 class=retired
leaf-text: Unix: then the user's command through `/bin/sh -c`
source: fct-horlp4ay «runs the user's own statusline command, read at start from one named source, through `/bin/sh -c` with the same stdin (the only shell-out in viola)»
replace-with: then the user's command through `/bin/sh -c`
@leaf_edit n=9 leaf=CLAUDE.md line=28 class=retired
leaf-text: `viola-harness` behind `scripts/agent-run.*`
source: fct-yqn2bvnu «`scripts/agent-run.sh` (POSIX) is the one thin shim.»
replace-with: `viola-harness` behind `scripts/agent-run.sh`
@leaf_edit n=10 leaf=CLAUDE.md line=42 class=retired
leaf-text: the relied set (`LedgerRow::relied()`: all twenty-four on Unix; on Windows, where the status line is not wrapped, twenty-one, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`)
source: fct-qwpklckh «The relied set is `LedgerRow::relied()`: every row of `ALL`, in its order, all twenty-four.»
replace-with: the relied set (`LedgerRow::relied()`: all twenty-four on every build)
@leaf_edit n=11 leaf=CLAUDE.md line=42 class=retired
leaf-text: stamped on 2.1.287 on the Linux dev host only (its Windows reading owed on "Windows-only live measurements")
source: fct-wfge3rsz «`paste-newline`, probed by Run B's newline paste and stamped on 2.1.287 on the Linux dev host only»
replace-with: stamped on 2.1.287 on the Linux dev host only
@leaf_edit n=12 leaf=CLAUDE.md line=42 class=retired
leaf-text: stamped on 2.1.287 on the Linux dev host and relied on Unix only (their Windows legs owed on "Windows-only live measurements")
source: fct-ehpssyzz «relied on every build (`LedgerRow::relied()`: all twenty-four rows)»
replace-with: stamped on 2.1.287 on the Linux dev host and relied on every build
@leaf_edit n=13 leaf=CLAUDE.md line=42 class=retired
leaf-text: probed by verify's Run E, stamped on 2.1.287 on the Linux dev host and relied on every OS (its Windows reading owed on "Windows-only live measurements")
source: fct-wfge3rsz «`resume-session`, probed by verify's Run E ([CLI Version Compatibility]), relied and stamped live on 2.1.287 on the Linux dev host»
replace-with: probed by verify's Run E, stamped on 2.1.287 on the Linux dev host and relied on every build
@leaf_edit n=14 leaf=CLAUDE.md line=42 class=retired
leaf-text: probed by Run B's one agents read, stamped on 2.1.287 on the Linux dev host and relied on every OS (its Windows reading owed on "Windows-only live measurements")
source: fct-wfge3rsz «row `agents-join`, probed by Run B's one agents read, relied, stamped live on 2.1.287 on the Linux dev host»
replace-with: probed by Run B's one agents read, stamped on 2.1.287 on the Linux dev host and relied on every build
@leaf_edit n=15 leaf=CLAUDE.md line=42 class=retired
leaf-text: probed by Run B's sixth paste, stamped on 2.1.287 on the Linux dev host and relied on every OS (its Windows reading owed on "Windows-only live measurements")
source: fct-wfge3rsz «probed by Run B's sixth paste, relied, stamped live on 2.1.287 on the Linux dev host (`stamped 2.1.287  24 pass  0 fail`)»
replace-with: probed by Run B's sixth paste, stamped on 2.1.287 on the Linux dev host and relied on every build
@leaf_edit n=16 leaf=CLAUDE.md line=73 class=retired
leaf-text: `scripts/agent-run.sh <boot|run|status|cleanup|logs>` (`scripts/agent-run.ps1` on PowerShell)
source: fct-yqn2bvnu «`scripts/agent-run.sh` (POSIX) is the one thin shim.»
replace-with: `scripts/agent-run.sh <boot|run|status|cleanup|logs>`
@leaf_edit n=17 leaf=CLAUDE.md line=86 class=retired
leaf-text: (named pipe / per-user Unix socket)
source: fct-ao5mmzh3 «Each endpoint, a Unix socket file, must not be connectable by another OS user.»
replace-with: (a per-user Unix socket)
@leaf_edit n=18 leaf=CLAUDE.md line=86 class=stale
leaf-text: built and CI-tested on Windows, macOS and Linux from the first commit, with Windows the first target (the first live run was on the Linux dev host)
source: fct-hcu7z4ke «It is built and CI-tested on Linux (`ubuntu-latest`), and the on-disk state is parsed defensively. Rationale: the Windows and macOS support is cut, recoverable at the tag `multi-os-final`»
replace-with: built and CI-tested on Linux only (`ubuntu-latest`), the Windows and macOS code out of the tree, recoverable at the tag `multi-os-final` (the first live run was on the Linux dev host)
