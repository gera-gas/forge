//! Модуль для работы с kanban-доской
//!
//! Предоставляет функции для:
//! - Поиска корневой директории kanban
//! - Сканирования стадий и задач
//! - Поиска задач по ID
//! - Определения следующего доступного ID

pub mod frontmatter;
pub mod slug;
pub mod stage;
pub mod task;

use std::fs;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::kanban::frontmatter::{parse_frontmatter, TaskFrontmatter};
use crate::kanban::stage::Stage;
use crate::kanban::task::Task;

/// Имя корневой директории kanban
const KANBAN_DIR: &str = "forge-src/kanban";

/// Ищет корневую директорию kanban от текущей директории вверх по дереву
///
/// # Возвращает
/// * `Some(PathBuf)` — путь к `forge-src/kanban/`
/// * `None` — если директория не найдена
pub fn find_kanban_root() -> Option<PathBuf> {
    let current_dir = std::env::current_dir().ok()?;
    let mut dir = current_dir.as_path();

    loop {
        let kanban_path = dir.join(KANBAN_DIR);
        if kanban_path.is_dir() {
            return Some(kanban_path);
        }

        // Поднимаемся на уровень выше
        dir = dir.parent()?;
    }
}

/// Возвращает список всех стадий kanban-доски
///
/// Сканирует директорию kanban и возвращает найденные стадии
pub fn list_stages(kanban_root: &Path) -> Vec<Stage> {
    let mut stages = Vec::new();

    if let Ok(entries) = fs::read_dir(kanban_root) {
        for entry in entries.flatten() {
            let file_name = entry.file_name();
            let name = file_name.to_string_lossy();
            if let Some(stage) = Stage::from_dir_name(&name) {
                stages.push(stage);
            }
        }
    }

    stages.sort();
    stages
}

/// Сканирует все стадии и возвращает список всех задач
///
/// # Аргументы
/// * `kanban_root` — путь к `forge-src/kanban/`
///
/// # Возвращает
/// Вектор всех найденных задач
pub fn scan_all_tasks(kanban_root: &Path) -> Vec<Task> {
    let mut tasks = Vec::new();

    for stage in Stage::all() {
        let stage_dir = kanban_root.join(stage.dir_name());
        if !stage_dir.is_dir() {
            continue;
        }

        // Сканируем содержимое стадии
        for entry in WalkDir::new(&stage_dir)
            .max_depth(2) // Задача может быть папкой
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();

            // Пропускаем саму директорию стадии
            if path == stage_dir {
                continue;
            }

            // Проверяем, является ли элемент задачей
            // Задача — это .md файл или папка с .md файлами внутри
            let file_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");

            // Пропускаем .gitkeep и скрытые файлы
            if file_name.starts_with('.') {
                continue;
            }

            // Если это .md файл — это простая задача
            if path.extension().and_then(|e| e.to_str()) == Some("md") {
                // Убедимся, что файл находится непосредственно в стадии
                // (а не вложен в папку задачи)
                let parent = path.parent();
                if parent == Some(stage_dir.as_path()) {
                    if let Some(task) = load_task_from_file(path, *stage) {
                        tasks.push(task);
                    }
                }
            }
            // Если это папка — это сложная задача
            else if path.is_dir() {
                // Убедимся, что папка находится непосредственно в стадии
                let parent = path.parent();
                if parent == Some(stage_dir.as_path()) {
                    // Ищем файл 0_*.md внутри папки для frontmatter
                    if let Some(task) = load_task_from_folder(path, *stage) {
                        tasks.push(task);
                    }
                }
            }
        }
    }

    tasks
}

/// Загружает задачу из .md файла
fn load_task_from_file(path: &Path, stage: Stage) -> Option<Task> {
    let content = fs::read_to_string(path).ok()?;
    let (frontmatter, _) = parse_frontmatter(&content)?;
    Some(Task::new(frontmatter, stage, path.to_path_buf()))
}

/// Загружает задачу из папки (ищет файл 0_*.md)
fn load_task_from_folder(dir_path: &Path, stage: Stage) -> Option<Task> {
    // Ищем файл 0_*.md внутри папки
    for entry in fs::read_dir(dir_path).ok()? {
        let entry = entry.ok()?;
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();

        if name.starts_with("0_") && name.ends_with(".md") {
            let file_path = entry.path();
            let content = fs::read_to_string(&file_path).ok()?;
            let (frontmatter, _) = parse_frontmatter(&content)?;
            return Some(Task::new(frontmatter, stage, dir_path.to_path_buf()));
        }
    }
    None
}

