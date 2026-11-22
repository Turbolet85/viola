@row_add n=1 row=decision/permission-request-question-answerable area=surface-cli detector=none
name: A PermissionRequest question is answerable
rule: A question first raised by PermissionRequest with no matching armed dialog is a held dialog that viola answer and the MCP answer tool answer, like any other held dialog.
except: A human key, an expired hold or an unverified CLI still leave the question to the human.
basis: adaptation:6
because: A question first raised by PermissionRequest is held and answered as built, so the CLI surface states which dialogs viola answer reaches.
