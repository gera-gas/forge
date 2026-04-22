//! Модуль для генерации slug из названия задачи
//!
//! Slug — это URL-безопасное представление названия:
//! - Кирилица → транслит
//! - Спецсимволы → удаление
//! - Пробелы → `_`
//! - Обрезка до 50 символов
//! - Повторяющиеся `_` → один

/// Транслитерация кириллицы в латиницу
///
/// Поддерживает русскую кириллицу (а-я, А-Я) и ё/Ё
fn translit_char(c: char) -> Option<String> {
    match c {
        // Строчные
        'а' => Some("a".to_string()),
        'б' => Some("b".to_string()),
        'в' => Some("v".to_string()),
        'г' => Some("g".to_string()),
        'д' => Some("d".to_string()),
        'е' => Some("e".to_string()),
        'ё' => Some("yo".to_string()),
        'ж' => Some("zh".to_string()),
        'з' => Some("z".to_string()),
        'и' => Some("i".to_string()),
        'й' => Some("y".to_string()),
        'к' => Some("k".to_string()),
        'л' => Some("l".to_string()),
        'м' => Some("m".to_string()),
        'н' => Some("n".to_string()),
        'о' => Some("o".to_string()),
        'п' => Some("p".to_string()),
        'р' => Some("r".to_string()),
        'с' => Some("s".to_string()),
        'т' => Some("t".to_string()),
        'у' => Some("u".to_string()),
        'ф' => Some("f".to_string()),
        'х' => Some("kh".to_string()),
        'ц' => Some("ts".to_string()),
        'ч' => Some("ch".to_string()),
        'ш' => Some("sh".to_string()),
        'щ' => Some("shch".to_string()),
        'ъ' => Some("".to_string()),
        'ы' => Some("y".to_string()),
        'ь' => Some("".to_string()),
        'э' => Some("e".to_string()),
        'ю' => Some("yu".to_string()),
        'я' => Some("ya".to_string()),
        // Прописные
        'А' => Some("A".to_string()),
        'Б' => Some("B".to_string()),
        'В' => Some("V".to_string()),
        'Г' => Some("G".to_string()),
        'Д' => Some("D".to_string()),
        'Е' => Some("E".to_string()),
        'Ё' => Some("Yo".to_string()),
        'Ж' => Some("Zh".to_string()),
        'З' => Some("Z".to_string()),
        'И' => Some("I".to_string()),
        'Й' => Some("Y".to_string()),
        'К' => Some("K".to_string()),
        'Л' => Some("L".to_string()),
        'М' => Some("M".to_string()),
        'Н' => Some("N".to_string()),
        'О' => Some("O".to_string()),
        'П' => Some("P".to_string()),
        'Р' => Some("R".to_string()),
        'С' => Some("S".to_string()),
        'Т' => Some("T".to_string()),
        'У' => Some("U".to_string()),
        'Ф' => Some("F".to_string()),
        'Х' => Some("Kh".to_string()),
        'Ц' => Some("Ts".to_string()),
        'Ч' => Some("Ch".to_string()),
        'Ш' => Some("Sh".to_string()),
        'Щ' => Some("Shch".to_string()),
        'Ъ' => Some("".to_string()),
        'Ы' => Some("Y".to_string()),
        'Ь' => Some("".to_string()),
        'Э' => Some("E".to_string()),
        'Ю' => Some("Yu".to_string()),
        'Я' => Some("Ya".to_string()),
        _ => None,
    }
}

/// Генерирует slug из названия задачи
///
/// # Правила:
/// 1. Кирилица → транслит
/// 2. Пробелы и дефисы → `_`
/// 3. Оставить только `[a-z0-9_A]`
/// 4. Всё в lowercase
/// 5. Обрезать до 50 символов
/// 6. Повторяющиеся `_` → один
/// 7. Убрать `_` в начале и конце
///
/// # Примеры:
/// ```
/// use forge::kanban::slug::generate_slug;
/// assert_eq!(generate_slug("Kanban board CLI"), "kanban_board_cli");
/// assert_eq!(generate_slug("Задача на рефакторинг"), "zadacha_na_refaktoring");
/// ```
pub fn generate_slug(title: &str) -> String {
    let mut result = String::new();

    for c in title.chars() {
        match c {
            // Пробелы и дефисы → _
            ' ' | '-' | '_' => {
                result.push('_');
            }
            // Кирилица → транслит (всегда в lowercase)
            c if ('а'..='я').contains(&c)
                || ('А'..='Я').contains(&c)
                || c == 'ё'
                || c == 'Ё' =>
            {
                if let Some(translit) = translit_char(c) {
                    // Транслит может содержать uppercase для прописных букв — lowercase
                    for ch in translit.chars() {
                        result.push(ch.to_ascii_lowercase());
                    }
                }
            }
            // Латиница и цифры — оставляем как есть
            c if c.is_ascii_alphanumeric() => {
                result.push(c.to_ascii_lowercase());
            }
            // Остальные символы — удаляем
            _ => {}
        }
    }

    // Заменяем повторяющиеся `_` на один
    let mut cleaned = String::new();
    let mut prev_underscore = false;
    for c in result.chars() {
        if c == '_' {
            if !prev_underscore {
                cleaned.push('_');
                prev_underscore = true;
            }
        } else {
            cleaned.push(c);
            prev_underscore = false;
        }
    }

    // Убираем `_` в начале и конце
    let trimmed = cleaned.trim_matches('_');

    // Обрезаем до 50 символов, не разрывая транслитерированный символ
    // (но так как мы уже сделали транслит, можно просто обрезать)
    let mut slug = String::new();
    for c in trimmed.chars().take(50) {
        slug.push(c);
    }

    // После обрезки может остаться `_` на конце — убираем
    slug.trim_end_matches('_').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slug_basic() {
        assert_eq!(generate_slug("Kanban board CLI"), "kanban_board_cli");
    }

    #[test]
    fn test_slug_cyrillic() {
        assert_eq!(generate_slug("Задача на рефакторинг"), "zadacha_na_refaktoring");
    }

    #[test]
    fn test_slug_special_chars() {
        assert_eq!(generate_slug("Task #1: Fix bug!"), "task_1_fix_bug");
    }

    #[test]
    fn test_slug_spaces_to_underscore() {
        assert_eq!(generate_slug("hello world"), "hello_world");
    }

    #[test]
    fn test_slug_multiple_underscores() {
        assert_eq!(generate_slug("hello   world"), "hello_world");
        assert_eq!(generate_slug("hello - world"), "hello_world");
    }

    #[test]
    fn test_slug_truncate_50() {
        let long_title = "a".repeat(100);
        let slug = generate_slug(&long_title);
        assert_eq!(slug.len(), 50);
    }

    #[test]
    fn test_slug_trim_underscores() {
        assert_eq!(generate_slug("  hello  "), "hello");
        assert_eq!(generate_slug("---hello---"), "hello");
    }

    #[test]
    fn test_slug_mixed() {
        assert_eq!(
            generate_slug("Реализация forge task new"),
            "realizatsiya_forge_task_new"
        );
    }

    #[test]
    fn test_slug_empty() {
        assert_eq!(generate_slug(""), "");
        assert_eq!(generate_slug("!!!"), "");
    }

    #[test]
    fn test_slug_numbers() {
        assert_eq!(generate_slug("Task 123"), "task_123");
    }
}