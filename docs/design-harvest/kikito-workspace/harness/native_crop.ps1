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
N move 900 600
Start-Sleep -Milliseconds 400
N shot 0 0 "$s\$Tag`_rest.png"
Click 32 250
N move 900 700
Start-Sleep -Milliseconds 600
N shot 0 0 "$s\$Tag`_modal.png"
Click 586 417
Start-Sleep 2
N keys 0 0 "C:\Users\Marcos\AppData\Local\Temp\claude\D--DEVJUANMARCOS-PROJETOS-KIKITO-WARP\594f3941-77bf-483a-a5e1-364a7ec5d42c\scratchpad\kit\assets\logo\kikito-terminal-icon.png"
Start-Sleep -Milliseconds 500
N keys 0 0 "{ENTER}"
Start-Sleep 3
N front
N move 900 700
Start-Sleep -Milliseconds 600
N shot 0 0 "$s\$Tag`_crop.png"
N move 900 600
N back
[System.Windows.Forms.Cursor]::Position = $orig
Click 737 511
Click 737 511
N move 640 368
Start-Sleep -Milliseconds 500
N down 640 368
Start-Sleep -Milliseconds 300
N move 590 330
Start-Sleep -Milliseconds 500
N shot 0 0 "$s\$Tag`_drag.png"
N up
Start-Sleep -Milliseconds 400
N wheel 640 368
N wheel 640 368
Start-Sleep -Milliseconds 400
N move 900 700
Start-Sleep -Milliseconds 400
N shot 0 0 "$s\$Tag`_zoomed.png"
Click 765 578
Start-Sleep 2
N move 900 700
Start-Sleep -Milliseconds 400
N shot 0 0 "$s\$Tag`_applied.png"
N kill
