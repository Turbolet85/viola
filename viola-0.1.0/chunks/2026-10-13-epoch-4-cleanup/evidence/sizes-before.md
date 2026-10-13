# Sizes before the split

Read 2026-10-13 on the untouched tree (HEAD `d9ad9f1`), `tokei 14.0.0 --files src/run/send.rs src/human.rs`,
the Rust code column:

| file | lines | code | comments | blanks |
|---|---|---|---|---|
| `src/run/send.rs` | 2238 | 2088 | 6 | 144 |
| `src/human.rs` | 1169 | 1064 | 0 | 105 |

The test-name record beside it, `split-test-names.txt`, holds 223 lines (`cargo nextest list -p viola --bin viola
--message-format oneline -E 'test(/^human::|^run::send::/)'`, normalised by the split-names gate entry's `sed -E`
and `sort`), the same count P4 read on `d9ad9f1f1d96`.
