# layouts · standing conventions

- S1 · every new viola verify probe prints one plain-ASCII step line in the fixed form, counts in the summary, and names its failing row when it fails · surface-cli
- S2 · every new or reworded human output line is fixed wording with a --json twin in one typed document, results on stdout and the rest on stderr · surface-cli
  contract/send-json/output · viola send --json · typed document — `--json` prints only the typed document (`ok` with `submitted_at` and `cursor`, `ok` with `confirmed:false` and `unconfirmable`, or `refusal` with `detail`) with the typed exit code, no glyph and no hint.
  contract/viola-send-output/json-mode · viola send output · --json — Under --json the mirror is replaced by the typed ok / refusal document.
