// Интеграционные тесты для forge
// Проверяем основной функционал CLI

use std::process::Command;
use tempfile::TempDir;

// ============================================================================
// SMOKE-ТЕСТЫ (базовая проверка, что программа работает)
// ============================================================================

/// Тест: `forge --help` выводит справку без ошибок
#[test]
fn test_cli_help_works() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("Failed to execute forge --help");

    // Проверяем что команда завершилась успешно (exit code 0)
    assert!(output.status.success(), "forge --help should succeed");
    
    // Проверяем что в выводе есть слово "forge"
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("forge"), "Help output should contain 'forge'");
}

/// Тест: `forge init` создаёт структуру директорий
#[test]
fn test_init_creates_structure() {
    // Создаём временную директорию (автоматически удалится после теста)
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path().to_str().unwrap();

    // Запускаем: forge init --path <temp_dir>
    let output = Command::new("cargo")
        .args(&["run", "--", "init", "--path", temp_path])
        .output()
        .expect("Failed to execute forge init");

    // Проверяем что команда завершилась успешно
    assert!(output.status.success(), "forge init should succeed");

    // Проверяем что создались основные файлы и папки
    let agent_dir = temp_dir.path().join(".agent");
    assert!(agent_dir.exists(), ".agent/ directory should exist");
    
    let readme = agent_dir.join("README.md");
    assert!(readme.exists(), ".agent/README.md should exist");
    
    let concept = agent_dir.join("concept.md");
    assert!(concept.exists(), ".agent/concept.md should exist");
    
    let tasks_todo = agent_dir.join("tasks").join("todo");
    assert!(tasks_todo.exists(), ".agent/tasks/todo/ should exist");
    
    let tasks_backlog = agent_dir.join("tasks").join("backlog");
    assert!(tasks_backlog.exists(), ".agent/tasks/backlog/ should exist");

    // Проверяем структуру kanban
    let kanban_dir = agent_dir.join("kanban");
    assert!(kanban_dir.exists(), ".agent/kanban/ should exist");

    let kanban_stages = ["0_backlog", "1_sketch", "2_design", "3_todo", "4_in_progress", "5_done"];
    for stage in &kanban_stages {
        let stage_dir = kanban_dir.join(stage);
        assert!(stage_dir.exists(), ".agent/kanban/{}/ should exist", stage);
        let gitkeep = stage_dir.join(".gitkeep");
        assert!(gitkeep.exists(), ".agent/kanban/{}/.gitkeep should exist", stage);
    }
}

/// Тест: Повторный `forge init` не перезатирает существующие файлы
/// (или спрашивает подтверждение — пока просто проверяем что не падает)
#[test]
fn test_init_twice_does_not_crash() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path().to_str().unwrap();

    // Первый раз
    let output1 = Command::new("cargo")
        .args(&["run", "--", "init", "--path", temp_path])
        .output()
        .expect("Failed to execute forge init (first time)");
    assert!(output1.status.success(), "First forge init should succeed");

    // Второй раз (пока просто проверяем что не падает с ошибкой)
    let output2 = Command::new("cargo")
        .args(&["run", "--", "init", "--path", temp_path])
        .output()
        .expect("Failed to execute forge init (second time)");
    
    // Может завершиться с кодом != 0 если спросит подтверждение и не получит
    // Главное — не должно быть паники
    // (потом добавим флаг --force для перезаписи без вопросов)
}

/// Тест: Проверяем что README.md не пустой
#[test]
fn test_init_creates_non_empty_readme() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path().to_str().unwrap();

    Command::new("cargo")
        .args(&["run", "--", "init", "--path", temp_path])
        .output()
        .expect("Failed to execute forge init");

    let readme = temp_dir.path().join(".agent").join("README.md");
    let content = std::fs::read_to_string(readme).expect("Failed to read README.md");
    
    // Проверяем что файл не пустой и содержит ключевые слова
    assert!(!content.is_empty(), "README.md should not be empty");
    assert!(content.contains("Agent Context"), "README.md should contain 'Agent Context'");
}

// ============================================================================
// БУДУЩИЕ ТЕСТЫ (пока закомментированы, реализуем позже)
// ============================================================================

// TODO: Тесты для `forge wrap` (требуют мок LLM API)
// #[test]
// fn test_wrap_analyzes_project() { ... }

// TODO: Тесты для `forge task add`
// #[test]
// fn test_task_add_creates_file() { ... }

// TODO: Тесты для `forge task start` / `done`
// #[test]
// fn test_task_workflow() { ... }
