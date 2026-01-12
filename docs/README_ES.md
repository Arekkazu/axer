# Axer

```
     >>==================================================<<
     ||                                                  ||
     ||       /$$$$$$                                    || 
     ||      /$$__  $$                                   || 
     ||     | $$  \ $$ /$$   /$$  /$$$$$$   /$$$$$$      || 
     ||     | $$$$$$$$|  $$ /$$/ /$$__  $$ /$$__  $$     || 
     ||     | $$__  $$ \  $$$$/ | $$$$$$$$| $$  \__/     || 
     ||     | $$  | $$  >$$  $$ | $$_____/| $$           || 
     ||     | $$  | $$ /$$/\  $$|  $$$$$$$| $$           || 
     ||     |__/  |__/|__/  \__/ \_______/|__/           || 
     ||                                                  ||
     >>==================================================<<
```

> 🚀 **Generador de proyectos rápido y flexible** - Crea proyectos desde plantillas predefinidas con una interfaz TUI intuitiva

Axer es una herramienta CLI escrita en Rust que acelera el inicio de nuevos proyectos mediante plantillas predefinidas. Olvídate de configuraciones repetitivas y comienza a codificar en segundos.

## ✨ Características

- **⚡ Rápido y eficiente** - Escrito en Rust para un rendimiento óptimo
- **🎨 Interfaz TUI interactiva** - Navegación fluida y visual para seleccionar plantillas (inquire)
- **📦 Plantillas basadas en TOML** - Configuración simple y flexible
- **🔄 Sistema de variables** - Reemplazo dinámico usando Handlebars
- **🌍 Soporte multi-lenguaje** - Arquitectura extensible para diferentes ecosistemas (NodeJS, Rust, Python, etc.)
- **🔧 Configuración por usuario** - Templates en `~/.config/axer-cli/templates/`
- **🎯 Sin opiniones fuertes** - Las plantillas establecen un punto de partida, no imponen arquitecturas

## 🎯 ¿Por qué Axer?

Axer nace de la necesidad de eliminar la fricción al iniciar proyectos. En lugar de copiar carpetas, configurar archivos y recordar la estructura "correcta", simplemente ejecuta Axer y tendrás un proyecto base sólido en segundos.

Las plantillas son **opinionadas pero no restrictivas** - proporcionan convenciones y una estructura inicial clara, pero te dan la libertad de modificar según tus necesidades.

## 📦 Instalación

### Desde crates.io (Recomendado)

```bash
cargo install axer
```

Esto descargará, compilará e instalará Axer en tu sistema. Funciona en Linux, macOS y Windows.

### Requisitos

