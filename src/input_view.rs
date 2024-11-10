use iced::{
    widget::{button, column, container, row, text, text_input},
    Element,
};

// three similar widgets in panel
// Hours ^[ ]v
// etc

#[derive(Debug, Clone, Copy)]
pub enum Message {
    ErrorInput,
    NewValue(u8),
}

// this is not a widget but a "view" thingie
#[derive(Default)]
pub struct InputView {
    value: u8,
    label: &'static str,
}

// Deprecated since 0.13.0: components introduce encapsulated state and hamper the use
// of a single source of truth. Instead, leverage the Elm Architecture directly,
// or implement a custom widget (this means impl Widget trait).

impl InputView {
    pub fn new(label: &'static str, init_value: Option<u8>) -> Self {
        Self {
            value: init_value.unwrap_or(0),
            label,
        }
    }

    pub fn view(&self) -> Element<Message> {
        column![
            text(self.label),
            button("^").on_press(Message::NewValue(self.value.saturating_add(1))),
            text_input("0", &format!("{}", self.value)).on_input(|s| {
                let v = s.parse::<u8>();
                if v.is_ok() {
                    Message::NewValue(v.unwrap())
                } else {
                    Message::ErrorInput
                }
            }),
            button("v").on_press(Message::NewValue(self.value.saturating_sub(1)))
        ]
        .into()
    }
}
