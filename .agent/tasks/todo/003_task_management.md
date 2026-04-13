# Task 003: Реализовать команды управления задачами (Kanban)

**Design:** [003_task_kanban.md](../../design/003_task_kanban.md)  
**Ticket:** (пусто)  
**Status:** todo  
**Created:** 2026-04-13

## Описание
Реализовать команды для управления задачами в папках `tasks/todo/`, `tasks/in_progress/`, `tasks/done/`.

## Требования

### Базовые команды (Фаза 1)
1. Команды:
   - `forge task add "описание"` – создает файл в `tasks/todo/`
   - `forge task list` – список задач (по папкам)
   - `forge task start <id>` – перемещает задачу из `todo` в `in_progress`
   - `forge task done <id>` – перемещает в `done/`
2. Поддержка флага `--format json` для `list`.
3. Поддержка cross-platform.

### Интеграция с Trello/Jira (Фаза 2)
4. Команды синхронизации:
   - `forge task sync [--from trello|jira]` – синхронизирует статусы задач с внешними системами
   - `forge task import --from trello --board <id>` – импортирует задачи из Trello/Jira
5. Поле `Ticket:` в файлах задач для хранения ссылок на внешние тикеты.

## Технические детали
- Использовать `clap` для парсинга аргументов.
- Логика перемещения файлов в `cli/task.rs`.
- ID задачи – имя файла без расширения.

## Критерии приемки
- [ ] `forge task add "сделать тесты"` создает файл в `todo/`.
- [ ] `forge task list` показывает все задачи.
- [ ] `forge task start 001` перемещает в `in_progress/`.
- [ ] `forge task done 001` перемещает в `done/`.
- [ ] Работает на всех платформах.