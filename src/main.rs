// Подключаем библиотеку clap для парсинга аргументов командной строки
use clap::Parser;

// Подключаем наши модули
mod cli;       // Модуль с логикой команд
mod templates; // Модуль с шаблонами файлов
mod kanban;    // Модуль для работы с kanban-доской

// Подключаем типы из kanban-модуля
use kanban::frontmatter::{Priority, TaskFrontmatter, TaskType};
use kanban::slug;
use kanban::stage::Stage;

// ============================================================================
// ГЛАВНАЯ СТРУКТУРА CLI
// ============================================================================

// #[derive(Parser)] — это макрос (как декоратор в Python или сложный #define в C)
// Он автоматически генерирует код для парсинга аргументов командной строки
#[derive(Parser)]
// #[command(...)] — это атрибуты (как аргументы декоратора в Python)
#[command(name = "forge")]
#[command(about = "CLI утилита для управления контекстом ИИ-агентов")]
struct Cli {
    // В Rust командная строка представляется как вложенные структуры данных
    // subcommand означает, что после "forge" идёт подкоманда: init, wrap, task и т.д.
    #[command(subcommand)]
    command: Commands,
}

// ============================================================================
// ПЕРЕЧИСЛЕНИЯ (ENUM) С ДАННЫМИ
// ============================================================================

// В C/C++: enum — это просто целые числа без данных
// В Rust: enum может хранить данные в каждом варианте (как tagged union в C)
// Это как если бы в C был union + enum вместе, но type-safe
#[derive(clap::Subcommand)]
enum Commands {
    /// Инициализация структуры forge-src/ в проекте
    /// 
    /// Аналог `git init` — создаёт структуру каталогов и файлы-шаблоны
    Init {
        /// Путь к проекту (по умолчанию: текущая директория)
        #[arg(short, long)]
        path: Option<String>,
    },
    
    /// Анализ проекта и генерация описания через LLM
    /// 
    /// Сканирует код, отправляет в API OpenAI/Claude и генерирует
    /// concept.md, struct.md, tech_stack.md
    Wrap {
        /// Исходная директория для анализа
        #[arg(short, long, default_value = ".")]
        source: String,
        
        /// Модель LLM для анализа
        #[arg(short, long, default_value = "gpt-4o")]
        model: String,
    },
    
    /// Управление задачами (Kanban)
    /// 
    /// Подкоманды: new, move, list, show
    Task {
        // Вложенная подкоманда (как в git: git commit, git push)
        #[command(subcommand)]
        command: TaskCommands,
    },
    
    /// Генерация шаблонов проектов
    /// 
    /// Создаёт CMakeLists.txt, Bazel BUILD и т.д. через LLM
    Template {
        #[command(subcommand)]
        command: TemplateCommands,
    },
    
    /// NLP-интерфейс (Natural Language Processing)
    /// 
    /// Принимает запрос на естественном языке и преобразует в команду
    /// Пример: forge ask "создай задачу на рефакторинг"
    Ask {
        /// Запрос на естественном языке
        query: String,
    },
}

// ============================================================================
// ТИПЫ ДЛЯ KANBAN
// ============================================================================

/// Стадии Kanban-доски
#[derive(Debug, Clone, clap::ValueEnum)]
enum StageArg {
    Backlog,
    Sketch,
    Design,
    Todo,
    InProgress,
    Done,
}

/// Типы задач
#[derive(Debug, Clone, clap::ValueEnum)]
enum TaskTypeArg {
    Feature,
    Fix,
    Spike,
    Chore,
}

/// Приоритеты задач
#[derive(Debug, Clone, clap::ValueEnum)]
enum PriorityArg {
    Low,
    Medium,
    High,
    Critical,
}

// ============================================================================
// ПОДКОМАНДЫ ДЛЯ TASK
// ============================================================================

