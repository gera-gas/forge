# CLI как ядро, VSCode Extension как GUI-редактор

**Приоритет:** MVP  
**Родитель:** [0_base_idea.md](0_base_idea.md)

---

## Цель

Определить архитектурное разделение между CLI-ядром `forge` и VSCode Extension. CLI — источник истины и бизнес-логики, Extension — графический слой, делегирующий в CLI.

---

## Принципы разделения

### 1. CLI — источник истины

`forge` CLI содержит **всю бизнес-логику**:
- Создание и парсинг `forge-src/` структуры
- Kanban CRUD (создание, перемещение, список, детали задач)
- Валидация правил переходов между стадиями
- Git-интеграция (генерация команд, ветки)
- Шаблоны файлов (`src/templates/`)
- В будущем: LLM-вызовы (`wrap`, `generate`, `ask`)

### 2. Extension — тонкий GUI-слой

VSCode Extension **не дублирует бизнес-логику**. Он:
- Вызывает `forge` CLI через `child_process` / `vscode.tasks`
- Парсит JSON-вывод CLI (`forge task list --format json`)
- Отображает данные в webview (Kanban-доска, формы)
- Передаёт действия обратно в CLI (`forge task move 001 design`)

### 3. JSON как контракт

CLI и Extension общаются через **JSON-вывод**:
- `forge task list --format json` → массив задач
- `forge task show 001 --format json` → детали задачи (план)
- `forge task new "Title" --format json` → созданная задача (план)

Это позволяет:
- Extension не зависеть от внутренних структур Rust
- Другим GUI (Tauri, web) использовать тот же контракт
- Тестировать CLI независимо от GUI

---

## Схема взаимодействия

```
┌─────────────────────────────────────────────┐
│  VSCode Extension (TypeScript)              │
│                                             │
│  ┌─────────────┐    ┌──────────────────┐    │
│  │  Webview    │    │  Command Handler │    │
│  │  (React)    │───▶│                  │    │
│  │  Kanban UI  │    │  forge task list │    │
│  │  Forms      │    │  forge task move │    │
│  └─────────────┘    │  forge task new  │    │
│                     └────────┬─────────┘    │
└──────────────────────────────┼──────────────┘
                               │ child_process
                               ▼
┌─────────────────────────────────────────────┐
│  forge CLI (Rust)                           │
│                                             │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  │
│  │ context/ │  │ codegen/ │  │   llm/   │  │
│  │ init     │  │ generate │  │  openai  │  │
│  │ kanban   │  │  rules   │  │  prompts │  │
│  │ wrap     │  │          │  │          │  │
│  └──────────┘  └──────────┘  └──────────┘  │
│                                             │
│  Вывод: text (человек) или JSON (GUI)       │
└─────────────────────────────────────────────┘
```

---

## Что Extension НЕ делает

| Действие | Кто делает | Почему |
|----------|-----------|--------|
| Парсинг frontmatter | CLI | Единая логика парсинга |
| Валидация переходов kanban | CLI | Правила в `stage.rs` |
| Генерация slug | CLI | Логика в `slug.rs` |
| Определение следующего ID | CLI | Сканирование файлов |
| Git-команды | CLI | Генерация команд в CLI |
| LLM-вызовы | CLI | API-ключи, промпты в CLI |

## Что Extension делает

| Действие | Как |
|----------|-----|
| Отображение Kanban-доски | Webview + React, данные из `forge task list --format json` |
| Drag & drop | Webview UI, вызывает `forge task move <id> <stage>` |
| Создание задачи | Форма в webview, вызывает `forge task new "Title" ...` |
| Просмотр деталей | `forge task show <id> --format json` → отображение |
| Открытие файла задачи | `vscode.window.showTextDocument` на пути из JSON |
| Git-статус | `vscode.extensions.getExtension('vscode.git')` или `git` CLI |

---

## Эволюция

### Phase 1 (MVP) — CLI + Extension

- CLI: `init`, `task new/move/list/show`, `wrap` (заглушка)
- Extension: Kanban-доска, drag&drop, формы создания/редактирования
- Контракт: JSON-вывод через `--format json`

### Phase 2 — Расширение CLI

- CLI: `wrap` (реализация), `generate`, `agent`, `ask`
- Extension: интеграция LLM-фич (кнопка "Generate code", "Ask AI")
- Extension: Git-панель (ветки, коммиты, PR)

### Phase 3 — Выделение forge-core

- `forge-core` — библиотека (парсинг, kanban, LLM)
- `forge` — CLI-бинарник (зависит от `forge-core`)
- `forge-vscode` — Extension (может использовать `forge-core` через WASM или CLI)
- Решение о WASM vs CLI принимается в Phase 3 на основе замеров производительности

---

## Решения по открытым вопросам

| Вопрос | Решение | Обоснование |
|--------|---------|-------------|
| Extension дублирует логику? | **Нет**, делегирует в CLI | DRY, единый источник истины |
| Формат обмена | **JSON** (`--format json`) | Машиночитаемый, язык-агностичный |
| Extension на React или vanilla? | **React** (как в скетче 002) | Богатый UI, экосистема, webview |
| WASM или child_process? | **Phase 1: child_process**, Phase 3: рассмотреть WASM | Простота сейчас, оптимизация потом |
| Расширение CLI для JSON? | **Да**, добавить `--format json` к `show` и `new` | Полный контракт CLI ↔ GUI |