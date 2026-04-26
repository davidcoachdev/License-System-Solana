# 📖 Manual de Usuario - License System TUI

**Versión**: 1.0.0  
**Última actualización**: 2026-04-26

---

## 🎯 ¿Qué es License System TUI?

License System TUI es una **interfaz de terminal interactiva** (Terminal User Interface) construida con Ratatui para gestionar licencias en Solana blockchain.

Permite a los administradores del sistema:
- ✅ Emitir licencias nuevas
- ✅ Extender licencias existentes
- ✅ Validar el estado de licencias
- ✅ Revocar licencias
- ✅ Listar información de licencias

---

## 🚀 Inicio Rápido

### Requisitos Previos
- Solana Test Validator corriendo (localnet)
- Wallet configurada en `~/.config/solana/id.json`
- Program deployado en localnet

### Iniciar la TUI

**Opción 1: Script automático**
```bash
cd /home/dcdebian/Proyects/License-System-on-Solana/crates/tui
./run-tui.sh
```

**Opción 2: Manual**
```bash
cd /home/dcdebian/Proyects/License-System-on-Solana/crates/tui
export ANCHOR_WALLET=/home/dcdebian/.config/solana/id.json
./target/release/license-tui
```

---

## ⌨️ Controles

| Tecla | Acción |
|-------|--------|
| `↑` `↓` | Navegar por el menú |
| `1-6` | Selección rápida de opción |
| `Enter` | Confirmar acción / Ejecutar |
| `ESC` | Volver al menú principal |
| `q` o `6` | Salir de la aplicación |
| `Backspace` | Borrar caracteres en input |

---

## 📋 Funciones Disponibles

### 1️⃣ Issue License (Emitir Licencia)

**Propósito**: Crear una nueva licencia para un owner específico.

**Formato de entrada**:
```
owner_pubkey,product_id,days
```

**Ejemplo**:
```
3whY1ohdAV3uRXSpyzsKtwLg84X9fTZ1pSdCS8Vvqt7c,premium-plan,30
```

**Parámetros**:
- `owner_pubkey` — Pubkey del dueño de la licencia (43-44 caracteres)
- `product_id` — ID del producto (máximo 64 caracteres)
- `days` — Duración en días (número entero positivo)

**Qué hace**:
1. Deriva la PDA (Program Derived Address) para el owner
2. Crea una cuenta de licencia on-chain
3. Establece: owner, product_id, expires_at (timestamp), is_revoked (false)
4. Retorna la signature de la transacción

**Resultado**:
```
✅ Ready to issue license!
PDA: <license_pda>
Bump: <bump_seed>
Owner: <owner_pubkey>
Product: <product_id>
Days: <duration_days>
Payer: <payer_pubkey>
```

**Errores comunes**:
- `Invalid owner pubkey` — El pubkey no tiene formato válido
- `Invalid days number` — Los días no son un número válido
- `Format: owner_pubkey,product_id,days` — Formato incorrecto (faltan comas)

---

### 2️⃣ Extend License (Extender Licencia)

**Propósito**: Agregar más días a una licencia existente.

**Formato de entrada**:
```
owner_pubkey,additional_days
```

**Ejemplo**:
```
3whY1ohdAV3uRXSpyzsKtwLg84X9fTZ1pSdCS8Vvqt7c,15
```

**Parámetros**:
- `owner_pubkey` — Pubkey del dueño de la licencia
- `additional_days` — Días adicionales a agregar (número entero positivo)

**Qué hace**:
1. Deriva la PDA de la licencia
2. Verifica que la licencia NO esté revocada
3. Verifica que NO haya expirado hace más de 7 días (grace period)
4. Suma `additional_days * 24 * 60 * 60` al `expires_at`
5. Retorna la signature de la transacción

**Resultado**:
```
✅ Ready to extend license!
PDA: <license_pda>
Bump: <bump_seed>
Additional days: <days>
```

**Errores comunes**:
- `License has been revoked` — No se puede extender una licencia revocada
- `License has expired and cannot be extended` — Expiró hace más de 7 días
- `Unauthorized` — Solo el owner puede extender su licencia

---

### 3️⃣ Validate License (Validar Licencia)

**Propósito**: Verificar si una licencia es válida (no revocada, no expirada, product_id correcto).

**Formato de entrada**:
```
owner_pubkey,product_id
```

