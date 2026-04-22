//! Модуль для работы со стадиями Kanban-доски
//!
//! Стадии: backlog, sketch, design, todo, in_progress, done
//! Каждая стадия соответствует папке в .agent/kanban/

use std::fmt;
use std::str::FromStr;

/// Стадии Kanban-доски
///
/// Порядок стадий определяет допустимые переходы:
/// backlog -> sketch -> design -> todo -> in_progress -> done
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Stage {
    Backlog,
    Sketch,
    Design,
    Todo,
    InProgress,
    Done,
}

impl Stage {
    /// Возвращает имя папки для стадии (например, "0_backlog")
    pub fn dir_name(&self) -> &'static str {
        match self {
            Stage::Backlog => "0_backlog",
            Stage::Sketch => "1_sketch",
            Stage::Design => "2_design",
            Stage::Todo => "3_todo",
            Stage::InProgress => "4_in_progress",
            Stage::Done => "5_done",
        }
    }

    /// Возвращает все стадии в порядке
    pub fn all() -> &'static [Stage] {
        &[
            Stage::Backlog,
            Stage::Sketch,
            Stage::Design,
            Stage::Todo,
            Stage::InProgress,
            Stage::Done,
        ]
    }

    /// Определяет стадию по имени папки
    ///
    /// # Аргументы
    /// * `name` - Имя папки (например, "1_sketch" или "sketch")
    ///
    /// # Возвращает
    /// * `Some(Stage)` если имя распознано
    /// * `None` если имя не распознано
    pub fn from_dir_name(name: &str) -> Option<Stage> {
        // Сначала пробуем точное совпадение с именем папки
        for stage in Stage::all() {
            if name == stage.dir_name() {
                return Some(*stage);
            }
        }
        // Затем пробуем совпадение без числового префикса
        match name {
            "backlog" => Some(Stage::Backlog),
            "sketch" => Some(Stage::Sketch),
            "design" => Some(Stage::Design),
            "todo" => Some(Stage::Todo),
            "in_progress" | "in-progress" | "inprogress" => Some(Stage::InProgress),
            "done" => Some(Stage::Done),
            _ => None,
        }
    }

    /// Проверяет, допустим ли переход к целевой стадии
    ///
    /// Правила:
    /// - Вперёд: sketch -> design -> todo -> in_progress -> done
    /// - В backlog можно перейти из любой стадии
    /// - Из backlog можно перейти только в sketch
    /// - В in_progress — предупреждение если уже есть задача (не ошибка)
    /// - Запрет возврата назад (кроме backlog)
    pub fn can_move_to(&self, target: &Stage) -> bool {
        if self == target {
            return false; // Нельзя переместить в ту же стадию
        }
        match target {
            Stage::Backlog => true, // В backlog можно из любой стадии
            Stage::Sketch => {
                // В sketch можно из backlog или оставаясь на месте
                matches!(self, Stage::Backlog)
            }
            Stage::Design => {
                matches!(self, Stage::Sketch)
            }
            Stage::Todo => {
                matches!(self, Stage::Design | Stage::Sketch)
            }
            Stage::InProgress => {
                // В in_progress можно из todo
                matches!(self, Stage::Todo)
            }
            Stage::Done => {
                // В done можно из in_progress (или из любой для гибкости)
                true
            }
        }
    }
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stage::Backlog => write!(f, "backlog"),
            Stage::Sketch => write!(f, "sketch"),
            Stage::Design => write!(f, "design"),
            Stage::Todo => write!(f, "todo"),
            Stage::InProgress => write!(f, "in_progress"),
            Stage::Done => write!(f, "done"),
        }
    }
}

