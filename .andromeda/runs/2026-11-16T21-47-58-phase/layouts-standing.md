# layouts · standing conventions

- S1 · every --json output is one document on stdout and every new field on it is a member of that verb's typed document · surface-cli
  contract/viola-revive-json/list-document · viola revive --json · --list document — `--list --json` prints the rows as one document `{"v":1,"ok":{"sessions":[{"ts":…,"cause":…,"id":…}]}}` in log order, fields as the human rows, or the `no-session` document with exit 1.
  contract/viola-revive-json/refusal-document · viola revive --json · refusal document — Each refusal is one stdout document `{"v":1,"error":"<code>","detail":null}`, exit 1, empty stderr; without `--json` the pairs and rows stay byte-identical.
- S2 · human output is a contract: a changed or added human line is asserted by its exact text in a CLI test · surface-cli
