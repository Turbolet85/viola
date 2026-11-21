@row_add n=1 row=decision/cross-session-send-by-uds-address area=established-decisions fact=F2 detector=D-arch-decisions
name: Cross-session sends addressed by inbox socket
rule: A local claude session is addressed for a cross-session send by its inbox socket address, uds:<runtime>/cc-socks/<pid>.sock, composed from the runtime dir and the session's pid, never by a bare session id.
why: As the 2.1.328 bundle's strings read, a bare session id resolves no local session; confirmed only by the mended live start passing.
basis: fct-vgvx3eex
because: The bare-id addressing read red live and was replaced by the composed uds address; the address shape is security-plan's input rule, and the cause is said as measured, on one version.
@row_add n=2 row=contract/hook-capture-arm/names-only area=occupied-resources fact=F4 detector=D-arch-resources
boundary: viola hook --capture arm
item: --names-only
promise: A hidden mode valid only with --capture: it files only owner-only Tool.<k>.json files of closed tool and send-result words, never a payload byte.
basis: fct-zjeenbpz
because: The capture arm gained a hidden names-only mode, a surface its contract items do not name; the promise holds what the report states and no more.
