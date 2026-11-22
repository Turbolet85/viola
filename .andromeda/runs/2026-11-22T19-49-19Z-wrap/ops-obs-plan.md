@row_add n=1 row=contract/confirm-window-close/failed-snapshot-write area=span-trace-coverage fact=F1 detector=none
boundary: run.confirm_window close
item: claimed session-start whose snapshot write fails
promise: A /clear or /compact session-start line the window claims, whose snapshot write then fails, leaves the send unclaimed and does not close the window; the window's own deadline ends the send unconfirmed as send-refused not-delivered with detail no-prompt-submitted.
basis: fct-ybqmyx4p
because: The chunk made a claimed session-start whose snapshot write fails settle the slot unclaimed, so such a send now ends by the window's deadline as no-prompt-submitted, a close the /clear and /compact rows do not state and the /compact row has no room left to hold.
