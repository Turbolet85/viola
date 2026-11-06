@answer n=1 entity=color-palette#5.33 v=no
why: the readback open state's hex values and word still hold, since no status-state token moved and Linux output stays byte-identical
@answer n=2 entity=color-palette#5.34 v=no
why: the readback read state's fill, box and word still hold, since the session moved no readback state or token value
@answer n=3 entity=color-palette#5.35 v=no
why: the readback refused strike and its word still hold, since the session retired no readback state
@answer n=4 entity=color-palette#5.36 v=no
why: the unconfirmable open drawing still holds, since the session adds and removes no readback state
@answer n=5 entity=color-palette#5.37 v=no
why: RefusalReason::human-typing is not among the revive or verify codes the session moved, and its line is unchanged
@answer n=6 entity=color-palette#5.38 v=no
why: RefusalReason::budget-paused is not among the codes the session moved, and its line is unchanged
@answer n=7 entity=color-palette#5.39 v=no
why: RefusalReason::unverified-cli is not among the codes the session moved, and its line is unchanged
@answer n=8 entity=color-palette#5.40 v=no
why: RefusalReason::not-delivered is not among the codes the session moved, and its line is unchanged
@answer n=9 entity=color-palette#5.41 v=no
why: RefusalReason::unknown is not among the codes the session moved, and its detail still prints as plain text
@answer n=10 entity=color-palette#5.42 v=no
why: RefusalDetail::null still holds, since the session retired no refusal detail and the empty detail prints nothing
@answer n=11 entity=color-palette#5.43 v=no
why: RefusalDetail::manual-pause still holds, since the session names no change to the manual-pause detail
@answer n=12 entity=color-palette#5.44 v=no
why: RefusalDetail::five-hour still holds, since the session names no change to the budget detail text
@answer n=13 entity=color-palette#5.45 v=no
why: RefusalDetail::seven-day still holds, since the session names no change to the budget detail text
@answer n=14 entity=color-palette#5.46 v=no
why: RefusalDetail::input-not-ready still holds, since the session names no change to the not-delivered details
@answer n=15 entity=color-palette#5.47 v=no
why: RefusalDetail::no-prompt-submitted still holds, since the session names no change to the not-delivered details
@answer n=16 entity=color-palette#5.48 v=no
why: RefusalDetail::turn-running still holds, since the session names no change to the not-delivered details
@answer n=17 entity=color-palette#5.49 v=no
why: RefusalDetail::unknown-dialog still holds, since the session names no change to the not-delivered details
@answer n=18 entity=color-palette#5.50 v=no
why: RefusalDetail::control-character still holds, since the session retired no refusal detail and moved no security-plan wording
@answer n=19 entity=color-palette#5.51 v=no
why: RefusalDetail::empty-text still holds, since the session names no change to the not-delivered details
@answer n=20 entity=color-palette#5.52 v=no
why: EventKind::session-start still holds, since the session retired no event kind and its tape line is unchanged
@answer n=21 entity=color-palette#5.53 v=no
why: EventKind::prompt-submitted still holds, since the session moved no event kind or token and the folding is unchanged
@answer n=22 entity=color-palette#5.54 v=no
why: EventKind::turn-ended still holds, since the session moved no event kind and the no message wording is unchanged
@answer n=23 entity=color-palette#5.55 v=no
why: EventKind::question still holds, since the session names no change to the question dialog line or its tape colour
@answer n=24 entity=color-palette#5.56 v=no
why: EventKind::permission still holds, since the session names no change to the permission dialog line or its tool-name wording
@answer n=25 entity=color-palette#5.57 v=no
why: EventKind::plan still holds, since the session names no change to the plan dialog line or its first plan line
