// Модуль для команды `forge init`
// 
// Этот модуль отвечает за создание структуры .agent/ в проекте

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use std::io::{self, Write};

use crate::templates;

// ============================================================================
// ГЛАВНАЯ ФУНКЦИЯ КОМАНДЫ
// ============================================================================

/// Выполняет команду `forge init`
/// 
/// # Аргументы
/// * `path` - Путь к проекту (если None, используется текущая директория)
/// 
/// # Возвращает
/// * `Ok(())` - если всё прошло успешно
/// * `Err(...)` - если произошла ошибка
pub fn execute(path: Option<String>) -> Result<()> {
    // Определяем базовую директорию проекта
    // unwrap_or(".") означает: если path = None, используем "."
    // Path::new() создаёт путь (кросс-платформенно, в отличие от строк)
    let base_dir = Path::new(path.as_deref().unwrap_or("."));
    
    // .agent/ будет создаваться внутри base_dir
    let agent_dir = base_dir.join(".agent");

    // Проверяем существует ли .agent/
    if agent_dir.exists() {
        // Спрашиваем пользователя что делать
        if !confirm_overwrite()? {
            println!("Отменено пользователем.");
            return Ok(());
        }
    }

    // Создаём структуру директорий
    create_directories(&agent_dir)?;
    
    // Создаём файлы с шаблонами
    create_files(&agent_dir)?;
    
    // Создаём файлы-указатели в корне проекта
    create_pointer_files(base_dir)?;

    println!("✅ Структура .agent/ успешно создана в {:?}", base_dir);
    println!("📖 Начните с редактирования .agent/concept.md");
    
    Ok(())
}

// ============================================================================
// СОЗДАНИЕ ДИРЕКТОРИЙ
// ============================================================================

/// Создаёт все необходимые директории для .agent/
/// 
/// # Аргументы
/// * `agent_dir` - Путь к .agent/
fn create_directories(agent_dir: &Path) -> Result<()> {
    // fs::create_dir_all() - рекурсивно создаёт все папки в пути
    // Это аналог `mkdir -p` в Unix или `mkdir` с ключом в Windows
    // Если папка уже существует — не ошибка (idempotent)
    
    fs::create_dir_all(agent_dir)
        .context("Не удалось создать .agent/")?;
    
    fs::create_dir_all(agent_dir.join("design"))
        .context("Не удалось создать .agent/design/")?;
    
    fs::create_dir_all(agent_dir.join("specs"))
        .context("Не удалось создать .agent/specs/")?;
    
    fs::create_dir_all(agent_dir.join("workflows"))
        .context("Не удалось создать .agent/workflows/")?;
    
    // Создаём все папки для задач (Kanban)
    fs::create_dir_all(agent_dir.join("tasks/backlog"))
        .context("Не удалось создать .agent/tasks/backlog/")?;
    
    fs::create_dir_all(agent_dir.join("tasks/todo"))
        .context("Не удалось создать .agent/tasks/todo/")?;
    
    fs::create_dir_all(agent_dir.join("tasks/in_progress"))
        .context("Не удалось создать .agent/tasks/in_progress/")?;
    
    fs::create_dir_all(agent_dir.join("tasks/done"))
        .context("Не удалось создать .agent/tasks/done/")?;

    Ok(())
}

// ============================================================================
// СОЗДАНИЕ ФАЙЛОВ
// ============================================================================

/// Создаёт основные файлы .agent/ с шаблонами
/// 
/// # Аргументы
/// * `agent_dir` - Путь к .agent/
fn create_files(agent_dir: &Path) -> Result<()> {
    // Записываем каждый файл из шаблонов
    // Функция write_file() определена ниже
    
    write_file(&agent_dir.join("README.md"), templates::AGENT_README)?;
    write_file(&agent_dir.join("concept.md"), templates::CONCEPT)?;
    write_file(&agent_dir.join("tech_stack.md"), templates::TECH_STACK)?;
    write_file(&agent_dir.join("rules.md"), templates::RULES)?;
    write_file(&agent_dir.join("struct.md"), templates::STRUCT)?;
    
    write_file(&agent_dir.join("workflows/agent.md"), templates::WORKFLOW_AGENT)?;
    write_file(&agent_dir.join("workflows/git.md"), templates::WORKFLOW_GIT)?;

    Ok(())
}

/// Создаёт файлы-указатели в корне проекта
/// 
/// Эти файлы направляют разные ИИ-агенты в .agent/README.md
/// 
/// # Аргументы
/// * `base_dir` - Корневая директория проекта
fn create_pointer_files(base_dir: &Path) -> Result<()> {
    let pointers = vec![
        "CLAUDE.md",      // Claude Code
        "AGENTS.md",      // OpenAI Codex
        ".clinerules",    // Cline
        ".cursorrules",   // Cursor
        "KODA.md",        // Koda
    ];

    for filename in pointers {
        write_file(&base_dir.join(filename), templates::AGENT_POINTER)?;
    }

    Ok(())
}

// ============================================================================
// ВСПОМОГАТЕЛЬНЫЕ ФУНКЦИИ
// ============================================================================

/// Записывает содержимое в файл
/// 
/// # Аргументы
/// * `path` - Путь к файлу
/// * `content` - Содержимое для записи
fn write_file(path: &Path, content: &str) -> Result<()> {
    fs::write(path, content)
        .with_context(|| format!("Не удалось записать файл {:?}", path))?;
    Ok(())
}

/// Спрашивает у пользователя подтверждение на перезапись
/// 
/// # Возвращает
/// * `Ok(true)` - если пользователь подтвердил
/// * `Ok(false)` - если пользователь отказался
/// * `Err(...)` - если произошла ошибка ввода/вывода
fn confirm_overwrite() -> Result<bool> {
    print!("⚠️  Директория .agent/ уже существует. Перезаписать? [y/n]: ");
    // В Rust нужно явно flush, чтобы текст появился до ввода
    io::stdout().flush()?;

    // Читаем строку из stdin (стандартный ввод)
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // trim() убирает \n и пробелы, to_lowercase() приводит к нижнему регистру
    // Это делает ввод case-insensitive: Y, y, YES, yes — всё подойдёт
    Ok(input.trim().to_lowercase().starts_with('y'))
}
