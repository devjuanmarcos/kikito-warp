param([string]$Tag = "n")
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
Click 32 118      # org Keener
Click 190 120     # project keener-api
function Shot($name, $x, $y) { N move $x $y; Start-Sleep -Milliseconds 400; N shot 0 0 "$s\$Tag`_$name.png" }
function Park() { N move 900 600; N up }
Park; Start-Sleep -Milliseconds 400; N shot 0 0 "$s\$Tag`_rest.png"
Shot "hover_header_plus" 292 17
Park
N down 292 17; Start-Sleep -Milliseconds 300; N shot 0 0 "$s\$Tag`_press_header_plus.png"; N move 900 600; N up; Park
Shot "hover_footer" 188 269
Park
N down 188 269; Start-Sleep -Milliseconds 300; N shot 0 0 "$s\$Tag`_press_footer.png"; N move 900 600; N up; Park
Shot "hover_row" 190 176
Shot "hover_git" 1121 17
Shot "hover_link" 841 17
Shot "hover_tab_plus" 781 17
Park
N back
[System.Windows.Forms.Cursor]::Position = $orig
Get-Content "C:\Users\Marcos\AppData\Local\warp\WarpOss-parity\data\logs\warp-oss.log" | Select-String "panicked" | Select-Object -Last 1
