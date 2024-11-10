// Hours [ ] : Minutes [ ] : Seconds [ ]
//        v
//       10:00
//       [start]
#![allow(unused_imports)]

use iced::{
    widget::{column, container, Container},
    Element, Task,
};

mod init_panel;
mod input_view;
mod timer_panel;

pub fn main() -> iced::Result {
    tracing_subscriber::fmt()
        .map_event_format(|f| f.pretty())
        .init();
    iced::run("Iced Timer", IcedTimer::update, IcedTimer::view)
}

#[derive(Debug, Clone, Copy)]
enum Message {
    InitPanel(init_panel::Message),
    TimerPanel(timer_panel::Message),
}

#[derive(Default)]
struct IcedTimer {
    init_panel: init_panel::InitPanel,
    timer_panel: timer_panel::TimerPanel,
}

impl IcedTimer {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::InitPanel(message) => self.init_panel.update(message).map(Message::InitPanel),
            Message::TimerPanel(message) => {
                self.timer_panel.update(message).map(Message::TimerPanel)
            }
        }
    }

    fn view(&self) -> Element<Message> {
        container(column![
            self.init_panel.view().map(Message::InitPanel),
            self.timer_panel.view().map(Message::TimerPanel),
        ])
        .into()
    }
}
