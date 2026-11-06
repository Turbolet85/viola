@answer n=1 entity=dependency-security#1.1 v=no
why: the change empties only the advisories ignore list, and deny-probes 14 of 14 banned with a clean control plus cargo deny bans ok on the changed deny.toml leave the serde_json unbounded_depth ban holding
@answer n=2 entity=dependency-security#1.2 v=no
why: the change moves no licenses.exceptions entry, and cargo deny check reads licenses ok on the changed deny.toml
@answer n=3 entity=dependency-security#2.1 v=no
why: the arch bans in deny.toml and deny-sync.toml are not moved, deny-probes stay 14 of 14 banned with a clean control, and cargo deny check reads bans ok on the changed file
@answer n=4 entity=dependency-security#2.2 v=yes mode=contradict
why: the move to portable-pty =0.9.0 drops pin-utils and closes RUSTSEC-2026-0441 (F1), so advisories reads ok and the ignore list is now empty rather than keeping one entry
@answer n=5 entity=dependency-security#2.3 v=yes mode=contradict
why: the =0.8.1 pin is now =0.9.0, its Windows garbage-read rationale is retired with the Windows code, and the move-off-the-pin measurement it owes is discharged
@answer n=6 entity=dependency-security#3.1 v=no
why: fuzz/Cargo.lock and the ci.yml and nightly.yml steps it names are not reached, and the walk-up resolution of the root deny.toml that it describes is unmoved
@answer n=7 entity=dependency-security#4.1 v=no
why: the exemption names only src/cmd/hook/seam.rs, which is not among the reached files, so its panic line stays where the exemption records it
@answer n=8 entity=dependency-security#5.1 v=no
why: no workflow file, action pin or CI rule was changed, as the task's untouched items record
