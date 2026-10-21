param([string]$Tag = "d")
$s = "C:\Users\Marcos\AppData\Local\Temp\claude\D--DEVJUANMARCOS-PROJETOS-KIKITO-WARP\594f3941-77bf-483a-a5e1-364a7ec5d42c\scratchpad\harness"
Add-Type -AssemblyName System.Windows.Forms
$orig = [System.Windows.Forms.Cursor]::Position
function N($a, $x = 0, $y = 0, $o = "") { & "$s\native.ps1" -Action $a -X $x -Y $y -Out $o -W 1280 -H 800 | Out-Null }
N launch
N front
Start-Sleep 3
N move 700 500
Start-Sleep 1
function Click($x, $y) { N move $x $y; Start-Sleep -Milliseconds 700; N down $x $y; Start-Sleep -Milliseconds 200; N up; Start-Sleep -Milliseconds 1500 }
Click 32 118
Click 190 120
Start-Sleep 2
Click 1121 17
Start-Sleep 2
N move 900 600
Start-Sleep -Milliseconds 400
N shot 0 0 "$s\$Tag`_git.png"
N move 900 700
Start-Sleep -Milliseconds 600
N move 900 600
N back
[System.Windows.Forms.Cursor]::Position = $orig
Click 1241 197
Start-Sleep 2
N shot 0 0 "$s\$Tag`_staged.png"
Click 1090 118
N keys 0 0 "feat: teste do painel"
Start-Sleep -Milliseconds 600
N shot 0 0 "$s\$Tag`_message.png"
N keys 0 0 "^{ENTER}"
Start-Sleep 3
N move 700 600
Start-Sleep -Milliseconds 500
N shot 0 0 "$s\$Tag`_committed.png"
N kill