#[derive(clap::Subcommand)]
enum TaskCommands {
    /// Создать новую задачу в kanban-доске
    ///
    /// Пример: forge task new "Kanban board CLI"
    /// Пример: forge task new "Bug fix" --stage todo --type fix --priority high --design 001
    New {
        /// Название задачи
        title: String,

        /// Стадия (по умолчанию: sketch)
        #[arg(short, long, value_enum, default_value = "sketch")]
        stage: StageArg,

        /// Тип задачи
        #[arg(short = 't', long, value_enum, default_value = "feature")]
        r#type: TaskTypeArg,

        /// Приоритет
        #[arg(short, long, value_enum, default_value = "medium")]
        priority: PriorityArg,

        /// ID дизайна, из которого порождена задача
        #[arg(long)]
        design: Option<String>,

        /// Теги (через запятую)
        #[arg(long, default_value = "")]
        tags: String,
    },

    /// Переместить задачу между стадиями kanban-доски
    ///
    /// Пример: forge task move 001 design
    /// Пример: forge task move 003 done
    Move {
        /// ID задачи
        id: String,

        /// Целевая стадия
        stage: StageArg,
    },

    /// Список задач в kanban-доске
    ///
    /// Пример: forge task list
    /// Пример: forge task list --stage todo --format json --design 001
    List {
        /// Фильтр по стадии (все стадии, если не указано)
        #[arg(short, long, value_enum)]
        stage: Option<StageArg>,

        /// Формат вывода: table (по умолчанию) или json
        #[arg(long, default_value = "table")]
        format: String,

        /// Фильтр по типу задачи
        #[arg(short = 't', long, value_enum)]
        r#type: Option<TaskTypeArg>,

        /// Фильтр по ID дизайна
        #[arg(long)]
        design: Option<String>,
    },

    /// Показать детали задачи
    ///
    /// Пример: forge task show 001
    Show {
        /// ID задачи
        id: String,
    },
}

// ============================================================================
// ПОДКОМАНДЫ ДЛЯ TEMPLATE
// ============================================================================

#[derive(clap::Subcommand)]
enum TemplateCommands {
    /// Создать новый шаблон проекта
    /// 
    /// Пример: forge template new --lang c --build cmake
    New {
        /// Язык программирования (c, cpp, rust)
        #[arg(short, long)]
        lang: String,
        
        /// Система сборки (cmake, bazel, make)
        #[arg(short, long)]
        build: String,
    },
}

// ============================================================================
// ГЛАВНАЯ ФУНКЦИЯ
// ============================================================================

