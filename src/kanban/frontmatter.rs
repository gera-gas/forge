//! Модуль для парсинга и сериализации YAML frontmatter
//!
//! Frontmatter — это YAML-блок в начале .md файла, заключённый между `---`.
//!
//! Пример:
//! ```yaml
//! ---
//! id: "001"
//! title: "Kanban board CLI"
//! type: "feature"
//! priority: "high"
//! blocked_by: null
//! tags: ["cli", "kanban"]
//! ---
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;

/// Тип задачи
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskType {
    Feature,
    Fix,
    Spike,
    Chore,
}

impl fmt::Display for TaskType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskType::Feature => write!(f, "feature"),
            TaskType::Fix => write!(f, "fix"),
            TaskType::Spike => write!(f, "spike"),
            TaskType::Chore => write!(f, "chore"),
        }
    }
}

/// Приоритет задачи
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Priority::Low => write!(f, "low"),
            Priority::Medium => write!(f, "medium"),
            Priority::High => write!(f, "high"),
            Priority::Critical => write!(f, "critical"),
        }
    }
}

/// Frontmatter задачи — метаданные в YAML-заголовке .md файла
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskFrontmatter {
    /// Уникальный ID задачи (например, "001")
    pub id: String,
    /// Краткое название задачи
    pub title: String,
    /// Тип задачи: feature, fix, spike, chore
    #[serde(rename = "type")]
    pub task_type: TaskType,
    /// Приоритет: low, medium, high, critical
    pub priority: Priority,
    /// ID дизайна, из которого порождена задача (например, "001")
    /// Задачи в 3_todo+ должны иметь design для связи с утверждённым ТЗ
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub design: Option<String>,
    /// ID задачи-блокера или null
    pub blocked_by: Option<String>,
    /// Теги задачи
    #[serde(default)]
    pub tags: Vec<String>,
    /// Дата завершения задачи (YYYY-MM-DD), заполняется при перемещении в 5_done
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
}

impl TaskFrontmatter {
    /// Создаёт новый frontmatter с значениями по умолчанию
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            task_type: TaskType::Feature,
            priority: Priority::Medium,
            design: None,
            blocked_by: None,
            tags: Vec::new(),
            completed_at: None,
        }
    }
}

/// Парсит frontmatter из содержимого .md файла
///
/// Ожидает формат:
/// ```text
/// ---
/// id: "001"
/// title: "..."
/// ...
/// ---
/// ```
///
/// # Возвращает
/// * `Some((TaskFrontmatter, remaining_content))` если frontmatter найден
/// * `None` если frontmatter отсутствует
pub fn parse_frontmatter(content: &str) -> Option<(TaskFrontmatter, &str)> {
    // Ищем начало frontmatter
    let content = content.trim_start();
    if !content.starts_with("---") {
        return None;
    }

    // Пропускаем первый ---
    let after_first = content.strip_prefix("---")?;
    // Ищем закрывающий ---
    let end_pos = after_first.find("---")?;
    let yaml_str = &after_first[..end_pos];
    let remaining = &after_first[end_pos + 3..];

    // Парсим YAML
    let fm: TaskFrontmatter = serde_yaml::from_str(yaml_str).ok()?;
    Some((fm, remaining.trim_start()))
}

