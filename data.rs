use im::Vector;

#[derive(Clone, Data, Lens)]
pub struct TodoState {
    pub todos: Vector<TodoItem>,
}
#[derive(Clone, Data, Lense)]
pub struct TodoItem {
    pub checked: bool,
    pub text: String
}
