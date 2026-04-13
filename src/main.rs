// Подключаем библиотеку clap для парсинга аргументов командной строки
// Это аналог argparse в Python или getopt в C
use clap::Parser;

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
    /// Инициализация структуры .agent/ в проекте
    /// 
    /// Аналог `git init` — создаёт структуру каталогов и файлы-шаблоны
    Init {
        /// Путь к проекту (по умолчанию: текущая директория)
        /// 
        /// Option<String> — это как `nullable` / `None` в Python
        /// None означает "значения нет", Some(value) — "значение есть"
        /// В C++ это аналог std::optional<std::string>
        #[arg(short, long)]
        path: Option<String>,
    },
    
    /// Анализ проекта и генерация описания через LLM
    /// 
    /// Сканирует код, отправляет в API OpenAI/Claude и генерирует
    /// concept.md, struct.md, tech_stack.md
    Wrap {
        /// Исходная директория для анализа
        /// 
        /// String — это владеющая строка (как std::string в C++)
        /// default_value = "." означает, что если не задано, будет "."
        #[arg(short, long, default_value = ".")]
        source: String,
        
        /// Модель LLM для анализа
        #[arg(short, long, default_value = "gpt-4o")]
        model: String,
    },
    
    /// Управление задачами (Kanban)
    /// 
    /// Подкоманды: add, list, start, done
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
// ПОДКОМАНДЫ ДЛЯ TASK
// ============================================================================

#[derive(clap::Subcommand)]
enum TaskCommands {
    /// Добавить задачу в tasks/todo/
    /// 
    /// Пример: forge task add "Реализовать тесты"
    Add {
        /// Описание задачи
        description: String,
    },
    
    /// Список всех задач (todo, in_progress, done)
    /// 
    /// Пример: forge task list
    /// Пример: forge task list --format json
    List {
        /// Формат вывода: table (по умолчанию) или json
        #[arg(long, default_value = "table")]
        format: String,
    },
    
    /// Начать задачу (переместить из todo в in_progress)
    /// 
    /// Пример: forge task start 001
    Start {
        /// ID задачи (имя файла без .md, например 001)
        id: String,
    },
    
    /// Завершить задачу (переместить из in_progress в done)
    /// 
    /// Пример: forge task done 001
    Done {
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
            println!("Инициализация структуры .agent/...");
            // TODO: Реализовать логику init
            // Здесь будем создавать каталоги и файлы
            // path — это Option<String>, можно проверить:
            // if let Some(p) = path { ... } else { ... }
        }
        
        // Когда пользователь ввёл: forge wrap --source ./src --model gpt-4o
        Commands::Wrap { source, model } => {
            println!("Анализ проекта: {} (модель: {})", source, model);
            // TODO: Реализовать логику wrap
            // 1. Сканировать файлы в source
            // 2. Собрать контекст
            // 3. Отправить в LLM API
            // 4. Сохранить результаты в .agent/
        }
        
        // Когда пользователь ввёл: forge task <subcommand>
        Commands::Task { command } => {
            // Вложенный match для подкоманд
            match command {
                TaskCommands::Add { description } => {
                    println!("Добавление задачи: {}", description);
                    // TODO: Реализовать add
                    // 1. Найти следующий номер (например, 006)
                    // 2. Создать файл tasks/todo/006_<sanitized_description>.md
                }
                
                TaskCommands::List { format } => {
                    println!("Список задач (формат: {})", format);
                    // TODO: Реализовать list
                    // 1. Прочитать tasks/todo/, tasks/in_progress/, tasks/done/
                    // 2. Вывести в таблице или JSON
                }
                
                TaskCommands::Start { id } => {
                    println!("Начало задачи: {}", id);
                    // TODO: Реализовать start
                    // 1. Найти файл в tasks/todo/ с префиксом id
                    // 2. Переместить в tasks/in_progress/
                    // 3. Обновить поле Status в файле
                }
                
                TaskCommands::Done { id } => {
                    println!("Завершение задачи: {}", id);
                    // TODO: Реализовать done
                    // 1. Найти файл в tasks/in_progress/ с префиксом id
                    // 2. Переместить в tasks/done/
                    // 3. Обновить поле Status и дату завершения
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