// #[tokio::main] — это макрос для асинхронности (как asyncio.run() в Python)
// Rust требует явно указать async runtime, tokio — самый популярный
// async — потому что будем делать HTTP-запросы к LLM API
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // anyhow::Result<()> — это обёртка для ошибок
    // Аналог try/except в Python, но компилятор заставляет обрабатывать ошибки
    // () — это unit type, как void в C/C++
    
    // Парсим аргументы командной строки
    // Если аргументы невалидны, clap автоматически выведет help и завершит программу
    let cli = Cli::parse();

    // match — это как switch в C, но гораздо мощнее:
    // 1. Обязательно покрыть все варианты (compile-time check)
    // 2. Можно извлекать данные из enum
    // 3. Можно делать pattern matching
    match cli.command {
        // Когда пользователь ввёл: forge init [--path /some/path]
        Commands::Init { path } => {
            // Вызываем функцию из модуля cli::init
            // ? оператор означает: если результат Err, вернуть ошибку из main
            // Это аналог try/except но более явный
            cli::init::execute(path)?;
        }
        
        // Когда пользователь ввёл: forge wrap --source ./src --model gpt-4o
        Commands::Wrap { source, model } => {
            println!("Анализ проекта: {} (модель: {})", source, model);
            // TODO: Реализовать логику wrap
            // 1. Сканировать файлы в source
            // 2. Собрать контекст
            // 3. Отправить в LLM API
            // 4. Сохранить результаты в forge-src/
        }
        
        // Когда пользователь ввёл: forge task <subcommand>
        Commands::Task { command } => {
            // Вложенный match для подкоманд kanban
            match command {
                TaskCommands::New { title, stage, r#type, priority, design, tags } => {
                    // Конвертируем CLI-enum в kanban-enum
                    let kanban_stage = match stage {
                        StageArg::Backlog => Stage::Backlog,
                        StageArg::Sketch => Stage::Sketch,
                        StageArg::Design => Stage::Design,
                        StageArg::Todo => Stage::Todo,
                        StageArg::InProgress => Stage::InProgress,
                        StageArg::Done => Stage::Done,
                    };
                    let kanban_type = match r#type {
                        TaskTypeArg::Feature => TaskType::Feature,
                        TaskTypeArg::Fix => TaskType::Fix,
                        TaskTypeArg::Spike => TaskType::Spike,
                        TaskTypeArg::Chore => TaskType::Chore,
                    };
                    let kanban_priority = match priority {
                        PriorityArg::Low => Priority::Low,
                        PriorityArg::Medium => Priority::Medium,
                        PriorityArg::High => Priority::High,
                        PriorityArg::Critical => Priority::Critical,
                    };
                    let tags_vec: Vec<String> = if tags.is_empty() {
                        Vec::new()
                    } else {
                        tags.split(',').map(|s| s.trim().to_string()).collect()
                    };

                    // Ищем корень kanban-доски
                    let kanban_root = match kanban::find_kanban_root() {
                        Some(root) => root,
                        None => {
                            eprintln!("Ошибка: не найдена директория forge-src/kanban/");
                            eprintln!("Запустите 'forge init' для создания структуры.");
                            std::process::exit(1);
                        }
                    };

                    // Определяем следующий доступный ID
                    let next_id = kanban::next_available_id(&kanban_root);

                    // Генерируем slug из названия
                    let task_slug = slug::generate_slug(&title);

                    // Создаём frontmatter
                    let fm = TaskFrontmatter {
                        id: next_id.clone(),
                        title: title.clone(),
                        task_type: kanban_type,
                        priority: kanban_priority,
                        design: design,
                        blocked_by: None,
                        tags: tags_vec,
                        completed_at: None,
                    };

                    // Создаём файл задачи
                    match kanban::create_task_file(&kanban_root, &kanban_stage, &fm, &task_slug) {
                        Ok(path) => {
                            println!("Created: {}", path.display());
                            println!("Git:     task({}): add {} \"{}\"", fm.id, kanban_stage, fm.title);
                        }
                        Err(e) => {
                            eprintln!("Ошибка при создании задачи: {}", e);
                            std::process::exit(1);
                        }
                    }
                }

                TaskCommands::Move { id, stage } => {
                    let target_stage = match stage {
                        StageArg::Backlog => Stage::Backlog,
                        StageArg::Sketch => Stage::Sketch,
                        StageArg::Design => Stage::Design,
                        StageArg::Todo => Stage::Todo,
                        StageArg::InProgress => Stage::InProgress,
                        StageArg::Done => Stage::Done,
                    };

                    // Ищем корень kanban-доски
                    let kanban_root = match kanban::find_kanban_root() {
                        Some(root) => root,
                        None => {
                            eprintln!("Ошибка: не найдена директория forge-src/kanban/");
                            eprintln!("Запустите 'forge init' для создания структуры.");
                            std::process::exit(1);
                        }
                    };

                    // Находим задачу по ID
                    let task = match kanban::find_task_by_id(&kanban_root, &id) {
                        Some(t) => t,
                        None => {
                            eprintln!("Ошибка: задача с ID '{}' не найдена.", id);
                            std::process::exit(1);
                        }
                    };

                    // Проверяем правила перехода
                    if !task.stage.can_move_to(&target_stage) {
                        eprintln!(
                            "Ошибка: нельзя переместить задачу из '{}' в '{}'.",
                            task.stage, target_stage
                        );
                        std::process::exit(1);
                    }

                    let source_stage = task.stage;

                    // При перемещении в done — записываем completed_at
                    if target_stage == Stage::Done {
                        if let Err(e) = kanban::set_completed_at(&task) {
                            eprintln!("Предупреждение: не удалось записать completed_at: {}", e);
                        }
                    }

                    // Перемещаем задачу
                    match kanban::move_task(&task, &target_stage, &kanban_root) {
                        Ok(new_path) => {
                            println!("Moved:   task({}): move {} -> {}", id, source_stage, target_stage);
                            println!("From:    {}", task.path.display());
                            println!("To:      {}", new_path.display());
                            println!("Git:     task({}): move {} -> {}", id, source_stage, target_stage);

                            // При переходе design -> todo предлагаем создать git-ветку
                            if source_stage == Stage::Design && target_stage == Stage::Todo {
                                let slug = slug::generate_slug(task.title());
                                println!();
                                println!("💡 Рекомендуется создать git-ветку:");
                                let task_type_prefix = match task.frontmatter.task_type {
                                    TaskType::Feature => "feature",
                                    TaskType::Fix => "fix",
                                    TaskType::Spike => "spike",
                                    TaskType::Chore => "chore",
                                };
                                println!("   git checkout -b {}/{}_{}", task_type_prefix, id, slug);
                            }
                        }
                        Err(e) => {
                            eprintln!("Ошибка при перемещении задачи: {}", e);
                            std::process::exit(1);
                        }
                    }
                }

                TaskCommands::List { stage, format, r#type, design } => {
                    // Ищем корень kanban-доски
                    let kanban_root = match kanban::find_kanban_root() {
                        Some(root) => root,
                        None => {
                            eprintln!("Ошибка: не найдена директория forge-src/kanban/");
                            eprintln!("Запустите 'forge init' для создания структуры.");
                            std::process::exit(1);
                        }
                    };

                    // Сканируем все задачи
                    let mut tasks = kanban::scan_all_tasks(&kanban_root);

                    // Фильтруем по стадии
                    if let Some(ref stage_arg) = stage {
                        let filter_stage = match stage_arg {
                            StageArg::Backlog => Stage::Backlog,
                            StageArg::Sketch => Stage::Sketch,
                            StageArg::Design => Stage::Design,
                            StageArg::Todo => Stage::Todo,
                            StageArg::InProgress => Stage::InProgress,
                            StageArg::Done => Stage::Done,
                        };
                        tasks.retain(|t| t.stage == filter_stage);
                    }

                    // Фильтруем по типу
                    if let Some(ref type_arg) = r#type {
                        let filter_type = match type_arg {
                            TaskTypeArg::Feature => TaskType::Feature,
                            TaskTypeArg::Fix => TaskType::Fix,
                            TaskTypeArg::Spike => TaskType::Spike,
                            TaskTypeArg::Chore => TaskType::Chore,
                        };
                        tasks.retain(|t| t.frontmatter.task_type == filter_type);
                    }

                    // Фильтруем по дизайну
                    if let Some(ref design_id) = design {
                        tasks.retain(|t| t.frontmatter.design.as_ref() == Some(design_id));
                    }

                    // Сортируем по ID
                    tasks.sort_by_key(|t| t.frontmatter.id.clone());

                    if tasks.is_empty() {
                        println!("Задач не найдено.");
                        return Ok(());
                    }

                    // Выводим в нужном формате
                    if format == "json" {
                        // JSON формат
                        println!("[");
                        for (i, task) in tasks.iter().enumerate() {
                            let comma = if i < tasks.len() - 1 { "," } else { "" };
                            let design_json = task.frontmatter.design.as_ref()
                                .map(|s| format!("\"{}\"", s))
                                .unwrap_or("null".to_string());
                            let completed_json = task.frontmatter.completed_at.as_ref()
                                .map(|s| format!("\"{}\"", s))
                                .unwrap_or("null".to_string());
                            println!(
                                "  {{\"id\": \"{}\", \"title\": \"{}\", \"stage\": \"{}\", \"type\": \"{}\", \"priority\": \"{}\", \"design\": {}, \"blocked_by\": {}, \"completed_at\": {}, \"path\": \"{}\"}}{}",
                                task.frontmatter.id,
                                task.frontmatter.title,
                                task.stage,
                                task.frontmatter.task_type,
                                task.frontmatter.priority,
                                design_json,
                                task.frontmatter.blocked_by.as_ref().map(|s| format!("\"{}\"", s)).unwrap_or("null".to_string()),
                                completed_json,
                                task.path.display(),
                                comma
                            );
                        }
                        println!("]");
                    } else {
                        // Table формат
                        println!("{:<5} {:<30} {:<12} {:<10} {:<10}", "ID", "TITLE", "STAGE", "TYPE", "PRIORITY");
                        println!("{}", "-".repeat(67));
                        for task in &tasks {
                            let title = if task.frontmatter.title.len() > 28 {
                                format!("{}...", &task.frontmatter.title[..25])
                            } else {
                                task.frontmatter.title.clone()
                            };
                            println!(
                                "{:<5} {:<30} {:<12} {:<10} {:<10}",
                                task.frontmatter.id,
                                title,
                                task.stage,
                                task.frontmatter.task_type,
                                task.frontmatter.priority
                            );
                        }
                    }
                }

                TaskCommands::Show { id } => {
                    // Ищем корень kanban-доски
                    let kanban_root = match kanban::find_kanban_root() {
                        Some(root) => root,
                        None => {
                            eprintln!("Ошибка: не найдена директория forge-src/kanban/");
                            eprintln!("Запустите 'forge init' для создания структуры.");
                            std::process::exit(1);
                        }
                    };

                    // Находим задачу по ID
                    let task = match kanban::find_task_by_id(&kanban_root, &id) {
                        Some(t) => t,
                        None => {
                            eprintln!("Ошибка: задача с ID '{}' не найдена.", id);
                            std::process::exit(1);
                        }
                    };

                    // Выводим информацию о задаче
                    println!("ID:       {}", task.frontmatter.id);
                    println!("Title:    {}", task.frontmatter.title);
                    println!("Stage:    {}", task.stage);
                    println!("Type:     {}", task.frontmatter.task_type);
                    println!("Priority: {}", task.frontmatter.priority);
                    if let Some(ref design) = task.frontmatter.design {
                        println!("Design:   {}", design);
                    }
                    if let Some(ref blocked) = task.frontmatter.blocked_by {
                        println!("Blocked:  {}", blocked);
                    }
                    if !task.frontmatter.tags.is_empty() {
                        println!("Tags:     {}", task.frontmatter.tags.join(", "));
                    }
                    if let Some(ref completed) = task.frontmatter.completed_at {
                        println!("Done:     {}", completed);
                    }
                    println!("Path:     {}", task.path.display());
                    println!("Folder:   {}", if task.is_folder { "да" } else { "нет" });
                }
            }
        }
        
        // Когда пользователь ввёл: forge template new --lang c --build cmake
        Commands::Template { command } => {
            match command {
                TemplateCommands::New { lang, build } => {
                    println!("Генерация шаблона: {} ({})", lang, build);
                    // TODO: Реализовать template new
                    // 1. Прочитать project.forge.toml (если есть)
                    // 2. Сформировать промпт для LLM
                    // 3. Отправить в API
                    // 4. Сохранить CMakeLists.txt / BUILD и т.д.
                }
            }
        }
        
        // Когда пользователь ввёл: forge ask "создай задачу XXX"
        Commands::Ask { query } => {
            println!("NLP-запрос: {}", query);
            // TODO: Реализовать ask
            // 1. Отправить query в LLM с промптом "преобразуй в команду"
            // 2. LLM вернёт JSON: {"command": "task", "action": "add", "args": {...}}
            // 3. Выполнить эту команду
        }
    }

    // Ok(()) — это как return 0 в C, означает "всё хорошо"
    // В Rust функции возвращают последнее выражение без return
    Ok(())
}
