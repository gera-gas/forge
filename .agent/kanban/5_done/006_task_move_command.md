---
id: "006"
title: "Реализация forge task move"
type: "feature"
priority: "high"
design: "001"
blocked_by: "004"
tags: ["cli", "kanban"]
completed_at: "2026-04-22"
---

# Реализация forge task move

## Описание

Реализовать команду `forge task move` — перемещение задачи между стадиями kanban-доски.

## Что сделать

1. Реализовать логику команды `task move`:
   - Найти задачу по ID (вызвать `find_task_by_id()`)
   - Если не найдена — ошибка
   - Определить текущую стадию
   - Проверить правила переходов (см. ниже)
   - Если задача — папка, переместить всю папку
   - Если задача — файл, переместить файл
   - При переходе `design → todo`: предложить создать git-ветку

2. Правила переходов:
   - Вперёд: `sketch → design → todo → in_progress → done`
   - В `backlog` — из любой стадии
   - Из `backlog` → только в `sketch`
   - В `in_progress` — предупреждение если уже есть задача (не ошибка)
   - Запрет возврата назад (кроме backlog)

3. Формат вывода:
   ```
   Moved:  task(001): move sketch -> design
   From:   .agent/kanban/1_sketch/001/
   To:     .agent/kanban/2_design/001/
   Git:    task(001): move sketch -> design
   ```

4. При `design → todo` дополнительно:
   ```
   Branch: feature/001_kanban_cli
   Run:    git checkout -b feature/001_kanban_cli
   ```

## Зависимости

- 004 (модуль kanban + поиск задач)

## Оценка

~1-2 дня