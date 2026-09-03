# Design 001: Команда `forge init`

**Дата:** 2026-04-13  
**Статус:** Approved  
**Связанные задачи:** [001_init_command.md](../tasks/todo/001_init_command.md)

---

## Цель

Реализовать команду `forge init`, которая создаёт базовую структуру `.agent/` в проекте.

## Требования

### Функциональные

1. **Команда:** `forge init [--path <путь>]`
   - По умолчанию создаёт `.agent/` в текущей директории
   - С флагом `--path` — в указанной директории

2. **Создаваемая структура:**
   ```
   .agent/
   ├── README.md
   ├── concept.md
   ├── tech_stack.md
   ├── rules.md
   ├── struct.md
   ├── workflows/
   │   ├── agent.md
   │   └── git.md
   ├── design/
   ├── specs/
   └── tasks/
       ├── todo/
       ├── in_progress/
       └── done/
   ```

3. **Файлы-указатели в корне проекта:**
   - `CLAUDE.md`
   - `AGENTS.md`
   - `.clinerules`
   - `.cursorrules`
   - `KODA.md`
   
   Каждый содержит: `Read '.agent/README.md' first for project context and configuration.`

4. **Обработка существующей структуры:**
   - Если `.agent/` уже существует — вывести предупреждение и спросить подтверждение
   - Предложить варианты: `[o]verwrite / [s]kip / [a]bort`

### Технические

- Использовать `std::fs` для создания файлов/директорий (кросс-платформенность)
- Шаблоны файлов хранить как константы или встроить через `include_str!`
- Обработка ошибок через `anyhow`

## Архитектурные решения

1. **Шаблоны файлов:** Хранить как константы в `src/templates/`
2. **Модульная структура:** Логика в `src/cli/init.rs`

## Источники

- Исходный дизайн: [00_init/from_z-ai_GLM.md](00_init/from_z-ai_GLM.md)
- Исходный дизайн: [00_init/from_DeepSeek.md](00_init/from_DeepSeek.md)
