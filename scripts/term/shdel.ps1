$bitness = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "i686" }
$__TAG_NAME__ = "v0.1.0-346c7ab6"
$url="https://github.com/xTeKc/shdel/releases/download/$__TAG_NAME__/shdel-$bitness-pc-windows-msvc.exe"
#$url="https://shdel.onrender.com/download/bin?tag=$__TAG_NAME__&arch=$bitness-pc-windows-msvc&ext=.exe"
$outFile = "$Env:TEMP\shdel.exe"

Write-Output "$($PSStyle.Bold)$($PSStyle.Foreground.Green)info:$($PSStyle.Reset) downloading shdel"

$oldProgressPreference = $ProgressPreference
$ProgressPreference = 'SilentlyContinue'
Invoke-WebRequest -Uri $url -OutFile $outFile
$ProgressPreference = $oldProgressPreference

Start-Process -FilePath $outFile -Wait -NoNewWindow