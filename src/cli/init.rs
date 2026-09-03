// Модуль для команды `forge init`
// 
// Этот модуль отвечает за создание структуры forge-src/ в проекте

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
    
    // forge-src/ будет создаваться внутри base_dir
    let forge_dir = base_dir.join("forge-src");

    // Проверяем существует ли forge-src/
    if forge_dir.exists() {
        // Спрашиваем пользователя что делать
        if !confirm_overwrite()? {
            println!("Отменено пользователем.");
            return Ok(());
        }
    }

    // Создаём структуру директорий
    create_directories(&forge_dir)?;
    
    // Создаём файлы с шаблонами
    create_files(&forge_dir)?;
    
    // Создаём файлы-указатели в корне проекта
    create_pointer_files(base_dir)?;

    println!("✅ Структура forge-src/ успешно создана в {:?}", base_dir);
    println!("📖 Начните с редактирования forge-src/concept.md");
    
    Ok(())
}

// ============================================================================
// СОЗДАНИЕ ДИРЕКТОРИЙ
// ============================================================================

/// Создаёт все необходимые директории для forge-src/
/// 
/// # Аргументы
/// * `forge_dir` - Путь к forge-src/
fn create_directories(forge_dir: &Path) -> Result<()> {
    // fs::create_dir_all() - рекурсивно создаёт все папки в пути
    // Это аналог `mkdir -p` в Unix или `mkdir` с ключом в Windows
    // Если папка уже существует — не ошибка (idempotent)
    
    fs::create_dir_all(forge_dir)
        .context("Не удалось создать forge-src/")?;
    
    fs::create_dir_all(forge_dir.join("design"))
        .context("Не удалось создать forge-src/design/")?;
    
    fs::create_dir_all(forge_dir.join("specs"))
        .context("Не удалось создать forge-src/specs/")?;
    
    fs::create_dir_all(forge_dir.join("workflows"))
        .context("Не удалось создать forge-src/workflows/")?;
    
    // Создаём структуру kanban (новый формат)
    fs::create_dir_all(forge_dir.join("kanban/0_backlog"))
        .context("Не удалось создать forge-src/kanban/0_backlog/")?;
    
    fs::create_dir_all(forge_dir.join("kanban/1_sketch"))
        .context("Не удалось создать forge-src/kanban/1_sketch/")?;
    
    fs::create_dir_all(forge_dir.join("kanban/2_design"))
        .context("Не удалось создать forge-src/kanban/2_design/")?;
    
    fs::create_dir_all(forge_dir.join("kanban/3_todo"))
        .context("Не удалось создать forge-src/kanban/3_todo/")?;
    
    fs::create_dir_all(forge_dir.join("kanban/4_in_progress"))
        .context("Не удалось создать forge-src/kanban/4_in_progress/")?;
    
    fs::create_dir_all(forge_dir.join("kanban/5_done"))
        .context("Не удалось создать forge-src/kanban/5_done/")?;
    
    // Создаём .gitkeep в каждой пустой папке kanban
    for stage in &["0_backlog", "1_sketch", "2_design", "3_todo", "4_in_progress", "5_done"] {
        let gitkeep = forge_dir.join("kanban").join(stage).join(".gitkeep");
        write_file(&gitkeep, "")?;
    }

    Ok(())
}

// ============================================================================
// СОЗДАНИЕ ФАЙЛОВ
// ============================================================================

/// Создаёт основные файлы forge-src/ с шаблонами
/// 
/// # Аргументы
/// * `forge_dir` - Путь к forge-src/
fn create_files(forge_dir: &Path) -> Result<()> {
    // Записываем каждый файл из шаблонов
    // Функция write_file() определена ниже
    
    write_file(&forge_dir.join("README.md"), templates::FORGE_README)?;
    write_file(&forge_dir.join("concept.md"), templates::CONCEPT)?;
    write_file(&forge_dir.join("tech_stack.md"), templates::TECH_STACK)?;
    write_file(&forge_dir.join("rules.md"), templates::RULES)?;
    write_file(&forge_dir.join("struct.md"), templates::STRUCT)?;
    write_file(&forge_dir.join("codegen.md"), templates::CODEGEN)?;
    
    write_file(&forge_dir.join("workflows/agent.md"), templates::WORKFLOW_AGENT)?;
    write_file(&forge_dir.join("workflows/git.md"), templates::WORKFLOW_GIT)?;

    Ok(())
}

/// Создаёт файлы-указатели в корне проекта
/// 
/// Эти файлы направляют разные ИИ-агенты в forge-src/README.md
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
        write_file(&base_dir.join(filename), templates::FORGE_POINTER)?;
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
    print!("⚠️  Директория forge-src/ уже существует. Перезаписать? [y/n]: ");
    // В Rust нужно явно flush, чтобы текст появился до ввода
    io::stdout().flush()?;

    // Читаем строку из stdin (стандартный ввод)
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // trim() убирает \n и пробелы, to_lowercase() приводит к нижнему регистру
    // Это делает ввод case-insensitive: Y, y, YES, yes — всё подойдёт
    Ok(input.trim().to_lowercase().starts_with('y'))
}