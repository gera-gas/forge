---
id: "001"
title: "Kanban Board — CLI реализация"
type: "feature"
priority: "high"
blocked_by: null
tags: ["cli", "kanban", "core"]
---

# Kanban Board — CLI реализация

## Проблема

Утилита `forge` нуждается в системе управления задачами, которая покрывает весь жизненный цикл: от идеи (скетча) до завершения. Текущая реализация (`forge task add/list/start/done`) работает с плоской структурой `tasks/` и не поддерживает стадии дизайна.

## Решение

Kanban-доска с 6 стадиями, реализованная через файловую систему в `.agent/kanban/`. CLI-команды управляют перемещением задач между стадиями.

---

## Структура Kanban

```
.agent/kanban/
├── 0_backlog/      # Отложенные задачи
├── 1_sketch/       # Идеи, user stories, наброски
├── 2_design/       # Утверждённый дизайн, ТЗ
├── 3_todo/         # Технически проработанные задачи, готовые к коду
├── 4_in_progress/  # В работе (не более одной)
└── 5_done/         # Завершённые задачи
```

### Организация задач внутри стадии

- **Простая задача:** `001_simple_task.md` (один файл)
- **Сложная задача:** `001/` (папка с несколькими файлами)

**Правила нумерации файлов внутри папки:**
- `0_<name>.md` — точка входа (проблема, идея, user story)
- `1_<name>.md`, `2_<name>.md` — итерации/обсуждения
- Последний файл с суффиксом `_decision` или `_summary` — итоговое решение
- Файлы без номера — приложения (диаграммы, спецификации)

---

## Формат файла задачи

Минимальный YAML frontmatter:

```yaml
---
id: "001"
title: "Краткое название"
type: "feature"     # feature | fix | spike | chore
priority: "medium"  # low | medium | high | critical
blocked_by: null    # ID задачи-блокера или null
tags: []
---
# Содержание задачи
...
```

**Статус и стадия НЕ дублируются в frontmatter** — они определяются расположением файла в папке.

---

## CLI-команды

### `forge task new`

Создание новой задачи в указанной стадии.

```
forge task new "Название задачи" [--stage sketch] [--type feature] [--priority medium] [--tags tag1,tag2]
```

**Поведение:**
1. Определить следующий доступный ID (сканирование всех стадий)
2. Создать файл `<id>_<slug>.md` в папке стадии (по умолчанию `1_sketch`)
3. Заполнить frontmatter
4. Вывести путь к созданному файлу

**Slug генерация:** название транслителируется или sanitизируется (lowercase, пробелы → `_`, только `[a-z0-9_]`), макс. 50 символов.

### `forge task move`

Перемещение задачи между стадиями.

```
forge task move <id> <stage>
```

**Стадии:** `backlog`, `sketch`, `design`, `todo`, `in_progress`, `done`

**Поведение:**
1. Найти задачу с указанным ID во всех стадиях
2. Если задача — папка, переместить всю папку
3. Если задача — файл, переместить файл
4. Проверить правила переходов (см. ниже)
5. При переходе `design → todo`: создать git-ветку `feature/<id>_<slug>` (если не существует)
6. Вывести подтверждение: `task(001): move sketch -> design`

**Правила переходов:**
- Движение только вперёд: `sketch → design → todo → in_progress → done`
- В `backlog` можно перейти из любой стадии
- Из `backlog` можно вернуться в `sketch`
- В `in_progress` — не более одной задачи (предупреждение, не ошибка)

### `forge task list`

Вывод списка задач.

```
forge task list [--stage <stage>] [--format table|json] [--type <type>]
```

**Поведение:**
1. Сканировать все стадии (или указанную)
2. Парсить frontmatter каждого файла
3. Вывести в формате table (по умолчанию) или json

**Формат table:**
```
ID    TITLE                    STAGE        TYPE      PRIORITY
001   Kanban board CLI         design       feature   high
002   VSCode Extension         sketch       feature   medium
```

**Формат json:**
```json
[
  {
    "id": "001",
    "title": "Kanban board CLI",
    "stage": "design",
    "type": "feature",
    "priority": "high",
    "blocked_by": null,
    "path": ".agent/kanban/2_design/001/"
  }
]
```

### `forge task show`

Показать детали задачи.

```
forge task show <id>
```

**Поведение:**
1. Найти задачу по ID
2. Вывести frontmatter + содержимое файла `0_*.md` (если папка)

---

## Git-интеграция

### Ветвление

Ветка отражает суть задачи, а не стадию:

- `feature/<id>_<slug>` — новая функциональность
- `fix/<id>_<slug>` — исправление бага
- `spike/<id>_<slug>` — исследование/прототип
- `chore/<id>_<slug>` — служебные задачи

### Коммиты при перемещении задач

Формат: `task(<id>): <action>`

Примеры:
- `task(001): add sketch "Kanban board"`
- `task(001): move sketch -> design`
- `task(001): start work`
- `task(001): complete`

> **NOTE:** Утилита `forge` не делает коммиты автоматически. Она выводит команду для выполнения пользователем/агентом.

---

## Обновление `forge init`

Команда `forge init` должна создавать структуру kanban:

```
.agent/kanban/
├── 0_backlog/
├── 1_sketch/
├── 2_design/
├── 3_todo/
├── 4_in_progress/
└── 5_done/
```

И обновлять `.agent/README.md` с правилами kanban.

---

## Критерии приёмки

- [ ] `forge task new "Test"` создаёт файл в `1_sketch/` с корректным frontmatter
- [ ] `forge task move 001 design` перемещает задачу из `1_sketch/` в `2_design/`
- [ ] `forge task list` выводит задачи из всех стадий
- [ ] `forge task list --format json` выводит в JSON
- [ ] `forge task show 001` показывает детали задачи
- [ ] `forge init` создаёт структуру kanban
- [ ] При `move design -> todo` предлагается создать git-ветку
- [ ] Frontmatter парсится корректно (YAML)
- [ ] Slug генерируется из названия задачи
- [ ] ID автоинкрементируется (сканирование существующих задач)