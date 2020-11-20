use iced::{Checkbox, Element, Row,Button, Text, button};

#[derive(Debug,Default)]
pub struct TodoItem{
    pub completed:bool,
    pub desc:String,
    pub btn:button::State,
    pub text:String,
}
// impl Default for TodoItem {
//     fn default() -> Self {
//         Self {
//             completed: false,
//             desc: String::from("New item"),
//             btn: button::State,

//         }
//     }
// }
#[derive(Debug, Clone, Copy)]
pub enum TodoMessage{
    ToggleCompleted(bool),
    RemoveClicked,
}
impl TodoItem{
    pub fn new() ->Self{
        Self::default()
    }
    pub fn update(&mut self, message: TodoMessage){
        match message{
            TodoMessage::ToggleCompleted(b)=> self.completed=b,
            TodoMessage::RemoveClicked => {}
        }

    }
    pub fn view(&mut self)->Element<TodoMessage>{
        Row::new()
        .spacing(20)
        .push(
            Checkbox::new (self.completed, &self.desc, TodoMessage::ToggleCompleted),
        )
        .push(
            Button::new(&mut self.btn, Text::new("remove" )).on_press(TodoMessage::RemoveClicked),
        )
        .into()
       
    }
}

