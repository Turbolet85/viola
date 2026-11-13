@row_retire n=1 row=contract/artifact-canary-scan-classes/cookie-header seen=1 detector=none
why: The artifact canary scan holds no cookie class any more; a cookie header line in an artifact is no hit.
basis: fct-xhrefo2i; fct-qlmrr4ao; fct-45cfy2kk
because: The chunk retired the scan's cookie class and its planted case now reads a cookie header as no hit, so the row promising a fail on a cookie header is false.
@row_add n=1 row=contract/artifact-canary-scan-classes/unreadable area=secret-management fact=F1 detector=none
boundary: artifact canary scan classes
item: unreadable
promise: The scan fails on a file it cannot read.
basis: fct-45cfy2kk; fct-r22lxaiu; fct-io25xen2
because: The scan's unreadable class now hits its own planted case, and the closed class list of the secret-management contract names every class but this one.
@row_none n=1 fact=F2 type=7
