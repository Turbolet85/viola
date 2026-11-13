@row_set n=1 row=decision/mcp-dependencies cell=rule seen=1 fact=F2 detector=D-arch-decisions
value: viola-mcp depends on viola-channel with its tokio feature, on tokio and on veil through the workspace pin without its toggle feature, and takes no serde.
basis: fct-xc7c6ijy
because: veil was added to viola-mcp through the workspace pin, so the row naming viola-mcp's dependencies must name it.
@row_set n=2 row=decision/recorded-fixtures-scrubbed cell=rule seen=1 fact=F5 detector=none
value: Fixtures recorded by viola verify --record are written only when no row fails, each is scrubbed and hygiene-walked, and an unclean recording, an email-shaped token in any string or key of any payload kind among it, is refused whole, naming the file and the check code, never the content.
basis: fct-yeqzuggx
because: The record scrub now refuses an email-shaped token in every payload kind, not only the screen, so the row of what an unclean recording is must say it.
@row_set n=3 row=contract/human-stderr/refusal cell=promise seen=1 detector=none
value: A refusal writes its fixed `unable:` line and, for every refusal reason but `unknown`, its fixed `hint:` line, as one write.
basis: fct-avspoz2u
because: The report states every refusal but unknown ends with a hint and unknown gets none, so the promise of two lines on every refusal is no longer whole.
@row_add n=1 row=decision/payload-types-derive-redact area=cross-cutting-patterns fact=F1 detector=none
name: Content-bearing payload types derive veil Redact
rule: Every hook, dialog and MCP payload type that carries content derives veil Redact, so a Debug render never reveals its content.
why: The rule was ratified to the measured fact that veil refuses a Redact derive with no redacted field.
guard: normalised_debug_holds_no_prompt_text; input_debug_holds_no_answer_or_text
except: A payload type with no content field derives no Redact, since veil 0.3.0 refuses a Redact derive with no redacted field.
basis: fct-yeqzuggx
because: The hook, dialog and MCP payload types now derive veil Redact where they carry content, a cross-cutting redaction decision no row states.
@row_add n=2 row=decision/strict-modes-refuses-symlink area=occupied-resources fact=F3 detector=none
name: Strict-modes check refuses a symbolic link
rule: The strict-modes check reads each trusted path itself and refuses a symbolic link at the home, instances/, an instance directory, the snapshot or the stamps before its mode verdict, leaving link and target as found.
basis: fct-yeqzuggx
because: The strict-modes check now refuses a link at every trusted path instead of following it, a rule of the trusted home tree no row states.
@row_add n=3 row=decision/private-append-no-follow area=occupied-resources fact=F4 detector=none
name: Private append opens refuse a symbolic link
rule: The owner-only append open of a log or diagnostics file refuses a symbolic link at the path, a refused open being a failed open, and leaves the opened file at mode 0600.
basis: fct-yeqzuggx
because: The private append open now refuses a link at the path and narrows the file it opened, a rule of the appended files no row states.
@row_add n=4 row=contract/human-hint-lines/unverified-cli area=standard-contracts fact=F6 detector=none
boundary: Human hint lines
item: unverified-cli
promise: A human-mode refusal of send or answer for unverified-cli ends with the hint run viola verify for this CLI version, and answer's not-delivered details carry the same hint send holds for that detail.
basis: fct-yeqzuggx
because: send now prints the unverified-cli hint answer already printed and answer's not-delivered details now carry send's hint, a human-line contract no row holds.
