# 09 - Solana Developer Certification - Resumen

> Resumen del programa de certificación WayLearn Solana Developer Certification.
> Conceptos clave para desarrollo en Solana con Rust + Anchor.

---

## Fundamentos Blockchain y Solana

### ¿Qué es Blockchain?

- **Inmutabilidad**: datos no se pueden alterar una vez registrados
- **Transparencia**: todo queda registrado y es verificable
- **Descentralización**: red de nodos, sin punto único de fallo

### Solana: Características Clave

| Característica | Descripción |
|----------------|-------------|
| **Mecanismo de consenso** | Proof-of-Stake + Proof-of-History (PoH) |
| **Velocidad** | Miles de TPS |
| **Comisiones** | Muy bajas ($0.001 - $0.01 típicamente) |
| **Lenguaje principal** | Rust |
| **Frameworks** | Anchor (default), Pinocchio (high-performance) |

### Comparación con otras blockchains

| Aspecto | Solana | Ethereum | Sui |
|---------|--------|----------|-----|
| Consenso | PoS + PoH | PoS | PoS |
| Lenguaje | Rust | Solidity | Move |
| TPS | Miles | Moderada | Alta |
| Comisiones | Muy bajas | Variables | Bajas |

---

## Rust para Solana

### Por qué Rust?

- **Performance de nivel sistema** (como C/C++)
- **Seguridad en memoria** sin garbage collector
- **Ownership y Borrowing** previenen bugs en tiempo de compilación

### Ownership y Borrowing (esencial para Solana)

```rust
// Ownership: cada valor tiene un "dueño"
let s1 = String::from("hello");
let s2 = s1; // s1 se "mueve" a s2
// println!("{}", s1); // ERROR - s1 ya no es válido

// Borrowing: prestar referencias
fn calcular_longitud(s: &String) -> usize {
    s.len()
} // s no se modifica, solo se lee

// Referencia mutable
fn modificar(s: &mut String) {
    s.push_str(", mundo");
}
```

**Reglas:**
- Referencias inmutables (`&T`): muchas simultáneas, sin modificaciones
- Referencias mutables (`&mut T`): solo UNA a la vez, sin referencias inmutables

---

## Modelo de Cuentas (Accounts)

### Concepto fundamental

En Solana, **el estado NO vive en el programa**. El estado vive en **cuentas (accounts)** separadas.

- El **programa** es como una función sin estado
- Las **accounts** son los datos persistentes
- El programa recibe cuentas como input y las modifica

### Estructura de una Account

```
Account {
    lamports: u64,      // SOL en la cuenta
    data: Vec<u8>,      // tus datos serializados
    owner: Pubkey,      // QUIÉN puede modificar esta cuenta
    executable: bool,   // si es un programa
    rent_epoch: u64
}
```

### Regla de oro del owner

> **Solo el owner puede modificar los datos de una cuenta**

Si una cuenta tiene `owner = ProgramID`, solo ese programa puede escribir en ella.

---

## Anchor Framework

### ¿Por qué Anchor?

- **Reduce boilerplate** (código repetitivo)
- **IDL automática** (genera cliente TypeScript)
- **Seguridad por default** (validaciones automáticas)
- **Estructura estándar** de proyecto

### Estructura de un programa Anchor

```
my_program/
├── programs/
│   └── my_program/
│       ├── src/
│       │   └── lib.rs        # Lógica del programa
│       └── Cargo.toml
├── tests/
│   └── my_program.ts         # Pruebas
├── Anchor.toml               # Configuración
└── migrations/               # Scripts de deploy
```

### macros Anchor esenciales

```rust
// Declara la dirección on-chain del programa
declare_id!("ProgramId11111111111111111111111111111");

// Define las instrucciones públicas
#[program]
pub mod my_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        ctx.accounts.my_account.data = data;
        Ok(())
    }
}

// Define las cuentas requeridas
#[derive(Accounts)]
pub struct Initialize {
    #[account(init, payer = user, space = 8 + 32)]
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Custom errors
#[error_code]
pub enum MyError {
    #[msg("Custom error message")]
    CustomError,
}
```

### Account Types en Anchor

| Type | Uso |
|------|-----|
| `Signer<'info>` | Verifica que la cuenta firmó la transacción |
| `SystemAccount<'info>` | Confirma ownership del System Program |
| `Account<'info, T>` | Cuenta serializada con validación automática |
| `Program<'info, T>` | Valida que es un programa ejecutable |
| `UncheckedAccount<'info>` | Raw account, validación manual |

### Account Constraints

