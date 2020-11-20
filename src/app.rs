use super::todo_item::*;
use iced::{Checkbox, checkbox, Sandbox, Row, TextInput, text_input, button, Button, Column, Text};
#[derive(Debug,Default)]
pub struct TodoApp{
    data: Vec<TodoItem>,
    input_data:String,
    input_state:text_input::State,
    add_btn: button::State,
    // remove: button::State,/*remove button*/
    selected: usize,
}
#[derive(Debug, Clone)]
pub enum TodoAppMessage {
    ToggleItem(usize, TodoMessage),
    InputChange(String),
    AddTodo,

}

impl Sandbox for TodoApp{

    type Message = TodoAppMessage;

    fn new() -> Self {
        let mut new_self= Self::default();
        let mut data: Vec<TodoItem> =Vec::new();
    /*this is for loop data from todo_item*/
        // for _ in 1..3{
        //     data.push(TodoItem::default())
        // }
        new_self.data = data;
        new_self
    }

    fn title(&self) -> String {
        String::from("Todo List")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            TodoAppMessage::ToggleItem(i,message)=>{
                match message {
                    TodoMessage::ToggleCompleted(b) => {
                        if let Some(item) = self.data.get_mut(i){
                            item.update(message);
                        }
                    },
                    TodoMessage::RemoveClicked => {
                        self.data.remove(i);
                    }
                }
                
            }
            TodoAppMessage::InputChange(data)=>{
                self.input_data =data;
                println!("{}", self.input_data);
            }
            TodoAppMessage::AddTodo=>{
                let mut new_todo = TodoItem::default(); 
                new_todo.desc = self.input_data.clone();
                self.input_data = String::new();
                self.data.push(new_todo);
            }
        }
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        self.data
            .iter_mut()
            .enumerate()
            //to map the the item in todo_item
            .fold(Column::new().spacing(10), |c, (index, item)|{
                c.push(
                    item.view()
                        .map(move |message | TodoAppMessage::ToggleItem(index, message))
                )
            })
            .push(TextInput::new(
                &mut self.input_state,
                "Add todo",
                &self.input_data,
                TodoAppMessage::InputChange,

            ))
            .push(
                Row::new()
                .push(
                    Button::new (
                        &mut self.add_btn, 
                        Text::new("Add")).on_press(TodoAppMessage::AddTodo),
                )
                // .push(Button::new (&mut self.remove, Text::new("Remove")).on_press(TodoAppMessage::RemoveTodo),
                // ) /*for only remove button*/
            )
             
            .into()
    }
}
