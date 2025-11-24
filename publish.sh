#!/bin/bash
# Script para publicar @kleosr/pe2-cli a npm

echo "🚀 Publicando @kleosr/pe2-cli v3.4.5 a npm..."

# Verificar que estás logueado
if ! npm whoami &> /dev/null; then
    echo "❌ No estás logueado en npm"
    echo "Por favor ejecuta primero:"
    echo "  npm login"
    echo ""
    echo "O usa un token:"
    echo "  npm config set //registry.npmjs.org/:_authToken YOUR_TOKEN"
    exit 1
fi

echo "✅ Autenticado como: $(npm whoami)"
echo ""
echo "📦 Publicando paquete..."
npm publish --access public

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ ¡Publicación exitosa!"
    echo "📦 Paquete disponible en: https://www.npmjs.com/package/@kleosr/pe2-cli"
else
    echo ""
    echo "❌ Error al publicar"
    exit 1
fi

