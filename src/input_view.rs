use iced::{
    widget::{button, column, container, row, text, text_input},
    Element, Task,
};

// three similar widgets in panel
// Hours ^[ ]v
// etc

#[derive(Debug, Clone, Copy)]
pub enum Message {
    ErrorInput,
    NewValue(u8),
}

#[derive(Default)]
pub struct InputView {
    value: u8,
    label: &'static str,
    clamp: Option<(u8, u8)>,
}

impl InputView {
    pub fn new(label: &'static str, init_value: Option<u8>) -> Self {
        Self {
            value: init_value.unwrap_or(0),
            label,
            clamp: None,
        }
    }

    pub fn clamped(self, min: u8, max: u8) -> Self {
        Self {
            clamp: Some((min, max)),
            ..self
        }
    }

    pub fn value(&self) -> u8 {
        self.value
    }

    fn clamp(&self, value: u8) -> u8 {
        self.clamp
            .map(|(min, max)| value.clamp(min, max))
            .unwrap_or(value)
    }

    pub fn view(&self) -> Element<Message> {
        column![
            text(self.label),
            button("+").on_press(Message::NewValue(self.clamp(self.value.saturating_add(1)))),
            text_input("0", &format!("{}", self.value)).on_input(|s| {
                let v = s.parse::<u8>();
                if v.is_ok() {
                    Message::NewValue(self.clamp(v.unwrap()))
                } else {
                    Message::ErrorInput
                }
            }),
            button("-").on_press(Message::NewValue(self.clamp(self.value.saturating_sub(1))))
        ]
        .into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ErrorInput => {
                // show error
            }
            Message::NewValue(v) => {
                self.value = v;
            }
        }
        Task::none()
    }
}
