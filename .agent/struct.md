# Модульная структура проекта

```
src/
├── main.rs           # Точка входа, clap CLI
├── cli/
│   ├── mod.rs        # Определение команд (clap)
│   ├── init.rs       # Логика `forge init`
│   ├── wrap.rs       # Логика `forge wrap`
│   ├── task.rs       # Логика `forge task`
│   ├── template.rs   # Логика `forge template`
│   └── ask.rs        # Логика `forge ask`
├── llm/
│   ├── mod.rs        # Трейт LlmClient
│   ├── openai.rs     # OpenAI-compatible клиент
│   └── prompts.rs    # Шаблоны промптов
├── scanner/
│   ├── mod.rs        # Сканер файловой структуры
│   └── heuristics.rs # Определение языка/фреймворка
├── templates/        # Встроенные Markdown-шаблоны
│   ├── readme.md
│   ├── concept.md
│   └── ...
└── config.rs         # Работа с forge.toml