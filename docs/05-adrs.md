# 05 - Architecture Decision Records

Registra las decisiones arquitectónicas significativas: qué se decidió, por qué, y qué alternativas se consideraron.

---

## ADR-001: Estado mínimo on-chain

**Status:** Accepted  
**Date:** 2024-01-01  
**Context:** Diseñar el smart contract para licencias

**Decision:** Mantener estado mínimo en Solana. Solo `owner`, `product_id`, `expires_at`, `is_revoked`.

**Alternativas consideradas:**

1. **Full state on-chain** — guardar planes, seat counts, features flags, metadata completa
2. **Pure metadata** — solo una referencia a IPFS/CID, estado completo off-chain
3. **Minimal state** (elegida) — solo datos inmutables post-emisión

**Razonamiento:** Mantener el smart contract simple y cheap de operar. Si hay un bug, los fondos no están en juego. La lógica de producto vive off-chain. Esto maximiza flexibilidad y reduce el riesgo de bugs on-chain irreversibles.

**Consecuencias:** PostgreSQL es read model. Consistencia eventual en DB. Requiere indexer.

---

## ADR-002: SDK en Rust como single source of truth

**Status:** Accepted  
**Date:** 2024-01-01  
**Context:** CLI y backend necesitan interactuar con Solana

**Decision:** Un crate SDK en Rust (`license-sdk`) usado por CLI y backend. No hay bindings separate.

**Alternativas consideradas:**

1. **SDK duplicated** — bindings distintos para CLI y backend
2. **SDK único en Rust** (elegida) — crate compartida
3. **Wrapper HTTP** — backend se comunica con servicio que habla con Solana

**Razonamiento:** Evita duplicación de lógica de IX building y error parsing. Rust es el stack nativo de Solana. Un solo lugar para fixear bugs.

**Consecuencias:** Frontend web no usa SDK directo (JS/TS wrapper). CLI y Backend comparten 100% del código de integración.

---

## ADR-003: Indexer basado en webhooks, no polling

**Status:** Accepted  
**Date:** 2024-01-01  
**Context:** Mantener PostgreSQL sincronizado con estado on-chain

**Decision:** Usar Helius Webhooks para detectar eventos. Polling como fallback.

**Alternativas consideradas:**

1. **Polling constante** — `getProgramAccounts` cada N segundos
2. **Webhooks de Helius** (elegida) — push en tiempo real
3. **gRPC subscription** — requieren nodo propio

**Razonamiento:** Webhooks son más cheap que polling (menos RPC calls). gRPC subscriptions requieren correr un nodo validator propio, overkill para este caso. Helius tiene webhook support listo.

**Consecuencias:** Dependencia de Helius para el path rápido. Polling fallback agrega latency pero garantiza delivery.

---

## ADR-004: Licencias intransferibles

**Status:** Accepted  
**Date:** 2024-01-01  
**Context:** Diseñar invariantes de dominio

**Decision:** No existe operación `transfer_license`. Owner es inmutable.

**Alternativas consideradas:**

1. **Transferibles** — agregar IX `transfer_license`
2. **Intransferibles** (elegida) — no hay transfer

**Razonamiento:** Transferir licencias requiere lógica anti-fraude (no transfers durante disputes). Para un sistema B2B/B2C donde el software verifica acceso, no tiene sentido que el usuario transfiera la licencia a otro. Simplicidad > flexibilidad.

**Consecuencias:** Si un usuario cambia de wallet, necesita una nueva licencia. No hay migración automática.

---

## ADR-005: Offline validation con firma de transacción

**Status:** Accepted  
**Date:** 2024-01-01  
**Context:** Validación offline como diferenciador

**Decision:** Licencia se firma con mensaje que incluye `owner + product_id + expires_at + tx_sig`. Cliente verifica la firma sin network.

**Alternativas consideradas:**

1. **Cache local + timestamp** — guardar estado en archivo, verificar expiry
2. **Firma cryptográfica** (elegida) — sign del admin keypair
3. **Merkle proof** — proof on-chain verificado localmente

**Razonamiento:** Cache simple permite tampering trivial. Merkle proof requiere sync complejo. Firma cryptográfica equilibra seguridad (no se puede forgehear) con simplicidad (1 signature check).

**Consecuencias:** Admin wallet debe estar disponible para emitir. Offline validation verifica签名, no solo estado.

---

## ADR-006: PostgreSQL como read model

**Status:** Accepted  
**Date:** 2024-01-01  
**Context:** DB como proyección de estado on-chain

**Decision:** PostgreSQL (no Redis) como primary read model. Redis solo para cache de alta frecuencia.

**Alternativas consideradas:**

