mod todo_item;
mod app;
use app::*;
use iced::{Settings,Sandbox};
fn main() {
    TodoApp::run(Settings::default())   
}
