# Fan-out results — 2026-11-24-viola-adopt (wrap 2026-11-24T11-08-28Z)

architecture: add 7 · set 8 · retire 0 · none 1 · ops-architecture.md
security-plan: add 6 · set 6 · retire 0 · none 0 · ops-security-plan.md
design-system: add 13 · set 11 · retire 0 · none 1 · ops-design-system.md
layout-templates: add 14 · set 1 · retire 1 · none 0 · ops-layout-templates.md
test-plan: add 15 · set 5 · retire 0 · none 0 · ops-test-plan.md
obs-plan: add 4 · set 11 · retire 0 · none 0 · ops-obs-plan.md
a11y-plan: add 1 · set 11 · retire 0 · none 2 · ops-a11y-plan.md
main: ops-main-security-plan.md (add 1 · set 10) · ops-main-obs-plan.md (set 1) · ops-main-layout-templates.md (set 1)

Check 2 homes (one fact, one home): adopt refuses a held session → architecture decision/adopt-refuses-live-session; its fixed pair → design-system contract/viola-adopt-refusals/session-live; the `--json` code list → architecture contract/cli-json-output/adopt-refusal-codes; the verb's usage → architecture contract/cli-verbs/adopt; the id onto the argv → security-plan decision/adopt-session-id-argv-crossing; the preflight order → layout-templates contract/viola-adopt/preflight-order (beside revive's own order row there); the lineage → architecture decision/adopt-lineage-without-role-or-window; the command name → architecture name/command/adopt.

ops-architecture.md
decision/adopt-refuses-live-session · apply — check 5: the plan's expected amendment (adopt refuses a session a live claude holds), its home · word: delegate overseer, 2026-11-24
contract/cli-json-output/adopt-refusal-codes · apply — check 5: the plan's expected amendment, adopt's --json codes beside revive's; the one home of the list
contract/viola-adopt-refusals/session-live · reject — check 2: the fact's home is design-system contract/viola-adopt-refusals/session-live
contract/cli-verbs/adopt · apply — check 1: a new verb's usage, routine; states the shape's either-case hex (the disproved plan row)
name/command/adopt · apply — check 1: a new occupied command name, routine
decision/typed-session-id-on-child-argv · reject — check 2: the fact's home is security-plan decision/adopt-session-id-argv-crossing
decision/adopt-lineage-without-role-or-window · reject — check 7: not_master type 4 — the code holds how the lineage is chosen (lineage_of); decision/lineage-parent-authenticates-nothing (security-plan) already states the parent rule
decision/silent-roles-at-internal-error-catch · apply — check 5: the plan's expected amendment, adopt joins silent-roles-at-internal-error-catch
contract/cli-json-output/internal-error · apply — check 5: the same silent list, its contract
contract/child-argv/resume · apply — check 5: the plan's expected amendment, an adopted start's --resume <id>
contract/claude-child-argv/resume · apply — check 5: the same, its argv contract
decision/launch-pair-handed-not-verified · apply — check 1: adopt hands its pair like every start, routine
decision/no-start-over-live-instance · apply — check 1: adopt runs run's collision check, read in the code (preflight → collision_check)
decision/run-refuses-held-name · apply — check 1: the same holder check, routine
contract/diagnostics-role-files/run-name · apply — check 5: adopt logs as process run in run-<name>.ndjson

ops-security-plan.md
decision/state-tree-strict-modes-before-use · apply — check 5: adopt's start joins the named strict-modes list, read in the code (check_instance, then checked_exe → check_home_tree, before open_wrapper_log)
decision/home-tree-strict-modes-before-log · apply — check 5: the same, read in the code
decision/run-refuses-socket-dir-before-start · apply — check 1: adopt launches through run::start, which holds the socket-directory check
decision/claude-agents-resolved-from-path · apply — check 1: adopt calls revive's live_elsewhere, body unchanged
decision/launch-flags-checked · apply — check 1: adopt takes run's parse_model / parse_effort
contract/cli-json-error/internal-error · apply — check 5: the silent list gains adopt's start
decision/adopt-session-id-argv-crossing · reject — check 7: stands as decision/session-id-argv-closed-shape (also decision/session-id-argv-no-shell); main sets that row's except instead (ops-main-security-plan.md, Q1)
decision/adopt-refuses-live-session · reject — check 2: the fact's home is architecture decision/adopt-refuses-live-session
decision/adopt-agents-read-fails-open · reject — check 3: names the fail-open read «the no-signal arm», which W1 is not (W1 is acting on no foreign process); the fail-open fact stands in the sets of decision/session-live-read-fails-open (test-plan), decision/session-live-check-fails-open (obs-plan) and decision/claude-agents-read-fails-open (a11y-plan); W1 is main's own add
decision/adopt-arguments-checked · reject — check 2: the fact's home is architecture contract/cli-verbs/adopt
contract/strict-modes-refusal/adopt · apply — check 5: adopt's start refusal beside run and revive's contract, read in the code
decision/r5-adopt-script-ends-own-processes · apply — check 1: the test-only R5 script's process bound, the report's Symbols / APIs (close_host)

