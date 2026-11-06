@apply_row n=1 ref_file=fanout-a11y-plan--cascade-g2.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-a11y-plan--cascade-g2.md ref_n=2 v=applied edit=E2
@apply_row n=3 ref_file=fanout-a11y-plan--cascade-g2.md ref_n=3 v=applied edit=E3
@apply_row n=4 ref_file=fanout-a11y-plan--cascade-g2.md ref_n=4 v=applied edit=E4
@apply_row n=5 ref_file=fanout-a11y-plan--cascade-g2.md ref_n=5 v=applied edit=E5
@apply_row n=6 ref_file=fanout-a11y-plan--cascade-g2.md ref_n=6 v=applied edit=E6
@fact id=fct-6edxk4iz expect_rev=1
old: (green on the three CI runners, run 38098238731)
new: (green as measured at CI run 38098238731)

@fact id=fct-ms6n7636 expect_rev=1
old: strict-modes refusal with its hint on Unix (`verify`
new: strict-modes refusal with its hint (`verify`
old: exit 21); on Windows such a path stats `NotFound`, the check skips it as absent, and the `cli` catch site's fault line stands, with no hint; under `--json` a frame verb there prints the `internal-error` document on stdout in that line's place, stderr empty (`verify` takes no `--json`).
new: exit 21).

@fact id=fct-g3h7n6e2 expect_rev=1
old: These run on all three OS legs, except `tests/security_negatives_server_identity.rs`, gated `not(target_os = "macos")`, which runs on windows-2025 and ubuntu-latest only, a limit of its peer pid witness and not of the word: `server-verify-failed` also reaches macOS (below).
new: These run on ubuntu-latest, `tests/security_negatives_server_identity.rs` among them, ungated.
old:   - **cli, macOS:** no peer pid is read (the recorded gap, a named carve-out no route entry owns), so the peer pid cases compile out there; the server-verify refusal there means an euid mismatch alone, its decision and its real read held by `peer_euid_admitted_takes_only_the_owning_user_where_it_is_reported`, `client_server_euid_is_this_user` and `tokio_client_server_euid_is_this_user`; the socket-directory refusal's macOS witness is `tests/security_negatives_socket_dir.rs`, on `macos-latest`.
new:   - **cli, the socket-directory refusal:** its witness is `tests/security_negatives_socket_dir.rs` (below).
old: its real read on Linux and macOS;
new: its real read on Linux;

@fact id=fct-cetk7x7u expect_rev=1
old: `cfg(unix)`, so on ubuntu-latest and macos-latest only)
new: `cfg(unix)`, run on ubuntu-latest)

@fact id=fct-52gvy64u expect_rev=1
old: the latter's oracle per OS: the home-state pair on Unix, the catch-site line on Windows)
new: the latter's oracle the home-state pair)
old: Unlike the server-verify cases, these run on all three OS legs.
new: These run on ubuntu-latest, as the server-verify cases do.

@fact id=fct-trymhbcp expect_rev=1
old: it has no Windows witness: `tests/security_negatives_socket_dir.rs` is `#![cfg(unix)]`, covering `ubuntu-latest` and `macos-latest`, and off Unix the check reads nothing and passes (`check_socket_dir_off_unix_reads_nothing`).
new: `tests/security_negatives_socket_dir.rs` (`#![cfg(unix)]`) is its witness, on `ubuntu-latest`; `check_socket_dir`'s Unix body is unconditional.

@pattern id=a11y3-01
fixed: green on the three CI runners
@pattern id=a11y3-02
fixed: such a path stats `NotFound`
@pattern id=a11y3-03 ignore_case=true
fixed: these run on all three OS legs
@pattern id=a11y3-04
fixed: which runs on windows-2025 and ubuntu-latest only
@pattern id=a11y3-05
fixed: no peer pid is read
@pattern id=a11y3-06
fixed: its real read on Linux and macOS
@pattern id=a11y3-07
fixed: so on ubuntu-latest and macos-latest only
@pattern id=a11y3-08
fixed: the catch-site line on Windows
@pattern id=a11y3-09
fixed: it has no Windows witness
@pattern id=a11y3-10
fixed: off Unix the check reads nothing and passes
