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
            hours: InputView::new("Hours", None).clamped(0, 24),
            minutes: InputView::new("Minutes", Some(10)).clamped(0, 60),
            seconds: InputView::new("Seconds", None).clamped(0, 60),
        }
    }

    pub fn duration(&self) -> jiff::SignedDuration {
        let hours = self.hours.value() as i64;
        let minutes = self.minutes.value() as i64;
        let seconds = self.seconds.value() as i64;
        jiff::SignedDuration::from_hours(hours)
            + jiff::SignedDuration::from_mins(minutes)
            + jiff::SignedDuration::from_secs(seconds)
    }

    pub fn view(&self) -> Element<Message> {
        row![
            self.hours.view().map(Message::HoursChanged),
            self.minutes.view().map(Message::MinutesChanged),
            self.seconds.view().map(Message::SecondsChanged),
        ]
        .into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::HoursChanged(message) => self.hours.update(message).map(Message::HoursChanged),
            Message::MinutesChanged(message) => {
                self.minutes.update(message).map(Message::MinutesChanged)
            }
            Message::SecondsChanged(message) => {
                self.seconds.update(message).map(Message::SecondsChanged)
            }
        }
    }
}
