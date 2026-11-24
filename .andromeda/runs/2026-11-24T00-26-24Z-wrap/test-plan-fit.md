@fit n=1 row=contract/viola-list-json/items sha=cd40f7c69f6a v=fits type=3
@fit n=2 row=contract/run-start-events/order sha=ada34ae06c97 v=fits type=3
@fit n=3 row=decision/run-start-verify-and-trust-after-bind sha=4e1b01241157 v=not_master type=4
why: The added window step only retells where one function call sits in the start sequence, with no reason stated in the basis, and the code holds that order.
@fit n=4 row=decision/fake-agent-on-test-path sha=2564c22c123b v=fits type=1
@fit n=5 row=decision/fake-agent-as-claude sha=6a114500d9b5 v=fits type=1
@fit n=6 row=decision/fakes-for-external-services sha=be9c48b7cb6b v=fits type=1
@fit n=7 row=decision/snapshot-replay-window sha=f99e156d2a09 v=not_master type=4
why: How replay rebuilds one field from the event log is a mechanism whose basis states no reason for it, and the replay code and its named tests hold it.
