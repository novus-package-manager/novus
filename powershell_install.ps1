# Source: https://xtreme-cdn.herokuapp.com/electric/electricInstall.ps1 (Electric)

Import-Module BitsTransfer

$ErrorActionPreference = "Stop"

Start-BitsTransfer 'https://github.com/novus-package-manager/novus/releases/download/v1.0.0/Novus.v1.0.0.Setup.exe' "${Env:\TEMP}\NovusSetup.exe" -Description 'Downloading Novus Alpha v1.0.0 Setup from https://github.com/novus-package-manager/novus/releases' -DisplayName 'Downloading Novus' -TransferType Download

Write-Host 'Installing Novus' -ForegroundColor cyan

& "${Env:\TEMP}\NovusSetup.exe" /VERYSILENT | Out-Null

if ([System.IO.File]::Exists('C:\Program Files (x86)\Novus\novus.exe')) {

    Write-Host 'Successfully Installed Novus' -ForegroundColor green

} else {

    Write-Error 'Failed To Install Novus'

}