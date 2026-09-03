---
id: "009"
title: "Тесты для kanban-модуля (frontmatter, перемещение, slug)"
type: "feature"
priority: "medium"
design: "001"
blocked_by: "004"
tags: ["cli", "kanban", "testing"]
completed_at: "2026-04-22"
---

# Тесты для kanban-модуля

## Описание

Написать тесты для ключевой функциональности kanban-модуля: парсинг frontmatter, генерация slug, перемещение задач, определение стадий.

## Что сделать

1. **Юнит-тесты frontmatter** (`src/kanban/frontmatter.rs`):
   - Парсинг корректного frontmatter
   - Парсинг файла без frontmatter (None)
   - Сериализация frontmatter
   - Обработка пустых tags, null blocked_by

2. **Юнит-тесты slug** (`src/kanban/slug.rs`):
   - Кирилица → транслит
   - Спецсимволы → удаление
   - Пробелы → `_`
   - Обрезка до 50 символов
   - Повторяющиеся `_` → один

3. **Юнит-тесты Stage** (`src/kanban/stage.rs`):
   - FromStr для всех стадий
   - stage_from_dir для имён папок
   - Display

4. **Интеграционные тесты** (`tests/integration_test.rs`):
   - Создание задачи через `task new`
   - Перемещение задачи через `task move`
   - Листинг задач через `task list`
   - Поиск задачи по ID

## Зависимости

- 004 (модуль kanban)
- 005, 006, 007 (команды — для интеграционных тестов)

## Оценка

~1-2 дня