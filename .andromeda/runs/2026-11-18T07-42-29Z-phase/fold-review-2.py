import sys

p = sys.argv[1]
t = open(p, encoding="utf-8").read()

def sub(old, new):
    global t
    assert t.count(old) == 1, old[:80]
    t = t.replace(old, new)

sub("It names none of the 21 floor ids (the list in Implementation notes):",
    "It names none of the 21 floor ids (`shim-resolution`, `spine-hooks`, `session-start-fields`,\n"
    "         `prompt-verbatim`, `stop-message`, `largest-hook-payload`, `modal-signature`, `input-box-signature`,\n"
    "         `quiet-period`, `confirm-window`, `question-answer`, `plan-approve-revise`, `question-notes`,\n"
    "         `dialog-concurrency`, `long-paste-wrapper`, `tag-escaping`, `local-command-clear`, `paste-newline`,\n"
    "         `resume-session`, `agents-join`, `local-command-compact`, S0R-2's floor atoms):")

sub("- `src/cmd/verify.rs` — on a failed verify under `--record`, the first-screen files",
    "- `src/cmd/verify.rs` — the hidden `--record-first-screens` flag; on a failed verify under `--record` with it, the first-screen files")

sub("""- `rec-wordfilter` — `src/cmd/verify.rs`: the first-screen writer's word filter removed (every run with a kept reading
  written, `box` and A's `trust` included); case `verify_record_on_a_failed_run_writes_its_first_screen_only`, fired
  on entry 5 (its filter holds `binary(cli_verify)`; it builds the workspace, so the clone holds `viola` and the
  fake). With the guard gone the failed verify writes `FirstScreen.a.json` … `FirstScreen.f.json` (E's too when its
  reading is kept), so the case's «exactly `FirstScreen.f.json`» fails — measured at implement.""",
"""- `rec-hiddengate` — `src/cmd/verify.rs`: the hidden option's gate removed (the first-screen writer runs on every
  failed `--record` verify, the flag or no flag; inputs#I13 item 2); case
  `verify_record_on_a_failed_run_writes_no_screen_without_the_rig_option`, fired on entry 5 (its filter holds
  `binary(cli_verify)`; it builds the workspace, so the clone holds `viola` and the fake). With the guard gone a plain
  failed `--record` verify writes `FirstScreen.f.json` and creates the record dir, so the case's «the record dir does
  not exist» fails — measured at implement.""")

sub("""- (tests, Revision 3) A verify writes first-screen files only under `--record` and only when it failed, one
  `FirstScreen.<x>.json` per run whose first word is not `box` (nor Run A's `trust`), its rows whole through the
  scrub and the check: the failed fake verify writes exactly `FirstScreen.f.json` (`run` `f`, `screen` `trust`)
  (`verify_record_on_a_failed_run_writes_its_first_screen_only`); the word filter's table""",
"""- (tests, Revision 3) A verify writes first-screen files only under `--record` WITH the hidden
  `--record-first-screens` and only when it failed, one `FirstScreen.<x>.json` per run whose first word is not `box`
  (nor Run A's `trust`), its rows whole through the scrub and the check: with the option the failed fake verify writes
  exactly `FirstScreen.f.json` (`run` `f`, `screen` `trust`)
  (`verify_record_first_screens_on_a_failed_run_writes_its_first_screen_only`); a plain `--record` on the same failed
  verify writes nothing, its record dir absent as before this chunk
  (`verify_record_on_a_failed_run_writes_no_screen_without_the_rig_option`); `--help` names no such option
  (`verify_help_names_no_first_screens_option`); the word filter's table""")

sub("- (tests, Revision 3) The V arm passes `--record`, prints",
    "- (tests, Revision 3) The V arm passes `--record` and `--record-first-screens`, prints")
sub("- (tests, Revision 3) The control `rec-wordfilter` holds.",
    "- (tests, Revision 3) The control `rec-hiddengate` holds.")

sub("""- A first-screen file without `--record`, on a passing verify, for a `box` run or Run A's `trust`, or carrying any
  config value, environment value or a path the scrub's check refuses — never (inputs#I11 item 2).""",
"""- A first-screen file without `--record` AND the hidden `--record-first-screens`, on a passing verify, for a `box` run
  or Run A's `trust`, or carrying any config value, environment value or a path the scrub's check refuses — never
  (inputs#I11 item 2, inputs#I13 item 2).
- Any change to what a plain `viola verify --record` writes, the hidden option shown in `--help` or a user doc, or a
  caller of it but the rig's `V` arm — never (inputs#I13 item 2).""")

sub("""- Expected amendments (wrap, Revision 3): security-plan `decision/verify-signature-only`,
  `decision/verify-screen-read-against-literals` and `decision/fixture-scrub-and-refusal` — add-to (an except): on a
  failed verify under `--record`, the first settled screen of each interactive run that did not reach the box (Run A's
  trust dialog excepted) leaves verify whole-row, scrubbed and checked as `--record` scrubs, one `FirstScreen.<x>.json`
  per run into the record dir, nothing else, refused whole on an unclean row or seam. Pattern: the «Boundary
  widening» class (`rul-r74hshif`, `rul-p2zl2kgl`); who answered: the founder himself, «Verify с записью экрана», in the
  overseer's dialog, read 2026-11-18T07:41:36Z, relayed by viola-overseer (inputs#I11 item 1) — his ratification
  citing these words, for the wrap's answer card (the relay's own direction).""",
"""- Expected amendments (wrap, Revision 3): security-plan `decision/verify-signature-only`,
  `decision/verify-screen-read-against-literals` and `decision/fixture-scrub-and-refusal` — add-to (an except): a
  RIG-ONLY, HIDDEN, failed-verify first-screen record of whole scrubbed rows, kept local, never committed — not a
  change to what `--record` writes for a user. Under `--record` with the hidden `--record-first-screens` (absent from
  `--help` and the user docs, passed only by the live rig's `V` arm), a failed verify writes the first settled screen
  of each interactive run that did not reach the box (Run A's trust dialog excepted), its rows whole through the
  payload scrub, every row and join checked, one `FirstScreen.<x>.json` per run into the record dir, nothing else,
  refused whole on an unclean row or seam; the rig copies them into its gitignored kept folder; a plain `--record` is
  unchanged. Pattern: the «Boundary widening» class (`rul-r74hshif`, `rul-p2zl2kgl`); who answered: the founder
  himself, on BOTH answers — «Verify с записью экрана» (read 2026-11-18T07:41:36Z, inputs#I11 item 1) and «Строки
  целиком, только для рига» (read 2026-11-18T08:06:08Z, inputs#I13), each in the overseer's dialog and relayed by
  viola-overseer — his ratification citing these words, for the wrap's answer card (the relays' own direction). The
  hidden flag stands beside the invariant «no CLI flag widens redaction»: it is the founder's own narrowing of his
  ratified widening, named so on the card.""")

sub("""- Revision 3, the lean on whole rows (§Revision 3) is decisive by the word's own ask (inputs#I11 items 2, 4); the
  operator may reject it at review, and then the plan returns to P4.""",
"""- Revision 3, whole rows only for the rig: the founder's own answer at P5 to card 1's one choice (inputs#I13), folded
  at P5 through the review; the card is put again (`p5-card-2.md`).""")

sub("  so the cut would name nothing (§Revision 3, the lean).",
    "  so the cut would name nothing (§Revision 3; the founder's answer inputs#I13 item 1).")

open(p, "w", encoding="utf-8").write(t)
print("folded")
