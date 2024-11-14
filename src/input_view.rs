use iced::{
    alignment::Horizontal,
    border::Radius,
    theme,
    widget::{button, column, container, row, text, text_input},
    Border, Color, Element, Length, Task,
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
    error: bool,
}

impl InputView {
    pub fn new(label: &'static str, init_value: Option<u8>) -> Self {
        Self {
            value: init_value.unwrap_or(0),
            label,
            clamp: None,
            error: false,
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
        let c = column![
            text(self.label),
            button("+").on_press(Message::NewValue(self.clamp(self.value.saturating_add(1)))),
            text_input("0", &format!("{}", self.value))
                .on_input(|s| {
                    let v = s.parse::<u8>();
                    if v.is_ok() {
                        Message::NewValue(self.clamp(v.unwrap()))
                    } else {
                        Message::ErrorInput
                    }
                })
                .size(20)
                .width(Length::FillPortion(1)),
            button("-").on_press(Message::NewValue(self.clamp(self.value.saturating_sub(1))))
        ]
        .align_x(Horizontal::Center)
        .width(Length::Shrink);

        if self.error {
            container(c)
                .style(|theme| {
                    container::bordered_box(theme).border(Border {
                        color: Color::new(1.0, 0.0, 0.0, 0.5),
                        radius: Radius::new(5.0),
                        width: 3.0,
                    })
                })
                .into()
        } else {
            container(c).into()
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ErrorInput => {
                self.error = true;
            }
            Message::NewValue(v) => {
                self.value = v;
                self.error = false;
            }
        }
        Task::none()
    }
}
