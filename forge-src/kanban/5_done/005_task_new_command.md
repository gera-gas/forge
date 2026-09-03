---
id: "005"
title: "Реализация forge task new"
type: "feature"
priority: "high"
design: "001"
blocked_by: "004"
tags: ["cli", "kanban"]
completed_at: "2026-04-22"
---

# Реализация forge task new

## Описание

Реализовать команду `forge task new` — создание новой задачи в kanban-доске.

## Что сделать

1. Реализовать логику команды `task new`:
   - Определить следующий ID (вызвать `next_available_id()`)
   - Сгенерировать slug из названия (transliterate/sanitize)
   - Создать файл `<id>_<slug>.md` в папке стадии (по умолчанию `1_sketch`)
   - Заполнить frontmatter из аргументов
   - Вывести путь к созданному файлу

2. Slug-генератор (`src/kanban/slug.rs`):
   - Кирилица → транслит (например, "Задача" → "zadacha")
   - lowercase, пробелы → `_`, оставить только `[a-z0-9_]`
   - Обрезать до 50 символов
   - Удалить повторяющиеся `_`

3. Формат вывода:
   ```
   Created: .agent/kanban/1_sketch/003_my_task.md
   Git:     task(003): add sketch "My task"
   ```

## Зависимости

- 004 (модуль kanban + frontmatter)

## Оценка

~1 день