@row_set n=1 row=decision/live-rig-isolated-child cell=except seen=2 fact=F1 detector=none
value: The VH arm runs under the real home and config, reads no credentials, keeps only key names and one needle word, prints no value, and needs an explicit home, Claude dir and user config.
basis: fct-jzh734gm; fct-xiw3vi2f
because: The rig gained the VH arm, whose child runs without the private CLAUDE_CONFIG_DIR under the real home, so the isolation row must name it as its one exception.
@row_set n=2 row=decision/tests-use-own-home cell=except seen=1 fact=F1 detector=none
value: The live rig's VH arm runs under the real home and Claude config, reading key names only, and is refused without an explicit target home, Claude dir and user config.
basis: fct-jzh734gm; fct-xiw3vi2f
because: The VH arm is the one named exception to tests using their own home, so the row must carry it.
@row_set n=3 row=decision/verify-echo-probe-residuals cell=rule seen=1 fact=F3 detector=none
value: verify's Run B pastes /mcp disable viola-probe-absent seventh as the probe of the local-command-echo relied row and adds one input-history line; whether it writes to the user's real Claude config is measured absent on one CLI version only and stays unmeasured otherwise.
basis: fct-ubthpjx5; fct-ufvf53nu
because: A live reading found the probe wrote nothing naming its server into the user's Claude config or settings files on one version, so the row's flat «stays unmeasured» says it less than now known.
@row_set n=4 row=decision/verify-never-answers-dialogs cell=rule seen=1 fact=F4 detector=none
value: viola verify's interactive children never get a byte written into any dialog, and a start showing a modal is killed with no key; a browser-tools prompt standing at start is left for the human to answer once in claude, as verify says before its summary.
basis: fct-sfqjrkq4; fct-xiw3vi2f
because: verify now prints a fixed line before its summary when the browser-tools prompt stood, telling the human to answer it once in claude, so the never-answers row must say who answers it.
@row_add n=1 row=decision/live-rig-context-arm-keeps-no-transcript area=data-protection fact=F2 detector=D-security-input
name: Live rig's context arm keeps no transcript byte
rule: The live rig's CX arm reads the session transcript through MAX_FRAME, counting only tags past the kept length and keeping no byte of it, and sends only through viola send under the rig's private config.
basis: fct-jyarsz4g; fct-sgxfo7fm
because: The rig gained the CX arm, a new reader of the Claude transcript, whose bounded count-only read and private-config send must stand as a data-protection rule beside the other live-rig arms.
