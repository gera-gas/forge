# Design 002: Команда `forge wrap`

**Дата:** 2026-04-13  
**Статус:** Approved  
**Связанные задачи:** [002_wrap_command.md](../tasks/todo/002_wrap_command.md)

---

## Цель

Реализовать команду `forge wrap`, которая анализирует существующий проект и генерирует описание через LLM.

## Требования

### Функциональные

1. **Команда:** `forge wrap [--source <путь>] [--model <модель>]`
   - `--source` (по умолчанию `.`) — директория для анализа
   - `--model` (по умолчанию `gpt-4o`) — модель LLM

2. **Сканирование проекта:**
   - Обход файловой структуры (tree)
   - Определение языка программирования (эвристики по файлам)
   - Чтение ключевых файлов:
     - Rust: `Cargo.toml`, `main.rs`, `lib.rs`
     - Python: `requirements.txt`, `pyproject.toml`, `main.py`
     - C/C++: `CMakeLists.txt`, `Makefile`, `main.c/cpp`
     - Node.js: `package.json`, `index.js`, `app.js`

3. **Генерация через LLM:**
   - Формирование промпта с контекстом проекта
   - Отправка запроса к OpenAI-compatible API
   - Сохранение результатов в `.agent/`:
     - `concept.md` — концепция проекта
     - `tech_stack.md` — технологический стек
     - `struct.md` — модульная структура

4. **Конфигурация API:**
   - API Key из переменных окружения: `FORGE_API_KEY` или `OPENAI_API_KEY`
   - Возможность указать API endpoint через `FORGE_API_URL`

### Технические

- **HTTP-клиент:** `reqwest` + `tokio` для async
- **Сканирование файлов:** `walkdir`
- **Эвристики:** Модуль `src/scanner/heuristics.rs`
- **Промпты:** Шаблоны в `src/llm/prompts.rs`
- **Обработка ошибок:** Обрабатывать network errors, API errors, file I/O errors

## Архитектурные решения

1. **Модульная структура:**
   - `src/scanner/` — сканирование файловой структуры
   - `src/llm/` — взаимодействие с LLM API
   - `src/cli/wrap.rs` — логика команды

2. **Формат промпта:**
   ```
   Проанализируй проект с следующей структурой:
   
   Файловая структура:
   <tree>
   
   Ключевые файлы:
   <content>
   
   Сгенерируй:
   1. concept.md — концепция (цели, ограничения)
   2. tech_stack.md — стек технологий
   3. struct.md — модульная структура
   ```

## Источники

- Исходный дизайн: [00_init/from_z-ai_GLM.md](00_init/from_z-ai_GLM.md)
- Исходный дизайн: [00_init/from_DeepSeek.md](00_init/from_DeepSeek.md)
