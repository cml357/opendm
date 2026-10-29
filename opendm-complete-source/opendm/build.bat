@echo off
title Compilation automatique d'OpenDM Windows (.EXE)
echo ========================================================
echo   Compilation d'OpenDM (Open Download Manager en Rust)
echo ========================================================
echo.
echo Verification des outils Rust...
where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo [ERREUR] Rust et Cargo ne sont pas detectes sur ce PC!
    echo Veuillez installer Rust gratuitement depuis : https://rustup.rs
    pause
    exit /b 1
)

echo Compilation du binaire Windows natif en mode Release (optimise)...
cargo build --release

if %errorlevel% equ 0 (
    echo.
    echo ========================================================
    echo  SUCCES : Votre fichier opendm.exe est pret !
    echo  Emplacement : target\release\opendm.exe
    echo ========================================================
    echo.
    if exist "target\release" (
        explorer target\release
    )
) else (
    echo.
    echo [ERREUR] La compilation a echoue. Verifiez que les Build Tools C++ sont installes.
)
pause
