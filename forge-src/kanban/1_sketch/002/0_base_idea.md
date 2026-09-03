# Идея: VSCode Extension + Дистрибуция

## Введение

Развитие утилиты `forge` в сторону экосистемы:
1. VSCode Extension — графическая Kanban-доска для удобной работы с `forge/kanban/`
2. Дистрибуция — сайт/маркетплейс для скачивания плагина и документации

---

## Дочерние скетчи

Этот скетч разделён на независимые направления:

| Файл | Направление | Приоритет |
|------|-------------|-----------|
| [1_vscode_extension.md](1_vscode_extension.md) | VSCode Extension с Kanban-доской | **MVP** |
| [2_multi_agent.md](2_multi_agent.md) | Мульти-агентная система (Arch/Dev/Ops) | Phase 2 |
| [3_distribution.md](3_distribution.md) | Дистрибуция и сайт | Phase 2 |
| [4_paid_features.md](4_paid_features.md) | Платные функции и монетизация | Phase 3+ |
| [5_mobile_control.md](5_mobile_control.md) | Мобильное управление | Phase 2 |

---

## Решения по открытым вопросам

> Исходные вопросы из первой версии скетча. Решения зафиксированы 2026-05-03.

| # | Вопрос | Решение | Обоснование |
|---|--------|---------|-------------|
| 1 | LLM провайдер | MVP = OpenRouter (агрегатор). Архитектура: trait `LlmProvider` | Один API — доступ ко всем моделям. Легко добавить прямых провайдеров потом |
| 2 | Аутентификация | Отложить до Phase 2 | Для соло-разработчика не нужна. Появляется с облаком/командой |
| 3 | Монетизация | Open Core: Free → Pro → Enterprise | Соло = Free, команда = Pro, Enterprise = кастом. См. [4_paid_features.md](4_paid_features.md) |
| 4 | Комьюнити | Начать с Telegram, добавить Discord позже | RU-аудитория → Telegram. Международный масштаб → Discord |
| 5 | Лицензия | MIT для открытого, проприетарное — Enterprise | Максимум открытости для community |
| 6 | Оркестратор | MVP = правила (без LLM) | Детерминированно, бесплатно. LLM-классификатор — Phase 3+. См. [2_multi_agent.md](2_multi_agent.md) |
| 7 | Skills/роли | Да, отдельные файлы `forge/roles/<role>.md` | Независимая настройка каждой роли. См. [2_multi_agent.md](2_multi_agent.md) |
| 8 | Mobile control | Phase 2: Telegram Bot. Phase 3+: мобильное приложение | Минимум усилий для максимума пользы. См. [5_mobile_control.md](5_mobile_control.md) |
| 9 | Самообновление | Phase 2: `forge self update`. MVP — `cargo install` | Стандарт для CLI-утилит, но не критично для MVP |
| 10 | wrap | Высокий приоритет. ИИ для извлечения смысла из хаоса | Снижает барьер входа до нуля. Дизайн: [002_wrap.md](../../../design/002_wrap.md) |
| 11 | Zed 1.0 | Нет, VSCode первый. Zed — в backlog | VSCode = 70%+ рынка. Zed plugin API ещё сырой. Пересмотреть через 6-12 мес |

---

## Приоритеты

### MVP (первый релиз) → [1_vscode_extension.md](1_vscode_extension.md)
1. VSCode Extension с базовой Kanban-доской
2. Drag & drop между стадиями
3. Создание/редактирование задач
4. Git-интеграция (базовая)
5. Публикация в VSCode Marketplace
6. Простой landing page

### Phase 2 → [2_multi_agent.md](2_multi_agent.md), [3_distribution.md](3_distribution.md), [5_mobile_control.md](5_mobile_control.md)
1. Мульти-агентная система (Arch / Dev / Ops)
2. Оркестратор запросов
3. Настройки стоимости и лимитов
4. Локальный LLM через OpenRouter
5. Telegram Bot для уведомлений
6. Улучшенная Git-интеграция

### Phase 3+ → [4_paid_features.md](4_paid_features.md)
1. Сайт с логином и аналитикой
2. Платные функции (Jira/Trello синхронизация)
3. Мобильное приложение (Tauri или native)

---

## Следующие шаги

1. ~~Обсудить этот скетч и уточнить приоритеты~~ ✅
2. Создать дизайн Phase 1 (MVP) в `2_design/` — из [1_vscode_extension.md](1_vscode_extension.md)
3. Подготовить техническую спецификацию для мульти-агентной системы
4. Начать разработку VSCode Extension (псевдоним Forge VS)
5. Исследовать API Jira/Trello для синхронизации (Phase 3)