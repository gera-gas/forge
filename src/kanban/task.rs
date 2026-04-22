//! Модуль для работы с задачей kanban-доски
//!
//! Задача — это файл или папка в одной из стадий kanban-доски.
//! Содержит frontmatter (метаданные) и путь к файлу/папке.

use std::path::PathBuf;

use crate::kanban::frontmatter::TaskFrontmatter;
use crate::kanban::stage::Stage;

/// Полная информация о задаче на kanban-доске
#[derive(Debug, Clone)]
pub struct Task {
    /// Метаданные из frontmatter
    pub frontmatter: TaskFrontmatter,
    /// Текущая стадия задачи
    pub stage: Stage,
    /// Путь к файлу или папке задачи
    pub path: PathBuf,
    /// Является ли задача папкой (сложная задача)
    pub is_folder: bool,
}

impl Task {
    /// Создаёт новую задачу
    pub fn new(frontmatter: TaskFrontmatter, stage: Stage, path: PathBuf) -> Self {
        let is_folder = path.is_dir();
        Self {
            frontmatter,
            stage,
            path,
            is_folder,
        }
    }

    /// Возвращает имя файла или папки задачи
    pub fn file_name(&self) -> Option<String> {
        self.path
            .file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.to_string())
    }

    /// Возвращает ID задачи (из frontmatter)
    pub fn id(&self) -> &str {
        &self.frontmatter.id
    }

    /// Возвращает название задачи (из frontmatter)
    pub fn title(&self) -> &str {
        &self.frontmatter.title
    }
}