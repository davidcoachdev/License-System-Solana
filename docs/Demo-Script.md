# 🎬 Demo Script — Solana License System

Duración objetivo: 5–8 minutos
Formato: pantalla + voz (puede ser con cámara opcional)

---

# 🎯 0. Hook (0:00 - 0:20)

🎥 Mostrar: pantalla con el repo abierto

🗣️ Decir:

“Hoy te voy a mostrar cómo pasé de hacer un smart contract simple
a construir un sistema real de licencias usando Solana, Rust y una arquitectura completa.”

---

# 🧩 1. El Problema (0:20 - 0:50)

🎥 Mostrar: README o slide inicial

🗣️ Decir:

“La mayoría de proyectos en blockchain son demos.
Pero en el mundo real necesitás cosas como:

* validación rápida
* escalabilidad
* tooling de administración

Entonces me pregunté:
¿cómo diseñaría esto como un producto real?”

---

# 💡 2. La Solución (0:50 - 1:30)

🎥 Mostrar: diagrama de arquitectura

🗣️ Decir:

“Diseñé una arquitectura híbrida:

* Solana como fuente de verdad
* backend como orquestador
* SDK en Rust como capa central
* y una TUI para administración

Esto me permite tener lo mejor de Web2 y Web3.”

---

# 🏗️ 3. Arquitectura (1:30 - 2:30)

🎥 Mostrar: `docs/architecture.md`

🗣️ Decir:

“Acá se ve cómo todo se conecta:

* el frontend maneja wallets de usuario
* el backend ejecuta lógica
* el SDK abstrae Solana
* y la TUI permite operar el sistema

Lo importante es que cada pieza tiene una responsabilidad clara.”

---

# 🧱 4. Smart Contract (2:30 - 3:30)

🎥 Mostrar: código de Anchor

🗣️ Decir:

“El contrato es intencionalmente simple:

solo guarda:

* owner
* product_id
* expires_at
* is_revoked

No metí lógica de negocio acá
porque los contratos son inmutables.”

---

# 🔥 5. SDK en Rust (3:30 - 4:15)

🎥 Mostrar: `sdk-rust`

🗣️ Decir:

“En lugar de duplicar lógica, creé un SDK en Rust.

Esto me permite:

* reutilizar código
* mantener consistencia
* usarlo tanto en CLI como en backend”

---

# 🖥️ 6. TUI / CLI (4:15 - 5:15)

🎥 Mostrar: terminal corriendo la TUI

🗣️ Decir:

“La herramienta principal es esta TUI.

Desde acá puedo:

* emitir licencias
* extenderlas
* revocarlas
* cambiar wallets

Esto es lo que usaría un operador real del sistema.”

👉 Mostrar comandos reales:

```
issue-license
extend-license
wallet switch
```

---

# ⚡ 7. Event-Driven (5:15 - 6:00)

🎥 Mostrar: diagrama de eventos

🗣️ Decir:

“El sistema también es event-driven:

* un indexador escucha Solana
* actualiza PostgreSQL
* y dispara webhooks

Esto permite integraciones y escalabilidad.”

---

# 🔐 8. Wallets y Seguridad (6:00 - 6:40)

🎥 Mostrar: diagrama de wallets

🗣️ Decir:

“Separé roles claramente:

* usuario → wallet en frontend
* admin → wallet en TUI
* backend → wallet del sistema

Esto evita inconsistencias y mejora seguridad.”

---

# 💥 9. Cierre (6:40 - 7:10)

🎥 Volver al repo

🗣️ Decir:

“Este proyecto no es solo un smart contract,
es un sistema completo listo para evolucionar a un SaaS real.”

---

# 🙌 10. Call to Action (7:10 - 7:30)

🗣️ Decir:

“Si te interesa arquitectura, Rust o Web3 aplicado a productos reales,
este proyecto está en mi GitHub.”

---

# 🎥 Tips de Grabación

* Resolución: 1080p mínimo
* Fuente grande en terminal
* Cursor visible
* Voz clara (sin música fuerte)
* Ideal: OBS + micrófono decente

---

# 🧠 Tips PRO (esto marca la diferencia)

* No leas → explicá
* Contá decisiones (“por qué lo hice así”)
* Mostrá seguridad técnica
* No intentes cubrir TODO → foco en lo importante

---

# 🚀 Bonus (si querés destacar más)

* Agregá demo real (emitir licencia)
* Mostrá error handling
* Mostrá cambio de wallet en vivo

---
