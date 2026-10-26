# Remove-the-guard controls — 2026-10-26-explicit-session-links

Each new guard was neutralised in the working tree with one exact-string replacement (asserted to hit once, then
read back by `grep -n`), its cases run by `cargo nextest run --workspace --features fake-agent -E '…'`, and the
guard restored the same way and read back. The restored tree then re-read green through the gate tool
(`gate.py run … --only 2,3,8,9,10,13,15`: `entries 26 · green 7 · red 0`, firings `*.2.log` in the trail
`.andromeda/runs/2026-10-26T14-59-26-implement/gate-2026-10-26-explicit-session-links.json`). Linux dev host,
2026-10-26, base HEAD `8fc3d1df`.

## A · the instance check before any log (`src/cmd/link.rs`, `link_call`)
- Neutralised: `if check_instance(&absolute` → `if false && check_instance(&absolute` (line 98).
- Filter: `binary(cli_links) & test(/link_verbs_refuse_a_widened_instance_before_any_log/)`.
- Reading: exit 100 · `FAIL [   4.765s] (1/1) viola::cli_links link_verbs_refuse_a_widened_instance_before_any_log`
  · `panicked at tests/cli_links.rs:545:9: assertion left == right failed: link: left: Some(0) right: Some(21)` ·
  `Summary [   4.766s] 1 test run: 0 passed, 1 failed, 11 skipped`.
- Restored: line 98 reads `if check_instance(&absolute, …)`; no `if false` in the file.

## B · the wrapper's `driven` and self-link checks (`src/run/links.rs`, `LinkSet::driver_of`)
- Neutralised: `if driven != self.name || driver == driven {` → `if false {` (line 95).
- Filter: `(binary(channel_links) & test(/driven_foreign|driver_is_driven/)) | test(/links_set_method_refuses/)`.
- Reading: exit 100 · `Summary [   4.768s] 18 tests run: 14 passed, 4 failed, 3115 skipped`; the four FAIL lines:
  `links_set_method_refuses_a_bad_pair_with_invalid_params::case_16_driver_is_driven`, `…::case_15_driven_foreign`,
  `viola::channel_links channel_link_names_are_validated_by_the_wrapper::case_14_driver_is_driven`,
  `…::case_13_driven_foreign`; first panic `left: Ok(Object {"ok": Object {"since": …, "already_linked":
  Bool(false)}}) right: Err(InvalidParams)`. The fourteen name-class rows stayed green: the names are typed
  (`ViolaName::try_new`), so no neutralisation of the check leaves them buildable.
- `driver_is_driven` carries `driven` equal to the wrapper's name, so the `||`'s leading operand is false there and
  the second operand alone decides it (testing.md 2026-09-24, operator-token mutants).
- Restored: line 95 reads `if driven != self.name || driver == driven {`.

## C · the server check before the frame (`src/cmd/link.rs` / `src/cmd/unlink.rs`, `request`)
- Neutralised: `client::connect(live)?` → `viola_channel::Client::connect(&live.endpoint, "cli").map_err(CallError::from)?`
  in both files (link.rs:140, unlink.rs:21).
- Filter: `(binary(security_negatives_server_identity) & test(/link/)) | test(/(link|unlink)::tests::request_refuses/)`.
- Reading: exit 100 · `Summary [   0.017s] 8 tests run: 2 passed, 6 failed, 3125 skipped`; FAIL:
  `cmd::link::tests::request_refuses_an_unverified_server_inside_its_dispatch_span`,
  `cmd::unlink::tests::request_refuses_an_unverified_server_inside_its_dispatch_span`, and the mismatch table's
  `case_13_link_json`, `case_14_link_human`, `case_15_unlink_json`, `case_16_unlink_human`; PASS: the control rows
  `security_negative_cli_verb_reaches_the_recorded_wrapper::case_7_link`, `…::case_8_unlink`.
- Restored: link.rs:140 and unlink.rs:21 read `Ok(client::connect(live)?.request(…, params)?)`.
