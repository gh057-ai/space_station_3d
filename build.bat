@echo off

:: Find Visual Studio installation (needed for raylib compilation)
set "VSWHERE=%ProgramFiles(x86)%\Microsoft Visual Studio\Installer\vswhere.exe"
for /f "usebackq tokens=*" %%i in (`"%VSWHERE%" -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath`) do (
  set "VS_PATH=%%i"
)

if not defined VS_PATH (
    echo Error: Visual Studio with C++ build tools not found!
    echo Please install Visual Studio Build Tools from https://visualstudio.microsoft.com/visual-cpp-build-tools/
    exit /b 1
)

:: Set up Visual Studio environment
call "%VS_PATH%\VC\Auxiliary\Build\vcvars64.bat"

:: Build the project
cargo build --release
