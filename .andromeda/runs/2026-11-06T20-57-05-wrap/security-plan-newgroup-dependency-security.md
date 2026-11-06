@verdict n=1 ref_file=fanout-security-plan--cascade-g1.md ref_n=7 v=contradiction fact=fct-6nwwowx3
quote: `cargo deny check advisories` reads red on RUSTSEC-2026-0441 alone
why: the standing line has RUSTSEC-2026-0441 open and red via portable-pty =0.8.1, the proposal has it closed by portable-pty =0.9.0 · its own line rewritten
@verdict n=2 ref_file=fanout-security-plan--cascade-g1.md ref_n=9 v=contradiction fact=fct-zj6omeqt
quote: The `portable-pty =0.8.1` pin carries no Windows reason
why: the standing line keeps the pin at portable-pty =0.8.1 with the move off it still to be measured, the proposal states the =0.9.0 pin and its measured graph delta · its own line rewritten