impl FromStr for Stage {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "backlog" => Ok(Stage::Backlog),
            "sketch" => Ok(Stage::Sketch),
            "design" => Ok(Stage::Design),
            "todo" => Ok(Stage::Todo),
            "in_progress" | "in-progress" | "inprogress" => Ok(Stage::InProgress),
            "done" => Ok(Stage::Done),
            _ => Err(format!(
                "Неизвестная стадия: '{}'. Допустимые: backlog, sketch, design, todo, in_progress, done",
                s
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dir_name() {
        assert_eq!(Stage::Backlog.dir_name(), "0_backlog");
        assert_eq!(Stage::Sketch.dir_name(), "1_sketch");
        assert_eq!(Stage::Design.dir_name(), "2_design");
        assert_eq!(Stage::Todo.dir_name(), "3_todo");
        assert_eq!(Stage::InProgress.dir_name(), "4_in_progress");
        assert_eq!(Stage::Done.dir_name(), "5_done");
    }

    #[test]
    fn test_from_dir_name() {
        assert_eq!(Stage::from_dir_name("0_backlog"), Some(Stage::Backlog));
        assert_eq!(Stage::from_dir_name("1_sketch"), Some(Stage::Sketch));
        assert_eq!(Stage::from_dir_name("2_design"), Some(Stage::Design));
        assert_eq!(Stage::from_dir_name("3_todo"), Some(Stage::Todo));
        assert_eq!(Stage::from_dir_name("4_in_progress"), Some(Stage::InProgress));
        assert_eq!(Stage::from_dir_name("5_done"), Some(Stage::Done));

        // Без числового префикса
        assert_eq!(Stage::from_dir_name("backlog"), Some(Stage::Backlog));
        assert_eq!(Stage::from_dir_name("sketch"), Some(Stage::Sketch));
        assert_eq!(Stage::from_dir_name("design"), Some(Stage::Design));
        assert_eq!(Stage::from_dir_name("todo"), Some(Stage::Todo));
        assert_eq!(Stage::from_dir_name("in_progress"), Some(Stage::InProgress));
        assert_eq!(Stage::from_dir_name("done"), Some(Stage::Done));

        // Неизвестное имя
        assert_eq!(Stage::from_dir_name("unknown"), None);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Stage::Backlog), "backlog");
        assert_eq!(format!("{}", Stage::Sketch), "sketch");
        assert_eq!(format!("{}", Stage::Design), "design");
        assert_eq!(format!("{}", Stage::Todo), "todo");
        assert_eq!(format!("{}", Stage::InProgress), "in_progress");
        assert_eq!(format!("{}", Stage::Done), "done");
    }

    #[test]
    fn test_from_str() {
        assert_eq!("backlog".parse::<Stage>(), Ok(Stage::Backlog));
        assert_eq!("sketch".parse::<Stage>(), Ok(Stage::Sketch));
        assert_eq!("design".parse::<Stage>(), Ok(Stage::Design));
        assert_eq!("todo".parse::<Stage>(), Ok(Stage::Todo));
        assert_eq!("in_progress".parse::<Stage>(), Ok(Stage::InProgress));
        assert_eq!("done".parse::<Stage>(), Ok(Stage::Done));

        // Case insensitive
        assert_eq!("Backlog".parse::<Stage>(), Ok(Stage::Backlog));
        assert_eq!("DESIGN".parse::<Stage>(), Ok(Stage::Design));

        // Unknown
        assert!("unknown".parse::<Stage>().is_err());
    }

    #[test]
    fn test_can_move_to_forward() {
        // Прямой переход вперёд
        assert!(Stage::Sketch.can_move_to(&Stage::Design));
        assert!(Stage::Design.can_move_to(&Stage::Todo));
        assert!(Stage::Todo.can_move_to(&Stage::InProgress));
        assert!(Stage::InProgress.can_move_to(&Stage::Done));
    }

    #[test]
    fn test_can_move_to_backlog() {
        // В backlog можно из любой стадии
        assert!(Stage::Sketch.can_move_to(&Stage::Backlog));
        assert!(Stage::Design.can_move_to(&Stage::Backlog));
        assert!(Stage::Todo.can_move_to(&Stage::Backlog));
        assert!(Stage::InProgress.can_move_to(&Stage::Backlog));
        assert!(Stage::Done.can_move_to(&Stage::Backlog));

        // Из backlog можно только в sketch
        assert!(Stage::Backlog.can_move_to(&Stage::Sketch));
        assert!(!Stage::Backlog.can_move_to(&Stage::Design));
        assert!(!Stage::Backlog.can_move_to(&Stage::Todo));
    }

    #[test]
    fn test_can_move_to_same_stage() {
        // Нельзя переместить в ту же стадию
        assert!(!Stage::Sketch.can_move_to(&Stage::Sketch));
        assert!(!Stage::Done.can_move_to(&Stage::Done));
    }

    #[test]
    fn test_can_move_to_backward() {
        // Запрет возврата назад (кроме backlog)
        assert!(!Stage::Design.can_move_to(&Stage::Sketch));
        assert!(!Stage::Todo.can_move_to(&Stage::Design));
        assert!(!Stage::InProgress.can_move_to(&Stage::Todo));
        assert!(!Stage::Done.can_move_to(&Stage::InProgress));
    }
}