ops-design-system.md
contract/help-output/verb-groups · apply — check 5: the plan's expected amendment, setup: gains adopt
contract/viola-help/verb-groups · apply — check 5: the same, its twin
decision/run-prints-nothing-while-wrapped · apply — check 1: adopt pumps its child through run's pump, routine
contract/viola-run-output/child-started · apply — check 1: the same
decision/start-refusals-show-no-path-or-pid · apply — check 1: adopt's pairs hold no id, path or pid
contract/cli-refusal-line/fixed-unable-form · apply — check 1: adopt's refusals use the fixed form
contract/cli-stderr/fixed-message-refusal · apply — check 1: the same, its twin
contract/cli-start-refusal/fixed-message · apply — check 1: adopt's start refusals, first one wins
decision/silent-catch-sites · apply — check 5: the silent list gains adopt
contract/cli-catch-site/silent-verbs · apply — check 5: the same, its twin
decision/no-json-after-child-held-terminal · apply — check 1: adopt prints no document past pump_child, the report's Symbols / APIs
name/command/adopt · reject — check 2: the fact's home is architecture name/command/adopt
contract/adopt-json-refusal-code/strict-modes-failed · reject — check 2: the fact's home is architecture contract/cli-json-output/adopt-refusal-codes
contract/adopt-json-refusal-code/session-live · reject — check 2: the fact's home is architecture contract/cli-json-output/adopt-refusal-codes
contract/adopt-json-refusal-code/already-live · reject — check 2: the fact's home is architecture contract/cli-json-output/adopt-refusal-codes
contract/adopt-json-refusal-code/stale-heartbeat · reject — check 2: the fact's home is architecture contract/cli-json-output/adopt-refusal-codes
contract/adopt-json-refusal-code/pinned-hash-mismatch · reject — check 2: the fact's home is architecture contract/cli-json-output/adopt-refusal-codes
contract/adopt-json-refusal-code/squatted-name · reject — check 2: the fact's home is architecture contract/cli-json-output/adopt-refusal-codes
contract/adopt-json-refusal-code/socket-dir-refused · reject — check 2: the fact's home is architecture contract/cli-json-output/adopt-refusal-codes
contract/adopt-json-refusal-code/internal-error · reject — check 2: the fact's home is architecture contract/cli-json-output/adopt-refusal-codes
contract/cli-exit-codes/1-adopt-start · apply — check 1: adopt's exit-1 refusal form on the CLI surface, routine
contract/viola-adopt-refusals/session-live · apply — check 5: adopt's new fixed pair, its one home (W4 planning form) · word: delegate overseer, 2026-11-24
contract/viola-adopt-refusals/order · reject — check 2: the fact's home is layout-templates contract/viola-adopt/preflight-order
contract/viola-adopt-usage/exit-2 · reject — check 2: the fact's home is architecture contract/cli-verbs/adopt

