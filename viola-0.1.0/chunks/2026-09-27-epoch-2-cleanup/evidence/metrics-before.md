# Metrics controls on the untouched tree (plan step 1)

Tree: `a0e6506` plus the uncommitted phase artifacts (no source edit yet). Tools: tokei 14.0.0, jscpd 5.0.16 (host).

## `metrics.py size` — exit 0, last line `4` (predicted 4)

```
watched: crates/viola-channel/src/server.rs 869
over 800: crates/viola-channel/src/server.rs 869
watched: crates/viola-e2e/src/harness/pre_push.rs 1335
over 800: crates/viola-e2e/src/harness/pre_push.rs 1335
watched: crates/viola-e2e/src/harness/run/mutants.rs 1029
over 800: crates/viola-e2e/src/harness/run/mutants.rs 1029
watched: crates/viola-pty/src/lib.rs 1074
over 800: crates/viola-pty/src/lib.rs 1074
files 68
4
```

## `metrics.py clones` — exit 0, last line `4` (predicted 3)

First run, with the allowed list empty: 34 pairs, all printed; 4 in scope, 30 `new`. The allowed list was then
written into the script from those 30 lines (20 distinct file-pairs). Second run:

```
in-scope: crates/viola-channel/src/lib.rs:103 <-> src/cmd/run.rs:372 13 L
in-scope: crates/viola-channel/src/server.rs:919 <-> tests/channel_endpoint.rs:385 21 L
in-scope: crates/viola-channel/src/server.rs:961 <-> tests/channel_endpoint.rs:325 6 L
in-scope: crates/viola-channel/src/server.rs:967 <-> tests/channel_endpoint.rs:330 17 L
pairs 34
4
```

- The reading is 4, not the predicted 3. The fourth pair, `server.rs:961 ↔ tests/channel_endpoint.rs:325` (6 L), sits directly
  above the `canonical_sddl` pair (`:967 ↔ :330`) and is part of the same Windows DACL helper code. The step-4 dedupe covers it.
  research.md fact 10 listed jscpd's top 10 by size, which left out this 6-line fragment.
- Deviation in the instrument: the allowed-list check reads each end through `SPLIT_PARENTS`, which maps each new submodule to its
  parent file. A split moves a clone the control saw inside a split file (for example the pty pump tests' self-pairs,
  `lib.rs:532 ↔ :713`) into the submodule, and without the map that same clone would read as `new`. The in-scope check reads the
  real paths, unmapped.

## `obs_event!` census before the splits (viola-channel + viola-pty `src/`)

```
obs_event!( $level, ObsEvent::ChannelResponse, corr = self.corr, conn = self.peer.conn(), srv_conn = self.peer.srv_conn(), method = self.method, result_class = result_class, error_code = fault.map(ProtocolError::code), duration_ms = duration_ms, )
obs_event!( INFO, ObsEvent::ChannelRequest, corr = corr, conn = peer.conn(), srv_conn = peer.srv_conn(), method = method, sender = params.sender.as_deref().and_then(version_label), v = params.v, )
obs_event!( INFO, ObsEvent::ChannelRequest, corr = id, conn = self.conn.as_str(), method = label, sender = VERSION, v = PROTOCOL_V, )
obs_event!( WARN, ObsEvent::ParseRejected, parser = "channel-frame", detail = detail, count = 1u64, )
4
```

## `obs_event!` census after the splits (steps 3–5 landed)

Same instrument (every `obs_event!( … )` invocation under `crates/viola-channel/src` and `crates/viola-pty/src`, balanced-paren
extraction, whitespace-normalised, sorted). The output is byte-identical to the before-reading (`diff` exit 0): the same 4 invocations,
the same event names and the same fields. The splits moved no logging call (all four stay in `client.rs` and `server.rs`'s kept
parent) and added none.
