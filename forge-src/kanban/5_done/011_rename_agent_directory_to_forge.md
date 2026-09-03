---
id: '011'
title: Rename .agent directory to forge
type: chore
priority: high
blocked_by: null
tags: []
completed_at: '2026-08-04'
---

# Rename .agent directory to forge

## Что сделано

Переименование всех упоминаний `.agent/` → `forge/` в исходном коде и документации.

### Обновлённые файлы

1. **`README.md`** (корневой) — описание, структура, ссылки
2. **`src/templates/mod.rs`** — комментарии к константам-шаблонам
3. **`src/main.rs`** — комментарии к командам Init и Wrap
4. **`tests/integration_test.rs`** — пути проверок `.agent/` → `forge/`
5. **`src/templates/forge_readme.md`** — структура директории в шаблоне
6. **`src/templates/codegen.md`** — текст правил генерации кода
7. **`KODA.md`** — файл-указатель

### Уже было сделано ранее

- `src/cli/init.rs` — пути и сообщения
- `src/kanban/mod.rs` — `KANBAN_DIR = "forge/kanban"`
- `src/kanban/stage.rs` — комментарии
- `Cargo.toml` — `readme = "forge/README.md"`
- Файлы-указатели: `CLAUDE.md`, `AGENTS.md`, `.clinerules`, `.cursorrules`, `KOD`

### Проверка

- `cargo build` — успешно (только warnings о неиспользуемом коде)
- `cargo test` — 29 unit + 4 интеграционных теста, все passed
