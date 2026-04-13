# Design 003: Управление задачами (Kanban)

**Дата:** 2026-04-13  
**Статус:** Approved  
**Связанные задачи:** [003_task_management.md](../tasks/todo/003_task_management.md)

---

## Цель

Реализовать команды для управления задачами в файловой Kanban-доске (`tasks/todo/`, `in_progress/`, `done/`).

## Требования

### Функциональные

1. **Команда `forge task add "<описание>"`**
   - Создаёт файл задачи в `tasks/todo/`
   - Автоматически определяет следующий номер (например, `006`)
   - Формат имени: `<номер>_<sanitized_description>.md`
   - Заполняет шаблон с полями: Design, Ticket, Status, Created

2. **Команда `forge task list [--format <table|json>]`**
   - Выводит список всех задач (todo, in_progress, done)
   - Формат table: таблица с колонками ID, Title, Status, Design
   - Формат json: JSON-массив с полной информацией

3. **Команда `forge task start <id>`**
   - Находит файл в `tasks/todo/` с префиксом `<id>`
   - Перемещает в `tasks/in_progress/`
   - Обновляет поле `Status: in_progress` в файле

4. **Команда `forge task done <id>`**
   - Находит файл в `tasks/in_progress/` с префиксом `<id>`
   - Перемещает в `tasks/done/`
   - Обновляет поле `Status: done` и добавляет `Completed: <date>`

### Дополнительно (фаза 2)

5. **Команда `forge task sync [--from trello|jira]`**
   - Синхронизирует статусы задач с внешними системами
   - Читает поле `Ticket:` из файла задачи
   - Обновляет локальный статус на основе API

6. **Команда `forge task import --from trello --board <id>`**
   - Импортирует задачи из Trello/Jira
   - Создаёт файлы в `tasks/todo/`
   - Заполняет поле `Ticket:` ссылкой

## Формат файла задачи

```markdown
# Task <ID>: <Название>

**Design:** [<номер_дизайна>.md](../design/<номер_дизайна>.md)  
**Ticket:** <URL или пусто>  
**Status:** todo | in_progress | done  
**Created:** YYYY-MM-DD  
**Completed:** YYYY-MM-DD (только для done)

## Описание
...

## Требования
...

## Критерии приемки
- [ ] ...
```

## Технические

- **Парсинг Markdown:** Простой regex для извлечения полей
- **Обновление полей:** Regex замена в тексте файла
- **Sanitize имени файла:** Убрать спецсимволы, пробелы → `_`
- **Trello/Jira API:** `reqwest` + OAuth/Token авторизация

## Архитектурные решения

1. **Модульная структура:**
   - `src/cli/task.rs` — логика команд
   - `src/task/parser.rs` — парсинг файлов задач
   - `src/task/external.rs` — интеграция с Trello/Jira

2. **ID задачи:**
   - Номер из имени файла (префикс до первого `_`)
   - Пример: `001_init_command.md` → ID = `001`

## Источники

- Исходный дизайн: [00_init/from_z-ai_GLM.md](00_init/from_z-ai_GLM.md)
- Исходный дизайн: [00_init/from_DeepSeek.md](00_init/from_DeepSeek.md)
