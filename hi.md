# slop

# ANTI-SLOP NUCLEAR PROTOCOL

> Reglas para Cursor, Claude.md, Agents.md  
> Versión: 2025 | Nivel: Senior Engineer con 15+ años

---

## ACTIVACIÓN

Cuando el usuario diga cualquiera de estas frases (en cualquier idioma):
- "remove slop", "clean slop", "anti slop", "purge slop"
- "limpia el código", "quita la basura", "modo nuclear"

**Ejecutar todas las reglas sin pedir confirmación.**

---

## 1. COMENTARIOS

### Eliminar inmediatamente

- Comentarios que describen lo que el código ya dice: `// Increment counter`, `// Return the result`
- Comentarios TODO sin contexto: `// TODO: implement later`, `// FIXME`
- Comentarios con tono corporativo o de tutorial: `// This function handles...`, `// Note:`, `// Important:`
- Comentarios que explican sintaxis del lenguaje: `// This is an async function`, `// Destructuring the object`
- Bloques de comentarios generados en masa (JSDoc vacíos, `@param` sin descripción real)

### Conservar únicamente

- Comentarios que explican **por qué**, no **qué**: `// Workaround for Safari bug #1234`
- Referencias a tickets, issues, o decisiones de arquitectura
- Advertencias sobre edge cases no obvios

---

## 2. TYPESCRIPT Y TIPOS

### Eliminar o corregir

- `any` sin justificación documentada en comentario
- `as any`, `as unknown` para silenciar errores
- `@ts-ignore`, `@ts-expect-error` sin explicación del bug que evitan
- Tipos duplicados que ya existen en el proyecto o en `@types/*`
- Interfaces de 1-2 propiedades que podrían ser inline
- Generics innecesarios: `Array<string>` → `string[]`
- `Partial<T>`, `Required<T>`, `Pick<T>` anidados sin razón

### Mantener

- `children: React.ReactNode` → Es tipado correcto, NO es slop
- Type guards bien escritos
- Generics que realmente añaden type safety

---

## 3. CÓDIGO DEFENSIVO

### Eliminar cuando sea innecesario

- `try/catch` que solo hacen `console.log(error)` o re-throw sin transformar
- `try/catch` en cada función cuando existe error boundary global
- `if (x !== null && x !== undefined)` cuando el tipo ya garantiza que existe
- Optional chaining `?.` en cadenas donde los valores previos ya fueron validados
- `!!value` para coerción cuando el contexto ya es booleano

### Conservar

- Optional chaining en datos de APIs externas no controladas
- Validación en boundaries (inputs de usuario, respuestas de red)
- Error handling que transforma errores en mensajes útiles

---

## 4. ESTRUCTURAS INFLADAS

### Simplificar agresivamente

```
// SLOP
const temp = getData();
return temp;

// CORRECTO
return getData();
```

```
// SLOP
function wrapper(x) {
  return innerFunction(x);
}

// CORRECTO
// Eliminar wrapper, usar innerFunction directamente
```

```
// SLOP
if (condition) {
  return true;
} else {
  return false;
}

// CORRECTO
return condition;
```

### Regla general

Si una función de 5+ líneas puede ser 1-2 líneas sin perder claridad → reducir.

---

## 5. HOOKS Y UTILS DUPLICADOS

### Eliminar si ya existen en el proyecto o en dependencias

**Hooks típicamente duplicados:**
- `useToggle`, `useBoolean`, `useDisclosure`
- `useDebounce`, `useThrottle`
- `usePrevious`
- `useLocalStorage`, `useSessionStorage`
- `useClickOutside`, `useOnClickOutside`
- `useIsMounted` (casi nunca necesario con cleanup correcto)
- `useWindowSize` que retorna 8 valores cuando solo se usa 1

**Utils típicamente duplicados:**
- `cn()`, `clsx()`, `classNames()` → usar solo UNO en todo el proyecto
- `isEmpty`, `isNil`, `isNullOrUndefined` → usar lodash o nativo
- `formatDate`, `formatCurrency` → usar `date-fns` o `Intl`
- `debounce`, `throttle` → usar lodash o implementación única
- `capitalize`, `slugify` → verificar si ya existe

