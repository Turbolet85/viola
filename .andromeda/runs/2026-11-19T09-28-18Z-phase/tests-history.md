@history_item n=1 marker=amd-gotsfh2z
**amd-gotsfh2z** — fixtures named `<Event>.<variant>.json` by the CLI's PascalCase hook name, no mapping table; `boot` passes `--fixtures <root>/fixtures/claude` and the fake agent joins `<cli-version>`. The 2.1.328 set must land under `fixtures/claude/2.1.328/` in exactly this naming (decision/fixture-named-by-hook-event, contract/claude-fixtures/payload-path).

@history_item n=2 marker=amd-ljexexcl
**amd-ljexexcl** — the fake agent reads `<fixtures>/<cli-version>/<Event>.<variant>.json`; consumer-first, no shapes invented before a recorded fixture. Precedent that a new set is a plain recorded copy, nothing hand-shaped.

@history_item n=3 marker=amd-xdupmfpc
**amd-xdupmfpc** — fixture hygiene is a run-time directory walk (`claude_fixtures`), not rstest `#[files]` (rstest 0.27 refuses an empty glob at compile time); it covers `fixtures/claude/*/*.json`. The new set joins the walk automatically; verify's step lines pinned by literals in `tests/cli_verify.rs`.

@history_item n=4 marker=amd-e4lqodj7
**amd-e4lqodj7** — root `stamped_home` runs verify against the fake agent with a pinned `--cli-version`, `stamped` only on exit 0 and a `stamped <ver>  <n> pass  0 fail` last line. Where the stamped-home version literal lives, should the default move.

@history_item n=5 marker=amd-hq5y6rqc
**amd-hq5y6rqc** — fake agent `DEFAULT_CLI_VERSION` is the recorded set's version, and the harness `boot --cli-version` default equals it (decision/harness-cli-version-matches-fake). Any decision on whether CI's default moves from 2.1.325 to 2.1.328 rests on this rule.

@history_item n=6 marker=amd-hcwxgizg
**amd-hcwxgizg** — `contract_fake_agent_drift` compares each recorded `fixtures/claude/<ver>/` set, walked at run time (an empty walk fails), byte for byte by receipt `stdin_hex`; recorded files are the pinned artifact, no insta snapshot. The drift suite will pick up 2.1.328 by walk; a red there stops implement.

@history_item n=7 marker=amd-kae2hbyp
**amd-kae2hbyp** — dialog tier first relayed, default version moved 2.1.283 → 2.1.287 across `DEFAULT_CLI_VERSION`, `RECORDED_CLI_VERSION`, `stamped_home`, `Wrapper::boot`; hygiene walk lists sets. Map of every site a default-version move touches.

@history_item n=8 marker=amd-j7kezqv6
**amd-j7kezqv6** — `contract_ledger_probes`: stamped and drift-only sets are literal lists, every fixture dir on exactly one list; screen fixtures (`Screen.*.json`, `claude-screen.v1.json`) recorded signature-rows-only, scrub-as-detector. The 2.1.328 dir must be placed on the stamped list (`STAMPED`) or the suite fails (decision/ledger-probe-contract-suite).

@history_item n=9 marker=amd-ucfbnmwh
**amd-ucfbnmwh** — dialog tier recorded by verify's Runs C/D as `<Event>.<stem>-<n>.json` (12 per version), scrubbed and unclean-checked; the drift contract replays every variant. Shape a clean `--record` on 2.1.328 must yield.

@history_item n=10 marker=amd-6nd2bx5a
**amd-6nd2bx5a** — framing tier (`paste-<n>`, `SessionEnd/Start.clear-<n>`) joins recorded sets; the drift contract replays them under `--framing`, and a set without them runs without the option and must exit 1. A new stamped set must carry the full framing tier.

@history_item n=11 marker=amd-pnr7hcja
**amd-pnr7hcja** — Run B's four added pastes, `paste-3` recorded; framing variants counted per set and hygiene-walked. Part of the framing tier the 2.1.328 recording must reproduce.

@history_item n=12 marker=amd-eijbfi3v
**amd-eijbfi3v** — `contract_ledger_probes` lists each relied id once (`ROW_IDS`) and ends at the relied count; StatusLine fixtures and the kept `workspace.repo` added (both later retired). History of the relied-count literals the new set's probe run must meet.

