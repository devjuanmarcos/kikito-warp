param([string]$Tag = "m")
$s = "C:\Users\Marcos\AppData\Local\Temp\claude\D--DEVJUANMARCOS-PROJETOS-KIKITO-WARP\76b83ee0-665c-47d8-a6dc-2c6001b9e265\scratchpad\harness"
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
N move 900 600
Start-Sleep -Milliseconds 400
N move 32 118
Start-Sleep -Milliseconds 500
N rclick 32 118
N move 900 600
Start-Sleep -Milliseconds 500
N shot 0 0 "$s\$Tag`_org.png"
N move 72 139
Start-Sleep -Milliseconds 500
N shot 0 0 "$s\$Tag`_org_hover1.png"
N move 72 172
Start-Sleep -Milliseconds 500
N shot 0 0 "$s\$Tag`_org_hover2.png"
N move 900 600
N back
[System.Windows.Forms.Cursor]::Position = $orig
