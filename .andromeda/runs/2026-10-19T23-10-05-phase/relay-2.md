# P3 readings outside this repository (the orchestrator's own reads, 2026-10-19T23:2xZ, read-only)

Recorded instead of copying the sources: the two CLI transcripts carry the CLI's own `api-request-blob` lines (its
system prompt and context), which do not belong in this repository; the CLI bundle is 285 MB.

## R1 — the 2.1.302 verify's Run B transcript

Source: `~/.claude/projects/-home-turbolet-dev-projects-viola--viola-verify-3090040/685f6ab3-97d4-4377-86f9-628e823adc83.jsonl`
(523 410 B, 57 lines, last write 18:21:40Z; the only file in that dir). Read by a scratch script listing each line's
`type`, and for `user` / `system` lines the timestamp and the first 70 characters.

- `user` turns: exactly 2 — line 4, 18:18:48Z, `viola verify probe: reply with the single word ok`; line 40, 18:18:51Z,
  `\n\n<pasted_content id="576c">\nviola verify probe: this message is one long synthetic paste …` (the long paste).
- `system` lines: `stop_hook_summary` + `turn_duration` at 18:18:50Z and at 18:18:53.356Z (the long paste's turn end).
- No tag-like paste, no newline paste, no `/compact`, no `/clear`, no `compact_boundary`, no second session file.
- Type counts: attachment 16, mode 5, permission-mode 5 (`auto`), atis-latch 5, last-prompt 4, system 4,
  api-request-blob 3, ai-title 3, assistant 3, file-history-snapshot 2, user 2, api-request 2, cost-state 2,
  api-request-shape 1.
- Run C's project dir `…--viola-verify-3090040-dialogs` was created at 18:20:55Z, Run D's `…-plan` at 18:21:17Z:
  Run B's next act after the long paste's turn end came ~122 s later.

## R2 — the 2.1.296 verify's Run B transcripts (the same probe, 24/24)

Source dir: `~/.claude/projects/-home-turbolet-dev-projects-viola--viola-verify-635187/` (three files: the session,
its `-backup-<n>` copy, the `/clear` session). Same script.

- Long paste turn end 11:09:44.417Z → the tag-like paste's `user` turn 11:09:49.303Z (4.9 s) → its end 11:09:50.676Z →
  the newline paste 11:09:51.017Z (113 chars, one LF) → its end 11:09:52.465Z → `/compact` 11:09:52.807Z →
  `compact_boundary` 11:09:56.432Z → the `/clear` session's `<command-name>/clear</command-name>` 11:09:56.778Z.

## R3 — the overseer's home stamp, `~/.viola/ledger/stamps.json` (read, not changed)

- `2.1.296`: verified_at 2026-10-19T11:10:29.012Z, 24/24 pass; typed_probe ready_settle_ms 1085, turn_settle_ms 321,
  prompt_latency_ms 66, max_turn_gap_ms 265, compact_latency_ms 3648.
- `2.1.302`: verified_at 2026-10-19T18:21:41.189Z, 20/24 pass; typed_probe ready_settle_ms 1058, turn_settle_ms 317,
  prompt_latency_ms 50, max_turn_gap_ms 312, compact_latency_ms null.

## R4 — the 2.1.302 CLI bundle, `~/.local/share/mise/installs/claude/2.1.302/claude` (285 793 528 B)

Read by a scratch script printing the bytes around a needle (mmap, read-only). Minified names are module-local and
repeat across the bundle.

- 2.1.296 (`…/2.1.296/claude`): `if(Me[21]===S)Ho=r(n,{dimColor:!0,children:[A1," for agents"]})` — drawn unconditionally.
- 2.1.302, offset ~267002127: `function OH(h){let ze=w(36),{available:C,restingHintExpired:L}=h,…` and, inside it,
  `if(!le||De===void 0||De===0){let so;if(ze[22]!==L)so=L?null:r(n,{dimColor:!0,children:[kW," for agents"]}),…return so}`
  — the `" for agents"` hint is drawn only while `restingHintExpired` is false.
- 2.1.302, offset ~267024209: `ur=OH({available:ze&&_e&&!Bo&&!le,restingHintExpired:V((ks)=>ks.agentsHintExpired===!0)})`.
- 2.1.302, offset ~267786569: `var sIo=1e4;function xxe(){let h=Ht(gH,iIo),C=Hr(sIo,h),L=Un();P(()=>{L((H)=>(H.agentsHintExpired??!1)===C?H:{...H,agentsHintExpired:C})},[C,L])}function iIo(){return Fc()!=null}`.
- 2.1.302, offset ~233899080: `function Fc(){return n().surfaceCapabilities.attacherCaps()}` · `function gH(e){return n().surfaceCapabilities.attacherCapsChanged.subscribe(e)}`.
- 2.1.302, offset ~265544835: an incoming message `{type:"attacher-caps"}` → `handleAttacherCaps(e){if($Go(e.caps),bgn(!0),e.caps)…}`.
- Twelve `function Hr(` definitions exist in the bundle; which one `xxe` calls was not resolved.

## R5 — ci#39321117796 attempt 1, `test (macos-latest)`, its artifacts `diag-macos-latest` / `harness-macos-latest` / `junit-macos-latest` (created 20:47:16–18Z)

Downloaded to `$TMPDIR/{scratchpad}/macos-a1/`; job log by `gh api … /actions/jobs/118295119002/logs`.

- The two failing tests ran beside `cross_list_rows_style_each_span_beside_its_word_on_a_terminal` and
  `cross_list_rows_truncate_an_unwrapped_name_only_on_a_terminal_too_narrow_for_it` (all four SLOW > 10 s) and
  `cli_verify verify_window_without_screens_fails_every_interactive_row` (PASS 47.794 s); both FAIL at 16.698 s /
  16.835 s, `tests/support/watch.rs:54:13`, `wrapper builder not ready`, report `starts w0 c0 r0 snapshot false beat
  false` then `starts w1 c0 r0 snapshot false beat false`.
- Homes `viola-test-yyRGqm` and `viola-test-7lGEmW`: `diagnostics/run-builder.ndjson` holds ONE line each —
  `process-start{subject:"self"}` at 20:44:56.288Z (pid 16375) and 20:44:56.328Z (pid 16377); no version-probe line, no
  claude-child line, no process-exit.
- Every other `run-builder.ndjson` of 20:44:39–45Z: `process-start{self}` → `process-start{version-probe}` within
  33–47 ms (e.g. `viola-test-4Lt5GD`: 20:44:43.976Z → 20:44:44.019Z → exit 65 ms → claude-child 20:44:44.102Z).
