// Модуль CLI — содержит логику команд
// 
// В Rust модули организуются через файловую систему:
// cli/mod.rs — это "главный файл" модуля cli
// cli/init.rs — это подмодуль

// Объявляем подмодули (pub = public, доступны извне)
pub mod init;

// В будущем добавим:
// pub mod wrap;
// pub mod task;
// pub mod template;
// pub mod ask;