/// Находит задачу по ID во всех стадиях
///
/// # Аргументы
/// * `kanban_root` — путь к `forge-src/kanban/`
/// * `id` — ID задачи (например, "001")
///
/// # Возвращает
/// * `Some(Task)` если задача найдена
/// * `None` если задача не найдена
pub fn find_task_by_id(kanban_root: &Path, id: &str) -> Option<Task> {
    let tasks = scan_all_tasks(kanban_root);
    tasks.into_iter().find(|t| t.id() == id)
}

/// Определяет следующий доступный ID
///
/// Сканирует все стадии и находит максимальный ID, затем возвращает следующий.
/// ID имеет формат трёхзначного числа: "001", "002", ..., "999"
///
/// # Аргументы
/// * `kanban_root` — путь к `forge-src/kanban/`
///
/// # Возвращает
/// Строку с следующим доступным ID (например, "006")
pub fn next_available_id(kanban_root: &Path) -> String {
    let tasks = scan_all_tasks(kanban_root);
    let max_id = tasks
        .iter()
        .filter_map(|t| t.id().parse::<u32>().ok())
        .max()
        .unwrap_or(0);

    format!("{:03}", max_id + 1)
}

/// Создаёт файл задачи в указанной стадии
///
/// # Аргументы
/// * `kanban_root` — путь к `forge-src/kanban/`
/// * `stage` — стадия для создания задачи
/// * `frontmatter` — метаданные задачи
/// * `slug` — slug из названия задачи
///
/// # Возвращает
/// Путь к созданному файлу
pub fn create_task_file(
    kanban_root: &Path,
    stage: &Stage,
    frontmatter: &TaskFrontmatter,
    slug: &str,
) -> anyhow::Result<PathBuf> {
    let stage_dir = kanban_root.join(stage.dir_name());

    // Убедимся, что директория стадии существует
    fs::create_dir_all(&stage_dir)?;

    // Формируем имя файла: <id>_<slug>.md
    let file_name = format!("{}_{}.md", frontmatter.id, slug);
    let file_path = stage_dir.join(&file_name);

    // Формируем содержимое файла
    let frontmatter_str = frontmatter::serialize_frontmatter(frontmatter);
    let content = format!(
        "{}\n# {}\n\nОписание задачи...\n",
        frontmatter_str, frontmatter.title
    );

    // Записываем файл
    fs::write(&file_path, content)?;

    Ok(file_path)
}

/// Записывает дату завершения в frontmatter задачи
///
/// Читает файл задачи, добавляет `completed_at` с текущей датой (YYYY-MM-DD)
/// и перезаписывает файл. Вызывать ДО перемещения в 5_done.
///
/// # Аргументы
/// * `task` — задача для обновления
pub fn set_completed_at(task: &Task) -> anyhow::Result<()> {
    let content = fs::read_to_string(&task.path)?;
    let (mut fm, remaining) = parse_frontmatter(&content)
        .ok_or_else(|| anyhow::anyhow!("Не удалось распарсить frontmatter задачи"))?;

    // Текущая дата в формате YYYY-MM-DD
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    fm.completed_at = Some(today);

    // Перезаписываем файл
    let new_content = format!(
        "{}\n{}",
        frontmatter::serialize_frontmatter(&fm),
        remaining
    );
    fs::write(&task.path, new_content)?;

    Ok(())
}

/// Перемещает задачу в другую стадию
///
/// # Аргументы
/// * `task` — задача для перемещения
/// * `target_stage` — целевая стадия
/// * `kanban_root` — путь к `forge-src/kanban/`
///
/// # Возвращает
/// Новый путь к задаче
pub fn move_task(task: &Task, target_stage: &Stage, kanban_root: &Path) -> anyhow::Result<PathBuf> {
    let _source_dir = kanban_root.join(task.stage.dir_name());
    let target_dir = kanban_root.join(target_stage.dir_name());

    // Убедимся, что целевая директория существует
    fs::create_dir_all(&target_dir)?;

    let source_path = &task.path;
    let file_name = source_path
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("Не удалось получить имя файла задачи"))?;

    let target_path = target_dir.join(file_name);

    // Перемещаем файл или папку
    fs::rename(source_path, &target_path)?;

    Ok(target_path)
}