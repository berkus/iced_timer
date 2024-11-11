//       10:00
//       [start]
//          v
//        9:59

use iced::{
    time::{self, Duration},
    widget::{button, column, row, text, Column, Row},
    Element, Subscription, Task,
};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Init(jiff::SignedDuration),
    Start,
    Stop,
    Tick,
    Finished,
}

// TODO: Subscribe to a timer event if self.running, otherwise drop subscription.

#[derive(Default)]
pub struct TimerPanel {
    remaining: jiff::SignedDuration,
    running: bool,
}

impl TimerPanel {
    fn start(&mut self) {
        self.running = true;
    }

    fn stop(&mut self) {
        self.running = false;
    }

    pub fn subscription(&self) -> Subscription<Message> {
        if self.running {
            time::every(Duration::from_secs(1)).map(|_| Message::Tick)
        } else {
            Subscription::none()
        }
    }

    pub fn view(&self) -> Element<Message> {
        let r = row([
            self.remaining.as_hours(),
            self.remaining.as_mins(),
            self.remaining.as_secs(),
        ]
        .into_iter()
        .skip_while(|&value| value == 0)
        .map(|value| text(value).into()));

        let b = if self.running {
            button("Stop").on_press(Message::Stop)
        } else {
            button("Start").on_press(Message::Start)
        };

        column![r, b].into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Init(remaining) => self.remaining = remaining,
            Message::Start => self.start(),
            Message::Stop => self.stop(),
            Message::Tick => {
                if self.remaining.as_secs() == 0 {
                    self.running = false;
                    return Task::done(Message::Finished);
                }
                self.remaining = self.remaining - jiff::SignedDuration::from_secs(1);
            }
            Message::Finished => {
                self.remaining = jiff::SignedDuration::from_secs(0);
                return Task::done(Message::Stop);
            }
        }
        Task::none()
    }
}
