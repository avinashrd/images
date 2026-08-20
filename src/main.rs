#[derive(Debug)]
enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
    priority: Priority,
    done: bool,
}

impl Task {
    fn new(id: u32, title: &str, priority: Priority) -> Self {
        Task {
            id,
            title: title.to_string(),
            priority,
            done: false,
        }
    }

    fn complete(&mut self) {
        self.done = true;
    }

    fn status(&self) -> &str {
        if self.done { "✓" } else { "○" }
    }
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add(&mut self, title: &str, priority: Priority) -> u32 {
        let id = self.next_id;
        self.tasks.push(Task::new(id, title, priority));
        self.next_id += 1;
        id
    }

    fn complete(&mut self, id: u32) -> bool {
        match self.tasks.iter_mut().find(|t| t.id == id) {
            Some(task) => { task.complete(); true }
            None => false,
        }
    }

    fn summary(&self) {
        let total = self.tasks.len();
        let done = self.tasks.iter().filter(|t| t.done).count();
        println!("Tasks: {}/{} completed\n", done, total);

        for task in &self.tasks {
            let priority = match task.priority {
                Priority::High   => "HIGH  ",
                Priority::Medium => "MEDIUM",
                Priority::Low    => "LOW   ",
            };
            println!("  [{}] #{} [{}] {}", task.status(), task.id, priority, task.title);
        }
    }
}

fn main() {
    let mut manager = TaskManager::new();

    manager.add("Set up Rust project", Priority::High);
    manager.add("Write unit tests", Priority::Medium);
    manager.add("Update documentation", Priority::Low);
    manager.add("Deploy to production", Priority::High);

    manager.complete(1);
    manager.complete(3);

    println!("=== Task Manager ===\n");
    manager.summary();
}
