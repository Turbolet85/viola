# Classified — design · spacing.b1

## Answers
spacing#1.1 · no · the 4px gap beside the web readback box is a 0.2.0 frontend value, the readback box's drawing being deferred
spacing#1.2 · no · the 8px field-cell padding and rack row gap are web strip values the chunk does not render (0.2.0 frontend input)
spacing#1.3 · no · the 12px gap between ATIS header cells is a web sticky-header value the chunk does not render (0.2.0 frontend input)
spacing#1.4 · no · the 16px page inline margin is a web page value the chunk does not render (0.2.0 frontend input)
spacing#1.5 · no · the 24px gap between racks and between the bay and the tape is web layout the chunk does not render (its CLI BAY line is text, not a px gap)
spacing#1.6 · no · the 32px top margin and strip height are web layout values the chunk does not render (0.2.0 frontend input)
spacing#2.1 · no · the --strip-h value is a stylesheet value and no stylesheet exists in 0.1.0 (0.2.0 frontend input)
spacing#2.2 · no · the --line-h tape line height is a web layout value and no stylesheet exists in 0.1.0 (0.2.0 frontend input)
spacing#2.3 · no · the --band-w band width is a web stylesheet value and no stylesheet exists in 0.1.0 (0.2.0 frontend input)
spacing#2.4 · no · the --rb-size readback square is the web drawing the task defers to 0.2.0, and the CLI readback line has no pixel size
