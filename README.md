# 🧬 Nexus-H: Bio-Inclusión Médica en la "Hora Dorada"

![Solana Hackathon](https://img.shields.io/badge/Solana-Hackathon--Entry-9945FF?style=for-the-badge&logo=solana)
![Status](https://img.shields.io/badge/Status-Prototype-14F195?style=for-the-badge)

**Nexus-H** es un ecosistema **IoMT (Internet of Medical Things)** desarrollado sobre la blockchain de **Solana** para optimizar la respuesta médica en situaciones de emergencia crítica.

---

## 🚀 El Problema: El "Silencio" en la Hora Dorada
En accidentes graves, cada segundo cuenta. La falta de identificación, el desconocimiento de alergias, tipo de sangre o antecedentes médicos, sumado a las **barreras de idioma** (especialmente en comunidades indígenas o personas con discapacidad), causan muertes evitables. 

Mi visión como desarrollador y aspirante a **Neurocirujano** es eliminar ese silencio informativo mediante tecnología inmutable.

## 💡 La Solución
Utilizamos un **Smart Contract en Rust (Anchor)** para vincular un hash biométrico único con datos médicos vitales de forma segura.
* **Velocidad Extrema:** Confirmaciones en menos de 1 segundo gracias a la red de Solana.
* **Inclusión Radical:** Traducción automática de datos críticos para paramédicos y soporte para interfaces de accesibilidad.
* **Identidad Soberana:** El cuerpo del paciente es su propia llave; no se requieren documentos físicos en el lugar del accidente.

---

## 🛠️ Stack Técnico
- **Blockchain:** Solana (Devnet/Localnet)
- **Framework:** Anchor v0.30+ (Rust)
- **Frontend:** HTML5 / CSS3 (Glassmorphism UI)
- **Lógica de Negocio:** Gestión de estados de pacientes y metadatos médicos inmutables (PDA - Program Derived Addresses).

## 📁 Estructura del Proyecto
- `/programs/nexus_h`: Contrato inteligente en Rust (`lib.rs`).
- `Anchor.toml`: Configuración del despliegue y Program ID.
- `Cargo.toml`: Dependencias y optimización de compilación.
- `/app`: Interfaz web de visualización para personal de emergencias.

---

## 🏥 Impacto Social y Visión Humana
Este proyecto no es solo código; es una herramienta de justicia social. 
1.  **Comunidades Indígenas:** Rompiendo la brecha del idioma en urgencias.
2.  **Personas con Discapacidad:** Garantizando que sus necesidades específicas sean visibles al instante.
3.  **Futuro Quirúrgico:** Diseñado para ser la interfaz que conecte la ambulancia directamente con el quirófano de neurocirugía.

---

## 👤 Autor
**Ruiz Lopez Luis Angel**
* **Carrera:** Desarrollo de Software Multiplataforma.
* **Meta:** Fusión de Ingeniería de Software y Medicina (Especialidad en Neurocirugía).

---
🔗 **Demo en vivo:** [Explora el Ecosistema Nexus-H](https://25291058-dev.github.io/Nexus-H/)
