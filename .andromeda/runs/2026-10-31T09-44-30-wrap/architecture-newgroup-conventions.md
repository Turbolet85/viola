# New groups — architecture · conventions

## Verdicts
- fanout-architecture--cascade.md 11 · distinct · no group names `is_code_word` or states a code word's length and alphabet; line 8 says only that `detail` values are kebab-case strings from a closed set
- fanout-architecture--cascade.md 12 · contradiction · lines 15–15 · «Readers treat an unrecognised `detail` as opaque text, never as an error.» · the file has readers keep every unrecognised `detail` as opaque text and never err on it, the proposal has a string `detail` that is no code word read null and a non-string `detail` count as no known reply
