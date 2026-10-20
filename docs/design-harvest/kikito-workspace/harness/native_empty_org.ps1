param([string]$Tag = "o")
$s = "C:\Users\Marcos\AppData\Local\Temp\claude\D--DEVJUANMARCOS-PROJETOS-KIKITO-WARP\594f3941-77bf-483a-a5e1-364a7ec5d42c\scratchpad\harness"
Add-Type -AssemblyName System.Windows.Forms
$orig = [System.Windows.Forms.Cursor]::Position
function N($a, $x = 0, $y = 0, $o = "") { & "$s\native.ps1" -Action $a -X $x -Y $y -Out $o -W 1280 -H 800 | Out-Null }
N launch
N front
Start-Sleep 3
N move 32 250
Start-Sleep -Milliseconds 700
N down 32 250
Start-Sleep -Milliseconds 200
N up
Start-Sleep -Milliseconds 1500
N move 900 600
Start-Sleep 1
N shot 0 0 "$s\$Tag`_empty_org.png"
N back
[System.Windows.Forms.Cursor]::Position = $orig
