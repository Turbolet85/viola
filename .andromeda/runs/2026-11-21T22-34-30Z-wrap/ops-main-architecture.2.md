@row_set n=1 row=decision/mcp-wait-cap cell=why seen=1 fact=F3 detector=D-arch-decisions
value: The cap sits below the 55 s MCP call deadline, itself below the client's tool-call bound, so a wait returns before either cuts the call.
basis: fct-usvnottw
because: The standing why rests on the 2.1.287 stdio default, which 2.1.328 and 2.1.330 no longer show — their strings read a 1e8 ms hard limit, a 30 min stdio idle limit and a 120 s auto-background, the one 60 s figure flooring the HTTP transports only (readings, never measured); measured live: a 50 s wait returned whole on both (MB, MB8) and a 5000 ms server timeout cut a call on 2.1.330 (MC); the cap's reason is now the compiled call deadline above it.
@row_set n=2 row=decision/send-time-bound cell=rule seen=1 fact=F2 detector=D-arch-decisions
value: send blocks at most the readiness gate's maximum wait plus the paste plus the window; on the MCP surface the MCP call deadline bounds the driver's wait, never the send itself.
basis: fct-t2uvkzop
because: The standing clause «that total stays below the MCP client's tool-call timeout» is false while the paste write has no time bound; the deadline bounds the driver's wait only (the paste's gap is the route's CARRY, fit's not_master on the longer set).
