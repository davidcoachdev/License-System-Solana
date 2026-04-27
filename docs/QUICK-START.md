# 🚀 Quick Start Guide - License System TUI

**Última actualización**: 2026-04-27  
**Versión**: Sprint 1 (feat/sprint1-sdk-modular)

---

## 📋 Requisitos Previos

- Rust 1.89+
- Solana CLI 3.1+
- Anchor 0.32+
- Surfpool instalado
- Wallet configurada en `~/.config/solana/id.json`

---

## 🎮 Cómo Ejecutar el Proyecto

### **Paso 1: Iniciar Surfpool (Validador Local)**

```bash
# Iniciar Surfpool
surfpool start
```

**Nota**: Surfpool es más estable que `solana-test-validator` — no se cae.

**Verificar que está corriendo**:
```bash
solana config set --url localhost
solana balance
```

Deberías ver: `500000000 SOL`

---

### **Paso 2: Deploy el Program**

```bash
# Ir al worktree Sprint 1
cd ~/Proyects/License-System-on-Solana-sprint1/license-system

# Configurar red local
solana config set --url localhost

# Deploy program
solana program deploy target/deploy/license_system.so
```

**Output esperado**:
```
Program Id: H93kfiExrB3hCMCxkvnCRDC3PGuGC5cyL7eh6VfmgfaB
Signature: <tx_signature>
```

**Verificar deployment**:
```bash
solana program show H93kfiExrB3hCMCxkvnCRDC3PGuGC5cyL7eh6VfmgfaB
```

---

### **Paso 3: Ejecutar la TUI**

```bash
# Ir al directorio del proyecto
cd ~/Proyects/License-System-on-Solana-sprint1/license-system

# Ejecutar TUI
ANCHOR_WALLET=/home/dcdebian/.config/solana/id.json ./target/release/license-tui
```

**Nota**: Si el binary no existe, compilar primero:
```bash
cargo build --release --package license-tui
```

---

## ⌨️ Controles de la TUI

### Navegación General
- `↑↓` — Navegar por el menú
- `Enter` — Seleccionar opción
- `ESC` — Volver al menú anterior
- `q` — Salir (con confirmación)
- `F1` o `?` — Abrir help popup

### Formularios
- `↑↓` — Navegar entre campos
- `←→` — Cambiar opción en select fields
- `Type` — Escribir en campos de texto
- `Backspace` — Borrar
- `Enter` — Submit formulario

### Modals
- `←→` — Cambiar selección (Yes/No)
- `Enter` — Confirmar
- `ESC` — Cancelar
- `Any key` — Cerrar notification modal

---

## 📝 Operaciones Disponibles

### 1. Issue License (Crear Licencia)

**Campos**:
- Owner Pubkey: `3whY1ohdAV3uRXSpyzsKtwLg84X9fTZ1pSdCS8Vvqt7c`
- Product Plan: Basic, Premium, Enterprise, Developer, Startup (select)
- Duration: 30, 60, 90, 180, 365 días (select)

**Resultado**: Modal verde (✅) con signature

---

### 2. Extend License (Extender Licencia)

**Campos**:
- Select License: Dropdown con licencias existentes
- Additional Days: 15, 30, 60, 90 días (select)

**Validación**:
- ✅ Verifica que la licencia exista
- ✅ Verifica que NO esté revocada

**Resultado**: Modal verde (✅) con signature

---

### 3. Validate License (Validar Licencia)

**Campos**:
- Select License: Dropdown con licencias existentes
- Product Plan: Select con planes

**Validación**:
- ✅ Verifica: !is_revoked && expires_at > now && product_id matches

**Resultado**: 
- Modal verde (✅) si es válida
- Modal rojo (❌) si es inválida (con razón)

---

### 4. Revoke License (Revocar Licencia)

**Campos**:
- Select License: Dropdown con licencias existentes

**Acción**:
- ✅ Revoca la licencia (permanente)
- ✅ Guarda en historial: `~/.config/license-tui/revoked_history.json`

**Resultado**: Modal verde (✅) con signature y ruta del historial

---

