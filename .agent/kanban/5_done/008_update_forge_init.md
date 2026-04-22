---
id: "008"
title: "Обновить forge init для создания структуры kanban"
type: "feature"
priority: "medium"
design: "001"
blocked_by: null
tags: ["cli", "kanban"]
completed_at: "2026-04-22"
---

# Обновить forge init для создания структуры kanban

## Описание

Команда `forge init` должна создавать структуру `.agent/kanban/` со всеми стадиями.

## Что сделать

1. В `src/cli/init.rs`: добавить создание папок kanban:
   - `0_backlog/`, `1_sketch/`, `2_design/`, `3_todo/`, `4_in_progress/`, `5_done/`
2. Добавить `.gitkeep` в каждую пустую папку (чтобы git отслеживал)
3. Не удалять существующие папки/файлы при повторном `forge init`
4. Вывести список созданных папок

## Зависимости

- Нет (можно делать параллельно)

## Оценка

~0.5 дня