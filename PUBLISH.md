# Publicar @kleosr/pe2-cli a npm

## Pasos para publicar:

### 1. Login en npm

Ejecuta en tu terminal:

```bash
npm login
```

Ingresa las siguientes credenciales cuando se soliciten:
- **Username:** kleosr
- **Password:** [tu contraseña de npm]
- **Email:** kleosr@proton.me

### 2. Verificar autenticación

```bash
npm whoami
```

Deberías ver: `kleosr`

### 3. Publicar el paquete

```bash
npm publish --access public
```

### Alternativa: Usar token de autenticación

Si prefieres usar un token en lugar de login interactivo:

1. Obtén tu token de npm desde: https://www.npmjs.com/settings/kleosr/tokens
2. Configura el token:

```bash
npm config set //registry.npmjs.org/:_authToken TU_TOKEN_AQUI
```

3. Luego publica:

```bash
npm publish --access public
```

## Verificación

Después de publicar, verifica en:
https://www.npmjs.com/package/@kleosr/pe2-cli

## Información del paquete

- **Nombre:** @kleosr/pe2-cli
- **Versión actual:** 3.4.5
- **Tamaño:** ~31.5 kB (empaquetado), ~137.4 kB (desempaquetado)
- **Archivos incluidos:** 19 archivos

