use std::sync::Mutex;

pub type ApiData = lv_server::deps::actix_web::web::Data<AppData>;

/// A shared mutable state to simulate a database, the implementation is far from
/// efficient, lots of cloning is happening to keep the code simple.
pub struct AppData {
  todos: Mutex<Vec<Todo>>
}

#[derive(Clone, Default)]
pub struct Todo {
  pub text: String
}

impl AppData {
  pub fn new() -> Self {
    Self {
      todos: Mutex::new(Vec::new())
    }
  }

  pub fn todos(&self) -> Vec<Todo> {
    self.todos.lock().unwrap().clone()
  }

  pub fn set_todos(&self, todos: Vec<Todo>) {
    *self.todos.lock().unwrap() = todos;
  }

  pub fn add_todo(&self, text: String) {
    let mut cur = self.todos();
    cur.push(Todo { text });

    self.set_todos(cur);
  }

  pub fn update_todo_by_index(&self, index: usize, text: String) -> Todo {
    let mut cur = self.todos();
    let new_todo = Todo { text };

    if cur.len() > index {
      cur[index] = new_todo.clone();
    }

    self.set_todos(cur);

    new_todo
  }

  pub fn remove_todo_by_index(&self, index: usize) {
    let mut cur = self.todos();
    cur.remove(index);

    self.set_todos(cur);
  }
}
