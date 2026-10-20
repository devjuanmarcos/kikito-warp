param(
  [string]$Action = "launch",
  [string]$Out = "",
  [int]$X = 0, [int]$Y = 0,
  [int]$W = 1280, [int]$H = 800
)
Add-Type -AssemblyName System.Drawing
Add-Type @"
using System;
using System.Runtime.InteropServices;
public class Win {
  [DllImport("user32.dll")] public static extern bool SetProcessDPIAware();
  [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
  [DllImport("user32.dll")] public static extern bool MoveWindow(IntPtr h, int x, int y, int w, int hh, bool repaint);
  [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h);
  [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
  [DllImport("user32.dll")] public static extern void mouse_event(uint f, uint dx, uint dy, uint d, UIntPtr e);
  [DllImport("user32.dll")] public static extern bool SetWindowPos(IntPtr h, IntPtr after, int x, int y, int w, int hh, uint f);
  [DllImport("user32.dll")] public static extern bool PostMessage(IntPtr h, uint m, IntPtr w, IntPtr l);
  [DllImport("user32.dll")] public static extern bool PrintWindow(IntPtr h, IntPtr hdc, uint f);
  [DllImport("user32.dll")] public static extern bool ShowWindow(IntPtr h, int c);
  public delegate bool EnumProc(IntPtr h, IntPtr l);
  [DllImport("user32.dll")] public static extern bool EnumWindows(EnumProc p, IntPtr l);
  [DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr h, out uint pid);
  [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr h);
  [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern int GetClassName(IntPtr h, System.Text.StringBuilder s, int n);
  [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern int GetWindowText(IntPtr h, System.Text.StringBuilder s, int n);
  public static System.Collections.Generic.List<string> Windows(uint pid) {
    var res = new System.Collections.Generic.List<string>();
    EnumWindows((h, l) => { uint p; GetWindowThreadProcessId(h, out p); if (p == pid && IsWindowVisible(h)) {
      var c = new System.Text.StringBuilder(256); GetClassName(h, c, 256); var t = new System.Text.StringBuilder(256); GetWindowText(h, t, 256);
      RECT r; GetWindowRect(h, out r); res.Add(h.ToInt64() + "|" + c + "|" + t + "|" + r.L + "," + r.T + "," + r.R + "," + r.B); } return true; }, IntPtr.Zero);
    return res;
  }
  [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }
}
"@
[Win]::SetProcessDPIAware() | Out-Null
$exe = "D:\DEVJUANMARCOS\PROJETOS\KIKITO WARP\target\debug\warp-oss.exe"

function Get-AppWindow {
  $procs = Get-Process warp-oss -ErrorAction SilentlyContinue | Where-Object { $_.Path -eq $exe }
  foreach ($p in $procs) {
    foreach ($line in [Win]::Windows([uint32]$p.Id)) {
      $f = $line.Split("|")
      if ($f[1] -eq "Window Class") {
        return [pscustomobject]@{ Id = $p.Id; MainWindowHandle = [IntPtr][int64]$f[0]; Class = $f[1] }
      }
    }
  }
  return $null
}

switch ($Action) {
  "launch" {
    Get-Process warp-oss -ErrorAction SilentlyContinue | Where-Object { $_.Path -eq $exe } | Stop-Process -Force
    $env:WARP_DATA_PROFILE = "parity"
    Start-Process $exe -WorkingDirectory "D:\DEVJUANMARCOS\PROJETOS\KIKITO WARP"
    for ($i = 0; $i -lt 60; $i++) { Start-Sleep 1; if (Get-AppWindow) { break } }
    $p = Get-AppWindow
    if (-not $p) { Write-Output "NO WINDOW"; exit 1 }
    [Win]::ShowWindow($p.MainWindowHandle, 9) | Out-Null
    [Win]::MoveWindow($p.MainWindowHandle, $X, $Y, $W, $H, $true) | Out-Null
        Start-Sleep 3
    Write-Output "launched pid=$($p.Id)"
  }
  "move" {
    Add-Type -AssemblyName System.Windows.Forms
    $c = [System.Windows.Forms.Cursor]::Position
    $steps = 12
    for ($i = 1; $i -le $steps; $i++) {
      $nx = [int]($c.X + ($X - $c.X) * $i / $steps); $ny = [int]($c.Y + ($Y - $c.Y) * $i / $steps)
      [Win]::SetCursorPos($nx, $ny) | Out-Null; Start-Sleep -Milliseconds 20
    }
    Start-Sleep -Milliseconds 400
  }
  "click" {
    [Win]::SetCursorPos($X, $Y) | Out-Null; Start-Sleep -Milliseconds 200
    [Win]::mouse_event(2, 0, 0, 0, [UIntPtr]::Zero); Start-Sleep -Milliseconds 80
    [Win]::mouse_event(4, 0, 0, 0, [UIntPtr]::Zero); Start-Sleep -Milliseconds 500
  }
  "down" { [Win]::SetCursorPos($X, $Y) | Out-Null; Start-Sleep -Milliseconds 200; [Win]::mouse_event(2, 0, 0, 0, [UIntPtr]::Zero); Start-Sleep -Milliseconds 300 }
  "up" { [Win]::mouse_event(4, 0, 0, 0, [UIntPtr]::Zero); Start-Sleep -Milliseconds 300 }
  "shot" {
    $p = Get-AppWindow
    $r = New-Object Win+RECT
    [Win]::GetWindowRect($p.MainWindowHandle, [ref]$r) | Out-Null
    $w = $r.R - $r.L; $h = $r.B - $r.T
    $bmp = New-Object System.Drawing.Bitmap $w, $h
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.CopyFromScreen($r.L, $r.T, 0, 0, (New-Object System.Drawing.Size $w, $h))
    $bmp.Save($Out, [System.Drawing.Imaging.ImageFormat]::Png)
    Write-Output "shot $w x $h at $($r.L),$($r.T) -> $Out"
  }
  "rect" {
    $p = Get-AppWindow
    $r = New-Object Win+RECT
    [Win]::GetWindowRect($p.MainWindowHandle, [ref]$r) | Out-Null
    Write-Output "$($r.L),$($r.T),$($r.R),$($r.B)"
  }
  "pm-move" { $p = Get-AppWindow; $l = [IntPtr](($Y -shl 16) -bor ($X -band 0xFFFF)); [Win]::PostMessage($p.MainWindowHandle, 0x200, [IntPtr]::Zero, $l) | Out-Null; Start-Sleep -Milliseconds 400 }
  "pm-down" { $p = Get-AppWindow; $l = [IntPtr](($Y -shl 16) -bor ($X -band 0xFFFF)); [Win]::PostMessage($p.MainWindowHandle, 0x200, [IntPtr]::Zero, $l) | Out-Null; Start-Sleep -Milliseconds 150; [Win]::PostMessage($p.MainWindowHandle, 0x201, [IntPtr]1, $l) | Out-Null; Start-Sleep -Milliseconds 300 }
  "pm-up" { $p = Get-AppWindow; $l = [IntPtr](($Y -shl 16) -bor ($X -band 0xFFFF)); [Win]::PostMessage($p.MainWindowHandle, 0x202, [IntPtr]::Zero, $l) | Out-Null; Start-Sleep -Milliseconds 400 }
  "pm-click" { $p = Get-AppWindow; $l = [IntPtr](($Y -shl 16) -bor ($X -band 0xFFFF)); [Win]::PostMessage($p.MainWindowHandle, 0x200, [IntPtr]::Zero, $l) | Out-Null; Start-Sleep -Milliseconds 150; [Win]::PostMessage($p.MainWindowHandle, 0x201, [IntPtr]1, $l) | Out-Null; Start-Sleep -Milliseconds 80; [Win]::PostMessage($p.MainWindowHandle, 0x202, [IntPtr]::Zero, $l) | Out-Null; Start-Sleep -Milliseconds 500 }
  "pshot" {
    $p = Get-AppWindow
    $r = New-Object Win+RECT
    [Win]::GetWindowRect($p.MainWindowHandle, [ref]$r) | Out-Null
    $w = $r.R - $r.L; $h = $r.B - $r.T
    $bmp = New-Object System.Drawing.Bitmap $w, $h
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $hdc = $g.GetHdc()
    [Win]::PrintWindow($p.MainWindowHandle, $hdc, 2) | Out-Null
    $g.ReleaseHdc($hdc)
    $bmp.Save($Out, [System.Drawing.Imaging.ImageFormat]::Png)
    Write-Output "pshot $w x $h -> $Out"
  }
  "place" { $p = Get-AppWindow; [Win]::SetWindowPos($p.MainWindowHandle, [IntPtr](-2), $X, $Y, $W, $H, 0x40) | Out-Null }
  "front" { $p = Get-AppWindow; [Win]::ShowWindow($p.MainWindowHandle, 9) | Out-Null; [Win]::SetWindowPos($p.MainWindowHandle, [IntPtr](-1), $X, $Y, $W, $H, 0x40) | Out-Null; [Win]::SetForegroundWindow($p.MainWindowHandle) | Out-Null; Start-Sleep -Milliseconds 800 }
  "back" { $p = Get-AppWindow; [Win]::SetWindowPos($p.MainWindowHandle, [IntPtr](-2), $X, $Y, $W, $H, 0x40) | Out-Null }
  "rclick" {
    [Win]::SetCursorPos($X, $Y) | Out-Null; Start-Sleep -Milliseconds 300
    [Win]::mouse_event(8, 0, 0, 0, [UIntPtr]::Zero); Start-Sleep -Milliseconds 120
    [Win]::mouse_event(16, 0, 0, 0, [UIntPtr]::Zero); Start-Sleep -Milliseconds 600
  }
  "list" { Get-Process warp-oss | Where-Object { $_.Path -eq $exe } | ForEach-Object { [Win]::Windows([uint32]$_.Id) } }
  "kill" { Get-Process warp-oss -ErrorAction SilentlyContinue | Where-Object { $_.Path -eq $exe } | Stop-Process -Force }
}