### Acción

Antes de crear un util/hook → buscar en el proyecto si ya existe.

---

## 6. SERVICES Y API CLIENTS

### Eliminar

- Axios instances con 40+ líneas de interceptors que no se usan
- Clases Service con métodos CRUD completos cuando solo se usa 1-2
- `try/catch` + `toast.error` en cada método cuando hay manejo global
- Wrappers de fetch que solo añaden `Content-Type: application/json`

### Patrón correcto

- Un solo cliente HTTP configurado globalmente
- Funciones individuales por endpoint, no clases monolíticas
- Error handling en el boundary, no en cada llamada

---

## 7. CONSTANTES Y CONFIGURACIÓN

### Eliminar duplicación

- `ROUTES`, `API_ENDPOINTS` definidos en múltiples archivos → centralizar
- `STATUS`, `ROLES`, `PERMISSIONS` repetidos en frontend y tipos → single source of truth
- Magic strings/numbers → extraer a constantes CON NOMBRES DESCRIPTIVOS
- Colores, spacing, breakpoints fuera del theme/tailwind config

### Regla

Cada constante debe existir en **exactamente un lugar**.

---

## 8. COMPONENTES UI

### No reinventar

Si el proyecto usa shadcn/radix/mantine/headless-ui:
- NO crear `<Button />`, `<Card />`, `<Modal />`, `<Table />` custom
- NO crear `<LoadingSpinner />` de 70 líneas cuando existe uno global
- NO crear `<ErrorBoundary />` por página

### Excepciones válidas

- Componentes con lógica de negocio específica
- Wrappers que añaden comportamiento real (no solo props por defecto)

---

## 9. REACT QUERY / TANSTACK QUERY

### Eliminar

- `queryKey` construidos de forma diferente en cada archivo
- `select: (data) => data.data` repetido (configurar en el cliente)
- `staleTime`, `cacheTime` hardcodeados en cada query
- `onError` con toast en cada query cuando hay handler global

### Patrón correcto

- Query keys centralizados o con factory
- Defaults globales en QueryClient
- Custom hooks por dominio, no queries sueltas

---

## 10. FORMULARIOS (React Hook Form / Zod)

### Eliminar

- `zodResolver({ schema })` repetido → extraer a helper
- `defaultValues` de 40 líneas cuando el form empieza vacío
- `watch()` en campos que no necesitan reactividad
- Schemas Zod de 300 líneas para forms de 3 campos
- Validaciones duplicadas (frontend + backend idénticas sin compartir)

---

## 11. CSS / TAILWIND / LAYOUT

### Eliminar

- Divs anidados solo para centrar: `div > div > div > div` → una clase
- `grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3` repetido → componente `<Grid />`
- `flex flex-col gap-4` en 200 lugares → componente `<Stack />`
- Colores hardcodeados fuera del theme: `bg-[#1a1a1a]` → usar tokens
- `backdrop-filter: blur(120px)` en móviles → mata performance

### Preferir

- CSS animations sobre JS animations cuando sea posible
- `loading="lazy"` en imágenes below-the-fold
- `font-display: swap` en fuentes custom

---

## 12. IMPORTS Y EXPORTS

### Eliminar

- Imports no usados
- Imports duplicados
- `index.ts` que re-exporta 150 cosas sin organización
- Barrel files que causan bundle bloat

### Verificar

- Tree-shaking funciona correctamente
- No importar lodash entero: `import { debounce } from 'lodash'` → `import debounce from 'lodash/debounce'`

---

## 13. DEBUGGING Y CÓDIGO MUERTO

### Eliminar siempre

- `console.log`, `console.warn`, `console.error` de debugging
- Código comentado "por si acaso"
- Funciones no llamadas desde ningún lugar
- Variables declaradas pero no usadas
- Features completas que nadie pidió (paginación para 8 items, dark mode en proyecto light-only)

---

## 14. KEYS EN LISTAS

### Regla correcta

