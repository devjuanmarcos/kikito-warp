param([string]$Tag = "t")
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
function Park() { N move 900 600; Start-Sleep -Milliseconds 400 }
Park
N shot 0 0 "$s\$Tag`_rest.png"
$targets = [ordered]@{ home = @(32,60); K = @(32,118); P = @(32,162); F = @(32,206); add = @(32,250); gear = @(32,769); header_plus = @(292,17); tab_plus = @(781,17); git = @(1121,17); link = @(841,17) }
foreach ($k in $targets.Keys) {
  Park
  N move $targets[$k][0] $targets[$k][1]
  Start-Sleep -Milliseconds 600
  N shot 0 0 "$s\$Tag`_$k.png"
}
Park
N move 190 180
N move 258 175
Start-Sleep -Milliseconds 600
N shot 0 0 "$s\$Tag`_row_plus.png"
Park
N back
[System.Windows.Forms.Cursor]::Position = $orig