/// Сериализует frontmatter в YAML-блок для записи в .md файл
///
/// # Формат вывода:
/// ```text
/// ---
/// id: "001"
/// title: "..."
/// ...
/// ---
///
/// # Содержание задачи
/// ```
pub fn serialize_frontmatter(fm: &TaskFrontmatter) -> String {
    let yaml = serde_yaml::to_string(fm).unwrap_or_else(|e| {
        eprintln!("Ошибка сериализации frontmatter: {}", e);
        String::new()
    });
    format!("---\n{}---\n", yaml)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frontmatter_basic() {
        let content = r#"---
id: "001"
title: "Kanban board CLI"
type: "feature"
priority: "high"
blocked_by: null
tags: ["cli", "kanban"]
---
# Kanban board CLI
Some content here.
"#;
        let result = parse_frontmatter(content);
        assert!(result.is_some());
        let (fm, remaining) = result.unwrap();
        assert_eq!(fm.id, "001");
        assert_eq!(fm.title, "Kanban board CLI");
        assert_eq!(fm.task_type, TaskType::Feature);
        assert_eq!(fm.priority, Priority::High);
        assert!(fm.blocked_by.is_none());
        assert_eq!(fm.tags, vec!["cli", "kanban"]);
        assert!(remaining.starts_with("# Kanban"));
    }

    #[test]
    fn test_parse_frontmatter_no_frontmatter() {
        let content = "# Just a regular markdown\nNo frontmatter here.";
        assert!(parse_frontmatter(content).is_none());
    }

    #[test]
    fn test_parse_frontmatter_with_blocked_by() {
        let content = r#"---
id: "005"
title: "Task with blocker"
type: "feature"
priority: "medium"
blocked_by: "004"
tags: []
---
Content here.
"#;
        let result = parse_frontmatter(content);
        assert!(result.is_some());
        let (fm, _) = result.unwrap();
        assert_eq!(fm.blocked_by, Some("004".to_string()));
    }

    #[test]
    fn test_serialize_frontmatter() {
        let fm = TaskFrontmatter {
            id: "003".to_string(),
            title: "Test task".to_string(),
            task_type: TaskType::Fix,
            priority: Priority::Low,
            design: None,
            blocked_by: None,
            tags: vec!["bug".to_string()],
            completed_at: None,
        };
        let serialized = serialize_frontmatter(&fm);
        assert!(serialized.starts_with("---"));
        // serde_yaml может сериализовать строки с кавычками или без
        assert!(serialized.contains("003"));
        assert!(serialized.contains("Test task"));
        assert!(serialized.contains("fix"));
        assert!(serialized.contains("low"));
        assert!(serialized.contains("---\n"));
    }

    #[test]
    fn test_roundtrip() {
        let fm = TaskFrontmatter {
            id: "010".to_string(),
            title: "Round trip test".to_string(),
            task_type: TaskType::Chore,
            priority: Priority::Medium,
            design: Some("001".to_string()),
            blocked_by: Some("009".to_string()),
            tags: vec!["test".to_string()],
            completed_at: None,
        };
        let serialized = serialize_frontmatter(&fm);
        let parsed = parse_frontmatter(&serialized);
        assert!(parsed.is_some());
        let (fm2, _) = parsed.unwrap();
        assert_eq!(fm.id, fm2.id);
        assert_eq!(fm.title, fm2.title);
        assert_eq!(fm.task_type, fm2.task_type);
        assert_eq!(fm.priority, fm2.priority);
        assert_eq!(fm.design, fm2.design);
        assert_eq!(fm.blocked_by, fm2.blocked_by);
        assert_eq!(fm.tags, fm2.tags);
        assert_eq!(fm.completed_at, fm2.completed_at);
    }

    #[test]
    fn test_design_field() {
        let content = r#"---
id: "003"
title: "Task with design"
type: "feature"
priority: "high"
design: "001"
blocked_by: null
tags: ["cli"]
---
Content here.
"#;
        let result = parse_frontmatter(content);
        assert!(result.is_some());
        let (fm, _) = result.unwrap();
        assert_eq!(fm.design, Some("001".to_string()));
    }

    #[test]
    fn test_completed_at_field() {
        let content = r#"---
id: "005"
title: "Completed task"
type: "fix"
priority: "medium"
design: "002"
blocked_by: null
tags: []
completed_at: "2026-04-22"
---
Content here.
"#;
        let result = parse_frontmatter(content);
        assert!(result.is_some());
        let (fm, _) = result.unwrap();
        assert_eq!(fm.completed_at, Some("2026-04-22".to_string()));
        assert_eq!(fm.design, Some("002".to_string()));
    }

    #[test]
    fn test_optional_fields_absent() {
        // Без design и completed_at — поля должны быть None
        let content = r#"---
id: "001"
title: "Simple task"
type: "feature"
priority: "low"
blocked_by: null
tags: []
---
Content here.
"#;
        let result = parse_frontmatter(content);
        assert!(result.is_some());
        let (fm, _) = result.unwrap();
        assert_eq!(fm.design, None);
        assert_eq!(fm.completed_at, None);
    }

    #[test]
    fn test_roundtrip_with_all_fields() {
        let fm = TaskFrontmatter {
            id: "042".to_string(),
            title: "Full task".to_string(),
            task_type: TaskType::Feature,
            priority: Priority::Critical,
            design: Some("007".to_string()),
            blocked_by: Some("040".to_string()),
            tags: vec!["api".to_string(), "core".to_string()],
            completed_at: Some("2026-12-31".to_string()),
        };
        let serialized = serialize_frontmatter(&fm);
        let parsed = parse_frontmatter(&serialized);
        assert!(parsed.is_some());
        let (fm2, _) = parsed.unwrap();
        assert_eq!(fm.design, fm2.design);
        assert_eq!(fm.completed_at, fm2.completed_at);
        assert_eq!(fm.blocked_by, fm2.blocked_by);
    }

    #[test]
    fn test_task_type_display() {
        assert_eq!(format!("{}", TaskType::Feature), "feature");
        assert_eq!(format!("{}", TaskType::Fix), "fix");
        assert_eq!(format!("{}", TaskType::Spike), "spike");
        assert_eq!(format!("{}", TaskType::Chore), "chore");
    }

    #[test]
    fn test_priority_display() {
        assert_eq!(format!("{}", Priority::Low), "low");
        assert_eq!(format!("{}", Priority::Medium), "medium");
        assert_eq!(format!("{}", Priority::High), "high");
        assert_eq!(format!("{}", Priority::Critical), "critical");
    }
}