- `key={item.id}` cuando el item tiene ID único → **usar siempre**
- `key={index}` es VÁLIDO para listas estáticas que nunca reordenan (menús, tabs fijos)
- `key={index}` es INCORRECTO para listas dinámicas que pueden reordenar/filtrar

---

## 15. DATA-TESTID

### Regla correcta

- NO eliminar `data-testid` si el proyecto tiene tests que los usan
- NO añadir `data-testid` a cada div "porque sí"
- Añadir solo en elementos interactivos que necesitan ser testeados

---

## 16. PERFORMANCE REAL

### Verificar

- Imágenes: `loading="lazy"`, `srcset`, formatos modernos (WebP/AVIF)
- Scroll listeners: usar `passive: true`, debounce/throttle
- Arrays/objetos constantes: declarar fuera del componente
- Animaciones: CSS > JS cuando sea posible
- Fonts: `font-display: swap`

---

## 17. ARCHIVOS QUE SON RED FLAGS

Revisar y probablemente refactorizar:

- `lib/utils.ts` con 500+ líneas
- `types/index.ts` exportando todo
- `constants/index.ts` con re-exports masivos
- `components/ui/` con 50+ componentes usándose 8

---

## PALABRAS QUE INDICAN SLOP (CONTEXTO IMPORTANTE)

Estas palabras en **comentarios o documentación** son red flags:
- "robust", "resilient", "seamless", "leverage"
- "comprehensive", "cutting-edge", "state-of-the-art"
- "revolutionize", "game-changing", "empower"

**PERO** en código técnico pueden ser válidas:
- `robustErrorHandler` → válido si realmente maneja edge cases
- Variable/función con nombre técnico descriptivo → evaluar caso por caso
1. NEXT.JS 14/15 (APP ROUTER)
Server Components vs Client Components

Por defecto todo es Server Component, NO añadir "use client" sin razón
"use client" SOLO es necesario cuando: hooks de React (useState, useEffect, useContext, useReducer), event handlers (onClick, onChange, onSubmit), APIs del browser (localStorage, window, navigator), o librerías third-party client-only
Nunca poner "use client" en archivos page.tsx que solo muestran datos
Los Server Components pueden ser children de Client Components y mantienen sus beneficios

Data Fetching

En Server Components llamar funciones directamente, NUNCA fetch a Route Handlers propios
Next.js 15: fetch NO está cached por defecto, requiere cache: 'force-cache' o next: { revalidate: X } explícito
Usar Server Actions para mutaciones, siempre con revalidatePath o revalidateTag después
redirect() lanza una excepción, nunca ponerlo dentro de try/catch

Next.js 15 Breaking Changes

cookies(), headers(), params, searchParams ahora son async y requieren await
Actualizar todos los componentes que usen estas APIs

Metadata

Metadata solo funciona en Server Components
Usar cache() de React para deduplicar fetches entre generateMetadata y el componente

Red Flags Next.js

"use client" en archivos sin hooks ni eventos
useEffect + fetch cuando podría ser Server Component
Llamadas a Route Handlers desde Server Components
Missing revalidatePath/revalidateTag después de mutaciones
URLs hardcodeadas con localhost
useSearchParams en pages cuando debería usar prop searchParams
Auth verificada solo en layouts (layouts no re-renderizan en navegación)
cookies()/headers() síncronos en Next.js 15


2. REACT 18/19
Nuevos Hooks React 19

use(): Solo para promises pasadas desde fuera (parent, loader, cache), NUNCA crear promise inline
useActionState: Reemplaza useFormState, manejar estado de acciones async
useFormStatus: DEBE estar en componente hijo del form, NO en el mismo componente que contiene el form
useOptimistic: Para UI optimista que auto-revierte si la acción falla

State Management

NO usar useEffect para setear estado derivado, computarlo directamente en render o con useMemo si es costoso
Estado que representa URL (filtros, paginación, búsqueda) debe estar en URL params, no useState
NO crear objetos o funciones nuevas dentro de render sin memoización cuando se pasan como props

useEffect