**Ejemplo**:
```
3whY1ohdAV3uRXSpyzsKtwLg84X9fTZ1pSdCS8Vvqt7c,premium-plan
```

**Parámetros**:
- `owner_pubkey` — Pubkey del dueño de la licencia
- `product_id` — ID del producto a validar

**Qué hace**:
1. Deriva la PDA de la licencia
2. Verifica 3 condiciones:
   - `!license.is_revoked` — NO está revocada
   - `license.expires_at > now` — NO ha expirado
   - `license.product_id == product_id` — Product ID coincide
3. Retorna `true` si todas las condiciones se cumplen, `false` si alguna falla

**Resultado**:
```
✅ Ready to validate license!
PDA: <license_pda>
Bump: <bump_seed>
Product: <product_id>
```

**Errores comunes**:
- `Account does not exist` — La licencia no existe para ese owner
- `Format: owner_pubkey,product_id` — Formato incorrecto

---

### 4️⃣ Revoke License (Revocar Licencia)

**Propósito**: Revocar una licencia existente (marcarla como inválida permanentemente).

**Formato de entrada**:
```
owner_pubkey
```

**Ejemplo**:
```
3whY1ohdAV3uRXSpyzsKtwLg84X9fTZ1pSdCS8Vvqt7c
```

**Parámetros**:
- `owner_pubkey` — Pubkey del dueño de la licencia

**Qué hace**:
1. Deriva la PDA de la licencia
2. Verifica que NO esté ya revocada
3. Establece `license.is_revoked = true`
4. Retorna la signature de la transacción

**Resultado**:
```
✅ Ready to revoke license!
PDA: <license_pda>
Bump: <bump_seed>
```

**Errores comunes**:
- `License has already been revoked` — La licencia ya estaba revocada
- `Unauthorized` — Solo el owner puede revocar su licencia
- `Account does not exist` — La licencia no existe

**⚠️ IMPORTANTE**: La revocación es **permanente** y **no se puede deshacer**.

---

### 5️⃣ List Licenses (Listar Licencias)

**Propósito**: Mostrar información detallada de una licencia.

**Formato de entrada**:
```
owner_pubkey
```

**Ejemplo**:
```
3whY1ohdAV3uRXSpyzsKtwLg84X9fTZ1pSdCS8Vvqt7c
```

**Parámetros**:
- `owner_pubkey` — Pubkey del dueño de la licencia

**Qué hace**:
1. Deriva la PDA de la licencia
2. Muestra información calculada:
   - PDA (Program Derived Address)
   - Bump seed
   - Owner pubkey
   - Payer pubkey
   - Program ID

**Resultado**:
```
📋 License Info:
PDA: <license_pda>
Bump: <bump_seed>
Owner: <owner_pubkey>
Payer: <payer_pubkey>
Program ID: <program_id>
```

**Nota**: En modo demo, solo muestra la PDA derivada. Para ver datos reales de la licencia (product_id, expires_at, is_revoked), usar los tests TypeScript.

---

### 6️⃣ Exit (Salir)

**Propósito**: Cerrar la aplicación TUI.

**Atajos**:
- Presionar `6`
- Presionar `q`
- Desde cualquier pantalla: `ESC` → `6`

---

## 🔐 Modelo de Seguridad

### PDAs (Program Derived Addresses)

Cada licencia se almacena en una **PDA única** derivada del owner:

```rust
seeds = [b"license", owner.pubkey().as_ref()]
```

**Características**:
- ✅ Determinística (siempre la misma PDA para el mismo owner)
- ✅ Sin colisiones (cada owner tiene su propia PDA)
- ✅ No necesita almacenamiento externo (se calcula on-demand)

### Restricciones de Acceso

| Operación | Quién puede ejecutarla |
|-----------|------------------------|
| Issue License | Solo cuando `owner == authority` |
| Extend License | Solo el owner de la licencia |
| Revoke License | Solo el owner de la licencia |
| Validate License | Cualquiera (read-only) |

### Grace Period

Las licencias tienen un **grace period de 7 días** después de expirar:
- ✅ Dentro del grace period: se puede extender
- ❌ Fuera del grace period: NO se puede extender (debe emitirse una nueva)

---

## 🐛 Troubleshooting

### Error: "SDK not initialized"
**Causa**: La wallet no se cargó correctamente

**Solución**:
```bash
export ANCHOR_WALLET=/home/dcdebian/.config/solana/id.json
./target/release/license-tui
```