### 5. List License (Ver Detalles)

**Campos**:
- Select License: Dropdown con licencias existentes

**Resultado**: Modal azul (ℹ️) con detalles completos

---

### 6. View All Licenses (Ver Todas)

**Acción**: Muestra tabla con TODAS las licencias

**Formato**:
```
Total Licenses: 2

1. Owner: 3whY1o...qt7c
   Product: developer-plan
   Expires: 1735689600
   Status: ✅ Active

2. Owner: 3whY1o...qt7c
   Product: premium-plan
   Expires: 1733097600
   Status: ❌ Revoked
```

**Controles**: ESC para volver

---

### 7. Revoked History (Historial de Revocadas)

**Acción**: Muestra historial de licencias revocadas (desde JSON)

**Formato**:
```
Total Revoked: 1

1. Owner: 3whY1o...qt7c
   Product: developer-plan
   Revoked At: 1735689600
   Signature: 59gKn3W6iW3gzhQFTVw...

History file: ~/.config/license-tui/revoked_history.json
```

**Controles**: ESC para volver

---

### 8. Settings

**Opciones**:
- **Theme**: 6 temas (Dc Studio, Dark, Light, Dracula, Nord, Gruvbox)
- **Network**: Localnet, Devnet, Mainnet (password "dc-ok" para mainnet)

---

## 🎨 Temas Disponibles

1. **Dc Studio** (default) — Dark red/burgundy
2. **Dark** — Dark blue
3. **Light** — Light theme
4. **Dracula** — Dracula colors
5. **Nord** — Nord colors
6. **Gruvbox** — Gruvbox colors

---

## 🔧 Troubleshooting

### Surfpool no inicia
```bash
# Verificar si está corriendo
ps aux | grep surfpool

# Matar proceso
pkill -9 surfpool

# Reiniciar
surfpool start
```

### Program no existe
```bash
# Verificar program
solana program show H93kfiExrB3hCMCxkvnCRDC3PGuGC5cyL7eh6VfmgfaB

# Si no existe, deploy de nuevo
cd ~/Proyects/License-System-on-Solana-sprint1/license-system
solana program deploy target/deploy/license_system.so
```

### TUI no inicia
```bash
# Verificar wallet
ls -la ~/.config/solana/id.json

# Verificar ANCHOR_WALLET
echo $ANCHOR_WALLET

# Ejecutar con variable explícita
ANCHOR_WALLET=/home/dcdebian/.config/solana/id.json ./target/release/license-tui
```

### Error "DeclaredProgramIdMismatch"
```bash
# Recompilar program
cd ~/Proyects/License-System-on-Solana-sprint1/license-system
anchor build

# Deploy nuevo binary
solana program deploy target/deploy/license_system.so
```

---

## 📊 Estado del Proyecto

### Sprint 1 (COMPLETADO ✅)
- ✅ Estructura reorganizada
- ✅ SDK modular con transacciones reales
- ✅ TUI profesional con 9 opciones
- ✅ Primera licencia creada exitosamente
- ✅ 28 commits en worktree

### Sprint 2 (Próximo)
- [ ] Validación de Revoke (como Extend)
- [ ] Fix bloqueo en View All/History
- [ ] Persistencia de tema (guardar en JSON)
- [ ] Progress modal con fondo sólido
- [ ] Validar red actual antes de cambiar

---

## 🔗 Recursos

- [AGENTS.md](../AGENTS.md) — Estrategia de trabajo
- [TUI User Manual](./TUI-USER-MANUAL.md) — Manual completo
- [Comparison](./COMPARISON-TRUST-WORK-ESCROW.md) — Gap analysis

---

## 👥 Información

**Wallet**: `3whY1ohdAV3uRXSpyzsKtwLg84X9fTZ1pSdCS8Vvqt7c`  
**Program ID**: `H93kfiExrB3hCMCxkvnCRDC3PGuGC5cyL7eh6VfmgfaB`  
**Network**: Localnet (http://localhost:8899)  
**Balance**: 500M SOL

---

**¡Listo para mañana, papus!** 🚀