SIEMPRE incluir cleanup para subscriptions, timers, fetch con AbortController
Usar functional updates en setters dentro de effects para evitar dependencias stale
Dependencies array debe incluir TODO lo que se lee dentro del effect
useEffect es para sincronización con sistemas externos, NO para transformar datos

React 19 Changes

forwardRef está deprecated, ref ahora es prop normal
Refs se limpian automáticamente en unmount

Render Optimization

NO usar React.memo, useMemo, useCallback prematuramente
Usar React DevTools Profiler para identificar bottlenecks reales antes de optimizar
memo/useMemo/useCallback justificados cuando: componente renderiza frecuentemente con mismos props, computación genuinamente costosa, callbacks pasados a componentes ya memoizados

Red Flags React

useEffect con empty dependency array que lee props/state
useEffect que solo setea estado derivado
Crear promises/objects inline dentro de render
Missing cleanup en effects con subscriptions
forwardRef en proyectos React 19
useFormStatus en mismo componente que el form
Importar Server Components en Client Components
Pasar funciones como props de Server a Client Components
Index como key para listas dinámicas que pueden reordenarse
Missing ErrorBoundary alrededor de Suspense


3. TAILWIND CSS V3 → V4
Breaking Changes Críticos

v4 usa @import "tailwindcss" en vez de directivas @tailwind
v4 usa @theme para definir design tokens en CSS, no tailwind.config.js
v4 usa @utility para crear utilidades custom

Utilidades Deprecated

bg-opacity-, text-opacity- → usar sintaxis bg-black/50
flex-shrink-, flex-grow- → shrink-, grow-
shadow-sm → shadow-xs
rounded-sm → rounded-xs
outline-none → outline-hidden
ring → ring-3
!hover:bg-red-600 → hover:bg-red-600! (importante al final)

@apply es Anti-Pattern

Evitar @apply para crear clases, usar componentes en su lugar
@apply solo válido para: overrides de librerías third-party, elementos muy pequeños y reusables

Arbitrary Values

