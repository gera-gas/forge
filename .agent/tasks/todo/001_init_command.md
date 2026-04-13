# Task 001: Реализовать команду `init`

**Design:** [001_init.md](../../design/001_init.md)  
**Ticket:** (пусто)  
**Status:** todo  
**Created:** 2026-04-13

## Описание
Создать базовую структуру каталогов `.agent/` и файлы по умолчанию, если они отсутствуют.

## Требования
1. Команда: `forge init [--path <path>]`.
2. Создавать папки: `.agent/design`, `.agent/specs`, `.agent/tasks/todo`, `.agent/tasks/in_progress`, `.agent/tasks/done`, `.agent/workflows`.
3. Создавать файлы: `README.md`, `concept.md` (пустой шаблон), `tech_stack.md` (пустой), `rules.md` (пустой), `struct.md` (пустой), `workflows/agent.md` (дефолтный).
4. Создавать файлы-указатели в корне: `CLAUDE.md`, `AGENTS.md`, `.clinerules`, `.cursorrules`, `KODA.md`.
5. Если папка `.agent` уже существует — спрашивать подтверждение на перезапись (или пропуск).
6. Использовать `clap` для парсинга аргументов.
7. Поддержка cross-platform (Windows, Linux, macOS).

## Критерии приемки
- [ ] `cargo run -- init` создает структуру файлов.
- [ ] Файлы содержат шаблонный текст из нашего стандарта.
- [ ] Команда работает на Windows, Linux, macOS.
- [ ] Есть обработка существующей папки `.agent/`.
- [ ] Smoke-тесты проходят: `cargo test` завершается успешно.
