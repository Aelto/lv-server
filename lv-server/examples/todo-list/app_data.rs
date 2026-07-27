use std::sync::Mutex;

pub type ApiData = lv_server::deps::actix_web::web::Data<AppData>;

/// A shared mutable state to simulate a database, the implementation is far from
/// efficient, lots of cloning is happening to keep the code simple.
pub struct AppData {
  todos: Mutex<Vec<Todo>>
}

#[derive(Clone, Default)]
pub struct Todo {
  pub id: String,
  pub text: String
}

pub struct FakeItem {
  pub id: String
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
    let id = cur.len().to_string();
    cur.push(Todo { id, text });

    self.set_todos(cur);
  }

  pub fn update_todo_by_id(&self, id: String, text: String) -> Todo {
    let mut cur = self.todos();
    let new_todo = Todo { id: id.clone(), text };

    for todo in &mut cur {
      if todo.id == id {
        *todo = new_todo.clone();
      }
    }


    self.set_todos(cur);

    new_todo
  }

  pub fn remove_todo_by_index(&self, index: usize) {
    let mut cur = self.todos();
    cur.remove(index);

    self.set_todos(cur);
  }

  pub fn remove_todo_by_id(&self, id: &str) {
    let mut cur = self.todos();
    cur.retain(|t| t.id != id);

    self.set_todos(cur);
  }

  pub fn find_fake_items_after(&self, id: &str) -> Vec<FakeItem> {
    let number: i64 = id.parse().unwrap();

    vec![
      FakeItem { id: (number + 1).to_string() },
      FakeItem { id: (number + 2).to_string() },
      FakeItem { id: (number + 3).to_string() },
      FakeItem { id: (number + 4).to_string() },
      FakeItem { id: (number + 5).to_string() },
    ]
  }
}
