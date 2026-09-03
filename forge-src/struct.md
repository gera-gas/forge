# Модульная структура проекта

## Текущая структура (Phase 1)

```
src/
├── main.rs           # Точка входа, clap CLI
├── cli/
│   ├── mod.rs        # Определение команд (clap)
│   └── init.rs       # Логика `forge init`
├── kanban/
│   ├── mod.rs        # Координация модулей kanban
│   ├── frontmatter.rs # Парсинг YAML frontmatter
│   ├── slug.rs       # Генерация slug из названия
│   ├── stage.rs      # Стадии kanban и правила переходов
│   └── task.rs       # Операции с задачами (CRUD)
├── templates/
│   ├── mod.rs        # Константы шаблонов (include_str!)
│   ├── forge_readme.md  # Шаблон forge-src/README.md
│   └── codegen.md    # Шаблон forge-src/codegen.md
└── tests/
    └── integration_test.rs
```

## Целевая структура (Phase 2)

```
src/
├── main.rs           # Точка входа, clap CLI
├── context/          # Управление спецификацией
│   ├── mod.rs
│   ├── init.rs       # Логика `forge init`
│   ├── kanban/       # Kanban CRUD
│   │   ├── frontmatter.rs
│   │   ├── slug.rs
│   │   ├── stage.rs
│   │   └── task.rs
│   └── wrap.rs       # Логика `forge wrap` (код → спецификация)
├── codegen/          # Генерация кода
│   ├── mod.rs
│   ├── rules.rs      # Чтение и парсинг codegen.md
│   └── generate.rs   # Логика `forge generate` (спецификация → код)
├── llm/              # LLM-интерфейс (общий для wrap и generate)
│   ├── mod.rs        # Трейт LlmProvider
│   ├── openai.rs     # OpenAI-compatible клиент
│   └── prompts.rs    # Шаблоны промптов
├── agent/            # ИИ-агент (Phase 2)
│   ├── mod.rs
│   ├── orchestrator.rs # Оркестратор (правила / LLM-классификатор)
│   └── roles/        # Роли агентов (Arch / Dev / Ops)
├── templates/        # Встроенные шаблоны
│   ├── mod.rs
│   ├── forge_readme.md
│   └── codegen.md
└── config.rs         # Работа с forge.toml
```

## Структура `forge-src/` (спецификация)

```
forge-src/
├── README.md          # Точка входа для агентов
├── concept.md         # Концепция и цели проекта
├── tech_stack.md      # Технологический стек
├── rules.md           # Стандарты кодирования (для человека)
├── codegen.md         # Правила генерации кода (для ИИ-агента)
├── struct.md          # Модульная структура (этот файл)
├── arch.md            # Архитектурные решения (ADR)
├── kanban/            # Kanban-доска
│   ├── 0_backlog/
│   ├── 1_sketch/
│   ├── 2_design/
│   ├── 3_todo/
│   ├── 4_in_progress/
│   └── 5_done/
└── workflows/         # Процессы
    ├── agent.md
    └── git.md
```

## Разделение ответственности

| Модуль | Ответственность | Не отвечает за |
|--------|-----------------|----------------|
| `context/` | Создание и поддержка `forge-src/` | Генерацию кода, LLM-вызовы |
| `codegen/` | Генерация кода из спецификации | Управление kanban, шаблоны |
| `llm/` | HTTP-вызовы к LLM API | Бизнес-логику, парсинг ответов |
| `agent/` | Оркестрация работы ИИ-агента | Прямые LLM-вызовы (делегирует в `llm/`) |
| `templates/` | Встроенные шаблоны файлов | Логику заполнения шаблонов |