### Error: "Invalid owner pubkey"
**Causa**: El pubkey no tiene formato válido (debe ser base58, 43-44 caracteres)

**Solución**: Verificar que el pubkey sea correcto. Ejemplo válido:
```
3whY1ohdAV3uRXSpyzsKtwLg84X9fTZ1pSdCS8Vvqt7c
```

### Error: "Format: ..."
**Causa**: El formato de entrada no coincide con el esperado

**Solución**: Seguir el formato exacto mostrado en cada pantalla:
- Issue: `owner,product,days`
- Extend: `owner,days`
- Validate: `owner,product`
- Revoke: `owner`
- List: `owner`

### TUI no responde
**Causa**: El validador de Solana no está corriendo

**Solución**:
```bash
# Terminal 1: Iniciar validador
solana-test-validator --reset

# Terminal 2: Ejecutar TUI
cd crates/tui
ANCHOR_WALLET=/home/dcdebian/.config/solana/id.json ./target/release/license-tui
```

### Error: "Failed to load keypair"
**Causa**: El archivo de keypair no existe en la ruta especificada

**Solución**:
```bash
# Verificar que existe
ls -la ~/.config/solana/id.json

# Si no existe, generar uno nuevo
solana-keygen new --outfile ~/.config/solana/id.json
```

---

## 📊 Flujo de Uso Típico

### Escenario 1: Emitir y Validar Licencia

1. **Iniciar TUI** → Menú principal
2. **Presionar `1`** → Issue License
3. **Ingresar**: `<owner_pubkey>,premium-plan,30`
4. **Presionar `Enter`** → Ver PDA y detalles
5. **Presionar `ESC`** → Volver al menú
6. **Presionar `3`** → Validate License
7. **Ingresar**: `<owner_pubkey>,premium-plan`
8. **Presionar `Enter`** → Ver resultado de validación
9. **Presionar `ESC`** → Volver al menú

### Escenario 2: Extender Licencia

1. **Iniciar TUI** → Menú principal
2. **Presionar `2`** → Extend License
3. **Ingresar**: `<owner_pubkey>,15`
4. **Presionar `Enter`** → Ver confirmación
5. **Presionar `ESC`** → Volver al menú

### Escenario 3: Revocar Licencia

1. **Iniciar TUI** → Menú principal
2. **Presionar `4`** → Revoke License
3. **Ingresar**: `<owner_pubkey>`
4. **Presionar `Enter`** → Ver confirmación
5. **⚠️ ADVERTENCIA**: La revocación es permanente
6. **Presionar `ESC`** → Volver al menú

---

## 🔍 Información Técnica

### Arquitectura

```
TUI (Ratatui)
    ↓
SDK (Rust)
    ↓
Solana RPC
    ↓
License Program (on-chain)
```

### Componentes

| Componente | Descripción |
|------------|-------------|
| **TUI** | Interfaz de usuario (Ratatui + Crossterm) |
| **SDK** | Abstracción de Solana RPC (license-sdk) |
| **Program** | Smart contract en Solana (Anchor) |

### Program ID

```
5pXEX8z1aTSnm7jCKqvJCXezczKPVuPQif2BZh5u5Axq
```

### Estructura de Licencia

```rust
pub struct License {
    pub owner: Pubkey,        // 32 bytes
    pub product_id: String,   // max 64 chars
    pub expires_at: i64,      // Unix timestamp
    pub is_revoked: bool,     // 1 byte
}
```

**Tamaño total**: ~120 bytes + discriminator (8 bytes)

---

## 🎨 Capturas de Pantalla

### Menú Principal
```
┌─────────────────────────────────────────────────┐
│ License System on Solana - TUI                  │
└─────────────────────────────────────────────────┘
┌ Main Menu - Use ↑↓ or numbers to select ────────┐
│ 1. Issue License                                │
│ 2. Extend License                               │
│ 3. Validate License                             │
│ 4. Revoke License                               │
│ 5. List Licenses                                │
│ 6. Exit                                         │
└─────────────────────────────────────────────────┘
┌ Status ──────────────────────────────────────────┐
│ Connected to Solana localnet                    │
└─────────────────────────────────────────────────┘
```

