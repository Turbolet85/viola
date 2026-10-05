# Capture race — red before green (plan step 3, the operator's direction at P5)

The test `cmd::hook::tests::capture_two_claims_racing_for_one_k_keep_both_payloads` (`src/cmd/hook.rs`) forces the
race open deterministically. It reads `free_k` twice before either capture claims a name, so both captures start from
the same `k`, then it files two payloads through `file_from(event, dir, bytes, start)`. It never samples timing.

Both readings ran on 2026-10-05 against base `75198e53b918` plus this chunk's working tree, with one command:
`cargo nextest run -p viola --bin viola -E 'test(=cmd::hook::tests::capture_two_claims_racing_for_one_k_keep_both_payloads)'`.

| reading | `file_from` body | result |
|---|---|---|
| RED (HEAD's naming) | `replace_private` over `<Event>.<start>.json` (the `free_k` → rename shape of `75198e5`) | exit 100; `FAIL`; `assertion left == right failed` at the `(first.k, second.k)` assertion: left `(1, 1)`, right `(1, 2)`. The second payload replaced the first under one name |
| GREEN (the exclusive claim) | `create_private_new` claims `<Event>.<k>.json`, the next `k` on `AlreadyExists`, then `replace_private` writes the bytes over its own claim | exit 0; `PASS`; both payloads kept, `PreToolUse.1.json` = `first`, `PreToolUse.2.json` = `second` |

The test now runs under gate entry 3's `cmd::hook::tests::` filter.
