# The closed list, pins first (step 5a, 2026-10-08)

The seven reply shapes the founder ratified (inputs#I17) join `is_reply` in `src/run/wheel.rs`, each by its exact
grammar after `CSI`. All fields are decimal digits, no intermediate byte, the field count exact.

| shape | grammar | as foot wrote it |
|---|---|---|
| DSR status | no prefix, one field equal to `0`, final `n` | `CSI 0 n` |
| colour scheme | `?`, two fields, the first `997`, the second `1` or `2`, final `n` | `CSI ? 997;1 n` |
| window size | no prefix, three fields, the first `4`, final `t` | `CSI 4;675;1260 t` |
| cell size | no prefix, three fields, the first `6`, final `t` | `CSI 6;15;6 t` |
| text area | no prefix, three fields, the first `8`, final `t` | `CSI 8;45;210 t` |
| in-band resize | no prefix, five fields, the first `48`, final `t` | `CSI 48;45;210;675;1260 t` |
| modifyOtherKeys reply | `>`, two fields, the first `4`, final `m` | `CSI > 4;1 m` |

## The two readings

| order | tree | command | reading |
|---|---|---|---|
| 1 | the tests alone, the producer untouched: `REPLIES` at 23 strings (15 and the eight new), the new rstest `classifier_terminal_reply_is_not_editing` of eight named cases, and 27 negative controls added to `classifier_typing_is_editing` | `bash scripts/agent-run.sh run --unit` (08:33Z) | exit 1, `"ok":false`; suite `nextest-unit` 1396 passed, 9 failed, 0 skipped |
| 2 | the producer changed too (`is_reply`: four new arms and one `first` binding; its doc comment) | the fence's unit entry, through the gate tool (08:34Z) | exit 0, `"ok":true`; suite `nextest-unit` 1405 passed, 0 failed, 0 skipped |

The forecast was 9 failed in the first run and all passed in the second (1370 and the new cases). Both held: 1405
is 1370 and 35 new cases (8 reply cases and 27 negative controls).

The nine that failed in the first run, all in `run::wheel::tests` of the root bin:

- `classifier_terminal_reply_is_not_editing::case_1_dsr_status`
- `classifier_terminal_reply_is_not_editing::case_2_colour_scheme_dark`
- `classifier_terminal_reply_is_not_editing::case_3_window_size`
- `classifier_terminal_reply_is_not_editing::case_4_cell_size`
- `classifier_terminal_reply_is_not_editing::case_5_text_area`
- `classifier_terminal_reply_is_not_editing::case_6_in_band_resize`
- `classifier_terminal_reply_is_not_editing::case_7_modify_other_keys`
- `classifier_terminal_reply_is_not_editing::case_8_colour_scheme_light`
- `classifier_every_listed_reply_is_not_editing_whole_and_split`

One red per shape (two for the colour scheme, dark and light), and the whole-and-split test, which reads the
same eight strings.

## The negative controls

All 27 passed in the first run, on the tree where the producer was untouched (they are typing at HEAD), and
passed again in the second, with the producer:

- the nearest human keys: `n`, `t`, `m`; `ESC n`, `ESC t`, `ESC m`; the kitty key events `CSI 110 u`,
  `CSI 116;5 u`, `CSI 109 u`;
- one field or one prefix away from each shape: `CSI 1 n`, `CSI 3 n`, `CSI n`, `CSI 0;0 n`; `CSI 997;1 n`,
  `CSI ? 996;1 n`, `CSI ? 997;3 n`, `CSI ? 997 n`; `CSI 4;675 t`, `CSI 9;45;210 t`, `CSI ? 8;45;210 t`,
  `CSI 8;45;210;1 t`; `CSI 48;45;210;675 t`, `CSI 49;45;210;675;1260 t`; `CSI 4;1 m`, `CSI > 5;1 m`,
  `CSI > 4 m`, `CSI ? 4;1 m`.

## What changed in the file

Nothing but `is_reply`, its doc comment and the test tables: no state of the classifier, no bound, no other
arm. The values are compared as bytes: a field of `00` is not the DSR status, and `0997` is not the colour
scheme. `cargo fmt --all --check` exits 0.

## The rebuilt product binary (step 5b)

The block's release-check entry cleared the product package's cached build and built it again from the fixed
tree (08:36Z): exit 0, last line `release-check: viola only`, its artifact fresh.

| binary | sha256, first 16 hex digits |
|---|---|
| `target/release-check/release/viola` before the fix (the plan's baseline) | `8d625c7bd9d2e873` |
| the same path, rebuilt by this firing | `2bf1b8ab19e16c8e` |

Step 5c's reply probe and start 7 use the rebuilt binary; their rows carry the same digits.

The block in the same firing: the fifteen standing entries green (fmt, clippy, the unit entry at 1405 passed,
both census reads, the default selection, the preservation guard, the smoke with G2 and G4, the native
`pre-push`, the product build), the pinned red half of the reply probe green, and the four readers of records
not yet written red by their letter.
