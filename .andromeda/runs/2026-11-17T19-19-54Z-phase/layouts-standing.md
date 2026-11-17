# layouts · standing conventions

- S1 · each new test that waits takes a bound under the 20 s lint, and every process a booted test starts ends with that test · surface-cli
- S2 · each new verify output line follows the verify output contract: plain ASCII, results on stdout and the rest on stderr, closed words for each row's state · surface-cli
  decision/stdout-results-stderr-rest · Results on stdout, the rest on stderr — Results go to stdout; `waiting:`, the issue line, refusals, hints and errors go to stderr, and the two are never mixed.
  decision/words-carry-every-state · Words carry every state — Piped viola list output is the same characters with no SGR, and nothing is lost because the words carry every state.
