# Classified — design · color-palette.b3

## Answers
color-palette#5.33 · no · the readback box's drawn open state is not needed, since a link is not a send and carries no readback
color-palette#5.34 · no · the readback box's drawn read state is not needed, since a link is not a send and carries no readback
color-palette#5.35 · no · the readback box's drawn refused strike is not needed, since the verbs refuse by the CLI line under the viola send component's rules
color-palette#5.36 · no · the readback box's drawn unconfirmable state is not needed, since a link is not a send and carries no readback
color-palette#5.37 · no · human-typing is a send refusal reason, and the link verbs refuse only with instance-unreachable or the usage, internal and wrapper-fault codes
color-palette#5.38 · no · budget-paused is a send refusal reason, and the budget gate refuses sends only, so link never raises it
color-palette#5.39 · no · unverified-cli is a send refusal reason, and link answers no dialog and needs no verify stamp
color-palette#5.40 · no · not-delivered is a send refusal reason, and link types no text into the child
color-palette#5.41 · no · unknown is a reason the link verbs never raise, so no opaque detail is printed
color-palette#5.42 · yes · the exit 21 refusal line prints its reason with a null detail, so nothing may follow the reason
color-palette#5.43 · no · manual-pause is the detail of a send refused under viola pause, and the chunk changes no pause path
color-palette#5.44 · no · five-hour is a send budget detail, and the budget gate refuses sends only
color-palette#5.45 · no · seven-day is a send budget detail, and the budget gate refuses sends only
color-palette#5.46 · no · input-not-ready is a send detail for a paste, and link types nothing
color-palette#5.47 · no · no-prompt-submitted is a send readback detail, and a link carries no readback or prompt
color-palette#5.48 · no · turn-running is a send detail, and link does not type into a running turn
color-palette#5.49 · no · unknown-dialog is a send detail, and link answers no dialog
color-palette#5.50 · no · control-character is a paste-text detail, and link names are checked by ViolaName::try_new, not validate_paste_text
color-palette#5.51 · no · empty-text is a send paste detail, and link takes no text
color-palette#5.52 · no · the tape line is a 0.2.0 frontend input (Semantic Colors status rows), and the chunk adds no event kind
color-palette#5.53 · no · the tape line folds into a send, a 0.2.0 frontend input (Semantic Colors status rows)
color-palette#5.54 · no · the tape line is a 0.2.0 frontend input (Semantic Colors status rows), and wait / last lines are untouched
color-palette#5.55 · no · the tape line is a 0.2.0 frontend input (Semantic Colors status rows), and the chunk adds no event kind
color-palette#5.56 · no · the tape line is a 0.2.0 frontend input (Semantic Colors status rows), and the chunk adds no event kind
color-palette#5.57 · no · the tape line is a 0.2.0 frontend input (Semantic Colors status rows), and the chunk adds no event kind
