# design · picked accounting

## Picked
brand-identity#2.1 · not in scope · standing S3 · the colour decision order, with colour spent only on `DIALOG` (amber) and `stale` (dim) and no redraw or spinner
color-palette#1.4 · carried · the handoff blue it reserves for `link` / `unlink` names in tape lines is what the new CLI result lines must not paint
color-palette#2.4 · not in scope · Raised-3 is a reserved 0.2.0 frontend surface for the brake confirmation; the verbs print one plain line and draw no surface
color-palette#4.3 · not in scope · the semantic colour of an `unable` refusal is a 0.2.0 frontend row; 0.1.0 renders no page and the verbs' refusals are uncoloured CLI text
color-palette#5.42 · not in scope · the null-detail treatment is a 0.2.0 frontend domain status row; the CLI's detail-less exit-21 line is ruled by the exit-code table
typography#1.3 · carried · a logged link name re-parsed through `ViolaName::try_new` prints exactly as typed, never text-transformed
typography#1.7 · not in scope · the Data web type role (face, size, tabular figures) is a 0.2.0 frontend input; the CLI prints in the user's terminal font
iconography#1.1 · carried · the link result lines print the pair with ASCII `->`, no inbound form, and links print only as verb output, never a `viola list` column
iconography#1.3 · not in scope · standing S1 · two spaces between columns in every new human line; the link lines carry no ` - ` field separator
surface#1.5 · carried · the link and unlink result lines are plain at every colour depth, with no handoff-blue SGR
surface#2.1 · carried · the verbs' refusals take the two-space `unable  <name>  <reason>  <detail>` form, not the fixed `unable:` form reserved to the listed verbs
surface#5.1 · carried · the verbs' exit-21 refusals print the per-cause `instance-unreachable` hints, name no `viola release`, and give faults no hint
surface#6.1 · carried · the server-verification hint's list of frame-writing verbs is the list the two verbs join when the form named at P4 has them take the server check
surface#7.1 · carried · the scope names the macOS recorded gap the verbs inherit through `client::connect`
surface#8.1 · not in scope · the `release --budget` line is another verb's output; its `v1-13` half landed and only keeps holding
surface#11.1 · carried · the verbs' exit-0 result goes to stdout, and under `--json` is one `{"v":1,"ok":{…}}` document
surface#11.2 · carried · the verbs' exit 1 is only the fixed `error: internal error`, with no `--json` document and no new fixed-message refusal
surface#11.3 · carried · a bad `link` / `unlink` argv is clap usage, exit 2
surface#11.9 · carried · a wrapper fault on a `link` / `unlink` frame is `error: wrapper fault  <code>`, exit 20, a typed code the `-32602` refusals may land on
surface#11.10 · carried · the verbs' failed liveness pre-check or server identity check prints `unable  instance-unreachable` and the cause's hint, or the `--json` document with stderr empty
surface#12.1 · carried · the exit-21 server-verification cause's verb list is the one the two verbs join under the form named at P4
surface#13.1 · carried · `viola --help` gains the `handoff:  link, unlink` row between `wheel:` and `setup:` once the verbs are served
