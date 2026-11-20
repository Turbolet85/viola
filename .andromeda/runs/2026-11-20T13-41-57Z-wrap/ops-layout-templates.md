@row_add n=1 row=decision/permission-request-question-answerable area=surface-cli fact=F1 detector=none
name: A PermissionRequest question is answerable
rule: A question first raised by PermissionRequest with no armed PreToolUse is a held dialog that viola answer and the MCP answer tool answer, like any other held dialog.
except: A human key, an expired hold or an unverified CLI still leave the question to the human.
basis: fct-xnoawzki
because: The rule that left a PermissionRequest-first question unanswerable was removed and such a question is now held and answered, so the CLI surface must state which dialogs viola answer reaches.
