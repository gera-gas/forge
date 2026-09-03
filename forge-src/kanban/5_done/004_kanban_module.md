---
id: "004"
title: "Модуль kanban: чтение структуры и парсинг frontmatter"
type: "feature"
priority: "high"
design: "001"
blocked_by: null
tags: ["cli", "kanban", "core"]
completed_at: "2026-04-22"
---

# Модуль kanban: чтение структуры и парсинг frontmatter

## Описание

Создать модуль `src/kanban/` с базовой функциональностью для работы с kanban-досками: чтение структуры папок, парсинг frontmatter, поиск задач по ID.

## Что сделать

1. Создать `src/kanban/mod.rs` с публичным интерфейсом:
   - `fn find_kanban_root()` — поиск `.agent/kanban/` от текущей директории вверх
   - `fn list_stages()` — вернуть список стадий
   - `fn scan_all_tasks()` — сканировать все стадии и вернуть список задач
   - `fn find_task_by_id(id: &str)` — найти задачу по ID во всех стадиях
   - `fn next_available_id()` — определить следующий свободный ID

2. Создать `src/kanban/frontmatter.rs`:
   - Структура `TaskFrontmatter { id, title, task_type, priority, blocked_by, tags }`
   - `fn parse_frontmatter(content: &str) -> Option<TaskFrontmatter>`
   - `fn serialize_frontmatter(fm: &TaskFrontmatter) -> String`

3. Создать `src/kanban/stage.rs`:
   - Enum `Stage { Backlog, Sketch, Design, Todo, InProgress, Done }`
   - `impl FromStr` и `impl Display` для Stage
   - `fn dir_name()` — вернуть имя папки (например, `1_sketch`)
   - `fn stage_from_dir(name: &str) -> Option<Stage>`

4. Создать `src/kanban/task.rs`:
   - Структура `Task { frontmatter, stage, path }` — полная информация о задаче

## Зависимости

- Нет (можно делать параллельно с 003)

## Оценка

~1-2 дня