@history_item n=13 marker=amd-aogmb6c5
**amd-aogmb6c5** — Run E records `SessionStart.resume-1.json`; the fake agent's `--resume` builds on `resume-1` where the set holds it, else `SessionStart.default` — a version-keyed fake behaviour (the scope's P3 size question on what in the fake is keyed to a version).

@history_item n=14 marker=amd-rkuzizt2
**amd-rkuzizt2** — fake agent sets `source`/`session_id` on the launch SessionStart under `--resume`; the live payload's key set differed from the rewritten one. Origin of decision/fake-agent-replays-recorded-bytes' set-field list.

@history_item n=15 marker=amd-c4x5jj55
**amd-c4x5jj55** — Run B's agents read recorded as `Agents.recorded-1.json` against `claude-agents-fixture.v1.json`, hygiene-walked, shape-compared by `fake_agent_agents_replay_matches_every_recorded_agents_set`; fake as `claude` answers from files beside its exe, no env (decision/agents-stub-from-files-no-env, decision/fake-agent-on-test-path).

@history_item n=16 marker=amd-fwkd577g
**amd-fwkd577g** — `agents-join` row and revive's `session-live` (one `claude agents --json` read lists the session live elsewhere) landed with their cases (contract/revive-exit-1/session-live).

@history_item n=17 marker=amd-n3og23a3
**amd-n3og23a3** — `local-command-compact` row; `compact-<n>` framing variant and `fake_agent_compact_replay_matches_every_recorded_compact_set`; fake agent's compact replay sets the last SessionStart's id. Another per-set replay the new set enters.

@history_item n=18 marker=amd-np7arnqh
**amd-np7arnqh** — `/compact` confirmed only by a same-id `session-start` of cause compact inside `COMPACT_CONFIRM_WINDOW` 40 s (PROVISIONAL), proven on the recorded `compact-1` (decision/compact-confirm-window, decision/compact-confirmed-by-same-session).

@history_item n=19 marker=amd-nsngbyjo
**amd-nsngbyjo** — local command `unconfirmable` outcome and human line; `/clear` confirmed by a new-session `session-start`, proven on the recorded `clear-1` variants (decision/clear-confirmed-by-new-session, contract/viola-send-human-mode/unconfirmable).

@history_item n=20 marker=amd-mbgpqmaf
**amd-mbgpqmaf** — the 2.1.302 set entered the suite: `contract_ledger_probes`' stamped sets became `2.1.287` and `2.1.302`, the hygiene walk and Recorded payloads row include the set's dialog, framing, resume, agents variants and screens, a second input-box literal and `--long-turn-screen`. Direct precedent for landing a newer stamped set.

@history_item n=21 marker=amd-g7wbfdxe
**amd-g7wbfdxe** — `wait_ready` appends the stalled wrapper's last event, wchan and state to the watch report before panicking (decision/readiness-stall-report).