1. **Redis puro** — todo en Redis, Solana como backup
2. **PostgreSQL como read model** (elegida) — estado canonical en Postgres
3. **Dual write** — escribir ambos

**Razonamiento:** PostgreSQL tiene schema, migraciones, SQL queries, backup tools maduros. Licencias son datos relacionales (user → product). Redis como cache puede perder datos por eviction. No hay presupuesto para Redis cluster dedicado.

**Consecuencias:** Consistencia eventual. Indexer debe mantener sync. Queries complejas posible con SQL (group by, aggregations).

---

## ADR-007: Anchor 0.30+ para el programa

**Status:** Accepted  
**Date:** 2024-01-01  
**Context:** Elegir framework para smart contract

**Decision:** Usar Anchor 0.30+ sobre writing vanilla Solana SDK.

**Alternativas consideradas:**

1. **Vanilla SDK** — máximo control, más código boilerplate
2. **Anchor 0.30+** (elegida) — IDL, checks, CPI helpers
3. **Sealevel** — si necesidad de performance extrema

**Razonamiento:** Anchor reduce bugs (account checks, signer checks), genera IDL automático, y tiene ecosistema establecido. El overhead de usar Anchor es mínimo vs el beneficio de IDL y validación.

**Consecuencias:** Programa usa Anchor IDL. Migraciones futuras más complejas (requiere upgrade authority o migración manual).

---

## ADR-008: TUI como herramienta principal de admin

**Status:** Accepted  
**Date:** 2024-01-01  
**Context:** Interfaz de administración del sistema

**Decision:** TUI interactiva (Ratatui) como herramienta principal. CLI para scripting. Backend API para automatización.

**Alternativas consideradas:**

1. **Solo CLI** — simple, no hay TUI
2. **Panel web admin** — más features, más código
3. **TUI + CLI + API** (elegida) — mejor UX para cada caso

**Razonamiento:** Admin no quiere abrir un browser para revoke una licencia. TUI es rápida y local. CLI sirve para scripts y CI. API sirve para integración con backend.

**Consecuencias:** Tres interfaces para mantener. Ratatui tiene curva de aprendizaje.

---

## ADR-009: Expiración como única fuente de plan

**Status:** Accepted  
**Date:** 2024-01-01  
**Context:** Modelo de licensing

**Decision:** No hay "planes" en blockchain. `expires_at` determina validez temporal. Features del plan se manejan off-chain.

**Alternativas consideradas:**

1. **Planes on-chain** — enum de planes en el contract
2. **Solo timestamp** (elegida) — expires_at único parámetro temporal
3. **Features flags on-chain** — bitset de features

**Razonamiento:** Un sistema de "planes" complica el contract y las migraciones entre planes. Si solo guardamos cuándo expira, el producto/cliente maneja la lógica de features off-chain. Esto da máxima flexibilidad.

**Consecuencias:** El software cliente debe interpretar qué significa la licencia (no está en la chain). Requiere documentación del producto.

---

## ADR-010: Backend firma transacciones de emisión

**Status:** Accepted  
**Date:** 2024-01-01  
**Context:** UX de emisión de licencias

**Decision:** En Fase 1, el admin firma directamente via CLI/TUI. El backend no firma por sí mismo (no hay backend en Fase 1).

**Alternativas consideradas:**

1. **Backend firma** — UX simple, menos wallets
2. **Admin firma directo** (elegida) — máxima descentralización
3. **MPC / threshold** — overkill para el caso

**Razonamiento:** Backend en Fase 3 tiene su propia wallet, pero la UX de "admin approve en TUI" es más descentralizada. El backend procesa pagos, luego invoca una IX que el admin firma desde la CLI, o el admin wallet vive en el backend.

**Consecuencias:** En la práctica, Fase 3 probablemente sea "backend wallet" como admin signer. Decisión final se toma cuando se diseña el flujo de pagos.

---

## ADR-011: Rate limiting por wallet + IP

**Status:** Accepted  
**Date:** 2024-01-01  
**Context:** Rate limiting en backend API

**Decision:** Rate limit por wallet address (no IP) para endpoints de usuario. IP-based como fallback para endpoints sin wallet.

**Alternativas consideradas:**

1. **IP-only** — simple, evitable con proxies
2. **Wallet-based** (elegida) — más preciso
3. **Hybrid** — wallet primary, IP fallback

**Razonamiento:** En web3, el usuario puede cambiar de IP (VPN, mobile). La wallet es el identifier real. Wallet-based rate limiting es más efectivo contra abuse. Para endpoints sin wallet (como validate), IP es lo único disponible.

**Consecuencias:** Rate limiter debe resolver wallet desde signature o request context.