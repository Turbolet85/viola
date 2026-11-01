# Classified — security · data-protection.b1

## Answers
data-protection#1.1 · yes · the Unix 0700 directory and 0600 file modes of `~/.viola/` are what the home strict-modes check (`check_path` write rule, `check_instance` read rule, any group or other bit) and `board::scan`'s home check enforce
data-protection#2.1 · no · pinned-copy placement by `place_private_new` in `pin_bytes`; the chunk reaches no pin placement
data-protection#3.1 · no · `place_private_new`'s `AlreadyExists` handling for a present pinned-copy name; outside the chunk's units
data-protection#4.1 · no · lost-placement re-hash in `pin_bytes`; outside the chunk's reached units
data-protection#5.1 · no · release-build key collision of `bin/<key>/` and `plugin/<key>/` folders; no unit the chunk touches
data-protection#6.1 · no · `link` / `unlink` lines in `events.ndjson`; the chunk changes neither verb nor its log
data-protection#7.1 · no · wrapper process-log rule for the `link` / `unlink` verbs; the chunk changes no link method and adds no name-bearing line
data-protection#8.1 · no · `check_diagnostics` on the `diagnostics/` folder and its role or detail files is not a path the chunk names; its strict-modes scope is the home, instance and socket directories
data-protection#9.1 · no · repository fixture scrub and recording; the chunk records no fixture
data-protection#10.1 · no · `verify`'s refusal codes on its `process-exit`; the chunk's new refusal codes are run and client surfaces, not `verify`'s set
data-protection#11.0 · no · Minimal-tier exception and `verify`'s accepted risks; the chunk's cleanup hypothesis is its own measurement in harness `cleanup`, which this text does not state
</content>