```rust
#[account(
    init,                           // Crear cuenta
    payer = user,                   // Quién paga
    space = 8 + MyStruct::INIT_SPACE // Tamaño
)]
pub my_account: Account<'info, MyAccount>,

#[account(
    mut,                            // Cuenta mutable
    seeds = [b"vault", user.key().as_ref()],  // PDA seeds
    bump                            // Bump seed
)]
pub vault: SystemAccount<'info>,

#[account(
    has_one = authority @ Error::InvalidAuthority  // Ownership check
)]
pub account: Account<'info, MyAccount>,

#[account(
    mut,
    realloc = new_size,    // Redimensionar
    realloc::payer = payer,
    realloc::zero = true
)]
pub account: Account<'info, MyAccount>,
```

---

## PDAs (Program Derived Addresses)

### Concepto

Las **PDAs** son direcciones derivadas determinísticamente que **NO tienen clave privada**. Son идеаль para:

- Cuentas de estado owned por el programa
- Vaults/treasuries
- Perfiles de usuario
- Cualquier dato que el programa controle

### Cómo funcionan

```rust
// Derivar PDA:
// seeds = ["vault", user_pubkey]
// program_id = mi_programa
// → PDA = hash(seeds + program_id)

// El programa puede "firmar" por el PDA usando invoke_signed
let seeds = &[b"vault", user.key().as_ref(), &[ctx.bumps.vault]];
let signer = &[&seeds[..]];
invoke_signed(
    instruction,
    account_infos,
    signer,
)?;
```

### En Anchor

```rust
#[account(
    seeds = [b"vault", user.key().as_ref()],
    bump,
    seeds::program = system_program.key()  // opcional
)]
pub vault: Account<'info, Vault>,
```

---

## Despliegue

### Flujo de `anchor deploy`

1. **Compilar** → código Rust → BPF/SBF
2. **Publicar** → binario a la blockchain
3. **Registrar** → Program ID como cuenta ejecutable
4. **Listo** → cualquier transacción puede invocarlo

### Comandos básicos

```bash
# Desarrollo local
anchor test              # Ejecuta pruebas
anchor build             # Compila el programa
anchor deploy            # Despliega a la red configurada

# Con NO_DNA para agentes
NO_DNA=1 anchor test
NO_DNA=1 anchor build

# Desplegar a devnet
solana program deploy \
  --keypair ~/.config/solana/id.json \
  --url devnet \
  target/deploy/my_program.so
```

---

## Costos y Comisiones

### Tipos de costo

| Tipo | Descripción |
|------|-------------|
| **Fee de transacción** | Costo de procesamiento (~$0.0005) |
| **Costo de cuenta** | SOL para crear cuenta con `space` |

### Rent-exemption

Las cuentas deben mantener un balance mínimo para ser "rent-exempt" y no perder fondos progresivamente. El tamaño determina este mínimo.

### Calcular space

```rust
const DISCRIMINATOR: usize = 8;  // Anchor agrega 8 bytes

#[account]
pub struct MyAccount {
    pub data: u64,        // 8 bytes
    pub authority: Pubkey, // 32 bytes
}

// INIT_SPACE = DISCRIMINATOR + serialized size
```

---

## Security Checklist

1. **Validar cuentas recibidas**
   - Usar constraints: `has_one`, `seeds`, `bump`, `owner`

2. **Requerir Signer cuando corresponda**
   - Acciones que necesitan autorización del usuario

3. **Usar PDAs para estado controlado por programa**
   - Evita dependencias de claves privadas

4. **Validar ownership**
   - Solo el owner puede modificar sus datos

5. **Controlar authority**
   - Si existe campo `authority`, validar antes de permitir cambios

6. **Definir space correctamente**
   - space se decide al crear, no crece dinámicamente

7. **No confiar en datos del cliente**
   - Todo dato crítico debe validarse on-chain

---

## Ecosistema y Recursos

### Herramientas esenciales

- **Solana CLI**: `solana.com/docs/intro/installation`
- **Anchor**: `anchor-lang.com`
- **Solana Explorer**: `explorer.solana.com`

### Wallets para desarrollo

- **Phantom** (más popular)
- **Solflare**
- **Backpack**
- **CLI de Solana** (para desarrollo)

### Exploración del ecosistema

- `solanacompass.com` - Dashboard de red
- `solana.com/ecosystem` - Proyectos
- `helius.dev` - RPC y herramientas

---

## Requisitos para Proyecto de Certificación

- Repositorio público en GitHub
- Proyecto desarrollado en Solana (Rust + Anchor)
- **CRUD + PDA** implementado
- Documentación clara (README o comentarios en código)

### Verticales sugeridas

- Tokenización de activos
- Stablecoins
- DePIN (infraestructura descentralizada)
- Gaming
- Pagos
- DeFi
- NFTs
- Solana Actions