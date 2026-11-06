@answer n=1 entity=color-palette#5.33 v=yes
why: Readback::open is the sent-not-yet-read mirror state; the live session's evidence must not count the send until it reads back.
@answer n=2 entity=color-palette#5.34 v=yes
why: Readback::read is the `[RB] read back` mirror state the live session's evidence reads as the only counting state.
@answer n=3 entity=color-palette#5.35 v=yes
why: Readback::refused is the `unable` plus reason mirror state the live session's evidence reads for a refused send.
@answer n=4 entity=color-palette#5.36 v=yes
why: Readback::unconfirmable is not read back, so the live session's counting rule must not take it as delivered.
@answer n=5 entity=color-palette#5.37 v=yes
why: RefusalReason::human-typing is a closed reason word of the Refused fold (item 6) whose human form item 9 keeps and the live evidence reads.
@answer n=6 entity=color-palette#5.38 v=yes
why: RefusalReason::budget-paused is a closed reason word of the Refused fold (item 6) whose human form item 9 keeps and the live evidence reads.
@answer n=7 entity=color-palette#5.39 v=yes
why: RefusalReason::unverified-cli is a closed reason word of the Refused fold (item 6) whose human form item 9 keeps and the live evidence reads.
@answer n=8 entity=color-palette#5.40 v=yes
why: RefusalReason::not-delivered is a closed reason word of the Refused fold (item 6) whose human form item 9 keeps and the live evidence reads.
@answer n=9 entity=color-palette#5.41 v=yes
why: RefusalReason::unknown is the closed fallback reason word of the Refused fold (item 6) and its plain-text detail rule, which item 9 keeps.
@answer n=10 entity=color-palette#5.42 v=yes
why: RefusalDetail::null (no dangling detail after the reason) is part of the closed detail form item 9 keeps and the live unable-plus-reason evidence reads.
@answer n=11 entity=color-palette#5.43 v=yes
why: RefusalDetail::manual-pause is a closed detail word of the Refused fold (item 6) that item 9 keeps exactly and the live evidence reads.
@answer n=12 entity=color-palette#5.44 v=yes
why: RefusalDetail::five-hour is a closed detail word of the Refused fold (item 6) that item 9 keeps exactly and the live evidence reads.
@answer n=13 entity=color-palette#5.45 v=yes
why: RefusalDetail::seven-day is a closed detail word of the Refused fold (item 6) that item 9 keeps exactly and the live evidence reads.
@answer n=14 entity=color-palette#5.46 v=yes
why: RefusalDetail::input-not-ready is a closed detail word of the Refused fold (item 6) that item 9 keeps exactly and the live evidence reads.
@answer n=15 entity=color-palette#5.47 v=yes
why: RefusalDetail::no-prompt-submitted is a closed detail word of the Refused fold (item 6) that item 9 keeps exactly and the live evidence reads.
@answer n=16 entity=color-palette#5.48 v=yes
why: RefusalDetail::turn-running is a closed detail word of the Refused fold (item 6) that item 9 keeps exactly and the live evidence reads.
@answer n=17 entity=color-palette#5.49 v=yes
why: RefusalDetail::unknown-dialog is a closed detail word of the Refused fold (item 6) that item 9 keeps exactly and the live evidence reads.
@answer n=18 entity=color-palette#5.50 v=yes
why: RefusalDetail::control-character is a closed detail word of the Refused fold (item 6) that item 9 keeps exactly and the live evidence reads.
@answer n=19 entity=color-palette#5.51 v=yes
why: RefusalDetail::empty-text is a closed detail word of the Refused fold (item 6) that item 9 keeps exactly and the live evidence reads.
@answer n=20 entity=color-palette#5.52 v=no
why: EventKind::session-start is a tape line outside the chunk's scope; it is no enum fold, readback word or wait line the chunk touches.
@answer n=21 entity=color-palette#5.53 v=yes
why: EventKind::prompt-submitted folds into the open send line as the matching driver prompt, which is the readback::read match the live send evidence depends on.
@answer n=22 entity=color-palette#5.54 v=yes
why: EventKind::turn-ended must never fill a readback box, so the live session's counting of read-back sends must not take a turn-ended event as read back.
@answer n=23 entity=color-palette#5.55 v=no
why: EventKind::question is a dialog tape line outside the chunk's scope; the live readback evidence and the named folds do not read it.
@answer n=24 entity=color-palette#5.56 v=no
why: EventKind::permission is a dialog tape line outside the chunk's scope; the live readback evidence and the named folds do not read it.
@answer n=25 entity=color-palette#5.57 v=no
why: EventKind::plan is a dialog tape line outside the chunk's scope; the live readback evidence and the named folds do not read it.
