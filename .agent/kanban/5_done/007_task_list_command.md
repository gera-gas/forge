---
id: "007"
title: "Реализация forge task list и forge task show"
type: "feature"
priority: "medium"
design: "001"
blocked_by: "004"
tags: ["cli", "kanban"]
completed_at: "2026-04-22"
---

# Реализация forge task list и forge task show

## Описание

Реализовать команды `forge task list` и `forge task show` для просмотра задач.

## Что сделать

### forge task list

1. Сканировать все стадии (или указанную через `--stage`)
2. Парсить frontmatter каждого файла
3. Фильтровать по `--type` (если указан)
4. Вывести в формате `table` (по умолчанию) или `json`

Формат table:
```
ID    TITLE                    STAGE        TYPE      PRIORITY
001   Kanban board CLI         design       feature   high
002   VSCode Extension         sketch       feature   medium
```

Формат json: массив объектов с полями id, title, stage, type, priority, blocked_by, path.

### forge task show

1. Найти задачу по ID
2. Вывести frontmatter + содержимое
3. Если задача — папка, показать содержимое файла `0_*.md`
4. Показать путь и текущую стадию

## Зависимости

- 004 (модуль kanban + frontmatter)

## Оценка

~1 день