# Технологический стек

## Окружение разработки

- **Host OS**: Windows 10
- **Target OS**: Cross-platform (Windows, Linux, macOS)
- **Build OS**: Windows (dev), Linux (CI — GitHub Actions)
- **Shell**: cmd.exe / PowerShell
- **Line endings**: CRLF (Windows), `.gitattributes` для нормализации

## Языки и фреймворки

- **Язык**: Rust 1.78+
- **CLI Framework**: Clap (derive macros)
- **Config**: TOML (forge.toml)
- **LLM Interface**: OpenAI API compatible endpoints
- **HTTP Client**: reqwest
- **File system operations**: std::fs, walkdir
- **TOML parsing**: toml
- **Markdown parsing**: pulldown-cmark