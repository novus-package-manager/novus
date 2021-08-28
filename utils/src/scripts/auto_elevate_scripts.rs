pub const CONFIG_JSON: &str = "
{
    
}
";

pub const AUTO_ELEVATE_INSTALL: &str = "
@echo off

:: BatchGotAdmin
:-------------------------------------
REM  --> Check for permissions
    IF \"%PROCESSOR_ARCHITECTURE%\" EQU \"amd64\" (
>nul 2>&1 \"%SYSTEMROOT%\\SysWOW64\\cacls.exe\" \"%SYSTEMROOT%\\SysWOW64\\config\\system\"
) ELSE (
>nul 2>&1 \"%SYSTEMROOT%\\system32\\cacls.exe\" \"%SYSTEMROOT%\\system32\\config\\system\"
)

REM --> If error flag set, we do not have admin.
if '%errorlevel%' NEQ '0' (
    goto UACPrompt
) else ( goto gotAdmin )

:UACPrompt
    echo Set UAC = CreateObject^(\"Shell.Application\"^) > \"%appdata%\\getadmin.vbs\"
    set params= %*
    echo UAC.ShellExecute \"cmd.exe\", \"/c \"\"%~s0\"\" %params:\"=\"\"%\", \"\", \"runas\", 1 >> \"%appdata%\\getadmin.vbs\"

    \"%appdata%\\getadmin.vbs\"
    del \"%appdata%\\getadmin.vbs\"
    exit /B

:gotAdmin
    pushd \"%CD%\"
    CD /D \"%~dp0\"
:--------------------------------------

powershell novus i %1 -y
timeout /t 2";

pub const AUTO_ELEVATE_UNINSTALL: &str = "
@echo off

:: BatchGotAdmin
:-------------------------------------
REM  --> Check for permissions
    IF \"%PROCESSOR_ARCHITECTURE%\" EQU \"amd64\" (
>nul 2>&1 \"%SYSTEMROOT%\\SysWOW64\\cacls.exe\" \"%SYSTEMROOT%\\SysWOW64\\config\\system\"
) ELSE (
>nul 2>&1 \"%SYSTEMROOT%\\system32\\cacls.exe\" \"%SYSTEMROOT%\\system32\\config\\system\"
)

REM --> If error flag set, we do not have admin.
if '%errorlevel%' NEQ '0' (
    goto UACPrompt
) else ( goto gotAdmin )

:UACPrompt
    echo Set UAC = CreateObject^(\"Shell.Application\"^) > \"%appdata%\\getadmin.vbs\"
    set params= %*
    echo UAC.ShellExecute \"cmd.exe\", \"/c \"\"%~s0\"\" %params:\"=\"\"%\", \"\", \"runas\", 1 >> \"%appdata%\\getadmin.vbs\"

    \"%appdata%\\getadmin.vbs\"
    del \"%appdata%\\getadmin.vbs\"
    exit /B

:gotAdmin
    pushd \"%CD%\"
    CD /D \"%~dp0\"
:--------------------------------------

powershell novus u %1 -y
timeout /t 2";

// SOURCE: https://stackoverflow.com/questions/11525056/how-to-create-a-batch-file-to-run-cmd-as-administrator
