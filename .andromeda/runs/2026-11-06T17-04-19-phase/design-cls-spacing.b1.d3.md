@answer n=1 entity=spacing#1.1 v=no
why: the web-spa Spacing token scale (space-micro 4px) is a 0.2.0 frontend design input; the chunk builds no web surface and touches no CSS (task item 2 not_needed)
@answer n=2 entity=spacing#1.2 v=no
why: the space-xs token (8px) is a web-spa Spacing input for strips and the rack; no chunk code or test reaches it (task item 2 not_needed)
@answer n=3 entity=spacing#1.3 v=no
why: the space-sm token (12px) is a web-spa Spacing input for ATIS header cells; the chunk does not change or measure it (task item 2 not_needed)
@answer n=4 entity=spacing#1.4 v=no
why: the space-md token (16px) is a web-spa Spacing input for page margins; the chunk adds no page and no layout (task item 2 not_needed)
@answer n=5 entity=spacing#1.5 v=no
why: the space-lg token (24px) is a web-spa Spacing input for rack separation; the chunk builds no rack (task item 2 not_needed)
@answer n=6 entity=spacing#1.6 v=no
why: the space-xl token (32px) is a web-spa Spacing input for the page margin and strip height; the chunk builds no surface (task item 2 not_needed)
@answer n=7 entity=spacing#2.1 v=no
why: the --strip-h CSS variable is a web-spa frontend layout value; no Rust crate, CLI string, pty or dependency reads it (task item 2 not_needed)
@answer n=8 entity=spacing#2.2 v=no
why: the --line-h CSS variable is a web-spa tape-line value; no chunk code or test reads it (task item 2 not_needed)
@answer n=9 entity=spacing#2.3 v=no
why: the --band-w CSS variable is a web-spa band width; the chunk builds no frontend and reaches no CSS (task item 2 not_needed)
@answer n=10 entity=spacing#2.4 v=no
why: the --rb-size CSS variable is a web-spa readback square size; the CLI readback mirror is a text string, not this square (task item 2 not_needed)
