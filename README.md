# 🏥 Nexus-H: Salud e Inclusión en la "Hora Dorada"

**Nexus-H** es un ecosistema **IoMT (Internet of Medical Things)** desarrollado sobre la blockchain de **Solana** para optimizar la respuesta médica en situaciones de emergencia.

## 🚀 El Problema
En accidentes críticos, cada segundo cuenta (la "Hora Dorada"). La falta de identificación, el desconocimiento de alergias o tipo de sangre, y las barreras de idioma (especialmente en comunidades indígenas) causan muertes evitables.

## 💡 La Solución
Utilizamos un **Smart Contract en Rust (Anchor)** para vincular un hash biométrico con datos médicos vitales de forma inmutable y segura.
* **Velocidad:** Confirmaciones en menos de 1 segundo gracias a Solana.
* **Inclusión:** Traducción de datos críticos para paramédicos sin importar el idioma del paciente.
* **Seguridad:** Identificación biométrica sin necesidad de documentos físicos.

## 🛠️ Stack Técnico
- **Blockchain:** Solana (Devnet/Localnet)
- **Framework:** Anchor v0.30+ (Rust)
- **Lógica de Negocio:** Gestión de estados de pacientes y metadatos médicos inmutables.

## 📁 Estructura del Proyecto
- `/programs/nexus_h`: Contrato inteligente en Rust (`lib.rs`).
- `Anchor.toml`: Configuración del despliegue y Program ID.
- `Cargo.toml`: Dependencias y optimización de compilación.

## 👤 Autor
- **Ruiz Lopez Luis Angel** - Software Multiplataforma.
- Proyecto para la **Solana Hackathon**.
