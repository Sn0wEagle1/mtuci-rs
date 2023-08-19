use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
enum Priority {
    Low,
    Medium,
    High
}

impl Priority {
    fn to_string(&self) -> String {
        match self {
            Priority::Low => "Низкий".to_owned(),
            Priority::Medium => "Средний".to_owned(),
            Priority::High => "Высокий".to_owned()
        }
    }

    fn order(&self) -> u32 {
        match self {
            Priority::Low => 2,
            Priority::Medium => 1,
            Priority::High => 0,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Task {
    name: String,
    description: String,
    priority: Priority,
    add_time: DateTime<Local>,
    completed: bool,
}

impl Task {
    fn new(name: String, description: String, priority: Priority) -> Self {
        Self {name, description, priority, add_time: Local::now(), completed: false}
    }

    fn new_from_console() -> Self {
        let name = ConsoleTask::input("Введите имя задачи: ").unwrap();
        let description = ConsoleTask::input("Введите описание задачи: ").unwrap();
        let priority = match ConsoleTask::input("Введите индекс приоритета задачи (1 - высокий, 2 - средний, 3 - низкий): ").unwrap().as_str() {
                "3" => Priority::Low,
                "2" => Priority::Medium,
                "1" => Priority::High,
                _ => {
                    println!("Неправильный приоритет, изменено на Низкий");
                    Priority::Low
                }
            }; 

            Self::new(name, description, priority)
        }

    fn print_task(&self) {
        let status = if self.completed { "Выполнено" } else { "Не выполнено" };

        println!(
            "Название: {} | Приоритет: {} | Статус: {} | Дата и время: {}\nОписание: \"{}\"",
            self.name,
            self.priority.to_string(),
            status,
            self.add_time.format("%d-%m-%Y %H:%M:%S"),
            self.description
        );
    }

    fn mark_as_completed(&mut self) {
        self.completed = true;
    }
}

struct TasksManager {
    tasks: Vec<Task>
}

impl TasksManager {
    fn new() -> Self {
        Self { tasks: vec![] }
    }


    fn print_tasks(&mut self) {
        if self.tasks.is_empty() {
            println!("Нет задач.");
            return;
        }
        
        let mut tasks_by_priority: Vec<Vec<&Task>> = vec![vec![], vec![], vec![]];
    
        for task in &self.tasks {
            let priority_order = task.priority.order() as usize;
            tasks_by_priority[priority_order].push(task);
        }
    
        for priority_tasks in tasks_by_priority.iter_mut() {
            priority_tasks.sort_by(|a, b| a.add_time.cmp(&b.add_time));
        }
    
        let mut task_number = 1; 
    
        for priority_tasks in tasks_by_priority.iter() {
            for task in priority_tasks {
                println!("{}. ", task_number);
                task.print_task(); 
                println!(); 
                task_number += 1; 
            }
        }
    }
    

    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    fn find_task(&self, name: &str) -> Option<usize> {
        self.tasks.iter().position(|task| task.name == name)
    }

    fn remove_task(&mut self, name: &str) -> Result<String, String> {
        if let Some(index) = self.find_task(name) {
            self.tasks.remove(index);
            Ok(format!("Задача \"{}\" удалена успешно", name))
        } else {
            Err(format!("Задача с именем \"{}\" не существует", name))
        }
    }

    fn edit_task(&mut self, name: &str, updated_task: Task) -> Result<String, String> {
        if let Some(index) = self.find_task(name) {
            match self.tasks.get_mut(index) {
                None => Err("Ошибка при доступе к задаче".to_owned()),
                Some(task) => {
                    task.name = updated_task.name;
                    task.description = updated_task.description;
                    task.priority = updated_task.priority;
                    Ok(format!("Задача \"{}\" обновлена успешно", name))
                }
            }
        } else {
            Err(format!("Задача с именем \"{}\" не существует", name))
        }
    }

    fn store_to_file(&self, filename: &str) -> Result<String, String> {
        if !Path::new(filename).exists() {
            let file = match File::create(filename) {
                Ok(file) => file,
                Err(err) => return Err(format!("Ошибка при создании файла: {}", err))
            };

            match serde_json::to_writer(&file, &self.tasks) {
                Ok(_) => Ok("Данные успешно сохранены".to_owned()),
                Err(err) => Err(format!("Ошибка при сохранении данных: {}", err))
            }
        } else {
            Err("Файл \"{filename}\" уже существует".to_owned())
        }
    }

    fn read_from_file(&mut self, filename: &str) -> Result<String, String> {
        if Path::new(filename).exists() {
            let file = match File::open(filename) {
                Ok(file) => file,
                Err(err) => return Err(format!("Ошибка при создании файла {}", err))
            };

            let reader = BufReader::new(file);

            self.tasks = match serde_json::from_reader(reader) {
                Ok(data) => data,
                Err(err) => {
                    return Err(format!("Ошибка при чтении файла: {}", err));
                }
            };

            Ok("Данные успешно считаны".to_owned())
        } else {
            Err(format!("Файл \"{}\" не существует", filename))
        }
    }
}

struct ConsoleTask {
    tasks_manager: TasksManager,
    menu_options: Vec<String>
}

impl ConsoleTask {
    fn new() -> Self {
        Self {
            tasks_manager: TasksManager::new(),
            menu_options: vec![
                "Добавить задачу".to_owned(),
                "Найти задачу".to_owned(),
                "Изменить задачу".to_owned(),
                "Удалить задачу".to_owned(),
                "Вывести задачи".to_owned(),
                "Сохранить задачи в файл".to_owned(),
                "Считать задачи из файла".to_owned(),
                "Отметить задачу как выполненную".to_owned()
            ]
        }
    }
    
    fn print_menu(&self) {
        for (index, menu_option) in self.menu_options.iter().enumerate() {
            println!("{}. {}", index + 1, menu_option);
        }
    }

    fn input(query: &str) -> std::io::Result<String> {
        print!("{}", query);
        std::io::stdout().flush()?;

        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer)?;
        Ok(buffer.trim().to_owned())
    }

    fn process_command(&mut self) {
        match Self::input("\nВведите индекс команды: ") {
            Ok(command) => {
                match command.as_str() {
                    "1" => {
                        self.tasks_manager.add_task(Task::new_from_console());
                    }

                    "2" => {
                        let name = match Self::input("Введите имя задачи для поиска: ") {
                            Ok(name) => name,
                            Err(err) => {
                                println!("Ошибка при получении ввода пользователя: {}", err);
                                return;
                            }
                        };

                        match self.tasks_manager.find_task(name.as_str()) {
                            None => println!("Задача с именем \"{}\" не существует", name),
                            Some(index) => {
                                println!("Задача найдена!");
                                self.tasks_manager.tasks.get(index).unwrap().print_task();
                            }
                        }
                    }

                    "3" => {
                        let name = match Self::input("Введите имя задачи для изменения: ") {
                            Ok(name) => name,
                            Err(err) => {
                                println!("Ошибка при получении ввода пользователя: {}", err);
                                return;
                            }
                        };

                        match self.tasks_manager.edit_task(name.as_str(), Task::new_from_console()) {
                            Ok(msg) => println!("{}", msg),
                            Err(msg) => println!("{}", msg),
                        }
                    }

                    "4" => {
                        let name = match Self::input("Введите имя задачи для удаления: ") {
                            Ok(name) => name,
                            Err(err) => {
                                println!("Ошибка при получении ввода пользователя: {}", err);
                                return;
                            }
                        };

                        match self.tasks_manager.remove_task(name.as_str()) {
                            Ok(msg) => println!("{}", msg),
                            Err(msg) => println!("{}", msg),
                        }
                    }

                    "5" => {
                        self.tasks_manager.print_tasks();
                    }

                    "6" => {
                        let filename = match Self::input("Введите имя файла для сохранения туда данных: ") {
                            Ok(filename) => filename,
                            Err(err) => {
                                println!("Ошибка при получении ввода пользователя: {}", err);
                                return;
                            }
                        };

                        match self.tasks_manager.store_to_file(filename.as_str()) {
                            Ok(msg) => println!("{}", msg),
                            Err(msg) => println!("{}", msg),
                        }
                    }

                    "7" => {
                        let filename = match Self::input("Введите имя файла для чтения задач из него: ") {
                            Ok(filename) => filename,
                            Err(err) => {
                                println!("Ошибка при получении ввода пользователя: {}", err);
                                return;
                            }
                        };

                        match self.tasks_manager.read_from_file(filename.as_str()) {
                            Ok(msg) => println!("{}", msg),
                            Err(msg) => println!("{}", msg),
                        }
                    }

                    "8" => {
                        let name = match Self::input("Введите имя задачи для отметки как выполненной: ") {
                            Ok(name) => name,
                            Err(err) => {
                                println!("Ошибка при получении ввода пользователя: {}", err);
                                return;
                            }
                        };
        
                        match self.tasks_manager.find_task(name.as_str()) {
                            None => println!("Задачи с именем \"{}\" не существует", name),
                            Some(index) => {
                                self.tasks_manager.tasks[index].mark_as_completed();
                                println!("Задача \"{}\" отмечена как выполненная", name);
                            }
                        }
                    }        

                    _ => println!("Неверный ввод команды")
                }
            }
            Err(err) => println!("Ошибка при получении ввода пользователя: {err}")
        }
    }
}

fn main() {
    let mut manager = ConsoleTask::new();
    manager.print_menu();

    loop {
        manager.process_command();
    }
}