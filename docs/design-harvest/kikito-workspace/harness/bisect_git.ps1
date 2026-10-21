param([string[]]$Configs = @("nochanges,nograph", "nograph", "nochanges", "nocommit", "nosvg", "nopercent", ""))
$h = "C:\Users\Marcos\AppData\Local\Temp\claude\D--DEVJUANMARCOS-PROJETOS-KIKITO-WARP\594f3941-77bf-483a-a5e1-364a7ec5d42c\scratchpad\harness"
$rec = "C:\Users\Marcos\AppData\Local\warp\WarpOss-parity\data\logs\warp-oss.log.recovery"
foreach ($c in $Configs) {
  & "$h\native.ps1" -Action kill | Out-Null; Start-Sleep 1
  python -I "$h\seed.py" | Out-Null
  $env:KIKITO_GIT_BISECT = $c
  $before = (Get-Item $rec -ErrorAction SilentlyContinue).LastWriteTime
  & "$h\native.ps1" -Action launch | Out-Null
  & "$h\native.ps1" -Action front | Out-Null; Start-Sleep 3
  foreach ($pt in @(@(32,118),@(190,120),@(1121,17))) { & "$h\native.ps1" -Action click -X $pt[0] -Y $pt[1] | Out-Null; Start-Sleep 2 }
  Start-Sleep 3
  $after = (Get-Item $rec -ErrorAction SilentlyContinue).LastWriteTime
  $crashed = $after -ne $before
  "config='$c' crash=$crashed"
  & "$h\native.ps1" -Action kill | Out-Null
}
Remove-Item Env:KIKITO_GIT_BISECT -ErrorAction SilentlyContinue
