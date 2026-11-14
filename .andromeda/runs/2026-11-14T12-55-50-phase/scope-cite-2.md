old:    [inferred] The enclosing function's name: the carry says `read_within`; P3 confirms it at HEAD.
new:    The enclosing function is `read_within` (defined `read.rs:56`, called by `read` at `:51`; verified at P3).
old:    [inferred] Whether the seam holds any other `MAX_FRAME` comparison without an exact-frame case: P3 reads it; one
new:    The seam holds one other `MAX_FRAME` comparison, `reply.rs:98` in `json()`, already pinned at the exact frame by
old:    found is folded here only as the directive's item 4 allows.
new:    that `parse_clients` test (verified at P3: `grep -n MAX_FRAME crates/viola-hyprland/src/*.rs`).
