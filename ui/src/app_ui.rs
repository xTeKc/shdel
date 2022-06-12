#[allow(unused)]
use core_app;
use crate::style;

use iced::{
    button, Alignment, Button, Checkbox,
    Column, Container, Element, Length,
    Row, Rule, Sandbox, Text,
};

#[derive(Default)]
pub struct TheApplication {
    theme: style::Theme,
    delete_button: button::State,
    reboot_button: button::State,
    bash_checkbox_value: bool,
    zsh_checkbox_value: bool,
    auto_reboot_checkbox_value: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    ButtonPressed,
    BashCheckboxToggled(bool),
    ZshCheckboxToggled(bool),
    AutoRebootCheckboxToggled(bool),
}

impl Sandbox for TheApplication {
    type Message = Message;

    fn new() -> Self {
        TheApplication::default()
    }

    fn title(&self) -> String {
        String::from("ShellDel")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed => {}
            Message::BashCheckboxToggled(value) => self.bash_checkbox_value = value,
            Message::ZshCheckboxToggled(value) => self.zsh_checkbox_value = value,
            Message::AutoRebootCheckboxToggled(value) => self.auto_reboot_checkbox_value = value,
        }
    }

    fn view(&mut self) -> Element<Message> {
        let delete_button = Button::new(&mut self.delete_button, Text::new("Delete History"))
            .padding(10)
            .on_press(Message::ButtonPressed)
            .style(self.theme);

            let reboot_button = Button::new(&mut self.reboot_button, Text::new("Reboot Terminal"))
            .padding(10)
            .on_press(Message::ButtonPressed)
            .style(self.theme);

        let bash_checkbox = Checkbox::new(
            self.bash_checkbox_value,
            "Bash",
            Message::BashCheckboxToggled,
        )
        .style(self.theme);

        let zsh_checkbox = Checkbox::new(
            self.zsh_checkbox_value,
            "Zsh",
            Message::ZshCheckboxToggled,
        )
        .style(self.theme);

        let auto_reboot_checkbox = Checkbox::new(
            self.auto_reboot_checkbox_value,
            "Auto-Reboot Terminal",
            Message::AutoRebootCheckboxToggled,
        )
        .style(self.theme);

        let content = Column::new()
            .spacing(5)
            .max_width(850)
            .align_items(Alignment::Center)
            .push(Rule::horizontal(50).style(self.theme))
            .push(
                Row::new()
                    .height(Length::Units(100))
                    .align_items(Alignment::Center)
                    .spacing(10)
                    .push(reboot_button)
                    .push(Rule::vertical(50).style(self.theme))
                    .push(
                        Row::new()
                            .height(Length::Units(100))
                            .align_items(Alignment::Center)
                            .push(delete_button)
                            .push(auto_reboot_checkbox)
                            .spacing(10)
                            .push(Rule::vertical(50).style(self.theme)))
                    .push(
                        Column::new()
                            .width(Length::Shrink)
                            .spacing(20)
                            .push(bash_checkbox)
                            .push(zsh_checkbox)
                    ),
            );

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(self.theme)
            .into()
    }
}