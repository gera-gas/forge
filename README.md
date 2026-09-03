# Forge

CLI-утилита для управления спецификациями ИИ-агентов в проектах разработки.

## Описание

**Forge** стандартизирует взаимодействие между разработчиками и ИИ через файловую систему. Утилита создаёт и поддерживает структуру `forge-src/` — единое место для хранения спецификации проекта (концепция, архитектура, задачи, дизайн), понятного ИИ-агентам (Claude Code, Cline, Aider и др.).

## Возможности

- 🚀 **`forge init`** — создание структуры `forge-src/` с шаблонами документации
- 🔍 **`forge wrap`** — анализ существующего проекта и генерация описания через LLM
- 📋 **`forge task`** — управление задачами (Kanban: backlog/sketch/design/todo/in_progress/done)
- 🤖 **`forge ask`** — NLP-интерфейс для команд на естественном языке
- 🔧 **`forge template`** — генерация файлов сборки (CMake, Bazel) через LLM

## Установка

```bash
cargo install forge-agent
```

Или из исходников:

```bash
git clone https://github.com/yourusername/forge.git
cd forge
cargo build --release
```

## Быстрый старт

```bash
# Инициализировать forge-src/ в текущем проекте
forge init

# Проанализировать существующий проект
forge wrap --source ./src

# Добавить задачу
forge task new "Реализовать аутентификацию"

# Список задач
forge task list
```

## Структура `forge-src/`

```
forge-src/
├── README.md          # Точка входа для агентов
├── concept.md         # Концепция и цели проекта
├── tech_stack.md      # Технологический стек
├── rules.md           # Стандарты кодирования (для человека)
├── codegen.md         # Правила генерации кода (для ИИ-агента)
├── struct.md          # Модульная структура
├── arch.md            # Архитектурные решения (ADR)
├── kanban/            # Kanban-доска задач
│   ├── 0_backlog/     # Отложенные
│   ├── 1_sketch/      # Идеи, наброски
│   ├── 2_design/      # Утверждённый дизайн
│   ├── 3_todo/        # Готовы к работе
│   ├── 4_in_progress/ # В работе
│   └── 5_done/        # Завершены
└── workflows/         # Процессы (agent, git)
```

## Документация

Подробная документация находится в [`forge-src/README.md`](forge-src/README.md).

## Статус проекта

**Phase 0: Bootstrapping** — ✅ Завершена  
**Phase 1: MVP (forge init)** — 🚧 В разработке  

См. план в [`forge-src/kanban/`](forge-src/kanban/).

## Лицензия

MIT License

## Вклад

Приветствуются pull requests! См. [`forge-src/workflows/git.md`](forge-src/workflows/git.md) для процесса.