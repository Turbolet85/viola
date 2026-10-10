## Relevant amendment history

- **2026-09-28-cli-output-tokens** — §Surface: cli → Toolkit: clap 4.6.7 is built without its `color` feature, so
  help and usage are plain in every mode and every styled byte is viola's own SGR module's (clap's `color` had put
  bold headers into `--help` and let `CLICOLOR_FORCE` push SGR into a pipe). Bears on any flag, subcommand or help
  text this chunk adds (a remover for scope item 6, a count on a CLI line): it prints plain, and the gate
  `cli_output_plain` (green on three OSes at that wrap) already judges it. The extract's "no colour under `--json`,
  non-TTY" criterion rests on this.
- **2026-09-28-capability-ledger-and-viola-verify** — §Surface: cli Component Patterns (`verify`) and the exit-1 row:
  the step counter became one static stdout line per ledger row with the `stamped <version>  N pass  0 fail` summary
  as the last stdout line; exit 1 gained verify's refusals, each `unable: ...` with its own hint, and a failing row
  prints no stderr word. This is the origin of the rule the extract cites for scope item 6: a leftover probe dir
  found or removed gets no step line (a step line is a ledger row, and the count grows only as a row lands) and no
  line after the summary; a refusal by the remover would be a new exit-1 phrase, which is an amendment of that row.
- **2026-09-29-t15-07-57-wrap** — registry migration (U35): the Decisions Log left the body for
  `design-system-amendments-archive.md` (10 entries, all 2026-09-24), with its in-force items lifted into §Brand
  Identity, §Color Palette and the web-spa sections. Bears twice: (a) the §Color Palette lift is the second deviation
  note behind the extract's "colour is never the carrier" pattern (holder colour marks kind; the only state
  exceptions are the `stale` fill and the cocked DIALOG cell, so a skip or recovery state gets neither); (b) the
  Navigation Pattern lift (content jumping met for order, not pixel position) is the sentence that names a nonzero
  `skipped` adding an ATIS line. Trap: a "Decisions Log" citation no longer resolves in the body; the original
  `skipped` decisions (the ATIS `<dl>` with `skipped` as a `<dt>`, Z9) are in the archive, and no amendment in this
  history has touched the `Skipped::zero` / `Skipped::nonzero` rows since: the three names `unknown_kinds` /
  `unknown_fields` / `torn_lines` stand as first planned.
- **2026-10-01-t12-19-55-wrap** — web-spa toolkit moved from Lit to React + TypeScript with the bundler, React
  version, embedding and CSP all OPEN, owned by the frontend-toolchain entry (Epoch 8's head); until it lands the
  page has no JS build step; the event tape's text bindings are JSX text children. Bears on scope item 3's
  "surfaced": the web surfaces the extract names for this chunk's counts (the nonzero `skipped` box, the
  `skipped  1 unknown kind` tape line at its position, the `state-unreadable` strip) cannot be built or asserted
  here, so the chunk owes them only a result shape that does not foreclose them. Standing rule from the same entry:
  relaxing a ban or a CSP directive is a boundary widening the founder rules live.
- **2026-10-04-wait-and-last** — §Surface: cli → Component Patterns 3: the result lines gained `session-end` (every
  non-dialog kind takes the `<kind>  <name>  <time>  cursor <n>` form) and `dialog unknown` for an event without a
  `dialog_id`; message mode prints every control character but LF and TAB as `\xHH`, never stripped; `--json` stays
  serde-escaped. The chunk that landed the reader `read_from` this chunk heals behind. Bears as: the fixed forms
  the extract says carry no skip or recovery word (a new one is a named addition, as this entry was); the precedent
  for printing an absent field as `unknown` rather than a filler; and a trap for scope item 3, since "every other
  non-dialog kind" takes the `turn-ended` form, so a kind the reader starts counting as unknown must not also reach
  that line under a guessed name.
- **2026-10-05-real-cli-verify-probes** — verify's counter went to `[01/10]`; `viola verify --help` gained one static
  ASCII paragraph that names no path; the exit-1 fixed sentence became the named refusal
  `a recorded fixture is not clean: <file> <code>` with closed codes and never the content (the founder's live
  ruling). Precedent for scope item 6: verify's human text names no path, and where a refusal had to name a thing it
  named a file plus a closed code by founder ruling, not by a plan's own choice. A remover line that would name a
  probe dir (its name carries a pid) falls under the same ruling path and under the extract's no-path, no-pid ban.
- **2026-10-05-dialog-rows-and-re-probe** — the counter went to `[01/14]`, the four dialog rows named with their
  words; "the pattern (static appended lines, uncoloured `fail`) is unchanged". Confirms the invariant scope item 6
  must keep: the counter's denominator moves only when a ledger row lands, and the line form never changes.
- **2026-10-06-local-command-and-paste-framing-rows** — the counter stands at `[01/17]` ("seventeen rows today, the
  last seven the dialog rows and the framing rows"), summary example `stamped 2.1.287  17 pass  0 fail`. This is the
  count in force (it matches CLAUDE.md's "seventeen rows gate `cli_verified`"). This chunk lands no ledger row, so
  the plan leaves `[01/17]` and the summary untouched; a plan step that changes either number is drift, and a count
  amended in a master carries its rule beside it (the playbook rule of the last wrap).
- **2026-10-06-local-command-send-outcomes** — §Brand Identity (the Readback anchor and signature element): three
  sentences that named the matching `prompt-submitted` as the only thing that fills the box were reworded to name
  `/clear`'s post-condition (a `session-start` with cause `clear`) beside it; no new word was added. Precedent in
  the section the extract leans on: a second source for a state was absorbed by rewording "only" sentences, with
  no new vocabulary. If replay becomes a second source of a field a surface prints, look for "only" sentences of
  that kind; and a replay that derives the read-back or the session id from the log has two confirming events to
  honour, not one.
- **2026-10-08-first-live-test-and-self-drive** — §Surface: cli → Component Patterns 2: the `input-not-ready` hint
  was reworded to the founder's wording, chosen among three shown at the chunk's planning. Precedent for scope
  item 4: a hint line for a refusal is the founder's wording put to him at planning, so a newer-peer refusal that
  needs a hint goes to the review card with options, never minted at implement. No other hint, exit code or
  refusal moved in that entry.
- **2026-10-09-epoch-3-cleanup** — the newest precedent for a new refusal: `empty-text` took three edits at once,
  a `RefusalDetail::empty-text` row in §Color Palette's Domain status colors (the three tokens of its neighbour),
  a hint in Component Patterns 2 (the founder's wording, relayed) and the detail appended to the exit-13 row of
  Exit-code phraseology. This is the exact shape a newer-peer (version) refusal would take if it is a new detail or
  a new exit word: the extract notes the exit-code table holds no row for a version refusal. Trap stated in the
  entry: "No detector covers this vocabulary, so the plan's own list raised it", so the plan must itself list every
  design-system row, hint and exit-row this chunk's refusal or count vocabulary would add; the wrap's detectors
  will not catch an omission. The same holds for `state-recovered` and its three details, which have no domain
  status row and stay obs-only unless the plan names a row.
