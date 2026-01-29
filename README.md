# Secure Hotel Check-in System (Rust implementation) 🏨🔐

Implementación de un sistema de Identidad Autosoberana (SSI) desarrollada en **Rust** para el **Extrimian SSI HackAlong 2024** (Track: Travel & Hospitality).

## 🚀 Objetivo
Demostrar un flujo de **Check-in Privado** donde un hotel puede verificar la identidad de un huésped mediante Credenciales Verificables (VCs) sin necesidad de almacenar copias físicas de pasaportes.

## 🛠️ Stack Tecnológico
* **Lenguaje:** Rust 🦀
* **Estándares:** W3C Verifiable Credentials, DID (Decentralized Identifiers).
* **Arquitectura:**
    * **src/**: Lógica de emisión y verificación.
    * **json_model/**: Esquemas de datos para VCs y VPs.

## ⚙️ Cómo ejecutar (How to run)
Asegúrate de tener Rust instalado.

```bash
# Clonar el repositorio
git clone [https://github.com/limbertlino/extrimian-ssi-hackalong-project](https://github.com/limbertlino/extrimian-ssi-hackalong-project)

# Instalar dependencias y compilar
cargo build

# Ejecutar el sistema
cargo run
```