ops-layout-templates.md
contract/cli-verbs/help · apply — check 5: the plan's expected amendment, setup: gains adopt
contract/cli-catch-site/internal-error · reject — check 7: its heir reads stands; main retires the row with no heir (ops-main-layout-templates.md)
contract/cli-catch-site-outside-revive-adopt/internal-error · reject — check 7: stands as contract/cli-internal-error/human (also contract/cli-internal-error/json)
contract/cli-verbs/adopt · apply — check 1: the verb's row in the layout's one-row-a-verb list, routine
contract/viola-adopt/usage · reject — check 2: the fact's home is architecture contract/cli-verbs/adopt
contract/viola-adopt-refusals/session-live · reject — check 2: the fact's home is design-system contract/viola-adopt-refusals/session-live
contract/viola-adopt/preflight-order · reject — check 7: not_master type 4 — the code holds the order (trusted → checked_exe → open_wrapper_log → preflight_order), the adopt-order control its guard
contract/viola-adopt-json/strict-modes-failed · reject — check 2: the fact's home is architecture contract/cli-json-output/adopt-refusal-codes
contract/viola-adopt-json/session-live · reject — check 2: the fact's home is architecture contract/cli-json-output/adopt-refusal-codes
contract/viola-adopt-json/already-live · reject — check 2: the fact's home is architecture contract/cli-json-output/adopt-refusal-codes
contract/viola-adopt-json/stale-heartbeat · reject — check 2: the fact's home is architecture contract/cli-json-output/adopt-refusal-codes
contract/viola-adopt-json/pinned-hash-mismatch · reject — check 2: the fact's home is architecture contract/cli-json-output/adopt-refusal-codes
contract/viola-adopt-json/squatted-name · reject — check 2: the fact's home is architecture contract/cli-json-output/adopt-refusal-codes
contract/viola-adopt-json/socket-dir-refused · reject — check 2: the fact's home is architecture contract/cli-json-output/adopt-refusal-codes
contract/viola-adopt-json/internal-error · reject — check 2: the fact's home is architecture contract/cli-json-output/adopt-refusal-codes
contract/viola-help/setup-row · apply — check 5: the setup row's literal, split from verb-groups at its bound (main's ops-main-layout-templates.md sets the verb-groups row)

ops-test-plan.md
contract/cli-human/internal-error · apply — check 5: the plan's expected amendment, cli-human/internal-error's silent list gains adopt's start
contract/strict-modes-refusal/no-line · apply — check 5: adopt's refusal writes no line
decision/real-claude-local-only · apply — check 1: the R5 script is a gated live-reading route
decision/session-live-read-fails-open · apply — check 1: adopt shares the fail-open read
decision/verify-driven-tests-20s-kill · apply — check 6: the folded CI item, the two verify-driven cases now in the verify_window_ class
decision/adopt-proven-live · apply — check 1: v2-13's gated clause, real adopt proven only by R5
decision/adopt-preflight-order · reject — check 2: the fact's home is layout-templates contract/viola-adopt/preflight-order
decision/adopt-refuses-live-session · reject — check 2: the fact's home is architecture decision/adopt-refuses-live-session
contract/adopt-refusal-code/strict-modes-failed · reject — check 2: the fact's home is architecture contract/cli-json-output/adopt-refusal-codes
contract/adopt-refusal-code/session-live · reject — check 2: the fact's home is architecture contract/cli-json-output/adopt-refusal-codes
contract/adopt-refusal-code/already-live · reject — check 2: the fact's home is architecture contract/cli-json-output/adopt-refusal-codes
contract/adopt-refusal-code/stale-heartbeat · reject — check 2: the fact's home is architecture contract/cli-json-output/adopt-refusal-codes
contract/adopt-refusal-code/pinned-hash-mismatch · reject — check 2: the fact's home is architecture contract/cli-json-output/adopt-refusal-codes
contract/adopt-refusal-code/squatted-name · reject — check 2: the fact's home is architecture contract/cli-json-output/adopt-refusal-codes
contract/adopt-refusal-code/socket-dir-refused · reject — check 2: the fact's home is architecture contract/cli-json-output/adopt-refusal-codes
contract/adopt-refusal-code/internal-error · reject — check 2: the fact's home is architecture contract/cli-json-output/adopt-refusal-codes
contract/adopt-json/error-before-child-pump · reject — check 2: the fact's home is architecture contract/cli-json-output/internal-error (set, adopt's start silent past the pump) and design-system decision/no-json-after-child-held-terminal
decision/adopt-id-session-shape · reject — check 2: the fact's home is security-plan decision/adopt-session-id-argv-crossing
decision/child-argv-read-from-proc · apply — check 6: the disproved claim «the fake's receipt holds argv», its disposition
decision/adopt-lineage-and-cwd · reject — check 2: the fact's home is architecture decision/adopt-lineage-without-role-or-window

