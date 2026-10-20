- line: .andromeda/architecture.md:45
old: «`input_box: ["for agents"]` and `modals:»
new: «`input_box: ["for agents", "(shift+tab to cycle)"]` and `modals:»

- line: .andromeda/architecture.md:45
old: «(`Signatures::holds_any`). A screen is ready when»
new: «(`Signatures::holds_any`). The second input-box literal is the resting footer a CLI keeps once it hides the first: on 2.1.302 the footer hint `for agents` is gone 15 s after the start and the row reads `⏵⏵ auto mode on (shift+tab to cycle)`, whose mode word is the user's permission mode, so the literal is the mode-independent `(shift+tab to cycle)`, which no other CLI text read holds (as measured at `viola-0.1.0/chunks/2026-10-19-newer-claude-cli-rows/evidence/footer-2.1.302.md`). A screen is ready when»

- line: .andromeda/architecture.md:45
old: «`input-box-signature` row (the ready and turn screens hold the input-box literal and no modal) validate them.»
new: «`input-box-signature` row (the ready screen, the turn screen and the screen Run B settles on after the long paste's turn each hold an input-box literal and no modal; a long turn whose screen never settles fails it) validate them.»

- line: .andromeda/architecture.md:101
old: «(2.1.287 alone today; a verify is six `claude` starts)»
new: «(2.1.287 and 2.1.302 today, each `24 pass  0 fail` on the Linux dev host, 2.1.302 as measured at `viola-0.1.0/chunks/2026-10-19-newer-claude-cli-rows/evidence/live-round.md`; a verify is six `claude` starts)»

- line: .andromeda/architecture.md:315
old: «twelve argv options (the count's rule is this list), no env, of which eight serve»
new: «thirteen argv options (the count's rule is this list), no env, of which nine serve»

- line: .andromeda/architecture.md:315
old: «a missing file draws nothing), and two serve `viola revive`'s tests»
new: «a missing file draws nothing), and the test-only screen switch `--long-turn-screen <phase>` (with `--framing` and `--turn-stop`: after the long paste's turn alone, after its Stop, `Screen.<phase>` of the set is drawn in place of `Screen.turn`), and two serve `viola revive`'s tests»

- line: .andromeda/architecture.md:374
old: «Committed sets: `2.1.283`, `2.1.287` and `2.1.288`.»
new: «Committed sets: `2.1.283`, `2.1.287`, `2.1.288` and `2.1.302`.»

- line: .andromeda/architecture.md:374
old: «`24 pass  0 fail` on Unix); `2.1.288` (four spine payloads»
new: «`24 pass  0 fail` on Unix); `2.1.302` (4 spine payloads + 3 screens + 12 dialog variants + 6 framing variants + 2 status line variants + 1 resume variant + 1 agents variant, recorded whole by one `viola verify --record` round; 29 files, the count's rule `ls fixtures/claude/2.1.302 | wc -l`; its `Screen.ready` and `Screen.turn` keep one row holding both input-box literals) is stamped the same way; `2.1.288` (four spine payloads»

- line: .andromeda/test-plan.md:379
old: «12 under each of `2.1.288` and `2.1.287`)»
new: «12 under each of `2.1.288`, `2.1.287` and `2.1.302`)»

- line: .andromeda/test-plan.md:379
old: «six under `2.1.287` — `paste-1`»
new: «six under each of `2.1.287` and `2.1.302` — `paste-1`»

- line: .andromeda/test-plan.md:508
old: «(`SIGNATURES`: the input-box literal `for agents`, the modal literals»
new: «(`SIGNATURES`: the input-box literals `for agents` and `(shift+tab to cycle)`, the second the resting footer a CLI keeps once it hides the first, pinned by the `resting_footer` cases in `screen.rs`, the modal literals»

- line: .andromeda/test-plan.md:589
old: «`verify_pastes_no_local_command_once_the_tag_turn_screen_shows_a_modal`)»
new: «`verify_pastes_no_local_command_once_the_tag_turn_screen_shows_a_modal`), pasting nothing more once the screen drawn after the long paste's turn alone shows a modal (`19 pass  5 fail` on Unix, `16 pass  5 fail` on Windows, the receipt holding the probe prompt and the long paste only; the fake agent's `--long-turn-screen`; `verify_pastes_nothing_more_once_the_long_turn_screen_shows_a_modal`)»

- line: .andromeda/test-plan.md:601
old: «(`fixtures/claude/2.1.287/` alone today)»
new: «(`fixtures/claude/2.1.287/` and `fixtures/claude/2.1.302/` today)»

- line: .andromeda/test-plan.md:934
old: «(six today, under `2.1.287` only)»
new: «(six under each of `2.1.287` and `2.1.302`)»

- line: .andromeda/test-plan.md:934
old: «(two today, under `2.1.287` only, validated»
new: «(two under each of `2.1.287` and `2.1.302`, validated»

- line: .andromeda/test-plan.md:934
old: «(one today, under `2.1.287` only)»
new: «(one under each of `2.1.287` and `2.1.302`)»

- line: .andromeda/test-plan.md:934
old: «; one today, under `2.1.287` only, chunk»
new: «; one under each of `2.1.287` and `2.1.302`, chunk»

- line: .andromeda/test-plan.md:960
old: «twelve argv options (no env; the count's rule is architecture's list of them)»
new: «thirteen argv options (no env; the count's rule is architecture's list of them)»

- line: .andromeda/test-plan.md:960
old: «and the ten that follow, eight of them for verify's»
new: «and the eleven that follow, nine of them for verify's»

- line: .andromeda/test-plan.md:960
old: «in place of `Screen.turn`; a missing file draws nothing), and two for `viola revive`'s tests»
new: «in place of `Screen.turn`; a missing file draws nothing), the test-only screen switch `--long-turn-screen <phase>` (with `--framing` and `--turn-stop`: after the long paste's turn alone, after its Stop, the agent draws `Screen.<phase>.json` of its set in place of `Screen.turn`; parse cases `opts_long_turn_screen_takes_the_phase_it_names_alone` and `opts_long_turn_screen_defaults_to_none`), and two for `viola revive`'s tests»

- line: .andromeda/test-plan.md:968
old: «(today the sets `2.1.283`, `2.1.287` and `2.1.288`:»
new: «(today the sets `2.1.283`, `2.1.287`, `2.1.288` and `2.1.302`:»

- line: .andromeda/test-plan.md:968
old: «under each of `2.1.287` and `2.1.288`, `2.1.287`'s six recorded framing variants»
new: «under each of `2.1.287`, `2.1.288` and `2.1.302`, the six recorded framing variants of each of `2.1.287` and `2.1.302`»

- line: .andromeda/test-plan.md:968
old: «and `2.1.287`'s one recorded resume variant,»
new: «and the one recorded resume variant of each of `2.1.287` and `2.1.302`,»

- line: .andromeda/test-plan.md:968
old: «; `2.1.287`'s two status line variants,»
new: «; the two status line variants of each of `2.1.287` and `2.1.302`,»

- line: .andromeda/test-plan.md:968
old: «; `2.1.287`'s one agents fixture,»
new: «; the one agents fixture of each of `2.1.287` and `2.1.302`,»

- line: .andromeda/test-plan.md:968
old: «screens of `2.1.287` and `2.1.288` against»
new: «screens of `2.1.287`, `2.1.288` and `2.1.302` against»
