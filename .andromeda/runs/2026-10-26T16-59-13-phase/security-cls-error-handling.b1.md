# Classified — security · error-handling.b1

## Answers
error-handling#1.1 · yes · the new strict-modes check in `hook` must fail open with exit 0, no body and empty stderr, as its server, stdin and parse checks do
error-handling#1.2 · yes · a `hook` event or dialog that fails the new check must write no frame and exit 0 with empty streams, keeping the `hook-decision` and SessionEnd lines of a failed server check
error-handling#1.3 · yes · the frame tools' strict-modes failure must return `isError` `instance-unreachable` with `detail: null` and text `refused: instance-unreachable`, sending no frame and writing no log line
error-handling#1.4 · no · the chunk reaches neither the `link` / `unlink` verbs nor the wrapper's link / unlink replies, and no reached unit holds their error mapping