ops-obs-plan.md
contract/strict-modes-refusal-exit/1 · apply — check 5: the plan's expected amendment, strict-modes-refusal-exit/1 gains adopt's start
decision/home-tree-check-before-log-init · apply — check 5: the plan's expected amendment, read in the code
decision/home-tree-check-before-start-log · apply — check 5: the same, read in the code
decision/instance-check-before-log-init · apply — check 5: adopt's trusted → check_instance before open_wrapper_log, read in the code
contract/exit-1-detail/session-live · apply — check 1: session-live no longer revive's alone
contract/run-exit-1-detail/session-live · apply — check 1: the same, run's role file
decision/session-live-check-fails-open · apply — check 1: adopt shares the fail-open read
contract/diag-subject/agents-probe · apply — check 1: adopt's agents read writes run's agents-probe pair
contract/cli-internal-error/human-line · apply — check 5: the silent list gains adopt's start
decision/launch-line-untraced · apply — check 1: adopt writes the launch line through run::start
decision/lineage-line-untraced · apply — check 1: adopt writes the lineage line through run::start
decision/adopt-start-logs-as-run · apply — check 5: the plan's expected amendment, adopt logs as process run
contract/process-exit-detail/exit-1-adopt · apply — check 1: adopt's exit-1 process-exit detail
decision/adopt-preflight-order · reject — check 2: the fact's home is layout-templates contract/viola-adopt/preflight-order
decision/adopt-session-id-never-logged · apply — check 1: the report's Coverage line holds the id absent from every line (assert_agents_read)

ops-a11y-plan.md
contract/start-refusal/home-strict-modes · apply — check 5: the plan's expected amendment
contract/start-refusal/socket-dir · apply — check 5: the plan's expected amendment
decision/strict-modes-refusal-before-any-write · apply — check 5: the plan's expected amendment
contract/json-fault/internal-error · apply — check 5: the plan's expected amendment
contract/tui-passthrough/entry · apply — check 1: adopt enters the passthrough through run's pump
decision/socket-dir-refusal-before-any-frame · apply — check 1: adopt shares run's socket-dir refusal
decision/claude-agents-read-fails-open · apply — check 1: adopt shares the fail-open read
contract/cli-refusal/unable-line · apply — check 1: adopt's two own pairs are the one-line form
contract/cli-fixed-message/form · apply — check 1: the same
contract/log-identity/process · apply — check 1: adopt logs as process run
contract/start-json-refusal/document · apply — check 1: adopt's --json refusals one document each
decision/adopt-session-live-pair · reject — check 2: the fact's home is design-system contract/viola-adopt-refusals/session-live

ops-main-security-plan.md
decision/adopt-signals-no-foreign-process · apply — check 5: the plan's expected amendment, W1's arm a row of its own · word: delegate overseer, 2026-11-24
decision/start-checks-home-tree-before-log-opens · apply — check 5: the directive's named strict-modes list read against the code (checked_exe → check_home_tree before open_wrapper_log)
decision/start-checks-socket-dir-before-bind · apply — check 5: the same, run::start's socket-dir check
decision/start-writes-trust-record-flag-only · escalate → Q2 — check 1: boundary widening (rul-r74hshif): adopt's start runs run's trust step (run.rs trust_folder) on the caller's folder; outside his recorded answers · resolved: apply (Q2) — delegate overseer, 2026-11-24, inputs#I7: Q2: option 1, land the ten sets PROVISIONAL on the delegate word, each naming adopt's start
decision/nothing-written-under-claude-home · escalate → Q2 — check 1: the same widening, its exception · resolved: apply (Q2) — delegate overseer, 2026-11-24, inputs#I7: Q2: option 1, land the ten sets PROVISIONAL on the delegate word, each naming adopt's start
decision/no-writes-under-claude-home · escalate → Q2 — check 1: the same widening, its exception · resolved: apply (Q2) — delegate overseer, 2026-11-24, inputs#I7: Q2: option 1, land the ten sets PROVISIONAL on the delegate word, each naming adopt's start
decision/viola-writes-nothing-in-claude-dir · escalate → Q2 — check 1: the same widening, its exception · resolved: apply (Q2) — delegate overseer, 2026-11-24, inputs#I7: Q2: option 1, land the ten sets PROVISIONAL on the delegate word, each naming adopt's start
decision/credential-strip-by-name-only · escalate → Q2 — check 1: the same trust step reads CLAUDE_CONFIG_DIR's value when kept · resolved: apply (Q2) — delegate overseer, 2026-11-24, inputs#I7: Q2: option 1, land the ten sets PROVISIONAL on the delegate word, each naming adopt's start
decision/env-not-config-channel · escalate → Q2 — check 1: the same · resolved: apply (Q2) — delegate overseer, 2026-11-24, inputs#I7: Q2: option 1, land the ten sets PROVISIONAL on the delegate word, each naming adopt's start
decision/no-stored-secrets · escalate → Q2 — check 1: the same · resolved: apply (Q2) — delegate overseer, 2026-11-24, inputs#I7: Q2: option 1, land the ten sets PROVISIONAL on the delegate word, each naming adopt's start
decision/r8-claude-env-strip · escalate → Q2 — check 1: the same · resolved: apply (Q2) — delegate overseer, 2026-11-24, inputs#I7: Q2: option 1, land the ten sets PROVISIONAL on the delegate word, each naming adopt's start
decision/session-id-argv-closed-shape · escalate → Q1 — check 1: boundary widening (rul-r74hshif, rul-p2zl2kgl): an operator-typed id crosses onto the child's argv (W2), its answer a delegate's · resolved: apply (Q1) — delegate overseer, 2026-11-24, inputs#I7: Q1: option 1, land it PROVISIONAL on the delegate word (fork W2)

