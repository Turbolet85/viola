@row_add n=1 row=decision/test-binaries-never-released area=occupied-resources rec=1,2
name: Test-only binaries kept out of release builds
rule: viola-fake-agent, viola-harness and viola-mcp-tap are test-only binaries and never ship in a release build; the fake agent is built only behind the fake-agent feature.
@row_add n=2 row=name/command/run area=occupied-resources rec=3,4
class: command
name: run
@row_add n=3 row=name/command/revive area=occupied-resources rec=3,4
class: command
name: revive
@row_add n=4 row=name/command/send area=occupied-resources rec=3,4
class: command
name: send
@row_add n=5 row=name/command/wait area=occupied-resources rec=3,4
class: command
name: wait
@row_add n=6 row=name/command/last area=occupied-resources rec=3,4
class: command
name: last
@row_add n=7 row=name/command/list area=occupied-resources rec=3,4
class: command
name: list
@row_add n=8 row=name/command/answer area=occupied-resources rec=3,4
class: command
name: answer
@row_add n=9 row=name/command/hook area=occupied-resources rec=3,4
class: command
name: hook
@row_add n=10 row=name/command/mcp area=occupied-resources rec=3,4
class: command
name: mcp
@row_add n=11 row=name/command/verify area=occupied-resources rec=3,4
class: command
name: verify
@row_add n=12 row=name/command/pause area=occupied-resources rec=3,4
class: command
name: pause
@row_add n=13 row=name/command/release area=occupied-resources rec=3,4
class: command
name: release
@row_add n=14 row=name/command/link area=occupied-resources rec=3,4
class: command
name: link
@row_add n=15 row=name/command/unlink area=occupied-resources rec=3,4
class: command
name: unlink
@row_add n=16 row=name/command/plugin-install area=occupied-resources rec=3,4
class: command
name: plugin install
@row_add n=17 row=name/other/mcp-tool-send area=occupied-resources rec=5
class: other
name: send
@row_add n=18 row=name/other/mcp-tool-wait area=occupied-resources rec=5
class: other
name: wait
@row_add n=19 row=name/other/mcp-tool-last area=occupied-resources rec=5
class: other
name: last
@row_add n=20 row=name/other/mcp-tool-answer area=occupied-resources rec=5
class: other
name: answer
@row_add n=21 row=name/other/mcp-tool-list area=occupied-resources rec=5
class: other
name: list
