# Forge

CLI-утилита для управления контекстом ИИ-агентов в проектах разработки.

## Описание

**Forge** стандартизирует взаимодействие между разработчиками и ИИ через файловую систему. Утилита создаёт и поддерживает структуру `.agent/` — единое место для хранения контекста проекта (концепция, архитектура, задачи, дизайн), понятного ИИ-агентам (Claude Code, Cline, Aider и др.).

## Возможности

- 🚀 **`forge init`** — создание структуры `.agent/` с шаблонами документации
- 🔍 **`forge wrap`** — анализ существующего проекта и генерация описания через LLM
- 📋 **`forge task`** — управление задачами (Kanban: backlog/todo/in_progress/done)
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
# Инициализировать .agent/ в текущем проекте
forge init

# Проанализировать существующий проект
forge wrap --source ./src

# Добавить задачу
forge task add "Реализовать аутентификацию"

# Список задач
forge task list
```

## Структура `.agent/`

```
.agent/
├── README.md          # Точка входа для агентов
├── concept.md         # Концепция и цели проекта
├── tech_stack.md      # Технологический стек
├── rules.md           # Стандарты кодирования
├── struct.md          # Модульная структура
├── design/            # Дизайн-документы
├── tasks/             # Kanban-доска задач
│   ├── backlog/       # Отложенные
│   ├── todo/          # Готовы к работе
│   ├── in_progress/   # В работе
│   └── done/          # Завершены
└── workflows/         # Процессы (git, pm)
```

## Документация

Подробная документация находится в [`.agent/README.md`](.agent/README.md).

## Статус проекта

**Phase 0: Bootstrapping** — ✅ Завершена  
**Phase 1: MVP (forge init)** — 🚧 В разработке  

См. план в [`.agent/tasks/todo/`](.agent/tasks/todo/).

## Лицензия

MIT License

## Вклад

Приветствуются pull requests! См. [`.agent/workflows/git.md`](.agent/workflows/git.md) для процесса.
