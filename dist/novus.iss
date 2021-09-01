; Script generated by the Inno Setup Script Wizard.
; SEE THE DOCUMENTATION FOR DETAILS ON CREATING INNO SETUP SCRIPT FILES!

#define MyAppName "Novus"
#define MyAppVersion "1.1.2"
#define MyAppPublisher "Pranav Doshi"
#define MyAppURL "https://www.novuspkg.com/"
#define MyAppExeName "novus.exe"

[Setup]
; NOTE: The value of AppId uniquely identifies this application. Do not use the same AppId value in installers for other applications.
; (To generate a new GUID, click Tools | Generate GUID inside the IDE.)
AppId={{B715F1A9-55D1-40E1-B05A-E23E42EDB2AD}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
;AppVerName={#MyAppName} {#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultDirName={autopf}\{#MyAppName}
ChangesAssociations=yes
DefaultGroupName={#MyAppName}
DisableProgramGroupPage=yes
LicenseFile=D:\prana\Programming\My Projects\novus-package-manager\novus\LICENSE
; Uncomment the following line to run in non administrative install mode (install for current user only.)
; PrivilegesRequired=lowest
OutputDir=D:\prana\Programming\My Projects\novus-package-manager\novus\dist
OutputBaseFilename=Novus v1.1.2 Alpha Setup
SetupIconFile=D:\prana\Pictures\novus\logo.ico
Compression=lzma
SolidCompression=yes
WizardStyle=modern
LZMAUseSeparateProcess=yes
LZMANumBlockThreads=6

[Dirs]
Name: "{userappdata}\Novus";

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Code]
const
  EnvironmentKey = 'SYSTEM\CurrentControlSet\Control\Session Manager\Environment';
procedure RemovePath(Path: string);
var
  Paths: string;
  P: Integer;
begin
  if not RegQueryStringValue(HKEY_LOCAL_MACHINE, EnvironmentKey, 'Path', Paths) then
  begin
    Log('PATH not found');
  end
    else
  begin
    Log(Format('PATH is [%s]', [Paths]));
    P := Pos(';' + Uppercase(Path) + ';', ';' + Uppercase(Paths) + ';');
    if P = 0 then
    begin
      Log(Format('Path [%s] not found in PATH', [Path]));
    end
      else
    begin
      if P > 1 then P := P - 1;
      Delete(Paths, P, Length(Path) + 1);
      Log(Format('Path [%s] removed from PATH => [%s]', [Path, Paths]));
      if RegWriteStringValue(HKEY_LOCAL_MACHINE, EnvironmentKey, 'Path', Paths) then
      begin
        Log('PATH written');
      end
        else
      begin
        Log('Error writing PATH');
      end;
    end;
  end;
end;
procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
begin
  if CurUninstallStep = usUninstall then
  begin
    RemovePath('C:\Program Files (x86)\Novus');
  end;
end;
function NeedsAddPath(Param: string): boolean;
var
  OrigPath: string;
begin
  if not RegQueryStringValue(HKEY_LOCAL_MACHINE,
    'SYSTEM\CurrentControlSet\Control\Session Manager\Environment',
    'Path', OrigPath)
  then begin
    Result := True;
    exit;
  end;
  { look for the path with leading and trailing semicolon }
  { Pos() returns 0 if not found }
  Result := Pos(';' + Param + ';', ';' + OrigPath + ';') = 0;
  
  { SOURCE: https://stackoverflow.com/questions/3304463/how-do-i-modify-the-path-environment-variable-when-running-an-inno-setup-install }

end;

[Registry]
Root: HKLM; Subkey: "SYSTEM\CurrentControlSet\Control\Session Manager\Environment"; \
    ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};C:\Program Files (x86)\{#MyAppName}"; \
    Check: NeedsAddPath('C:\Program Files (x86)\{#MyAppName}')
Root: HKLM; Subkey: "SYSTEM\CurrentControlSet\Control\Session Manager\Environment"; \
    ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{%USERPROFILE}\novus\shims"; \
    Check: NeedsAddPath('{%USERPROFILE}\novus\shims')
    
[Files]
Source: "D:\prana\Programming\My Projects\novus-package-manager\novus\target\release\novus.exe"; DestDir: "{app}"; Flags: ignoreversion
; NOTE: Don't use "Flags: ignoreversion" on any shared system files

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"
; Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon