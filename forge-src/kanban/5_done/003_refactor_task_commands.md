---
id: "003"
title: "Рефакторинг TaskCommands под Kanban"
type: "feature"
priority: "high"
design: "001"
blocked_by: null
tags: ["cli", "kanban"]
completed_at: "2026-04-22"
---

# Рефакторинг TaskCommands под Kanban

## Описание

Обновить enum `TaskCommands` в `main.rs` — заменить старые команды `add/list/start/done` на новые: `new/move/list/show`.

## Что сделать

1. В `src/main.rs`: заменить `TaskCommands` enum:
   - `Add { description }` → `New { title, --stage, --type, --priority, --tags }`
   - `Start { id }` → `Move { id, stage }`
   - `Done { id }` → удалить (покрывается `move <id> done`)
   - `List { format }` → `List { --stage, --format, --type }`
   - Добавить `Show { id }`
2. Обновить match-блоки в `main()`
3. Добавить enum `Stage` и `TaskType` для аргументов

## Зависимости

- Нет (первая задача, можно начать сразу)

## Оценка

~0.5-1 день