- [Rust 1.70+](https://www.rust-lang.org/tools/install)
- Cargo (viene con Rust)

### Desde el código fuente

```bash
git clone https://github.com/Arekkazu/axer.git
cd axer
cargo build --release
```

El binario compilado estará en `target/release/axer`

### Instalar globalmente (opcional)

```bash
cargo install --path .
```

Esto instalará `axer` en tu sistema para poder ejecutarlo desde cualquier lugar.

## 🚀 Uso

### Modo TUI (Interfaz Terminal)

Ejecuta Axer sin argumentos para abrir la interfaz interactiva:

```bash
# Si instalaste globalmente
axer

# O desde el directorio del proyecto
cargo run
```

Navega con las flechas del teclado, selecciona tu plantilla, responde las preguntas de configuración y ¡listo!

### Modo CLI (Próximamente)

El soporte CLI completo está en desarrollo activo. La dependencia `clap` ya está integrada y se implementará pronto para permitir crear proyectos directamente desde la línea de comandos:

```bash
# Ejemplo de uso futuro
axer new my-project --template nest-api
axer list # Listar templates disponibles
```

## 🗂️ Plantillas Disponibles

Actualmente Axer incluye:

- 🌐 **NodeJS**: NestJS API

### Plantillas en Roadmap

Las siguientes plantillas están planificadas:

- 🌐 **Web**: React, Vue, Angular, Next.js, Express
- 🦀 **Rust**: CLI, Web API (Axum/Actix), Desktop (Tauri)
- 🐍 **Python**: FastAPI, Django, Flask
- 📱 **Mobile**: React Native, Flutter

> ¡Las contribuciones de nuevas plantillas son bienvenidas!

## 📝 Estructura de Templates

### Ubicación de los Templates

Los templates se almacenan en directorios específicos según el sistema operativo. **El directorio se crea automáticamente** la primera vez que ejecutas Axer.

> 🚧 **Próximamente**: Descarga automática de templates oficiales en la primera ejecución. Actualmente, necesitas copiar manualmente los templates del repositorio a tu directorio local. En futuras versiones, Axer descargará e instalará automáticamente los templates oficiales cuando inicies la aplicación por primera vez.

| Sistema Operativo | Ubicación |
|-------------------|-----------|
| **Linux** | `~/.config/axer-cli/templates/` |
| **macOS** | `~/Library/Application Support/top.Arekkazu.axer-cli/templates/` |
| **Windows** | `C:\Users\<Usuario>\AppData\Roaming\Arekkazu\axer-cli\config\templates\` |

Para agregar tus propios templates, crea una nueva carpeta para cada template dentro del directorio `templates/` (ej: `templates/mi-template/`).

### Configuración del Template

Cada template debe tener un archivo `template.toml`:

```toml
[metadata]
name = "NestJs Api"
language = "nodejs"

[[variables]]
field = "author"
prompt = "Project's author?: "
default = "arekkazu"

[[variables]]
field = "description"
prompt = "Project Description: "
default = "Building an API using NestJs"
```

### Campos del template.toml

- **metadata.name**: Nombre descriptivo del template
- **metadata.language**: Lenguaje/runtime (`nodejs`, `rust`, `python`, etc.)
- **variables**: Array de variables que se reemplazarán en los archivos
  - **field**: Nombre de la variable (usado en Handlebars como `{{field}}`)
  - **prompt**: Pregunta mostrada al usuario
  - **default**: Valor por defecto si el usuario no ingresa nada

### Uso de variables en archivos

Usa sintaxis Handlebars en cualquier archivo del template:

```json
{
  "name": "{{project_name}}",
  "author": "{{author}}",
  "description": "{{description}}"
}
```

## 🛠️ Desarrollo

### Compilar

```bash
cargo build
```

### Ejecutar en modo desarrollo

```bash
cargo run
```

### Ejecutar tests

```bash
cargo test
```

> 📝 **Nota**: Los tests del proyecto están en desarrollo. Próximamente se agregarán tests unitarios e integración para garantizar la calidad del código.

### Verificar el código

```bash
cargo check
cargo clippy
```

## 🧪 Estado del Proyecto

**Axer está en desarrollo activo.** Estoy aprendiendo y mejorando mis habilidades en Rust, por lo que:

- ⚠️ Pueden existir **warnings de compilación** (variables no usadas, código muerto, etc.)
- 🔧 El código está en **constante refactorización** para seguir mejores prácticas
- 📚 Estoy implementando características mientras aprendo patrones idiomáticos de Rust
- 🐛 Si encuentras bugs o mejoras, ¡son bienvenidos los issues y PRs!
- ⏱️ **Ritmo de desarrollo**: Este proyecto se actualiza ocasionalmente, no tiene un calendario fijo

Este proyecto es también una oportunidad de aprendizaje, así que agradezco cualquier feedback constructivo sobre el código.

> 💡 **Nota**: Axer se mantiene como un proyecto personal y recibe actualizaciones cuando el tiempo lo permite. La versión `0.x.x` indica que el proyecto es funcional pero aún está evolucionando.

## 🗺️ Roadmap

- [x] Interfaz TUI básica (inquire)
- [x] Sistema de plantillas con TOML
- [x] Sistema de reemplazo de variables (Handlebars)
- [x] Soporte para múltiples package managers
- [x] Gestión de directorio de configuración
- [ ] Soporte CLI completo con clap
- [ ] Descarga automática de templates oficiales en primera ejecución
- [ ] Limpiar warnings y mejorar calidad del código
- [ ] Binarios precompilados para múltiples plataformas
- [ ] Más plantillas predefinidas (React, Vue, Rust, Python)
- [ ] Plantillas personalizadas de usuario (documentación)
- [ ] Sistema de plugins
- [ ] Configuración global de preferencias
- [ ] Soporte para plantillas remotas (GitHub, GitLab)

## 🤝 Contribuir

¡Las contribuciones son bienvenidas! Si tienes ideas para nuevas plantillas o mejoras:

1. Fork el proyecto
2. Crea una rama para tu feature (`git checkout -b feature/nueva-plantilla`)
3. Commit tus cambios (`git commit -m 'Agrega plantilla para X'`)
4. Push a la rama (`git push origin feature/nueva-plantilla`)
5. Abre un Pull Request

### Contribuir con Templates

Para agregar un nuevo template:

1. Ejecuta Axer una vez para auto-generar el directorio de templates
2. Dentro de `templates/`, crea una nueva carpeta para tu template (ej: `mi-app-react/`)
3. Agrega un archivo `template.toml` con la configuración
4. Incluye los archivos de tu proyecto usando variables `{{variable}}` para contenido dinámico
5. Prueba el template localmente con Axer
6. Envía un PR con la documentación

## 📋 Dependencias Principales

- **inquire** - Interfaz TUI interactiva
- **clap** - Parsing de argumentos CLI (en implementación)
- **handlebars** - Motor de templates para reemplazo de variables
- **fs_extra** - Operaciones avanzadas de sistema de archivos
- **colored** - Output colorizado en terminal
- **serde + toml** - Parsing de archivos TOML

## 📄 Licencia

Este proyecto está bajo la licencia MIT. Ver el archivo [LICENSE](LICENSE) para más detalles.

```
MIT License - Copyright (c) 2026 Arekkazu
```

---

<div align="center">

**Hecho con ❤️ y 🦀 Rust**

<img width="500" height="500" alt="Axer Logo" src="https://github.com/user-attachments/assets/9e5449f6-e5f8-4671-8274-4eedb3efd1b7" />

[![Crates.io](https://img.shields.io/crates/v/axer.svg)](https://crates.io/crates/axer)
[![Downloads](https://img.shields.io/crates/d/axer.svg)](https://crates.io/crates/axer)
[![Documentation](https://docs.rs/axer/badge.svg)](https://docs.rs/axer)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

[English](../README.md) | [Español](README_ES.md)

</div>