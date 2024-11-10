//       10:00
//       [start]
//          v
//        9:59

use {
    iced::{
        widget::{row, text},
        Element, Task,
    },
    std::time::Duration,
};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Start,
    Stop,
}

#[derive(Default)]
pub struct TimerPanel {
    remaining: Duration,
    running: bool,
}

impl TimerPanel {
    fn start(&mut self) {
        self.running = true;
    }

    fn stop(&mut self) {
        self.running = false;
    }

    pub fn view(&self) -> Element<Message> {
        row![text("hello"),].into()
    }

    pub fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }
}
