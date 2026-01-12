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

> 🚀 **Fast and flexible project generator** - Create projects from predefined templates with an intuitive TUI interface

Axer is a CLI tool written in Rust that accelerates the startup of new projects through predefined templates. Forget about repetitive configurations and start coding in seconds.

## ✨ Features

- **⚡ Fast and efficient** - Written in Rust for optimal performance
- **🎨 Interactive TUI interface** - Smooth and visual navigation to select templates (inquire)
- **📦 TOML-based templates** - Simple and flexible configuration
- **🔄 Variable system** - Dynamic replacement using Handlebars
- **🌍 Multi-language support** - Extensible architecture for different ecosystems (NodeJS, Rust, Python, etc.)
- **🔧 User configuration** - Templates stored in system-specific directories
- **🎯 Non-opinionated** - Templates establish a starting point, not imposed architectures

## 🎯 Why Axer?

Axer was born from the need to eliminate friction when starting projects. Instead of copying folders, configuring files, and remembering the "correct" structure, simply run Axer and you'll have a solid base project in seconds.

Templates are **opinionated but not restrictive** - they provide conventions and a clear initial structure, but give you the freedom to modify according to your needs.

## 📦 Installation

### From crates.io (Recommended)

```bash
cargo install axer
```

This will download, compile, and install Axer on your system. Works on Linux, macOS, and Windows.

### Requirements

- [Rust 1.70+](https://www.rust-lang.org/tools/install)
- Cargo (comes with Rust)

### From source

```bash
git clone https://github.com/Arekkazu/axer.git
cd axer
cargo build --release
```

The compiled binary will be in `target/release/axer`

### Install globally (optional)

```bash
cargo install --path .
```

This will install `axer` on your system to run it from anywhere.

## 🚀 Usage

### TUI Mode (Terminal Interface)

Run Axer without arguments to open the interactive interface:

```bash
# If installed globally
axer

# Or from the project directory
cargo run
```

Navigate with the keyboard arrows, select your template, answer the configuration questions and you're done!

### CLI Mode (Coming Soon)

Full CLI support is in active development. The `clap` dependency is already integrated and will be implemented soon to allow creating projects directly from the command line:

```bash
# Example of future usage
axer new my-project --template nest-api
axer list # List available templates
```

## 🗂️ Available Templates

Currently Axer includes:

- 🌐 **NodeJS**: NestJS API

### Templates on Roadmap

The following templates are planned:

- 🌐 **Web**: React, Vue, Angular, Next.js, Express
- 🦀 **Rust**: CLI, Web API (Axum/Actix), Desktop (Tauri)
- 🐍 **Python**: FastAPI, Django, Flask
- 📱 **Mobile**: React Native, Flutter

> Contributions of new templates are welcome!

## 📝 Template Structure

### Template Locations

Templates are stored in system-specific directories. **The directory is automatically created** the first time you run Axer.

> 🚧 **Coming Soon**: Automatic download of official templates on first run. Currently, you need to manually copy templates from the repository to your local directory. In future versions, Axer will automatically download and install official templates when you first launch the application.

| Operating System | Location |
|------------------|----------|
| **Linux** | `~/.config/axer-cli/templates/` |
| **macOS** | `~/Library/Application Support/top.Arekkazu.axer-cli/templates/` |
| **Windows** | `C:\Users\<Username>\AppData\Roaming\Arekkazu\axer-cli\config\templates\` |

To add your own templates, create a new folder for each template inside the `templates/` directory (e.g., `templates/my-template/`).

### Template Configuration

Each template must have a `template.toml` file:

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

### template.toml Fields

- **metadata.name**: Descriptive name of the template
- **metadata.language**: Language/runtime (`nodejs`, `rust`, `python`, etc.)
- **variables**: Array of variables that will be replaced in files
  - **field**: Variable name (used in Handlebars as `{{field}}`)
  - **prompt**: Question shown to the user
  - **default**: Default value if the user doesn't enter anything

### Using Variables in Files

Use Handlebars syntax in any template file:

```json
{
  "name": "{{project_name}}",
  "author": "{{author}}",
  "description": "{{description}}"
}
```

## 🛠️ Development

### Build

```bash
cargo build
```

### Run in development mode

```bash
cargo run
```

### Run tests

```bash
cargo test
```

> 📝 **Note**: Project tests are under development. Unit and integration tests will be added soon to ensure code quality.

### Check the code

```bash
cargo check
cargo clippy
```

## 🧪 Project Status

**Axer is in active development.** I'm learning and improving my Rust skills, so:

- ⚠️ There may be **compilation warnings** (unused variables, dead code, etc.)
- 🔧 The code is in **constant refactoring** to follow best practices
- 📚 I'm implementing features while learning idiomatic Rust patterns
- 🐛 If you find bugs or improvements, issues and PRs are welcome!
- ⏱️ **Development pace**: This project is updated occasionally, not on a fixed schedule

This project is also a learning opportunity, so I appreciate any constructive feedback on the code.

> 💡 **Note**: Axer is maintained as a side project and receives updates when time allows. The `0.x.x` version indicates that the project is functional but still evolving.

## 🗺️ Roadmap

- [x] Basic TUI interface (inquire)
- [x] TOML template system
- [x] Variable replacement system (Handlebars)
- [x] Support for multiple package managers
- [x] Configuration directory management
- [ ] Full CLI support with clap
- [ ] Automatic download of official templates on first run
- [ ] Clean warnings and improve code quality
- [ ] Binaries precompiled for multiple platforms
- [ ] More predefined templates (React, Vue, Rust, Python)
- [ ] User custom templates (documentation)
- [ ] Plugin system
- [ ] Global preferences configuration
- [ ] Support for remote templates (GitHub, GitLab)

## 🤝 Contributing

Contributions are welcome! If you have ideas for new templates or improvements:

1. Fork the project
2. Create a branch for your feature (`git checkout -b feature/new-template`)
3. Commit your changes (`git commit -m 'Add template for X'`)
4. Push to the branch (`git push origin feature/new-template`)
5. Open a Pull Request

### Contributing Templates

To add a new template:

1. Run Axer once to auto-generate the templates directory
2. Inside `templates/`, create a new folder for your template (e.g., `my-react-app/`)
3. Add a `template.toml` file with the configuration
4. Include your project files using `{{variable}}` variables for dynamic content
5. Test the template locally with Axer
6. Submit a PR with documentation

## 📋 Main Dependencies

- **inquire** - Interactive TUI interface
- **clap** - CLI argument parsing (in implementation)
- **handlebars** - Template engine for variable replacement
- **fs_extra** - Advanced file system operations
- **colored** - Colorized terminal output
- **serde + toml** - TOML file parsing
- **directories** - System-specific directory paths

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

```
MIT License - Copyright (c) 2026 Arekkazu
```

---

<div align="center">

**Made with ❤️ and 🦀 Rust**

<img width="500" height="500" alt="Axer Logo" src="https://github.com/user-attachments/assets/9e5449f6-e5f8-4671-8274-4eedb3efd1b7" />

[![Crates.io](https://img.shields.io/crates/v/axer.svg)](https://crates.io/crates/axer)
[![Downloads](https://img.shields.io/crates/d/axer.svg)](https://crates.io/crates/axer)
[![Documentation](https://docs.rs/axer/badge.svg)](https://docs.rs/axer)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

[English](README.md) | [Español](docs/README_ES.md)

</div>