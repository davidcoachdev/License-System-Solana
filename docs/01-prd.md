# 01 - Product Requirements Document

## Producto

**Nombre:** License System on Solana  
**Tipo:** Sistema SaaS de gestión de licencias  
**Stack:** Solana (Anchor) + Rust + Backend + TUI/CLI + Frontend  
**Alcance:** Emitir, validar, extender y revocar licencias on-chain con validación offline

---

## Users

| Rol | Descripción | Acciones |
|-----|------------|---------|
| **End User** | Compra y usa licencias | Comprar, ver estado, validar software |
| **Admin** | Opera el sistema | Emitir, extender, revocar licencias |
| **Developer** | Integra vía API | Validar licencias, integrar en software |

---

## Core Features

### Licencias

- **Emitir licencia:** Admin genera licencia asociada a wallet + producto + expiración
- **Extender licencia:** Admin extiende `expires_at` (solo si no está revocada)
- **Revocar licencia:** Admin marca `is_revoked = true` (irreversible)
- **Validar licencia:** Software cliente verifica firma localmente (offline-first)

### Wallet Integration

- Frontend conecta wallet (Phantom, Solflare, etc.)
- TUI/CLI soporta múltiples wallets configuradas
- Cada rol usa wallet distinta (user / admin / system)

### Validación Offline

- Licencia se firma cryptográficamente con mensaje + timestamp
- Software cliente verifica sin necesidad de internet
- Validación online opcional como fallback

---

## Non-Functional Requirements

| Requisito | Target |
|----------|--------|
| Latencia de validación offline | < 50ms |
| Latencia de validación online | < 500ms |
| Disponibilidad on-chain | 99.9% (Solana) |
| Consistencia eventual (DB) | < 5s |
| Throughput de emisión | 100 tx/s |

---

## Out of Scope (Fase 1)

- Billing / pagos (Stripe, etc.)
- Multi-chain
- NFT-based licensing
- SSO / email auth
- Métricas de uso (analytics)
- Panel de usuario web

---

## Success Metrics

- Licencias emitidas / día
- Validaciones exitosas / día
- Tasa de éxito de validación offline
- Uptime del sistema

---

## Roadmap

```
Fase 1 (Bootcamp)
├── Anchor program (issue/extend/revoke)
├── CLI básica
└── Deploy devnet

Fase 2
├── SDK Rust completo
├── TUI interactiva
└── Wallet manager

Fase 3
├── Backend API
├── Indexer + PostgreSQL
└── Webhooks

Fase 4
├── Frontend web
└── Integración de pagos
```

---

## Definitions

| Término | Definición |
|---------|-----------|
| **Licencia** | Registro on-chain que otorga derecho de uso de un producto |
| **Producto** | Software o servicio asociado a una licencia |
| **Owner** | Wallet Solana dueña de la licencia |
| **Admin** | Entidad con autoridad para emitir/extender/revocar |
| **Offline validation** | Verificación de licencia sin conexión a internet |

---

## Constraints

1. Smart contract es inmutable post-deploy (sin upgrade authority en Fase 1)
2. Licencias son intransferibles (owner no cambia)
3. Revocación es irreversible
4. No hay refunds automático
5. Máximo de expiración: 10 años desde emisión