ops-main-obs-plan.md
contract/start-trust-log/trust-record · escalate → Q2 — check 1: the same trust step writes its trust-record line on adopt's start · resolved: apply (Q2) — delegate overseer, 2026-11-24, inputs#I7: Q2: option 1, land the ten sets PROVISIONAL on the delegate word, each naming adopt's start

ops-main-layout-templates.md
contract/viola-help/verb-groups · apply — check 5: the stale setup list left by the layout proposer's split, read against src/cmd/mod.rs's HELP
contract/cli-catch-site/internal-error · apply — check 7: the proposer's heir read stands; the row's promise stands under contract/cli-internal-error/human and /json

ops-main-architecture.md
decision/silent-roles-at-internal-error-catch · apply — check 7 cascade: the row's name still listed run, revive, mcp and hook after its rule gained adopt's start
decision/env-strip-names-only · escalate → Q2 — check 1: the same trust step's CLAUDE_CONFIG_DIR read, a further row of Q2's class found by the cascade sweep · resolved: apply (Q2) — delegate overseer, 2026-11-24, inputs#I7: Q2: option 1, land the ten sets PROVISIONAL on the delegate word, each naming adopt's start
decision/env-strip-reads-names-only · escalate → Q2 — check 1: the same trust step's CLAUDE_CONFIG_DIR read, a further row of Q2's class found by the cascade sweep · resolved: apply (Q2) — delegate overseer, 2026-11-24, inputs#I7: Q2: option 1, land the ten sets PROVISIONAL on the delegate word, each naming adopt's start
contract/frame-verb-exit-detail/socket-dir-refused · apply — check 1 cascade: adopt's start shares run's socket-dir refusal (run.rs refused_with)


ops-main-security-plan.2.md
decision/exec-commands-absolute-pinned-path · apply — check 1 cascade: adopt's start pins viola and rewrites plugin/ through run::start

ops-main-obs-plan.2.md
decision/r8-strip-logged-as-names-only · escalate → Q2 — check 1: the same trust step's CLAUDE_CONFIG_DIR read, a further row of Q2's class found by the cascade sweep · resolved: apply (Q2) — delegate overseer, 2026-11-24, inputs#I7: Q2: option 1, land the ten sets PROVISIONAL on the delegate word, each naming adopt's start
decision/env-vars-not-configuration · escalate → Q2 — check 1: the same trust step's CLAUDE_CONFIG_DIR read, a further row of Q2's class found by the cascade sweep · resolved: apply (Q2) — delegate overseer, 2026-11-24, inputs#I7: Q2: option 1, land the ten sets PROVISIONAL on the delegate word, each naming adopt's start
contract/socket-dir-refused-exit/codes · apply — check 1 cascade: adopt's start shares run's socket-dir refusal
contract/diag-detail/socket-dir-refused · apply — check 1 cascade: adopt's start shares run's socket-dir refusal
