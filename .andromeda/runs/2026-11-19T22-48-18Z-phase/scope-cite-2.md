old: - [inferred] the wrapper's byte source (the pty pump that reads the human's keys) and the wheel's origin decision in
new: - [premise-corrected: the origin is the hook's (`prompt_origin`, crates/viola-agent-claude/src/hook.rs:266), the wheel's two human moves are src/run/wheel.rs:308 and src/run/send/slot.rs:251, research Graph impact] the wrapper's byte source (the stdin observer `Observed`, which moves the wheel at the human's key) and the wheel's human-prompt move in
old: - [inferred] the normalised event for a submitted prompt (`viola-core`) and the turn end's `scheduled_wakes` count
new: - the normalised event for a submitted prompt (`viola-core`) and the turn end's `scheduled_wakes` count (verified: crates/viola-agent-claude/src/hook.rs:210, no relied row behind it)
old: - [inferred] the hook payload parse in `viola-agent-claude` (UserPromptSubmit), and the capability ledger if a
new: - the hook payload parse in `viola-agent-claude` (UserPromptSubmit; verified, crates/viola-agent-claude/src/hook.rs:180-225), and the capability ledger if a
old: - [inferred] the live rig (`viola-e2e`'s `viola-live-rig`) and the fake agent, where a wake would be rehearsed.
new: - the live rig (`viola-e2e`'s `viola-live-rig`, crates/viola-e2e/src/rig/wake.rs) and the fake agent (src/bin/viola-fake-agent.rs, its keyless `--inject-harness-turn` the precedent), where a wake would be rehearsed (verified); a root test of the wheel (tests/cli_wheel.rs) is where the rehearsal lands.
old: - CARRY (cry-umbkmr7p) [inferred]: the founder's own word (2026-11-17, relay
new: - CARRY (cry-umbkmr7p) [verified at HEAD: the 2.1.328 bundle's UserPromptSubmit hook input spreads `...!1` where `source` stood, so no command hook receives it; research Patterns]: the founder's own word (2026-11-17, relay
old: - CARRY (cry-4tpqeixz) [inferred], measured-marked and hypothesis-marked, kept verbatim: the founder's own word
new: - CARRY (cry-4tpqeixz) [measured part spot-checked at HEAD: the bundle carries no source to a command hook; hypothesis verified as an equality at HEAD: a prompt whose bytes passed viola's stdin moved the wheel at its key (src/run/wheel.rs:305-312) before its hook, so the hook-side move (src/run/send/slot.rs:251) is the only one a keyless prompt makes — research Graph impact], measured-marked and hypothesis-marked, kept verbatim: the founder's own word
old: - CARRY (cry-iuatjqkx) [inferred]: PROVISIONAL (delegate overseer, 2026-11-19T21:45:43Z, the route dialog of the
new: - CARRY (cry-iuatjqkx) [coordinates verified: `arm_sf`'s `sf_cell_attrs` and `arm_cc`'s `hint_cells`, crates/viola-e2e/src/rig/mod.rs:2326, crates/viola-e2e/src/rig/screen.rs:186]: PROVISIONAL (delegate overseer, 2026-11-19T21:45:43Z, the route dialog of the
old: - CARRY (cry-yx2svady) [inferred]: (re-pinned whole from «`/context` as a local command», its carry cry-4olcx4il,
new: - CARRY (cry-yx2svady) [coordinates verified: the rehearsal and its kept `after-del` frame, crates/viola-e2e/src/rig/mod.rs:2340; no red read since, its cause still unread]: (re-pinned whole from «`/context` as a local command», its carry cry-4olcx4il,
old: - [inferred] the phase directive `phase-wake-directive.md` (inputs#I1; the phase invocation inputs#I2; the overseer's folder, outside this repository; its
new: - the phase directive `phase-wake-directive.md` (verified against its snapshot; inputs#I1; the phase invocation inputs#I2; the overseer's folder, outside this repository; its
old: - [inferred] facts the directive (inputs#I1) states as measured by the overseer 2026-11-19: a bare `claude` on this host is
new: - facts the directive (inputs#I1) states as measured by the overseer 2026-11-19 (cited as the directive's, not re-measured here: no live read in this phase): a bare `claude` on this host is
