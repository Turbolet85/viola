@fact id=fct-hhwc4gim expect_rev=1
old: `stamps.json` changes only when `viola verify` runs. The case is `tests/security_negatives_strict_modes.rs` `only_the_wrapper_writes_the_snapshot_and_only_verify_the_stamps`: both files read byte-identical after every other writer-capable entry point.
new: `stamps.json` changes only when `viola verify` runs.
old: Its two controls, on test-owned `CreateNamedPipeW` pipes read through the same client-end instrument, tell the bit apart: `security_negative_control_accept_remote_pipe_reads_the_bit_clear` (`PIPE_ACCEPT_REMOTE_CLIENTS`) reads it clear, and `security_negative_control_reject_remote_pipe_reads_the_bit_set` (`PIPE_REJECT_REMOTE_CLIENTS`) reads it set.
new: As landed: `tests/security_negatives_strict_modes.rs` `only_the_wrapper_writes_the_snapshot_and_only_verify_the_stamps` reads both files byte-identical after every other writer-capable entry point.

@fact id=fct-pq4mk5dy expect_rev=2
old: OPEN — the board's read rule stands on a delegate's answer · owner: the Epoch 7 boundary batch
new: **Read rule:** a home another user may read is refused, with or without an instance.
