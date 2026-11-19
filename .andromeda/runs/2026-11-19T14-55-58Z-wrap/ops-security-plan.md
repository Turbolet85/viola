@row_set n=1 row=decision/live-rig-context-arm-keeps-no-transcript cell=rule seen=1 fact=F1 detector=none
value: The live rig's CX arm reads the session transcript through MAX_FRAME, counting only tags past the kept length or, for a transcript first found after the send, from its start, keeping no byte of it, and sends only through viola send under the rig's private config.
basis: fct-sie7vnw5
because: The context arm now looks the transcript up once more after the send when none was held before it and counts a transcript found then from its start, so the rule's count bound must name that case.
