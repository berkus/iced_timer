//       10:00
//       [start]
//          v
//        9:59

use {
    iced::{
        widget::{button, column, row, text},
        Element, Task,
    },
    jiff::{ToSpan, Unit},
};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Start,
    Stop,
}

// TODO: Subscribe to a timer event if self.running, otherwise drop subscription.

#[derive(Default)]
pub struct TimerPanel {
    remaining: jiff::Span,
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
        // let mut c = Column::new();
        // if self.remaining.total(Unit::Hour) > 0 {
        //     c = c.push(text(format!("{:?}", self.remaining)));
        // }

        // let c = column![
        //     Some(text("hello")),
        //     None,
        //     Some(button("Start").on_press(Message::Start)),
        // ];

        row![text("hello"), button("Start").on_press(Message::Start)].into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Start => self.start(),
            Message::Stop => self.stop(),
        }
        Task::none()
    }
}
