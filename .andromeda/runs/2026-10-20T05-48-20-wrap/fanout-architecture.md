proposals:
  - detector: D-arch-decisions
    severity: warning
    section: Established Decisions → [Screen Model]
    change: >-
      The signatures sentence reads `input_box: ["for agents", "(shift+tab to cycle)"]` (two compiled literals;
      `modals` unchanged), and adds the measured reason: on live 2.1.302, at chunk 2026-10-19-newer-claude-cli-rows
      (`evidence/footer-2.1.302.md`), the resting footer at ~15 s and ~25 s has no `for agents` and keeps
      `⏵⏵ auto mode on (shift+tab to cycle)`. The mode word follows the user's permission mode (`manual mode on` in
      2.1.287's recording), so the added literal is the mode-independent `(shift+tab to cycle)`, which appears in no
      other bundle text read (the shift+tab tip, the Skills hint). The rule "a screen is ready when a row contains an
      input-box literal and no row contains a modal literal" is unchanged.
    sidecar: >-
      arch [Screen Model]: SIGNATURES.input_box 1 → 2 literals (`for agents`, `(shift+tab to cycle)`), with the
      measured 2.1.302 resting footer (chunk 2026-10-19-newer-claude-cli-rows).
    rationale: >-
      Report Symbols / APIs: `SIGNATURES.input_box` changed from `["for agents"]` to `["for agents", "(shift+tab to
      cycle)"]`. Spec claims disproved 1 (F1-literal) and Counts F1-literal list architecture.md:45 @c2084/@c2111 as
      stale. Expected amendment 1. The locked decision's literal set now contradicts the code.
    basis: crates/viola-agent-claude/src/screen.rs:41
  - detector: D-arch-decisions
    severity: warning
    section: Established Decisions → [Screen Model]
    change: >-
      The `input-box-signature` row's parenthesis reads: three screens each hold an input-box literal and no modal.
      They are the ready screen, the turn screen and the screen Run B settles on after the long paste's turn
      (`TypedRun::long_turn`, kept by `typed::framing_turns`; `None`, which fails the row, when that turn's `box_wait`
      fails or the turn never ends; never written to `measured`). The row's words are unchanged (`a trusted start
      shows a compiled input-box literal and no modal`). Add the reason: on 2.1.302 the two-screen row passed while the
      literal vanished later. That was the overseer's 2.1.302 verify, reproduced at `20 pass  4 fail` in
      chunk 2026-10-19-newer-claude-cli-rows `evidence/reproduction.md`.
    sidecar: >-
      arch [Screen Model]: input-box-signature reads ready + turn + the long turn's settled screen (TypedRun::long_turn),
      not ready + turn alone (chunk 2026-10-19-newer-claude-cli-rows).
    rationale: >-
      Report Symbols / APIs: `check(InputBoxSignature)` now reads `ready`, `turn` and `long_turn` (ledger.rs:1011-1013).
      Spec claims disproved 2 (F2-ibsig) names architecture.md:45 @c2417 ("the ready and turn screens hold the
      input-box literal") as disproved as a sufficient row. Expected amendment 1.
    basis: crates/viola-agent-claude/src/ledger.rs:1011-1013
  - detector: D-arch-decisions
    severity: warning
    section: Established Decisions → [CLI Version Compatibility]
    change: >-
      "a re-stamp of every stamped version (2.1.287 alone today; …)" reads "(2.1.287 and 2.1.302 today; …)". Add that
      2.1.302 was stamped `24 pass  0 fail` on the Linux dev host by one `viola verify --record` round at chunk
      2026-10-19-newer-claude-cli-rows (the record home's stamp, all 24 rows `pass`, read as `cli_verified:true`).
      It is not measured on Windows (owed to "Windows-only live measurements").
    sidecar: >-
      arch [CLI Version Compatibility]: stamped versions 2.1.287 alone → 2.1.287 and 2.1.302 (24 pass 0 fail, Linux dev
      host, chunk 2026-10-19-newer-claude-cli-rows).
    rationale: >-
      Report Counts F3-stamp "Changed (3)" names architecture.md:101 @c19987 as stale. Outcome, Gated: the record round
      printed `stamped 2.1.302  24 pass  0 fail`, and the record home reads `cli_verified:true`. Expected amendment 1
      ("2.1.302 stamped").
    basis: tests/contract_ledger_probes.rs:69
  - detector: D-arch-resources
    severity: warning
    section: Occupied Resources → Binary, subcommands and exit codes
    change: >-
      The fake agent's option list reads "thirteen argv options (the count's rule is this list), no env, of which nine
      serve verify's interactive runs". "the test-only screen switch `--tag-turn-screen <phase>` (…)" becomes "the two
      test-only screen switches": `--tag-turn-screen <phase>` as before, and `--long-turn-screen <phase>` (with
      `--framing` and `--turn-stop`: after the long paste's turn alone, after its Stop, `Screen.<phase>` of the set is
      drawn in place of `Screen.turn`, in the way `--tag-turn-screen` acts after the tag-like turn; chunk
      2026-10-19-newer-claude-cli-rows; `cfg(feature = "fake-agent")`).
    sidecar: >-
      arch Occupied Resources: fake agent argv options 12 → 13 (verify-serving 8 → 9), adding the test-only
      `--long-turn-screen <phase>` (chunk 2026-10-19-newer-claude-cli-rows).
    rationale: >-
      Report Symbols / APIs and Harness / gate surface: the fake agent gains the test-only `--long-turn-screen <phase>`.
      Counts F4-fakeopt "Changed (3), stale by omission" names architecture.md:315 @c2085. The list counts its own
      options, so both counts move with it.
    basis: src/bin/viola-fake-agent.rs:99
  - detector: D-arch-resources
    severity: warning
    section: Occupied Resources → Repository
    change: >-
      "Committed sets: `2.1.283`, `2.1.287` and `2.1.288`" reads "`2.1.283`, `2.1.287`, `2.1.288` and `2.1.302`". Add
      the `2.1.302` set: 29 files (4 spine payloads + 3 screens + 12 dialog variants + 6 framing variants + 2 status
      line variants + 1 resume variant + 1 agents variant), recorded whole by one `viola verify --record` round at
      chunk 2026-10-19-newer-claude-cli-rows at `24 pass  0 fail` on the Linux dev host. Its `Screen.ready` and
      `Screen.turn` keep one row holding both input-box literals, and its `StatusLine.*` variants keep this
      repository's own `workspace.repo.owner`. The stamped clause reads that `2.1.287` and `2.1.302` are both stamped
      at the relied set by `contract_ledger_probes` (its `STAMPED` set `["2.1.287", "2.1.302"]`). `2.1.288` and
      `2.1.283` stay drift-only.
    sidecar: >-
      arch Occupied Resources → Repository: committed fixture sets gain `2.1.302` (29 files, recorded at 24 pass 0
      fail), stamped by contract_ledger_probes beside 2.1.287 (chunk 2026-10-19-newer-claude-cli-rows).
    rationale: >-
      Report Schema / config: the new recorded fixture set `fixtures/claude/2.1.302/` (29 files). Symbols / APIs:
      `tests/contract_ledger_probes.rs:69` `STAMPED` is `["2.1.287", "2.1.302"]`. The registry at architecture.md:374
      lists the committed sets and names 2.1.287 as the stamped one. The report lists :374 only under F7's unchanged
      walk rule, but its set enumeration and stamped clause are stale by omission. This restates the stamp claim of
      the [CLI Version Compatibility] proposal.
    basis: tests/contract_ledger_probes.rs:69
note: >-
  D-arch-decisions found no new library, runtime or pattern (Dependencies: none). D-arch-resources found no IPC method,
  endpoint, event, port, socket, env var or crate change (report: none). Also seen, with no proposal: the test-only
  `wait_ready` stall report writes `/usr/bin/sample` output to a file under the OS temp dir, and the report does not
  name that file (its home is test-plan, per Counts F5). The `CLAUDE.md:42` "stamped on 2.1.287" leaf is the cascade's
  to re-derive.
