use iced::widget::{button, center, center_x, center_y, column, row, text, Column, vertical_space};
use iced::color;
use iced::Font;
use iced::font::Weight::Bold;
use iced::Length::Fill;
use iced::Theme;
use iced::theme::Palette;

#[derive(Default)]
struct App {
    
}

#[derive(Debug, Clone, Copy)]
pub enum AppMessage
{
    Quit,
    Go,
}

impl App {
    pub fn update(&mut self, message: AppMessage) {
        match message {
            AppMessage::Quit => {
                println!("I want to quit, why won't you let me?!");
            }
            AppMessage::Go => {
                println!("I want to go... please, just let me go.");
            }
        }
    }

    pub fn view(&self) -> Column<AppMessage> {
        let gap_big = 50.0;
        let gap_small = 12.5;

        column![
            center_x(text("Rustcog Lab").size(70).font(Font {
                weight: Bold,
                ..Default::default()
            })),
            vertical_space().height(Fill),
            row![
                button(
                    center(
                        row![
                            text("X").color(color!(0xFF6666)).font(Font {
                                weight: Bold,
                                ..Default::default()
                            }),
                            text("STOP").color(color!(0xFFFFFF))
                        ].spacing(gap_small)
                    )
                ).on_press(AppMessage::Quit).height(gap_big),
                button(
                    center(
                        row![
                            text(">").color(color!(0x66FF66)).font(Font {
                                weight: Bold,
                                ..Default::default()
                            }),
                            text("GO!").color(color!(0xFFFFFF))
                        ].spacing(gap_small)
                    )
                ).on_press(AppMessage::Go).height(gap_big)
            ].spacing(gap_big)
        ]
        .spacing(gap_big)
        .padding(gap_big)
    }

    fn theme(&self) -> Theme {
        Theme::custom(
            "RustyCog",
            Palette {
                background: color!(0x444444),
                text: color!(0xEEEEEE),
                primary: color!(0xB34100),
                success: color!(0xFFFF00),
                warning: color!(0x8A3200),
                danger: color!(0xFF5C00)
            }
        )
    }
}


fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("Rustcog Lab")
        .theme(App::theme)
        .run()
}