// Hours ^[ 0]v : Minutes ^[10]v : Seconds ^[ 0]v

use {
    crate::input_view::{InputView, Message as InputViewMessage},
    iced::{
        widget::{button, column, container, row, text, text_input},
        Element, Task,
    },
};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    HoursChanged(InputViewMessage),
    MinutesChanged(InputViewMessage),
    SecondsChanged(InputViewMessage),
}

#[derive(Default)]
pub struct InitPanel {
    hours: InputView,
    minutes: InputView,
    seconds: InputView,
}

impl InitPanel {
    pub fn new() -> Self {
        Self {
            hours: InputView::new("Hours", None),
            minutes: InputView::new("Minutes", Some(10)),
            seconds: InputView::new("Seconds", None),
        }
    }

    pub fn view(&self) -> Element<Message> {
        row![
            self.hours.view().map(Message::HoursChanged),
            self.minutes.view().map(Message::MinutesChanged),
            self.seconds.view().map(Message::SecondsChanged),
        ]
        .into()
    }

    pub fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }
}