Evitar magic values como w-[347px], h-[89px], text-[#2d3748]
Usar escala de diseño existente, arbitrary values solo cuando genuinamente necesario

Dark Mode

v4: Usar CSS-first con @custom-variant y CSS variables
Theming con variables CSS que cambian en .dark, no clases duplicadas

Container Queries

v4 soporta @container nativamente
Usar para componentes responsivos a su contenedor, no solo viewport

Red Flags Tailwind

Directivas @tailwind en proyectos v4
@apply excesivo
Class ordering aleatorio (usar Prettier plugin)
Valores hardcoded everywhere
Desktop-first responsive (max-* variants en vez de mobile-first)
Utilidades deprecated
Crear tailwind.config.js en v4 cuando CSS-first bastaría


4. SHADCN/UI
Estructura

components/ui/ contiene componentes base de shadcn, NO modificar directamente
Extender mediante composición creando nuevos componentes que usen los base
Usar CLI para añadir componentes, no copiar código manualmente

cn() Utility

Siempre usar cn() de lib/utils para combinar clases
Nunca concatenación de strings para clases condicionales
Siempre permitir className prop para override externo

Forms

Usar React Hook Form + Zod siempre
zodResolver es obligatorio
defaultValues es requerido para controlled inputs
Spread field completo en inputs, no value={field.value}
FormField, FormItem, FormLabel, FormControl, FormMessage para estructura

Red Flags Shadcn

Modificar archivos en ui/ directamente
No usar cn() utility
Colores hardcoded en vez de CSS variables
Componentes monolíticos en vez de compound
Missing "use client" para componentes interactivos
Forms sin React Hook Form
Missing zodResolver
Missing defaultValues
Spread parcial de field en inputs
Copiar código manualmente vs usar CLI


5. VITE
Configuración

Minimal y intencional, NO incluir valores que ya son default
NO duplicar plugins
Usar @vitejs/plugin-react-swc para proyectos grandes (20x faster HMR)
@vitejs/plugin-react (Babel) solo cuando se necesitan custom Babel plugins

Environment Variables

Usar loadEnv() en vite.config.ts, NO process.env directo
Variables client-side deben tener prefijo VITE_

Path Aliases

Definir en vite.config.ts Y tsconfig.json, deben coincidir

Build

Usar manualChunks estratégicamente para vendor splitting
NO subir chunkSizeWarningLimit para ocultar warnings

Red Flags Vite

Plugins duplicados
Valores default explícitos innecesarios
process.env.VITE_* en config sin loadEnv
@vitejs/plugin-react sin justificación vs SWC
chunkSizeWarningLimit alto para ocultar problemas


6. ESLINT (FLAT CONFIG 2025)
Migración Obligatoria

ESLint v9: .eslintrc está deprecated, usar eslint.config.js
Flat config usa arrays de objetos, no extends/plugins strings
env property no existe en flat config, usar languageOptions.globals
.eslintignore no soportado, usar ignores en config

Config Structure

Ignores primero
Base configs (eslint, typescript-eslint)
Plugin configs
eslint-config-prettier ÚLTIMO para deshabilitar conflictos con Prettier

Red Flags ESLint

Mezclar sintaxis .eslintrc y flat config
env property en flat config
.eslintignore file en proyectos con flat config
extends como array con spread incorrecto


7. NODE.JS + EXPRESS
ESM en 2025

Usar ESM con "type": "module" en package.json
import/export, no require/module.exports
Prefijo node: para built-ins

Environment Configuration

Validar TODAS las variables de entorno al inicio con Zod
Fallar rápido si configuración es inválida
Nunca secrets hardcodeados

Error Handling

Centralizado, NO try/catch en cada ruta
Usar express-async-errors o wrapper para Express 4
Dejar que errores burbujeen al handler global
Handler global al FINAL del middleware chain

Middleware Order

Security headers (helmet)
CORS
Body parsing
Logging
Rate limiting
Authentication
Routes
Error handling (ÚLTIMO)

Validation

Validar input en boundaries con Zod
Middleware de validación reutilizable
Nunca confiar en input del cliente

Logging

Pino para producción (más rápido que Winston)
Structured logging con JSON
Redactar campos sensibles (tokens, passwords)
NO console.log en producción

Red Flags Node.js/Express

CommonJS en proyectos nuevos sin justificación
Secrets hardcodeados
Try/catch repetido en cada ruta
Business logic en middleware chains
console.log en producción
Missing input validation
SQL queries con string concatenation


8. WEBSOCKET
Librería

ws: Alta performance, bajo overhead, simple message passing
Socket.IO: Rooms, namespaces, broadcasting, browser fallback

Reconnection

NUNCA reconexión inmediata (thundering herd problem)
Exponential backoff con jitter obligatorio
Máximo de intentos antes de desistir
Reset contador en conexión exitosa

React Integration

Guardar instancia en useRef
Cleanup SIEMPRE en useEffect return con close(1000)
AbortController o flags para prevenir state updates después de unmount
Manejar reconexión fuera del componente

Red Flags WebSocket

Reconexión inmediata sin backoff
Missing cleanup en React
State updates sin verificar si componente está montado
Crear nueva conexión en cada render


9. ZUSTAND
Store Organization

Stores pequeños y enfocados por dominio, NO god stores con todo
Exportar custom hooks tipados, no el store raw
Separar actions en objeto para mejor organización

Selectors

SIEMPRE usar selectors, nunca destructuring directo del store completo
useShallow para seleccionar múltiples valores sin re-renders innecesarios
Selectors atómicos para valores individuales

Actions

Event-style (incrementPopulation), NO setter-style (setCount)
Lógica en el store, no en el componente

Middleware

devtools outermost en la cadena
persist con partialize para excluir funciones y datos temporales

Cuándo NO usar Zustand

Server data: TanStack Query / SWR
URL state: URL params (nuqs)
Form state: React Hook Form
Estado local simple: useState
SÍ usar para: Auth token/user con persist, Global UI (modals, sidebars, themes)

Red Flags Zustand

Store con 20+ fields (god store)
Destructuring sin selector
fetchX, loading, error en store (es server state)
Actions nombradas setX
Missing partialize en persist middleware


10. TYPESCRIPT
Type Inference vs Explicit

Dejar que inference trabaje para variables locales
Explicit types NECESARIOS para: parámetros de función, return types de funciones exportadas, cuando inference sería muy amplia

Generics

T debe aparecer al menos dos veces para justificar generic
Si T solo aparece una vez, no añade valor

Type Safety

Type guards con validación runtime para data externa, NO as casting sin validación
Discriminated unions para state machines
satisfies para validar Y preservar literal types
as const para preservar literal types en objetos/arrays

Red Flags TypeScript

Explicit types en cada variable local
Generic parameter usado solo una vez
as assertions para data externa sin validación
Utility types anidados profundamente sin razón
any donde unknown + narrowing funcionaría
Missing discriminant properties en unions
strict: false en tsconfig


11. TESTING
Framework

Vitest: Proyectos Vite, ESM nativo, TypeScript, proyectos nuevos
Jest: Codebase maduro con pipelines Jest existentes

React Testing Library Query Priority

PRIMERO: getByRole (accessibilidad)
SEGUNDO: getByLabelText (forms)
TERCERO: getByText
ÚLTIMO RECURSO: getByTestId

User Interactions

SIEMPRE userEvent, NUNCA fireEvent
userEvent.setup() al inicio del test
await en todas las interacciones

Async Testing

findBy para contenido async (incluye waitFor internamente)
queryBy para verificar ausencia
NUNCA waitFor + getBy para contenido async

API Mocking

MSW para mocking de red a nivel de service worker
Mock solo external boundaries, no implementaciones internas

Red Flags Testing

getBy para contenido async
fireEvent en vez de userEvent
Missing await en findBy queries
getByTestId como primera opción
Sintaxis Jest en proyectos Vitest
Over-mocking de implementaciones internas


12. PERFORMANCE
Code Splitting

React.lazy para rutas y componentes pesados
Suspense con fallback apropiado
NO over-splitting componentes pequeños

Images

Siempre width y height para evitar CLS
priority en imágenes LCP (above the fold)
Formatos modernos: WebP, AVIF
loading="lazy" para imágenes below the fold

Fonts

next/font o font-display: swap
Preload fonts críticas
Subset solo caracteres necesarios

Core Web Vitals

LCP: ≤2.5s
INP: ≤200ms (reemplaza FID)
CLS: ≤0.1
startTransition para actualizaciones no urgentes

Bundle

Importar funciones específicas de lodash, no librería completa
Analizar bundle con herramientas antes de optimizar

Red Flags Performance

Static imports para componentes pesados
Images sin width/height
Missing priority en LCP images
Scripts sin async/defer
Import de librería completa cuando solo se usa una función
Missing font-display: swap


13. COMENTARIOS
Eliminar

Comentarios que describen lo que el código ya dice
TODO/FIXME sin contexto ni ticket
Comentarios con tono tutorial o corporativo
Comentarios que explican sintaxis del lenguaje
JSDoc vacíos o con @param sin descripción real

Conservar

Comentarios que explican POR QUÉ, no QUÉ
Referencias a tickets, issues, decisiones de arquitectura
Advertencias sobre edge cases no obvios
Workarounds documentados con referencia al bug


14. CÓDIGO MUERTO Y DEBUGGING
Eliminar Siempre

console.log, console.warn, console.error de debugging
Código comentado "por si acaso"
Funciones no llamadas desde ningún lugar
Variables declaradas pero no usadas
Imports no usados
Features completas que nadie pidió


15. ESTRUCTURAS INFLADAS
Simplificar

Variables temporales inútiles antes de return
Wrappers que solo llaman a otra función sin añadir nada
if/else que retorna booleano cuando puede retornar la condición directamente
Early returns innecesarios que complican el flujo
Funciones de 5+ líneas que pueden ser 1-2 sin perder claridad


16. HOOKS Y UTILS DUPLICADOS
Verificar Antes de Crear

useToggle, useBoolean, useDisclosure
useDebounce, useThrottle
usePrevious, useLocalStorage, useClickOutside
cn(), clsx(), classNames() — usar solo UNO
isEmpty, isNil, formatDate, formatCurrency, debounce, throttle

Si ya existe en el proyecto o en dependencias, NO crear otro.

17. SERVICES Y API CLIENTS
Eliminar

Axios instances con interceptors que no se usan
Clases Service con CRUD completo cuando solo se usa 1-2 métodos
try/catch + toast.error en cada método cuando hay manejo global
Wrappers de fetch que solo añaden Content-Type

Patrón Correcto

Un solo cliente HTTP configurado globalmente
Funciones individuales por endpoint
Error handling en el boundary


18. CONSTANTES
Regla

Cada constante en exactamente UN lugar
ROUTES, API_ENDPOINTS, STATUS, ROLES centralizados
Colores, spacing, breakpoints en theme/tailwind config
Magic strings/numbers extraídos con nombres descriptivos


19. COMPONENTES UI
No Reinventar

Si el proyecto usa shadcn/radix/mantine: NO crear Button, Card, Modal, Table, Dropdown custom
NO crear LoadingSpinner de 70 líneas cuando existe uno global
NO crear ErrorBoundary por página

Válido Crear

Componentes con lógica de negocio específica
Composiciones que añaden comportamiento real


20. REACT QUERY / TANSTACK QUERY
Centralizar

Query keys con factory o constantes
Defaults globales en QueryClient
Custom hooks por dominio, no queries sueltas

Eliminar

queryKey construidos diferente en cada archivo
select: (data) => data.data repetido
staleTime, cacheTime hardcodeados en cada query
onError con toast en cada query cuando hay handler global


21. KEYS EN LISTAS

key={item.id} cuando el item tiene ID único — SIEMPRE
key={index} VÁLIDO para listas estáticas que nunca reordenan
key={index} INCORRECTO para listas dinámicas que pueden reordenar/filtrar


22. DATA-TESTID

NO eliminar si el proyecto tiene tests que los usan
NO añadir a cada div "porque sí"
Añadir solo en elementos interactivos que necesitan ser testeados


23. CSS / LAYOUT
Eliminar

Divs anidados solo para centrar — una clase basta
grid-cols-1 md:grid-cols-2 lg:grid-cols-3 repetido 200 veces — crear componente Grid
flex flex-col gap-4 everywhere — crear componente Stack
Colores hardcodeados fuera del theme
backdrop-filter: blur(120px) en móviles — mata performance

Preferir

CSS animations sobre JS animations cuando sea posible
loading="lazy" en imágenes below-the-fold
font-display: swap en fuentes custom


24. IMPORTS Y EXPORTS
Eliminar

Imports no usados
Imports duplicados
index.ts que re-exporta 150 cosas sin organización
Barrel files que causan bundle bloat

Verificar

Tree-shaking funciona correctamente
Imports específicos de lodash, no librería completa


25. ARCHIVOS RED FLAG
Revisar y refactorizar:

lib/utils.ts con 500+ líneas
types/index.ts exportando todo
constants/index.ts con re-exports masivos
components/ui/ con 50+ componentes usándose 8


26. PALABRAS QUE INDICAN SLOP
En comentarios o documentación son red flags:
robust, resilient, seamless, leverage, comprehensive, cutting-edge, state-of-the-art, revolutionize, game-changing, empower, enhance, streamline, optimize
En código técnico pueden ser válidas si describen funcionalidad real.
---

## REGLA FINAL

> Si un ingeniero senior con 10+ años en este codebase exacto miraría el código y diría "esto lo escupió una IA en 3 segundos", **eliminarlo o reescribir**.

El código debe parecer escrito por alguien que:
1. Conoce el proyecto íntimamente
2. Tomó decisiones conscientes
3. No copió de tutoriales genéricos

---

## EJECUCIÓN

Al completar la limpieza, reportar:

```
✓ Slop eliminado. Código limpio.
```

Nunca dejar rastro de código generado por IA sin criterio humano.