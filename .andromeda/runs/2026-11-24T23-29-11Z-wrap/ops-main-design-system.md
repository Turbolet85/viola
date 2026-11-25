@row_add n=1 row=contract/viola-adopt-refusals/not-resumed area=surface-cli detector=D-design-cli-wording
boundary: viola adopt refusals
item: not-resumed
promise: A child that ends before its session resumed is refused with exit 1: `unable: <name>'s session did not resume` then `hint: check the session id - claude --resume <id> must open it by hand, under the same claude config`; `<id>` is a literal word, the session id in no line.
basis: fct-x4brbvla
because: adopt gains a third own refusal with a fixed pair beside session-live's row, and the hint's `<id>` stays a literal word so the session id is never printed.