@history_item n=22 marker=amd-7t5rnlqi
**amd-7t5rnlqi** — the `Mcp.<variant>.json` exchange recorded by test-only `viola-mcp-tap` (`record` 0600 never committed, `fixture` through the `--record` scrub and four refusal codes); `mcp_fixtures` walks it against `claude-mcp-fixture.v1.json`; `tests/mcp_recorded_exchange.rs` replays it. The R4M re-record lands here (decision/mcp-exchange-recorded-by-tap, decision/mcp-tap-refuses-dirty-exchange, contract/mcp-fixture-scrub-refusal/*).

@history_item n=23 marker=amd-mrxt76c2
**amd-mrxt76c2** — a test that plants a fake-agent copy per recorded set must plant every copy sequentially before any thread spawns (ETXTBSY on tmpfs). Adding a set adds a copy to `fake_agent_agents_replay_matches_every_recorded_agents_set`: trap.

@history_item n=24 marker=amd-e7zvymxm
**amd-e7zvymxm** — verify writes its refusal code on `process-exit` under a parsed `VIOLA_NAME`, `unclean-fixture` among them; the fixture's file name enters no code; a failing row is no refusal (contract/verify-exit-detail/unclean-fixture, decision/verify-failing-row-no-refusal).

@history_item n=25 marker=amd-ee2rxbze
**amd-ee2rxbze** — `LedgerRow::relied()` is all twenty-four rows on every build; `STEPS_PASS`, `ROW_IDS`, `LEDGER_ROWS` `[&str; 24]`. Count literals the 2.1.328 probe run must meet.

@history_item n=26 marker=amd-duwfklqu
**amd-duwfklqu** — `dialog_drift_names_a_changed_annotation` plants a forwarded-annotation drift in `synthetic_dialog_set`; the dialog replay's synthetic witness (decision/fake-agent-matches-verify-fixtures).

@history_item n=27 marker=amd-tvc747xe
**amd-tvc747xe** — `option_details` and `background_tasks` read from recorded fixtures (the 2.1.287 fixture holds no `preview`, the Stop's empty array reads 0); per-version fixture content decides these readings (contract/question-event/option-details, contract/dialog-data/null-details, contract/turn-ended/background-tasks).

@history_item n=28 marker=amd-gsat5bin
**amd-gsat5bin** — readiness gate waits up to `GATE_MAX_WAIT` 8 500 ms on a quiet screen with no input-box literal; `verify_window_` kills sized to that bound by design.

@history_item n=29 marker=amd-ktivg7fc
**amd-ktivg7fc** — `~ contract/readiness-gate/gate-max-wait`: the input box reads up on its own rule and prompt rows with no footer literal. A moved 2.1.328 footer literal is caught by the gate's box-row reading.

@history_item n=30 marker=amd-haw6vw3o
**amd-haw6vw3o** — `+ decision/screen-fixture-box-rows-as-literals`: recorded screen fixtures keep box rows as their literals alone, the hygiene walk refuses more. Binds the 2.1.328 screens.

@history_item n=31 marker=amd-53xcjrzh
**amd-53xcjrzh** — `~ decision/hygiene-reports-class-only`: the class list dropped (email class added); the checker reports class only.

@history_item n=32 marker=amd-5mya4wvt
**amd-5mya4wvt** — `~ decision/fixtures-recorded-by-live-verify`: recorders named per fixture kind, viola-hypr-tap added beside viola-mcp-tap.

@history_item n=33 marker=amd-pfdqr7rr
**amd-pfdqr7rr** — `~ decision/fixture-hygiene-walk`, `~ decision/fake-agent-on-test-path`, `~ decision/recorder-scrub`: Hyprland reply keep-list, fake as hyprctl by file name, same scrub in viola-hypr-tap.

@history_item n=34 marker=amd-eefnob23
**amd-eefnob23** — `~ decision/dirty-payload-refuses-recording`: a dirty payload, MCP exchange or Hyprland set refuses the whole recording. Any hygiene refusal on 2.1.328 is a red, never a hand scrub.

@history_item n=35 marker=amd-erzqi2ha
**amd-erzqi2ha** — `~ decision/fixtures-walked-for-hygiene`: fake scripts, Claude and Hyprland fixtures walked with planted cases.

@history_item n=36 marker=amd-trilsjw7
**amd-trilsjw7** — `~ decision/fake-agent-as-claude`: the fake agent serves claude on a test-scoped PATH and hyprctl in a test bin dir.

@history_item n=37 marker=amd-itty4qan
**amd-itty4qan** — `~ decision/fakes-for-external-services`: the real hyprctl replaced by the fake agent too.

@history_item n=38 marker=amd-jqhveuv3
**amd-jqhveuv3** — `~ decision/fakes-for-external-services` except: viola-hyprland's own unit tests use a planted shell stub (cargo-mutants runs only the mutated package's tests).

@history_item n=39 marker=amd-rxbjdtib
**amd-rxbjdtib** — `~ decision/ci-never-real-claude`: CI never touches the real claude or hyprctl.

@history_item n=40 marker=amd-gxqopq3w
**amd-gxqopq3w** — `~ decision/no-production-data-in-tests`: fixtures come from `viola verify` probe text or a test-only recorder's capture, scrubbed.

@history_item n=41 marker=amd-mtepomer
**amd-mtepomer** — `~ decision/fakes-for-external-services`: no generated statusline stdin stands in for usage values any more.

@history_item n=42 marker=amd-zdr4o5cm
**amd-zdr4o5cm** — `~ decision/self-bootstrapping-test-data`: no budget.json seeding; stamps from verify against the fake agent.

@history_item n=43 marker=amd-qdobhuzf
**amd-qdobhuzf** — `~ decision/version-gate-needs-all-relied-rows`: a retired row a stamp still holds changes nothing (`statusline-rate-limits` retired). The 2.1.328 stamp is judged on the relied rows alone.

@history_item n=44 marker=amd-5w4h5cl7
**amd-5w4h5cl7** — live `viola verify` read `statusline-rate-limits` red from account plan state, not the CLI (later retired). Precedent: a live red may be the account, not the CLI version.

@history_item n=45 marker=amd-fsc6cf3x
**amd-fsc6cf3x** — `~ decision/fake-agent-contract-suite`, `~ decision/fixtures-recorded-by-live-verify`: StatusLine fixtures and verify's status-line variants removed; drift covers spine, dialog, framing, resume, compact. A 2.1.328 recording holds no StatusLine variant, unlike older sets.

@history_item n=46 marker=amd-ii6ctcvk
**amd-ii6ctcvk** — `~ decision/fixture-hygiene-walk` except emptied (no `workspace.repo` kept), `~ contract/fake-agent-receipt/kind` loses `statusline`, `+ contract/fake-agent-cli/settings`. The hygiene walk now refuses any repo-owner handle in a 2.1.328 file.

@history_item n=47 marker=amd-u64exh5i
**amd-u64exh5i** — `~ decision/no-production-data-in-tests` except emptied: no fixture holds a `workspace.repo`.

@history_item n=48 marker=amd-lbwwxikq
**amd-lbwwxikq** — `~ decision/recorder-scrub`: the recorder scrubs hook payloads only, status line variants gone.

@history_item n=49 marker=amd-tepf5o6p
**amd-tepf5o6p** — `~ decision/recorder-scrub` except emptied: the `workspace.repo.owner` carve-out is gone; the whole-word OS-user rule applies everywhere.

@history_item n=50 marker=amd-aml2265l
**amd-aml2265l** — `~ contract/event-log/start-wheel-record`: where folder-trust keys pass through the wrapper a human-input wheel record precedes session-start, measured live. Relevant to 2.1.328's new trust prompt behaviour.

@history_item n=51 marker=amd-uwsvrnci
**amd-uwsvrnci** — `+ decision/start-lines-read-from-start-offset`: a start's lines read from its own offset, never line 1 of a reused instance log.

@history_item n=52 marker=amd-tveox2hq
**amd-tveox2hq** — `~ decision/root-wait-bound`: root waits 12 s, below the 15 s mutants kill and cargo-mutants' 20 s floor.

@history_item n=53 marker=amd-kmpo2mdb
**amd-kmpo2mdb** — `~ decision/within-7s-wait-bound`: `WITHIN` = 12 s for every wait not bounded by a per-test kill (the scope's 20 s lint bound on waits).

@history_item n=54 marker=amd-6r6ezlbv
**amd-6r6ezlbv** — `~ decision/mutants-kill-below-cargo-mutants-timeout`: mutants default kill 15 s, below the 20 s floor.

@history_item n=55 marker=amd-bfkajssx
**amd-bfkajssx** — `+ decision/record-home-names-no-home`: a home given to `verify --record` comes from the shared record-home helper, no path component named home.

@history_item n=56 marker=amd-or4mje25
**amd-or4mje25** — `~ decision/record-home-names-no-home` (2.1.325 set chunk): the live rig's record arm refuses `record-home`, exit 3, uncounted, on a home-named scratch base before any credentials read. Binds R4's record start; the 2.1.325 set is this chunk's stated ground.

@history_item n=57 marker=amd-warr2i6n
**amd-warr2i6n** — `~ decision/hook-payloads-recorded-by-verify` except (StopFailure has no recorded payload, ledger row or probe; synthetic in tests), `+ decision/send-cases-copy-recorded-fixtures` (a send-after-turn fixtures root starts as a copy of the recorded set).

@history_item n=58 marker=amd-tzbez4pp
**amd-tzbez4pp** — StopFailure's no-ledger-row ratified; its live reading owned by «The 2.1.325 recorded fixture set» — the live-payload line this chunk's C3 carry moved from.

@history_item n=59 marker=amd-r3tbeyu7
**amd-r3tbeyu7** — the same ratification for §7's `hook-payloads-recorded-by-verify` except.

@history_item n=60 marker=amd-anewhkwa
**amd-anewhkwa** — `+ decision/one-verify-stamp-per-build`: stamped homes copy one verify stamp per pair of built binaries under a lock, keyed by hashes.

@history_item n=61 marker=amd-jiwpp76u
**amd-jiwpp76u** — `~ decision/fresh-home-per-test`: a stamped home is a copy of its build's stamp, never a link.

@history_item n=62 marker=amd-smxrzozm
**amd-smxrzozm** — `~ decision/verify-driven-tests-20s-kill` why: one stamp per build keeps each stamped-home test under the 20 s kill. A second stamped set's verify runs bear on this sizing.

@history_item n=63 marker=amd-fi4mla3k
**amd-fi4mla3k** — `~ contract/fake-agent-input/bracketed-paste`: `--trim-prompt-end` removes the trailing run.

@history_item n=64 marker=amd-ea56tn73
**amd-ea56tn73** — `~ contract/driver-send/local-command`: a ledger-listed local command classified by name, confirmed only by post-condition, else unconfirmable.

@history_item n=65 marker=amd-owwzhlbv
**amd-owwzhlbv** — `~ decision/local-command-post-condition`, `~ contract/send-result/unconfirmable`: `/mcp` confirmed by a new stdout tag in the transcript past its pre-paste end; an echo-confirmed command with no readable transcript is unconfirmable. The mechanism C3's `/context` half (read from a readable transcript) rests on.

@history_item n=66 marker=amd-t37kppzk
**amd-t37kppzk** — `~ decision/fake-agent-replays-recorded-bytes`: the fake sets `transcript_path` on the launch SessionStart under `--transcript`.

@history_item n=67 marker=amd-cqpelreg
**amd-cqpelreg** — `~ contract/fake-agent-hooks/stdin-payload`, `+ contract/fake-agent-cli/transcript`, `+ contract/fake-agent-input/echo-command`: the fake's transcript and echo tag line, which Run B's echo probe relies on.

@history_item n=68 marker=amd-5m6sscbm
**amd-5m6sscbm** — `+ decision/stamping-verify-passes-transcript`: every stamping verify passes the fake `--transcript` beside `--framing`, else the local-command-echo row fails. Binds any new stamping helper over 2.1.328.

@history_item n=69 marker=amd-ponzfcew
**amd-ponzfcew** — `~ decision/local-command-unconfirmable`: an echo-confirmed command with no readable transcript gives unconfirmable.

@history_item n=70 marker=amd-iyj7hlaq
**amd-iyj7hlaq** — the local-command post-condition and unconfirmable rows ratified on the founder's own word.

@history_item n=71 marker=amd-ktkoz6oa
**amd-ktkoz6oa** — `~ contract/event-data/turn-ended`: `cause` one of nine closed words on an API-error end.

@history_item n=72 marker=amd-vxm5gokf
**amd-vxm5gokf** — `~ contract/event-data/turn-ended`: `scheduled_wakes` joins the data keys.

@history_item n=73 marker=amd-wznql5rf
**amd-wznql5rf** — `+ contract/dialog-data/scheduled-wakes`: Stop's `session_crons` read by length only; a non-array writes one drift entry. A 2.1.328 Stop payload's shape is read through this.

@history_item n=74 marker=amd-2nxmikkg
**amd-2nxmikkg** — `+ contract/turn-ended/scheduled-wakes` and its reach.

@history_item n=75 marker=amd-cpftmyp5
**amd-cpftmyp5** — `− decision/live-readings-not-a-test-layer` → `+ decision/standing-live-rig`: live rounds reuse the one committed `viola-live-rig`, rehearsals in CI, live starts by hand under the founder's cap, each ledgered. R4/R4M run on this rig (C2).

@history_item n=76 marker=amd-lbeg2hcc
**amd-lbeg2hcc** — `+ contract/fake-agent-screens/trust-dialog` (the fake draws claude 2.1.325's trust form), `~ contract/fake-agent-events/session-start`, `~ contract/fake-agent-exit/exit-triggers`, `~ contract/fake-agent-input/keystroke`. The trust form is version-specific: 2.1.328's new decline text is the fake's P3 question.

@history_item n=77 marker=amd-jfjpogdl
**amd-jfjpogdl** — `~ decision/never-parse-rendered-child-screen` except: the rig reads the child's screen by compiled literals into closed words only.

@history_item n=78 marker=amd-o74pvmhg
**amd-o74pvmhg** — `~ decision/no-child-screen-assertions` except, `~ decision/real-claude-local-only`: the real CLI reached only via verify and the rig's live starts, never in CI.

@history_item n=79 marker=amd-5oi2g4ua
**amd-5oi2g4ua** — `~ decision/discipline-at-every-layer` except: the rig reads the screen by literals and passes a read trust dialog under its key grant.

@history_item n=80 marker=amd-brtfidko
**amd-brtfidko** — `~ decision/no-blanket-approval-tests` except: the rig passes a read trust dialog under its key grant.

@history_item n=81 marker=amd-uhywqnah
**amd-uhywqnah** — `~ decision/no-screen-parsing` except: the rig's literal-only screen read.

@history_item n=82 marker=amd-vahqpute
**amd-vahqpute** — `~ decision/wrapper-drop-guard`, `+ decision/no-untied-viola-spawn`, `+ decision/tied-child-spawn-thread`: every long-lived viola child tied to its test thread by parent-death kill; a census holds it.

@history_item n=83 marker=amd-6eevcqfn
**amd-6eevcqfn** — `~ decision/supervisor-ordinary-child`: in-test supervisor SIGKILL, its stamping verify SIGTERM at the test thread's end.

@history_item n=84 marker=amd-aavmgz35
**amd-aavmgz35** — `~ decision/rstest-case-tables` except: viola-e2e/viola-core and shared helpers taken by path use labelled tables, not rstest.

@history_item n=85 marker=amd-yckbqyq2
**amd-yckbqyq2** — `+ decision/no-verify-start-outside-test-home`: a census holds every test-started verify tied to a test-owned home, a closed exempt list with reasons.

@history_item n=86 marker=amd-ot6nnteh
**amd-ot6nnteh** — `+ decision/verify-under-test-owned-user-home`: test verifies run under a test-owned user home; a `--record` verify keeps the real HOME and points a kept `CLAUDE_CONFIG_DIR` at the test home (the scrub reads HOME). Trap for any recording path a test drives.

@history_item n=87 marker=amd-yq2ulaob
**amd-yq2ulaob** — `+ contract/fake-agent-cli/trust-config`, `+ contract/fake-agent-cli/trust-record-blank`: fake agent options for the trust record.

@history_item n=88 marker=amd-udylcpzx
**amd-udylcpzx** — `+ decision/live-failing-reading-not-verification`: a failing live reading is recorded as read, never verification; a confounded one leaves the row unverified, its re-read owed. Binds how C3's `/context` reading is recorded.

@history_item n=89 marker=amd-ympnjham
**amd-ympnjham** — `~ decision/record-only-at-zero-failing-rows` except: a failed recording verify by the rig alone writes first-screen files, never committed. The landed set must come from a 0-fail recording.

@history_item n=90 marker=amd-5bz3flwz
**amd-5bz3flwz** — `~ contract/fake-agent-cli/trust-config` (the CLI's trust walk), `+ decision/trust-start-folders-outside-repository`: trust-start test folders sit under /tmp.

@history_item n=91 marker=amd-46h6wtq6
**amd-46h6wtq6** — `~ contract/verify-output/step-line`, `~ contract/verify-output/summary-line`: `absent` readable on the browser-tools-prompt row alone and counted. 2.1.328's browser-tools prompt is read unchanged from 2.1.325 per the scope.

@history_item n=92 marker=amd-u5d3v76z
**amd-u5d3v76z** — `~ decision/version-gate-needs-all-relied-rows`: every relied row counting, `absent` on browser-tools-prompt only.

@history_item n=93 marker=amd-zlzmc2mh
**amd-zlzmc2mh** — `~ contract/local-live-suite/pass`: one pass line per row, or one absent on browser-tools-prompt.

@history_item n=94 marker=amd-rnygpidn
**amd-rnygpidn** — `~ decision/ledger-row-verify-probe`: a probe reports pass, absent or fail and a stamped line.

@history_item n=95 marker=amd-uzh4o2cr
**amd-uzh4o2cr** — `+ contract/fake-agent-cli/start-screen`: `--start-screen` draws `Screen.<phase>` from the set; the CI proof of the browser-tools literal rests on it.

@history_item n=96 marker=amd-npoy5txr
**amd-npoy5txr** — `run --local-live`: refuses `live-in-ci` exit 2 under CI before any build; builds then verifies the real claude; suite `local-live` and its closed failure codes (`build`, `verify-exit-<n>`, `verify-exit-none`, `row-missing`).

@history_item n=97 marker=amd-m2fk4ggl
**amd-m2fk4ggl** — `--local-live` alone is its own selection; verify names no program and stamps the first `claude` on PATH. On this host that is 2.1.328.

@history_item n=98 marker=amd-5gt53dby
**amd-5gt53dby** — mutation moves to the epoch boundary; no per-chunk, pre-push or CI mutation gate (decision/mutation-at-epoch-boundary, decision/mutation-gate-cadence). The scope's "no mutation run" rests on this.

@history_item n=99 marker=amd-puxntbrv
**amd-puxntbrv** — `schema-check` and `secret-scan` declared as internal subcommands with output shapes and reasons (contract/schema-check-*), scan-gated uploads.

@history_item n=100 marker=amd-3j55jjnw
**amd-3j55jjnw** — `target/e2e-home` keeper: a linked base on the dev host's tmpfs, 0700, refused otherwise (decision/e2e-homes-under-target-e2e-home).

@history_item n=101 marker=amd-75ste4oe
**amd-75ste4oe** — headroom floor 9 GiB on the linked base; fake agent's spawned commands in their own process group.

@history_item n=102 marker=amd-btx2rfv5
**amd-btx2rfv5** — the chaos feed-panic home boots outside the scans (decision/fresh-home-per-test except).

@history_item n=103 marker=amd-6ct2ncg2
**amd-6ct2ncg2** — the 11-name identity floor replaces the 14-name literal (decision/r8-claude-env-strip, decision/literal-test-oracles).

@history_item n=104 marker=amd-p5q5fgy2
**amd-p5q5fgy2** — R8 strip by exact name; `claude_env_keep` the persistent exception (decision/claude-child-env-strip).

@history_item n=105 marker=amd-3qaxqoxx
**amd-3qaxqoxx** — shared root helpers in `tests/support/`; no root test file defines its own copy (decision/one-shared-test-helper).

@history_item n=106 marker=amd-l6hnar3d
**amd-l6hnar3d** — a home a test makes before viola goes through `create_private_dir`, never `create_dir_all` (decision/test-homes-made-private).

@history_item n=107 marker=amd-hwmom55r
**amd-hwmom55r** — the impostor cases' test-built snapshots are an exception to "never hand-written" (decision/product-state-never-hand-written, decision/product-state-from-real-runs).

@history_item n=108 marker=amd-xntsyub4
**amd-xntsyub4** — the hand-written state-file ban scoped to booted-wrapper tests; a reader's unit cases may alter the writer's bytes (decision/state-files-never-hand-written).

@history_item n=109 marker=amd-cnisrb36
**amd-cnisrb36** — MCP tests own their `viola mcp` child, never rmcp's child-process transport (truncated coverage); `tools/list` exactly five tools with `ttlMs` 0 and `cacheScope` private (decision/mcp-tests-own-server-child, contract/mcp-server/tools-list). What the recorded MCP exchange replay holds.

@history_item n=110 marker=amd-opxwr6qu
**amd-opxwr6qu** — the forced vt100 panic case byte-searches the outer terminal, never parses the child screen (decision/no-child-screen-assertions except).

@history_item n=111 marker=amd-h2oapnuy
**amd-h2oapnuy** — the `VIOLA_DIR` home step and canonicalisation ratified (decision/home-resolution-order).

@history_item n=112 marker=amd-rgs65g6f
**amd-rgs65g6f** — CLI outputs pinned by literals in their test files; no trycmd or assert_cmd (decision/cli-outputs-pinned-by-literal, decision/human-output-pinned-by-literals).

@history_item n=113 marker=amd-62z53en5
**amd-62z53en5** — `[profile.ci]`'s 20 s kill line lists twenty-six verify-driven binaries (decision/ci-stamped-home-kill-line); a new binary would join by the filter-line rule.

@history_item n=114 marker=amd-zrj7cxkl
**amd-zrj7cxkl** — Linux only, one `ubuntu-latest` CI leg; Windows/macOS out of the tree (absence/windows-and-macos-support).

@history_item n=115 marker=amd-gzxcxwvf
**amd-gzxcxwvf** — the GUI surfaces and browser leg leave (absence/browser-suite).

@history_item n=116 marker=amd-glv22nra
**amd-glv22nra** — the fake agent exits on `\x03` or stdin EOF, waiting for a running hook first (contract/fake-agent-exit/exit-triggers).

@history_item n=117 marker=amd-w5k5ft3a
**amd-w5k5ft3a** — the fake agent fires SessionStart/default once after `start_receipts` (contract/fake-agent-events/session-start).

@history_item n=118 marker=amd-hlgfwft6
**amd-hlgfwft6** — the event log reader's three counts and the appender's heal (decision/defensive-log-reader, guard `three_counts`).
