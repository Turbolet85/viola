@carried n=1 label=design-system/color-palette fact=fct-3y3wqz2o
why: Item 6's fold or ratification of `Refused` (and of any enum beside it) must leave the closed RefusalReason set and each variant's printed word exactly as these rows record them, the hex columns staying 0.2.0 frontend inputs this chunk paints nowhere.
quote: | RefusalReason::budget-paused | #1E2124 | #E6D8AE | #E6D8AE | `unable · budget-paused` |
@carried n=1 run=2 label=design-system/color-palette fact=fct-meyzxdzk
quote: | RefusalReason::unverified-cli | #1E2124 | #E6D8AE | #E6D8AE | `unable · unverified-cli` |
@carried n=1 run=3 label=design-system/color-palette fact=fct-cq3onf5q
quote: | RefusalReason::not-delivered | #1E2124 | #E6D8AE | #E6D8AE | `unable · not-delivered` |
@carried n=1 run=4 label=design-system/color-palette fact=fct-j5shg3ew
quote: | RefusalReason::unknown | #1E2124 | #E6D8AE | #E6D8AE | `unable · unknown`, with the detail, a code word or nothing, printed as plain text |
@carried n=1 run=5 label=design-system fact=fct-uxesswnh
quote: These cover every enum in architecture.md's data model and contracts that gets a visual, one row per variant.
@carried n=2 label=design-system/color-palette fact=fct-d4g7hq3o
why: The closed refusal `detail` words must stay exactly as these rows record them through item 6's enum folds or ratifications, the live session's `unable` mirror line reading the same words, with no variant added, renamed or dropped.
quote: | RefusalDetail::null | #1E2124 | #E6D8AE | #E6D8AE | Nothing after the reason (no dangling `·`) |
@carried n=2 run=2 label=design-system/color-palette fact=fct-w34k4ar3
quote: | RefusalDetail::manual-pause | #1E2124 | #E6D8AE | #E6D8AE | `human-typing · manual-pause` ("I have control" spoken via `viola pause`) |
@carried n=2 run=3 label=design-system/color-palette fact=fct-y3q52siw
quote: | RefusalDetail::five-hour | #1E2124 | #E6D8AE | #E6D8AE | `budget-paused · five-hour` |
@carried n=2 run=4 label=design-system/color-palette fact=fct-j5f6s6jh
quote: | RefusalDetail::seven-day | #1E2124 | #E6D8AE | #E6D8AE | `budget-paused · seven-day` |
@carried n=2 run=5 label=design-system/color-palette fact=fct-gvtyin2m
quote: | RefusalDetail::input-not-ready | #1E2124 | #E6D8AE | #E6D8AE | `not-delivered · input-not-ready` |
@carried n=2 run=6 label=design-system/color-palette fact=fct-pwe5ixq2
quote: | RefusalDetail::no-prompt-submitted | #1E2124 | #E6D8AE | #E6D8AE | `not-delivered · no-prompt-submitted` |
@carried n=2 run=7 label=design-system/color-palette fact=fct-67zxdiqq
quote: | RefusalDetail::turn-running | #1E2124 | #E6D8AE | #E6D8AE | `not-delivered · turn-running` |
@carried n=2 run=8 label=design-system/color-palette fact=fct-6e6po4jg
quote: | RefusalDetail::unknown-dialog | #1E2124 | #E6D8AE | #E6D8AE | `not-delivered · unknown-dialog` |
@carried n=2 run=9 label=design-system/color-palette fact=fct-vvwtx5vm
quote: | RefusalDetail::control-character (security plan) | #1E2124 | #E6D8AE | #E6D8AE | `not-delivered · control-character` |
@carried n=2 run=10 label=design-system/color-palette fact=fct-cxjknylz
quote: | RefusalDetail::empty-text | #1E2124 | #E6D8AE | #E6D8AE | `not-delivered · empty-text` |
@carried n=3 label=design-system/color-palette fact=fct-4fgnsert
why: The live session in which a driver drives a wrapped builder through viola (and `v1-53`'s live `viola run`) must count a send only once its matching driver `prompt-submitted` reads it back, never on `turn-ended`.
quote: | EventKind::turn-ended | #1E2124 | none | #E6D8AE | `turn-ended` + first line of `last_assistant_message` as plain text, or `no message` for `null`. Never fills a readback box. |
@carried n=3 run=2 label=design-system/color-palette fact=fct-7xc7ht7t
quote: | EventKind::prompt-submitted | #1E2124 | none | #E6D8AE (#9C9278 if harness) | Folded into the open send line when it is the matching driver prompt.
@carried n=3 run=3 label=design-system fact=fct-juf7qttk
quote: a refused send folds into its send line, never a separate line; `turn-ended` never fills a readback box;
@binding n=1
**color-palette status rows ↔ surface (cli-send, cli-exit-codes)**: the closed RefusalReason / RefusalDetail words the rows record (design-system/color-palette fct-3y3wqz2o) (design-system/color-palette fct-d4g7hq3o) are the same words the CLI readback mirror prints two-space separated as `unable  <name>  <reason>  <detail>` (design-system fct-zolhtb6x) and under `--json` as the refusal object (design-system fct-tj4jq2zv); item 6's folds must keep both sides naming one set.
@binding n=2
**color-palette status rows ↔ architecture data model**: the rows cover one row per variant of architecture's enums (design-system fct-uxesswnh); a fold or ratification of `Refused` recorded in architecture must leave the variant set and this table agreeing, the masters-agree check reading both.
@binding n=3
**color-palette EventKind rows ↔ surface (cli-send, cli-wait)**: the readback a live session reads — `[RB] read back` on the matching `prompt-submitted` (design-system/color-palette fct-7xc7ht7t) and never on `turn-ended` (design-system/color-palette fct-4fgnsert) — is the CLI mirror's (design-system fct-ahmeujab), the `wait` result line `turn-ended  builder  <time>  cursor <n>` (design-system fct-iee6axpq) reporting a turn's end without counting a send.