### Pantalla de Input (Issue License)
```
┌─────────────────────────────────────────────────┐
│ License System on Solana - TUI                  │
└─────────────────────────────────────────────────┘
┌ IssueLicense - Press ESC to return ─────────────┐
│ 3whY1ohdAV3uRXSpyzsKtwLg84X9fTZ1pSdCS8Vvqt7c,p│
│                                                 │
└─────────────────────────────────────────────────┘
┌ Status ──────────────────────────────────────────┐
│ Issue License - Enter: owner_pubkey,product_id,d│
└─────────────────────────────────────────────────┘
```

---

## 💡 Tips y Mejores Prácticas

### 1. Validar antes de Extender
Siempre valida una licencia antes de extenderla para verificar que existe y no está revocada.

### 2. Guardar PDAs
Aunque las PDAs se calculan on-demand, es útil guardar las PDAs de licencias activas para referencia rápida.

### 3. Grace Period
Recuerda que las licencias tienen 7 días de grace period después de expirar. Planifica las extensiones con anticipación.

### 4. Product ID Consistency
Usa product IDs consistentes (ej: `premium-plan`, `basic-plan`) para facilitar la validación.

### 5. Testing
Usa el validador local (`solana-test-validator`) para testing antes de deployar a devnet/mainnet.

---

## 🔗 Recursos Adicionales

### Documentación del Proyecto
- [README Principal](../../README.md)
- [Phase 1: Core Program](../phases/phase-1/README.md)
- [Phase 2: SDK + TUI](../phases/phase-2/README.md)
- [Phase 3: Backend + Frontend](../phases/phase-3/README.md)

### Comandos Útiles

**Iniciar validador local**:
```bash
solana-test-validator --reset
```

**Deploy program**:
```bash
cd license-system
anchor deploy
```

**Ejecutar tests TypeScript**:
```bash
cd license-system
export ANCHOR_WALLET=~/.config/solana/id.json
yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/license-system.ts
```

**Ver logs del validador**:
```bash
tail -f test-ledger/validator.log
```

**Verificar program deployado**:
```bash
solana program show 5pXEX8z1aTSnm7jCKqvJCXezczKPVuPQif2BZh5u5Axq
```

---

## 📞 Soporte

### Reportar Issues
Si encontrás bugs o problemas, documentalos con:
- Versión de la TUI
- Comando ejecutado
- Error message completo
- Logs del validador (si aplica)

### Contribuir
Pull requests son bienvenidos. Por favor:
1. Seguir las convenciones del proyecto
2. Agregar tests para nuevas features
3. Actualizar esta documentación

---

## 📝 Notas de Versión

### v1.0.0 (2026-04-26)
- ✅ Implementación inicial con Ratatui
- ✅ 6 opciones: Issue, Extend, Validate, Revoke, List, Exit
- ✅ Integración con SDK
- ✅ PDA derivation
- ✅ Input validation
- ✅ Status bar con feedback
- ⚠️ Modo demo: muestra información pero no ejecuta transacciones reales
- 📝 Para transacciones reales, usar tests TypeScript

---

## ⚠️ Limitaciones Conocidas

### Modo Demo
La versión actual de la TUI funciona en **modo demo**:
- ✅ Valida inputs
- ✅ Deriva PDAs correctamente
- ✅ Muestra información detallada
- ❌ NO ejecuta transacciones reales en Solana

**Para transacciones reales**, usar los tests TypeScript:
```bash
cd license-system
yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/license-system.ts
```

### Una Licencia por Owner
El diseño actual permite **una sola licencia por owner**. Si necesitás múltiples licencias por owner (diferentes productos), el seed de la PDA debe incluir el `product_id`:

```rust
seeds = [b"license", owner.as_ref(), product_id.as_bytes()]
```

### Sin Persistencia Local
La TUI no guarda historial de operaciones. Todas las operaciones se registran on-chain pero no hay cache local.

---

## 🚀 Roadmap Futuro

### v1.1.0 (Próximo)
- [ ] Transacciones reales (no solo demo)
- [ ] Fetch de datos on-chain (mostrar licencias existentes)
- [ ] Historial de transacciones
- [ ] Múltiples wallets

### v1.2.0
- [ ] Múltiples licencias por owner
- [ ] Búsqueda y filtrado
- [ ] Export a CSV/JSON
- [ ] Estadísticas y analytics

### v2.0.0
- [ ] Integración con backend API
- [ ] Sincronización con PostgreSQL
- [ ] Notificaciones de expiración
- [ ] Renovación automática

---

**Última actualización**: 2026-04-26  
**Autor**: License System Team  
**Licencia**: [Agregar licencia]
