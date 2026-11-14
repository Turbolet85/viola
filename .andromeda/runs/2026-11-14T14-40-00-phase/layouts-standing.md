# layouts · standing conventions

- S1 · human output is a contract — each changed or new test of a CLI verb's output asserts its exact lines and words, on the right stream · surface-cli
- S2 · each --json document a test pins is one document with its typed fields, its refusal carrying the closed word and its exit code · surface-cli
  contract/cli-json-mode/one-document · CLI --json mode · one document — Under --json each verb prints exactly one document on stdout (`{"v":1,"ok":…}`, or the exit-21 / wrapper-fault document) and nothing on stderr.
