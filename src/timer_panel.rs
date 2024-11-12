//       10:00
//       [start]
//          v
//        9:59

use iced::{
    alignment::Horizontal,
    time::{self, Duration},
    widget::{button, column, horizontal_space, row, text, Column, Row},
    Element, Length, Subscription, Task,
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
        let hours = self.remaining.as_hours();
        let minutes = self.remaining.as_mins() - self.remaining.as_hours() * 60;
        let seconds = self.remaining.as_secs() - self.remaining.as_mins() * 60;

        let r = row([
            Some(horizontal_space().into()),
            if hours == 0 {
                None
            } else {
                Some(text(format!("{hours:>2}")).size(65).into())
            },
            if hours == 0 {
                None
            } else {
                Some(text(":").size(65).into())
            },
            if minutes == 0 && hours == 0 {
                None
            } else {
                if hours == 0 {
                    Some(text(format!("{minutes:>2}")).size(65).into())
                } else {
                    Some(text(format!("{minutes:02}")).size(65).into())
                }
            },
            if minutes == 0 && hours == 0 {
                None
            } else {
                Some(text(":").size(65).into())
            },
            Some(text(format!("{seconds:02}")).size(65).into()),
            Some(horizontal_space().into()),
        ]
        .into_iter()
        .filter_map(|value| value));

        let b = if self.running {
            button("Stop").on_press(Message::Stop)
        } else {
            button("Start").on_press(Message::Start)
        };

        column![r, b].align_x(Horizontal::Center).into()
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
