# Classified — arch · conventions.b1

## Answers
conventions#1.1 · yes · the pre-frame refusal checks of `send` (`control-character`, `empty-text`) and `answer` (`control-character`) keep their first-refusal order against the new pre-frame check
conventions#2.1 · yes · exit 21 with the `instance-unreachable` document is the mismatch outcome of every CLI verb the chunk gates, so `wait` and `last` must not map a failed check to exit 1
conventions#3.1 · yes · an unreachable instance is the `isError: true` `instance-unreachable` result with `refused: <code>` text that the `viola-mcp` pre-check returns before any frame
