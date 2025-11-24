@echo off
REM Script para publicar @kleosr/pe2-cli a npm (Windows)

echo 🚀 Publicando @kleosr/pe2-cli v3.4.5 a npm...

REM Verificar que estás logueado
npm whoami >nul 2>&1
if errorlevel 1 (
    echo ❌ No estás logueado en npm
    echo Por favor ejecuta primero:
    echo   npm login
    echo.
    echo O usa un token:
    echo   npm config set //registry.npmjs.org/:_authToken YOUR_TOKEN
    exit /b 1
)

echo ✅ Autenticado como:
npm whoami
echo.
echo 📦 Publicando paquete...
npm publish --access public

if errorlevel 1 (
    echo.
    echo ❌ Error al publicar
    exit /b 1
) else (
    echo.
    echo ✅ ¡Publicación exitosa!
    echo 📦 Paquete disponible en: https://www.npmjs.com/package/@kleosr/pe2-cli
)

